use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        for i in 0..4 {
            tx_clone.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        for i in 4..8 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // blocking
    println!("# {}", rx.recv().unwrap());

    // the rest
    for received in rx {
        println!("## {}", received);
    }
}
