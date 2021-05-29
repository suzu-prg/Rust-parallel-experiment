// スレッドセーフな例
// コンパイラはスレッドでの処理にどのくらいの時間がかかるかわからない

use std::thread;

fn main() {
    let a = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", a);
    });
    // drop(a);
    handle.join().unwrap();
}

/*
	let handle = thread::spawn(|| {
	                           -> move
*/