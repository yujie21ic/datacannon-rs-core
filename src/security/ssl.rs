//! UAA  Configuration
//!
//! ---
//! author: Andrew Evans
//! ---


/// SSL Configuration
///
/// # Arguments
/// * `cert_file`- Certification file path `std::string::String`
/// * `is_pem` - Certification type is PEM
/// * `domain`- Domain for the SSL file
#[derive(Clone, Debug)]
pub struct SSLConfig{
    cert_file: Option<String>,
    is_pem: bool,
    domain: String,
    ssl_timeout: i64,
    der_password: Option<String>,
}


///SSL Configuration
impl SSLConfig{

    /// Get the cert file `std::string::String`
    pub fn get_cert_file(&self) -> Option<String>{
        self.cert_file.clone()
    }

    /// Is the file in pem format `std::bool`
    pub fn is_pem(&self) -> &bool{
        &self.is_pem
    }

    /// Get the domain for the file `std::string::String`
    pub fn get_domain(&self) -> &String{
        &self.domain
    }

    /// Get the SSL Timeout
    pub fn get_ssl_timeout(&self) -> i64{
        self.ssl_timeout.clone()
    }

    /// Get the der password
    pub fn get_der_password(&self) -> Option<String>{
        self.der_password.clone()
    }

    /// Create the new SSL config
    ///
    /// # Arguments
    /// * `cert_file`- Certification file path `std::string::String`
    /// * `is_pem` - Certification type is PEM
    /// * `domain`- Domain for the SSL file
    /// * `ssl_timeout` - The SSL timeout
    pub fn new(cert_file: Option<String>, is_pem: bool, domain: String, ssl_timeout: i64, der_password: Option<String>) -> SSLConfig{
        SSLConfig{
            cert_file: cert_file,
            is_pem: is_pem,
            domain: domain,
            ssl_timeout: ssl_timeout,
            der_password: der_password,
        }
    }

}
