use std::io::Write;
use std::net::{
    TcpListener,
};
use std::thread;
use std::sync::{
    Arc,
    Mutex,
};

const NUM_THREADS: usize = 128;

fn main() {
    let mut threads = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("Listening started, ready to accept");
    let listener = Arc::new(Mutex::new(listener));
    for i in 0..NUM_THREADS {
        let col_listener = listener.clone();
        threads.push(thread::spawn(move || {
            loop {
                let thread_id = i;
                let (mut stream, address) = {
                    let listener = col_listener.lock().unwrap();
                    listener.accept().unwrap()
                };
                stream.write(b"Hello World\r\n").unwrap();
                let (_, _) = (thread_id, address);
            }
        }));
    }

//    for _ in 0..100 {
//        let mut s = TcpStream::connect("127.0.0.1:9123").unwrap();
//        let mut buff = [0u8; 128];
//        use std::io::Read;
//        s.read(&mut buff).expect("Error in reading.");
//        println!("{} {} {} {} {} {}", buff[0], buff[1], buff[2], buff[3], buff[4], buff[5]);
//    }

    for th in threads {
        th.join().expect("Error in joining thread.");
    }
}