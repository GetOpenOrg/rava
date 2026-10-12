//! `java/lang/reflect/Constructor` 手写伴生：L3 分派键（NativeAccessor.newInstance0 按键经
//! reflect_invoke 构造，FS-R R2）。newInstance 与注解查询族已回到 JDK 字节码（FS-R R2 / R4b）。
//! 挂载键：(clazz, 描述符)——描述符参数段从 parameterTypes 还原。
//!
//! 分派键免重建与 Method 伴生（method_impl.rs 文件头）同构：表构造的 Constructor 携带
//! (clazz, slot)（`__table_declared_ctors` 构造，slot 是声明类方法表全行序的行号；
//! 字节码侧唯一构造点 `Constructor.copy()` 原样转传），首用经 [`__slot_row`] 直取
//! `<init>` 行的静态描述符，按 (clazz 身份, slot) 进程级缓存；键只依赖声明二元组，与
//! setAccessible 等访问标志无涉。缓存键的回收论证与驻留池口径见 method_impl.rs 文件头。

use crate::prelude::*;
use super::method_impl::__slot_row;
use super::Constructor;
use crate::java::lang::Class;
use crate::java::lang::Object;

crate::__process_static! {
    /// (声明类镜像身份, slot) → 构造键 (类名斜线形态, 完整描述符——返回段 V)。
    /// None = 已判定非表形态（恒走重建路径）。
    static CTOR_KEYS: crate::sync_model::__RefSlot<
        std::collections::HashMap<(usize, i32), Option<(&'static str, &'static str)>>>
        = crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
    /// 非表形态重建键的驻留池（同一键一份；见 method_impl.rs 文件头回收论证）。
    static REBUILT_CTOR_KEYS: crate::sync_model::__RefSlot<
        std::collections::HashMap<(std::string::String, std::string::String), (&'static str, &'static str)>>
        = crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
}

/// (clazz, slot) 的首用解析：行直取，行须为 `<init>`（构造器身份核对；表构造恒成立）。
fn __resolve_key(clazz: &Class, slot: i32) -> Option<(&'static str, &'static str)> {
    let (cls, meta) = __slot_row(clazz, slot)?;
    if meta.name != "<init>" {
        return None;
    }
    Some((cls, meta.descriptor))
}

/// Class 名（点/斜线形态均可）→ 描述符形态（method_impl 同一归一规则）。
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

/// 非表形态的键重建（原实现逐字保留：parameterTypes 还原描述符，返回段 V），结果驻留。
fn __rebuild_key<T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe>(
    c: &Constructor<T>) -> Option<(&'static str, &'static str)> {
    let cls_key = format!("{}", c.__get_clazz().__get_name()).replace('.', "/");
    let params = c.__get_parameterTypes();
    let mut desc = std::string::String::from("(");
    for i in 0..params.len().ok()? {
        let p = params.get(i).ok()?;
        desc.push_str(&__type_desc(&format!("{}", p.__get_name())));
    }
    desc.push_str(")V");
    REBUILT_CTOR_KEYS.with(|pool| {
        let mut g = pool.borrow_mut();
        let entry = g.entry((Clone::clone(&cls_key), Clone::clone(&desc)));
        match entry {
            std::collections::hash_map::Entry::Occupied(e) => Some(*e.get()),
            std::collections::hash_map::Entry::Vacant(e) => {
                let v: (&'static str, &'static str) =
                    (Box::leak(cls_key.into_boxed_str()), Box::leak(desc.into_boxed_str()));
                e.insert(v);
                Some(v)
            }
        }
    })
}

/// 接收者的构造器挂载键（声明类斜线名, 完整描述符——返回段 V）。
fn __member_key<T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe>(
    c: &Constructor<T>) -> Option<(&'static str, &'static str)> {
    let clazz = c.__get_clazz();
    let obj = Object::from(Clone::clone(&clazz));
    if obj.0.is_jvm_null() {
        return None;
    }
    let slot = c.__get_slot();
    let id = obj.0.__identity() as usize;
    // 快路径：已解析 / 已判定非表形态的 (类, slot)——静态引用直取，零分配
    if let Some(hit) = CTOR_KEYS.with(|cache| cache.borrow().get(&(id, slot)).copied()) {
        return match hit {
            Some(k) => Some(k),
            None => __rebuild_key(c),
        };
    }
    let resolved = __resolve_key(&clazz, slot);
    CTOR_KEYS.with(|cache| { cache.borrow_mut().insert((id, slot), resolved); });
    match resolved {
        Some(k) => Some(k),
        None => __rebuild_key(c),
    }
}

impl<T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe> Constructor<T> {
    /// 反射族内部：本构造器的 L3 分派键 (声明类斜线名, 描述符)。两分量均为 `&'static`
    /// （表构造：元数据表行；非表形态：驻留池），调用方零分配消费。
    pub(crate) fn __reflect_key(&self) -> Option<(&'static str, &'static str)> {
        __member_key(self)
    }
}
