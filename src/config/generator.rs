use crate::{store::Store, utils::{GenericResult, CommandGenerator}, template};
use serde::{Deserialize, de::{Visitor, SeqAccess, self, Unexpected}};

pub enum Generator {
    String(String),
    Template(String),
    Shell(String),
}
impl<'de> Deserialize<'de> for Generator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GeneratorVisitor;
        impl<'de> Visitor<'de> for GeneratorVisitor {
            type Value = Generator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "generator")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Template(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                Ok(Self::Value::Template(v))
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let method = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                match method.as_str() {
                    "string" => {
                        let value = seq
                            .next_element::<String>()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok(Self::Value::String(value))
                    },
                    "template" => {
                        let pattern = seq
                            .next_element::<String>()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok(Self::Value::Template(pattern))
                    },
                    "shell" => {
                        let command = seq
                            .next_element::<String>()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok(Self::Value::Shell(command))
                    },
                    _ => Err(de::Error::invalid_value(Unexpected::Str(&method), &self)),
                }
            }
        }

        return deserializer.deserialize_any(GeneratorVisitor);
    }
}

impl Generator {
    fn expand_string(value: &str) -> GenericResult<String> {
        Ok(value.to_owned())
    }
    fn expand_template<T: Store>(store: &T, template: &str) -> GenericResult<String> {
        Ok(template::expand_template(store, template))
    }

    fn expand_shell<T: CommandGenerator>(cmdgen: &T, command: &str) -> GenericResult<String> {
        let output = cmdgen.generate()
            .arg(command)
            .output()?;
        Ok(std::str::from_utf8(&output.stdout)?.to_owned())
    }

    pub fn expand_without_shell<T: Store>(&self, store: &T) -> GenericResult<String> {
        match self {
            Generator::String(value) => Self::expand_string(value),
            Generator::Template(template) => Self::expand_template(store, template),
            Generator::Shell(_) => panic!("call expand_without_shell with shell"),
        }
    }

    pub fn expand<Tc: CommandGenerator, Ts: Store>(&self, cmdgen: &Tc, store: &Ts) -> GenericResult<String> {
        match self {
            Generator::String(value) => Self::expand_string(value),
            Generator::Template(template) => Self::expand_template(store, template),
            Generator::Shell(command) => Self::expand_shell(cmdgen, command),
        }
    }
}
