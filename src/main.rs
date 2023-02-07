use std::{error::Error, process::exit};

use wayfinder_shared::{Config, WayfindError};

#[tokio::main]
async fn main() {
    startup();
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

fn startup() {
    let ascii_name = r#"     __    __             __ _           _           
    / / /\ \ \__ _ _   _ / _(_)_ __   __| | ___ _ __ 
    \ \/  \/ / _` | | | | |_| | '_ \ / _` |/ _ \ '__|
     \  /\  / (_| | |_| |  _| | | | | (_| |  __/ |   
      \/  \/ \__,_|\__, |_| |_|_| |_|\__,_|\___|_|   
                   |___/                             "#;
    println!("{} v{}", &ascii_name, &env!("CARGO_PKG_VERSION"));
    println!("================================================================");


CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
}
