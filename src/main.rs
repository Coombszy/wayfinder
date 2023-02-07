use std::{error::Error, process::exit};

use wayfinder_shared::{Config, WayfindError};

#[tokio::main]
async fn main() {
    let c = Config {
        auth_key: Some("".to_owned()),
        auth_secret: "".to_owned(),
        domain: "".to_owned(),
        wait: 20,
    };

    // For now, run godaddy by default
    if let Err(e) = wayfinder_godaddy::main(&c).await {
        eprintln!("{}", e);
        exit(1);
    }
}
