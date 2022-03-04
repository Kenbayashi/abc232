use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let directions = (0..h + w - 2).into_iter()
                               .combinations(h - 1)
                               .collect::<Vec<Vec<usize>>>();

    let mut ans: u32 = 1;

    for direction in directions.into_iter() {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut current: u32 = 1;

        for count in 0..h + w - 2 {
            if direction.contains(&count) {
                y += 1;
            } else {
                x += 1;
            }

            if c[y][x] == '.' {
                current += 1;
                ans = ans.max(current);
            } else {
                break;
            }
        }
    }

    println!("{}", ans);
}
