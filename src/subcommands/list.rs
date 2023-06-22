use std::path::{Path, PathBuf};
use clap::Args;
use crate::config::fields_definition::FieldsDefinition;
use crate::config::generator::Generator;
use crate::config::pattern::Pattern;
use crate::error::{BuiltInCommandError, PathStringifyError};
use crate::config::Config;
use crate::store::{EnvironmentStore, Store};
use crate::utils::{GenericResult, StringMatcher, DirectoryMatcher, Compilable, CommandGenerator};

#[derive(Args)]
pub struct Subcommand {
    #[clap(short, long, use_value_delimiter = true, default_values_t = [ "path".to_owned() ])]
    fields: Vec<String>
}

struct FsVisitor<'a, Tcg: CommandGenerator, Tdm: DirectoryMatcher> {
    root: PathBuf,
    store: EnvironmentStore,
    cmdgen: Tcg,
    excludes: Vec<Box<dyn StringMatcher>>,
    condition: Tdm,
    fields: Vec<&'a Generator>,
}
impl<'a, Tcg: CommandGenerator, Tdm: DirectoryMatcher> FsVisitor<'a, Tcg, Tdm> {
    pub fn new<Tc: Compilable<Tdm>>(root: &str, store: EnvironmentStore, cmdgen: Tcg, exclude_patterns: &Vec<Pattern>, condition: &Tc, fields_def: &'a FieldsDefinition, fields: &Vec<String>) -> GenericResult<Self> {
        let mut excludes: Vec<Box<dyn StringMatcher>> = Vec::new();
        for ptn in exclude_patterns {
            excludes.push(ptn.compile()?)
        }
        let mut generators = Vec::<&Generator>::new();
        for field_name in fields {
            if let Some(def) = fields_def.get(field_name) {
                generators.push(&def)
            } else {
                return Err(Box::new(BuiltInCommandError::new(&format!("field name not found: {}", field_name))));
            }
        }
        if generators.is_empty() {
            if let Some(def) = fields_def.get("path") {
                generators.push(def);
            }
        }
        Ok(Self {
            root: PathBuf::from(root),
            store: store,
            cmdgen: cmdgen,
            excludes: excludes,
            condition: condition.compile()?,
            fields: generators,
        })
    }

    fn convert_path_to_str(path: &Path) -> GenericResult<&str> {
        path.to_str().ok_or(Box::new(PathStringifyError::new("convert path failed")))
    }

    fn accept_directory(&mut self, path: &Path) -> GenericResult<()> {
        for entry in path.read_dir()? {
            if let Ok(entry) = entry {
                if let Err(err) = self.accept(&entry.path()) {
                    eprintln!("{}", err);
                }
            }
        }
        Ok(())
    }

    fn accept_file(&mut self, _: &Path) -> GenericResult<()> {
        // ignore
        Ok(())
    }

    fn accept_project(&mut self, path: &Path) -> GenericResult<()> {
        let path_str = Self::convert_path_to_str(path)?;
        let relpath = path.strip_prefix(&self.root)?;
        let relpath_str = Self::convert_path_to_str(relpath)?;
        self.store.set_local_path(path_str.to_owned(), relpath_str.to_owned());
        self.store.export_all();

        let mut field_strs: Vec<String> = Vec::new();
        
        for &field in &self.fields {
            match field.expand(&self.cmdgen, &self.store) {
                Ok(v) => field_strs.push(v),
                Err(err) => eprintln!("{}", err),
            }
        }

        println!("{}", field_strs.join("\t"));
        Ok(())
    }

    fn accept_exclude(&mut self, _: &Path) -> GenericResult<()> {
        Ok(())
    }

    fn is_project(&mut self, path: &Path) -> bool {
        self.condition.is_match(&self.root, path)
    }

    fn is_exclude(&mut self, path: &Path) -> bool {
        if let Ok(path) = path.strip_prefix(&self.root) {
            if let Some(path) = path.to_str() {
                for exclude in &self.excludes {
                    if exclude.is_match(path) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn accept(&mut self, path: &Path) -> GenericResult<()> {
        if self.is_exclude(path) {
            self.accept_exclude(path)
        } else if self.is_project(path) {
            self.accept_project(path)
        } else if path.is_dir() {
            self.accept_directory(path)
        } else if path.is_file() {
            self.accept_file(path)
        } else {
            Err(Box::new(BuiltInCommandError::new("unknown fs item")))
        }
    }

    pub fn run(&mut self) -> GenericResult<()> {
        let root = self.root.clone();
        self.accept(&root)
    }
}

impl super::BuiltInCommand for Subcommand {
    fn run(&self, config: &Config) -> GenericResult<i32> {
        let mut store = EnvironmentStore::new();
        let cmdgen = config.shell.compile(&store)?;
        let root_path = config.core.root.expand(&cmdgen, &store)?;
        store.set_root_path(root_path.to_owned());
        let mut visitor = FsVisitor::new(&root_path, store, cmdgen, &config.subcommands.list.project.excludes, &config.subcommands.list.project.condition, &config.subcommands.list.fields, &self.fields)?;
        visitor.run()?;
        return Ok(0);
    }
}
