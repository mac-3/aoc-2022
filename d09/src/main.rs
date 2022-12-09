use std::{collections::HashSet, str::FromStr};

const ROPE_LEN: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: u8,
    units: i32,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        Ok(Move {
            direction: split.next().unwrap().bytes().next().unwrap(),
            units: split.next().map(|x| x.parse().unwrap()).unwrap(),
        })
    }
}

fn move_part(h: (i32, i32), t: &mut (i32, i32)) {
    if h.0.abs_diff(t.0) > 1 {
        t.0 += (h.0 - t.0).clamp(-1, 1);
        if h.1.abs_diff(t.1) != 0 {
            t.1 += (h.1 - t.1).clamp(-1, 1);
        }
    }
    if h.1.abs_diff(t.1) > 1 {
        t.1 += (h.1 - t.1).clamp(-1, 1);
        if h.0.abs_diff(t.0) != 0 {
            t.0 += (h.0 - t.0).clamp(-1, 1);
        }
    }
}

fn move_head(h: &mut (i32, i32), m: Move) {
    match m.direction {
        b'U' => h.1 += m.units,
        b'R' => h.0 += m.units,
        b'D' => h.1 -= m.units,
        b'L' => h.0 -= m.units,
        _ => unreachable!(),
    };
}

fn main() {
    let moves = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<Move>().unwrap());
    let mut visited: HashSet<(i32, i32)> = HashSet::default();

    let (mut hpos, mut tpos) = ((0i32, 0i32), (0i32, 0i32));
    visited.insert(tpos);
    moves.clone().for_each(|m| {
        move_head(&mut hpos, m);
        for _ in 0..m.units {
            move_part(hpos, &mut tpos);
            visited.insert(tpos);
        }
    });
    println!("A: {}", visited.len());

    visited.drain();
    let mut rope = [(0i32, 0i32); ROPE_LEN];
    visited.insert(rope[ROPE_LEN - 1]);
    moves.for_each(|m| {
        move_head(&mut rope[0], m);
        for _ in 0..m.units {
            (1..ROPE_LEN).for_each(|i| move_part(rope[i - 1], &mut rope[i]));
            visited.insert(rope[ROPE_LEN - 1]);
        }
    });
    println!("B: {}", visited.len());
}
