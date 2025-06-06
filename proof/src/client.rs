use std::collections::HashMap;

use crate::{
    ZkTlsClientConfig, ZkTlsClientError,
    template::{HttpTemplate, HttpTemplateName, Templates},
};
use anyhow::Result;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct ProofRequestParams {
    pub template_name: HttpTemplateName,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ZkTlsClient {
    config: ZkTlsClientConfig,
    templates: Templates,
}

impl ZkTlsClient {
    pub fn new(config: ZkTlsClientConfig) -> Result<Self, ZkTlsClientError> {
        let templates = Templates::new()?;
        Ok(Self::builder().config(config).templates(templates).build())
    }

    pub async fn generate_proof(&self, params: ProofRequestParams) -> Result<(), ZkTlsClientError> {
        todo!()
    }
}
