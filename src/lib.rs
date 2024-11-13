#[macro_export]
macro_rules! main {
    () => {
        fn main() -> Result<(), Box<dyn std::error::Error>> {
            use std::io::Read;
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input)?;
            println!("{}", part1(&input));
            println!("{}", part2(&input));
            Ok(())
        }
    };
}
