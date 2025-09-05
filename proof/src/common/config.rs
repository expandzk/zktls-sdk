use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Deserialize, Serialize, TypedBuilder)]
pub struct ZkTlsClientConfig {
    #[serde(default = "default_server_url")]
    #[builder(default = default_server_url())]
    pub server_url: String,

    #[serde(default = "default_response_cache_ttl")]
    #[builder(default = default_response_cache_ttl())]
    pub response_cache_ttl: u64,

    #[serde(default = "default_proof_cache_ttl")]
    #[builder(default = default_proof_cache_ttl())]
    pub proof_cache_ttl: u64,

    #[serde(default = "default_max_cache_size")]
    #[builder(default = default_max_cache_size())]
    pub max_cache_size: usize,

    #[serde(default = "default_enable_cache")]
    #[builder(default = default_enable_cache())]
    pub enable_cache: bool,
}

fn default_server_url() -> String {
    "ws://0.0.0.0:8001/ws".to_string()
}

fn default_response_cache_ttl() -> u64 {
    300
}

fn default_proof_cache_ttl() -> u64 {
    3600
}

fn default_max_cache_size() -> usize {
    1000
}

fn default_enable_cache() -> bool {
    true
}
