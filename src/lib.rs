use std::error::Error;
use std::time::Instant;
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

fn time_fn_micros<T, F>(f: F) -> u128
where
    F: Fn() -> T,
{
    // Warm up
    for _ in 0..100 {
        f();
    }

    let mut sum_micros = 0;
    for _ in 0..100 {
        let start_time = Instant::now();
        f();
        sum_micros += Instant::now().duration_since(start_time).as_micros();
    }

    sum_micros / 100
}

pub fn time_solution<T1, T2, F1, F2>(f1: F1, f2: F2)
where
    F1: Fn() -> T1,
    F2: Fn() -> T2,
{
    if env::var("AOCTIME").is_err() {
        return;
    }

    let time1 = time_fn_micros(f1);
    let time2 = time_fn_micros(f2);

    println!("Part 1: {time1} microseconds");
    println!("Part 2: {time2} microseconds");
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

            $crate::time_solution(|| $part_1_fn(&input), || $part_2_fn(&input));

            ::std::result::Result::Ok(())
        }
    };
}
