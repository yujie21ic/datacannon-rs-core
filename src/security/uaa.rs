//! UAA Server Configuration
//!
//! ---
//! author: Andrew Evans
//! ---


/// UAA server configuration
///
/// # Arguments
/// * `auth_url` - Authorization URL
/// * `client_id` - Optional client id
/// * `client_secret` - Optional client secret
/// * `token_url` - Optional token url
#[derive(Clone, Debug)]
pub struct UAAConfig{
    auth_url: String,
    client_id: Option<String>,
    client_secret: Option<String>,
    token_url: Option<String>,
}
