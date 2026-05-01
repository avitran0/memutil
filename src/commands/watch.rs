use std::{thread::sleep, time::Duration};

use crate::{address::AddressLocator, data_type::DataType, memory::Memory};

pub fn watch(pid: i32, address: AddressLocator, data_type: DataType, interval: Duration) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(err) => {
            eprintln!("Unable to open process memory: {err}");
            return;
        }
    };

    loop {
        let address = match address.resolve(&memory) {
            Ok(address) => address,
            Err(err) => {
                eprintln!("Unable to resolve address: {err}");
                return;
            }
        };

        let value = match data_type.read(&memory, address) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Unable to read memory: {err}");
                return;
            }
        };
        println!("0x{address:X} = {value}");
        sleep(interval);
    }
}
