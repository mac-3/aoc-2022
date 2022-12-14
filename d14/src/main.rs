use std::collections::HashSet;

const SAND_SOURCE: (u32, u32) = (500, 0);

fn main() {
    let mut points = HashSet::new();
    include_str!("../input.txt")
        .lines()
        .map(|s| {
            s.split(" -> ").map(|s| {
                let mut s = s.split(',');
                (
                    s.next().unwrap().parse::<u32>().unwrap(),
                    s.next().unwrap().parse::<u32>().unwrap(),
                )
            })
        })
        .for_each(|mut p| {
            let mut curr = p.next().unwrap();
            for p in p {
                for x in curr.0.min(p.0)..=curr.0.max(p.0) {
                    points.insert((x, curr.1));
                }
                for y in curr.1.min(p.1)..=curr.1.max(p.1) {
                    points.insert((curr.0, y));
                }
                curr = p;
            }
        });
    let floor_level = points
        .iter()
        .map(|x| x.1)
        .reduce(|acc, x| acc.max(x))
        .unwrap();

    let mut count = 0;
    'outer: loop {
        let (mut x, mut y) = SAND_SOURCE;
        while y < floor_level {
            if let None = points.get(&(x, y + 1)) {
                y += 1;
            } else if let None = points.get(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if let None = points.get(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                points.insert((x, y));
                count += 1;
                continue 'outer;
            }
        }
        break;
    }
    println!("A: {}", count);

    'outer: loop {
        let (mut x, mut y) = SAND_SOURCE;
        while y < floor_level {
            if points.get(&(x, y + 1)).is_none() {
                y += 1;
            } else if points.get(&(x - 1, y + 1)).is_none() {
                x -= 1;
                y += 1;
            } else if points.get(&(x + 1, y + 1)).is_none() {
                x += 1;
                y += 1;
            } else if (x, y) == SAND_SOURCE {
                count += 1;
                break 'outer;
            } else {
                break;
            }
        }
        points.insert((x, y));
        count += 1;
    }
    println!("B: {}", count);
}
