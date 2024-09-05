use reqwest::Client;

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
}
