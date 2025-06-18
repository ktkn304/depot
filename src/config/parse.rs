use regex::Regex;
use serde::Deserialize;
use url::Url;

use crate::utils::GenericResult;

#[derive(Deserialize)]
pub struct Parse {
    pub default: ParseDefault,
}

impl Parse {
    pub fn parse_url(&self, uri: &str) -> GenericResult<Url> {
        let scp_regex = Regex::new(r"^(\w+)@([\w\.]+):(.+)$")?;
        let converted_url_str = scp_regex.replace(uri, "ssh://$1@$2/$3");
        let base_url_str = format!("{}://{}/", &self.default.scheme, &self.default.host);
        let base_url = Url::parse(&base_url_str)?;
        let remote_url = Url::options().base_url(Some(&base_url)).parse(&converted_url_str)?;

        return Ok(remote_url);
    }
}

#[derive(Deserialize)]
pub struct ParseDefault {
    #[serde(default = "ParseDefault::default_scheme")]
    pub scheme: String,
    #[serde(default = "ParseDefault::default_host")]
    pub host: String,
}

impl ParseDefault {
    fn default_scheme() -> String {
        "depot".to_owned()
    }
    fn default_host() -> String {
        "localhost".to_owned()
    }
}
