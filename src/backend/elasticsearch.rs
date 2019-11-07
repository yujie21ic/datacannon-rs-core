/*
Elasticsearch configuration and utilities for a backend

Author Andrew Evans
*/


use crate::backend::config::BackendConfig;
use crate::security::ssl::SSLConfig;
use std::collections::HashMap;


#[derive(Clone, Debug)]
pub struct ElasticHandler{
    pub conn_inf: BackendConfig,
    pub ssl_config: Option<SSLConfig>,
    //fields: Option<HashMap<String, Field>>,
}

impl ElasticHandler{

}
