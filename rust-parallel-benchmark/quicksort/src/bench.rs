#![feature(test)]

pub mod quicksort;
pub mod utils;

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

fn bench(in_file: &str, out_file: &str) {
    let testin = utils::read_vec(in_file.to_string());
    let testout = utils::read_vec(out_file.to_string());
    let mut tmpin: Vec<i32> = Vec::new();
    print!("test {} ... : ", in_file);
    measure!(
        {
            utils::copy(&testin, &mut tmpin);
        },
        {
            quicksort::quicksort(&mut tmpin);
        },
        { tmpin == testout },
        { 1 }
    );
}

fn main() {
    println!();
    bench("./tests/bench1.in", "./tests/bench1.out"); // n = 5
    bench("./tests/bench2.in", "./tests/bench2.out"); // n = 100
    bench("./tests/bench3.in", "./tests/bench3.out"); // n = 10^4
    bench("./tests/bench4.in", "./tests/bench4.out"); // n = 10^5
    bench("./tests/bench5.in", "./tests/bench5.out"); // n = 10^6
    println!();
}
