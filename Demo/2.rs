// スライドの例：Borrowing
// ダングリングポインタの防止

fn main() {
	let r;
	{
		let x = 1;
		r = &x;
		assert_eq!(*r, 1);
	}
	
}

/*
	assert_eq!() -> after 7th line
*/