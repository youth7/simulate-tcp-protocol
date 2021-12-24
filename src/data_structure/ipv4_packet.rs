
#[derive(Debug)]
pub struct IPV4Packet {
    // payload: Box<[u8]>,
    pub header: IPV4Header,
}

impl IPV4Packet {
    pub fn new(header: IPV4Header) -> IPV4Packet {
        IPV4Packet { header }
    }
}

#[derive(Debug)]
pub struct IPV4Header {
    src_port: u16,
    dest_port: u16,
    seq: u32,
    ack: u32,
    // header_size: u8,
    flag: Flag,
    // window_size: u16,
}

impl IPV4Header {
    pub fn new(src_port: u16, dest_port: u16, seq: u32, ack: u32, flag: Flag) -> IPV4Header {
        IPV4Header {
            src_port,
            dest_port,
            seq,
            ack,
            flag,
        }
    }

    pub fn get_buffer() -> Box<[u8]> {
        //将packet转为符合tcp规定的buffer
        todo!();
    }
}

#[derive(Debug)]
pub enum Flag {
    NS,
    CWR,
    ECE,
    URG,
    ACK,
    PSH,
    RST,
    SYN,
    FIN,
}
