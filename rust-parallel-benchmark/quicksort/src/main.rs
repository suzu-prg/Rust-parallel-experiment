#![feature(test)]

extern crate test;

pub mod quicksort;
pub mod utils;

fn main() {
    let mut a = vec![3, 2, 1];
    quicksort::quicksort(&mut a);
    println!("{:?}", a);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_pos() {
        let mut a = vec![3, 2, 1];
        quicksort::quicksort(&mut a);
        assert_eq!(a, [1, 2, 3]);
    }

    #[test]
    fn test_neg() {
        let mut a = vec![-1, -2, -3];
        quicksort::quicksort(&mut a);
        assert_eq!(a, [-3, -2, -1]);
    }

    #[test]
    fn test_int() {
        let mut a = vec![10, 0, -10];
        quicksort::quicksort(&mut a);
        assert_eq!(a, [-10, 0, 10]);
    }

    #[test]
    fn test_id() {
        let mut a = vec![1, 2, 3];
        quicksort::quicksort(&mut a);
        assert_eq!(a, [1, 2, 3]);
    }

    #[test]
    fn test_same_v() {
        let mut a = vec![1, 1, 1];
        quicksort::quicksort(&mut a);
        assert_eq!(a, [1, 1, 1]);
    }

    #[test]
    fn test_min_max() {
        let mut a = vec![std::i32::MAX, 0, std::i32::MIN];
        quicksort::quicksort(&mut a);
        assert_eq!(a, [std::i32::MIN, 0, std::i32::MAX]);
    }

    #[bench]
    fn bench1(b: &mut Bencher) {
        let testin = utils::read_vec("./tests/bench1.in".to_string());
        let testout = utils::read_vec("./tests/bench1.out".to_string());
        b.iter(|| {
            let mut tmpin: Vec<i32> = Vec::new();
            utils::copy(&testin, &mut tmpin);
            quicksort::quicksort(&mut tmpin);
            assert_eq!(tmpin, testout);
        });
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        let testin = utils::read_vec("./tests/bench2.in".to_string());
        let testout = utils::read_vec("./tests/bench2.out".to_string());
        b.iter(|| {
            let mut tmpin: Vec<i32> = Vec::new();
            utils::copy(&testin, &mut tmpin);
            quicksort::quicksort(&mut tmpin);
            assert_eq!(tmpin, testout);
        });
    }

    #[bench]
    fn bench3(b: &mut Bencher) {
        let testin = utils::read_vec("./tests/bench3.in".to_string());
        let testout = utils::read_vec("./tests/bench3.out".to_string());
        b.iter(|| {
            let mut tmpin: Vec<i32> = Vec::new();
            utils::copy(&testin, &mut tmpin);
            quicksort::quicksort(&mut tmpin);
            assert_eq!(tmpin, testout);
        });
    }
}
