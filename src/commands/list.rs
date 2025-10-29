use crate::memory::Memory;

pub fn list(pid: i32) {
    let memory = match Memory::new(pid) {
        Ok(memory) => memory,
        Err(e) => {
            eprintln!("Unable to open process memory: {e}");
            return;
        }
    };

    for region in memory.memory_regions() {
        println!("{:X}-{:X} {}", region.start, region.end, region.pathname);
    }
}
