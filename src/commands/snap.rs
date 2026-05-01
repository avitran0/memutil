use crate::memory::Memory;

pub fn snap(pid: i32, lib: String) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(err) => {
            eprintln!("Unable to open process memory: {err}");
            return;
        }
    };

    let Some(region) = memory.memory_regions().iter().find(|region| {
        let Some((_, filename)) = region.pathname.rsplit_once('/') else {
            return false;
        };
        filename.contains(&lib)
    }) else {
        eprintln!("Failed to find library '{lib}'");
        return;
    };

    let bytes = match memory.read_bytes(region.start, region.end - region.start) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Unable to read memory: {err}");
            return;
        }
    };

    if let Err(err) = std::fs::write(&lib, bytes) {
        eprintln!("Unable to write to file '{lib}': {err}");
    }
}
