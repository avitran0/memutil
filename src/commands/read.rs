use crate::{data_type::DataType, memory::Memory, address::AddressLocator};

pub fn read_once(pid: i32, addresss: AddressLocator, data_type: DataType) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(e) => {
            eprintln!("Unable to open process memory: {e}");
            return;
        }
    };

    let address = match addresss.resolve(&memory) {
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
}
