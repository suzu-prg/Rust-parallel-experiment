pub fn n_queens(n: u32) -> u64 {
  let mut row = vec![-1; n as usize]; // 各行のクイーンの位置 [0,n)
  count(&mut row, 0, 0)

  // 0で初期化，中の数字はそのマスから到達できるクイーンの数を表す．
  // let mut board: Vec<Vec<u32>> = vec![vec![0; n]; n];

  // println!("{}-Queens", n);
  // for i in 0..n {
  //   for j in 0..n {
  //     print!("{}", if board[i][j] { 1 } else { 0 });
  //   }
  //   println!("");
  // }
}

fn check(row: &[i32]) -> bool{
  let n = row.len() as usize;
  for i in 0..n {
    for j in 0..i {
      if row[i] == row[j] || (row[i] - row[j]).abs() == (i-j) as i32 {
        return false;
      }
    }
  }
  true
}


fn count(row: &mut [i32], now: usize, cnt: u64) -> u64 {
  let mut _cnt: u64 = 0;
  let n = row.len() as usize;
  for i in 0..n {
    row[now] = i as i32;
    if now+1 == n {
      if check(&row) {
        _cnt += 1;
        // println!("a");
      }
    } else {
      _cnt += count(row, now+1, cnt);
    }
  }
  cnt + _cnt
}

#[cfg(test)]
mod tests {
  use super::n_queens;

  #[test]
  fn one_queen() {
    assert_eq!(n_queens(1), 1);
  }
  #[test]
  fn four_queens() {
    assert_eq!(n_queens(4), 2);
  }
  #[test]
  fn eight_queens() {
    assert_eq!(n_queens(8), 92);
  }
}