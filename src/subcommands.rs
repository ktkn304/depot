use clap;
use crate::{config::Config, utils::GenericResult};

mod root;
mod list;
mod get;
mod create;
mod resolve;
mod get_overload;
mod external;

pub trait Subcommand {
    fn run(&self, config: &Config) -> GenericResult<i32>;
}

macro_rules! as_item {
    ($i:item) => { $i };
}

macro_rules! define_subcommands_enum {
    ($(($name:ident, $mod:tt);)+) => {
        as_item! {
            #[derive(clap::Subcommand)]
            pub enum Subcommands {
                $(
                    #[clap(about = $mod::ABOUT)]
                    $name($mod::Subcommand),
                )+
                #[clap(external_subcommand)]
                External(Vec<String>),
            }
        }
    };
}

macro_rules! define_subcommands_run {
    ($(($name:ident, $mod:tt);)+) => {
        fn run_subcommand(config: &Config, command: Subcommands) -> GenericResult<i32> {
            match command {
                $(
                    Subcommands::$name(cmd) => {
                        cmd.run(config)
                    }
                )+
                Subcommands::External(args) => {
                    let cmd = external::Subcommand { args: args };
                    cmd.run(config)
                }
            }
        }
    };
}

macro_rules! define_subcommands {
    ($(($name:ident, $mod:tt);)+) => (
        define_subcommands_enum!($(($name, $mod);)+);
        define_subcommands_run!($(($name, $mod);)+);
    );
}

define_subcommands!{
    (Root, root);
    (List, list);
    (Resolve, resolve);
    (GetOverload, get_overload);
    (Get, get);
    (Create, create);
}

pub fn run(config: &Config, command: Subcommands) -> i32 {
    match run_subcommand(config, command) {
        Ok(return_code) => {
            return return_code;
        },
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        }
    }
}
