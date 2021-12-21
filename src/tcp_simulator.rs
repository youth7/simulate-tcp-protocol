use std::collections::LinkedList;
use std::sync::mpsc;
use std::thread;
use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};
#[derive(Debug)]
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
        Proxy::receive_data(Arc::clone(&buffer1), rx1);
        Proxy::receive_data(Arc::clone(&buffer2), rx2);
        Proxy::send_data(Arc::clone(&buffer1), tx1);
        Proxy::send_data(Arc::clone(&buffer2), tx2);
        Proxy {
            buffer2,
            buffer1,
        }
    }

    fn receive_data(buffer: ShareBuffer, rx: Receiver<Packet>) {
        thread::spawn(move || loop {
            let data = rx.recv().unwrap();
            println!("从channel中读取到一个包{:?}", data);
            let mut internal_buffer = buffer.lock().unwrap();
            // println!("2{:?}", "2");
            internal_buffer.push_back(data);
            // println!("3{:?}", "3");
        });
    }

    fn send_data(buffer: ShareBuffer, tx: Sender<Packet>) {
        thread::spawn(move || loop {
            let mut internal_buffer = buffer.lock().unwrap();
            if let Some(data) = internal_buffer.pop_back() {
                println!("从buffer中读取到一个包{:?}", data);
                tx.send(data).unwrap();
            }
        });
    }
}
/////////////////////////////////////////////////////////////////////
pub struct TCPSimulator {
    end_point1: EndPoint,
    end_point2: EndPoint,
    proxy: Proxy,
}
// 模拟器里面负责触发各种场景，但是不负责具体的实现
impl TCPSimulator {
    pub fn new() -> TCPSimulator {
        let (tx_in_end_porint1, rx1_in_proxy) = mpsc::channel::<Packet>(); // end point1 ⟾ proxy ⟾ end point2
        let (tx1_in_proxy, rx_in_end_point2) = mpsc::channel::<Packet>();
        let (tx_in_end_porint2, rx2_in_proxy) = mpsc::channel::<Packet>(); // end point2 ⟽ proxy ⟽ end point2
        let (tx2_in_proxy, rx_in_end_point1) = mpsc::channel::<Packet>();
        let end_point1 = EndPoint::new(123, tx_in_end_porint1, rx_in_end_point1);
        let end_point2 = EndPoint::new(123, tx_in_end_porint2, rx_in_end_point2);
        let proxy = Proxy::new(tx1_in_proxy, rx1_in_proxy, tx2_in_proxy, rx2_in_proxy);
        TCPSimulator {
            end_point1,
            end_point2,
            proxy,
        }
    }

    // 模拟三次握手的连接
    pub fn three_way_hand_shake(&self) {
        &self.end_point1.syn();
    }
}
