/// Handles a connection to rabbit mq.
/// Author: Andrew Evans
use amiquip::{Channel, Connection, Exchange, Publish, Result, ConnectionOptions, ConnectionTuning, Auth};
use native_tls::{TlsConnector, Certificate};
use std::fs;
use std::sync::{Arc, Mutex};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use crate::amqp::amqp::AMQPConnectionInf;
use mio::net::TcpStream;
use std::net::SocketAddr;
use std::str::FromStr;


/// Connection object
pub struct RabbitMQConnection{
    pub connection: Connection,
    pub channel: Channel,
}


/// Implementation
impl RabbitMQConnection {

    /// get the pem bytes
    fn get_fbytes(cert_file: &String) -> Option<String>{
        let contents = fs::read_to_string(cert_file);
        if contents.is_ok(){
            let fstr = contents.unwrap();
            Some(fstr)
        }else{
            None
        }
    }

    /// get the certificate
    fn get_certificate(is_pem: &bool, cbytes: String) -> Certificate{
        if is_pem.clone() {
            Certificate::from_pem(cbytes.as_bytes()).unwrap()
        }else{
            Certificate::from_der(cbytes.as_bytes()).unwrap()
        }
    }

    /// get the tls connector
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

    /// get the connection
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
    pub fn new(url: String, conn_inf: &AMQPConnectionInf) -> Result<RabbitMQConnection, &'static str> {
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
                Err("Failed to Establish a Channel")
            }
        }else {
            match conn_result{
                Err(e) =>{
                    println!("{}", e);
                }
                _ => {}
            }
            Err("Failed to Establish a Connection")
        }
    }
}
