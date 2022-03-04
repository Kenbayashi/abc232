use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }

    let ans = s.into_iter()
               .zip(t.into_iter())
               .map(|(a, b)| (a as i32 - b as i32) % 26)
               .collect::<Vec<i32>>();

    println!("{}", if ans.iter().min() == ans.iter().max() {"Yes"} else {"No"});
}
