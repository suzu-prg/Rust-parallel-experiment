#![feature(test)]

extern crate test;

pub mod product;
pub mod utils;

fn main() {
    let a = vec![vec![0, 1, 2], vec![1, 0, -1], vec![3, 2, 1]];
    let b = vec![vec![3, 5, 6], vec![1, 1, 2], vec![3, 5, 8]];
    let c = product::product(&a, &b);
    println!("{:?}", c);
    assert_eq!(c, vec![vec![7, 11, 18], vec![0, 0, -2], vec![14, 22, 30]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton() {
        let a = vec![vec![3]];
        let b = vec![vec![5]];
        let c = product::product(&a, &b);
        assert_eq!(c, vec![vec![15]]);
    }

    #[test]
    fn test_id() {
        let a = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let b = vec![vec![3, 5, 6], vec![1, 1, 2], vec![3, 5, 8]];
        let c = product::product(&a, &b);
        assert_eq!(c, vec![vec![3, 5, 6], vec![1, 1, 2], vec![3, 5, 8]]);
    }

    #[test]
    fn test_zero() {
        let a = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let b = vec![vec![3, 5, 6], vec![1, 1, 2], vec![3, 5, 8]];
        let c = product::product(&a, &b);
        assert_eq!(c, vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
    }

    #[test]
    fn test_32x24() {
        let a = vec![vec![2, 3], vec![1, 4], vec![2, 1]];
        let b = vec![vec![3, 1, 2, 1], vec![2, 4, 2, 4]];
        let c = product::product(&a, &b);
        assert_eq!(
            c,
            vec![vec![12, 14, 10, 14], vec![11, 17, 10, 17], vec![8, 6, 6, 6]]
        );
    }
}
