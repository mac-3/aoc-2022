#[macro_use]
extern crate scan_fmt;

fn main() {
    let (mut stacks, rest) = parse_stacks(include_str!("../input.txt"));
    let stacks_save = stacks.clone();
    let moves = rest
        .lines()
        .skip(2)
        .map(|l| scan_fmt!(l, "move {d} from {d} to {d}", usize, usize, usize).unwrap())
        .map(|(n, from, to)| (n, from - 1, to - 1));

    moves.clone().for_each(|(n, from, to)| {
        (0..n).for_each(|_| {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        })
    });
    print_result('A', &stacks);

    let mut stacks = stacks_save;
    moves.for_each(|(n, from, to)| {
        let n = stacks[from].len() - n;
        let mut chunks: Vec<char> = stacks[from].drain(n..).collect();
        stacks[to].append(&mut chunks);
    });
    print_result('B', &stacks);
}

fn print_result(part: char, v: &[Vec<char>]) {
    print!("{}: ", part);
    v.iter().for_each(|v| {
        print!("{}", v.last().unwrap_or(&' '));
    });
    println!();
}

fn parse_stacks(raw: &str) -> (Vec<Vec<char>>, &str) {
    let mut s = raw.chars();
    let mut acc = vec![];
    let mut max = 0usize;

    while let Some('[') = s.next() {
        let mut stack_n = 0;
        while let Some(c) = s.next() {
            match c {
                'A'..='Z' => acc.push((stack_n, c)),
                ' ' => (),
                _ => unreachable!(),
            }
            if stack_n > max {
                max = stack_n;
            }
            let _ = s.next();
            if s.next().unwrap() == '\n' {
                break;
            }
            let _ = s.next();
            stack_n += 1;
        }
    }

    (
        acc.into_iter()
            .rev()
            .fold(vec![vec![]; max + 1], |mut acc, e| {
                acc[e.0].push(e.1);
                acc
            }),
        s.as_str(),
    )
}
