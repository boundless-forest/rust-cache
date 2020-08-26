use rust_cache::cache;
use std::time::Duration;

fn main() {
    let mut cache = cache::new(100, 5);
    cache.set("test".to_string(), 2, 100);

    println!("cache {:?}", cache);

    assert_eq!(cache.get("test".to_string()).unwrap(), 2);
    // std::thread::sleep(Duration::from_secs(200));
    assert_eq!(cache.get("test".to_string()).unwrap(), 0);
}
