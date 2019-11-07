/*
UAA  Configuration

Author Andrew Evans
*/

/// SSL Configuration
#[derive(Clone, Debug)]
pub struct SSLConfig{
    key_file: String,
    certifle: String,
    ca_certs: String,
    cert_reqs: bool,
}
