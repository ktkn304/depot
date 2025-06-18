use crate::config::Config;
use crate::store::Store;
use crate::utils;
use crate::utils::GenericResult;
use clap::Args;

pub const ABOUT: &str = "create directory from other resource";

#[derive(Args)]
pub struct Subcommand {
    address: String,
}

impl super::Subcommand for Subcommand {
    fn run(&self, config: &Config, mut store: impl Store) -> GenericResult<i32> {
        let overload = config.overloads.find_overload_name(&self.address)?;
        let cmdgen = config.shell.compile(&store)?;
        let remote_url = config.parse.parse_url(&self.address)?;
        store.set_remote_raw(&self.address);
        store.set_remote_url(&remote_url);

        let root_path = config.core.root.expand(&cmdgen, &store)?;
        store.set_root_path(root_path.clone());

        let rel_path = config.resolve.expand_path(&cmdgen, &store, overload)?;
        let path = utils::concat_path(&root_path, &rel_path);
        store.set_local_path(path, rel_path);

        config
            .subcommands
            .get
            .get_params(overload)
            .command
            .execute(&cmdgen, &store)
    }
}
