use std::collections::HashSet;

const SAMPLE_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
const CRT_W: i32 = 40;
const CRT_H: i32 = 6;
const CRT_DIM: i32 = CRT_W * CRT_H;

fn main() {
    let sampling_set = HashSet::from(SAMPLE_CYCLES);
    let mut x_reg = 1;
    let mut tot = 0;
    let mut crt = (0..CRT_DIM).map(|_| b'.').collect::<Vec<u8>>();
    include_str!("../input.txt")
        .lines()
        .flat_map(|l| {
            let mut split = l.split(' ');
            match split.next().unwrap() {
                "noop" => vec![0],
                "addx" => vec![0, split.next().unwrap().parse::<i32>().unwrap()],
                _ => unreachable!(),
            }
        })
        .zip(1i32..)
        .for_each(|(a, i)| {
            if ((i - 1) % CRT_W).abs_diff(x_reg) <= 1 {
                let i = (i - 1) % CRT_DIM;
                crt[i as usize] = b'#';
            }
            if sampling_set.contains(&i) {
                tot += x_reg * i;
            }
            x_reg += a;
        });
    println!("A: {}", tot);

    print!("B:\t");
    for r in 0..CRT_H as usize {
        for c in 0..CRT_W as usize {
            print!("{}", crt[r * CRT_W as usize + c] as char);
        }
        print!("\n\t");
    }
}
