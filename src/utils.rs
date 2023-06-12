use std::error::Error;
use std::env;
use std::path::Path;
use std::process::Command;
pub type GenericError = Box<dyn Error + Sync + Send + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

pub trait StringMatcher {
    fn is_match(&self, str: &str) -> bool;
}

pub trait DirectoryMatcher {
    fn is_match(&self, prefix: &Path, path: &Path) -> bool;
}

pub trait CommandGenerator {
    fn generate(&self) -> Command;
}

pub trait Compilable<T> {
    fn compile(&self) -> GenericResult<T>;
}

pub fn concat_path(base: &str, path: &str) -> String {
    let mut result = base.to_owned();
    let is_absolute = result.starts_with("/");
    result.push_str(&path);
    result = result.split("/").filter(|i| !i.is_empty()).collect::<Vec<&str>>().join("/");
    if is_absolute {
        result.insert_str(0, "/");
    }
    return result;
}

pub fn expand_env(text: &str) -> String {
    let mut result = String::new();
    let mut remain_text = &text[0..];

    while remain_text.len() > 0 {
        if let Some(index) = remain_text.find('$') {
            result.push_str(&remain_text[0..index]);
            remain_text = &remain_text[(index + 1)..];

            match remain_text.chars().next() {
                None => {
                    break;
                },
                Some('$') => {
                    result.push_str("$");
                    remain_text = &remain_text[1..];
                },
                Some('{') => {
                    remain_text = &remain_text[1..];
                    if let Some((var_name, remain)) = remain_text.split_once("}") {
                        let val = env::var(var_name).unwrap_or(String::default());
                        result.push_str(&val);
                        remain_text = remain;
                    } else {
                        result.push_str(remain_text);
                        break;
                    }
                },
                Some(_) => {
                    result.push_str("$");
                }
            }
        } else {
            result.push_str(remain_text);
            break;
        }
    }

    return result;
}
