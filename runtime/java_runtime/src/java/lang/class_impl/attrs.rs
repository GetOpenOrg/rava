//! Class 的 class 文件属性 native：注解 / 签名 / 许可子类 / 接口 / record 组件 / 保护域 / 签名者（宿主 class_impl.rs 的私有辅助模块）

use super::*;

impl Class {
    /// native `getRawAnnotations()`：类级 RuntimeVisibleAnnotations 原始属性体（FS-R R4b，
    /// HotSpot 同源：class 文件属性字节）；无注解 → null。消费方：Class.createAnnotationData →
    /// AnnotationParser.parseAnnotations（字节码翻译）。
    #[jvm_native]
    pub fn getRawAnnotations(&self) -> Result<JArray<i8>> {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        Ok(__anno_bytes(crate::anno_pool::class_annotations(&cls_key)))
    }

    /// native `getRawTypeAnnotations()`：类型注解（RuntimeVisibleTypeAnnotations）不携带 → null。
    #[jvm_native]
    pub fn getRawTypeAnnotations(&self) -> Result<JArray<i8>> {
        Ok(JArray::default())
    }

    /// native `getConstantPool()`：本类常量池视图（HotSpot 返回持 constantPoolOop 的
    /// ConstantPool）。原生二进制的常量池是注解属性引用的稀疏表，以本 Class 作为
    /// constantPoolOop，ConstantPool natives 按类名查表（constant_pool_impl.rs）。
    #[jvm_native]
    pub fn getConstantPool(&self) -> Result<crate::jdk::internal::reflect::ConstantPool> {
        let cp = crate::jdk::internal::reflect::ConstantPool::new()?;
        cp.__set_constantPoolOop(Object::from(Clone::clone(self)));
        Ok(cp)
    }


    /// `Class.isRecord()`：record 类判定（JVMS §4.7.30 Record 属性在场；
    /// 发射侧 is_record 属性 → java_meta record 表）。数组 / 基本类型类恒 false。
    /// native `Class.isInstance(Object)`：null → false；否则按运行时类的
    /// is_instance_of（vtable 按 binary name 斜线形态应答，含超类与接口闭包）。
    /// 基本类型类恒 false（JLS：无值是基本类型 Class 的实例）。
    #[jvm_native]
    pub fn isInstance(&self, obj: Object) -> Result<bool> {
        if obj.0.is_jvm_null() || self.isPrimitive()? {
            return Ok(false);
        }
        let key = format!("{}", self.__get_name()).replace('.', "/");
        // JLS §4.10：任意引用（含数组，其 is_instance_of 有意不按 Object 匹配）都是 Object 实例
        if key == "java/lang/Object" {
            return Ok(true);
        }
        Ok(obj.0.is_instance_of(&key))
    }



    /// native `getInterfaces0()`：直接超接口（class 文件 interfaces 项，声明序）。数组类 →
    /// Cloneable / Serializable（JLS §10.8）；基本类型类 / 无接口 → 空数组。
    #[jvm_native]
    /// native `getGenericSignature0()`：类的泛型签名（Signature 属性，JVM_GetClassSignature 同形）；
    /// 无该属性 / 数组 / 基本类型 → null。数据源为 java_meta 的 CLASS_SIGNATURE 表，
    /// 解析由 sun/reflect/generics（ClassRepository）的字节码翻译承担。
    #[jvm_native]
    pub fn getGenericSignature0(&self) -> Result<String> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        Ok(match crate::meta::class_signature(&name) {
            Some(sig) => String::from(sig),
            None => String::default(),
        })
    }

    /// native `getPermittedSubclasses0()`：sealed 类 / 接口的许可子类型（PermittedSubclasses 属性，
    /// 声明序）；非 sealed → null（JVM_GetPermittedSubclasses 同形）。数据源为 java_meta 的
    /// PERMITTED_SUBCLASSES 表；许可子类型不在生成闭包内时跳过（JVM 对无法加载的条目同样跳过）。
    #[jvm_native]
    pub fn getPermittedSubclasses0(&self) -> Result<JArray<Class>> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        let Some((_, subs)) = crate::meta::permitted_subclasses().iter().find(|(n, _)| *n == name) else {
            return Ok(JArray::default());
        };
        let out: Vec<Class> = subs.iter()
            .filter(|s| Class::__is_known_class(s))
            .map(|s| Class::for_class(String::from(*s)))
            .collect();
        Ok(JArray::from(out))
    }

    /// 类文件是否声明了 `<clinit>`（java_meta 的 CLINIT_CLASSES 表；VM 注入的类信息）。
    /// 供 `ObjectStreamClass.hasStaticInitializer`（默认 serialVersionUID 计算）。
    #[doc(hidden)]
    pub fn __has_static_initializer(&self) -> bool {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        crate::meta::clinit_classes().contains(&name.as_str())
    }

    pub fn getInterfaces0(&self) -> Result<JArray<Class>> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        let list: Vec<&str> = if name.starts_with('[') {
            vec!["java/lang/Cloneable", "java/io/Serializable"]
        } else if self.isPrimitive()? {
            Vec::new()
        } else {
            crate::meta::class_interfaces().iter()
                .find(|(n, _)| *n == name)
                .map(|(_, l)| l.to_vec())
                .unwrap_or_default()
        };
        let out: Vec<Class> = list.into_iter()
            .map(|i| Class::for_class(String::from(i)))
            .collect();
        Ok(JArray::from(out))
    }

    /// native `Class.isInterface()`：接口（含注解类型）判定，读 java_meta 修饰符表的
    /// INTERFACE 位（与 getModifiers 同源）。数组类 / 基本类型类 → false
    /// （JLS：数组类型与基本类型都不是接口）；闭包外类（表中缺席）→ false。
    /// 消费方：ObjectStreamClass 构造链（Result.<clinit> 的序列化元数据查询）。
    #[jvm_native]
    pub fn isInterface(&self) -> Result<bool> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        if name.starts_with('[') || self.isPrimitive()? {
            return Ok(false);
        }
        Ok(crate::meta::class_modifiers().iter()
            .find(|(n, _)| *n == name)
            .map(|(_, m)| (*m & 0x0200) != 0)
            .unwrap_or(false))
    }


    /// native `Class.forName0(name, initialize, loader, caller)`：按 binary name 取 Class 对象。
    /// 原生镜像的「可加载类」= 生成闭包内的类（java_meta 修饰符表，含用户类）；数组名
    /// （`[I` / `[Ljava.lang.String;`）直接构造。未知类名 → `ClassNotFoundException(name)`
    /// （JDK 同消息）。initialize=true（`Class.forName(String)` 的缺省）立即执行类初始化
    /// （JLS §12.4.1 / FS-C5）：经 main 启动时登记的类初始化钩子（有 `<clinit>` 的用户类）
    /// 按名代调 `__class_init`，初始化异常按状态机语义传播（ExceptionInInitializerError /
    /// 其后 NoClassDefFoundError）；initialize=false 只取 Class 对象（init-passive）。
    #[jvm_native]
    pub fn forName0(name: String, initialize: bool, _loader: crate::java::lang::ClassLoader,
                    _caller: Class) -> Result<Class> {
        let dotted = format!("{}", name);
        let slash = dotted.replace('.', "/");
        let known = Class::__is_known_class(&slash);
        if !known {
            let ex = crate::java::lang::ClassNotFoundException::new_str(String::from(dotted.as_str()))?;
            return Err(ex.into());
        }
        if initialize && !slash.starts_with('[') {
            crate::ensure_class_initialized(&slash)?;
        }
        Ok(Class::for_class(String::from(slash.as_str())))
    }

    /// 原生镜像中「可加载」的类：生成闭包内的类（java_meta 修饰符表，含用户类）与数组类名；
    /// 隐藏类不可按名加载（JVM 同：`Class.forName` 对隐藏类名抛 ClassNotFoundException）。
    /// 供 `forName0` 与 `ClassLoader.findBootstrapClass` 共用。
    #[doc(hidden)]
    pub fn __is_known_class(slash_name: &str) -> bool {
        slash_name.starts_with('[')
            || !crate::meta::is_hidden_class(slash_name)
                && crate::meta::class_modifiers().iter().any(|(n, _)| *n == slash_name)
    }

    /// native `Class.getRecordComponents0()`：record 分量反射（声明序）。数据源是
    /// java_class! 块的 `record_components` 属性（classfile Record 属性：名字 /
    /// 描述符 / 泛型签名），java_meta 汇总为 RECORD_COMPONENTS 表。查询即构造
    /// RecordComponent：clazz=本类、type=描述符还原、accessor=同名无参声明方法、
    /// signature=泛型签名（无则 null）。非 record（表中缺席）→ null（JDK 语义）。
    /// 消费方：ObjectStreamClass 的 record 序列化（规范构造器 / 分量取值）。
    /// RecordComponent 由此处构造（无 `new` 指令可见，闭包分析按手写体的构造调用记为已实例化，
    /// toString 等覆盖经 Object 视图可达）；访问器经 getDeclaredMethod 查询。
    #[jvm_native]
    pub fn getRecordComponents0(&self) -> Result<JArray<crate::java::lang::reflect::RecordComponent>> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        let comps: &[(&str, &str, &str)] = match crate::meta::record_components().iter().find(|(c, _)| *c == name) {
            Some((_, comps)) => comps,
            // 无分量的 record（`record Empty()`）不产生 record_components 属性 → 空数组
            None if crate::meta::record_classes().contains(&name.as_str()) => &[],
            None => return Ok(JArray::default()),
        };
        let mut out: Vec<crate::java::lang::reflect::RecordComponent> = Vec::new();
        for (n, d, g) in comps.iter() {
            let mut rc = crate::java::lang::reflect::RecordComponent::default();
            rc._init_not_null();
            rc.__set_clazz(Clone::clone(self));
            rc.__set_name(String::from(*n));
            rc.__set_type_(class_for_descriptor(d));
            rc.__set_accessor(self.__table_method_noargs(n)?);
            rc.__set_signature(__signature(*g));
            out.push(rc);
        }
        Ok(JArray::from(out))
    }



    /// native `getProtectionDomain0()`：HotSpot `JVM_GetProtectionDomain` 返回定义类时 defineClass 传入、
    /// 写进镜像的保护域（VM 注入状态，准入 ③）。引导类、数组与基本类型为 null（调用方回落
    /// `Class$Holder.allPermDomain`，JDK 同）。应用类加载器定义的类：原生二进制无运行期类定义，按 JDK
    /// 内建加载器定义类的路径落地——`BuiltinClassLoader.defineClass(cn, Resource)` 以类路径条目 URL 建
    /// 无签名者的 CodeSource，经 `SecureClassLoader.getProtectionDomain(cs)`（按代码源缓存，同源类共享
    /// 同一保护域）取域；类路径条目即应用加载器 ucp 的首条目（initPhase3 按字节码建立，空
    /// `java.class.path` 为当前目录，与 JDK 同口径）。其余加载器定义的类为 null（同引导类回落）。
    #[jvm_native]
    pub fn getProtectionDomain0(&self) -> Result<crate::java::security::ProtectionDomain> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        if crate::meta::class_defining_loader(&name) != Some("app") {
            return Ok(Default::default());
        }
        let loader: crate::java::lang::ClassLoader = crate::jdk::internal::loader::ClassLoaders::appClassLoader()?;
        let builtin: crate::jdk::internal::loader::BuiltinClassLoader =
            <crate::jdk::internal::loader::BuiltinClassLoader as ::std::convert::From<Object>>::from(Object::from(Clone::clone(&loader)));
        let ucp: crate::jdk::internal::loader::URLClassPath = builtin.__get_ucp();
        let urls: JArray<crate::java::net::URL> = ucp.getURLs()?;
        let url: crate::java::net::URL = if urls.len()? > 0 { urls.get(0)? } else { Default::default() };
        let cs = crate::java::security::CodeSource::new_url_arr_codesigner(url, Default::default())?;
        let secure: crate::java::security::SecureClassLoader =
            <crate::java::security::SecureClassLoader as ::std::convert::From<Object>>::from(Object::from(loader));
        secure.getProtectionDomain(cs)
    }

    /// native `getSigners()`：HotSpot `JVM_GetClassSigners`——基本类型或未经 setSigners 记录 → null
    /// （原生二进制无 jar 签名者，未签名类的 JDK 返回值）；已记录 → 返回其 Object[] 副本。
    #[jvm_native]
    pub fn getSigners(&self) -> Result<JArray<Object>> {
        if self.isPrimitive()? {
            return Ok(Default::default());
        }
        let key = self.__slash_name();
        Ok(match _signers_table(|t| t.get(&key).cloned()) {
            Some(a) if !a.is_jvm_null() => JArray::from(a.to_vec()),
            _ => Default::default(),
        })
    }
}
