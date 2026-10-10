//! VM 支持类 `java/lang/reflect/Proxy$Dyn`（动态代理通用载体，FS-R R4a）的 VM 钩子。
//!
//! 宏据本文件提供的 `__vm_proxy_invoke` 识别「代理载体」：
//! - `ObjectVTable::is_instance_of` 另按实例接口列表应答（`__vm_proxy_implements`）；
//! - 接口载体分派 vtable 未命中时经 `__proxy_invoke` 转入 `__vm_proxy_invoke`：按
//!   (声明接口, 方法名, 描述符) 取接口方法的 Method 对象（元数据表直构，进程级缓存——
//!   JDK 生成类以 static final 字段持有，同一方法同一对象），交由字节码翻译的
//!   `Proxy$Dyn.dispatch` 转发 InvocationHandler。无参方法的 args 为 null（JDK 同）。
//! - 基本类型实参按生成体的装箱形态（`Integer.valueOf` 等）重装箱：接口载体侧只有原生值盒，
//!   InvocationHandler 对实参做 `(Integer) args[i]` 须得到翻译出的包装类实例。

use crate::prelude::*;
use super::Proxy_Dyn;
use crate::java::lang::{Boolean, Byte, Character, Class, Double, Float, Integer, Long, Short};
use crate::reflect_dispatch as rd;
use crate::java::lang::reflect::Method;

crate::__process_static! {
    /// (接口, 方法名, 描述符) → Method 对象。
    static PROXY_METHODS: RefCell<std::collections::HashMap<std::string::String, Method>> =
        RefCell::new(std::collections::HashMap::new());
}

fn proxy_method(iface: &str, name: &str, desc: &str) -> Result<Method> {
    let key = format!("{}.{}{}", iface, name, desc);
    if let Some(m) = PROXY_METHODS.with(|c| c.borrow().get(&key).cloned()) {
        return Ok(m);
    }
    let m = Class::for_class(String::from(iface)).__table_method(name, desc)?;
    Ok(PROXY_METHODS.with(|c| c.intern(key, m)))
}

/// 描述符形参的首字符序列（基本类型字符；引用 / 数组 → 'L'）。
fn param_kinds(desc: &str) -> Vec<u8> {
    let b = desc.as_bytes();
    let (mut out, mut i) = (Vec::new(), 1usize);
    while i < b.len() && b[i] != b')' {
        match b[i] {
            b'L' => {
                while b[i] != b';' { i += 1; }
                out.push(b'L');
            }
            b'[' => {
                while b[i] == b'[' { i += 1; }
                if b[i] == b'L' { while b[i] != b';' { i += 1; } }
                out.push(b'L');
            }
            c => out.push(c),
        }
        i += 1;
    }
    out
}

/// 基本类型实参（原生值盒）→ 生成体装箱形态（包装类 valueOf，含缓存池身份语义）。
fn rebox(kind: u8, v: Object) -> Result<Object> {
    let bad = || JvmError::illegal_argument("proxy argument type mismatch");
    Ok(match kind {
        b'I' => Object::from(Integer::valueOf_i(rd::unbox_i32(&v).ok_or_else(bad)?)?),
        b'J' => Object::from(Long::valueOf_l(rd::unbox_i64(&v).ok_or_else(bad)?)?),
        b'Z' => Object::from(Boolean::valueOf_z(rd::unbox_bool(&v).ok_or_else(bad)?)?),
        b'C' => Object::from(Character::valueOf(rd::unbox_char(&v).ok_or_else(bad)?)?),
        b'B' => Object::from(Byte::valueOf_b(rd::unbox_i32(&v).ok_or_else(bad)? as i8)?),
        b'S' => Object::from(Short::valueOf_s(rd::unbox_i32(&v).ok_or_else(bad)? as i16)?),
        b'F' => Object::from(Float::valueOf_f(rd::unbox_f32(&v).ok_or_else(bad)?)?),
        b'D' => Object::from(Double::valueOf_d(rd::unbox_f64(&v).ok_or_else(bad)?)?),
        _ => v,
    })
}

impl Proxy_Dyn {
    /// instanceof / checkcast：实例接口列表中任一接口（含其超接口）可赋值给 `type_id`。
    pub fn __vm_proxy_implements(&self, type_id: &str) -> bool {
        let intfs = self.__get_intfs();
        let Ok(n) = intfs.len() else { return false };
        (0..n).any(|i| match intfs.get(i) {
            Ok(c) => {
                let name = format!("{}", c.__get_name()).replace('.', "/");
                Class::__name_assignable(type_id, &name)
            }
            Err(_) => false,
        })
    }

    /// 接口方法调用（接口载体分派回退点）：转入 `dispatch(Method, Object[])`。VM 钩子（非 Java 成员，
    /// 类 2：动态代理由 VM 支持类 Proxy$Dyn 承载，登记见 vm_intrinsics.toml Proxy.newProxyInstance）。
    pub fn __vm_proxy_invoke(&self, iface: &str, name: &str, desc: &str, args: Vec<Object>) -> Result<Object> {
        let m = proxy_method(iface, name, desc)?;
        let kinds = param_kinds(desc);
        let mut boxed = Vec::with_capacity(args.len());
        for (i, a) in args.into_iter().enumerate() {
            boxed.push(rebox(kinds.get(i).copied().unwrap_or(b'L'), a)?);
        }
        let arr: JArray<Object> = if boxed.is_empty() { JArray::default() } else { JArray::from(boxed) };
        self.dispatch(m, arr)
    }
}
