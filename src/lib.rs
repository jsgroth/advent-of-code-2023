use std::error::Error;
use std::{env, fs};

// Read input filename from arg $1 and then read file contents into a String
pub fn read_input() -> Result<String, Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    let filename = args.next().ok_or("Missing required filename arg")?;

    let contents = fs::read_to_string(&filename)
        .map_err(|err| format!("Error reading file from '{filename}': {err}"))?;
    Ok(contents)
}

#[macro_export]
macro_rules! impl_standard_main {
    (p1: $part_1_fn:ident, p2: $part_2_fn:ident) => {
        fn main() -> ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> {
            let input = $crate::read_input()?;

            let solution1 = $part_1_fn(&input);
            ::std::println!("{solution1}");

            let solution2 = $part_2_fn(&input);
            ::std::println!("{solution2}");

            ::std::result::Result::Ok(())
        }
    };
}
