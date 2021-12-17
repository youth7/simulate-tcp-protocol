use std::{sync::{mpsc, Arc, Mutex}, thread, collections::LinkedList};
#[derive(Debug)]
struct Packet {}
mod tcp_simulator;

type ShareBuffer = Arc<Mutex<LinkedList<Packet>>>;
fn share_buffer() -> ShareBuffer {
    let buffer: Arc<Mutex<LinkedList<Packet>>> = Arc::new(Mutex::new(LinkedList::new()));
    let buffer1: Arc<Mutex<LinkedList<Packet>>> = Arc::clone(&buffer);
    thread::spawn( move||{
        // Arc::clone(&buffer).lock().unwrap()
        buffer1.lock().unwrap();
    });
    buffer

}



fn main() {
    let buffer = share_buffer();
    

}


    // let (a, b) = channel::<Test>();

    // thread::spawn(move ||  {
    //     let data = b.recv();
    // });

    // use std::sync::mpsc::channel;
    // use std::thread;
    // use std::time::Duration;

    // let (send, recv) = channel::<Test>();

    // thread::spawn(move || {
    //     println!("222 {:?}", recv.recv().unwrap()); // Received immediately
    //     println!("Waiting...");
    //     println!("222 {:?}", recv.recv().unwrap()); // Received after 2 seconds
    // });

    // send.send(Test {}).unwrap();
    // thread::sleep(Duration::from_secs(2)); // block for two seconds
    // send.send(Test {}).unwrap();