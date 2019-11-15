use std::borrow::{Borrow, BorrowMut};
use std::fs;
use std::net::SocketAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

/// Handles a connection to rabbit mq.
/// Author: Andrew Evans
use amiquip::{Auth, Channel, Connection, ConnectionOptions, ConnectionTuning, Exchange, Publish, Result};
use mio::net::TcpStream;
use native_tls::{Certificate, TlsConnector};

use crate::connection::amqp::connection_inf::AMQPConnectionInf;
use crate::error::connection_failed::ConnectionFailed;

/// Connection object containing a connection and a channel
pub struct RabbitMQConnection{
    pub connection: Connection,
    pub channel: Channel,
}


/// Implementation
impl RabbitMQConnection {

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

    /// Obtain a `native_tls::certificate` from byte contents
    ///
    /// # Argument
    /// * `is_pem` - Whether ot use PEM format
    /// * `cbytes` - Byte `std::string::String` containing the file
    fn get_certificate(is_pem: &bool, cbytes: String) -> Certificate{
        if is_pem.clone() {
            Certificate::from_pem(cbytes.as_bytes()).unwrap()
        }else{
            Certificate::from_der(cbytes.as_bytes()).unwrap()
        }
    }

    /// Get the `native_tls::TlsConnectior` from AMQP connection information
    ///
    /// # Arguments
    /// * `conn_inf` - Connection information stored in a `crate::connection::amqp::connection_inf::AMQPConnectionInf` structure
    fn get_tls_connector(conn_inf: &AMQPConnectionInf) -> TlsConnector{
        let mut b = TlsConnector::builder();
        let ssl_conf_opt = conn_inf.get_ssl_config();
        if ssl_conf_opt.is_some(){
            let ssl_conf = ssl_conf_opt.clone().unwrap();
            let cfile = ssl_conf.get_cert_file();
            let cbytes_opt = RabbitMQConnection::get_fbytes(cfile);
            if cbytes_opt.is_some(){
                let cbytes = cbytes_opt.unwrap();
                let cert = RabbitMQConnection::get_certificate(ssl_conf.is_pem(), cbytes);
                b.danger_accept_invalid_certs(false);
                b.add_root_certificate(cert);
            }
        }
        b.build().unwrap()
    }

    /// Get a `amiquip::Connection` to RabbitMQ or return an error
    ///
    /// # Arguments
    /// * `url` - Reference to a url `std::string::String`
    /// * `conn_inf` - Reference to the relevant `crate::connection::amqp::connection_inf::AMQPConnectionInf`
    fn get_connection(url: &String, conn_inf: &AMQPConnectionInf) -> Result<Connection>{
        let conn_result = Connection::insecure_open(url.as_str());
        if conn_inf.is_ssl() {
            let connector = RabbitMQConnection::get_tls_connector(conn_inf);
            let copts = ConnectionOptions::default()
                .auth(Auth::default());
            let ctun = ConnectionTuning::default();
            let domain = conn_inf.clone().get_ssl_config().clone().unwrap().get_domain().clone();
            let sock_addr = SocketAddr::from_str(url.as_str()).unwrap();
            let stream = mio::net::TcpStream::connect(&sock_addr).unwrap();
            Connection::open_tls_stream(connector, domain.as_str(), stream, copts,ctun)
        }else{
            conn_result
        }
    }

    /// Create the new connection
    pub fn new(url: String, conn_inf: &AMQPConnectionInf) -> Result<RabbitMQConnection, ConnectionFailed> {
        let conn_result = RabbitMQConnection::get_connection(&url, conn_inf);
        if(conn_result.is_ok()){
            let mut conn = conn_result.unwrap();
            let channel_result = conn.open_channel(None);
            if(channel_result.is_ok()) {
                let channel = channel_result.unwrap();
                let conn_object = RabbitMQConnection {
                    connection: conn,
                    channel: channel,
                };
                Ok(conn_object)
            }else{
                match channel_result{
                    Err(e) =>{
                        println!("{}", e);
                    }
                    _ => {}
                }
                Err(ConnectionFailed)
            }
        }else {
            match conn_result{
                Err(e) =>{
                    println!("{}", e);
                }
                _ => {}
            }
            Err(ConnectionFailed)
        }
    }
}
