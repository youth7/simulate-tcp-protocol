mod component;
mod data_structure;
mod tcp_simulator;
use std::{thread, time::Duration};

use tcp_simulator::TCPSimulator;

struct Test {
    name: u32,
}

impl Test {
    pub fn new() -> Test{
        let t = Test { name: 123 };
        t.test();
        t
    }
    pub fn test(&'static self){
        thread::spawn(move ||{
            self.do_sth();
        });
    }

    pub fn do_sth(&self){
        println!("{}", self.name);
    }
}



fn main() {
    let t = Test::new();
    t.do_sth();
    // let simulator = TCPSimulator::new();
    // simulator.three_way_hand_shake();
    // thread::sleep(Duration::from_secs(10000));
}
