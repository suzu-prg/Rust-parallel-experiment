pub fn eratosthenes(n: usize) {
    // let n: usize = 100 + 1;
    let n_sqrt = (n as f64).sqrt() as usize;
    let mut is_prime = vec![true; n];
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