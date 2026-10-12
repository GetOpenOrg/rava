//! `java/lang/reflect/Method` 手写伴生：L3 分派键（NativeAccessor.invoke0 按键经
//! reflect_invoke 调用，FS-R R2）。注解查询族已回到 JDK 字节码（FS-R R4b：
//! Method.annotations 字节 → AnnotationParser）。
//!
//! ## 分派键免重建（2026-10-12，invoke 热路径第一热点）
//!
//! 表构造的 Method 在构造时即携带 (clazz, slot)——slot 是声明类方法元数据表的行号
//! （`Class.__method_from_meta` / `__table_method` / record 组件访问器 / Proxy$Dyn
//! 接口方法直构一律如此；字节码侧唯一构造点 `Method.copy()` 原样转传 slot）。分派键
//! 据此零重建：
//!
//!   (clazz 身份, slot) --首用--> 类方法表行 → rows[slot] 的静态 name / descriptor
//!
//! 三分量都是 `&'static`（与发射层分派闭包键、`is_static_descriptor` 同一真源——
//! parameterTypes / returnType 本就从该描述符还原，直取即恒等往返）。首用结果按
//! (clazz 身份, slot) 进程级缓存，此后每次 invoke 只做一次哈希取静态引用：无
//! Java String→Rust 转换、无分配（原路径每次 invoke 3–4 次转换 + 约 5 次分配，
//! rt-perf-profiling §二 String 簇 ~17% 的主体）。
//!
//! - 键只依赖声明三元组（类 / 名 / 描述符，均 final 字段承载），与 setAccessible 等
//!   访问标志无涉——标志变化不影响缓存正确性。
//! - 缓存键的回收论证：类镜像是进程级规范对象（`Class::for_class` 的 CLASSES 驻留表 /
//!   引导映像 `&'static` / `getPrimitiveClass` 的 PRIMITIVES 驻留表永久持有），身份
//!   不存在回收复用；条目数 ≤ 被 invoke 的不同声明数（编译期有界）。
//! - 首用解析以 Method 的名字段核对行内容（表构造恒一致；不一致 = 非表形态，判定为
//!   恒走重建路径并缓存该判定）。重建路径（parameterTypes / returnType 还原描述符）
//!   语义与原实现逐字相同，结果驻留为 &'static——条目数 ≤ 不同的非表声明键数，
//!   现实语料全部构造点都是表构造，本池为空。
//! - 键缓存哈希器（2026-10-12，ikopt2）：invoke 段剖析显示缺省 RandomState+Sip13
//!   （`hash_one` + `Sip13::write`）占 ≈2.4%——METHOD_KEYS 换 FxHash 风格轻量乘法
//!   散列（[`FxKeyHasher`]）。碰撞安全性：键是 (类镜像指针身份, slot) 的内部值、非
//!   攻击者可控；HashMap 查找命中哈希后必做键相等核对，碰撞的代价只是一次 12 字节
//!   核对、绝无错答；条目数 ≤ 被 invoke 的不同声明数（编译期有界，量级数百），64 位
//!   乘法散列下碰撞概率可忽略（n²/2⁶⁵）。指针低位对齐零熵由 slot 分量的旋转异或
//!   补足。冷路径 REBUILT_KEYS（驻留池，现实为空）维持 RandomState 不动。

use crate::java::lang::Class__Methods as _;
use crate::java::lang::reflect::Method__Methods as _;
use crate::prelude::*;
use super::Method;
use crate::java::lang::Class;
use crate::java::lang::Object;

// ── 键缓存的轻量哈希器（FxHash 风格，文件头注记）─────────────────────────────────

/// (usize, i32) 小键的乘法散列：每分量「旋转 5 位异或后乘固定奇常数」——无 SipHash 的
/// 多轮压缩函数，热路径一次查键只做两次乘法 + 异或。逐字节 `write` 兜底仅为正确性
/// （本键型不触达：usize 走 `write_usize`、i32 走 `write_i32`）。
#[derive(Default)]
struct FxKeyHasher { hash: u64 }

/// FxHash 的固定乘数（rustc FxHasher 同款黄金比例常数）
const FX_KEY_SEED: u64 = 0x517c_c1b7_2722_0a95;

impl FxKeyHasher {
    #[inline]
    fn add(&mut self, w: u64) {
        self.hash = (self.hash.rotate_left(5) ^ w).wrapping_mul(FX_KEY_SEED);
    }
}

impl std::hash::Hasher for FxKeyHasher {
    #[inline]
    fn finish(&self) -> u64 { self.hash }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.add(b as u64);
        }
    }
    #[inline] fn write_u8(&mut self, i: u8)    { self.add(i as u64) }
    #[inline] fn write_u16(&mut self, i: u16)  { self.add(i as u64) }
    #[inline] fn write_u32(&mut self, i: u32)  { self.add(i as u64) }
    #[inline] fn write_u64(&mut self, i: u64)  { self.add(i) }
    #[inline] fn write_usize(&mut self, i: usize) { self.add(i as u64) }
    #[inline] fn write_i8(&mut self, i: i8)    { self.write_u8(i as u8) }
    #[inline] fn write_i16(&mut self, i: i16)  { self.write_u16(i as u16) }
    #[inline] fn write_i32(&mut self, i: i32)  { self.write_u32(i as u32) }
    #[inline] fn write_i64(&mut self, i: i64)  { self.write_u64(i as u64) }
    #[inline] fn write_isize(&mut self, i: isize) { self.write_usize(i as usize) }
}

/// [`FxKeyHasher`] 的 BuildHasher（空类型，无 RandomState 的每实例密钥初始化）
#[derive(Clone, Copy, Default)]
struct FxKeyBuild;

impl std::hash::BuildHasher for FxKeyBuild {
    type Hasher = FxKeyHasher;
    #[inline]
    fn build_hasher(&self) -> FxKeyHasher { FxKeyHasher::default() }
}

crate::__process_static! {
    /// (声明类镜像身份, slot) → 分派键 (类名斜线形态, 方法名, 完整描述符)。
    /// None = 已判定非表形态（恒走重建路径）。见文件头免重建注记（哈希器见其 ikopt2 段）。
    static METHOD_KEYS: crate::sync_model::__RefSlot<
        std::collections::HashMap<(usize, i32), Option<(&'static str, &'static str, &'static str)>, FxKeyBuild>>
        = crate::sync_model::__RefSlot::new(
            std::collections::HashMap::with_hasher(FxKeyBuild));
    /// 非表形态重建键的驻留池（同一键只驻留一份；见文件头回收论证）。
    static REBUILT_KEYS: crate::sync_model::__RefSlot<
        std::collections::HashMap<(std::string::String, std::string::String, std::string::String),
            (&'static str, &'static str, &'static str)>>
        = crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
}

/// 声明类方法表的行直取（Method / Constructor 伴生共用）：二分定位类行（与
/// [`crate::meta::row`] 同口径，另带回静态类名），`slot` 即行下标。表构造的成员
/// 对象 slot 由构造点写入行号；表外类 / 行号越界 → None（回落重建路径）。
pub(super) fn __slot_row(clazz: &Class, slot: i32)
    -> Option<(&'static str, &'static crate::meta::MethodMeta)>
{
    let cls_key = format!("{}", clazz.__get_name()).replace('.', "/");
    let table = crate::meta::class_methods();
    let i = table.partition_point(|(k, _)| *k < cls_key.as_str());
    match table.get(i) {
        Some(&(k, rows)) if k == cls_key => {
            let idx = usize::try_from(slot).ok()?;
            let meta = rows.get(idx)?;
            Some((k, meta))
        }
        _ => None,
    }
}

/// (clazz, slot) 的首用解析：行直取 + 名字段核对（表构造恒一致；不一致判非表形态）。
fn __resolve_key(clazz: &Class, slot: i32, name: &crate::java::lang::String)
    -> Option<(&'static str, &'static str, &'static str)>
{
    let (cls, meta) = __slot_row(clazz, slot)?;
    if format!("{}", name) != meta.name {
        return None;
    }
    Some((cls, meta.name, meta.descriptor))
}

/// Class 名（点/斜线形态均可）→ 描述符形态（与 class_impl 的 getDeclaredMethod
/// 参数归一同规则；对返回类型同样适用——void 的 Class 缺席形态 → "V"）。
fn __type_desc(class_name: &str) -> std::string::String {
    let slashes = class_name.replace('.', "/");
    match slashes.as_str() {
        "boolean" => "Z".into(),
        "byte" => "B".into(),
        "char" => "C".into(),
        "short" => "S".into(),
        "int" => "I".into(),
        "long" => "J".into(),
        "float" => "F".into(),
        "double" => "D".into(),
        "void" => "V".into(),
        _ if slashes.starts_with('[') => slashes,
        _ => format!("L{};", slashes),
    }
}

/// 非表形态的键重建（原实现逐字保留：parameterTypes / returnType 还原描述符），
/// 结果驻留为 &'static（同一键一份）。parameterTypes 缺席 → None（注解面为空，
/// 与原路径一致——调用方按「无声明键」处理）。
fn __rebuild_key(m: &Method) -> Option<(&'static str, &'static str, &'static str)> {
    let cls_key = format!("{}", m.__get_clazz().__get_name()).replace('.', "/");
    let name = format!("{}", m.__get_name());
    let params = m.__get_parameterTypes();
    let mut desc = std::string::String::from("(");
    for i in 0..params.len().ok()? {
        let p = params.get(i).ok()?;
        desc.push_str(&__type_desc(&format!("{}", p.__get_name())));
    }
    desc.push(')');
    let ret = m.__get_returnType();
    let ret_name = if Object::from(Clone::clone(&ret)).0.is_jvm_null() {
        std::string::String::from("void")
    } else {
        format!("{}", ret.__get_name())
    };
    desc.push_str(&__type_desc(&ret_name));
    Some(__intern_key(cls_key, name, desc))
}

/// 重建键驻留：命中即回既有静态引用，未命中驻留一份（见文件头回收论证）。
fn __intern_key(cls: std::string::String, name: std::string::String, desc: std::string::String)
    -> (&'static str, &'static str, &'static str)
{
    REBUILT_KEYS.with(|pool| {
        let mut g = pool.borrow_mut();
        let entry = g.entry((Clone::clone(&cls), Clone::clone(&name), Clone::clone(&desc)));
        match entry {
            std::collections::hash_map::Entry::Occupied(e) => *e.get(),
            std::collections::hash_map::Entry::Vacant(e) => {
                let v: (&'static str, &'static str, &'static str) = (
                    Box::leak(cls.into_boxed_str()), Box::leak(name.into_boxed_str()),
                    Box::leak(desc.into_boxed_str()));
                e.insert(v);
                v
            }
        }
    })
}

/// 接收者的方法挂载键（声明类斜线名, 方法名, 完整描述符）——与 build.rs
/// 方法注解表的键协议一致（含返回类型的完整描述符；返回段经 returnType
/// 还原，void 的 Class 缺席形态 → V）。
/// parameterTypes 缺席（非表构造的 Method）→ None（注解面为空）。
fn __member_key(m: &Method) -> Option<(&'static str, &'static str, &'static str)> {
    let clazz = m.__get_clazz();
    let obj = Object::from(Clone::clone(&clazz));
    if obj.0.is_jvm_null() {
        return None;
    }
    let slot = m.__get_slot();
    let id = obj.0.__identity() as usize;
    // 快路径：已解析 / 已判定非表形态的 (类, slot)——静态引用直取，零分配
    if let Some(hit) = METHOD_KEYS.with(|c| c.borrow().get(&(id, slot)).copied()) {
        return match hit {
            Some(k) => Some(k),
            None => __rebuild_key(m),
        };
    }
    let resolved = __resolve_key(&clazz, slot, &m.__get_name());
    METHOD_KEYS.with(|c| { c.borrow_mut().insert((id, slot), resolved); });
    match resolved {
        Some(k) => Some(k),
        None => __rebuild_key(m),
    }
}

impl Method {
    /// 反射族内部：本方法的 L3 分派键 (声明类斜线名, 方法名, 完整描述符)。三分量均为
    /// `&'static`（表构造：元数据表行；非表形态：驻留池），调用方零分配消费。
    pub(crate) fn __reflect_key(&self) -> Option<(&'static str, &'static str, &'static str)> {
        __member_key(self)
    }
}
