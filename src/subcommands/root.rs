use crate::config::Config;
use crate::store::Store;
use crate::utils::GenericResult;
use clap::Args;

pub const ABOUT: &str = "get root directory path";

#[derive(Args)]
pub struct Subcommand {}

impl super::Subcommand for Subcommand {
    fn run(&self, config: &Config, store: impl Store) -> GenericResult<i32> {
        let cmdgen = config.shell.compile(&store)?;
        let root_path = config.core.root.expand(&cmdgen, &store)?;
        println!("{}", root_path);
        return Ok(0);
    }
}
