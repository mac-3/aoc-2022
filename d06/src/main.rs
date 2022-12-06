use std::collections::HashSet;

const PACKET_WINDOW_SIZE: usize = 4;
const MESSAGE_WINDOW_SIZE: usize = 14;

fn main() {
    let input = include_str!("../input.txt");
    let (_, first) = input
        .as_bytes()
        .windows(PACKET_WINDOW_SIZE)
        .zip(0..)
        .skip_while(|(v, _)| marker_neg(v, PACKET_WINDOW_SIZE))
        .next()
        .unwrap();
    println!("A: {}", first + PACKET_WINDOW_SIZE);

    let (_, first) = input
        .as_bytes()
        .windows(MESSAGE_WINDOW_SIZE)
        .zip(0..)
        .skip_while(|(v, _)| marker_neg(v, MESSAGE_WINDOW_SIZE))
        .next()
        .unwrap();
    println!("B: {}", first + MESSAGE_WINDOW_SIZE);
}

fn marker_neg(v: &[u8], dist_len: usize) -> bool {
    v.iter().copied().collect::<HashSet<u8>>().len() != dist_len
}
