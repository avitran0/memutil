use std::{thread::sleep, time::Duration};

use crate::{data_type::DataType, memory::Memory, signature::Signature};

pub fn watch(pid: i32, signature: Signature, data_type: DataType, interval: Duration) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(e) => {
            eprintln!("Unable to open process memory: {e}");
            return;
        }
    };

    loop {
        let address = match signature.resolve(&memory) {
            Ok(address) => address,
            Err(e) => {
                eprintln!("Unable to resolve address: {e}");
                return;
            }
        };

        let value = match data_type.read(&memory, address) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Unable to read memory: {e}");
                return;
            }
        };
        println!("0x{address:X} = {value}");
        sleep(interval);
    }
}
