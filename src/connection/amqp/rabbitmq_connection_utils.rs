use std::borrow::{Borrow, BorrowMut};
use std::error::Error as StdError;
use std::fs;
use std::net::SocketAddr;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use amq_protocol::tcp::AMQPUriTcpExt;
use lapin::{
    BasicProperties, confirmation::Confirmation, Connection, ConnectionProperties,
    ConsumerDelegate, Error, message::DeliveryResult, options::*, Result, types::FieldTable,
};
use log::info;
use native_tls::{Certificate, Identity};
use tcp_stream::{HandshakeError, NativeTlsConnector};
use tokio::prelude::*;
use tokio;
use tokio::runtime::Runtime;
use tokio::time::timeout;


use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::error::connection_failed::ConnectionFailed;
use crate::error::ssl_error::SSLError;
use crate::security::ssl::SSLConfig;
use std::time::Duration;


/// Get the pem bytes `std::option::Option<std::string::String>`
///
/// # Arguments
/// * `cert_file` - Certification file `std::string::String`
fn get_fbytes(cert_file: &String) -> Option<String>{
    let contents = fs::read_to_string(cert_file);
    if contents.is_ok(){
        let fstr = contents.unwrap();
        Some(fstr)
    }else{
        None
    }
}


/// Get a `.pem` based SSL `native_tls::Certificate` and optional `native_tls::Identity`
/// which is always `std::option::None` for conformity with obtaining `.dem` certificate.
///
/// # Arguments
/// * `config` - The `crate::security::SSLConfig` reference
fn get_pem(config: &SSLConfig) -> StdResult<(Certificate, Option<Identity>), SSLError>{
    let pem_file = &config.get_cert_file().clone().unwrap();
    let file_bytes = get_fbytes(pem_file).unwrap();
    let cert = Certificate::from_pem(file_bytes.as_bytes());
    if cert.is_ok() {
        Ok((cert.unwrap(), None))
    }else{
        Err(SSLError)
    }
}


/// Get a `.dem` based SSL `native_tls::Certificate` and optional `native_tls::Identity`
///
/// # Arguments
/// * `config` - The `crate::security::SSLConfig` reference
fn get_der(config: &SSLConfig) -> StdResult<(Certificate, Option<Identity>), SSLError>{
    let der_file = &config.get_cert_file().clone().unwrap();
    let file_bytes = get_fbytes(der_file).unwrap();
    let cert = Certificate::from_der(file_bytes.as_bytes());
    if cert.is_ok() {
        if config.get_der_password().is_some(){
            let pword = config.get_der_password().clone().unwrap();
            let identity = Identity::from_pkcs12(file_bytes.as_bytes(), pword.as_str());
            if identity.is_ok() {
                Ok((cert.unwrap(), Some(identity.unwrap())))
            }else{
                Err(SSLError)
            }
        }else {
            Ok((cert.unwrap(), None))
        }
    }else{
        Err(SSLError)
    }
}


/// Get a custom SSL certificate and optional Identity
///
/// # Arguments
/// * `config`  - The `crate::security::SSLCOnfig` reference
fn get_custom_certificate(config: &SSLConfig) -> (Certificate, Option<Identity>) {
    if config.is_pem().clone(){
        get_pem(config).ok().unwrap()
    }else{
        get_der(config).ok().unwrap()
    }
}


/// Get a tls `lapin::Connection` asynchronously in a `lapin::Result`
///
/// # Argument
/// * `addr` - URI for the connection
/// * `config` - The `crate::security::SSLConfig`
async fn get_tls_connection(addr: String, config: SSLConfig) -> Result<Connection> {
    let conn = addr
        .connect(|stream, uri, poll| {
            let mut  tls_builder = NativeTlsConnector::builder();
            if config.get_cert_file().is_some(){
                let (cert, identity) = get_custom_certificate(&config);
                tls_builder.add_root_certificate(cert);
                if identity.is_some(){
                    tls_builder.identity(identity.unwrap());
                }
            }
            // Perform here your custom tls setup, with tls_builder.identity or whatever else you need
            let mut res = stream.into_native_tls(
                tls_builder.build().expect("TLS configuration failed"),
                &uri.authority.host,
            );
            let start_tstamp = chrono::Utc::now().timestamp();
            let max_wait = config.get_ssl_timeout().clone();
            while let Err(error) = res {
                match error {
                    HandshakeError::Failure(io_err) => {
                        panic!("TLS connection failed: {:?}", io_err)
                    }
                    HandshakeError::WouldBlock(mid) => res = mid.handshake(),
                }
                let tnow = chrono::Utc::now().timestamp();
                if tnow - start_tstamp < tnow{
                    panic!("Timeout in TLS Connection")
                }
            }
            let tlsstream = res.unwrap();
            let p = ConnectionProperties::default();
            Connection::connector(p)(tlsstream, uri, poll)
        })
        .map_err(Error::IOError)?;
    Confirmation::from(conn).await
}


/// Get a standard tcp `lapin::Connection` without SSL asynchronously
///
/// # Arguments
/// * `addr` - URI for the connection
async fn get_standard_connection(addr: String) -> Result<Connection>{
    let conn = addr.connect(|stream, uri, poll| {
        let p = ConnectionProperties::default();
        Connection::connector(p)(stream, uri, poll)
    }).map_err(Error::IOError)?;
    Confirmation::from(conn).await
}


/// Get a `lapin::Connection` or error in a `std::result::Result`
///
/// # Arguments
/// * `conn_fin` - The `crate::connection::amqp::connection_inf::AMQPConnectionInf`
/// * `runtime` - A reference to your base `tokio::runtime::Runtime`
pub fn get_connection(conn_inf: &AMQPConnectionInf, runtime: &mut Runtime) -> StdResult<Connection, ConnectionFailed>{
    let uri = conn_inf.to_url();
    if conn_inf.is_ssl() && conn_inf.get_ssl_config().is_some(){
        let ssl_conf = conn_inf.get_ssl_config().clone().unwrap();
        let conn_result = runtime.block_on(async move{
            timeout(Duration::from_millis(conn_inf.get_conection_timeout()), get_tls_connection(uri, ssl_conf)).await
        });
        if conn_result.is_ok() {
            let conn = conn_result.unwrap();
            if conn.is_ok() {
                Ok(conn.unwrap())
            }else{
                Err(ConnectionFailed)
            }
        } else {
            Err(ConnectionFailed)
        }
    }else {
        let conn_result = runtime.block_on( async move{
            timeout(Duration::from_millis(conn_inf.get_conection_timeout()),get_standard_connection(uri)).await
        });
        if conn_result.is_ok() {
            let conn = conn_result.unwrap();
            if conn.is_ok() {
                Ok(conn.unwrap())
            }else{
                Err(ConnectionFailed)
            }
        } else {
            Err(ConnectionFailed)
        }
    }
}


#[cfg(test)]
pub mod test{
    use super::*;
    use crate::config::config::{CannonConfig, BackendType};
    use crate::router::router::Routers;
    use crate::connection::connection::ConnectionConfig;
    use crate::backend::config::BackendConfig;
    use std::panic;
    use tokio::prelude::*;

    #[test]
    fn test_should_create_connection(){
        let amq_conf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 5672, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false, None,None,5000);
        let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
        let conn= get_connection(&amq_conf, &mut rt);
        assert!(conn.is_ok());
        assert!(conn.ok().unwrap().close(0, "").wait().is_ok());
    }

    #[test]
    fn test_should_panic_at_timeout_on_bad_uri(){
        let amq_conf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 8080, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), false, None,None,5000);
        let conn= panic::catch_unwind(||{
            let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
            get_connection(&amq_conf, &mut rt)
        });
        let r = conn.ok().unwrap();
        assert!(r.is_err());
    }

    #[test]
    fn test_should_panic_on_bad_tls_connection(){
        let ssl_conf = SSLConfig::new(None, false, "".to_string(), 5000,None);
        let amq_conf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 5672, Some("test".to_string()), Some("dev".to_string()), Some("rtp*4500".to_string()), true, Some(ssl_conf),None,5000);
        let mut rt = tokio::runtime::Builder::new().num_threads(1).enable_all().build().unwrap();
        let conn= get_connection(&amq_conf, &mut rt);
        assert!(conn.is_err());
        let cer = conn.err().unwrap();
        if let ConnectionFailed = cer{
            assert!(true)
        }else{
            assert!(false)
        }
    }

    #[test]
    fn test_should_connect_with_tls(){
        unimplemented!()
    }
}
