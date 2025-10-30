use std::{num::ParseIntError, time::Duration};

use crate::{
    data_type::DataType,
    signature::{AddressLocator, IdaSignature, Offset},
};

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    Read {
        #[clap(value_parser=parse_pid)]
        pid: i32,
        #[clap(value_parser=parse_address_locator)]
        address: AddressLocator,
        #[clap(value_parser=parse_data_type)]
        data_type: DataType,
    },
    Watch {
        #[clap(value_parser=parse_pid)]
        pid: i32,
        #[clap(value_parser=parse_address_locator)]
        address: AddressLocator,
        #[clap(value_parser=parse_data_type)]
        data_type: DataType,
        #[clap(value_parser=parse_duration)]
        #[arg(short, long, default_value = "1s")]
        interval: Duration,
    },
    Find {
        #[clap(value_parser=parse_pid)]
        pid: i32,
        #[clap(value_parser=parse_address_locator)]
        address: AddressLocator,
    },
    FindFunction {
        #[clap(value_parser=parse_pid)]
        pid: i32,
        function_name: String,
    },
    List {
        #[clap(value_parser=parse_pid)]
        pid: i32,
    },
}

fn parse_pid(s: &str) -> Result<i32, String> {
    if s == "self" {
        return Ok(std::process::id() as i32);
    }

    let pid = s
        .parse::<i32>()
        .map_err(|e: ParseIntError| format!("Invalid PID '{s}': {e}"))?;

    if pid <= 0 || pid > 2_i32.pow(22) {
        Err(format!("Invalid PID '{s}'"))
    } else {
        Ok(pid)
    }
}

fn parse_address_locator(s: &str) -> Result<AddressLocator, String> {
    // basic address
    if let Some(stripped) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        let addr =
            usize::from_str_radix(stripped, 16).map_err(|e| format!("Invalid hex address: {e}"))?;
        return Ok(AddressLocator::Absolute(addr));
    }

    // split into potential pattern and pointer chain parts
    let parts: Vec<&str> = s.split("->").map(|part| part.trim()).collect();

    let pattern = parse_ida_signature_with_offset(parts[0])?;

    if parts.len() > 1 {
        let pointers: Result<Vec<usize>, ParseIntError> =
            parts[1..].iter().map(|&ptr| parse_pointer(ptr)).collect();

        let pointers = pointers.map_err(|e| format!("Invalid pointer: {e}"))?;
        Ok(AddressLocator::PointerChain(pattern, pointers))
    } else {
        Ok(AddressLocator::Pattern(pattern))
    }
}

fn parse_ida_signature_with_offset(s: &str) -> Result<IdaSignature, String> {
    if let Some((signature, offset)) = s.split_once('@') {
        let Some((offset, instruction_size)) = offset.split_once('/') else {
            return Err(format!("Invalid offset '{offset}'"));
        };
        let offset: usize = offset
            .parse()
            .map_err(|e| format!("Invalid offset '{offset}': {e}"))?;
        let instruction_size: usize = instruction_size
            .parse()
            .map_err(|e| format!("Invalid instruction size '{instruction_size}': {e}"))?;
        let signature = parse_ida_signature(signature)?;
        Ok(IdaSignature::new(
            signature,
            Some(Offset {
                offset,
                instruction_size,
            }),
        ))
    } else {
        let signature = parse_ida_signature(s)?;
        Ok(IdaSignature::new(signature, None))
    }
}

fn parse_ida_signature(s: &str) -> Result<Vec<Option<u8>>, String> {
    s.split_whitespace()
        .map(|byte| {
            if byte == "?" || byte == "??" {
                Ok(None)
            } else {
                u8::from_str_radix(byte, 16)
                    .map(Some)
                    .map_err(|e| format!("Invalid hex byte '{byte}': {e}"))
            }
        })
        .collect()
}

fn parse_pointer(s: &str) -> Result<usize, ParseIntError> {
    if let Some(stripped) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        usize::from_str_radix(stripped, 16)
    } else {
        usize::from_str_radix(s, 16)
    }
}

fn parse_data_type(s: &str) -> Result<DataType, String> {
    use DataType::*;

    let data_type = match s {
        "u8" => U8,
        "u16" => U16,
        "u32" => U32,
        "u64" => U64,

        "i8" => I8,
        "i16" => I16,
        "i32" => I32,
        "i64" => I64,

        "f32" => F32,
        "f64" => F64,

        "pointer" => Pointer,
        "pointer32" => Pointer32,
        "pointer64" => Pointer64,

        "vec2" => Vec2,
        "vec3" => Vec3,
        "vec4" => Vec4,
        "mat4" => Mat4,

        "rgb" => Rgb,
        "rgba" => Rgba,
        "color32" => Color32,

        _ => return Err(format!("Unknown data type '{s}'")),
    };

    Ok(data_type)
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    if let Some(us) = s.strip_suffix("us") {
        let us = us.parse::<u64>().map_err(|e| e.to_string())?;
        Ok(Duration::from_micros(us))
    } else if let Some(ms) = s.strip_suffix("ms") {
        let ms = ms.parse::<u64>().map_err(|e| e.to_string())?;
        Ok(Duration::from_millis(ms))
    } else if let Some(s) = s.strip_suffix('s') {
        let s = s.parse::<u64>().map_err(|e| e.to_string())?;
        Ok(Duration::from_secs(s))
    } else {
        Err(format!("Invalid Duration '{s}'"))
    }
}
