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
  let n: i32 = read();
  let x: i32 = read();
  let mut ans = None;
  'outer: 
  for i in 0..n+1 {
    for j in 0..n-i+1 {
      let k = n - i - j;
      if i * 10000 + j * 5000 + k * 1000 == x {
        ans = Some((i,j,k));
        break 'outer;
      }
    }
  }
  let (x, y, z) = ans.unwrap_or((-1,-1,-1));
  println!("{} {} {}", x, y, z);
}
