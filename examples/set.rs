use rust_cache::cache;

fn main() {
    let mut cache = cache::new(0, 0);
    cache.set_with_default_exp("test", 2);

    assert!(cache.get("test"), 2)
}
