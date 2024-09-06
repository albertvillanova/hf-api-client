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

    /// List the repo's tree (files and directories) at a given reference.
    ///
    /// # Example
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let repo_tree = hf_api_client::HfApiClient::new()
    ///     .list_repo_tree()
    ///     .await?;
    ///
    /// println!("{:#?}", repo_tree);
    /// # Ok(())
    /// # }
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
