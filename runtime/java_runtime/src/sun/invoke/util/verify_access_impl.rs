//! `sun/invoke/util/VerifyAccess` 手写伴生：内部边界类，按调用链按需实现
//! （K-2 规则），其余保持 panic 存根。
//!
//! 消费链：MethodHandles 的 Lookup 访问检查族（checkSymbolicClass /
//! checkAccess / checkForTypeAlias）。JDK 语义核心是 JPMS 模块导出检查；
//! 原生二进制无模块系统——全部类位于未命名模块（对所有人开放），模块边
//! 界恒穿透；访问规则回落到 JLS §6.6 的修饰位判定（同类 / 同包 / 子类）。

use crate::prelude::*;
use super::verify_access::VerifyAccess;
use crate::java::lang::Class;
use crate::java::lang::invoke::MethodType;

impl VerifyAccess {
    /// static `isClassAccessible(refc, lookupClass, prevLookupClass, allowedModes)`：
    /// 模块导出检查的未命名模块等价物——无模块系统时唯一不可访问形态是
    /// allowedModes == 0（Lookup.revoked 后任何符号类都不可达）。
    pub fn isClassAccessible(_refc: Class, _lookupClass: Class, _prevLookupClass: Class, allowedModes: i32) -> Result<bool> {
        Ok(allowedModes != 0)
    }

    /// static `isTypeVisible(type, refc)`：type 从 refc 处是否可见（JDK 的
    /// 模块导出检查 + 回退类加载器检查）。未命名模块等价物：运行时全部
    /// 类型位于同一个未命名模块（对所有人开放），类型恒可见。
    /// 消费链：MemberName.checkForTypeAlias（resolve 后的类型别名核对）。
    pub fn isTypeVisible_class_class(_type_: Class, _refc: Class) -> Result<bool> {
        Ok(true)
    }

    /// static `isTypeVisible(MethodType type, Class refc)`：JDK21 方法类型形态（返回类型与
    /// 各参数逐个做 Class 形态检查；JDK25 改名 ensureTypeVisible）——各分量恒可见，恒真。
    /// 消费链：MemberName.checkForTypeAlias（Lookup.findStatic 等 resolveOrFail，MH-native 揭出）。
    pub fn isTypeVisible_methodtype_class(_type_: MethodType, _refc: Class) -> Result<bool> {
        Ok(true)
    }

    /// static `isSamePackage(Class, Class)`：同类恒真；否则比较运行时包名（JDK 另比类加载器——
    /// 启动类与用户类的包名空间本就不相交，按包名判定等价）。消费方：
    /// DirectMethodHandle.shouldBeInitialized（静态目标的类初始化屏障判定）。
    pub fn isSamePackage(class1: Class, class2: Class) -> Result<bool> {
        let n1 = format!("{}", class1.__get_name()).replace('/', ".");
        let n2 = format!("{}", class2.__get_name()).replace('/', ".");
        if n1 == n2 {
            return Ok(true);
        }
        let pkg = |n: &str| n.rsplit_once('.').map(|(p, _)| p.to_owned()).unwrap_or_default();
        Ok(pkg(&n1) == pkg(&n2))
    }

    /// static `isSameModule(Class, Class)`：全部类型同属未命名模块 → 恒真。
    pub fn isSameModule(_class1: Class, _class2: Class) -> Result<bool> {
        Ok(true)
    }

    /// static `isModuleAccessible(Class refc, Module m1, Module m2)`：未命名模块对所有人开放 → 恒真。
    pub fn isModuleAccessible(_refc: Class, _m1: crate::java::lang::Module, _m2: crate::java::lang::Module) -> Result<bool> {
        Ok(true)
    }

    /// static `ensureTypeVisible(Class type, Class refc)`：JDK25 对
    /// `isTypeVisible(Class, Class)` 的改名（语义不变）——未命名模块等价物同上，恒可见。
    pub fn ensureTypeVisible_class_class(_type_: Class, _refc: Class) -> Result<bool> {
        Ok(true)
    }

    /// static `ensureTypeVisible(MethodType type, Class refc)`：JDK25 方法类型形态
    /// （逐个参数 / 返回类型做 Class 形态检查）——各分量恒可见，故恒真。
    /// 消费链：MemberName.ensureTypeVisible（MemberName$Factory.resolveOrFail）。
    pub fn ensureTypeVisible_methodtype_class(_type_: MethodType, _refc: Class) -> Result<bool> {
        Ok(true)
    }

    /// static `isMemberAccessible(refc, defc, mods, lookupClass, prevLookupClass,
    /// allowedModes)`：成员访问规则（JLS §6.6 的核心判定，JDK 在模块检查后
    /// 执行）。无模块世界的等价形态（模块项恒同模块即导出）：
    ///   - defc == lookupClass → 恒真（同类私有访问，含嵌套类共享）；
    ///   - public → isClassAccessible（未命名模块恒真）；
    ///   - protected → 同包 或 lookupClass 是 defc 的子类（层次表）；
    ///   - private → lookup 具 PRIVATE 模式且互为 nestmate（同顶层类的巢）；
    ///   - 包私有 → 同包（binary name 前缀）。
    /// prevLookupClass 恒 null（无跨模块 lookup），allowedModes == 0 已吊销。
    pub fn isMemberAccessible(_refc: Class, defc: Class, mods: i32, lookupClass: Class, _prevLookupClass: Class, allowedModes: i32) -> Result<bool> {
        if allowedModes == 0 {
            return Ok(false);
        }
        let defc_name = format!("{}", defc.__get_name());
        let lookup_name = format!("{}", lookupClass.__get_name());
        if defc_name == lookup_name {
            return Ok(true);
        }
        let same_package = defc_name.rsplit_once('.').map(|(p, _)| p)
            == lookup_name.rsplit_once('.').map(|(p, _)| p);
        if mods & 0x0001 != 0 {
            // public 成员：可见性归 isClassAccessible（未命名模块恒真）
            return Ok(true);
        }
        if mods & 0x0004 != 0 {
            // protected：同包 或 子类（defc 的类型闭包含 lookupClass）
            if same_package {
                return Ok(true);
            }
            return defc.isAssignableFrom(Clone::clone(&lookupClass));
        }
        if mods & 0x0002 != 0 {
            // private：lookup 具 PRIVATE 模式且两类互为 nestmate（JDK 11+
            // `Reflection.areNestMates`）。javac 的 NestHost 恒为最外层封闭类，
            // 以 binary name 首个 `$` 前的顶层类名为巢主判定
            let nest_host = |n: &str| n.split('$').next().unwrap_or(n).to_string();
            return Ok(allowedModes & 0x0002 != 0 && nest_host(&defc_name) == nest_host(&lookup_name));
        }
        // 包私有：同包
        Ok(same_package)
    }
}
