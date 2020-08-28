use std::thread;
use std::time::Duration;
use rust_cache::cache::{DEFAULT_EXPIRATION, NO_EXPIRATION};
use rust_cache::cache::new;

fn main() {
    let mut tc = new(Duration::from_secs(50), Duration::from_secs(1));
    tc.set("a", 1, DEFAULT_EXPIRATION);
    tc.set("b", 2, NO_EXPIRATION);
    tc.set("c", 3, Duration::from_secs(20));

    thread::sleep(Duration::from_secs(25));
    assert_eq!(tc.get("a").unwrap(), 1);
    assert_eq!(tc.get("b").unwrap(), 2);
    assert_eq!(tc.get("c").unwrap(), 0);

    thread::sleep(Duration::from_secs(30));
    assert_eq!(tc.get("a").unwrap(), 0);
    assert_eq!(tc.get("b").unwrap(), 2);

}
