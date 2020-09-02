use rust_cache::cache::new;
use rust_cache::cache::{DEFAULT_EXPIRATION, NO_EXPIRATION};
use std::thread;
use std::time::Duration;

fn main() {
    let mut tc = new(Duration::from_secs(50), Duration::from_secs(1));
    tc.set("a", serde_json::json!(1), DEFAULT_EXPIRATION);
    tc.set("b", serde_json::json!(2), NO_EXPIRATION);
    tc.set("c", serde_json::json!(3), Duration::from_secs(20));

    thread::sleep(Duration::from_secs(25));
    assert_eq!(tc.get("a").unwrap(), 1);
    assert_eq!(tc.get("b").unwrap(), 2);
    assert!(tc.get("c").is_none());

    thread::sleep(Duration::from_secs(30));
    assert!(tc.get("a").is_none());
    assert_eq!(tc.get("b").unwrap(), 2);
}
