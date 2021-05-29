use std::thread;
use std::sync::Arc;

pub fn eratosthenes(is_prime: &mut [bool]) {
    let n: usize = is_prime.len() as usize;
    let n_sqrt = (n as f64).sqrt() as usize;
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 0..n_sqrt {
        if is_prime[i] {
            for j in 2..(n/i) {
                is_prime[i*j] = false;
            }
        }
    }
}

pub fn arithmetic_progression(n: usize) -> Vec<u64> {
    let mut is_prime = vec![true; n];
    eratosthenes(&mut is_prime);
    // let n: usize = is_prime.len() as usize;
    let mut result = vec![1; 3]; // 初項，公差，項数
    let is_prime:std::sync::Arc<Vec<bool>> = Arc::from(is_prime);

    let mut thread_handles = vec![];
    
    for a in (7..n).step_by(2) { // 初項を決める
        let is_prime = is_prime.clone();
        thread_handles.push(
            thread::spawn(move || {
                let mut result = vec![1; 3]; // 初項，公差，項数
                for d in (30..n-a).step_by(30) { // 公差を決める
                    let mut m = a;
                    let mut cnt = 0_u64;
                    while is_prime[m] { // 数える
                        m += d;
                        cnt += 1;
                        if m >= n {
                            break;
                        }
                    }
                    if (cnt == result[2]) && (a as u64 != result[0]) {
                        result[0] = a as u64;
                        result[1] = d as u64;
                    } else if cnt > result[2] {
                        result[0] = a as u64;
                        result[1] = d as u64;
                        result[2] = cnt;
                    }
                }
                result
            })
        );
    }
    for handle in thread_handles {
        let res = handle.join().unwrap();
        if (res[2] == result[2]) && (res[0] != result[0]) {
            result[0] = res[0];
            result[1] = res[1];
        } else if res[2] > result[2] {
            result[0] = res[0];
            result[1] = res[1];
            result[2] = res[2];
        }
    }
    result
}

fn main() {
    let n = 1000usize;
    let result = arithmetic_progression(n);
    println!("{:?}", result);
}
