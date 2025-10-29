use crate::{
    memory::{Memory, MemoryError},
    value::Value,
};

#[derive(Debug, Clone)]
pub enum DataType {
    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    F32,
    F64,

    Pointer,
    Pointer32,
    Pointer64,

    Vec2,
    Vec3,
    Vec4,
    Mat4,

    Rgb,
    Rgba,
    Color32,
}

impl DataType {
    pub fn read(&self, memory: &Memory, address: usize) -> Result<Value, MemoryError> {
        let value = match self {
            DataType::U8 => Value::U8(memory.read(address)?),
            DataType::U16 => Value::U16(memory.read(address)?),
            DataType::U32 => Value::U32(memory.read(address)?),
            DataType::U64 => Value::U64(memory.read(address)?),

            DataType::I8 => Value::I8(memory.read(address)?),
            DataType::I16 => Value::I16(memory.read(address)?),
            DataType::I32 => Value::I32(memory.read(address)?),
            DataType::I64 => Value::I64(memory.read(address)?),

            DataType::F32 => Value::F32(memory.read(address)?),
            DataType::F64 => Value::F64(memory.read(address)?),

            DataType::Pointer => Value::Pointer(memory.read(address)?),
            DataType::Pointer32 => Value::Pointer32(memory.read(address)?),
            DataType::Pointer64 => Value::Pointer64(memory.read(address)?),

            DataType::Vec2 => Value::Vec2(memory.read(address)?),
            DataType::Vec3 => Value::Vec3(memory.read(address)?),
            DataType::Vec4 => Value::Vec4(memory.read(address)?),
            DataType::Mat4 => Value::Mat4(memory.read(address)?),

            DataType::Rgb => Value::Rgb(memory.read(address)?),
            DataType::Rgba => Value::Rgba(memory.read(address)?),
            DataType::Color32 => Value::Color32(memory.read(address)?),
        };

        Ok(value)
    }
}
