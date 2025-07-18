use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process;

mod config;
mod error;
mod store;
mod subcommands;
mod template;
mod utils;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    #[clap(short = 'e', long = "reset_env", default_value_t = false)]
    pub reset_environment: bool,

    #[clap(subcommand)]
    subcommand: subcommands::Subcommands,
}

fn main() {
    let cli = Cli::parse();
    let config_file: PathBuf;

    if let Some(path) = cli.config {
        config_file = path;
    } else if let Ok(config_file_s) = env::var("DEPOT_CONFIG") {
        config_file = PathBuf::from(config_file_s);
    } else {
        let path = utils::expand_env("${HOME}/.depotconfig.toml");
        config_file = PathBuf::from(path);
    }

    let config = config::load_from_file(config_file).unwrap_or_else(|err| {
        eprintln!("config file load failed: {}", err);
        process::exit(1)
    });

    let store = if cli.reset_environment {
        store::EnvironmentStore::new_env()
    } else {
        store::EnvironmentStore::new_blank()
    };

    let return_code = subcommands::run(&config, cli.subcommand, store);
    process::exit(return_code);
}
