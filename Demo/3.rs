// 高度な所有権解析：制御フロー
// if式の分岐はどちらもチェックされる

fn main() {
	let a = vec![1,2,3];
	let b;
	let _c;
	if a[0] != 1 {
		b = a;
	} else {
		_c = a;
	}
	println!("{:?}", b);
}
