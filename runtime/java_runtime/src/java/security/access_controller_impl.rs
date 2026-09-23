use crate::prelude::*;
use super::AccessController;
use super::AccessControlContext;
use super::PrivilegedAction__VTable;

// 内部边界类 java.security.AccessController（java/security/ 属 VM 安全服务层，BFS 截断）。
// 按调用链按需实现（doPrivileged：Arrays$LegacyMergeSort；getContext：Thread.<init>），
// 其余方法保持 panic 存根。

impl AccessController {
    /// `AccessControlContext getContext()`：返回当前访问控制上下文
    /// （`Thread.<init>` 的 acc==null 分支消费）。
    ///
    /// 原生二进制无安全管制（SecurityManager 恒 null、访问控制上下文从不
    /// 安装）：HotSpot 在未安装 SM 时该上下文为空且从不被检查。返回 null
    /// 载体——消费点仅写入 `Thread.inheritedAccessControlContext` 字段，
    /// 读取侧（checkPermission 路径）在 SM==null 下全短路。
    #[jvm_boundary]
    pub fn getContext() -> Result<AccessControlContext> {
        Ok(AccessControlContext::default())
    }
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
    /// upcalls（接口级回调边，已激活）：声明 `java/security/PrivilegedAction.run`
    /// ——upcall 目标在边界接口上，callchain 的 `_pending_iface_edges` 把接口方法键
    /// 入队，`_propagate_virtual_targets` 接口分支自动翻译闭包内全部实现类
    /// （FileSystems$DefaultFileSystemHolder$1 / ZoneRulesProvider$1 两族 stub 消除，
    /// 零实现类枚举）。此前休眠在枚举形态（GetBooleanAction.run 单实现者）：
    /// 激活暴露的 blocks.py unify 三族编译缺口（三目合并退化接收者 E0599 /
    /// 双局部声明载体 E0308 / 基类调用载体实参装箱 E0308）已由 fix/unify-fourth
    /// 清偿——unify_pair 泛型 widening 第五增量、_store_local 绑定点区间判定、
    /// invokespecial 基类调用实参重建。
    #[jvm_boundary(upcalls = "java/security/PrivilegedAction.run:()Ljava/lang/Object;")]
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
