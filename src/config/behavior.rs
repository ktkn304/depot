use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    Deserialize,
};

use crate::{
    store::Store,
    template,
    utils::{CommandGenerator, GenericResult},
};

pub fn nop() -> Behavior {
    Behavior::Nop
}

pub enum Behavior {
    Template(String),
    Shell(Vec<String>),
    Nop,
    NotSupported,
}
impl Default for Behavior {
    fn default() -> Self {
        Self::NotSupported
    }
}
impl<'de> Deserialize<'de> for Behavior {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BehaviorVisitor;
        impl<'de> Visitor<'de> for BehaviorVisitor {
            type Value = Behavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "behavior")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Shell(vec![v.to_owned()]))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::Shell(vec![v]))
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let method = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                match method.as_str() {
                    "template" => {
                        let pattern = seq
                            .next_element::<String>()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok(Self::Value::Template(pattern))
                    }
                    "shell" => {
                        let mut commands = Vec::<String>::new();
                        while let Some(command) = seq.next_element::<String>()? {
                            commands.push(command);
                        }
                        if commands.len() < 1 {
                            return Err(de::Error::invalid_length(1, &self));
                        }
                        Ok(Self::Value::Shell(commands))
                    }
                    "nop" => Ok(Self::Value::Nop),
                    "not-supported" => Ok(Self::Value::NotSupported),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(&method), &self)),
                }
            }
        }

        return deserializer.deserialize_any(BehaviorVisitor);
    }
}

impl Behavior {
    fn execute_template<T: Store>(store: &T, template: &str) -> GenericResult<i32> {
        let str = template::expand_template(store, template);
        println!("{}", str);
        Ok(0)
    }

    fn execute_shell<Tc: CommandGenerator, Ts: Store>(
        cmdgen: &Tc,
        store: &Ts,
        commands: &Vec<String>,
    ) -> GenericResult<i32> {
        let mut last_code: i32 = 0;
        for command in commands {
            last_code = cmdgen
                .generate(store)
                .arg(command)
                .status()?
                .code()
                .unwrap_or(0);
            if last_code != 0 {
                break;
            }
        }
        Ok(last_code)
    }

    fn execute_nop() -> GenericResult<i32> {
        Ok(0)
    }

    fn execute_not_supported() -> GenericResult<i32> {
        println!("not supported");
        Ok(255)
    }

    pub fn execute<Tc: CommandGenerator, Ts: Store>(
        &self,
        cmdgen: &Tc,
        store: &Ts,
    ) -> GenericResult<i32> {
        match self {
            Behavior::Template(template) => Self::execute_template(store, template),
            Behavior::Shell(commands) => Self::execute_shell(cmdgen, store, commands),
            Behavior::Nop => Self::execute_nop(),
            Behavior::NotSupported => Self::execute_not_supported(),
        }
    }
}
