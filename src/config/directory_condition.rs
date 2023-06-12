use std::path::Path;

use serde::Deserialize;

use crate::utils::{StringMatcher, GenericResult, DirectoryMatcher, Compilable};

use super::pattern::Pattern;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct DirectoryCondition {
    pub entries: Vec<DirectoryConditionEntry>,
}
impl Default for DirectoryCondition {
    fn default() -> Self {
        Self {
            entries: vec![
                DirectoryConditionEntry {
                    mode: DirectoryConditionMode::Parent,
                    pattern: Pattern::Glob("**/.git".to_owned()),
                }
            ]
        }
    }
}
impl Compilable<CompiledDirectoryCondition> for DirectoryCondition {
    fn compile(&self) -> GenericResult<CompiledDirectoryCondition> {
        let mut entries: Vec<CompiledDirectoryConditionEntry> = Vec::new();
        for entry in &self.entries {
            entries.push(entry.compile()?);
        }
        Ok(CompiledDirectoryCondition {
            entries,
        })
    }
}

pub struct CompiledDirectoryCondition {
    entries: Vec<CompiledDirectoryConditionEntry>,
}
impl DirectoryMatcher for CompiledDirectoryCondition {
    fn is_match(&self, prefix: &Path, path: &Path) -> bool {
        for entry in &self.entries {
            match entry.mode {
                DirectoryConditionMode::Parent => {
                    if let Ok(children) = path.read_dir() {
                        for child in children {
                            if let Ok(child) = child {
                                if entry.is_match(prefix, &child.path()) {
                                    return true;
                                }
                            }
                        }
                    }
                },
                DirectoryConditionMode::Exact => {
                    if entry.is_match(prefix, path) {
                        return true;
                    }
                },
                DirectoryConditionMode::Ignore => {
                    if entry.is_match(prefix, path) {
                        return false;
                    }
                },
            }
        }
        false
    }
}

#[derive(Deserialize, Copy, Clone)]
pub enum DirectoryConditionMode {
    #[serde(alias = "parent")]
    Parent,
    #[serde(alias = "exact")]
    Exact,
    #[serde(alias = "ignore")]
    Ignore,
}

#[derive(Deserialize)]
pub struct DirectoryConditionEntry {
    pub mode: DirectoryConditionMode,
    pub pattern: Pattern,
}
impl Compilable<CompiledDirectoryConditionEntry> for DirectoryConditionEntry {
    fn compile(&self) -> GenericResult<CompiledDirectoryConditionEntry> {
        Ok(CompiledDirectoryConditionEntry {
            mode: self.mode,
            pattern: self.pattern.compile()?
        })
    }
}

pub struct CompiledDirectoryConditionEntry {
    pub mode: DirectoryConditionMode,
    pub pattern: Box<dyn StringMatcher>,
}
impl CompiledDirectoryConditionEntry {
    pub fn is_match(&self, prefix: &Path, path: &Path) -> bool {
        if let Ok(path) = path.strip_prefix(prefix) {
            if let Some(path) = path.to_str() {
                return self.pattern.is_match(path);
            }
        }
        false
    }
}
