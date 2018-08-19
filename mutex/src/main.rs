use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let data = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
        }));
    }

    for h in handles {
        let _ = h.join();
    }

    println!("{:?}", data);
}
