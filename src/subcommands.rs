use crate::{config::Config, store::Store, utils::GenericResult};
use clap;

mod create;
mod external;
mod get;
mod get_overload;
mod list;
mod r#move;
mod resolve;
mod root;

pub trait Subcommand {
    fn run(&self, config: &Config, store: impl Store) -> GenericResult<i32>;
}

macro_rules! as_item {
    ($i:item) => {
        $i
    };
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
        fn run_subcommand(config: &Config, command: Subcommands, store: impl Store) -> GenericResult<i32> {
            match command {
                $(
                    Subcommands::$name(cmd) => {
                        cmd.run(config, store)
                    }
                )+
                Subcommands::External(args) => {
                    let cmd = external::Subcommand { args: args };
                    cmd.run(config, store)
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

define_subcommands! {
    (Root, root);
    (List, list);
    (Resolve, resolve);
    (GetOverload, get_overload);
    (Get, get);
    (Create, create);
    (Move, r#move);
}

pub fn run(config: &Config, command: Subcommands, store: impl Store) -> i32 {
    match run_subcommand(config, command, store) {
        Ok(return_code) => {
            return return_code;
        }
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        }
    }
}
