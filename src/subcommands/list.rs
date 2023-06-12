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
/*
enum ListFields {
    Path,
    Command(String)
}

struct Processor {
    show_fullpath: bool,
    root_path: PathBuf,
    depth: usize,
    force_depth: bool,
    overrides: Vec<OverrideEntry>,
    fields: Vec<ListFields>,
    shell: String,
    shell_args: Vec<String>,
    excludes: Vec<GlobMatcher>,
}

struct OverrideEntry {
    matcher: GlobMatcher,
    depth: usize,
}

impl Processor {
    pub fn new(args: &Subcommand, config: &Config) -> GenericResult<Self> {
        let mut store = EnvironmentStore::new();
        let cmdgen = config.shell.compile(&store)?;
        let root_path = config.core.root.expand(&cmdgen, &store)?;

        let mut overrides = Vec::<OverrideEntry>::new();
        if let Some(config_overrides) = &config.builtin.list.overrides {
            for (_, config_override) in config_overrides {
                for pattern in &config_override.pattern {
                    if let Ok(glob) = GlobBuilder::new(pattern)
                            .literal_separator(true).build()
                            .or_else(|err| { eprintln!("{}", err); Err(err) }) {
                        overrides.push(OverrideEntry {
                            matcher: glob.compile_matcher(),
                            depth: config_override.depth,
                        });
                    }
                }
            }
        }

        let shell = utils::expand_env(&config.depot.shell);
        let shell_args = config.depot.shell_args.iter().map(|arg| { utils::expand_env(&arg) }).collect();

        let mut fields = Vec::<ListFields>::new();
        for field_name in &args.fields {
            if field_name == "path" {
                fields.push(ListFields::Path);
                continue;
            }
            if let Some(config_fields) = &config.builtin.list.fields {
                let cmd_str = config_fields.get(field_name)
                    .ok_or(BuiltInCommandError::new(&format!("field name not found: {}", field_name)))?;
                fields.push(ListFields::Command(String::from(cmd_str)));
            } else {
                return Err(Box::new(BuiltInCommandError::new(&format!("field name not found: {}", field_name))));
            }
        }
        if args.fields.is_empty() {
            fields.push(ListFields::Path);
            if let Some(config_fields) = &config.builtin.list.fields {
                for (_, cmd_str) in config_fields {
                    fields.push(ListFields::Command(String::from(cmd_str)));
                }
            }
        }

        let mut excludes = Vec::<GlobMatcher>::new();
        for pattern in &config.builtin.list.excludes {
            if let Ok(glob) = GlobBuilder::new(pattern)
                    .literal_separator(true).build()
                    .or_else(|err| { eprintln!("{}", err); Err(err) }) {
                excludes.push(glob.compile_matcher());
            }
        }

        return Ok(Processor {
            root_path: PathBuf::from(root_path),
            depth: args.depth.unwrap_or(config.builtin.list.depth),
            force_depth: args.depth.is_some(),
            overrides: overrides,
            show_fullpath: args.fullpath,
            fields: fields,
            shell: shell,
            shell_args: shell_args,
            excludes: excludes,
        });
    }

    fn is_excluded(&self, path: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let rpath = path.strip_prefix(&self.root_path)?;
        let rpath_str = rpath.to_str().ok_or(PathStringifyError::new("strigify failed"))?;
        return Ok(self.excludes.iter().find(|entry| { entry.is_match(rpath_str) }).is_some());
    }

    fn is_acceptable(&self, path: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let rpath = path.strip_prefix(&self.root_path)?;
        let depth = rpath.components().count();
        let mut max_depth = self.depth;

        if ! self.force_depth {
            if let Some(rpath_str) = rpath.to_str() {
                if let Some(override_entry) = self.overrides.iter()
                        .find(|entry| { entry.matcher.is_match(rpath_str) }) {
                    max_depth = override_entry.depth;
                }
            }
        }

        return Ok(max_depth <= depth);
    }

    fn accept(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut field_strs = Vec::<String>::new();

        let path_str = path.to_str().unwrap_or("");
        let rpath_str = path.strip_prefix(&self.root_path)?.to_str().unwrap_or("");

        utils::set_environ!(Local, path_str);
        for field in &self.fields {
            match field {
                ListFields::Path => {
                    if self.show_fullpath {
                        field_strs.push(String::from(path_str));
                    } else {
                        field_strs.push(String::from(rpath_str));
                    }
                }
                ListFields::Command(cmd_str) => {
                    let mut cmd = Command::new(&self.shell);
                    cmd.args(&self.shell_args)
                        .arg(cmd_str)
                        .stdin(Stdio::null());
                    match cmd.output() {
                        Ok(output) => {
                            let str = String::from(String::from_utf8_lossy(&output.stdout).trim());
                            field_strs.push(str);
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            field_strs.push(String::default());
                        }
                    }
                }
            }
        }

        println!("{}", field_strs.join("\t"));
        return Ok(());
    }

    fn walk_dir(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        for child in path.read_dir()?
                .filter_map(|entry| { entry.or_else(|err| { eprintln!("{}", err); Err(err)}).ok() }) {
            let child_path = child.path();
            if self.is_excluded(&child_path).unwrap_or_else(|e| { eprintln!("{}", e); true }) {
                continue;
            }
            if self.is_acceptable(&child_path)? {
                if let Err(err) = self.accept(&child_path) {
                    eprintln!("{}", err);
                }
            } else {
                if child.file_type()
                        .map_or_else(
                            |err| { eprintln!("{}", err); Err(err) },
                            |file_type| { Ok(file_type.is_dir()) }
                        ).unwrap_or(false) {
                    _ = self.walk_dir(&child_path).or_else(|err| { eprintln!("{}", err); Err(err) });
                }
            }
        }
        return Ok(());
    }

    pub fn run(&self) {
        if let Err(err) = self.walk_dir(&self.root_path) {
            eprintln!("{}", err);
        }
    }
}
 */

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
