use std::collections::HashMap;

const SAMPLE_Y: i32 = 2_000_000;
const DISTRESS_UPPER_BOUND: i32 = 4_000_000;

fn main() {
    let mut visited = vec![];
    let input = include_str!("../input.txt").lines().map(|l| {
        let (sx, sy, bx, by) = scan_fmt::scan_fmt!(
            l,
            "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();
        (sx, sy, bx, by, manhattan_distance((sx, sy), (bx, by)))
    });
    input.clone().for_each(|(x, y, bx, by, d)| {
        let r = (y - SAMPLE_Y).abs();
        if y == SAMPLE_Y {
            visited.push(x..=x);
        }
        if by == SAMPLE_Y {
            visited.push(bx..=bx);
        }
        if r <= d {
            let r = (r - d).abs();
            let (mut rx, mut ry) = (x - r, x + r);
            let v = visited.drain(..).filter_map(|x| {
                if x.start() <= &rx && x.end() <= &ry
                    || x.start() <= &rx && x.end() >= &ry
                    || x.start() > &rx && x.end() <= &rx
                {
                    rx = *x.start().min(&rx);
                    ry = *x.end().max(&ry);
                    None
                } else {
                    Some(x)
                }
            });
            visited = v.collect();
            visited.push(rx..=ry);
        }
    });
    let empty = visited
        .into_iter()
        .map(|x| (x.end() - x.start()).abs())
        .sum::<i32>();
    println!("A: {}", empty);

    let mut completed = [false; (DISTRESS_UPPER_BOUND + 1) as usize];
    let mut visited: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for (sx, sy, bx, by, d) in input {
        if (0..=DISTRESS_UPPER_BOUND).contains(&sx) && (0..=DISTRESS_UPPER_BOUND).contains(&sy) {
            let v = transform(visited.entry(sy).or_default().drain(..), (sx, sx));
            if v.len() == 1 && v[0] == (0, DISTRESS_UPPER_BOUND) {
                completed[sy as usize] = true;
                visited.remove(&sy);
            } else {
                visited.insert(sy, v);
            }
        }
        if (0..=DISTRESS_UPPER_BOUND).contains(&bx) && (0..=DISTRESS_UPPER_BOUND).contains(&by) {
            let v = transform(visited.entry(by).or_default().drain(..), (bx, bx));
            if v.len() == 1 && v[0] == (0, DISTRESS_UPPER_BOUND) {
                completed[by as usize] = true;
                visited.remove(&by);
            } else {
                visited.insert(by, v);
            }
        }
        for i in -d..=d {
            if sy + i >= 0 && sy + i <= DISTRESS_UPPER_BOUND {
                let sy = sy + i;
                let r = d - i.abs();
                let (mut rx, mut ry) = (sx - r, sx + r);
                if rx <= DISTRESS_UPPER_BOUND || ry >= 0 {
                    rx = rx.clamp(0, DISTRESS_UPPER_BOUND);
                    ry = ry.clamp(0, DISTRESS_UPPER_BOUND);
                    let v = transform(visited.entry(sy).or_default().drain(..), (rx, ry));
                    if v.len() == 1 && v[0] == (0, DISTRESS_UPPER_BOUND) {
                        completed[sy as usize] = true;
                        visited.remove(&sy);
                    } else {
                        visited.insert(sy, v);
                    }
                }
            }
        }
    }

    let y = completed
        .iter()
        .zip(0..)
        .filter(|(&x, _)| !x)
        .map(|(_, i)| i)
        .next()
        .unwrap();
    let mut v = visited.remove(&y).unwrap();
    v.sort();
    let x = v[0].1 + 1;
    println!(
        "B: {}",
        x as isize * DISTRESS_UPPER_BOUND as isize + y as isize
    );
}

fn transform(iter: impl Iterator<Item = (i32, i32)>, interval: (i32, i32)) -> Vec<(i32, i32)> {
    let mut interval = interval;
    let mut v = iter
        .filter_map(|(x, y)| intersect_adjust(&mut interval.0, &mut interval.1, x, y))
        .collect::<Vec<_>>();
    v.push(interval);
    v
}

fn intersect_adjust(ax: &mut i32, ay: &mut i32, bx: i32, by: i32) -> Option<(i32, i32)> {
    if *ax <= bx && *ay >= bx || *ax >= bx && *ax <= by || *ax <= by && *ay >= by {
        *ax = (*ax).min(bx);
        *ay = (*ay).max(by);
        return None;
    }
    Some((bx, by))
}

#[inline]
fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
