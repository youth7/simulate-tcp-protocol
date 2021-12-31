use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use crate::data_structure::ipv4_packet::{Flag, IPV4Header, IPV4Packet};

pub struct EndPoint {
    port: u16,
    name: String,
    tx: Sender<IPV4Packet>,
    // rx: Receiver<IPV4Packet>,
    pub status: Arc<Mutex<Status>>,
}

impl EndPoint {
    pub fn new(
        port: u16,
        name: &str,
        tx: Sender<IPV4Packet>,
        rx: Receiver<IPV4Packet>,
    ) -> EndPoint {
        // EndPoint::on_receive_data(tx.clone(), rx);
        let end_point = EndPoint {
            port,
            name: String::from(name),
            tx: tx.clone(),
            // rx,
            status: Arc::new(Mutex::new(Status::CLOSED)),
        };
        end_point.start(rx, tx);
        end_point
    }
    pub fn syn(&self) -> Result<(), ()> {
        let mut status = self.status.lock().unwrap();

        match *status {
            Status::CLOSED => {
                *status = Status::SYN_SENT;
                let header = IPV4Header::new(self.port, 222, self.isn(), 0, Flag::SYN);
                let syn_packet = IPV4Packet { header };
                // println!("<{}> ⟹  {:?}", self.name, syn_packet);
                self.tx.send(syn_packet).unwrap();
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn listen(&self) -> Result<u32, u32> {
        let mut status = self.status.lock().unwrap();
        match *status {
            Status::CLOSED => {
                *status = Status::LISTEN;
                Ok(1)
            }
            _ => {
                println!("end point不是处于closed状态");
                Err(1)
            }
        }
    }

    //开启线程来接收包
    pub fn start(&self, rx: Receiver<IPV4Packet>, tx: Sender<IPV4Packet>) {
        let status = Arc::clone(&self.status);
        let name = self.name.clone();
        thread::spawn(move || loop {
            // self.syn(123);
            let data = rx.recv().unwrap();
            //这里晚点用sleep来实现积累确认，即使是积累确认也必须逐个检查packet的类型的
            // println!("<{}> ⟸  {:?}", name.as_str(), data);
            EndPoint::handle(name.as_str(), &status, tx.clone(), data).unwrap();
        });
    }

    fn display_packet(packet: &IPV4Packet) {
        // let IPV4Header{src_port,_,_,_} = packet.header;
        println!("{} ⟹  {}, {:?}", packet.header.src_port, packet.header.dest_port, packet);
    }

    fn handle(
        name: &str,
        status: &Arc<Mutex<Status>>,
        tx: Sender<IPV4Packet>,
        data: IPV4Packet,
    ) -> Result<(), EndPonitError> {
        let mut status = status.lock().unwrap();
        EndPoint::display_packet(&data);
        //根据自己的状态和对方发送的包的类型，设置自身的多种状态，以及需要回应的包
        match *status {
            Status::LISTEN => match EndPoint::handle_while_listen(name, &mut status, tx, data) {
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("<{}>：handle_while_listen出错了", name);
                    Err(EndPonitError::UN_EXPECT_PACKET(*status, e.0))
                }
            },

            Status::SYN_SENT => {
                match EndPoint::handle_while_syn_sent(name, &mut status, tx, data) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        println!("<{}>：handle_while_syn_sent出错了", name);
                        Err(EndPonitError::UN_EXPECT_PACKET(*status, e.0))
                    }
                }
            }

            _ => {
                println!(
                    "end point<{}>不是处于listen状态，当前状态为{:?}",
                    name, *status
                );
                Err(EndPonitError::UNKNOWN_ERROR)
            }
        }
    }

    fn handle_while_listen(
        name: &str,
        status: &mut MutexGuard<Status>,
        tx: Sender<IPV4Packet>,
        data: IPV4Packet,
    ) -> Result<(), SendError<IPV4Packet>> {
        **status = Status::SYN_RECEIVED;
        let ack_packet = IPV4Packet::new(IPV4Header::new(
            data.header.dest_port,
            data.header.src_port,
            0,//需要随机生成一个isn
            data.header.seq + 1,
            Flag::ACK,
        ));
        
        tx.send(ack_packet)
    }

    fn handle_while_syn_sent(
        name: &str,
        status: &mut MutexGuard<Status>,
        tx: Sender<IPV4Packet>,
        data: IPV4Packet,
    ) -> Result<(), SendError<IPV4Packet>> {
        **status = Status::ESTABLISHED;
        let ack_packet = IPV4Packet::new(IPV4Header::new(
            data.header.dest_port,
            data.header.src_port,
            data.header.ack - 1,//第三次握手不消耗seq
            data.header.seq + 1,//第三次握手不需要消耗seq
            Flag::ACK,
        ));
        tx.send(ack_packet)
    }

    fn isn(&self) -> u32 {
        return 0;
    }
}
#[derive(Copy, Clone, Debug)]
pub enum Status {
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

#[derive(Debug)]
pub enum EndPonitError {
    UN_EXPECT_PACKET(Status, IPV4Packet),
    UNKNOWN_ERROR,
}
