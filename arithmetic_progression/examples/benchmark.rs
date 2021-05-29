// cargo run --release --example benchmark -- 8

use num_cpus;

use arithmetic_progression::seq_arithmetic_progression::arithmetic_progression as seq_arithmetic_progression;
use arithmetic_progression::par_arithmetic_progression::arithmetic_progression as par_arithmetic_progression;
use arithmetic_progression::par_arithmetic_progression::eratosthenes;

use std::{env,f64};
use std::str::FromStr;
use std::time::Instant;

fn main() {
  if let Some(n) = env::args().nth(1) {
    let n = usize::from_str(&n).expect("error parsing argument");
    run_arithmetic_progression(n);
  } else {
    eprintln!(
      "Usage {} <number of elements in bits>",
      env::args().nth(0).unwrap()
    );
    std::process::exit(1);
  }
}

fn run_arithmetic_progression(n: usize) {
  println!("{} primes", n);
  println!(
    "cpu info: {} physical cores, {} logical cores", 
    num_cpus::get_physical(),
    num_cpus::get()
  );
  let mut is_prime = vec![true; n];
  eratosthenes(&mut is_prime);
  let seq_duration = timed_arithmetic_progression(&seq_arithmetic_progression, &is_prime, "seq_arithmetic_progression");
  let par_duration = timed_arithmetic_progression(&par_arithmetic_progression, &is_prime, "par_arithmetic_progression");

  println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_arithmetic_progression<F>(arithmetic_progression: &F, is_prime: &[bool], name: &str) -> f64 
where
  F: Fn(&[bool]) -> Vec<u64>,
{
  let start = Instant::now();
  let _result = arithmetic_progression(&is_prime);
  let dur = start.elapsed();

  let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
  println!(
    "{}: in {} seconds", 
    name,  
    nano_secs / 1e9
  );
  nano_secs
}