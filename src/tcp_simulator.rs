use crate::component::end_point::{EndPoint};
use crate::component::proxy::Proxy;
use crate::data_structure::ipv4_packet::IPV4Packet;
use std::sync::mpsc;

 

//////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////
pub struct TCPSimulator {
    end_point1: EndPoint,
    end_point2: EndPoint,
    proxy: Proxy,
}
// 模拟器里面负责触发各种场景，但是不负责具体的实现
impl TCPSimulator {
    pub fn new() -> TCPSimulator {
        let (tx_in_end_porint1, rx1_in_proxy) = mpsc::channel::<IPV4Packet>(); // end point1 ⟾ proxy ⟾ end point2
        let (tx1_in_proxy, rx_in_end_point2) = mpsc::channel::<IPV4Packet>();
        let (tx_in_end_porint2, rx2_in_proxy) = mpsc::channel::<IPV4Packet>(); // end point2 ⟽ proxy ⟽ end point2
        let (tx2_in_proxy, rx_in_end_point1) = mpsc::channel::<IPV4Packet>();
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
