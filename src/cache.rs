use crossbeam::crossbeam_channel::tick;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime};

const DEFAULT_EXPIRATION: u64 = 0;

#[derive(Debug)]
pub struct Item {
    object: u64,
    expiration: SystemTime,
}

#[derive(Clone, Debug)]
pub struct RCache {
    cache: Arc<RwLock<Cache>>,
}

#[derive(Debug)]
pub struct Cache {
    default_expiration: u64,
    items: HashMap<String, Item>,
    janitor: Janitor,
}

#[derive(Debug)]
pub struct Janitor {
    interval: u64,
}

pub fn new(default_expiration: u64, clean_expiration: u64) -> RCache {
    let items = HashMap::new();
    return new_cache_with_janitor(default_expiration, clean_expiration, items);
}

fn new_cache_with_janitor(
    default_expiration: u64,
    clean_expiration: u64,
    items: HashMap<String, Item>,
) -> RCache {
    let c = new_cache(default_expiration, clean_expiration, items);
    let c_clone = c.clone();

    // if clean_expiration > 0 {
    // start clean up janitor
    // unimplemented!()
    // let janitor = thread::spawn(move || {
    //     // set time to clean
    //     let ticker = tick(Duration::from_secs(clean_expiration));

    //     loop {
    //         ticker.recv().unwrap();
    //         c_clone.delete_expired()
    //     }
    // });
    // }
    c
}
pub fn new_cache(
    default_expiration: u64,
    clean_expiration: u64,
    items: HashMap<String, Item>,
) -> RCache {
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

impl RCache {
    pub fn set(&mut self, key: String, value: u64, ed: u64) {
        let c_lock = self.cache.clone();
        let mut c = c_lock.write().unwrap();

        let expiration_time = SystemTime::now().checked_add(Duration::from_secs(ed));
        let i = Item {
            object: value,
            expiration: expiration_time.unwrap(),
        };
        c.items.insert(key, i);
    }

    pub fn set_with_default_exp(&mut self, key: String, value: u64) {
        self.set(key, value, DEFAULT_EXPIRATION)
    }

    pub fn flush(&mut self) {
        let c_lock = self.cache.clone();
        let mut c = c_lock.write().unwrap();

        c.items.clear()
    }

    pub fn get(&self, key: String) -> u64 {
        let c_lock = self.cache.clone();
        let c = c_lock.read().unwrap();

        if let Some(i) = c.items.get(&key) {
            if SystemTime::now() > i.expiration {
                return 0;
            }
            return i.object;
        }
        0
    }

    pub fn item_count(&self) -> usize {
        let c_lock = self.cache.clone();
        let c = c_lock.read().unwrap();

        c.items.len()
    }

    pub fn delete_expired(&self) {
        unimplemented!()
        // for key, item in self.items() {
        //     if item.expiration > 0 && now > item.expiration {
        //         self.item.delete(key)
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_set_u64() {
        unimplemented!();
    }

    #[test]
    fn test_set_default() {
        unimplemented!();
    }
}
