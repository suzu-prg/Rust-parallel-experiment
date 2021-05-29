use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let n: usize = 100 + 1;
    let n_sqrt = (n as f64).sqrt() as usize;
    let is_prime = Arc::new(Mutex::new(vec![true; n]));
    is_prime.lock().unwrap()[0] = false;
    is_prime.lock().unwrap()[1] = false;
    for i in 0..n_sqrt {
        let is_prime = is_prime.clone();
        thread::spawn(move || {
            let mut is_prime = is_prime.lock().unwrap();
            if is_prime[i] {
                for j in 2..n_sqrt {
                    is_prime[i*j] = false;
                }
            }
        });
    }

    thread::sleep(Duration::from_millis(50));
    for i in 0..20 {
        let p = is_prime.lock().unwrap()[i];
        if p {
            println!("{:?}", i);
        }
    }
}
