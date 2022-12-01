fn main() {
    let mut calories: Vec<u32> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<u32>().expect("not a valid number"))
                .sum()
        })
        .collect();
    println!("A: {}", calories.iter().max().unwrap());

    calories.sort();
    println!("B: {}", calories.iter().rev().take(3).sum::<u32>());
}
