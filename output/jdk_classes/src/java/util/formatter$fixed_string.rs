#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Formatter$FixedString",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Formatter$FormatString",
    access      = "",
    source      = "Formatter.java",
))]
pub struct Formatter_FixedString {
    #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub s: Field<String>,
    #[cfg_attr(any(), java_field(name = "start", descriptor = "I", access = "private final"))]
    pub start: Field<i32>,
    #[cfg_attr(any(), java_field(name = "end", descriptor = "I", access = "private final"))]
    pub end: Field<i32>,
}

impl Formatter_FixedString {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;II)V"))]
    pub fn new(s: String, start: i32, end: i32) -> Result<Self> {
        let this = Self { s: Field::new(String::new()), start: Field::new(0), end: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.s.set(s);
        this.start.set(start);
        this.end.set(end);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "index", descriptor = "()I", access = "public"))]
    pub fn index(&self) -> Result<i32> {
        let this = self;
        Ok(-2i32)
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "public"))]
    pub fn print(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        let _t0 = fmt.a.get().append(this.s.get(), this.start.get(), this.end.get())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.s.get().substring(this.start.get(), this.end.get())?;
        Ok(_t0)
    }
}
