use std::{error::Error, process::exit};

use log::{error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};
use wayfinder_shared::{Config, WayfindError};

#[tokio::main]
async fn main() {
    startup();
    let c = Config {
        auth_key: Some("".to_owned()),
        auth_secret: "".to_owned(),
        domain: "madvibes.uk".to_owned(),
        records: vec!["@".to_owned()],
        wait: 30,
    };

    // For now, run godaddy by default
    if let Err(e) = wayfinder_godaddy::main(&c).await {
        error!("{}", e);
        exit(1);
    }
}

fn startup() {
    let ascii_name = r#"     __    __             __ _           _           
    / / /\ \ \__ _ _   _ / _(_)_ __   __| | ___ _ __ 
    \ \/  \/ / _` | | | | |_| | '_ \ / _` |/ _ \ '__|
     \  /\  / (_| | |_| |  _| | | | | (_| |  __/ |   
      \/  \/ \__,_|\__, |_| |_|_| |_|\__,_|\___|_|   
                   |___/                             "#;
    println!("{} v{}", &ascii_name, &env!("CARGO_PKG_VERSION"));
    println!("================================================================");

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
