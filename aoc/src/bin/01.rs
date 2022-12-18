fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<u32>().expect("not a valid number"))
                .sum()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut calories = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<u32>().expect("not a valid number"))
                .sum()
        })
        .collect::<Vec<_>>();
    calories.sort();
    calories.iter().rev().take(3).sum::<u32>()
}

#[aoc::main(01)]
fn main() {}
