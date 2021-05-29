// 各手ごとに2分割，コピーを挟むせいで遅くなっている

extern crate rayon;
use std::thread;
use rayon::prelude::*;

pub fn n_queens(n: u32) -> u64 {
  let mut thread_handles = vec![];
  for i in 0..(n as usize){
    thread_handles.push(
      thread::spawn(move || {
        let mut row = vec![-1; n as usize];
        row[0] = i as i32;
        count(&mut row, 1, 0)
      })
    );
  }
  let mut ans = 0;
  for handle in thread_handles {
    ans += handle.join().unwrap()
    // println!("{:?}", handle.join().unwrap());
  }
  // println!("{:?}", ans);
  ans
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
  let n = row.len() as usize;
  let mut row_ = vec![-1; n as usize];
  for i in 0..n {
    row_[i] = row[i];
  }
  let mut row__ = vec![-1; n as usize];
  for i in 0..n {
    row__[i] = row[i];
  }
  let (cnt1, cnt2) = 
    rayon::join(|| count_(&mut row_, now, cnt, true),
                || count_(&mut row__, now, cnt, false));
  cnt1 + cnt2
}

fn count_(row: &mut [i32], now: usize, cnt: u64, mode: bool) -> u64 {
  let mut _cnt: u64 = 0;
  let n = row.len() as usize;
  if mode {
    for i in 0..n/2 {
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
  } else {
    for i in n/2..n {
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