use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::data_structure::ipv4_packet::{Flag, IPV4Header, IPV4Packet};

pub struct EndPoint {
    port: u32,
    tx: Sender<IPV4Packet>,
    // rx: Receiver<IPV4Packet>,
    status: Status,
}

impl EndPoint {
    pub fn new(port: u32, tx: Sender<IPV4Packet>, rx: Receiver<IPV4Packet>) -> EndPoint {
        // EndPoint::on_receive_data(tx.clone(), rx);
        let end_point = EndPoint {
            port,
            tx:tx.clone(),
            // rx,
            status: Status::CLOSED,
        };
        end_point.start(rx, tx);
        end_point
    }
    pub fn syn(&self) {
        let header = IPV4Header::new(6666, 7777, self.isn(), 0, Flag::SYN);
        let syn_packet = IPV4Packet { header };

        self.tx.send(syn_packet).unwrap();
    }

    pub fn start(&self, rx:Receiver<IPV4Packet>, tx:Sender<IPV4Packet>) {
        thread::spawn(move || loop {
            let data = rx.recv().unwrap();
            println!("endPoint：从channel中读取到一个包{:?}", data);
            tx.send(data).unwrap();
            // self.handle();
        });
    }

    fn handle(&self) {}

    fn isn(&self) -> u32 {
        return 0;
    }
}

enum Status {
    CLOSED,
    LISTEN,
    SYN_SENT,
    SYN_RECEIVED,
    ESTABLISHED,
    FIN_WAIT_1,
    FIN_WAIT_2,
    CLOSE_WAIT,
    CLOSING,
    LAST_ACK,
    TIME_WAIT,
}
