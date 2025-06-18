use crate::config::Config;
use crate::store::Store;
use crate::utils::GenericResult;
use clap::Args;

pub const ABOUT: &str = "get overload from url";

#[derive(Args)]
pub struct Subcommand {
    address: String,
}

impl super::Subcommand for Subcommand {
    fn run(&self, config: &Config, _store: impl Store) -> GenericResult<i32> {
        let overload = config.overloads.find_overload_name(&self.address)?;
        if let Some(name) = overload {
            println!("{}", name);
        } else {
            println!("(no overload)");
        }
        return Ok(0);
    }
}
