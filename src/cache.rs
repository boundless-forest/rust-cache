use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub struct Item {
    object: i64,
    Expiration: u64,
}

pub struct Cache {
    default_expiration: Duration,
    items: HashMap<String, Item>,
    janitor: Janitor,
}

pub struct Janitor {
    interval: u64,
}

pub fn new_cache(
    default_expiration: Duration,
    clean_expiration: u64,
    items: HashMap<String, Item>,
) -> Arc<RwLock<Cache>> {
    let c = Cache {
        default_expiration,
        items,
        janitor: Janitor {
            interval: clean_expiration,
        },
    };

    return Arc::new(RwLock::new(c));
}

pub fn new(
    default_expiration: Duration,
    clean_expiration: u64,
    items: HashMap<String, Item>,
) -> Arc<RwLock<Cache>> {
    let items = HashMap::new();
    return new_cache_with_janitor(default_expiration, clean_expiration, items);
}

fn new_cache_with_janitor(
    default_expiration: Duration,
    clean_expiration: u64,
    items: HashMap<String, Item>,
) -> Arc<RwLock<Cache>> {
    let c = new(default_expiration, clean_expiration, items);

    if clean_expiration > 0 {
        // start clean up janitor
        let janitor = thread::spawn(move || {
            // set time to clean
        });
    }
    return c;
}

fn Set(k: String, v: i64, d: Duration) {}
