/*
Configuration for backends

Author Andrew Evans
*/


#[derive(Clone, Debug)]
pub struct BackendConfig{
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub transport_options: Option<String>,
}