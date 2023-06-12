use std::{collections::HashMap, env, borrow::Cow};

use url::Url;

pub trait Store {
    fn get(&self, key: &str) -> Option<Cow<String>>;
    fn set(&mut self, key: String, value: String);
    fn set_remote_url(&mut self, remote_url: &Url) {
        self.set("DEPOT_REMOTE_URL".to_owned(), remote_url.to_string());
        self.set("DEPOT_REMOTE_SCHEME".to_owned(), remote_url.scheme().to_owned());
        self.set("DEPOT_REMOTE_USER".to_owned(), remote_url.username().to_owned());
        self.set("DEPOT_REMOTE_HOST".to_owned(), remote_url.host_str().unwrap_or("").to_owned());

        let path = remote_url.path();
        self.set("DEPOT_REMOTE_PATH".to_owned(), path.to_owned());

        let filename_index = path.rfind("/").map_or(0, |idx| { idx + 1 });
        self.set("DEPOT_REMOTE_FILENAME".to_owned(), path[filename_index..].to_owned());
        let extension_index = path.rfind(".").unwrap_or(path.len());
        self.set("DEPOT_REMOTE_FILENAME_WITHOUT_EXTENSION".to_owned(), path[filename_index..extension_index].to_owned());
    }
    fn set_local_path(&mut self, local_path: String, rel_local_path: String) {
        self.set("DEPOT_LOCAL_PATH".to_owned(), local_path);
        self.set("DEPOT_LOCAL_REL_PATH".to_owned(), rel_local_path);
    }
    fn set_root_path(&mut self, root_path: String) {
        self.set("DEPOT_ROOT_PATH".to_owned(), root_path);
    }
}

impl Store for HashMap<String, String> {
    fn get(&self, key: &str) -> Option<Cow<String>> {
        self.get(key).map(|s| Cow::Borrowed(s))
    }

    fn set(&mut self, key: String, value: String) {
        self.insert(key, value);
    }
    
}

pub struct EnvironmentStore {
    map: HashMap<String, String>,
}
impl Store for EnvironmentStore {
    fn get(&self, key: &str) -> Option<Cow<String>> {
        if self.map.contains_key(key) {
            self.map.get(key).map(|s| Cow::Borrowed(s))
        } else {
            env::var(key).map_or(Option::None, |s| Some(Cow::Owned(s)))
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}
impl EnvironmentStore {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
    pub fn export_all(&self) {
        for (key, value) in self.map.iter() {
            env::set_var(key, value);
        }
    }
}
