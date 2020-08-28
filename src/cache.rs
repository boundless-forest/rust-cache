use crossbeam::crossbeam_channel::tick;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const DEFAULT_EXPIRATION: Duration = Duration::from_secs(0);
pub const NO_EXPIRATION: Duration = Duration::from_secs(std::u64::MAX);

#[derive(Clone, Debug)]
pub struct RCache {
    cache: Arc<RwLock<Cache>>,
}
#[derive(Debug)]
pub struct Cache {
    default_expiration: Duration,
    items: HashMap<&'static str, Item>,
    janitor: Janitor,
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    object: u64,
    expiration: Duration,
}

#[derive(Debug)]
pub struct Janitor {
    interval: Duration,
}

// Return a new cache with a given default expiration time and
// cleanup interval, empty items map.
pub fn new(default_expiration: Duration, clean_expiration: Duration) -> RCache {
    let items = HashMap::new();
    return new_cache_with_janitor(default_expiration, clean_expiration, items);
}

// Create a cache with janitor or not
fn new_cache_with_janitor(
    default_expiration: Duration,
    clean_expiration: Duration,
    items: HashMap<&'static str, Item>,
) -> RCache {
    let c = new_cache(default_expiration, clean_expiration, items);
    let mut c_clone = c.clone();

    // If cleanup interval gt 0, start cleanup janitor
    if clean_expiration > Duration::from_secs(0) {
        let _ = thread::spawn(move || {
            let ticker = tick(clean_expiration);
            loop {
                ticker.recv().unwrap();
                c_clone.delete_expired()
            }
        });
    }
    c
}

pub fn new_cache(
    mut default_expiration: Duration,
    clean_expiration: Duration,
    items: HashMap<&'static str, Item>,
) -> RCache {
    // If the default expiration equal to DEFAULT_EXPIRATION, the items in
    // the cache will never expire, and must be deleted manually.
    if default_expiration == DEFAULT_EXPIRATION {
        default_expiration = NO_EXPIRATION;
    }

    let c = Cache {
        default_expiration,
        items,
        janitor: Janitor {
            interval: clean_expiration,
        },
    };

    return RCache {
        cache: Arc::new(RwLock::new(c)),
    };
}

impl Item {
    // Check whether an item is expired.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if now > self.expiration {
            return true;
        }
        false
    }
}

impl RCache {
    // Add an item to the cache, replace any existing item
    // If the expiration duration is zero(Default_Expiration), the cache's default
    // expiration time is used. If it is -1(NO_EXPIRATION), the item never expired.
    pub fn set(&mut self, key: &'static str, value: u64, mut ed: Duration) {
        let c_lock = self.cache.clone();
        let mut c = c_lock.write().unwrap();
        let mut expiration_time = Duration::from_secs(0);

        if ed == DEFAULT_EXPIRATION {
            ed = c.default_expiration
        }

        if ed == NO_EXPIRATION {
            expiration_time = NO_EXPIRATION;
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if ed != NO_EXPIRATION && ed > Duration::from_secs(0) {
            expiration_time = now.checked_add(ed).unwrap();
        }

        let i = Item {
            object: value,
            expiration: expiration_time,
        };
        c.items.insert(key, i);
    }

    // Add an item to the cache, using DEFAULT_EXPIRATION
    pub fn set_with_default_exp(&mut self, key: &'static str, value: u64) {
        self.set(key, value, DEFAULT_EXPIRATION)
    }

    // Delete all items from the cache
    pub fn flush(&mut self) {
        let c_lock = self.cache.clone();
        let mut c = c_lock.write().unwrap();

        c.items.clear()
    }

    // Delete all expired items from the cache
    pub fn delete_expired(&mut self) {
        let items = self.get_items();
        let cw_lock = self.cache.clone();
        match cw_lock.try_write() {
            Ok(mut cw) => {
                for entry in items.iter() {
                    let (key, item) = entry;
                    if item.is_expired() {
                        let _ = cw.items.remove_entry(*key);
                    }
                }
            }
            Err(err) => {
                println!("error msg {:?}", err);
            }
        };
    }

    pub fn get_items(&self) -> Vec<(&str, Item)> {
        let c_lock = self.cache.clone();
        let c = c_lock.read().unwrap();

        let keys: Vec<&str> = c.items.keys().map(|s| (*s).clone()).collect();
        let mut items: Vec<(&str, Item)> = Vec::new();
        for key in keys.iter() {
            let value = c.items.get(key).unwrap();
            items.push((key, *value));
        }
        drop(c);
        return items;
    }

    // Get an item from the cache. Return NONE or item's object
    pub fn get(&self, key: &'static str) -> Option<u64> {
        let c_lock = self.cache.clone();
        let c = c_lock.read().unwrap();

        if let Some(i) = c.items.get(key) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            if i.expiration > Duration::from_secs(0) && now > i.expiration {
                return Some(0);
            }
            return Some(i.object);
        }
        Some(0)
    }

    // Return the number of items in the cache.
    pub fn item_count(&self) -> usize {
        let c_lock = self.cache.clone();
        let c = c_lock.read().unwrap();

        c.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let mut tc = new(DEFAULT_EXPIRATION, Duration::from_secs(0));

        assert_eq!(tc.get("a").unwrap(), 0);
        assert_eq!(tc.get("b").unwrap(), 0);
        assert_eq!(tc.get("C").unwrap(), 0);

        tc.set("a", 1, DEFAULT_EXPIRATION);
        tc.set("b", 2, DEFAULT_EXPIRATION);
        tc.set("C", 3, DEFAULT_EXPIRATION);

        assert_eq!(tc.get("a").unwrap(), 1);
        assert_eq!(tc.get("b").unwrap(), 2);
        assert_eq!(tc.get("C").unwrap(), 3);
    }

    #[test]
    fn test_cache_times() {
        let mut tc = new(Duration::from_secs(50), Duration::from_secs(1));
        tc.set("a", 1, DEFAULT_EXPIRATION);
        tc.set("b", 2, NO_EXPIRATION);
        tc.set("c", 3, Duration::from_secs(20));
        tc.set("d", 4, Duration::from_secs(70));

        thread::sleep(Duration::from_secs(25));
        assert_eq!(tc.get("c").unwrap(), 0);
        assert_eq!(tc.get("b").unwrap(), 2);

        thread::sleep(Duration::from_secs(30));
        assert_eq!(tc.get("a").unwrap(), 0);

        assert_eq!(tc.get("d").unwrap(), 4);
        thread::sleep(Duration::from_secs(20));
        assert_eq!(tc.get("d").unwrap(), 0);
    }
}
