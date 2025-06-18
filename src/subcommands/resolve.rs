use crate::config::generator::Generator;
use crate::config::Config;
use crate::store::Store;
use crate::utils::{self, GenericResult};
use clap::Args;

pub const ABOUT: &str = "get stored directory path from url";

#[derive(Args)]
pub struct Subcommand {
    #[clap(short, long)]
    template: Option<String>,
    address: String,
}

impl super::Subcommand for Subcommand {
    fn run(&self, config: &Config, mut store: impl Store) -> GenericResult<i32> {
        let cmdgen = config.shell.compile(&store)?;
        let remote_url = config.parse.parse_url(&self.address)?;
        store.set_remote_raw(&self.address);
        store.set_remote_url(&remote_url);

        let root_path = config.core.root.expand(&cmdgen, &store)?;
        store.set_root_path(root_path.clone());

        let rel_path = if let Some(template) = &self.template {
            let generator = Generator::Template(template.to_owned());
            generator.expand(&cmdgen, &store)?
        } else {
            let overload = config.overloads.find_overload_name(&self.address)?;
            config.resolve.expand_path(&cmdgen, &store, overload)?
        };
        let path = utils::concat_path(&root_path, &rel_path);
        println!("{}", &path);
        return Ok(0);
    }
}
