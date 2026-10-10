//! Class 的类级 native：嵌套 / 外围 / record / 访问标志 / 签名者（数据源 = 元数据表）（宿主 class_impl.rs 的私有辅助模块）

use super::*;

// ── FS-R R1：类级 native（数据源 = 元数据表，JDK 公开方法体回到字节码）────────────
//
// HotSpot 从 class 文件的 InnerClasses / EnclosingMethod / Record 属性取值；原生二进制
// 以 java_meta 表承载同一数据（docs/plans/2026-09-27-reflection-metadata-table.md）。
impl Class {
    pub(super) fn __slash_name(&self) -> std::string::String {
        format!("{}", self.__get_name()).replace('.', "/")
    }

    fn __nest(&self) -> Option<&'static crate::meta::NestMeta> {
        let key = self.__slash_name();
        crate::meta::class_nest().iter().find(|(n, _)| *n == key).map(|(_, m)| m)
    }

    /// native `getDeclaringClass0()`：本类 InnerClasses 条目的 outer_class（成员类）；
    /// 顶层 / 局部 / 匿名类（outer 为空）与数组、基本类型 → null。
    #[jvm_native]
    pub fn getDeclaringClass0(&self) -> Result<Class> {
        Ok(match self.__nest() {
            Some(m) if m.self_entry && !m.outer.is_empty() => Class::for_class(String::from(m.outer)),
            _ => Class::default(),
        })
    }

    /// native `getSimpleBinaryName0()`：本类 InnerClasses 条目的 inner_name（匿名类为 null）；
    /// 无本类条目（顶层类）→ null。
    #[jvm_native]
    pub fn getSimpleBinaryName0(&self) -> Result<String> {
        Ok(match self.__nest() {
            Some(m) if m.self_entry && !m.simple.is_empty() => String::from(m.simple),
            _ => String::default(),
        })
    }

    /// native `getEnclosingMethod0()`：EnclosingMethod 属性 → `{封闭类, 方法名, 描述符}`
    /// （方法名 / 描述符在类初始化器或字段初始化器内声明时为 null）；无属性 → null。
    #[jvm_native]
    pub fn getEnclosingMethod0(&self) -> Result<JArray<Object>> {
        let Some((cls, name, desc)) = self.__nest().and_then(|m| m.enclosing) else {
            return Ok(JArray::default());
        };
        let opt = |s: &str| if s.is_empty() { Object::default() } else { Object::from(String::from(s)) };
        Ok(JArray::from(vec![
            Object::from(Class::for_class(String::from(cls))),
            opt(name),
            opt(desc),
        ]))
    }

    /// native `isRecord0()`：Record 属性在场（record 类集表）。
    #[jvm_native]
    pub fn isRecord0(&self) -> Result<bool> {
        let key = self.__slash_name();
        Ok(!key.starts_with('[') && crate::meta::record_classes().contains(&key.as_str()))
    }

    /// static native `desiredAssertionStatus0(Class)`：断言恒关（`-ea` 缺省，JVM 同）。
    #[jvm_native]
    pub fn desiredAssertionStatus0(_c: Class) -> Result<bool> {
        Ok(false)
    }

    /// native `isHidden()`：隐藏类判定，读 java_meta 隐藏类表（lambda 调用点隐藏类由生成器按站点
    /// 声明，元数据与其余类同表）；数组 / 基本类型 / 普通类 → false。
    #[jvm_native]
    pub fn isHidden(&self) -> Result<bool> {
        Ok(crate::meta::is_hidden_class(&self.__slash_name()))
    }

    /// native `getNestHost0()`：javac 的 NestHost 恒为最外层封闭类（binary name 首个 `$` 前）。
    #[jvm_native]
    pub fn getNestHost0(&self) -> Result<Class> {
        let key = self.__slash_name();
        if key.starts_with('[') || !key.contains('/') && !key.contains('$') {
            return Ok(Clone::clone(self));
        }
        Ok(Class::for_class(String::from(key.split('$').next().unwrap_or(&key))))
    }

    /// native `initClassName()`：镜像名在 for_class 建镜像时写入，直接返回。
    #[jvm_native]
    pub fn initClassName(&self) -> Result<String> {
        Ok(self.__get_name())
    }
}

// ── 嵌套成员 / 访问标志 / 签名者 native（数据源 = 元数据表；HotSpot 读同一 class 文件属性）────
impl Class {
    /// 实例类（非数组、非基本类型）的斜线名；数组 / 基本类型 → None。
    fn __instance_klass_name(&self) -> Result<Option<std::string::String>> {
        let key = self.__slash_name();
        Ok(if key.starts_with('[') || self.isPrimitive()? { None } else { Some(key) })
    }

    /// native `getDeclaredClasses0()`：InnerClasses 中 outer 为本类、inner 非本类的条目（属性序），
    /// HotSpot `JVM_GetDeclaredClasses` 同源；数组 / 基本类型 → 空数组（同 HotSpot）。
    #[jvm_native]
    pub fn getDeclaredClasses0(&self) -> Result<JArray<Class>> {
        if self.__instance_klass_name()?.is_none() {
            return Ok(JArray::from(Vec::<Class>::new()));
        }
        let members: &[&str] = self.__nest().map(|m| m.members).unwrap_or(&[]);
        Ok(JArray::from(members.iter().map(|n| Class::for_class(String::from(*n))).collect::<Vec<Class>>()))
    }

    /// native `getNestMembers0()`：嵌套宿主在首位，其后为宿主 NestMembers 属性所列成员（声明序），
    /// HotSpot `JVM_GetNestMembers` 同形；非宿主类先取其宿主（getNestHost0）再列宿主的成员；
    /// 数组 / 基本类型 → 仅自身（`Class.getNestMembers` 在 Java 侧已对其短路，此处与 VM 同解）。
    #[jvm_native]
    pub fn getNestMembers0(&self) -> Result<JArray<Class>> {
        if self.__instance_klass_name()?.is_none() {
            return Ok(JArray::from(vec![Clone::clone(self)]));
        }
        let host = self.getNestHost0()?;
        let host_name = host.__slash_name();
        let mut out = vec![Clone::clone(&host)];
        if let Some((_, list)) = crate::meta::nest_members().iter().find(|(n, _)| *n == host_name) {
            out.extend(list.iter().map(|n| Class::for_class(String::from(*n))));
        }
        Ok(JArray::from(out))
    }

    /// native `getClassAccessFlagsRaw0()`：类文件 access_flags 原值（含 ACC_SUPER / ACC_SYNTHETIC，
    /// 不含 InnerClasses 条目的修饰符），HotSpot `JVM_GetClassAccessFlags`：基本类型 →
    /// `ACC_ABSTRACT | ACC_FINAL | ACC_PUBLIC`（0x411）；数组类 → 0（JDK 21 实测同值）。
    #[jvm_native]
    pub fn getClassAccessFlagsRaw0(&self) -> Result<i32> {
        if self.isPrimitive()? {
            return Ok(0x0411);
        }
        let key = self.__slash_name();
        Ok(crate::meta::class_access_flags().iter().find(|(n, _)| *n == key).map_or(0, |(_, f)| *f))
    }

    /// native `setSigners(Object[])`：记录类的签名者（HotSpot `JVM_SetClassSigners` 写镜像注入字段
    /// `signers`；基本类型类不记录）。镜像按类名唯一（for_class 缓存），以类名为键的进程表承载该注入
    /// 状态；读取方为同文件的 `getSigners`。
    #[jvm_native]
    pub fn setSigners(&self, signers: JArray<Object>) -> Result<()> {
        if !self.isPrimitive()? {
            // 键在锁外求出；被覆盖的旧签名者数组在锁外释放
            let key = self.__slash_name();
            drop(_signers_table(|t| t.insert(key, signers)));
        }
        Ok(())
    }
}

/// 类镜像注入字段 `signers` 的承载表（类名 → 签名者数组）。
pub(super) fn _signers_table<R>(f: impl FnOnce(&mut HashMap<std::string::String, JArray<Object>>) -> R) -> R {
    crate::__process_static! {
        static SIGNERS: RefCell<HashMap<std::string::String, JArray<Object>>> = RefCell::new(HashMap::new());
    }
    SIGNERS.with(|t| f(&mut t.borrow_mut()))
}
