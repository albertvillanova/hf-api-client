use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoTree {
    pub r#type: String,
    pub oid: String,
    pub size: u64,
    pub path: String,
}

pub struct HfApiClient {
    client: Client,
    pub base_url: String,
}

impl HfApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: String::from("https://huggingface.co/api"),
        }
    }

    /// Lists the repo's tree (files and directories) at a given reference.
    ///
    /// # Example
    /// ```rust
    /// use hf_api_client::HfApiClient;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = HfApiClient::new();
    ///     let repo_tree = client.list_repo_tree().await.unwrap();
    ///     println!("{:#?}", repo_tree);
    /// }
    /// ```
    ///
    /// # Errors
    /// This function will return an error if the request fails or the response cannot be deserialized.
    ///
    /// # Panics
    /// This function will panic if the base URL is invalid.
    ///
    /// # Returns
    /// A vector of [`RepoTree`] objects representing the files and directories in the repo.
    pub async fn list_repo_tree(&self) -> Result<Vec<RepoTree>, reqwest::Error> {
        let url = format!("{}/datasets/hf-internal-testing/dataset_with_data_files/tree/main", self.base_url);
        let response = self.client.get(&url).send().await?;
        let repo_tree: Vec<RepoTree> = response.json().await?;
        Ok(repo_tree)
    }
}
