use java_runtime::prelude::*;
use super::*;

// StringBuilder 内部字符串内容通过注入字段 _sb 存储（Rc<RefCell<String>> 支持共享可变）。
/// @field _sb: JField<Rc<RefCell<std::string::String>>>

impl super::StringBuilder {
    pub fn new() -> Result<StringBuilder> {
        Ok(StringBuilder::default())
    }

    pub fn new_i(_capacity: i32) -> Result<Self> {
        Ok(Self::default())
    }

    pub fn new_str(str: String) -> Result<Self> {
        let sb = Self::default();
        sb._sb.get().borrow_mut().push_str(&format!("{}", str));
        Ok(sb)
    }

    // ── append 重载 ──────────────────────────────────────────────────────────

    pub fn append_str(&self, s: String) -> Result<StringBuilder> {
        self._sb.get().borrow_mut().push_str(&format!("{}", s));
        Ok(self.clone())
    }

    pub fn append_i(&self, v: i32) -> Result<StringBuilder> {
        self._sb.get().borrow_mut().push_str(&v.to_string());
        Ok(self.clone())
    }

    pub fn append_l(&self, v: i64) -> Result<StringBuilder> {
        self._sb.get().borrow_mut().push_str(&v.to_string());
        Ok(self.clone())
    }

    pub fn append_d(&self, v: f64) -> Result<StringBuilder> {
        self._sb.get().borrow_mut().push_str(&v.to_string());
        Ok(self.clone())
    }

    pub fn append_f(&self, v: f32) -> Result<StringBuilder> {
        self._sb.get().borrow_mut().push_str(&v.to_string());
        Ok(self.clone())
    }

    pub fn append_z(&self, v: bool) -> Result<StringBuilder> {
        self._sb.get().borrow_mut().push_str(&v.to_string());
        Ok(self.clone())
    }

    pub fn append_c(&self, v: u16) -> Result<StringBuilder> {
        if let Some(c) = char::from_u32(v as u32) {
            self._sb.get().borrow_mut().push(c);
        }
        Ok(self.clone())
    }

    pub fn append_obj(&self, obj: Object) -> Result<StringBuilder> {
        self._sb.get().borrow_mut().push_str("Object");
        Ok(self.clone())
    }

    pub fn toString(&self) -> Result<String> {
        let s = self._sb.get().borrow().clone();
        Ok(String::from(s.as_str()))
    }

    pub fn length(&self) -> Result<i32> {
        Ok(self._sb.get().borrow().len() as i32)
    }
}
