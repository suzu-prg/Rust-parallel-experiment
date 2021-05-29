use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
	let (tx, rx) = mpsc::channel();
	thread::spawn(move || {
		while true {
			thread::sleep(Duration::from_secs(1));
			let received = rx.recv();
			match received {
				Ok(k) => println!("{:?}", k),
				Err(err) => {
					println!("{:?}", err)
				}
			}
		}
	});
	tx.send(2).unwrap();
	thread::sleep(Duration::from_secs(1));
	drop(tx);
	thread::sleep(Duration::from_secs(1));
}