use log::debug;
use thiserror::Error;

const IPIFY: &str = "https://api.ipify.org";

#[derive(Error, Debug)]
pub enum WayfinderError<T> {
    #[error("{0}")]
    Ipify(T),
    #[error("{0}")]
    Godaddy(T),
}

pub struct Config {
    pub auth_key: Option<String>,
    pub auth_secret: String,
    pub domain: String,
    pub records: Vec<String>,
    pub wait: u64,
}

pub async fn get_external_ip() -> Result<String, IpifyError> {
    debug!("Getting external ip (ipify.org)");
    let url: String = IPIFY.to_owned();

    let response = reqwest::Client::new().get(url).send().await.unwrap();

    debug!("Response: {:?}", response);

    if !response.status().is_success() {
        return Err(IpifyError::GenericHttp(response.status().to_string()));
    }

    match response.text().await {
        Ok(d) => {
            debug!("Got external ip '{}' (ipify.org)", d);
            Ok(d)
        }
        Err(e) => Err(IpifyError::RequestFailed(e)),
    }
}

/// All error types for godaddy
#[derive(Error, Debug)]
pub enum IpifyError {
    #[error("Generic http error, {0}")]
    GenericHttp(String),
    #[error("Request failed, {0}")]
    RequestFailed(#[from] reqwest::Error),
}
