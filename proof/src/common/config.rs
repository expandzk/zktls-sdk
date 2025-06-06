use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Deserialize, Serialize, TypedBuilder)]
pub struct ZkTlsClientConfig {
    #[serde(default = "default_server_url")]
    #[builder(default = default_server_url())]
    pub server_url: String,
}

fn default_server_url() -> String {
    "ws://0.0.0.0:8001/ws".to_string()
}
