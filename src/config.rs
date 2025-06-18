use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

pub mod behavior;
pub mod directory_condition;
pub mod fields_definition;
pub mod generator;
pub mod overload;
pub mod parse;
pub mod pattern;
pub mod resolve;
pub mod shell;

use self::{
    behavior::Behavior,
    directory_condition::DirectoryCondition,
    fields_definition::FieldsDefinition,
    generator::Generator,
    overload::{Overloadable, OverloadsContainer},
    parse::Parse,
    pattern::Pattern,
    resolve::Resolve,
    shell::Shell,
};

#[derive(Deserialize)]
pub struct Config {
    pub core: Core,
    #[serde(default)]
    pub shell: Shell,
    pub parse: Parse,
    pub resolve: Resolve,
    pub subcommands: Subcommands,
    #[serde(default)]
    pub overloads: OverloadsContainer,
}

#[derive(Deserialize)]
pub struct Core {
    pub root: Generator,
}

#[derive(Deserialize)]
pub struct Subcommands {
    pub get: Get,
    pub create: Create,
    #[serde(rename = "move")]
    pub mv: Move,
    #[serde(default)]
    pub list: List,
}

pub type Get = Overloadable<GetParams>;
#[derive(Deserialize)]
pub struct GetParams {
    pub command: Behavior,
}
impl Default for GetParams {
    fn default() -> Self {
        Self {
            command: Default::default(),
        }
    }
}

type Create = Get;

pub type Move = Overloadable<MoveParams>;
#[derive(Deserialize)]
pub struct MoveParams {
    #[serde(default = "behavior::nop")]
    pub pre_command: Behavior,
    pub command: Behavior,
}
impl Default for MoveParams {
    fn default() -> Self {
        Self {
            pre_command: Default::default(),
            command: Default::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct List {
    pub project: Project,
    #[serde(default)]
    pub fields: FieldsDefinition,
}
impl Default for List {
    fn default() -> Self {
        Self {
            project: Default::default(),
            fields: Default::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct Project {
    pub condition: DirectoryCondition,
    pub excludes: Vec<Pattern>,
}
impl Default for Project {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            excludes: Default::default(),
        }
    }
}

pub fn load_from_file(config_file: PathBuf) -> Result<Config, Box<dyn Error>> {
    let mut f = File::open(config_file)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;

    return Ok(config);
}
