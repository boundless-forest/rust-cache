use rust_cache::cache;
use std::time::Duration;

fn main() {
    let mut cache = cache::new(100, 0);
    cache.set("test".to_string(), 2, 100);

    assert_eq!(cache.get("test".to_string()), 2);
    std::thread::sleep(Duration::from_secs(200));
    assert_eq!(cache.get("test".to_string()), 0);
}
