use hf_api_client::HfApiClient;
use tokio;

#[test]
fn test_hf_api_client_constructor() {
    let hf_api_client = HfApiClient::new();
    assert_eq!(hf_api_client.base_url, "https://huggingface.co/api");
}

#[tokio::test]
async fn test_hf_api_client_list_repo_tree() {
    let hf_api_client = HfApiClient::new();
    let repo_tree = hf_api_client.list_repo_tree().await.unwrap();
    assert_eq!(repo_tree.len(), 2);
    for item in repo_tree.iter() {
        if item.r#type == "directory" {
            assert_eq!(item.oid, "712e1efe29982b3c5b4291765716d42e6bdeadad");
            assert_eq!(item.size, 0);
            assert_eq!(item.path, "data");
        }
        else {
            assert_eq!(item.r#type, "file");
            assert_eq!(item.oid, "07f0db3339ad9053dc95b284c4ae14e014efff89");
            assert_eq!(item.size, 690);
            assert_eq!(item.path, ".gitattributes");
        }
    }
}
