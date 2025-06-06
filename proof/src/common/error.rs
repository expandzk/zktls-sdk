use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZkTlsClientError {
    #[error("template not found")]
    TemplateNotFound,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
