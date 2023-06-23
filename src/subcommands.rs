use clap;
use crate::{config::Config, utils::GenericResult};

mod root;
mod list;
mod get;
mod create;
mod resolve;
mod get_overload;
mod external;

pub trait BuiltInCommand {
    fn run(&self, config: &Config) -> GenericResult<i32>;
}

#[derive(clap::Subcommand)]
pub enum Subcommands {
    Root(root::Subcommand),
    List(list::Subcommand),
    Resolve(resolve::Subcommand),
    GetOverload(get_overload::Subcommand),
    Get(get::Subcommand),
    Create(create::Subcommand),

    #[clap(external_subcommand)]
    External(Vec<String>),
}

pub fn run(config: &Config, command: Subcommands) -> i32 {
    match command {
        Subcommands::Root(cmd) => {
            return run_builtin_command(&cmd, config);
        }
        Subcommands::List(cmd) => {
            return run_builtin_command(&cmd, config);
        }
        Subcommands::Resolve(cmd) => {
            return run_builtin_command(&cmd, config);
        }
        Subcommands::GetOverload(cmd) => {
            return run_builtin_command(&cmd, config);
        }
        Subcommands::Get(cmd) => {
            return run_builtin_command(&cmd, config);
        }
        Subcommands::Create(cmd) => {
            return run_builtin_command(&cmd, config);
        }
        Subcommands::External(args) => {
            let cmd = external::Subcommand { args: args };
            return run_builtin_command(&cmd, config);
        }
    }
}

fn run_builtin_command(cmd: &dyn BuiltInCommand, config: &Config) -> i32 {
    match cmd.run(config) {
        Ok(return_code) => {
            return return_code;
        },
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        }
    }
}
