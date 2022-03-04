use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [(usize, usize); m],
        c: [(usize, usize); m],
    }

    let perms = (1..=n).into_iter()
                      .permutations(n)
                      .map(|perm| a.iter()
                                   .map(|&(left, right)| (perm[left - 1], perm[right - 1]))
                                   .collect::<Vec<(usize, usize)>>())
                      .map(|perm| perm.into_iter()
                                      .map(|(left, right)| if left < right {(left, right)} else {(right, left)})
                                      .collect::<Vec<(usize, usize)>>())
                      .collect::<Vec<Vec<(usize, usize)>>>();

    let ans = perms.into_iter()
                   .map(|perm| perm.into_iter()
                                   .fold(true, |acc, stri| acc && c.contains(&stri)))
                   .any(|result| result);

    println!("{}", if ans {"Yes"} else {"No"});
}
