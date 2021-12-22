mod tcp_simulator;
mod data_structure;
mod component;
use std::{thread, time::Duration};

use tcp_simulator::TCPSimulator;

fn main() {
    let simulator = TCPSimulator::new();
    simulator.three_way_hand_shake();
    thread::sleep(Duration::from_secs(10000));
}
