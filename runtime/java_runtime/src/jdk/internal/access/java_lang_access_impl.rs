//! `jdk/internal/access/JavaLangAccess` 的实现对象（仅当 `java_lang_access.rs`
//! 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 中该接口由 `java/lang/System$JavaLangAccess`（System 的内部类）实现，
//! `System.<clinit>` 经 `setJavaLangAccess()` 登记到 SharedSecrets。该内部类实现
//! `jdk/internal/` 内部接口，属内部边界族 → 本文件整体手写（规则 3b）：只实现
//! 调用链触达的 `newStringNoRepl`、`getEnumConstantsShared` 与 `join`，其余方法
//! 走接口 vtable trait 的默认 `panic!("stub: ...")` 存根（生成侧
//! java_lang_access.rs 自带）。

use crate::prelude::*;
use super::java_lang_access::JavaLangAccess__VTable;
use crate::java::lang::Class;
use crate::java::lang::Enum;
use crate::java::lang::String;
use crate::java::nio::charset::Charset;

/// `java/lang/System$JavaLangAccess` 的手写实现对象。
///
/// 无实例状态（JDK 原型也无状态，全部方法转发静态语义）；唯一职责是经
/// `ObjectVTable::__interface` 把自身填入 `Option<Rc<dyn JavaLangAccess__VTable>>`
/// 槽位——等价 JVM itable 条目，接口载体（`Into::<JavaLangAccess>::into(obj)`）
/// 的分派由此命中。
pub(super) struct SystemJavaLangAccess;

/// `String.getBytes(byte[] dst, int dstBegin, byte coder)` 的字节级等价：
/// 把 `src` 的内容写入 `dst`（UTF-16 布局与生成侧 StringUTF16.putChar 的
/// HI_BYTE_SHIFT/LO_BYTE_SHIFT 一致，即平台字节序）。src 为 Latin1 且目标
/// coder 为 UTF16 时逐字节展宽（高位字节按平台字节序补 0）。
fn _get_bytes_into(src_val: &[i8], src_coder: i8, dst: &mut Vec<i8>, dst_coder: i8) {
    if src_coder == dst_coder {
        dst.extend_from_slice(src_val);
    } else {
        // Latin1 → UTF16：零字节与字符字节按 HI/LO_BYTE_SHIFT 顺序排列
        // （大端 [0, b]；小端 [b, 0]）
        for &b in src_val {
            if cfg!(target_endian = "big") {
                dst.push(0);
                dst.push(b);
            } else {
                dst.push(b);
                dst.push(0);
            }
        }
    }
}

impl JavaLangAccess__VTable for SystemJavaLangAccess {
    /// `getBytesNoRepl(String, Charset)`：JDK 转发 `StringCoding.getBytesNoRepl`
    /// （REPORT 动作编码，不可映射抛 CharacterCodingException）。POSIX 档 A
    /// 消费面（Files.writeString → UTF-8）：UTF-8 直编码（全部 Unicode 标量可
    /// 映射；孤立代理对经 Display 的替换呈现，NoRepl 严格面外）；ISO-8859-1
    /// 逐 code unit 一字节（> 0xFF 不可映射，错码路径与 newStringNoRepl 同一
    /// stub 文本报错惯例）。其余 charset 未消费。
    fn getBytesNoRepl(&self, arg0: String, arg1: Charset) -> Result<JArray<i8>> {
        let name = if arg1.is_jvm_null() {
            std::string::String::new()
        } else {
            format!("{}", arg1.__get_name())
        };
        match name.as_str() {
            "UTF-8" => {
                let s: std::string::String = format!("{}", arg0);
                Ok(JArray::from(
                    s.into_bytes().into_iter().map(|b| b as i8).collect::<Vec<i8>>(),
                ))
            }
            "ISO-8859-1" => {
                let val = arg0.__get_value().to_vec();
                let units: Vec<u16> = if arg0.__get_coder() == 0i8 {
                    val.iter().map(|b| *b as u8 as u16).collect()
                } else {
                    (0..val.len() / 2)
                        .map(|i| {
                            let b0 = val[i * 2] as u8;
                            let b1 = val[i * 2 + 1] as u8;
                            if cfg!(target_endian = "big") {
                                u16::from_be_bytes([b0, b1])
                            } else {
                                u16::from_le_bytes([b0, b1])
                            }
                        })
                        .collect()
                };
                let mut bytes: Vec<i8> = Vec::with_capacity(units.len());
                for u in units {
                    if u > 0xFF {
                        panic!(
                            "stub: java/nio/charset/CharacterCodingException (getBytesNoRepl ISO-8859-1 unmappable U+{:04X})",
                            u
                        );
                    }
                    bytes.push(u as u8 as i8);
                }
                Ok(JArray::from(bytes))
            }
            _ => panic!(
                "stub: jdk/internal/access/JavaLangAccess.getBytesNoRepl:(Ljava/lang/String;Ljava/nio/charset/Charset;)[B (charset {} 未消费)",
                name
            ),
        }
    }

    /// `newStringNoRepl(byte[], Charset)`：JDK 转发 `StringCoding.newStringNoRepl`
    /// （REPORT 动作解码，错码抛 CharacterCodingException）。语料消费面为
    /// Latin-1 / UTF-8 两族：Latin-1 逐字节为 char（紧凑 LATIN1 coder，无错码
    /// 面）；UTF-8 严格解码（错码路径以 stub 文本报错——CharacterCodingException
    /// 不强制入闭包，K-2 编译面约束，语料无误码输入）。其余 charset 未消费。
    fn newStringNoRepl(&self, arg0: JArray<i8>, arg1: Charset) -> Result<String> {
        let name = if arg1.is_jvm_null() {
            std::string::String::new()
        } else {
            format!("{}", arg1.__get_name())
        };
        match name.as_str() {
            "ISO-8859-1" => {
                let mut inst = String::default();
                inst._init_not_null();
                inst.__set_value(Clone::clone(&arg0));
                inst.__set_coder(0i8);
                Ok(inst)
            }
            "UTF-8" => {
                let mut raw: Vec<u8> = Vec::new();
                for i in 0..arg0.len()? {
                    raw.push(arg0.get(i)? as u8);
                }
                match std::str::from_utf8(&raw) {
                    Ok(text) => Ok(String::from_owned(text.to_owned())),
                    Err(e) => panic!("stub: java/nio/charset/CharacterCodingException (newStringNoRepl UTF-8 malformed at {})", e.valid_up_to()),
                }
            }
            _ => panic!("stub: jdk/internal/access/JavaLangAccess.newStringNoRepl:([BLjava/nio/charset/Charset;)Ljava/lang/String; (charset {} 未消费)", name),
        }
    }

    /// `getEnumConstantsShared(Class<E>)E[]`：枚举宇宙从运行时常量目录重建
    /// （`java_class!` 宏在类初始化后按「自身类型 static 字段」形态登记，枚举
    /// 常量即该形态，登记序 == 声明序 == ordinal 序）。入参 Class 的名字为点分
    /// binary name，与目录键一致。空 / 未命中返回 null（JDK 对非枚举类同此）。
    fn getEnumConstantsShared(&self, arg0: Class) -> Result<JArray<Enum<Object>>> {
        let cls_name = format!("{}", arg0.__get_name());
        // JVM 反射路径语义：读常量宇宙前强制目标类初始化（常量目录在
        // `<clinit>` 之后登记，未初始化则宇宙必然为空——见 lib.rs 钩子表）
        crate::ensure_class_initialized(&cls_name)?;
        match constant_directory_universe(&cls_name) {
            None => Ok(JArray::default()),
            Some(elems) => Ok(JArray::from(
                elems.into_iter().map(Enum::<Object>::from).collect::<Vec<Enum<Object>>>()
            )),
        }
    }

    /// `join(String prefix, String suffix, String delimiter, String[] elements, int size)`：
    /// JDK 中转发到 `String.join` 包私有指定例程（prefix + el0 + delim + ... + el_{size-1}
    /// + suffix，仅取 elements 前 size 个）。BFS 在本接口截断看不见该边，生成侧
    /// `String.join(String,String,String,String[],int)` 不在链上，故按其字节码算法
    /// 在此还原：coder = 各部 coder 按位或，长度以 char 计后 `<< coder` 折算字节，
    /// 分段写入（Latin1 段写入 UTF16 目标时按 getBytes 展宽），溢出抛
    /// OutOfMemoryError（与 JDK 一致）。
    fn join(&self, prefix: String, suffix: String, delimiter: String,
            elements: JArray<String>, size: i32) -> Result<String> {
        let prefix_val = prefix.__get_value().to_vec();
        let suffix_val = suffix.__get_value().to_vec();
        let delim_val = delimiter.__get_value().to_vec();
        let mut icoder: i8 = prefix.__get_coder() | suffix.__get_coder();
        // char 长度（Latin1：1 字节 1 char；UTF16：2 字节 1 char）
        let char_len = |v: &[i8], c: i8| -> i64 {
            (v.len() as i64) >> (c as u32 & 1)
        };
        let mut len: i64 = char_len(&prefix_val, prefix.__get_coder())
            + char_len(&suffix_val, suffix.__get_coder());
        if size > 1 {
            // 多于一个元素时发射 size - 1 个分隔符
            len += (size as i64 - 1) * char_len(&delim_val, delimiter.__get_coder());
            icoder |= delimiter.__get_coder();
        }
        let mut elem_vals: Vec<(Vec<i8>, i8)> = Vec::new();
        for i in 0..size {
            let el = elements.get(i)?;
            let v = el.__get_value().to_vec();
            len += char_len(&v, el.__get_coder());
            icoder |= el.__get_coder();
            elem_vals.push((v, el.__get_coder()));
        }
        let coder: i8 = icoder;
        // long 溢出与 int 溢出双重检查（JDK：len < 0 || (len <<= coder) != (int) len）
        if len < 0 {
            return Err(JvmError::out_of_memory(
                "Requested string length exceeds VM limit"));
        }
        let byte_len = len << (coder as u32 & 1);
        if byte_len != (byte_len as i32 as i64) {
            return Err(JvmError::out_of_memory(
                "Requested string length exceeds VM limit"));
        }
        let mut value: Vec<i8> = Vec::with_capacity(byte_len as usize);
        _get_bytes_into(&prefix_val, prefix.__get_coder(), &mut value, coder);
        if size > 0 {
            let (v0, c0) = &elem_vals[0];
            _get_bytes_into(v0, *c0, &mut value, coder);
            for i in 1..size {
                _get_bytes_into(&delim_val, delimiter.__get_coder(), &mut value, coder);
                let (v, c) = &elem_vals[i as usize];
                _get_bytes_into(v, *c, &mut value, coder);
            }
        }
        _get_bytes_into(&suffix_val, suffix.__get_coder(), &mut value, coder);
        let mut inst = String::default();
        inst._init_not_null();
        inst.__set_value(JArray::from(value));
        inst.__set_coder(coder);
        Ok(inst)
    }
}

impl ObjectVTable for SystemJavaLangAccess {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "java/lang/System$JavaLangAccess" }
    fn __obj_str(&self) -> std::string::String {
        "java.lang.System$JavaLangAccess".to_owned()
    }
    /// 接口视图查询（invokeinterface 的运行时入口）：调用方 slot 是
    /// `Option<Rc<dyn JavaLangAccess__VTable>>` 时填入自身（与 java_class! 宏为
    /// `impl Iface for Class` 生成的形态一致）。
    fn __interface(self: Rc<Self>, slot: &mut dyn std::any::Any) {
        if let Some(s) = slot.downcast_mut::<Option<Rc<dyn JavaLangAccess__VTable>>>() {
            *s = Some(self);
        }
    }
}
