use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt").lines().map(|x| {
        let mut cube = x.split(',');
        (
            cube.next().unwrap().parse::<i32>().unwrap(),
            cube.next().unwrap().parse::<i32>().unwrap(),
            cube.next().unwrap().parse::<i32>().unwrap(),
        )
    });

    let cubes = input.collect::<HashSet<_>>();
    let adj_cubes = cubes.iter().flat_map(|&x| {
        adjacent_cubes(x)
            .into_iter()
            .filter(|&x| !cubes.contains(&x))
    });
    let surface = adj_cubes.clone().count();
    println!("A: {}", surface);

    let bounds = cubes
        .iter()
        .fold(((0, 0), (0, 0), (0, 0)), |acc, &(x, y, z)| {
            (
                (acc.0 .0.min(x), acc.0 .1.max(x)),
                (acc.1 .0.min(y), acc.1 .1.max(y)),
                (acc.2 .0.min(z), acc.2 .1.max(z)),
            )
        });
    let cube_bounds = (
        bounds.0 .0.min(bounds.1 .0).min(bounds.2 .0),
        bounds.0 .1.max(bounds.1 .1).max(bounds.2 .1),
    );
    let mut filling: HashSet<(i32, i32, i32)> = HashSet::new();

    fill(
        &cubes,
        &mut filling,
        (cube_bounds.0 - 1, cube_bounds.1 + 1),
        (cube_bounds.0 - 1, cube_bounds.0 - 1, cube_bounds.0 - 1),
    );

    let c = adj_cubes.filter(|x| !filling.contains(x)).count();
    println!("B: {}", surface - c);
}

fn fill(
    cubes: &HashSet<(i32, i32, i32)>,
    filled: &mut HashSet<(i32, i32, i32)>,
    cube_bounds: (i32, i32),
    curr: (i32, i32, i32),
) {
    if !filled.insert((curr.0, curr.1, curr.2)) {
        return;
    }
    for x in [1, -1] {
        if !cubes.contains(&(curr.0 + x, curr.1, curr.2))
            && ((cube_bounds.0..=cube_bounds.1).contains(&(curr.0 + x))
                && (cube_bounds.0..=cube_bounds.1).contains(&(curr.1))
                && (cube_bounds.0..=cube_bounds.1).contains(&(curr.2)))
        {
            fill(cubes, filled, cube_bounds, (curr.0 + x, curr.1, curr.2));
        }
    }
    for y in [1, -1] {
        if !cubes.contains(&(curr.0, curr.1 + y, curr.2))
            && ((cube_bounds.0..=cube_bounds.1).contains(&(curr.0))
                && (cube_bounds.0..=cube_bounds.1).contains(&(curr.1 + y))
                && (cube_bounds.0..=cube_bounds.1).contains(&(curr.2)))
        {
            fill(cubes, filled, cube_bounds, (curr.0, curr.1 + y, curr.2));
        }
    }
    for z in [1, -1] {
        if !cubes.contains(&(curr.0, curr.1, curr.2 + z))
            && ((cube_bounds.0..=cube_bounds.1).contains(&(curr.0))
                && (cube_bounds.0..=cube_bounds.1).contains(&(curr.1))
                && (cube_bounds.0..=cube_bounds.1).contains(&(curr.2 + z)))
        {
            fill(cubes, filled, cube_bounds, (curr.0, curr.1, curr.2 + z));
        }
    }
}

fn adjacent_cubes(a: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (a.0 - 1, a.1, a.2),
        (a.0 + 1, a.1, a.2),
        (a.0, a.1 - 1, a.2),
        (a.0, a.1 + 1, a.2),
        (a.0, a.1, a.2 - 1),
        (a.0, a.1, a.2 + 1),
    ]
}
