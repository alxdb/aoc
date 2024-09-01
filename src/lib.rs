#[macro_export]
macro_rules! main {
    () => {
        include!(concat!(env!("OUT_DIR"), "/aoc_prefix.rs"));
    };
}
