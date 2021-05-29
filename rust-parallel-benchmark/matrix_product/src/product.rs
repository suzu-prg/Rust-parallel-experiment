// この関数を並列化する
pub fn product(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let l = a.len();
    let m = b.len();
    let n = b[0].len();
    let mut c: Vec<Vec<i32>> = Vec::new();
    for i in 0..l {
        c.push(Vec::new());
        for _j in 0..n {
            c[i].push(0);
        }
    }

    for i in 0..l {
        for j in 0..m {
            for k in 0..n {
                c[i][k] += a[i][j] * b[j][k];
            }
        }
    }

    return c;
}
