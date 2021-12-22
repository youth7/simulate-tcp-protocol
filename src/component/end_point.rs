use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::data_structure::ipv4_packet::IPV4Packet;

pub struct EndPoint {
    port: u32,
    tx: Sender<IPV4Packet>,
}

impl EndPoint {
    pub fn new(port: u32, tx: Sender<IPV4Packet>, rx: Receiver<IPV4Packet>) -> EndPoint {
        EndPoint::on_receive_data(tx.clone(), rx);
        EndPoint { port, tx }
    }
    pub fn syn(&self) {
        let syn_packet = IPV4Packet {};
        self.tx.send(syn_packet).unwrap();
    }
    fn on_receive_data(tx: Sender<IPV4Packet>, rx: Receiver<IPV4Packet>) {
        thread::spawn(move || loop {
            let data = rx.recv().unwrap();
            println!("endPoint：从channel中读取到一个包{:?}", data);
            tx.send(data).unwrap();
        });
    }
}
