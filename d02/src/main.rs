const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSS_SCORE: u32 = 0;

const SCORE_MULT: i8 = 3;

fn main() {
    let total_score = load_splitted_input().fold([0u32; 2], |mut acc, x| {
        let winner = x[0] - x[1];
        if winner == 0 {
            acc[0] += x[0] as u32 + DRAW_SCORE;
            acc[1] += x[0] as u32 + DRAW_SCORE;
        } else {
            let winner = if winner < 0 {
                (winner.abs() % 2) as usize
            } else {
                ((winner - 1).abs() % 2) as usize
            };
            acc[winner] += x[winner] as u32 + WIN_SCORE;
            acc[(winner + 1) % 2] += x[(winner + 1) % 2] as u32 + LOSS_SCORE;
        }
        acc
    });
    println!("A: {}", total_score[1]);

    let total_score = load_splitted_input().fold(0u32, |acc, x| {
        let play = match x[1] {
            // Loss
            1 => ((x[0] + 1) % 3) + 1,
            // Draw
            2 => x[0],
            // Win
            3 => ((x[0]) % 3) + 1,
            _ => unreachable!(),
        };
        acc + (play + (x[1] - 1) * SCORE_MULT) as u32
    });
    println!("B: {}", total_score);
}

fn load_splitted_input() -> impl Iterator<Item = [i8; 2]> {
    include_str!("../input.txt").lines().map(|l| {
        let mut chars = l.split(' ');
        [
            chars
                .next()
                .map(|x| x.chars().next().unwrap() as i8 - 'A' as i8 + 1)
                .unwrap(),
            chars
                .next()
                .map(|x| x.chars().next().unwrap() as i8 - 'X' as i8 + 1)
                .unwrap(),
        ]
    })
}
