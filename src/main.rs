use std::process::exit;

use clap::{Parser, Subcommand};
use log::{error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};

use wayfinder_godaddy::GodaddyArgs;
use wayfinder_shared::Config;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    /// Domain name
    #[arg(short, long)]
    domain: String,

    /// Record name(s)
    #[arg(short, long, required = true)]
    record: Vec<String>,

    /// Domain Registrar
    #[command(subcommand)]
    registrar: Registrars,

    /// Time between checks/updates
    #[arg(short, long, default_value_t = 30)]
    wait: u64,
}

impl From<Args> for Config {
    fn from(val: Args) -> Self {
        Config {
            domain: val.domain,
            records: val.record,
            wait: val.wait,
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Registrars {
    /// Manage domains registered with Godaddy
    Godaddy(GodaddyArgs),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    startup();
    let config: Config = args.clone().into();

    match args.registrar {
        Registrars::Godaddy(u_args) => {
            if let Err(e) = wayfinder_godaddy::main(&config, &u_args).await {
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
