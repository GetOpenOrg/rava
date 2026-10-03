//! @CallerSensitive 注入调用器的 VM 侧（方案 `docs/plans/2026-10-02-c1d-reflect-narrow.md`
//! §3.3「CallerSensitive 方法句柄路径」）。
//!
//! JDK 的 `MethodHandleImpl$BindCaller.makeInjectedInvoker(H)` 以 ASM 模板为调用者类 H 定义隐藏
//! 巢成员类 `H$$InjectedInvoker/0x…`（超类 Object，加载器与包同 H），其静态方法
//! `invoke_V` / `reflect_invoke_V` 转调目标句柄，CS 方法因此以该注入类为调用者。原生二进制无
//! 运行期类定义：
//! - `define`（类定义点的 VM 承载）按宿主登记注入类名，同一宿主恒为同一类；
//! - 注入类的类元数据面：隐藏类（`meta::is_hidden_class`）、定义加载器同宿主
//!   （`meta::class_defining_loader`）、超类 Object（`Class.getSuperclass`）、巢主按名即宿主的巢；
//! - 成员面：`MethodHandleNatives.resolve` 取 VM 支持类 `InjectedInvokerDyn` 的同名声明
//!   （`method_meta`）；`reflect_invoke` 把对注入类的调用转到支持类，并以注入类为
//!   @CallerSensitive 调用者压栈（`invoke`）。

use crate::error::Result;
use crate::java::lang::Object;
use crate::JArray;

/// VM 支持类 binary name（模板方法的字节码翻译实现）。
pub const TEMPLATE: &str = "java/lang/invoke/InjectedInvokerDyn";

/// 注入类名的隐藏类后缀基址（与 lambda 隐藏类的序号段错开，只作不透明标识）。
const SUFFIX_BASE: u64 = 0x0000_0070_0000_0000;

crate::__process_static! {
    /// 宿主（斜线 binary name）→ 注入类名；注入类名 → 宿主。
    static REGISTRY: crate::sync_model::__RefSlot<(
        std::collections::HashMap<std::string::String, &'static str>,
        std::collections::HashMap<&'static str, std::string::String>,
    )> = crate::sync_model::__RefSlot::new((std::collections::HashMap::new(), std::collections::HashMap::new()));
}

/// 注入类名的 JDK 规则：`H.getName() + "$$InjectedInvoker"`，H 为隐藏类时其名中的 `/` 换 `_`，
/// 再整体 `.` → `/`；隐藏类定义追加 `/0x…` 后缀。
fn injected_name(host_slash: &str, seq: usize) -> std::string::String {
    let mut name = format!("{}$$InjectedInvoker", crate::meta::java_name(host_slash));
    if crate::meta::is_hidden_class(host_slash) {
        name = name.replace('/', "_");
    }
    format!("{}/0x{:016x}", name.replace('.', "/"), SUFFIX_BASE + seq as u64)
}

/// `makeInjectedInvoker(H)` 的类定义：登记（或取已登记的）H 的注入类，返回其斜线名。
/// 命名读元数据面（`meta::java_name` / `is_hidden_class` 会查本登记表），须在写锁外完成。
pub fn define(host_slash: &str) -> &'static str {
    static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    if let Some(n) = REGISTRY.with(|r| r.borrow().0.get(host_slash).copied()) {
        return n;
    }
    let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let fresh = injected_name(host_slash, seq);
    REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        if let Some(&n) = r.0.get(host_slash) {
            return n;
        }
        let n: &'static str = Box::leak(fresh.into_boxed_str());
        r.0.insert(host_slash.to_owned(), n);
        r.1.insert(n, host_slash.to_owned());
        n
    })
}

/// 注入类的宿主（非注入类 → None）。
pub fn host_of(class_slash: &str) -> Option<std::string::String> {
    REGISTRY.with(|r| r.borrow().1.get(class_slash).cloned())
}

/// 注入类的登记名（`&'static`，作调用者压栈；非注入类 → None）。
fn registered(class_slash: &str) -> Option<&'static str> {
    REGISTRY.with(|r| r.borrow().1.get_key_value(class_slash).map(|(k, _)| *k))
}

/// 是否为已定义的注入类。
pub fn is_injected(class_slash: &str) -> bool {
    registered(class_slash).is_some()
}

/// MethodHandleNatives.resolve 的方法面：注入类上的方法取支持类的同名同描述符声明。
pub fn method_meta(owner: &str, name: &str, descriptor: &str) -> Option<(i32, bool, bool, bool)> {
    if !is_injected(owner) {
        return None;
    }
    crate::meta::class_methods().iter()
        .find(|(n, _)| *n == TEMPLATE)
        .and_then(|(_, ms)| ms.iter().find(|m| m.name == name && m.descriptor == descriptor))
        .map(|m| (m.modifiers, m.is_static, m.is_native, m.is_abstract))
}

/// reflect_invoke 前置：对注入类的调用 → 支持类的同名方法，以注入类为 @CallerSensitive 调用者。
pub fn invoke(declaring: &str, name: &str, descriptor: &str, recv: &Object, args: &JArray<Object>) -> Option<Result<Object>> {
    let caller = registered(declaring)?;
    Some(crate::reflect_dispatch::__caller_sensitive(caller, || {
        crate::reflect_dispatch::reflect_invoke(TEMPLATE, name, descriptor, Clone::clone(recv), args)
    }))
}
