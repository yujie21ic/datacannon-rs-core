//! UAA  Configuration
//!
//! ---
//! author: Andrew Evans
//! ---


/// SSL Configuration
///
/// # Arguments
/// * `key_file` - Key file path `std::string::String`
/// * `cert_file`- Certification file path `std::string::String`
/// * `ca_certs` - CA Certifications file path
/// * `cert_reqs` - Require certification
/// * `is_pem` - Certification type is PEM
/// * `domain`- Domain for the SSL file
#[derive(Clone, Debug)]
pub struct SSLConfig{
    key_file: String,
    cert_file: String,
    ca_certs: String,
    cert_reqs: bool,
    is_pem: bool,
    domain: String,
}


///SSL Configuration
impl SSLConfig{

    /// Obtain the key file reference `std::string::String`
    pub fn get_key_file(&self) -> &String{
        &self.key_file
    }

    /// Get the cert file `std::string::String`
    pub fn get_cert_file(&self) -> &String{
        &self.cert_file
    }

    /// Get the CA certs file `std::string::String`
    pub fn get_ca_certs(&self) -> &String{
        &self.ca_certs
    }

    /// Should certification be required `std::bool`
    pub fn get_cert_reqs(&self) -> &bool{
        &self.cert_reqs
    }

    /// Is the file in pem format `std::bool`
    pub fn is_pem(&self) -> &bool{
        &self.is_pem
    }

    /// Get the domain for the file `std::string::String`
    pub fn get_domain(&self) -> &String{
        &self.domain
    }

    /// Create teh new SSL config
    /// # Arguments
    /// * `key_file` - Key file path `std::string::String`
    /// * `cert_file`- Certification file path `std::string::String`
    /// * `ca_certs` - CA Certifications file path
    /// * `cert_reqs` - Require certification
    /// * `is_pem` - Certification type is PEM
    /// * `domain`- Domain for the SSL file
    pub fn new(key_file: String, cert_file: String, ca_certs: String, cert_reqs: bool, is_pem: bool, domain: String) -> SSLConfig{
        SSLConfig{
            key_file: key_file,
            cert_file: cert_file,
            ca_certs: ca_certs,
            cert_reqs: cert_reqs,
            is_pem: is_pem,
            domain: domain,
        }
    }

}
