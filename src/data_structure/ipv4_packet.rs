use std::fmt::{Display, Formatter, Result};

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

impl Display for IPV4Packet{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "seq:{} ack:{} flag:{}", self.header.seq, self.header.ack, self.header.flag)
    }
}

#[derive(Debug)]
pub struct IPV4Header {
    pub src_port: u16,
    pub dest_port: u16,
    pub seq: u32,
    pub ack: u32,
    // header_size: u8,
    pub flag: Flag,
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
pub struct Flag {
    ns: bool,
    cwr: bool,
    ece: bool,
    urg: bool,
    ack: bool,
    psh: bool,
    rst: bool,
    syn: bool,
    fin: bool,
}
impl Flag {
    pub fn new() -> Flag {
        Flag {
            ns: false,
            cwr: false,
            ece: false,
            urg: false,
            ack: false,
            psh: false,
            rst: false,
            syn: false,
            fin: false,
        }
    }

    pub fn enable_ack(mut self) -> Flag {
        self.ack = true;
        self
    }
    pub fn enable_syn(mut self) -> Flag {
        self.syn = true;
        self
    }

    pub fn get_enable_flags(&self) -> Vec<&str>{
        let mut vec : Vec<&str> = vec![];
        if self.syn {
            vec.push("syn");
        }
        if self.ack {
            vec.push("ack");
        }
        vec
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_enable_flags().join(","))
    }
}


