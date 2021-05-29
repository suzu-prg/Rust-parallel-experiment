// スライドの例：Move
// 所有権を静的解析

fn main() {
	let a = vec![1,2,3];
	let b = &a;
	println!("{}", a[0]);
	println!("{}", b[0]);
}

/*
	let b = a;
	        -> &a;
*/