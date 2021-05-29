use std::cmp;

// この関数を並列化する
pub fn nqueen(n: i32) -> i32 {
    let mut board: Vec<Vec<i32>> = Vec::new();
    for i in 0..(n as usize) {
        board.push(Vec::new());
        for _j in 0..n {
            board[i].push(0);
        }
    }
    return rec(&mut board, 0);
}

// 上段から置けるところに置いていく
fn rec(b: &mut Vec<Vec<i32>>, d: i32) -> i32 {
    let n = b.len() as i32;
    if d == n {
        return 1;
    }

    let i = d as usize;
    let mut res = 0;
    for j in 0..(n as usize) {
        if can_set(b, i, j) {
            b[i][j] = 1;
            res += rec(b, d + 1);
            b[i][j] = 0;
        }
    }

    return res;
}

// 上の段のコマが利いていないか
fn can_set(b: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let n = b.len();

    // 縦をチェック
    for i in 0..x {
        if b[i][y] == 1 {
            return false;
        }
    }

    // 左上をチェック
    for t in 0..cmp::min(x, y) {
        let i = x - t - 1;
        let j = y - t - 1;
        if b[i][j] == 1 {
            return false;
        }
    }

    // 右上をチェック
    for t in 0..cmp::min(x, n - y - 1) {
        let i = x - t - 1;
        let j = y + t + 1;
        if b[i][j] == 1 {
            return false;
        }
    }

    return true;
}
