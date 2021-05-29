use std::thread;
use std::sync::mpsc;

fn threads_run(tx_first: mpsc::Sender<Vec<u32>>, rx_before: mpsc::Receiver<Vec<u32>>, n: usize, p1: usize, p2: usize){
	if n==0{
		return;
	}
	let mut vec = vec![];
	let (tx, rx) = mpsc::channel::<Vec<u32>>();
	let tx_f = tx_first.clone();
	thread::spawn(move || {
		threads_run(tx_f, rx, n-p1, p1, p2);
	});
	loop {
		let x = rx_before.try_recv();
		match x {
			Ok(mut vec_recv) => {
				vec.append(&mut vec_recv);
				let n: usize = vec.len() as usize;
				for i in 1..n {
					let mut j = i;
					while (j > 0) && (vec[j-1] > vec[j]) {
						let tmp = vec[j-1];
						vec[j-1] = vec[j];
						vec[j] = tmp;
			            j -= 1;
			        }
				}
			},
			Err(_) => break
		};
		if vec.len() > p1 {
			// 次のスレッドにおくる
		}
	}
	tx_first.send(vec).unwrap();
	drop(tx);
	return;
}

pub fn insertion_sort(num: Vec<u32>, p1: usize, p2: usize) -> Vec<u32> {
	let n: usize = num.len() as usize;
	let (tx, rx) = mpsc::channel::<Vec<u32>>();
	let (tx_f, rx_f) = mpsc::channel::<Vec<u32>>();
	thread::spawn(move || {
		threads_run(tx_f, rx, n, p1, p2);
    });
	for val in num {
		tx.send(vec![val,1]).unwrap();
	}
	drop(tx);
	let mut num = vec![];
	for _ in 0..n {
        num.append(&mut (rx_f.recv().unwrap()));
    }
    return num;
}

fn main() {
	use insertion_sort::utils::{is_sorted_ascending, new_u32_vec};
    let n = 10000usize;
    let v = new_u32_vec(n);
    // println!("{:?}", &v);
    let result = insertion_sort(v, 100, 10);
    if !is_sorted_ascending(&result) {
    	for i in 0..n-1 {
    		if result[i] < result[i+1] {
    			continue;
    		}
			println!("{:?}: {:?} {:?}", i, result[i], result[i+1]);
    	}
    }
    // println!("{:?}", is_sorted_ascending(&result));
}