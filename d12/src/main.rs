use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let mut nodes = HashMap::new();
    let (mut starting_point, mut end_point) = (None, None);
    let mut w = 0;
    let h = include_str!("../input.txt")
        .lines()
        .zip(0usize..)
        .map(|(x, i)| {
            w = x
                .bytes()
                .zip(0usize..)
                .map(|(x, j)| {
                    match x {
                        b'S' => {
                            starting_point = Some((i, j));
                            nodes.insert((i, j), b'a');
                        }
                        b'E' => {
                            end_point = Some((i, j));
                            nodes.insert((i, j), b'z');
                        }
                        b => {
                            nodes.insert((i, j), b);
                        }
                    };
                })
                .count()
        })
        .count();
    let w = w;

    let (starting_point, end_point) = (starting_point.unwrap(), end_point.unwrap());
    let distances = djkstra(starting_point, end_point, h, w, &nodes);
    println!("A: {}", distances[&end_point]);

    let b = nodes
        .iter()
        .filter_map(|(&p, &v)| if v == b'a' { Some(p) } else { None })
        .filter_map(|x| djkstra(x, end_point, h, w, &nodes).get(&end_point).cloned())
        .min()
        .unwrap();
    println!("B: {}", b);
}

fn djkstra(
    start: (usize, usize),
    end: (usize, usize),
    h: usize,
    w: usize,
    nodes: &HashMap<(usize, usize), u8>,
) -> HashMap<(usize, usize), usize> {
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut visited_nodes = HashSet::new();
    queue.push(Node {
        idx: start,
        dist: 0,
    });
    while let Some(Node { idx: curr, dist }) = queue.pop() {
        if curr == end {
            break;
        }
        if !visited_nodes.insert(curr) {
            continue;
        }
        get_adj(curr, (h, w))
            .filter(|p| !visited_nodes.contains(p) && nodes[&curr] >= nodes[p] - 1)
            .for_each(|adj| {
                let new_dist = dist + 1;
                let shorter = distances.get(&adj).map_or(true, |&x| new_dist < x);
                if shorter {
                    distances.insert(adj, new_dist);
                    queue.push(Node {
                        idx: adj,
                        dist: new_dist,
                    })
                }
            })
    }
    distances
}

#[derive(Debug, Eq, Clone, Copy)]
struct Node {
    idx: (usize, usize),
    dist: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

fn get_adj(p: (usize, usize), s: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let mut v = Vec::with_capacity(4);
    if p.0 > 0 {
        v.push((p.0 - 1, p.1))
    }
    if p.0 < s.0 - 1 {
        v.push((p.0 + 1, p.1));
    }
    if p.1 > 0 {
        v.push((p.0, p.1 - 1));
    }
    if p.1 < s.1 - 1 {
        v.push((p.0, p.1 + 1));
    }
    v.into_iter()
}
