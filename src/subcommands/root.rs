use clap::Args;
use crate::config::Config;
use crate::store::EnvironmentStore;
use crate::utils::GenericResult;

#[derive(Args)]
pub struct Subcommand {
}

impl super::BuiltInCommand for Subcommand {
    fn run(&self, config: &Config) -> GenericResult<i32> {
        let store = EnvironmentStore::new();
        let cmdgen = config.shell.compile(&store)?;
        let root_path = config.core.root.expand(&cmdgen, &store)?;
        println!("{}", root_path);
        return Ok(0);
    }
}
