// RAYON_NUM_THREADS=2 cargo run --release --example benchmark2 -- 8

use num_cpus;

use n_queens::par_back_tracking_3::n_queens as par_n_queens;
use n_queens::seq_back_tracking::n_queens as seq_n_queens;


use std::{env,f64};
use std::str::FromStr;
use std::time::Instant;

fn main() {
  if let Some(n) = env::args().nth(1) {
    let n = u32::from_str(&n).expect("error parsing argument");
    run_n_queens(n);
  } else {
    eprintln!(
      "Usage {} <number of elements in bits>",
      env::args().nth(0).unwrap()
    );
    std::process::exit(1);
  }
}

fn run_n_queens(n: u32) {
  println!("{} queens", n);
  println!(
    "cpu info: {} physical cores, {} logical cores", 
    num_cpus::get_physical(),
    num_cpus::get()
  );

  let seq_duration = timed_n_queens(&seq_n_queens, n, "seq_n_queens");
  let par_duration = timed_n_queens(&par_n_queens, n, "par_n_queens");

  println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_n_queens<F>(n_queens: &F, n: u32, name: &str) -> f64 
where
  F: Fn(u32) -> u64,
{
  let start = Instant::now();
  let ans = n_queens(n);
  let dur = start.elapsed();

  let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
  println!(
    "{}: in {} seconds", 
    name,  
    nano_secs / 1e9
  );
  let n_queen_ans = vec![0,
  1,
  0,
  0,
  2,
  10,
  4,
  40,
  92,
  352,
  724,
  2_680,
  14_200,
  73_712,
  365_596,
  2_279_184,
  14_772_512,
  95_815_104,
  666_090_624,
  4_968_057_848,
  39_029_188_884,
  2_691_008_701_644,
  24_233_937_684_440,
  227_514_171_973_736,
  2_207_893_435_808_352,
  22_317_699_616_364_044,
  234_907_967_154_122_528,
  ];
  assert_eq!(n_queen_ans[n as usize], ans);
  nano_secs
}