//! 纯数据资源束的结构判定（seeds.toml `[data_bundle] carriers`）。
//!
//! 载体类的子类若只声明构造器 / 类初始化器 / 载体方法，且载体方法体只含常量装载与数组构造
//! （无方法调用），即「纯数据类」——即便位于边界前缀内也按字节码翻译（数据不是实现细节）。

use std::collections::HashMap;

use classfile::ClassFile;
use resolve::ClassPath;

/// 载体方法体允许的操作码：常量装载、数组构造与元素存储、局部变量存取、返回
const DATA_OPS: &[u8] = &[
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // aconst_null / iconst_m1..5
    0x10, 0x11, 0x12, 0x13, 0x14, // bipush / sipush / ldc / ldc_w / ldc2_w
    0x19, 0x2a, 0x2b, 0x2c, 0x2d, // aload / aload_0..3
    0x3a, 0x4b, 0x4c, 0x4d, 0x4e, // astore / astore_0..3
    0x53, 0x59, 0xb0, 0xbd, // aastore / dup / areturn / anewarray
];
/// 构造器 / 类初始化器另允许：调用父类构造器、return
const CTOR_OPS: &[u8] = &[0xb7, 0xb1];

/// 载体类 → (方法名, 描述符)
#[derive(Debug, Default)]
pub struct Carriers(HashMap<String, (String, String)>);

impl Carriers {
    /// 条目形如 `类.方法:描述符`
    pub fn new(entries: &[String]) -> Self {
        let mut m = HashMap::new();
        for e in entries {
            let Some((head, desc)) = e.split_once(':') else { continue };
            let Some((cls, name)) = head.rsplit_once('.') else { continue };
            m.insert(cls.to_string(), (name.to_string(), desc.to_string()));
        }
        Carriers(m)
    }

    /// 超类链上的数据载体方法
    pub fn carrier_of(&self, cp: &ClassPath, cf: &ClassFile) -> Option<(String, String)> {
        let mut cur = cf.super_name.clone();
        let mut guard = 0;
        while let Some(c) = cur {
            if let Some(x) = self.0.get(&c) {
                return Some(x.clone());
            }
            guard += 1;
            if guard > 64 {
                return None;
            }
            cur = cp.get(&c)?.super_name.clone();
        }
        None
    }

    pub fn is_pure_data_bundle(&self, cp: &ClassPath, cf: &ClassFile) -> bool {
        if cf.is_interface() || self.0.is_empty() {
            return false;
        }
        let Some((name, desc)) = self.carrier_of(cp, cf) else { return false };
        let ops_ok = |m: &classfile::Method, extra: &[u8]| {
            m.code.as_ref().is_some_and(|c| c.insns.iter().all(|i| DATA_OPS.contains(&i.opcode) || extra.contains(&i.opcode)))
        };
        let mut has_carrier = false;
        for m in &cf.methods {
            if m.is_init() || m.is_clinit() {
                if !ops_ok(m, CTOR_OPS) {
                    return false;
                }
            } else if m.name == name && m.desc == desc && !m.is_static() {
                if !ops_ok(m, &[]) {
                    return false;
                }
                has_carrier = true;
            } else {
                return false;
            }
        }
        has_carrier
    }
}
