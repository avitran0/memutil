use crate::{memory::Memory, signature::AddressLocator};

pub fn find(pid: i32, address: AddressLocator) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(e) => {
            eprintln!("Unable to open process memory: {e}");
            return;
        }
    };

    let address = match address.resolve(&memory) {
        Ok(address) => address,
        Err(e) => {
            eprintln!("Unable to resolve address: {e}");
            return;
        }
    };

    let memory_region = match memory.find_containing_region(address) {
        Some(region) => region,
        None => {
            eprintln!("Unable to find containing memory region for address 0x{address:X}");
            return;
        }
    };

    println!(
        "Found signature at 0x{address:X} in {}",
        memory_region.pathname
    );
}

pub fn find_function(pid: i32, function_name: String) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(e) => {
            eprintln!("Unable to open process memory: {e}");
            return;
        }
    };

    let functions = match memory.find_function(&function_name) {
        Ok(functions) => functions,
        Err(e) => {
            eprintln!("Unable to find function '{function_name}': {e}");
            return;
        }
    };

    if functions.is_empty() {
        eprintln!("Could not find function '{function_name}'");
        return;
    }

    println!("Found function '{function_name}' at these locations:");
    for function in functions {
        println!("0x{:X} at {}", function.address, function.pathname);
    }
}
