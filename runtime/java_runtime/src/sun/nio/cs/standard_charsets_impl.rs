//! `sun/nio/cs/StandardCharsets` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 的标准 charset provider（`Charset.<clinit>` 的 `standardProvider`）：`charsetForName`
//! 经别名表 + 类名反射实例化 `sun.nio.cs.*`。原生侧的 charset 类均为手写边界（本目录
//! `*_impl.rs`，各自在构造器里填 name / aliases 字段）——查找直接按这些实例的名字与别名
//! 大小写不敏感匹配（名字 / 别名的唯一真源是各 charset 的构造器）。实例在线程内缓存，
//! 与 JDK provider 的实例缓存语义一致（同名多次查找得同一对象）。
//! 消费方：`Charset.forName` / `lookup2`（`new PrintStream(out, autoFlush, "UTF-8")` 等）。

use crate::prelude::*;
use super::standard_charsets::StandardCharsets;
use crate::java::nio::charset::Charset;

std::thread_local! {
    static STANDARD: std::cell::RefCell<Vec<Charset>> = std::cell::RefCell::new(Vec::new());
}

fn standard_charsets() -> Result<Vec<Charset>> {
    if let Some(v) = STANDARD.with(|s| { let s = s.borrow(); (!s.is_empty()).then(|| s.clone()) }) {
        return Ok(v);
    }
    let v: Vec<Charset> = vec![
        super::UTF_8::INSTANCE()?.into(),
        super::ISO_8859_1::INSTANCE()?.into(),
        super::US_ASCII::INSTANCE()?.into(),
        super::UTF_16::new()?.into(),
        super::UTF_16BE::new()?.into(),
        super::UTF_16LE::new()?.into(),
        super::UTF_32::new()?.into(),
        super::UTF_32BE::new()?.into(),
        super::UTF_32LE::new()?.into(),
    ];
    STANDARD.with(|s| *s.borrow_mut() = v.clone());
    Ok(v)
}

fn matches(cs: &Charset, name: &str) -> Result<bool> {
    if format!("{}", cs.__get_name()).eq_ignore_ascii_case(name) {
        return Ok(true);
    }
    let aliases = cs.__get_aliases();
    if aliases.is_jvm_null() {
        return Ok(false);
    }
    for i in 0..aliases.len()? {
        if format!("{}", aliases.get(i)?).eq_ignore_ascii_case(name) {
            return Ok(true);
        }
    }
    Ok(false)
}

impl StandardCharsets {
    /// `<init>()`：provider 本身无状态。
    #[jvm_boundary(upcalls = "sun/nio/cs/UTF_8.<init>:()V sun/nio/cs/ISO_8859_1.<init>:()V sun/nio/cs/US_ASCII.<init>:()V sun/nio/cs/UTF_16.<init>:()V sun/nio/cs/UTF_16BE.<init>:()V sun/nio/cs/UTF_16LE.<init>:()V sun/nio/cs/UTF_32.<init>:()V sun/nio/cs/UTF_32BE.<init>:()V sun/nio/cs/UTF_32LE.<init>:()V")]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// `charsetForName(String)`：名字或别名（大小写不敏感）命中的标准 charset；未命中 → null
    /// （`Charset.lookup2` 继续扩展 provider 查找，最终 `UnsupportedCharsetException`）。
    pub fn __impl_charsetForName(&self, charsetName: String) -> Result<Charset> {
        let name = format!("{}", charsetName);
        for cs in standard_charsets()? {
            if matches(&cs, &name)? {
                return Ok(cs);
            }
        }
        Ok(Charset::default())
    }
}
