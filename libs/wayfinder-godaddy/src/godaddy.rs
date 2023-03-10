use log::debug;

use serde::{Deserialize, Serialize};
use wayfinder_shared::Config;

use crate::{GodaddyArgs, GodaddyError};

const GODADDY_URL: &str = "https://api.godaddy.com";

/// Clones api key from config and formats it for Godaddy api.
fn get_auth(args: &GodaddyArgs) -> String {
    format!(
        "sso-key {}:{}",
        args.auth_key.clone(),
        args.auth_secret.clone()
    )
}

/// Used for validating credentials are correct
/// TODO: Return body, Currently only used for validating that auth credentials are correct.
pub async fn get_all_domains(args: &GodaddyArgs) -> Result<(), GodaddyError> {
    debug!("Getting all domains");
    let url: String = format!("{GODADDY_URL}/v1/domains");
    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_auth(args))
        .send()
        .await?;

    debug!("Response: {:?}", response);

    if !response.status().is_success() {
        // Throw a nicer error
        if response.status() == 401 {
            return Err(GodaddyError::Auth);
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
pub async fn get_domain(config: &Config, args: &GodaddyArgs) -> Result<Domain, GodaddyError> {
    debug!("Getting domain, {}", config.domain);
    let url: String = format!("{GODADDY_URL}/v1/domains/{}", config.domain);
    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_auth(args))
        .send()
        .await
        .unwrap();

    debug!("Response: {:?}", response);

    if !response.status().is_success() {
        return Err(GodaddyError::GenericHttp(response.status().to_string()));
    }

    match response.json::<Domain>().await {
        Ok(d) => Ok(d),
        Err(e) => Err(GodaddyError::RequestFailed(e)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainData {
    pub data: String,
    pub name: String,
}

pub async fn update_domain_record(
    config: &Config,
    args: &GodaddyArgs,
    record: &String,
    data: &Vec<DomainData>,
) -> Result<(), GodaddyError> {
    debug!("Updating domain record(s), {}", config.domain);
    let url: String = format!(
        "{GODADDY_URL}/v1/domains/{}/records/A/{}",
        config.domain, record
    );

    let response = reqwest::Client::new()
        .put(url)
        .header("Authorization", get_auth(args))
        .json(&data)
        .send()
        .await
        .unwrap();

    debug!("Response: {:?}", response);

    if !response.status().is_success() {
        Err(GodaddyError::GenericHttp(response.status().to_string()))
    } else {
        Ok(())
    }
}

pub async fn get_domain_record(
    config: &Config,
    args: &GodaddyArgs,
    record: &String,
) -> Result<Vec<DomainData>, GodaddyError> {
    debug!("Getting domain record, {}", config.domain);
    let url: String = format!(
        "{GODADDY_URL}/v1/domains/{}/records/A/{}",
        config.domain, record
    );

    let response = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_auth(args))
        .send()
        .await
        .unwrap();

    debug!("Response: {:?}", response);

    if !response.status().is_success() {
        return Err(GodaddyError::GenericHttp(response.status().to_string()));
    }

    match response.json::<Vec<DomainData>>().await {
        Ok(d) => Ok(d),
        Err(e) => Err(GodaddyError::RequestFailed(e)),
    }
}
