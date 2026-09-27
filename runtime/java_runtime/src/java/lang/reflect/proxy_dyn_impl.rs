//! VM 支持类 `java/lang/reflect/Proxy$Dyn`（动态代理通用载体，FS-R R4a）的 VM 钩子。
//!
//! 宏据本文件提供的 `__vm_proxy_invoke` 识别「代理载体」：
//! - `ObjectVTable::is_instance_of` 另按实例接口列表应答（`__vm_proxy_implements`）；
//! - 接口载体分派 vtable 未命中时经 `__proxy_invoke` 转入 `__vm_proxy_invoke`：按
//!   (声明接口, 方法名, 描述符) 取接口方法的 Method 对象（元数据表直构，进程级缓存——
//!   JDK 生成类以 static final 字段持有，同一方法同一对象），交由字节码翻译的
//!   `Proxy$Dyn.dispatch` 转发 InvocationHandler。无参方法的 args 为 null（JDK 同）。

use crate::prelude::*;
use super::Proxy_Dyn;
use crate::java::lang::Class;
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
    PROXY_METHODS.with(|c| { c.borrow_mut().insert(key, Clone::clone(&m)); });
    Ok(m)
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

    /// 接口方法调用（接口载体分派回退点）：转入 `dispatch(Method, Object[])`。
    #[jvm_boundary(upcalls = "java/lang/reflect/Proxy$Dyn.dispatch:(Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;")]
    pub fn __vm_proxy_invoke(&self, iface: &str, name: &str, desc: &str, args: Vec<Object>) -> Result<Object> {
        let m = proxy_method(iface, name, desc)?;
        let arr: JArray<Object> = if args.is_empty() { JArray::default() } else { JArray::from(args) };
        self.dispatch(m, arr)
    }
}
