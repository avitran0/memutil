use std::fmt::Display;

use crate::memory::{Memory, MemoryError};

#[derive(Debug, Clone)]
pub enum AddressLocator {
    Absolute(usize),
    Pattern(IdaSignature),
    PointerChain(IdaSignature, Vec<usize>),
}

impl AddressLocator {
    pub fn resolve(&self, memory: &Memory) -> Result<usize, MemoryError> {
        match self {
            AddressLocator::Absolute(address) => {
                if memory.is_pointer_valid(*address) {
                    Ok(*address)
                } else {
                    Err(MemoryError::InvalidPointer(*address))
                }
            }
            AddressLocator::Pattern(signature) => self.resolve_signature(memory, signature),
            AddressLocator::PointerChain(signature, pointers) => {
                let base_address = self.resolve_signature(memory, signature)?;

                if pointers.is_empty() {
                    return Ok(base_address);
                }

                let mut address = base_address;
                let (deref_pointers, final_offset) = pointers.split_at(pointers.len() - 1);

                for &offset in deref_pointers {
                    let new_address: usize = memory.read(address + offset)?;
                    if !memory.is_pointer_valid(new_address) {
                        return Err(MemoryError::InvalidPointer(new_address));
                    }
                    address = new_address;
                }
                
                Ok(address + final_offset[0])
            }
        }
    }

    fn resolve_signature(
        &self,
        memory: &Memory,
        signature: &IdaSignature,
    ) -> Result<usize, MemoryError> {
        let Some(base_address) = memory.scan_signature(signature)? else {
            return Err(MemoryError::SignatureNotFound(signature.clone()));
        };

        if let Some(offset) = &signature.offset {
            let rip_address: i32 = memory.read(base_address + offset.offset)?;
            Ok(base_address
                .wrapping_add_signed(rip_address as isize)
                .wrapping_add(offset.instruction_size))
        } else {
            Ok(base_address)
        }
    }
}

impl Display for AddressLocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absolute(address) => write!(f, "0x{address:X}"),
            Self::Pattern(signature) => write!(f, "{signature}"),
            Self::PointerChain(signature, pointers) => {
                let pointer_str = pointers
                    .iter()
                    .map(|pointer| format!("{pointer:X}"))
                    .collect::<Vec<_>>()
                    .join(" -> ");
                write!(f, "{signature} {pointer_str}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdaSignature {
    signature: Vec<Option<u8>>,
    offset: Option<Offset>,
}

impl IdaSignature {
    pub fn new(signature: Vec<Option<u8>>, offset: Option<Offset>) -> Self {
        Self { signature, offset }
    }

    pub fn pattern(&self) -> &[Option<u8>] {
        &self.signature
    }
}

impl Display for IdaSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sig_str = self
            .signature
            .iter()
            .map(|byte_opt| match byte_opt {
                Some(byte) => format!("{byte:02X}"),
                None => String::from("??"),
            })
            .collect::<Vec<_>>()
            .join(" ");

        if let Some(offset) = &self.offset {
            write!(f, "{sig_str} {offset}")
        } else {
            write!(f, "{sig_str}")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Offset {
    pub offset: usize,
    pub instruction_size: usize,
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}/{}", self.offset, self.instruction_size)
    }
}
