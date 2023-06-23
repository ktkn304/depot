use globset::{GlobBuilder, GlobMatcher as GlobSetMatcher};
use serde::{Deserialize, de::{Visitor, self, SeqAccess, Unexpected}};

use crate::utils::{GenericResult, StringMatcher, Compilable};

pub enum Pattern {
    Glob(String),
    StartsWith(String),
}
impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PatternVisitor;
        impl<'de> Visitor<'de> for PatternVisitor {
            type Value = Pattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "pattern")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Glob(v.to_owned()))
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let method = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                match method.as_str() {
                    "glob" => {
                        let pattern = seq
                            .next_element::<String>()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok(Self::Value::Glob(pattern))
                    }
                    "starts-with" => {
                        let pattern = seq
                            .next_element::<String>()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok(Self::Value::StartsWith(pattern))
                    }
                    _ => Err(de::Error::invalid_value(Unexpected::Str(&method), &self)),
                }
            }
        }

        return deserializer.deserialize_any(PatternVisitor);
    }
}

impl Compilable<Box<dyn StringMatcher>> for Pattern {
    fn compile(&self) -> GenericResult<Box<dyn StringMatcher>> {
        match self {
            Pattern::Glob(pattern) => {
                let glob = GlobBuilder::new(pattern)
                    .literal_separator(true).build()
                    .or_else(|err| { eprintln!("{}", err); Err(err) })?;
                Ok(Box::new(GlobMatcher::new(glob.compile_matcher())))
            }
            Pattern::StartsWith(pattern) => {
                Ok(Box::new(StartsWithMatcher::new(pattern.to_owned())))
            }
        }
    }
}

pub struct GlobMatcher {
    glob_matcher: GlobSetMatcher,
}
impl GlobMatcher {
    fn new(glob_matcher: GlobSetMatcher) -> Self {
        Self {
            glob_matcher: glob_matcher,
        }
    }
}
impl StringMatcher for GlobMatcher {
    fn is_match(&self, str: &str) -> bool {
        self.glob_matcher.is_match(str)
    }
}

pub struct StartsWithMatcher {
    pattern: String,
}
impl StartsWithMatcher {
    fn new(pattern: String) -> Self {
        Self {
            pattern,
        }
    }
}
impl StringMatcher for StartsWithMatcher {
    fn is_match(&self, str: &str) -> bool {
        str.starts_with(&self.pattern)
    }
}

