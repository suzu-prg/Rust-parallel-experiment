use std::thread;
fn main() {
    let mut v = [1, 0, 3, 0, 5, 6];
    // scoped to restrict the lifetime of the borrows
    
    {
        let (left, right) = v.split_at_mut(2);
        let mut thread_handles = vec![];
        assert!(left == [1, 0]);
        assert!(right == [3, 0, 5, 6]);
        thread_handles.push(
            thread::spawn(move || {
                left[1] = 2;
            })
        );
        thread_handles.push(
            thread::spawn(move || {
                right[1] = 4;
            })
        );
        for handle in thread_handles {
            handle.join().unwrap()
        }
    }
    
    assert!(v == [1, 2, 3, 4, 5, 6]);
}

// fn main() {
// 	eratosthenes(100);
// }