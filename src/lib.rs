use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoTreeItem {
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

    pub async fn list_repo_tree(&self) -> Result<Vec<RepoTreeItem>, reqwest::Error> {
        let url = format!("{}/datasets/hf-internal-testing/dataset_with_data_files/tree/main", self.base_url);
        let response = self.client.get(&url).send().await?;
        let repo_tree_items: Vec<RepoTreeItem> = response.json().await?;
        Ok(repo_tree_items)
    }
}
