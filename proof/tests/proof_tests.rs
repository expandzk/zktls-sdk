use zktls_sdk_proof::{
    ProofRequestParams, ZkTlsClient, ZkTlsClientConfig, template::HttpTemplateName,
};

#[tokio::test]
async fn test_client_creation() {
    let config = ZkTlsClientConfig::builder().build();
    let client = ZkTlsClient::new(config);
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_template_loading() {
    let config = ZkTlsClientConfig::builder().build();
    let client = ZkTlsClient::new(config).unwrap();

    let template = client.get_template(HttpTemplateName::CoinGeckoTokenPrice);
    assert!(template.is_ok());

    let template = template.unwrap();
    assert_eq!(template.provider, "http");
    assert_eq!(template.parameters.method, "GET");
    assert!(template.parameters.url.contains("coingecko.com"));
}

#[tokio::test]
async fn test_generate_proof() {
    let config = ZkTlsClientConfig::builder().build();
    let client = ZkTlsClient::new(config).unwrap();

    let params = ProofRequestParams::builder()
        .template_name(HttpTemplateName::CoinGeckoTokenPrice)
        .build();

    let result = client.generate_proof(params).await;
    assert!(result.is_ok());
}
