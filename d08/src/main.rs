use std::collections::{BinaryHeap, HashSet};

fn visibility_run<L1, L2, F1, F2, F3, F4>(
    input: &[Vec<u8>],
    visibility_set: &mut HashSet<(usize, usize)>,
    h: usize,
    w: usize,
    l1: L1,
    l2: L2,
    rhsx: F1,
    rhsy: F2,
    lhsx: F3,
    lhsy: F4,
) where
    L1: Iterator<Item = usize>,
    L2: Iterator<Item = usize> + Clone,
    F1: Fn(usize, usize) -> usize,
    F2: Fn(usize, usize) -> usize,
    F3: Fn(usize, usize) -> usize,
    F4: Fn(usize, usize) -> usize,
{
    let mut precalc = vec![vec![None; w]; h];
    for i in l1 {
        for j in l2.clone() {
            if let Some(n) = precalc[rhsx(i, j)][rhsy(i, j)] {
                if n > input[rhsx(i, j)][rhsy(i, j)] {
                    precalc[lhsx(i, j)][lhsy(i, j)] = Some(n);
                } else {
                    precalc[lhsx(i, j)][lhsy(i, j)] = Some(input[rhsx(i, j)][rhsy(i, j)])
                }
            } else {
                precalc[lhsx(i, j)][lhsy(i, j)] = Some(input[rhsx(i, j)][rhsy(i, j)])
            }
        }
    }

    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            if input[i][j] > precalc[i][j].unwrap() {
                visibility_set.insert((i, j));
            }
        }
    }
}

fn scenic_score<L1, L2, F1, F2>(
    input: &[Vec<u8>],
    prod: &mut Vec<Vec<usize>>,
    l1: L1,
    l2: L2,
    x: F1,
    y: F2,
) where
    L1: Iterator<Item = usize>,
    L2: Iterator<Item = usize> + Clone,
    F1: Fn(usize, usize) -> usize,
    F2: Fn(usize, usize) -> usize,
{
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct StackElem {
        idx: (usize, usize),
        value: u8,
    }
    impl PartialOrd for StackElem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.value.partial_cmp(&self.value)
        }
    }
    impl Ord for StackElem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.value.cmp(&self.value)
        }
    }

    let (h, w) = (input.len(), input[0].len());
    let mut matr = vec![vec![0usize; w]; h];
    let mut stack: BinaryHeap<StackElem> = BinaryHeap::default();
    for i in l1 {
        for j in l2.clone() {
            stack.iter().for_each(|e| {
                matr[e.idx.0][e.idx.1] += 1;
            });
            while let Some(e) = stack.peek().copied() {
                if e.value <= input[x(i, j)][y(i, j)] {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(StackElem {
                idx: (x(i, j), y(i, j)),
                value: input[x(i, j)][y(i, j)],
            });
        }
        stack.drain();
    }

    for i in 0..h {
        for j in 0..w {
            prod[i][j] *= matr[i][j];
        }
    }
}

fn main() {
    let input: Vec<Vec<u8>> = include_str!("../input.txt")
        .lines()
        .map(|x| x.bytes().map(|c| c - b'0').collect())
        .collect();
    let perimeter = (input.len() + input[0].len() - 2) * 2;
    let mut visible: HashSet<(usize, usize)> = HashSet::default();

    visibility_run(
        &input,
        &mut visible,
        input.len(),
        input[0].len(),
        1..input[0].len() - 1,
        1..input.len() - 1,
        |_, j| j - 1,
        |i, _| i,
        |_, j| j,
        |i, _| i,
    );

    visibility_run(
        &input,
        &mut visible,
        input.len(),
        input[0].len(),
        1..input[0].len() - 1,
        (1..input.len() - 1).rev(),
        |_, j| j + 1,
        |i, _| i,
        |_, j| j,
        |i, _| i,
    );

    visibility_run(
        &input,
        &mut visible,
        input.len(),
        input[0].len(),
        1..input.len() - 1,
        1..input[0].len() - 1,
        |i, _| i,
        |_, j| j - 1,
        |i, _| i,
        |_, j| j,
    );

    visibility_run(
        &input,
        &mut visible,
        input.len(),
        input[0].len(),
        1..input.len() - 1,
        (1..input[0].len() - 1).rev(),
        |i, _| i,
        |_, j| j + 1,
        |i, _| i,
        |_, j| j,
    );

    println!("A: {}", visible.len() + perimeter);

    let mut prod = vec![vec![1usize; input[0].len()]; input.len()];

    scenic_score(
        &input,
        &mut prod,
        0..input[0].len(),
        0..input.len(),
        |_, j| j,
        |i, _| i,
    );

    scenic_score(
        &input,
        &mut prod,
        0..input[0].len(),
        (0..input.len()).rev(),
        |_, j| j,
        |i, _| i,
    );

    scenic_score(
        &input,
        &mut prod,
        0..input.len(),
        0..input[0].len(),
        |i, _| i,
        |_, j| j,
    );

    scenic_score(
        &input,
        &mut prod,
        0..input.len(),
        (0..input[0].len()).rev(),
        |i, _| i,
        |_, j| j,
    );

    println!("B: {}", prod.iter().flatten().max().unwrap());
}
