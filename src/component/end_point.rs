use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use crate::data_structure::ipv4_packet::{Flag, IPV4Header, IPV4Packet};

#[derive(Clone)]
pub struct EndPoint {
    pub port: u16,
    name: String,
    tx: Sender<IPV4Packet>,
    status: Arc<Mutex<Status>>,
}

impl EndPoint {
    pub fn new(
        port: u16,
        name: &str,
        tx: Sender<IPV4Packet>,
        rx: Receiver<IPV4Packet>,
    ) -> EndPoint {
        let end_point = EndPoint {
            port,
            name: String::from(name),
            tx,
            status: Arc::new(Mutex::new(Status::CLOSED)),
        };
        let background_worker = end_point.clone();
        end_point.run_in_background(rx);
        background_worker
    }
    pub fn syn(&self, dest_port:u16) -> Result<(), ()> {
        let mut status = self.status.lock().unwrap();
        match *status {
            Status::CLOSED => {
                let header = IPV4Header::new(self.port, dest_port, self.isn(), 0, Flag::new().enable_syn());
                let syn_packet = IPV4Packet { header };
                self.tx.send(syn_packet).unwrap();
                *status = Status::SynSend;
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn listen(&self) -> Result<(), EndPonitError> {
        let mut status = self.status.lock().unwrap();
        match *status {
            Status::CLOSED => {
                *status = Status::LISTEN;
                Ok(())
            }
            _ => {
                println!("end point不是处于closed状态");
                Err(EndPonitError::StatusError)
            }
        }
    }

    //开启线程来接收包
    pub fn run_in_background(self, rx: Receiver<IPV4Packet>) {
        thread::spawn(move || loop {
            let data = rx.recv().unwrap();
            //这里晚点用sleep来实现积累确认，即使是积累确认也必须逐个检查packet的类型的
            let status = self.status.lock().unwrap();
            self.display_packet(&data);
            //根据自己的状态和对方发送的包的类型，设置自身的多种状态，以及需要回应的包
            match *status {
                Status::LISTEN => {
                    self.handle_while_listen(data, status).unwrap();
                }
                Status::SynSend => {
                    self.handle_while_syn_sent(data, status).unwrap();                    
                }
                _ => {
                    println!(
                        "end point<{}>收到异常状态的packet，当前状态为{:?}",
                        self.name, *status
                    );
                }
            }
        });
    }

    fn handle_while_listen(&self, data: IPV4Packet, mut status: MutexGuard<Status>) -> Result<(), SendError<IPV4Packet>> {
        //当处于listen状态的时候，如果发来的不是syn包，则返回一个RST
        let ack_packet = IPV4Packet::new(IPV4Header::new(
            self.port,
            data.header.src_port,
            self.isn(),          //需要随机生成一个isn
            data.header.seq + 1, // 根据对方
            Flag::new().enable_ack(),
        ));
        *status = Status::SynReceived;
        self.tx.send(ack_packet)
    }

    fn handle_while_syn_sent(&self, data: IPV4Packet, mut status: MutexGuard<Status>) -> Result<(), SendError<IPV4Packet>> {
        
        let ack_packet = IPV4Packet::new(IPV4Header::new(
            self.port,
            data.header.src_port,
            data.header.ack, //第三次握手不消耗seq体现在第三次握手后的下一个包的seq不变
            data.header.seq + 1,
            Flag::new().enable_ack().enable_syn(),
        ));
        *status = Status::ESTABLISHED;
        self.tx.send(ack_packet)
    }

    fn isn(&self) -> u32 {
        return 0;
    }

    fn display_packet(&self, packet: &IPV4Packet) {
        println!(
            "[{}] {} ⟹  {}, {}",
            self.name, packet.header.src_port, packet.header.dest_port, packet
        );
        // println!("{}", packet);
    }
}
#[derive(Copy, Clone, Debug)]
pub enum Status {
    CLOSED,
    LISTEN,
    SynSend,
    SynReceived,
    ESTABLISHED,
    // FIN_WAIT_1,
    // FIN_WAIT_2,
    // CLOSE_WAIT,
    // CLOSING,
    // LAST_ACK,
    // TIME_WAIT,
}

#[derive(Debug)]
pub enum EndPonitError {
    // UN_EXPECT_PACKET(Status, IPV4Packet),
    StatusError,
    // UNKNOWN_ERROR,
}
