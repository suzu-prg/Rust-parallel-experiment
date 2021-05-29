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
  let n: u32 = read();
  let mut ans: u32 = 1000_000_000;
  for _ in 0..n {
    let mut a: u32 = read();
    let mut cnt = 0;
    while a % 2 == 0 {
      a /= 2;
      cnt += 1;
    }
    ans = if cnt < ans { cnt } else { ans };
  }
  println!("{}", ans);
}
