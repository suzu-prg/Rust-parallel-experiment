use std::thread;
use std::sync::mpsc;

fn threads_run(tx_first: mpsc::Sender<u32>, rx_before: mpsc::Receiver<u32>, n: usize){
	if n==0 {
		return;
	}

	let (tx, rx) = mpsc::channel::<u32>();
	let mut val = 0;
	let tx_f = tx_first.clone();
	thread::spawn(move || {
		threads_run(tx_f, rx, n-1);
	});
	let mut used = false;
	loop {
		let x = rx_before.recv();
		if !used {
			used = true;
			val = x.unwrap();
			continue;
		}
		match x {
			Ok(k) => {
				if k >= val {
					tx.send(k).unwrap();
				} else {
					tx.send(val).unwrap();
					val = k;
				}
			},
			Err(_) => break
		};
		
	}
	tx_first.send(val).unwrap();
	drop(tx);
	return;
}

pub fn insertion_sort(num: Vec<u32>) -> Vec<u32>{
	let n: usize = num.len() as usize;
	let (tx, rx) = mpsc::channel::<u32>();
	let (tx_f, rx_f) = mpsc::channel::<u32>();
	thread::spawn(move || {
		threads_run(tx_f, rx, n);
    });
	for val in num {
		tx.send(val).unwrap();
	}
	drop(tx);
	let mut num = vec![];
	for _ in 0..n {
        num.push(rx_f.recv().unwrap());
    }
    return num;
}

// fn main() {
// 	use insertion_sort::utils::{is_sorted_ascending, new_u32_vec};
//     let n = 10000usize;
//     let v = new_u32_vec(n);
//     // println!("{:?}", &v);
//     let result = insertion_sort(v);
//     if !is_sorted_ascending(&result) {
//     	for i in 0..n-1 {
//     		if result[i] < result[i+1] {
//     			continue;
//     		}
// 			println!("{:?}: {:?} {:?}", i, result[i], result[i+1]);
//     	}
//     }
//     // println!("{:?}", is_sorted_ascending(&result));
// }