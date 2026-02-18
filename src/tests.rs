use glam::{Mat4, Quat, vec3};

use crate::{
    address::AddressLocator,
    data_type::DataType,
    memory::{Memory, MemoryError},
    value::Value,
};

fn pid() -> i32 {
    std::process::id().cast_signed()
}

fn address<T>(value: &T) -> usize {
    value as *const T as usize
}

fn read(address: AddressLocator, data_type: DataType) -> Result<Value, MemoryError> {
    let memory = Memory::new(pid())?;
    let address = address.resolve(&memory)?;
    data_type.read(&memory, address)
}

#[test]
fn test_simple_read() -> Result<(), MemoryError> {
    let buf = 0x12345678;
    let address = address(&buf);

    let value = read(AddressLocator::Absolute(address), DataType::I32)?;

    assert!(value == Value::I32(buf));

    Ok(())
}

#[test]
fn test_large_read() -> Result<(), MemoryError> {
    let buf = Mat4::from_scale_rotation_translation(
        vec3(0.5, 1.2, 2.0),
        Quat::from_xyzw(2.0, 5.0, -7.2, 1.0),
        vec3(52.7, 105.5, -97.3),
    );
    let address = address(&buf);

    let value = read(AddressLocator::Absolute(address), DataType::Mat4)?;

    assert!(value == Value::Mat4(buf));

    Ok(())
}
