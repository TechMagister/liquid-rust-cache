use std::fs::File;
use std::path::PathBuf;

use std::io::{Read,Write};

use cache::Cache;

pub struct RawCache {
    path: String
}

impl RawCache {
    pub fn new(path: String) -> RawCache {
        RawCache { path: path }
    }
}

impl Cache for RawCache {
    fn get(&self, k : &String) -> Option<String> {
        let mut p = PathBuf::from(&self.path);
        p.push(k);
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(_) => return None
        };
        let mut buffer = String::new();
        match f.read_to_string(&mut buffer) {
            Ok(_) => Some(buffer),
            Err(_) => None
        }
    }

    fn set(&self, k : &String, v: String) {
        let mut p = PathBuf::from(&self.path);
        p.push(k);
        let mut f = match File::create(p) {
            Ok(f) => f,
            Err(_) => return
        };
        f.write_all(v.as_bytes()).unwrap();
    }
}

#[test]
fn hardcache() {
    let cache = RawCache::new("./tests/tmp".to_string());
    cache.set(&"testkey".to_string(), "data".to_string());

    assert_eq!(cache.get(&"fakekey".to_string()), None);
    assert_eq!(cache.get(&"testkey".to_string()), Some("data".to_string()));
}
