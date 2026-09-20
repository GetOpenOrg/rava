use crate::prelude::*;
use super::AccessController;
use super::PrivilegedAction__VTable;

// 内部边界类 java.security.AccessController（java/security/ 属 VM 安全服务层，BFS 截断）。
// 按 java.util.Arrays$LegacyMergeSort 调用链按需实现 doPrivileged(PrivilegedAction)，
// 其余方法保持 panic 存根。

impl AccessController {
    /// `doPrivileged(PrivilegedAction<T>)T`：直呼 action 的 `run()` 并返回其结果。
    ///
    /// 原生单线程二进制没有安全管制（JDK 21 的 SecurityManager 恒为 null、
    /// AccessControlContext 从不安装），「特权提升」无从谈起——所有代码本就全权限，
    /// 与 HotSpot 在未安装 SM 时 executePrivileged 的直通行为一致。
    ///
    /// action 是已擦除的 `Object`：经 `__interface` 分派（等价 JVM itable 查找）
    /// 取 `PrivilegedAction__VTable` 视图后调用擦除签名的 `run()`。
    /// action 为 null 时按 JVM 语义抛 NullPointerException。
    ///
    /// upcalls：经擦除 vtable 分派调用 action.run()。upcall 声明是静态的，
    /// 动态接收者无法表达——按当前调用图唯一的 PrivilegedAction 实现者声明
    /// （GetBooleanAction.run，其手写体 __impl_run 再声明自己的依赖）。
    #[jvm_boundary(upcalls = "sun/security/action/GetBooleanAction.run:()Ljava/lang/Boolean;")]
    pub fn doPrivileged_privilegedaction(action: Object) -> Result<Object> {
        if action.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let mut slot: Option<Rc<dyn PrivilegedAction__VTable>> = None;
        ObjectVTable::__interface(Rc::clone(&action.0), &mut slot);
        match slot {
            Some(vt) => <dyn PrivilegedAction__VTable>::run(&*vt),
            None => panic!(
                "stub: java/security/AccessController.doPrivileged:(Ljava/security/PrivilegedAction;)Ljava/lang/Object; \
                 (receiver 未实现 PrivilegedAction)"
            ),
        }
    }
}
