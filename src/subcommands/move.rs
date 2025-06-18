use crate::config::behavior::Behavior;
use crate::config::Config;
use crate::store::Store;
use crate::utils;
use crate::utils::GenericResult;
use clap::Args;

pub const ABOUT: &str = "move directory";

#[derive(Args)]
pub struct Subcommand {
    #[clap(short = 'r', long, default_value_t = false)]
    resolve_source: bool,
    source: String,
    address: String,
}

impl super::Subcommand for Subcommand {
    fn run(&self, config: &Config, mut store: impl Store) -> GenericResult<i32> {
        let cmdgen = config.shell.compile(&store)?;

        let root_path = config.core.root.expand(&cmdgen, &store)?;
        store.set_root_path(root_path.clone());

        let pre_command: &Behavior;
        if self.resolve_source {
            let mut tmp_store = store.clone();
            let source_url = config.parse.parse_url(&self.source)?;
            store.set_source_remote_raw(self.source.clone());
            store.set_source_remote_url(&source_url);
            tmp_store.set_root_path(root_path.clone());
            tmp_store.set_remote_raw(&self.source);
            tmp_store.set_remote_url(&source_url);
            let src_overload = config.overloads.find_overload_name(&self.source)?;
            let rel_path = config
                .resolve
                .expand_path(&cmdgen, &tmp_store, src_overload)?;
            let path = utils::concat_path(&root_path, &rel_path);
            store.set_source_local_path(path, rel_path);
            pre_command = &config.subcommands.mv.get_params(src_overload).pre_command;
        } else {
            let rel_local_path = String::default();
            store.set_source_local_path(self.source.to_owned(), rel_local_path);
            pre_command = &config.subcommands.mv.get_params(None).pre_command;
        }

        let dst_overload = config.overloads.find_overload_name(&self.address)?;
        let remote_url = config.parse.parse_url(&self.address)?;
        store.set_remote_raw(&self.address);
        store.set_remote_url(&remote_url);

        let rel_path = config.resolve.expand_path(&cmdgen, &store, dst_overload)?;
        let path = utils::concat_path(&root_path, &rel_path);
        store.set_local_path(path, rel_path);

        let return_code = pre_command.execute(&cmdgen, &store)?;
        if return_code == 0 {
            config
                .subcommands
                .mv
                .get_params(dst_overload)
                .command
                .execute(&cmdgen, &store)
        } else {
            Ok(return_code)
        }
    }
}
