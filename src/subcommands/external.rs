use std::process::Command;

use crate::config::Config;
use crate::store::Store;
use crate::utils::GenericResult;
use clap::Args;

#[derive(Args)]
pub struct Subcommand {
    pub args: Vec<String>,
}

impl super::Subcommand for Subcommand {
    fn run(&self, config: &Config, mut store: impl Store) -> GenericResult<i32> {
        let cmdgen = config.shell.compile(&store)?;

        let root_path = config.core.root.expand(&cmdgen, &store)?;
        store.set_root_path(root_path.clone());

        let program = format!("depot-{}", self.args[0]);
        let mut cmd = Command::new(program);
        cmd.args(&self.args[1..]);
        cmd.envs(store.iter());
        let mut process = cmd.spawn()?;
        let exit_status = process.wait()?;
        return Ok(exit_status.code().unwrap_or(1));
    }
}
