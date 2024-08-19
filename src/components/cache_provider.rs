use leptos::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct CachedUserData {
    username: String,
    pfp: String,
    timestamp: f64,
}

#[derive(Clone, Debug)]
struct ClientCache {
    cache: std::rc::Rc<std::cell::RefCell<HashMap<u64, CachedUserData>>>,
}

impl ClientCache {
    pub fn new() -> Self {
        ClientCache {
            cache: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, fid: u64) -> Option<(String, String)> {
        let cache = self.cache.borrow();
        cache.get(&fid).and_then(|data| {
            let now = js_sys::Date::now();
            if now - data.timestamp < 3600000.0 {
                Some((data.username.clone(), data.pfp.clone()))
            } else {
                None
            }
        })
    }

    pub fn set(&self, fid: u64, username: String, pfp: String) {
        let mut cache = self.cache.borrow_mut();
        cache.insert(fid, CachedUserData {
            username,
            pfp,
            timestamp: Date::now(),
        });
    }
}

pub fn provide_client_cache() {
    let client_cache = create_rw_signal(ClientCache::new());
    provide_context(client_cache);
}
