use crate::message::IoEvent;
use std::io;
use std::sync::mpsc::Sender;
pub struct IoHandler {
    tx: Sender<IoEvent>,
}

impl IoHandler {
    pub fn new(tx: Sender<IoEvent>) -> Self {
        IoHandler { tx }
    }

    pub fn run(&self) {
        loop {
            let mut command_line = String::new();
            if io::stdin().read_line(&mut command_line).is_ok() {
                if command_line.trim().is_empty() {
                    println!("empty!");
                    continue;
                }

                self.tx.send(IoEvent::new(&command_line)).unwrap();
            }
        }
    }
}
