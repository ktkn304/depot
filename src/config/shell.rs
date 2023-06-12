use std::process::Command;

use serde::{Deserialize};

use crate::{store::Store, utils::{GenericResult, CommandGenerator}};

use super::generator::Generator;

#[derive(Deserialize)]
pub struct Shell {
    pub path: Generator,
    pub args: Vec<Generator>,
}
impl Default for Shell {
    fn default() -> Self {
        Self {
            path: Generator::String("/bin/sh".to_owned()),
            args: vec![Generator::String("-c".to_owned())],
        }
    }
}

impl Shell {
    pub fn compile<T: Store>(&self, store: &T) -> GenericResult<CompiledShell> {
        let path = self.path.expand_without_shell(store)?;
        let mut args = Vec::<String>::new();
        for arg in &self.args {
            args.push(arg.expand_without_shell(store)?);
        }

        Ok(CompiledShell {
            path: path,
            args: args,
        })
    }
}

pub struct CompiledShell {
    pub path: String,
    pub args: Vec<String>,
}
impl CommandGenerator for CompiledShell {
    fn generate(&self) -> Command {
        let mut result = Command::new(&self.path);
        result.args(&self.args);
        result
    }
}
