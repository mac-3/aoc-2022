use std::collections::HashSet;

fn main() {
    let priorities_sum = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let mut common = None;
            for i in 0..l.len() / 2 {
                if l[l.len() / 2..].contains(|c| l[i..].chars().next().unwrap() == c) {
                    common = l[i..].chars().next();
                    break;
                }
            }
            common.expect("No common item")
        })
        .map(map_priority)
        .sum::<usize>();
    println!("A: {}", priorities_sum);

    let mut iter = include_str!("../input.txt").lines();
    let mut sum = 0usize;
    while let Some(l1) = iter.next() {
        let tmp = iter.next().unwrap();
        let common = l1
            .chars()
            .filter(|c| tmp.contains(*c))
            .collect::<HashSet<char>>();
        let tmp = iter.next().unwrap();
        let common = l1
            .chars()
            .filter(|c| tmp.contains(*c))
            .collect::<HashSet<char>>()
            .intersection(&common)
            .next()
            .cloned()
            .map(map_priority)
            .unwrap();
        sum += common;
    }
    println!("B: {}", sum);
}

fn map_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => unimplemented!(),
    }
}
