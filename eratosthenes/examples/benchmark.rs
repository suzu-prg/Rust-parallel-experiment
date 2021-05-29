// cargo run --release --example benchmark -- 8

use num_cpus;

use eratosthenes::seq_eratosthenes::eratosthenes as seq_eratosthenes;
use eratosthenes::par_eratosthenes::eratosthenes as par_eratosthenes;
use eratosthenes::par_eratosthenes2::eratosthenes as par_eratosthenes2;

use std::{env,f64};
use std::str::FromStr;
use std::time::Instant;

fn main() {
  if let Some(n) = env::args().nth(1) {
    let n = usize::from_str(&n).expect("error parsing argument");
    run_eratosthenes(n);
  } else {
    eprintln!(
      "Usage {} <number of elements in bits>",
      env::args().nth(0).unwrap()
    );
    std::process::exit(1);
  }
}

fn run_eratosthenes(n: usize) {
  println!("{} primes", n);
  println!(
    "cpu info: {} physical cores, {} logical cores", 
    num_cpus::get_physical(),
    num_cpus::get()
  );

  let seq_duration = timed_eratosthenes(&seq_eratosthenes, n, "seq_eratosthenes ");
  let par_duration = timed_eratosthenes(&par_eratosthenes, n, "par_eratosthenes ");
  let par_duration = timed_eratosthenes(&par_eratosthenes2, n, "par_eratosthenes2");

  println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_eratosthenes<F>(eratosthenes: &F, n: usize, name: &str) -> f64 
where
  F: Fn(usize),
{
  let start = Instant::now();
  eratosthenes(n);
  let dur = start.elapsed();

  let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
  println!(
    "{}: in {} seconds", 
    name,  
    nano_secs / 1e9
  );
  nano_secs
}