use std::collections::BTreeMap;

use elf::{ElfBytes, endian::AnyEndian, symbol::Symbol};
use libc::{iovec, process_vm_readv};
use thiserror::Error;

use crate::address::IdaSignature;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Signature '{0}' not found")]
    SignatureNotFound(IdaSignature),
    #[error("Invalid ELF Signature '{0:X}'")]
    InvalidElf(u32),
    #[error("Invalid ELF File ({0})")]
    InvalidElfData(#[from] elf::ParseError),
    #[error("Invalid Address ({0})")]
    InvalidAddress(#[from] std::num::ParseIntError),
    #[error("Invalid Pointer 0x{0:X}")]
    InvalidPointer(usize),
    #[error("Parial read: {0} out of {1} bytes")]
    PartialRead(isize, usize),
    #[error("I/O Error ({0})")]
    Io(#[from] std::io::Error),
}

pub struct Memory {
    pid: i32,
    memory_regions: Vec<MemoryRegion>,
}

impl Memory {
    pub fn new(pid: i32) -> Result<Self, MemoryError> {
        let memory_regions = Self::read_memory_regions(pid)?;

        Ok(Self {
            pid,
            memory_regions,
        })
    }

    pub fn read<T: bytemuck::Pod>(&self, address: usize) -> Result<T, MemoryError> {
        let size = std::mem::size_of::<T>();
        let mut value: T = unsafe { std::mem::zeroed() };
        let bytes = bytemuck::bytes_of_mut(&mut value);

        let local_iov = iovec {
            iov_base: bytes.as_mut_ptr() as *mut libc::c_void,
            iov_len: size,
        };
        let remote_iov = iovec {
            iov_base: address as *mut libc::c_void,
            iov_len: size,
        };

        let read = unsafe { process_vm_readv(self.pid, &local_iov, 1, &remote_iov, 1, 0) };
        if read == -1 {
            Err(MemoryError::Io(std::io::Error::last_os_error()))
        } else if read as usize != size {
            Err(MemoryError::PartialRead(read, size))
        } else {
            Ok(value)
        }
    }

    pub fn read_bytes(&self, address: usize, count: usize) -> Result<Vec<u8>, MemoryError> {
        let mut buffer = vec![0u8; count];

        let local_iov = iovec {
            iov_base: buffer.as_mut_ptr() as *mut libc::c_void,
            iov_len: count,
        };
        let remote_iov = iovec {
            iov_base: address as *mut libc::c_void,
            iov_len: count,
        };

        let read = unsafe { process_vm_readv(self.pid, &local_iov, 1, &remote_iov, 1, 0) };
        if read == -1 {
            Err(MemoryError::Io(std::io::Error::last_os_error()))
        } else if read as usize != count {
            Err(MemoryError::PartialRead(read, count))
        } else {
            Ok(buffer)
        }
    }

    pub fn scan_signature(&self, signature: &IdaSignature) -> Result<Option<usize>, MemoryError> {
        for region in &self.memory_regions {
            if region.pathname.starts_with('[') || region.pathname.starts_with("/dev") {
                continue;
            }

            let address = self.scan_signature_in_region(signature, region)?;
            if let Some(address) = address {
                return Ok(Some(address));
            }
        }

        Ok(None)
    }

    fn scan_signature_in_region(
        &self,
        signature: &IdaSignature,
        region: &MemoryRegion,
    ) -> Result<Option<usize>, MemoryError> {
        let data = self.dump_elf(region)?;
        let pattern = signature.pattern();

        'outer: for i in 0..=data.len() - pattern.len() {
            for (j, &pat_byte) in pattern.iter().enumerate() {
                if let Some(b) = pat_byte
                    && data[i + j] != b
                {
                    continue 'outer;
                }
            }
            return Ok(Some(region.start + i));
        }

        Ok(None)
    }

    fn dump_elf(&self, region: &MemoryRegion) -> Result<Vec<u8>, MemoryError> {
        let magic: u32 = self.read(region.start)?;
        if magic != 0x7F_45_4C_46 && magic != 0x46_4C_45_7F {
            return Err(MemoryError::InvalidElf(magic));
        }

        self.read_bytes(region.start, region.end - region.start)
    }

    fn read_memory_regions(pid: i32) -> Result<Vec<MemoryRegion>, MemoryError> {
        let maps_file_name = format!("/proc/{pid}/maps");
        let maps_file = std::fs::read_to_string(maps_file_name)?;

        let mut region_map = BTreeMap::new();
        for line in maps_file.lines() {
            let parts: Vec<&str> = line.splitn(6, ' ').collect();
            if parts.len() < 2 {
                continue;
            }

            // Parse address range (format: "start-end")
            let address_range = parts[0];
            let range_parts: Vec<&str> = address_range.split('-').collect();
            if range_parts.len() != 2 {
                continue;
            }

            let start =
                usize::from_str_radix(range_parts[0], 16).map_err(MemoryError::InvalidAddress)?;
            let end =
                usize::from_str_radix(range_parts[1], 16).map_err(MemoryError::InvalidAddress)?;

            // Get pathname (last field)
            let pathname = if parts.len() >= 6 && !parts[5].is_empty() {
                parts[5].trim().to_string()
            } else {
                "[anonymous]".to_string()
            };

            region_map
                .entry(pathname)
                .or_insert_with(Vec::new)
                .push((start, end));
        }

        let mut regions = Vec::new();
        for (pathname, mut ranges) in region_map {
            ranges.sort_by_key(|&(start, _)| start);

            let mut merged_ranges = Vec::new();
            let mut current_range = ranges[0];

            for &(start, end) in &ranges[1..] {
                if start <= current_range.1 {
                    current_range.1 = current_range.1.max(end);
                } else {
                    merged_ranges.push(current_range);
                    current_range = (start, end);
                }
            }
            merged_ranges.push(current_range);

            for (start, end) in merged_ranges {
                regions.push(MemoryRegion {
                    start,
                    end,
                    pathname: pathname.clone(),
                });
            }
        }

        // Sort regions by start address
        regions.sort_by_key(|r| r.start);

        Ok(regions)
    }

    pub fn find_containing_region(&self, address: usize) -> Option<&MemoryRegion> {
        self.memory_regions
            .iter()
            .find(|&region| address >= region.start && address <= region.end)
            .map(|v| v as _)
    }

    pub fn is_pointer_valid(&self, pointer: usize) -> bool {
        for region in &self.memory_regions {
            if pointer >= region.start && pointer <= region.end {
                return true;
            }
        }
        false
    }

    pub fn memory_regions(&self) -> &[MemoryRegion] {
        &self.memory_regions
    }

    pub fn find_function(&self, function_name: &str) -> Result<Vec<FunctionLocation>, MemoryError> {
        let mut found_functions = Vec::new();

        for region in &self.memory_regions {
            let file_name = &region.pathname;
            if !file_name.starts_with('/') {
                continue;
            }
            let data = std::fs::read(file_name)?;
            let elf = ElfBytes::<AnyEndian>::minimal_parse(&data)?;
            let common_data = elf.find_common_data()?;
            let (dynsyms, dynstr) = match (common_data.dynsyms, common_data.dynsyms_strs) {
                (Some(dynsyms), Some(dynstr)) => (dynsyms, dynstr),
                _ => {
                    eprintln!("Could not find dynamic symbols for {}", region.pathname);
                    continue;
                }
            };

            for sym in dynsyms {
                if !self.is_exported_function(&sym) {
                    continue;
                }

                let name = dynstr.get(sym.st_name as usize)?;
                if name == function_name {
                    found_functions.push(FunctionLocation {
                        pathname: region.pathname.clone(),
                        address: sym.st_value as usize,
                    });
                }
            }
        }

        Ok(found_functions)
    }

    fn is_exported_function(&self, sym: &Symbol) -> bool {
        let is_function = sym.st_symtype() == elf::abi::STT_FUNC;

        let is_global_or_weak = matches!(sym.st_bind(), elf::abi::STB_GLOBAL | elf::abi::STB_WEAK);

        let is_defined = sym.st_shndx != elf::abi::SHN_UNDEF;

        let has_name = sym.st_name != 0;

        is_function && is_global_or_weak && is_defined && has_name
    }
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub pathname: String,
}

pub struct FunctionLocation {
    pub pathname: String,
    pub address: usize,
}
