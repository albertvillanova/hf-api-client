use hf_api_client::HfApiClient;

#[test]
fn test_hf_api_client_constructor() {
    let hf_api_client = HfApiClient::new();
    assert_eq!(hf_api_client.base_url, "https://huggingface.co/api");
}
