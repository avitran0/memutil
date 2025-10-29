use std::fmt::Display;

use glam::{Mat4, Vec2, Vec3, Vec4};

#[derive(Debug)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Pointer(usize),
    Pointer32(u32),
    Pointer64(u64),

    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat4(Mat4),

    Rgb([u8; 3]),
    Rgba([u8; 4]),
    Color32([f32; 4]),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::U8(v) => write!(f, "{}u8", v),
            Value::U16(v) => write!(f, "{}u16", v),
            Value::U32(v) => write!(f, "{}u32", v),
            Value::U64(v) => write!(f, "{}u64", v),

            Value::I8(v) => write!(f, "{}i8", v),
            Value::I16(v) => write!(f, "{}i16", v),
            Value::I32(v) => write!(f, "{}i32", v),
            Value::I64(v) => write!(f, "{}i64", v),

            Value::F32(v) => write!(f, "{:?}f32", v),
            Value::F64(v) => write!(f, "{:?}f64", v),

            Value::Pointer(v) => write!(f, "0x{:x}", v),
            Value::Pointer32(v) => write!(f, "0x{:x}", v),
            Value::Pointer64(v) => write!(f, "0x{:x}", v),

            Value::Vec2(v) => write!(f, "{}", v),
            Value::Vec3(v) => write!(f, "{}", v),
            Value::Vec4(v) => write!(f, "{}", v),
            Value::Mat4(v) => write!(f, "{}", v),

            Value::Rgb(v) => write!(f, "#{:02x}{:02x}{:02x}", v[0], v[1], v[2]),
            Value::Rgba(v) => write!(f, "#{:02x}{:02x}{:02x}{:02x}", v[0], v[1], v[2], v[3]),
            Value::Color32(v) => write!(f, "({:?}, {:?}, {:?}, {:?})", v[0], v[1], v[2], v[3]),
        }
    }
}
