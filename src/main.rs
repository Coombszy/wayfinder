use std::process::exit;

use clap::{Parser, ValueEnum};
use log::{error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};

use wayfinder_shared::Config;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Registrars {
    Godaddy,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Domain name
    #[arg(short, long)]
    domain: String,

    /// Record name(s)
    #[arg(short, long)]
    record: Vec<String>,

    /// Domain Registrar
    #[arg(short = 'p', long)]
    registrar: Registrars,

    /// Authentication secret
    #[arg(short = 's', long)]
    auth_secret: String,

    /// Authentication key (Required for registrars: Godaddy)
    #[arg(short = 'k', long)]
    auth_key: Option<String>,

    /// Time between checks/updates
    #[arg(short, long, default_value_t = 30)]
    wait: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    startup();
    let c = Config {
        auth_key: args.auth_key,
        auth_secret: args.auth_secret,
        domain: args.domain,
        records: args.record,
        wait: args.wait,
    };

    match args.registrar {
        Registrars::Godaddy => {
            if let Err(e) = wayfinder_godaddy::main(&c).await {
                error!("{}", e);
                exit(1);
            }
        }
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
