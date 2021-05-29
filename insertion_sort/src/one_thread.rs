// use std::thread;

pub fn insertion_sort(mut num: Vec<u32>) -> Vec<u32>{
	let n: usize = num.len() as usize;
	for i in 1..n {
		let mut j = i;
		while (j > 0) && (num[j-1] > num[j]) {
			let tmp = num[j-1];
			num[j-1] = num[j];
			num[j] = tmp;
            j -= 1;
        }
	}
	return num;
}

// fn main() {
// 	use insertion_sort::utils::{is_sorted_ascending, new_u32_vec};
//     let n = 10000usize;
//     let v = new_u32_vec(n);
//     let result = insertion_sort(v);
//     is_sorted_ascending(&result);
//     println!("{:?}", result);
// }