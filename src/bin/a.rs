use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let (a, b) = (String::from(s[0]), String::from(s[2]));

    let a = a.parse::<i64>().ok().unwrap();
    let b = b.parse::<i64>().ok().unwrap();

    println!("{}", a * b);
}
