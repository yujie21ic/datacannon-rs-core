//! AMQP Connection utilities
//!
//! ---
//! author: Andrew Evans
//! ---

use crate::security::ssl::SSLConfig;
use crate::security::uaa::UAAConfig;


/// Struct for connection information
///
/// # Arguments
/// * `protocol` - Protocol to use such as amqp
/// * `host` - Host string
/// * `port` - AMQP connection port
/// * `vhost` - Virtual host name of necessary
/// * `username` - Username if necessary and usually provided with a password
/// * `password` - Password if necessary and usually provided with a username
/// * `is_ssl` - Whether to use ssl
/// * `ssl_config` - SSL configuration if necessary
/// * `uaa_config` - Connection information for an OAuth UAA server
#[derive(Clone, Debug)]
pub struct AMQPConnectionInf{
    protocol: String,
    host: String,
    port: i64,
    vhost: Option<String>,
    username: Option<String>,
    password: Option<String>,
    is_ssl: bool,
    ssl_config: Option<SSLConfig>,
    uaa_config: Option<UAAConfig>,
}


/// Implementation of the connection information
impl AMQPConnectionInf{

    /// Get the protocol `std::string::String`
    pub fn get_protocol(&self) -> String{
        self.protocol.clone()
    }

    ///Get the host `std::string::String`
    pub fn get_host(&self) -> String{
        self.host.clone()
    }

    /// Get the port  `std::i64`
    pub fn get_port(&self) -> i64{
        self.port.clone()
    }

    /// Get the vhost `std::option::Option::<std::string::String>`
    pub fn get_vhost(&self) -> Option<String>{
        self.vhost.clone()
    }

    /// Get the username `std::option::Option<std::string::String>`
    pub fn get_username(&self) -> Option<String>{
        self.username.clone()
    }

    /// Get the password `std::option::Option<std::string::String>`
    pub fn get_password(&self) -> Option<String>{
        self.password.clone()
    }

    /// Whether to use ssl. Returns `std::bool`
    pub fn is_ssl(&self) -> bool{
        self.is_ssl.clone()
    }

    /// Get the ssl config `&std::option::Option<crate::security::ssl::SSLConfig>`
    pub fn get_ssl_config(&self) -> &Option<SSLConfig>{
        &self.ssl_config
    }

    /// Get the UAA config `&std::option::Option<crate::security::uaa::UAAConfig>`
    pub fn get_uaa_config(&self) -> &Option<UAAConfig>{
        &self.uaa_config
    }

    /// convert the Information to a URL `std::string::String`
    pub fn to_url(&self) -> String {
        let cinf = self.clone();
        let mut url = "".to_string();
        if cinf.username.is_some() && cinf.password.is_some(){
            url = format!("{}://{}:{}@{}:{}", cinf.protocol, cinf.username.unwrap(), cinf.password.unwrap(), cinf.host, cinf.port);
        }else{
            url = format!("{}://{}:{}", cinf.protocol, cinf.host, cinf.port);
        }
        if self.vhost.is_some(){
            url = format!("{}/{}", url, cinf.vhost.unwrap());
        }
        url
    }

    /// Create a new connection information structure
    ///
    /// # Arguments
    /// * `protocol` - Protocol to use such as amqp
    /// * `host` - Host string
    /// * `port` - AMQP connection port
    /// * `vhost` - Virtual host name of necessary
    /// * `username` - Username if necessary and usually provided with a password
    /// * `password` - Password if necessary and usually provided with a username
    /// * `is_ssl` - Whether to use ssl
    /// * `ssl_config` - SSL configuration if necessary
    /// * `uaa_config` - Connection information for an OAuth UAA server
    pub fn new(protocol: String, host: String, port: i64, vhost: Option<String>, username: Option<String>, password: Option<String>, is_ssl: bool, ssl_config: Option<SSLConfig>, uaa_config: Option<UAAConfig>) -> AMQPConnectionInf{
        AMQPConnectionInf{
            protocol: protocol,
            host: host,
            port: port,
            vhost: vhost,
            username: username,
            password: password,
            is_ssl: is_ssl,
            ssl_config: ssl_config,
            uaa_config: uaa_config,
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::connection::amqp::connection_inf::AMQPConnectionInf;

    #[test]
    pub fn test_create_url(){
        let cinf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 3030, None, None, None, false, None, None);
        let url = cinf.to_url();
        assert!(url.eq("amqp://127.0.0.1:3030"));
    }

    #[test]
    pub fn should_use_credentials(){
        let cinf = AMQPConnectionInf::new("amqp".to_string(), "127.0.0.1".to_string(), 3030, None, Some("test".to_string()), Some("123".to_string()), false, None, None);
        let url = cinf.to_url();
        assert!(url.eq("amqp://test:123@127.0.0.1:3030"));
    }
}