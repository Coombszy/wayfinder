use std::thread;
use std::time::Duration;

use thiserror::Error;
use wayfinder_shared::{Config, WayfindError};
use log::info;

mod godaddy;

pub async fn main(config: &Config) -> Result<(), WayfindError<GodaddyError>> {
    check_auth(config).await?;
    validate(config).await?;

    loop {
        info!("aaa");
        thread::sleep(Duration::from_secs(config.wait));
    }
}

/// Check that credentials provided are correct.
async fn check_auth(config: &Config) -> Result<(), WayfindError<GodaddyError>> {
    match godaddy::get_all_domains(config).await {
        Ok(_) => info!("Godaddy auth successful!"),
        Err(e) => return Err(WayfindError::Godaddy(e)),
    }

    Ok(())
}

/// Validates that prequisites to manage the domain are in place.
async fn validate(config: &Config) -> Result<(), WayfindError<GodaddyError>> {
    // if this fails, assume domain does not exist in user account
    match godaddy::get_domain(config).await {
        Ok(_) => (),
        Err(e) => return Err(WayfindError::Godaddy(e)),
    };

    Ok(())
}

/// All error types for godaddy
#[derive(Error, Debug)]
pub enum GodaddyError {
    #[error("Godaddy authentication failed!")]
    Auth(),
    #[error("Domain in config does not exist!")]
    InvalidDomain,
    #[error("Generic http error, {0}")]
    GenericHttp(String),
    #[error("Request failed, {0}")]
    RequestFailed(#[from] reqwest::Error),
}
