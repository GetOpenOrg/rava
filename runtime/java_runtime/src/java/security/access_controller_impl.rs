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
    /// upcalls：经擦除 vtable 分派调用 action.run()。upcall 声明是静态的，
    /// 动态接收者无法表达——按当前调用图的 PrivilegedAction 实现者声明
    /// （GetBooleanAction.run，其手写体 __impl_run 再声明自己的依赖）。
    ///
    /// 注：接口级回调边（声明 `java/security/PrivilegedAction.run:()Ljava/lang/Object;`，
    /// BFS 接口分派自动翻译闭包内全部实现类）已在 callchain.py 落地并验证
    /// （FileSystems$DefaultFileSystemHolder$1 / ZoneRulesProvider$1 两例 stub 消除），
    /// 但激活会使含休眠序列化闭包的测试（StreamBasic/PatternMatch 等，其闭包内
    /// ObjectStreamClass.<init> 的 doPrivileged 调用点绑定匿名实现者）翻译反射
    /// 巨闭包，暴露 blocks.py unify 域（三目合并退化接收者 E0599）等三族编译缺口
    /// ——禁改域，待其清偿后把声明翻转为接口级（一行）。
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
