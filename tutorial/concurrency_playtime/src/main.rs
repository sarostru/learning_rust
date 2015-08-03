//! Examples of Concurrency
//!
//! Derived from the [concurrency section] (https://doc.rust-lang.org/book/concurrency.html)
//!
//! Also interesting this lightweight io project [metal io] (https://github.com/carllerche/mio)

use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from a thread!"
    });

    println!("{}", handle.join().unwrap());
	//fails_to_compile();
	
	//still_fails_to_compile();

	println!("#Using Sleep to Synchronize");
	compiles();
	
	println!("#Using Channels to Synchronize");
	using_channels();
	
	println!("#Sending Data on a Channel");
	send_data_on_channel();
}

// fn fails_to_compile() {
	// // This common datarace programming mistake is caught at compile time
	// let mut data = vec![1u32, 2, 3];

    // for i in 0..3 {
        // thread::spawn(move || {
            // data[i] += 1;
        // });
    // }

    // thread::sleep_ms(50);
// }

// use std::sync::Mutex;

// fn still_fails_to_compile() {
    // let mut data = Mutex::new(vec![1u32, 2, 3]);

    // for i in 0..3 {
        // let data = data.lock().unwrap();
        // thread::spawn(move || {
            // data[i] += 1;
        // });
    // }

    // thread::sleep_ms(50);
// }

use std::sync::{Arc, Mutex};

fn compiles() {
    let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }

    thread::sleep_ms(50);
}

use std::sync::mpsc;

fn using_channels() {
    let data = Arc::new(Mutex::new(0u32));

    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
			// Send an tmpty tuple ()
            tx.send(()).ok();
        });
    }

	// Wait for 10 empty tuples
    for _ in 0..10 {
        rx.recv().ok();
    }
}

fn send_data_on_channel() {
    let (tx, rx) = mpsc::channel();

    let tx = tx.clone();

    thread::spawn(move || {
        let answer = 42u32;

        tx.send(answer).ok().expect("Could not send answer");
    });

   rx.recv().ok().expect("Could not receive answer");
}
