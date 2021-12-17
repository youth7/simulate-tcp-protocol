use std::collections::LinkedList;
use std::sync::mpsc::{self, channel};
use std::thread::{self, JoinHandle};
use std::{
    rc::Rc,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

struct Packet {}
struct EndPoint {
    port: u32,
    tx: Sender<Packet>,
    rx: Receiver<Packet>,
}

impl EndPoint {
    fn new(port: u32, tx: Sender<Packet>, rx: Receiver<Packet>) -> EndPoint {
        EndPoint { port, tx, rx }
    }
    fn syn(&self) {
        let syn_packet = Packet {};
        self.tx.send(syn_packet);
    }
}
//////////////////////////////////////////////////////////////////////
struct Channel {
    tx: Sender<Packet>,
    rx: Receiver<Packet>,
}
impl Channel {
    fn new(tx: Sender<Packet>, rx: Receiver<Packet>) -> Channel {
        Channel { tx, rx }
    }
}

// struct ShareBuffer{
//     buffer : Arc<Mutex<LinkedList<Packet>>>,
// }

// impl ShareBuffer{
//     fn new() -> ShareBuffer{
//         ShareBuffer{
//             buffer : Arc::new(Mutex::new(LinkedList::new())),
//         }
//     }
// }

type ShareBuffer = Arc<Mutex<LinkedList<Packet>>>;
fn share_buffer() -> ShareBuffer {
    Arc::new(Mutex::new(LinkedList::new()))
}

struct Proxy {
    buffer1: ShareBuffer,
    buffer2: ShareBuffer,
}

impl Proxy {
    fn new(
        tx1: Sender<Packet>,
        rx1: Receiver<Packet>,
        tx2: Sender<Packet>,
        rx2: Receiver<Packet>,
    ) -> Proxy {
        let buffer1 = share_buffer();
        let buffer2 = share_buffer();
        let proxy = Proxy {
            buffer2 : Arc::clone(&buffer2),
            buffer1: Arc::clone(&buffer1),
        };

        thread::spawn(move || loop {
            let data = rx1.recv();
        });
        thread::spawn(|| loop {
            let data = Arc::clone(&buffer1).lock().unwrap().pop_back();
        });

        proxy
    }

    fn start(&self) {}
}
/////////////////////////////////////////////////////////////////////
pub struct TCPSimulator {
    end_point1: EndPoint,
    end_point2: EndPoint,
    proxy: Proxy,
}
// 模拟器里面负责触发各种场景，但是不负责具体的实现
impl TCPSimulator {
    fn new() -> TCPSimulator {
        let (tx_end_porint1, rx1_proxy) = mpsc::channel::<Packet>(); // end point1 ⟾ proxy ⟾ end point2
        let (tx1_proxy, rx_end_point2) = mpsc::channel::<Packet>();
        let (tx_end_porint2, rx2_proxy) = mpsc::channel::<Packet>(); // end point2 ⟽ proxy ⟽ end point2
        let (tx2_proxy, rx_end_point1) = mpsc::channel::<Packet>();
        let end_point1 = EndPoint::new(123, tx_end_porint1, rx_end_point1);
        let end_point2 = EndPoint::new(123, tx_end_porint2, rx_end_point2);
        let proxy = Proxy::new(tx1_proxy, rx1_proxy, tx2_proxy, rx2_proxy);
        TCPSimulator {
            end_point1,
            end_point2,
            proxy,
        }
    }

    // 模拟三次握手的连接
    fn three_way_hand_shake(&self) {
        let sync = &self.end_point1.syn();
    }
}
