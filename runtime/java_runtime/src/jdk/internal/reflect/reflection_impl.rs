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
    /// 帧源为 std::backtrace 的真实 Rust 栈（栈回溯数据面，compatibility.md
    /// 「栈回溯」近似等价边界——与 throwable_impl 的 fillInStackTrace 同族；
    /// 逐字节的 Java 帧元数据在原生二进制不存在）。跳帧规则：
    ///   1. 定位本方法帧（声明类 jdk.internal.reflect.Reflection）；
    ///   2. 其后第一帧组 = `@CallerSensitive` 声明者（如 MethodHandles.lookup）
    ///      ——同一 Java 方法的多个 Rust 帧（含解析不出类的辅助帧）按类名
    ///      聚合，整组跳过；
    ///   3. 首个声明类不同的帧即调用者的调用者，返回其 Class。
    /// 帧不可解析（匿名类 / PrivilegedAction 转发帧等符号形态不符，或平台符号化不全）时
    /// 退回 `java.lang.Object`：JDK 中仅 JNI 附着线程无 Java 调用者，Java 代码调用恒有；
    /// 原生单镜像内全部代码同属可信调用者（getModule 同一无名模块），退回可信类与 JDK
    /// 默认安装的可观测行为一致——`ServiceLoader.checkCaller` 不再误报 "no caller to check"
    /// （Charset.forName 未知名的扩展 provider 查找，macOS 实测揭出）。
    #[jvm_native]
    pub fn getCallerClass() -> Result<Class> {
        // 生成器显式传入的调用者（@CallerSensitive 调用点压栈，平台无关）优先
        if let Some(caller) = crate::reflect_dispatch::current_caller_sensitive() {
            return Ok(Class::for_class(String::from(caller)));
        }
        const SELF_CLASS: &str = "jdk/internal/reflect/Reflection";
        let fallback = || Class::for_class(String::from("java/lang/Object"));
        let frames = crate::vm_stack::capture_frame_classes();
        // 1. 定位本方法帧
        let Some(mut i) = frames.iter().position(|c| c.as_deref() == Some(SELF_CLASS)) else {
            return Ok(fallback());
        };
        // 2. 调用者帧组的类（@CallerSensitive 声明者）
        i += 1;
        let Some(caller) = frames.get(i).and_then(|c| c.clone()) else {
            return Ok(fallback());
        };
        // 3. 跳过同类（与解析失败跟随前帧）的帧，取首个异类帧
        i += 1;
        while i < frames.len() {
            match &frames[i] {
                Some(c) if c != &caller => {
                    return Ok(Class::for_class(String::from(c.as_str())));
                }
                _ => i += 1,
            }
        }
        Ok(fallback())
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
