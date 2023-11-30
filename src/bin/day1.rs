use std::error::Error;

fn solve(input: &str) -> u32 {
    input.lines().count() as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2023::read_input()?;

    let solution1 = solve(&input);
    println!("{solution1}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample/day1.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!(solve(SAMPLE_INPUT), 0);
    }
}
