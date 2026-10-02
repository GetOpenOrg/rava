//! `jdk/internal/reflect/Reflection` 手写伴生：内部边界类，按调用链按需实现
//! （K-2 规则），其余保持 panic 存根。

use crate::prelude::*;
use super::reflection::Reflection;
use crate::java::lang::Class;
use crate::java::util::Set;
use crate::sync_model::__RefSlot as RefCell;
use std::collections::HashMap;


impl Reflection {
    /// `isCallerSensitive(Method)`：JDK 体为 `m.isAnnotationPresent(CallerSensitive.class)`（系统域加载器
    /// 时）——经完整注解解析器（AnnotationParser + 动态代理）判定，令每个触达 Method.invoke 的闭包
    /// 带上注解解析族。原生二进制全部类在系统域，判定等价于扫描该方法的原始注解字节
    /// （anno_pool::has_annotation：类型索引经声明类的稀疏常量池比对 CallerSensitive 描述符）。
    #[jvm_boundary]
    pub fn isCallerSensitive(m: crate::java::lang::reflect::Method) -> Result<bool> {
        let raw = m.__get_annotations();
        if raw.is_jvm_null() {
            return Ok(false);
        }
        let bytes: Vec<u8> = raw.to_vec().into_iter().map(|b| b as u8).collect();
        let cls = format!("{}", m.__get_clazz().__get_name()).replace('.', "/");
        Ok(crate::anno_pool::has_annotation(&cls, &bytes, "Ljdk/internal/reflect/CallerSensitive;"))
    }



    /// native `getCallerClass()`：`@CallerSensitive`——返回「调用 getCallerClass
    /// 的方法」的调用者声明类（JDK javadoc：ignoring frames associated with
    /// java.lang.reflect.Method.invoke）。
    ///
    /// 生成器在 `@CallerSensitive` 调用点显式压栈的调用者优先（平台无关）；手写层直接调用 CS
    /// 方法时回退栈遍历数据面（`crate::vm_stack`），跳帧同 HotSpot `JVM_GetCallerClass`：
    /// 第 0 帧为本方法（手写 native 帧），第 1 帧为 `@CallerSensitive` 方法，其后首个不被安全栈
    /// 遍历忽略（`is_ignored_by_security_stack_walk`）的帧即调用者。
    /// 帧不可得（平台符号化不全等）时退回 `java.lang.Object`：JDK 中仅 JNI 附着线程无 Java 调用者，
    /// Java 代码调用恒有；原生单镜像内全部代码同属可信调用者（getModule 同一无名模块），退回可信类
    /// 与 JDK 默认安装的可观测行为一致——`ServiceLoader.checkCaller` 不再误报 "no caller to check"
    /// （Charset.forName 未知名的扩展 provider 查找，macOS 实测揭出）。
    #[jvm_native]
    pub fn getCallerClass() -> Result<Class> {
        if let Some(caller) = crate::reflect_dispatch::current_caller_sensitive() {
            return Ok(Class::for_class(String::from(caller)));
        }
        let frames = crate::vm_stack::capture_java_frames();
        let caller = frames
            .iter()
            .position(|f| f.class == "jdk/internal/reflect/Reflection" && f.method.name == "getCallerClass")
            .and_then(|at| frames[at + 1..].iter().skip(1).find(|f| !f.is_ignored_by_security_stack_walk()));
        Ok(Class::for_class(String::from(caller.map_or("java/lang/Object", |f| f.class))))
    }

    /// native `getClassAccessFlags(Class)`：class 文件的类访问标志（非 InnerClasses 的内部标志）。
    /// javac 对嵌套类在 class 文件中的写法：protected → ACC_PUBLIC，private → 包可见，static
    /// 不入类标志——由 Modifier 位集（含 InnerClasses 语义）还原。消费方：verifyMemberAccess 的
    /// 「非 public 类 → 同包判定」（FS-R R2a 字节码路径）。
    #[jvm_native]
    pub fn getClassAccessFlags(c: Class) -> Result<i32> {
        let mods = c.getModifiers()?;
        let public = if mods & (0x0001 | 0x0004) != 0 { 0x0001 } else { 0 };
        // 保留 final / interface / abstract / annotation / enum 等类级位，去掉成员级的 private / protected / static
        Ok((mods & !(0x0001 | 0x0002 | 0x0004 | 0x0008)) | public)
    }

    /// native `areNestMates(Class, Class)`：javac 的 NestHost 恒为最外层封闭类（binary name
    /// 首个 `$` 前），同巢即互为 nestmate（private 成员可达，JDK 11+）。
    #[jvm_native]
    pub fn areNestMates(current: Class, member: Class) -> Result<bool> {
        let host = |c: &Class| {
            let n = format!("{}", c.__get_name()).replace('.', "/");
            n.split('$').next().unwrap_or("").to_owned()
        };
        Ok(host(&current) == host(&member))
    }
}
