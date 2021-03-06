extern crate num;
use num::Complex;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
  let mut z = Complex { re: 0.0, im: 0.0 };
  for i in 0..limit {
    z = z * z + c;
    if z.norm_sqr() > 4.0 {
      return Some(i);
    }
  }
  None
}


use std::str::FromStr;
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T,T )> {
  match s.find(separator) {
    None => None,
    Some(index) => {
      match(T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
        (Ok(l), Ok(r)) => Some((l, r)),
        _ => None
      }
    }
  }
}

#[test]
fn test_parse_pair() {
  assert_eq!(parse_pair::<i32>("",        ','), None);
  assert_eq!(parse_pair::<i32>("10",      ','), None);
  assert_eq!(parse_pair::<i32>(",10",     ','), None);
  assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10,20)));
  assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
  assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
  assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5,1.5)));
}

fn main() {
    println!("Hello, world!");
}
