mod http;

pub use http::*;

use crate::ZkTlsClientError;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Templates {
    pub templates: HashMap<HttpTemplateName, HttpTemplate>,
}

impl Templates {
    pub fn new() -> Result<Self, ZkTlsClientError> {
        let templates = HashMap::from([(
            HttpTemplateName::CoinGeckoTokenPrice,
            HttpTemplate::load(HttpTemplateName::CoinGeckoTokenPrice)?,
        )]);
        Ok(Self::builder().templates(templates).build())
    }

    pub fn get(&self, name: HttpTemplateName) -> Result<&HttpTemplate, ZkTlsClientError> {
        self.templates
            .get(&name)
            .ok_or(ZkTlsClientError::TemplateNotFound)
    }
}
