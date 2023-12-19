use std::error::Error;
use std::fmt::{Display, Formatter};
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

struct SolutionTimeMicros {
    min: u128,
    max: u128,
    median: u128,
    mean: u128,
}

impl Display for SolutionTimeMicros {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ min={}, median={}, mean={}, max={} }}",
            self.min, self.median, self.mean, self.max
        )
    }
}

fn time_fn_micros<T, F>(f: F) -> SolutionTimeMicros
where
    F: Fn() -> T,
{
    // Warm up
    for _ in 0..100 {
        f();
    }

    let mut times = Vec::new();
    for _ in 0..100 {
        let start_time = Instant::now();
        f();
        times.push(Instant::now().duration_since(start_time).as_micros());
    }

    times.sort();

    let mean = times.iter().copied().sum::<u128>() / 100;

    SolutionTimeMicros {
        min: times[0],
        max: *times.last().unwrap(),
        median: (times[49] + times[50]) / 2,
        mean,
    }
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

    println!("Part 1 time (microseconds): {time1}");
    println!("Part 2 time (microseconds): {time2}");
}

#[macro_export]
macro_rules! impl_main {
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
