#![feature(test)]

extern crate test;

pub mod aggregate;
pub mod utils;

fn main() {
    let mut a = vec![vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]];
    let b = aggregate::aggregate(&mut a);
    println!("{:?}", b);
    assert_eq!(b, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let mut a = vec![vec![]];
        let mut ans = vec![];
        for _i in 0..10 {
            a[0].push(std::i32::MAX);
            ans.push(std::i32::MAX);
        }
        let b = aggregate::aggregate(&mut a);
        assert_eq!(b, ans);
    }

    #[test]
    fn test_min() {
        let mut a = vec![vec![]];
        let mut ans = vec![];
        for _i in 0..10 {
            a[0].push(std::i32::MIN);
            ans.push(std::i32::MIN);
        }
        let b = aggregate::aggregate(&mut a);
        assert_eq!(b, ans);
    }

    #[test]
    fn test_10x10() {
        let mut a = utils::read_vecvec("./tests/test1.in".to_string());
        let ans = utils::read_vec("./tests/test1.out".to_string());
        let b = aggregate::aggregate(&mut a);
        assert_eq!(b, ans);
    }
}
