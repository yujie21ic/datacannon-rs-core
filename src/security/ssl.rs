/*
UAA  Configuration

Author Andrew Evans
*/

/// SSL Configuration
#[derive(Clone, Debug)]
pub struct SSLConfig{
    key_file: String,
    cert_file: String,
    ca_certs: String,
    cert_reqs: bool,
    is_pem: bool,
    domain: String,
}


impl SSLConfig{

    pub fn get_key_file(&self) -> &String{
        &self.key_file
    }

    pub fn get_cert_file(&self) -> &String{
        &self.cert_file
    }

    pub fn get_ca_certs(&self) -> &String{
        &self.ca_certs
    }

    pub fn get_cert_reqs(&self) -> &bool{
        &self.cert_reqs
    }

    pub fn is_pem(&self) -> &bool{
        &self.is_pem
    }

    pub fn get_domain(&self) -> &String{
        &self.domain
    }

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
