// この関数を並列化する
pub fn aggregate(a: &mut Vec<Vec<i32>>) -> Vec<i32> {
    let mut tmp: Vec<i32> = Vec::new();
    let mut res: Vec<i32> = Vec::new();

    let n = a.len();
    let mut m = 0;
    for i in 0..n {
        m += a[i].len();
        assert!(a[i].len() != 0);
        tmp.append(&mut a[i]);
    }

    tmp.sort();

    for i in 0..10 {
        res.push(tmp[i * m / 10]);
    }

    return res;
}
