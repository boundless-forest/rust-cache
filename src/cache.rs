use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime};

const Default_Expiration: u64 = 0;

pub struct Item {
    object: u64,
    expiration: SystemTime,
}

pub struct Cache {
    default_expiration: u64,
    items: HashMap<String, Item>,
    janitor: Janitor,
}

pub struct Janitor {
    interval: u64,
}

pub fn new_cache(
    default_expiration: u64,
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

pub fn new(default_expiration: u64, clean_expiration: u64) -> Arc<RwLock<Cache>> {
    let items = HashMap::new();
    return new_cache_with_janitor(default_expiration, clean_expiration, items);
}

fn new_cache_with_janitor(
    default_expiration: u64,
    clean_expiration: u64,
    items: HashMap<String, Item>,
) -> Arc<RwLock<Cache>> {
    let c = new_cache(default_expiration, clean_expiration, items);

    if clean_expiration > 0 {
        // start clean up janitor
        let janitor = thread::spawn(move || {
            // set time to clean
        });
    }
    return c;
}

impl Cache {
    pub fn set(&mut self, key: String, value: u64, ed: u64) {
        let expiration_time = SystemTime::now().checked_add(Duration::from_secs(ed));
        // TODO: REMOVE UNWRAP
        let i = Item {
            object: value,
            expiration: expiration_time.unwrap(),
        };
        self.items.insert(key, i);
    }

    pub fn set_with_default_exp(&mut self, key: String, value: u64) {
        self.set(key, value, Default_Expiration)
    }

    pub fn get(&self, key: String) -> u64 {
        if let Some(i) = self.items.get(&key) {
            if SystemTime::now() > i.expiration {
                return 0;
            }
            return i.object;
        }
        0
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
