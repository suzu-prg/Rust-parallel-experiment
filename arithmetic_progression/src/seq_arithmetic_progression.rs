
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

pub fn arithmetic_progression(is_prime: &[bool]) -> Vec<u64> {
	let n: usize = is_prime.len() as usize;
	let mut result = vec![1; 3]; // 初項，公差，項数
	
	'outer: 
	for a in (3..n).step_by(2) { // 初項を決める
		if !is_prime[a] {
			continue;
		}
		for d in (2..n-a).step_by(2) { // 公差を決める
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
			if cnt == 6 {
				break 'outer;
			}
		}
	}


	for a in (7..n).step_by(2) { // 初項を決める
		if !is_prime[a] {
			continue;
		}
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
	}

	result
}

fn main() {
    let n = 100000usize;
    let mut is_prime = vec![true; n];
	eratosthenes(&mut is_prime);
	let result = arithmetic_progression(&is_prime);
    println!("{:?}", result);
}