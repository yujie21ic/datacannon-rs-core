/*
UAA Server Configuration

Author Andrew Evans
*/


/// UAA server configuration
#[derive(Clone, Debug)]
pub struct UAAConfig{
    auth_url: String,
    client_id: Option<String>,
    client_secret: Option<String>,
    token_url: Option<String>,
}
