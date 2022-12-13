use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
enum Elem {
    Value(u8),
    List(Vec<Elem>),
}

impl FromStr for Elem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut curr = Some(Elem::List(vec![]));
        let mut stack: Vec<Elem> = vec![];
        let mut bytes = s.bytes();
        while let Some(b) = bytes.next() {
            match b {
                b'[' => {
                    stack.push(curr.take().unwrap());
                    let _ = curr.insert(Elem::List(vec![]));
                }
                b']' => {
                    let mut parent = stack.pop();
                    match parent.as_mut() {
                        Some(Elem::List(v)) => v.push(curr.take().unwrap()),
                        _ => unreachable!(),
                    }
                    curr.replace(parent.unwrap());
                }
                b',' => (),
                n @ b'0'..=b'9' => {
                    if let Some(b'0') = bytes.clone().next() {
                        let _ = bytes.next();
                        curr.iter_mut().for_each(|v| match v {
                            Elem::List(v) => v.push(Elem::Value(10)),
                            _ => unreachable!(),
                        })
                    } else {
                        curr.iter_mut().for_each(|v| match v {
                            Elem::List(v) => v.push(Elem::Value(n - b'0')),
                            _ => unreachable!(),
                        })
                    }
                }
                _ => unreachable!(),
            }
        }
        match curr.take().unwrap() {
            Elem::List(mut v) => Ok(v.pop().unwrap()),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut input = include_str!("../input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(Elem::from_str)
        .filter_map(Result::ok);
    let input_clone = input.clone();
    let mut valid = 0;
    let mut count = 1;
    while let Some(a) = input.next() {
        let b = input.next().unwrap();
        if check_valid(a, b) == Ordering::Less {
            valid += count;
        }
        count += 1;
    }
    println!("A: {}", valid);

    let mut input = input_clone.collect::<Vec<Elem>>();
    let dividers = vec![
        Elem::List(vec![Elem::List(vec![Elem::Value(2)])]),
        Elem::List(vec![Elem::List(vec![Elem::Value(6)])]),
    ];
    input.extend(dividers.iter().cloned());
    input.sort_by(|a, b| check_valid(a.clone(), b.clone()));
    let b = input
        .iter()
        .zip(1..)
        .filter_map(|(e, i)| {
            let e = e.clone();
            if e == dividers[0] || e == dividers[1] {
                Some(i)
            } else {
                None
            }
        })
        .reduce(|acc, e| acc * e)
        .unwrap();
    println!("B: {}", b);
}

fn check_valid(a: Elem, b: Elem) -> Ordering {
    match a {
        Elem::Value(x) => match b {
            Elem::Value(y) => x.cmp(&y),
            Elem::List(y) => check_valid(Elem::List(vec![a]), Elem::List(y)),
        },
        Elem::List(x) => {
            if let Elem::List(y) = b {
                let mut y = y.into_iter();
                for x in x.into_iter() {
                    if let Some(y) = y.next() {
                        match check_valid(x, y) {
                            Ordering::Equal => continue,
                            o => return o,
                        }
                    } else {
                        return Ordering::Greater;
                    }
                }
                if y.next().is_some() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            } else {
                check_valid(Elem::List(x), Elem::List(vec![b]))
            }
        }
    }
}
