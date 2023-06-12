use serde::Deserialize;

use crate::{utils::{GenericResult, CommandGenerator}, store::Store};

use super::{generator::Generator, overload::Overloadable};

#[derive(Deserialize)]
pub struct ResolveParams {
    pub path: Generator,
}
impl Default for ResolveParams {
    fn default() -> Self {
        // dummy
        Self { path: Generator::String("".to_owned()) }
    }
}

pub type Resolve = Overloadable<ResolveParams>;
impl Resolve {
    pub fn expand_path<Tc: CommandGenerator, Ts: Store>(&self, cmdgen: &Tc, store: &Ts, overload_name: Option<&str>) -> GenericResult<String> {
        let params = self.get_params(overload_name);
        params.path.expand(cmdgen, store)
    }
}
