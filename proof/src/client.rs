use crate::{
    ZkTlsClientConfig, ZkTlsClientError,
    template::{HttpTemplateName, Templates},
};
use anyhow::Result;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct ProofRequestParams {
    pub template_name: HttpTemplateName,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ZkTlsClient {
    #[allow(dead_code)]
    config: ZkTlsClientConfig,
    templates: Templates,
}

impl ZkTlsClient {
    pub fn new(config: ZkTlsClientConfig) -> Result<Self, ZkTlsClientError> {
        let templates = Templates::new()?;

        Ok(Self::builder().config(config).templates(templates).build())
    }

    pub async fn generate_proof(
        &self,
        params: ProofRequestParams,
    ) -> Result<String, ZkTlsClientError> {
        let template = self.templates.get(params.template_name)?;

        let response_data = self.fetch_http_data(&template.parameters).await?;
        let proof = self.create_zk_proof(&response_data, template).await?;

        Ok(proof)
    }

    async fn fetch_http_data(
        &self,
        parameters: &crate::template::HttpParameters,
    ) -> Result<String, ZkTlsClientError> {
        let client = reqwest::Client::new();
        let response = client.get(&parameters.url).send().await?;

        let text = response.text().await?;
        Ok(text)
    }

    async fn create_zk_proof(
        &self,
        _data: &str,
        _template: &crate::template::HttpTemplate,
    ) -> Result<String, ZkTlsClientError> {
        //todo!()
        Ok("".to_string())
    }

    pub fn get_template(
        &self,
        name: HttpTemplateName,
    ) -> Result<&crate::template::HttpTemplate, ZkTlsClientError> {
        self.templates.get(name)
    }
}
