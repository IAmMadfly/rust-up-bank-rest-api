use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

pub struct UpClient {
    req_client: Client,
    base_url: &'static str,
}

impl UpClient {
    pub fn new(token: String) -> Result<Self, String> {
        let auth =
            HeaderValue::from_str(&format!("Bearer {token}")).map_err(|err| err.to_string())?;

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", auth);

        let req_client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| err.to_string())?;

        Ok(UpClient {
            req_client,
            base_url: "https://api.up.com.au/api/v1/",
        })
    }
}
