use std::thread;
use std::sync::mpsc;

fn threads_run(tx_first: mpsc::Sender<Vec<u32>>, rx_before: mpsc::Receiver<u32>, n: usize, p1: usize, p2: usize){
	if n==0 {
		return;
	}
	let (tx, rx) = mpsc::channel::<u32>();
	// let mut val = 0;
	let mut v = vec![];
	let tx_f = tx_first.clone();
	thread::spawn(move || {
		threads_run(tx_f, rx, n-p1, p1, p2);
	});
	loop {
		let x = rx_before.recv();
		match x {
			Ok(k) => {
				// とりあえず挿入 -> 余ったら送る
				v.push(k);
				let mut j = v.len()-1;
				while (j > 0) && (v[j-1] > v[j]) {
					let tmp = v[j-1];
					v[j-1] = v[j];
					v[j] = tmp;
					j -= 1;
				}
				// キャパオーバーなら最後の値を1つ次スレッドへ送る
				if v.len() > p1 {
					tx.send(v.pop().unwrap()).unwrap();
					
				}
			},
			Err(_) => break,
		};
	}

	tx_first.send(v).unwrap();
	drop(tx);
	return;
}

pub fn insertion_sort(num: Vec<u32>, p1: usize, p2: usize) -> Vec<u32> {
	let n: usize = num.len() as usize;
	let (tx, rx) = mpsc::channel::<u32>();
	let (tx_f, rx_f) = mpsc::channel::<Vec<u32>>();
	thread::spawn(move || {
		threads_run(tx_f, rx, n, p1, p2);
    });
	for val in num {
		tx.send(val).unwrap();
	}
	drop(tx);
	let mut num = vec![];
	for _ in 0..(n/p1) {
        num.append(&mut rx_f.recv().unwrap());
    }
    return num;
}

fn main() {
	use insertion_sort::utils::{is_sorted_ascending, new_u32_vec};
    let n = 1000usize;
    let v = new_u32_vec(n);
    // println!("{:?}", &v);
    let result = insertion_sort(v, 100, 50);
    if !is_sorted_ascending(&result) {
    	for i in 0..n-1 {
    		if result[i] < result[i+1] {
    			continue;
    		}
			println!("{:?}: {:?} {:?}", i, result[i], result[i+1]);
    	}
    }
    println!("{:?}", is_sorted_ascending(&result));
}