#![feature(test)]

pub mod nqueen;

use std::time::Instant;

macro_rules! measure {
    // 値のセット、計測部分、assert、ループ回数
    ($set:expr, $body:expr, $check:expr, $times:expr) => {{
        let t: i32 = $times;
        let mut sum: f64 = 0.0;
        let mut f: i32 = 0;
        for _i in 0..t {
            $set;
            let start = Instant::now();
            $body;
            sum += (start.elapsed().as_nanos() as f64) / 1_000_000_000.0;
            if !$check {
                f += 1;
            }
        }
        print!("{:.6} sec/iter ", sum / (t as f64));
        if f == 0 {
            // 正常
            println!("\x1b[{}m({}/{} failed)\x1b[m ", 32, f, t);
        } else {
            // 異常
            println!("\x1b[{}m({}/{} failed)\x1b[m ", 31, f, t);
        }
    }};
}

fn bench(n: i32, ans: i32) {
    let mut res: i32;
    print!("test n = {}  ... : ", n);
    measure!(
        {},
        {
            res = nqueen::nqueen(n);
        },
        { ans == res },
        { 1 }
    );
}

fn main() {
    println!();
    bench(10, 724);
    bench(11, 2680);
    bench(12, 14200);
    println!();
}
