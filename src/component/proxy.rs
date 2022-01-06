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
        Proxy::start_to_receive_data(Arc::clone(&buffer1), rx1);
        Proxy::start_to_receive_data(Arc::clone(&buffer2), rx2);
        Proxy::start_to_send_data(Arc::clone(&buffer1), tx1);
        Proxy::start_to_send_data(Arc::clone(&buffer2), tx2);
        Proxy { buffer2, buffer1 }
    }

    fn start_to_receive_data(buffer: ShareBuffer, rx: Receiver<IPV4Packet>) {
        thread::spawn(move || loop {
            let data = rx.recv().unwrap();
            // println!("【proxy】channel => buffer : {:?}", data);
            let mut internal_buffer = buffer.lock().unwrap();
            internal_buffer.push_back(data);
        });
    }

    fn start_to_send_data(buffer: ShareBuffer, tx: Sender<IPV4Packet>) {
        thread::spawn(move || loop {
            let mut internal_buffer = buffer.lock().unwrap();
            if let Some(data) = internal_buffer.pop_back() {
                // println!("【proxy】buffer => channel : {:?}", data);
                tx.send(data).unwrap();
            }
        });
    }
}
