use std::thread;
use std::time::Duration;

use godaddy::get_domain_record;
use log::info;
use thiserror::Error;
use wayfinder_shared::{get_external_ip, Config, IpifyError, WayfinderError};

use crate::godaddy::update_domain_records;

mod godaddy;

pub async fn main(config: &Config) -> Result<(), WayfinderError<GodaddyError>> {
    validate_args(config)?;
    check_auth(config).await?;
    validate(config).await?;

    loop {
        tick(config).await?;
        thread::sleep(Duration::from_secs(config.wait));
    }
}

fn validate_args(config: &Config) -> Result<(), WayfinderError<GodaddyError>> {
    let mut error: Vec<String> = Vec::new();

    if config.auth_key.is_none() {
        error.push("Authentication key is requried for Godaddy".to_owned());
    }

    // If no errors
    if error.is_empty() {
        return Ok(());
    }

    Err(WayfinderError::Godaddy(GodaddyError::InvalidArgs(
        error.join(", "),
    )))
}

/// Check that credentials provided are correct.
async fn check_auth(config: &Config) -> Result<(), WayfinderError<GodaddyError>> {
    match godaddy::get_all_domains(config).await {
        Ok(_) => info!("Godaddy auth successful!"),
        Err(e) => return Err(WayfinderError::Godaddy(e)),
    }

    Ok(())
}

/// Validates that prequisites to manage the domain are in place.
async fn validate(config: &Config) -> Result<(), WayfinderError<GodaddyError>> {
    // if this fails, assume domain does not exist in user account
    match godaddy::get_domain(config).await {
        Ok(_) => (),
        Err(e) => return Err(WayfinderError::Godaddy(e)),
    };

    Ok(())
}

///
async fn tick(config: &Config) -> Result<(), WayfinderError<GodaddyError>> {
    let external = match get_external_ip().await {
        Ok(ip) => ip,
        Err(e) => return Err(WayfinderError::Godaddy(GodaddyError::ExternalIp(e))),
    };
    for record in &config.records {
        let mut entries = match get_domain_record(config, record).await {
            Ok(d) => d,
            Err(e) => return Err(WayfinderError::Godaddy(e)),
        };

        // If any of the entries need updating update all!
        for mut entry in entries.iter_mut() {
            if entry.data != external {
                info!(
                    "Updating '{}.{}'... {} -> {}",
                    record, config.domain, entry.data, external
                );
                entry.data = external.clone();
                match update_domain_records(config, record, &vec![entry.clone()]).await {
                    Ok(_) => break,
                    Err(e) => return Err(WayfinderError::Godaddy(e)),
                }
            }
        }
    }
    Ok(())
}

/// All error types for godaddy
#[derive(Error, Debug)]
pub enum GodaddyError {
    #[error("Invalid arguements: {0}")]
    InvalidArgs(String),
    #[error("Godaddy authentication failed!")]
    Auth,
    #[error("Domain in config does not exist!")]
    InvalidDomain,
    #[error("Generic http error, {0}")]
    GenericHttp(String),
    #[error("Request failed, {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Fetch external IP")]
    ExternalIp(#[from] IpifyError),
}
