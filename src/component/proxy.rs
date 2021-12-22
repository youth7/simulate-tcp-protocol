use crate::data_structure::ipv4_packet::IPV4Packet;
use std::collections::LinkedList;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};
use std::thread;

type ShareBuffer = Arc<Mutex<LinkedList<IPV4Packet>>>;
fn share_buffer() -> ShareBuffer {
    Arc::new(Mutex::new(LinkedList::new()))
}

pub struct Proxy {
    buffer1: ShareBuffer,
    buffer2: ShareBuffer,
}

impl Proxy {
    pub fn new(
        tx1: Sender<IPV4Packet>,
        rx1: Receiver<IPV4Packet>,
        tx2: Sender<IPV4Packet>,
        rx2: Receiver<IPV4Packet>,
    ) -> Proxy {
        let buffer1 = share_buffer();
        let buffer2 = share_buffer();
        Proxy::receive_data(Arc::clone(&buffer1), rx1);
        Proxy::receive_data(Arc::clone(&buffer2), rx2);
        Proxy::send_data(Arc::clone(&buffer1), tx1);
        Proxy::send_data(Arc::clone(&buffer2), tx2);
        Proxy { buffer2, buffer1 }
    }

    fn receive_data(buffer: ShareBuffer, rx: Receiver<IPV4Packet>) {
        thread::spawn(move || loop {
            let data = rx.recv().unwrap();
            println!("proxy：从channel中读取到一个包{:?}", data);
            let mut internal_buffer = buffer.lock().unwrap();
            // println!("2{:?}", "2");
            internal_buffer.push_back(data);
            // println!("3{:?}", "3");
        });
    }

    fn send_data(buffer: ShareBuffer, tx: Sender<IPV4Packet>) {
        thread::spawn(move || loop {
            let mut internal_buffer = buffer.lock().unwrap();
            if let Some(data) = internal_buffer.pop_back() {
                println!("proxy：从buffer中读取到一个包{:?}", data);
                tx.send(data).unwrap();
            }
        });
    }
}
