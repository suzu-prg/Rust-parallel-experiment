#![feature(test)]

extern crate test;

pub mod nqueen;

fn main() {
    let n = 2;
    let ans = nqueen::nqueen(n);
    println!("{}", ans);
    assert_eq!(ans, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 1;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test_2() {
        let n = 2;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_3() {
        let n = 3;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_4() {
        let n = 4;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_5() {
        let n = 5;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 10);
    }

    #[test]
    fn test_6() {
        let n = 6;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_7() {
        let n = 7;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 40);
    }

    #[test]
    fn test_8() {
        let n = 8;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 92);
    }

    #[test]
    fn test_9() {
        let n = 9;
        let ans = nqueen::nqueen(n);
        assert_eq!(ans, 352);
    }
}
