use crate::ZkTlsClientError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum HttpTemplateName {
    CoinGeckoTokenPrice,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpTemplate {
    pub provider: String,
    pub parameters: HttpParameters,
    pub timestamp: u64,
    pub owner: String,
    pub context: HttpContext,
}

impl HttpTemplate {
    pub fn load(name: HttpTemplateName) -> Result<Self, ZkTlsClientError> {
        match name {
            HttpTemplateName::CoinGeckoTokenPrice => {
                let file = include_str!("../../files/coin-gecko.json");
                let template: HttpTemplate = serde_json::from_str(file)?;
                Ok(template)
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpParameters {
    pub method: String,
    pub url: String,
    #[serde(alias = "responseMatches")]
    pub response_matches: Vec<ResponseMatch>,
    #[serde(alias = "responseRedactions")]
    pub response_redactions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseMatch {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpContext {
    pub name: String,
    pub description: Vec<String>,
    #[serde(alias = "secretParams")]
    pub secret_params: SecretParams,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecretParams {
    pub headers: std::collections::HashMap<String, String>,
}
