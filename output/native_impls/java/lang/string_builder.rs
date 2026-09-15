use java_runtime::prelude::*;
use super::*;

// StringBuilder 的字符内容通过父类 AbstractStringBuilder 的 value (Vec<i8>) 和 count (i32) 字段存储。
// Latin-1 编码：每个字节直接对应一个字符（ASCII 范围内完全兼容）。

impl super::StringBuilder {
    pub fn new() -> Result<StringBuilder> {
        Ok(StringBuilder::default())
    }

    pub fn new_i(_capacity: i32) -> Result<Self> {
        Ok(Self::default())
    }

    pub fn new_str(str: String) -> Result<Self> {
        let sb = Self::default();
        _sb_append(&sb, format!("{}", str).as_str());
        Ok(sb)
    }

    // ── append 重载 ──────────────────────────────────────────────────────────

    pub fn append_str(&self, s: String) -> Result<StringBuilder> {
        _sb_append(self, format!("{}", s).as_str());
        Ok(Clone::clone(self))
    }

    pub fn append_i(&self, v: i32) -> Result<StringBuilder> {
        _sb_append(self, &v.to_string());
        Ok(Clone::clone(self))
    }

    pub fn append_l(&self, v: i64) -> Result<StringBuilder> {
        _sb_append(self, &v.to_string());
        Ok(Clone::clone(self))
    }

    pub fn append_d(&self, v: f64) -> Result<StringBuilder> {
        _sb_append(self, &v.to_string());
        Ok(Clone::clone(self))
    }

    pub fn append_f(&self, v: f32) -> Result<StringBuilder> {
        _sb_append(self, &v.to_string());
        Ok(Clone::clone(self))
    }

    pub fn append_z(&self, v: bool) -> Result<StringBuilder> {
        _sb_append(self, &v.to_string());
        Ok(Clone::clone(self))
    }

    pub fn append_c(&self, v: u16) -> Result<StringBuilder> {
        if let Some(c) = char::from_u32(v as u32) {
            let mut buf = [0u8; 4];
            _sb_append(self, c.encode_utf8(&mut buf));
        }
        Ok(Clone::clone(self))
    }

    pub fn append_obj(&self, obj: Object) -> Result<StringBuilder> {
        _sb_append(self, "Object");
        Ok(Clone::clone(self))
    }

    pub fn toString(&self) -> Result<String> {
        let value = self._super.value.get();
        let count = self._super.count.get() as usize;
        let borrowed = value.borrow();
        let slice = &borrowed[..count.min(borrowed.len())];
        let s = std::string::String::from_utf8_lossy(
            &slice.iter().map(|&b| b as u8).collect::<Vec<_>>()
        ).into_owned();
        Ok(String::from(s.as_str()))
    }

    pub fn length(&self) -> Result<i32> {
        Ok(self._super.count.get())
    }
}

fn _sb_append(sb: &super::StringBuilder, s: &str) {
    let bytes: Vec<i8> = s.bytes().map(|b| b as i8).collect();
    let value = sb._super.value.get();
    let count = sb._super.count.get() as usize;
    let needed = count + bytes.len();
    let mut v = value.borrow_mut();
    if v.len() < needed {
        v.resize(needed, 0i8);
    }
    v[count..needed].copy_from_slice(&bytes);
    sb._super.count.set(needed as i32);
}
