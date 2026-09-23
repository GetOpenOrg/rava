use crate::prelude::*;
use super::byte_array::ByteArray;

// jdk.internal.util.ByteArray 伴生：byte[] 的 BIG_ENDIAN 定宽访问器族
//（JDK 里经 VarHandle（SHORT/CHAR/INT/LONG/FLOAT/DOUBLE）落到内存序原语；
// 原生二进制以无符号位型的字节重排实现同一语义——有符号值按位型往返，
// 浮点族经 to_bits/from_bits，set 为 get 的镜像）。getFloatRaw/getDoubleRaw
// 与 getFloat/getDouble 同值（无 raw 位型差异）。

macro_rules! be_get {
    ($t:ty, $arr:expr, $off:expr) => {{
        let mut v: $t = 0;
        let n = std::mem::size_of::<$t>();
        for i in 0..n {
            v = (v << 8) | ($arr.get($off + i as i32)? as u8) as $t;
        }
        v
    }};
}

macro_rules! be_set {
    ($t:ty, $arr:expr, $off:expr, $val:expr) => {{
        let mut v: $t = $val;
        let n = std::mem::size_of::<$t>();
        for i in (0..n).rev() {
            $arr.set($off + i as i32, (v & 0xFF) as u8 as i8)?;
            v >>= 8;
        }
    }};
}

impl ByteArray {
    #[jvm_boundary]
    pub fn getBoolean(array: JArray<i8>, offset: i32) -> Result<bool> {
        Ok(array.get(offset)? != 0)
    }

    #[jvm_boundary]
    pub fn setBoolean(array: JArray<i8>, offset: i32, value: bool) -> Result<()> {
        array.set(offset, if value { 1 } else { 0 })
    }

    #[jvm_boundary]
    pub fn getChar(array: JArray<i8>, offset: i32) -> Result<u16> {
        Ok(be_get!(u16, array, offset))
    }

    #[jvm_boundary]
    pub fn setChar(array: JArray<i8>, offset: i32, value: u16) -> Result<()> {
        be_set!(u16, array, offset, value);
        Ok(())
    }

    #[jvm_boundary]
    pub fn getShort(array: JArray<i8>, offset: i32) -> Result<i16> {
        Ok(be_get!(u16, array, offset) as i16)
    }

    #[jvm_boundary]
    pub fn getUnsignedShort(array: JArray<i8>, offset: i32) -> Result<i32> {
        Ok(be_get!(u16, array, offset) as i32)
    }

    #[jvm_boundary]
    pub fn setShort(array: JArray<i8>, offset: i32, value: i16) -> Result<()> {
        be_set!(u16, array, offset, value as u16);
        Ok(())
    }

    #[jvm_boundary]
    pub fn setUnsignedShort(array: JArray<i8>, offset: i32, value: i32) -> Result<()> {
        be_set!(u16, array, offset, value as u16);
        Ok(())
    }

    #[jvm_boundary]
    pub fn getInt(array: JArray<i8>, offset: i32) -> Result<i32> {
        Ok(be_get!(u32, array, offset) as i32)
    }

    #[jvm_boundary]
    pub fn setInt(array: JArray<i8>, offset: i32, value: i32) -> Result<()> {
        be_set!(u32, array, offset, value as u32);
        Ok(())
    }

    #[jvm_boundary]
    pub fn getFloat(array: JArray<i8>, offset: i32) -> Result<f32> {
        Ok(f32::from_bits(be_get!(u32, array, offset)))
    }

    #[jvm_boundary]
    pub fn getFloatRaw(array: JArray<i8>, offset: i32) -> Result<f32> {
        Self::getFloat(array, offset)
    }

    #[jvm_boundary]
    pub fn setFloat(array: JArray<i8>, offset: i32, value: f32) -> Result<()> {
        be_set!(u32, array, offset, value.to_bits());
        Ok(())
    }

    #[jvm_boundary]
    pub fn setFloatRaw(array: JArray<i8>, offset: i32, value: f32) -> Result<()> {
        Self::setFloat(array, offset, value)
    }

    #[jvm_boundary]
    pub fn getLong(array: JArray<i8>, offset: i32) -> Result<i64> {
        Ok(be_get!(u64, array, offset) as i64)
    }

    #[jvm_boundary]
    pub fn setLong(array: JArray<i8>, offset: i32, value: i64) -> Result<()> {
        be_set!(u64, array, offset, value as u64);
        Ok(())
    }

    #[jvm_boundary]
    pub fn getDouble(array: JArray<i8>, offset: i32) -> Result<f64> {
        Ok(f64::from_bits(be_get!(u64, array, offset)))
    }

    #[jvm_boundary]
    pub fn getDoubleRaw(array: JArray<i8>, offset: i32) -> Result<f64> {
        Self::getDouble(array, offset)
    }

    #[jvm_boundary]
    pub fn setDouble(array: JArray<i8>, offset: i32, value: f64) -> Result<()> {
        be_set!(u64, array, offset, value.to_bits());
        Ok(())
    }

    #[jvm_boundary]
    pub fn setDoubleRaw(array: JArray<i8>, offset: i32, value: f64) -> Result<()> {
        Self::setDouble(array, offset, value)
    }
}
