fn main() {
    let input = include_str!("../input.txt").lines().map(|l| {
        l.split(',')
            .map(|s| s.split('-').map(|n| n.parse::<u8>().unwrap()))
            .fold(vec![], |mut acc, mut e| {
                acc.push((e.next().unwrap(), e.next().unwrap()));
                acc
            })
    });

    let a = input
        .clone()
        .filter(|v| {
            (v[0].0 <= v[1].0 && v[0].1 >= v[1].1) || (v[1].0 <= v[0].0 && v[1].1 >= v[0].1)
        })
        .count();
    println!("A: {}", a);

    let b = input
        .filter(|v| {
            (v[0].1 >= v[1].0 && v[0].0 < v[1].0 || v[0].0 <= v[1].1 && v[0].1 > v[1].1)
                || (v[1].1 >= v[0].0 && v[1].0 < v[0].0 || v[1].0 <= v[0].1 && v[1].1 > v[0].1)
                || (v[0].0 <= v[1].0 && v[0].1 >= v[1].1)
                || (v[1].0 <= v[0].0 && v[1].1 >= v[0].1)
        })
        .count();
    println!("B: {}", b);
}
