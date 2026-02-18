use glam::{Mat4, Quat, Vec2, Vec3, Vec4, vec2, vec3, vec4};

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

fn assert_read(address: usize, data_type: DataType, expected: Value) -> Result<(), MemoryError> {
    let value = read(AddressLocator::Absolute(address), data_type)?;
    assert!(value == expected);
    Ok(())
}

#[test]
fn test_simple_read() -> Result<(), MemoryError> {
    let buf = 0x12345678;
    let address = address(&buf);

    assert_read(address, DataType::I32, Value::I32(buf))
}

#[test]
fn test_large_read() -> Result<(), MemoryError> {
    let buf = Mat4::from_scale_rotation_translation(
        vec3(0.5, 1.2, 2.0),
        Quat::from_xyzw(2.0, 5.0, -7.2, 1.0),
        vec3(52.7, 105.5, -97.3),
    );
    let address = address(&buf);

    assert_read(address, DataType::Mat4, Value::Mat4(buf))
}

#[test]
fn test_integer_reads() -> Result<(), MemoryError> {
    let u8_val: u8 = 0x7a;
    let u16_val: u16 = 0xbeef;
    let u32_val: u32 = 0x1234_5678;
    let u64_val: u64 = 0x1234_5678_9abc_def0;
    let i8_val: i8 = -100;
    let i16_val: i16 = -32_000;
    let i32_val: i32 = -2_000_000;
    let i64_val: i64 = -9_000_000_000;

    assert_read(address(&u8_val), DataType::U8, Value::U8(u8_val))?;
    assert_read(address(&u16_val), DataType::U16, Value::U16(u16_val))?;
    assert_read(address(&u32_val), DataType::U32, Value::U32(u32_val))?;
    assert_read(address(&u64_val), DataType::U64, Value::U64(u64_val))?;
    assert_read(address(&i8_val), DataType::I8, Value::I8(i8_val))?;
    assert_read(address(&i16_val), DataType::I16, Value::I16(i16_val))?;
    assert_read(address(&i32_val), DataType::I32, Value::I32(i32_val))?;
    assert_read(address(&i64_val), DataType::I64, Value::I64(i64_val))?;

    Ok(())
}

#[test]
fn test_float_reads() -> Result<(), MemoryError> {
    let f32_val: f32 = 1.5;
    let f64_val: f64 = -3.25;

    assert_read(address(&f32_val), DataType::F32, Value::F32(f32_val))?;
    assert_read(address(&f64_val), DataType::F64, Value::F64(f64_val))?;

    Ok(())
}

#[test]
fn test_pointer_reads() -> Result<(), MemoryError> {
    let target: u64 = 0xdead_beef_cafe_babe;
    let pointer_value: usize = &target as *const u64 as usize;
    let pointer32_value: u32 = 0xdead_beef;
    let pointer64_value: u64 = 0x0123_4567_89ab_cdef;

    assert_read(
        address(&pointer_value),
        DataType::Pointer,
        Value::Pointer(pointer_value),
    )?;
    assert_read(
        address(&pointer32_value),
        DataType::Pointer32,
        Value::Pointer32(pointer32_value),
    )?;
    assert_read(
        address(&pointer64_value),
        DataType::Pointer64,
        Value::Pointer64(pointer64_value),
    )?;

    Ok(())
}

#[test]
fn test_vector_reads() -> Result<(), MemoryError> {
    let v2 = Vec2::new(1.0, -2.5);
    let v3 = vec3(0.25, 4.0, -8.0);
    let v4 = vec4(1.0, 2.0, 3.0, 4.0);

    assert_read(address(&v2), DataType::Vec2, Value::Vec2(v2))?;
    assert_read(address(&v3), DataType::Vec3, Value::Vec3(v3))?;
    assert_read(address(&v4), DataType::Vec4, Value::Vec4(v4))?;

    Ok(())
}

#[test]
fn test_color_reads() -> Result<(), MemoryError> {
    let rgb: [u8; 3] = [12, 34, 56];
    let rgba: [u8; 4] = [78, 90, 123, 255];
    let color32: [f32; 4] = [0.2, 0.4, 0.6, 0.8];

    assert_read(address(&rgb), DataType::Rgb, Value::Rgb(rgb))?;
    assert_read(address(&rgba), DataType::Rgba, Value::Rgba(rgba))?;
    assert_read(
        address(&color32),
        DataType::Color32,
        Value::Color32(color32),
    )?;

    Ok(())
}

#[test]
fn test_read_bytes() -> Result<(), MemoryError> {
    let memory = Memory::new(pid())?;
    let buffer: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let bytes = memory.read_bytes(address(&buffer), buffer.len())?;

    assert!(bytes == buffer);

    Ok(())
}
