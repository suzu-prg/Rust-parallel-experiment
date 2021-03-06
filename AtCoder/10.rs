// AtCoder に登録したら解くべき精選過去問 10 問を Rust で解いてみた
// https://qiita.com/tubo28/items/e6076e9040da57368845

use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char) 
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
  let n = read();
  let mut v: Vec<(i32, i32, i32)> = (0..n).map(|_| (read(), read(), read())).collect();
  v.insert(0, (0,0,0));
  let yes = v[..].windows(2).all(|w| {
    let (t, x, y) = w[0];
    let (nt, nx, ny) = w[1];
    let time = nt - t;
    let dist = (nx - x).abs() + (ny - y).abs();
    dist <= time && time % 2 == dist % 2
  });
  println!("{}", if yes {"Yes"} else{"No"});
}
