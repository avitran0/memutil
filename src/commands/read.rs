use crate::{data_type::DataType, memory::Memory, address::AddressLocator};

pub fn read_once(pid: i32, addresss: AddressLocator, data_type: DataType) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(err) => {
            eprintln!("Unable to open process memory: {err}");
            return;
        }
    };

    let address = match addresss.resolve(&memory) {
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
}
