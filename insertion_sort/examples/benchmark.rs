// cargo run --release --example benchmark -- 100000

use num_cpus;

use insertion_sort::one_thread::insertion_sort as seq_sort;
use insertion_sort::par_values::insertion_sort as par_values;
use insertion_sort::par_values2::insertion_sort as par_values2;
use insertion_sort::utils::{is_sorted_ascending, new_u32_vec};

use std::{env,f64};
use std::str::FromStr;
use std::time::Instant;

fn main() {
  if let Some(n) = env::args().nth(1) {
    let n = usize::from_str(&n).expect("error parsing argument");
    run_sorts(n);
  } else {
    eprintln!(
      "Usage {} <number of elements>",
      env::args().nth(0).unwrap()
    );
    std::process::exit(1);
  }
}

fn run_sorts(len: usize) {
  println!("{} elements", len);
  println!(
    "cpu info: {} physical cores, {} logical cores", 
    num_cpus::get_physical(),
    num_cpus::get()
  );
  let v = new_u32_vec(len);
  let _seq_duration  = timed_sort(v.to_vec(), &seq_sort,    len, "seq_sort   ");
  // let _par_duration  = timed_sort(v.to_vec(), &par_values,  len, "par_values ");
  let _par_duration2 = timed_sort(v.to_vec(), &par_values2, len, "par_values2");

  // println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_sort<F>(v: Vec<u32>, sorter: &F, len: usize, name: &str) -> f64 
where
  F: Fn(Vec<u32>) -> Vec<u32>,
{
  let start = Instant::now();
  let result = sorter(v);
  let dur = start.elapsed();

  let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
  println!(
    "{}: sorted {} integers in {} seconds", 
    name, 
    len, 
    nano_secs / 1e9
  );

  assert!(is_sorted_ascending(&result));

  nano_secs
}