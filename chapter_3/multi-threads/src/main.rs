use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn _thread() {
    let handle = thread::spawn(|| {
        println!("Hello, world!");
    });

    dbg!(handle.join());
}

fn _move() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, world!: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn _shared_memory() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}

fn _message_passing() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    let _ = tx.send("Hello, world!");
    let _ = handle.join();
}
fn _message_passing2() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut send_channels = Vec::new();
    let mut recv_channels = Vec::new();

    for _ in 0..10 {
        let (snd_tx, snd_rx) = mpsc::channel();
        let (rcv_tx, rcv_rx) = mpsc::channel();
        send_channels.push(snd_tx);
        recv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    for x in 0..10 {
        let _ = send_channels[x].send(data[x]);
    }

    for x in 0..10 {
        data[x] = recv_channels[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}

fn main() {
    // _move();
    // _shared_memory();
    // _message_passing()
    _message_passing2()
}
