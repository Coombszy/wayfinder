use reqwest;
use serde::Deserialize;
use wayfinder_shared::Config;

use crate::GodaddyError;

const GODADDY_URL: &str = "https://api.godaddy.com";

/// Clones api key from config and formats it for Godaddy api.
fn get_auth(config: &Config) -> String {
    format!(
        "sso-key {}:{}",
        config.auth_key.clone().unwrap().as_mut(),
        config.auth_secret.clone()
    )
}

/// Used for validating credentials are correct
/// TODO: Return body, Currently only used for validating that auth credentials are correct.
pub async fn get_all_domains(config: &Config) -> Result<(), GodaddyError> {
    let url: String = format!("{GODADDY_URL}/v1/domains");
    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_auth(config))
        .send()
        .await?;

    if !response.status().is_success() {
        // Throw a nicer error
        if response.status() == 401 {
            return Err(GodaddyError::Auth());
        } else {
            return Err(GodaddyError::GenericHttp(response.status().to_string()));
        }
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Domain {
    pub domain: String,
    #[serde(rename = "domainId")]
    pub domain_id: i32,
    pub status: String,
}

/// Gets infomation about domain in config.
pub async fn get_domain(config: &Config) -> Result<Domain, GodaddyError> {
    let url: String = format!("{GODADDY_URL}/v1/domains/{}", config.domain);
    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_auth(config))
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        return Err(GodaddyError::GenericHttp(response.status().to_string()));
    }

    match response.json::<Domain>().await {
        Ok(d) => Ok(d),
        Err(e) => Err(GodaddyError::RequestFailed(e)),
    }
}
