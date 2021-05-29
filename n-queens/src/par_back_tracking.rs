use std::thread;

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

fn check(row: &[i32], n: usize) -> bool{
  for i in 0..n {
    if row[i] == -1 {
      continue;
    } else {
      for j in 0..i {
        if row[j] == -1 {continue;}
        if row[i] == row[j] || (row[i] - row[j]).abs() == (i-j) as i32 {
          return false;
        }
      }
    }
  }
  true
}

fn count(row: &mut [i32], now: usize, cnt: u64) -> u64 {
  if check(&row, now)==false {return cnt;}
  let mut _cnt: u64 = 0;
  let n = row.len() as usize;
  for i in 0..n {
    row[now] = i as i32;
    if now+1 == n {
      if check(&row, now+1) {
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

  // #[test]
  // fn one_queen() {
  //   assert_eq!(n_queens(1), 1);
  // }
  #[test]
  fn four_queens() {
    assert_eq!(n_queens(4), 2);
  }
  #[test]
  fn eight_queens() {
    assert_eq!(n_queens(8), 92);
  }
  #[test]
  fn ten_queens() {
    assert_eq!(n_queens(10), 724);
  }
}