use std::collections::VecDeque;

const WORRY_LEVEL_DIV: usize = 3;
const FIRST_ITER: usize = 20;
const SECOND_ITER: usize = 10_000;

#[derive(Debug, Clone)]
struct MonkeyStruct<'input, T> {
    items: VecDeque<T>,
    op: u8,
    rhs: &'input str,
    cond: usize,
    cond_true: usize,
    cond_false: usize,
}

fn parse_input(input: &str) -> impl Iterator<Item = MonkeyStruct<usize>> + '_ {
    input.split("\n\n").map(|mstr| {
        let mut l = mstr.lines().skip(1);
        let starting_items = l.next().unwrap()["  Starting items: ".len()..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<VecDeque<_>>();
        let mut op_iter = l.next().unwrap()["  Operation: new = old ".len()..].split(' ');
        let (op, rhs) = (
            op_iter.next().unwrap().bytes().next().unwrap(),
            op_iter.next().unwrap(),
        );
        let cond = l.next().unwrap()["  Test: divisible by ".len()..]
            .parse::<usize>()
            .unwrap();
        let (cond_true, cond_false) = (
            l.next().unwrap()["    If true: throw to monkey ".len()..]
                .parse::<usize>()
                .unwrap(),
            l.next().unwrap()["    If false: throw to monkey ".len()..]
                .parse::<usize>()
                .unwrap(),
        );
        MonkeyStruct {
            items: starting_items,
            op,
            rhs,
            cond,
            cond_true,
            cond_false,
        }
    })
}

fn main() {
    let mut monkeys = parse_input(include_str!("../input.txt")).collect::<Vec<_>>();
    let monkeys_copy = monkeys.clone();

    let mut inspected = vec![0usize; monkeys.len()];
    for _ in 0..FIRST_ITER {
        for i in 0..monkeys.len() {
            let items_count = monkeys[i].items.len();
            inspected[i] += items_count;
            for _ in 0..items_count {
                let mut worry = monkeys[i].items.pop_front().unwrap();
                let rhs = match monkeys[i].rhs {
                    "old" => worry,
                    s => s.parse::<usize>().unwrap(),
                };
                match monkeys[i].op {
                    b'+' => worry += rhs,
                    b'*' => worry *= rhs,
                    _ => unreachable!(),
                }
                worry /= WORRY_LEVEL_DIV;
                let j = if worry % monkeys[i].cond == 0 {
                    monkeys[i].cond_true
                } else {
                    monkeys[i].cond_false
                };
                monkeys[j].items.push_back(worry);
            }
        }
    }

    inspected.sort();
    let tot = inspected.pop().unwrap() * inspected.pop().unwrap();
    println!("A: {}", tot);

    let mut monkeys = monkeys_copy
        .clone()
        .into_iter()
        .map(|m| {
            let mv = m
                .items
                .iter()
                .copied()
                .map(|v| {
                    (0..monkeys_copy.len())
                        .map(|i| v % monkeys_copy[i].cond)
                        .collect::<Vec<_>>()
                })
                .collect::<VecDeque<_>>();
            MonkeyStruct {
                items: mv,
                op: m.op,
                rhs: m.rhs,
                cond: m.cond,
                cond_true: m.cond_true,
                cond_false: m.cond_false,
            }
        })
        .collect::<Vec<_>>();

    let mut inspected = vec![0usize; monkeys.len()];
    for _ in 0..SECOND_ITER {
        for i in 0..monkeys.len() {
            let items_count = monkeys[i].items.len();
            inspected[i] += items_count;
            for _ in 0..items_count {
                let mut worry = monkeys[i].items.pop_front().unwrap();
                let mut rhs = match monkeys[i].rhs {
                    "old" => worry.clone(),
                    s => {
                        let n = s.parse::<usize>().unwrap();
                        (0..worry.len()).into_iter().map(|_| n).collect()
                    }
                };
                let opf: Box<dyn Fn(((&mut usize, usize), usize)) -> ()> = match monkeys[i].op {
                    b'+' => Box::new(|((w, i), rhs)| *w = (*w + rhs) % monkeys[i].cond),
                    b'*' => Box::new(|((w, i), rhs)| *w = (*w * rhs) % monkeys[i].cond),
                    _ => unreachable!(),
                };
                worry
                    .iter_mut()
                    .zip(0usize..)
                    .zip(rhs.drain(..))
                    .for_each(opf);
                let j = if worry[i] == 0 {
                    monkeys[i].cond_true
                } else {
                    monkeys[i].cond_false
                };
                monkeys[j].items.push_back(worry);
            }
        }
    }

    inspected.sort();
    println!(
        "B: {:?}",
        inspected.pop().unwrap() * inspected.pop().unwrap()
    );
}
