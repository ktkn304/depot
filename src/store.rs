use std::{collections::HashMap, env};

use url::Url;

pub trait Store: Clone {
    fn get(&self, key: &str) -> Option<&str>;
    fn set(&mut self, key: String, value: String);
    fn set_remote_raw(&mut self, remote_url_raw: &str) {
        self.set("DEPOT_REMOTE_RAW".to_owned(), remote_url_raw.to_owned());
    }
    fn set_remote_url(&mut self, remote_url: &Url) {
        self.set("DEPOT_REMOTE_URL".to_owned(), remote_url.to_string());
        self.set(
            "DEPOT_REMOTE_SCHEME".to_owned(),
            remote_url.scheme().to_owned(),
        );
        self.set(
            "DEPOT_REMOTE_USER".to_owned(),
            remote_url.username().to_owned(),
        );
        self.set(
            "DEPOT_REMOTE_HOST".to_owned(),
            remote_url.host_str().unwrap_or("").to_owned(),
        );

        let path = remote_url.path();
        self.set("DEPOT_REMOTE_PATH".to_owned(), path.to_owned());

        let filename_index = path.rfind("/").map_or(0, |idx| idx + 1);
        self.set(
            "DEPOT_REMOTE_FILENAME".to_owned(),
            path[filename_index..].to_owned(),
        );
        let extension_index = path.rfind(".").unwrap_or(path.len());
        self.set(
            "DEPOT_REMOTE_FILENAME_WITHOUT_EXTENSION".to_owned(),
            path[filename_index..extension_index].to_owned(),
        );
    }
    fn set_local_path(&mut self, local_path: String, rel_local_path: String) {
        self.set("DEPOT_LOCAL_PATH".to_owned(), local_path);
        self.set("DEPOT_LOCAL_REL_PATH".to_owned(), rel_local_path);
    }
    fn set_root_path(&mut self, root_path: String) {
        self.set("DEPOT_ROOT_PATH".to_owned(), root_path);
    }
    fn set_source_remote_raw(&mut self, source_raw: String) {
        self.set("DEPOT_SOURCE_REMOTE_RAW".to_owned(), source_raw);
    }
    fn set_source_remote_url(&mut self, source_url: &Url) {
        self.set("DEPOT_SOURCE_REMOTE_URL".to_owned(), source_url.to_string());
        self.set(
            "DEPOT_SOURCE_REMOTE_SCHEME".to_owned(),
            source_url.scheme().to_owned(),
        );
        self.set(
            "DEPOT_SOURCE_REMOTE_USER".to_owned(),
            source_url.username().to_owned(),
        );
        self.set(
            "DEPOT_SOURCE_REMOTE_HOST".to_owned(),
            source_url.host_str().unwrap_or("").to_owned(),
        );

        let path = source_url.path();
        self.set("DEPOT_SOURCE_REMOTE_PATH".to_owned(), path.to_owned());

        let filename_index = path.rfind("/").map_or(0, |idx| idx + 1);
        self.set(
            "DEPOT_SOURCE_REMOTE_FILENAME".to_owned(),
            path[filename_index..].to_owned(),
        );
        let extension_index = path.rfind(".").unwrap_or(path.len());
        self.set(
            "DEPOT_SOURCE_REMOTE_FILENAME_WITHOUT_EXTENSION".to_owned(),
            path[filename_index..extension_index].to_owned(),
        );
    }
    fn set_source_local_path(&mut self, source_path: String, rel_source_path: String) {
        self.set("DEPOT_SOURCE_LOCAL_PATH".to_owned(), source_path);
        self.set("DEPOT_SOURCE_LOCAL_REL_PATH".to_owned(), rel_source_path);
    }

    fn iter(&self) -> impl Iterator<Item = (&'_ str, &'_ str)>;
}

pub struct EnvironmentStore {
    map: HashMap<String, String>,
}
impl Clone for EnvironmentStore {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}
impl Store for EnvironmentStore {
    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|v| v.as_str())
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn iter(&self) -> impl Iterator<Item = (&'_ str, &'_ str)> {
        self.map.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}
impl EnvironmentStore {
    pub fn new(vars: Option<impl Iterator<Item = (String, String)>>) -> Self {
        let mut map = HashMap::new();

        if let Some(vars) = vars {
            for (k, v) in vars {
                map.insert(k, v);
            }
        }

        Self { map }
    }
    pub fn new_blank() -> Self {
        Self::new(None::<std::iter::Empty<(String, String)>>)
    }
    pub fn new_env() -> Self {
        Self::new(Some(env::vars()))
    }
}
