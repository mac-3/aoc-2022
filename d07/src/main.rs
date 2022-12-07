use std::collections::HashMap;

const TOTAL_SIZE: usize = 100_000;
const PATH_SEP: &str = "/";
const TOTAL_SPACE: usize = 70_000_000;
const UNUSED_SPACE: usize = 30_000_000;

fn main() {
    let mut fs: HashMap<String, usize> = HashMap::new();
    let mut stack = vec![];
    include_str!("../input.txt").lines().for_each(|l| {
        let mut s = l.chars();
        match s.next().unwrap() {
            '$' => {
                let _ = s.next();
                let mut cmd = s.as_str().split(' ');
                match cmd.next().unwrap() {
                    "cd" => match cmd.next().unwrap() {
                        ".." => {
                            let _ = stack.pop();
                        }
                        "/" => {
                            stack = vec![];
                        }
                        p => {
                            stack.push(p);
                        }
                    },
                    "ls" => (),
                    _ => unreachable!(),
                }
            }
            _ => match l.split(' ').next().unwrap() {
                "dir" => (),
                n => {
                    let n = n.parse::<usize>().unwrap();
                    for i in 0..=stack.len() {
                        let k = stack[..i]
                            .iter()
                            .copied()
                            .map(|s| format!("{}{}", PATH_SEP, s))
                            .collect::<String>();
                        *fs.entry(k).or_default() += n;
                    }
                }
            },
        }
    });

    println!("{:?}", &fs);

    let a = fs
        .iter()
        .filter(|(_, v)| **v <= TOTAL_SIZE)
        .map(|(_, v)| v)
        .copied()
        .sum::<usize>();
    println!("A: {}", a);

    let offset = TOTAL_SPACE - fs[""];
    // Note: It is safe to assume that the unused space is always greater or equal
    // than the offset for the premises of the text.
    let b = fs
        .iter()
        .filter_map(|(_, v)| {
            if *v >= UNUSED_SPACE - offset {
                Some(v)
            } else {
                None
            }
        })
        .min()
        .unwrap();
    println!("B: {}", b);
}
