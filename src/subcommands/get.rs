use clap::Args;
use crate::config::Config;
use crate::store::{EnvironmentStore, Store};
use crate::utils;
use crate::utils::GenericResult;

#[derive(Args)]
pub struct Subcommand {
    address: String
}

impl super::BuiltInCommand for Subcommand {
    fn run(&self, config: &Config) -> GenericResult<i32> {
        let mut store = EnvironmentStore::new();
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

        store.export_all();

        config.subcommands.get.get_params(overload).command.execute(&cmdgen, &store)
    }
}
