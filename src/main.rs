use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::sync::{
    Arc,
    Mutex,
};

const NUM_THREADES: usize = 128;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("Listening started, ready to accept");
    let listener = Arc::new(Mutex::new(listener));
    for i in 1..NUM_THREADES {
        let col_listener = listener.clone();
        thread::spawn(move || {
            loop {
                let thread_id = i;
                let (mut stream, address) = {
                    let listener = col_listener.lock().unwrap();
                    listener.accept().unwrap()
                };
                stream.write(b"Hello World\r\n").unwrap();
                let (_, _) = (thread_id, address);
            }
        });
    }
    let col_listener = listener.clone();
    loop {
        let (mut stream, address) = {
            let listener = col_listener.lock().unwrap();
            listener.accept().unwrap()
        };
        stream.write(b"Hello World\r\n").unwrap();
        let _ = address;
    }
}