use java_runtime::prelude::*;
use super::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/StringBuilder",
    super_class       = "java/lang/AbstractStringBuilder",
    interfaces        = "java/lang/Appendable,java/io/Serializable,java/lang/Comparable,java/lang/CharSequence",
    access            = "public",
    modifiers         = "final",
    generic_signature = "Ljava/lang/AbstractStringBuilder;Ljava/lang/Appendable;Ljava/io/Serializable;Ljava/lang/Comparable<Ljava/lang/StringBuilder;>;Ljava/lang/CharSequence;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "StringBuilder.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct StringBuilder {
    pub _super: AbstractStringBuilder,
    pub _sb: JField<Rc<RefCell<std::string::String>>>,
}

impl StringBuilder {
    pub fn as_abstract_string_builder(&self) -> &AbstractStringBuilder { &self._super }
    pub fn into_abstract_string_builder(self) -> AbstractStringBuilder { self._super }
}

impl From<StringBuilder> for AbstractStringBuilder {
    fn from(v: StringBuilder) -> AbstractStringBuilder { v._super }
}

impl StringBuilder {
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
