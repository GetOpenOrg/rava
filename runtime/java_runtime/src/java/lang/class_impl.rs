use crate::prelude::*;
use super::*;
use super::reflect::Field;
use crate::sync_model::__RefSlot as RefCell;
use std::collections::HashMap;

impl Class {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// `getModule()`：类所属模块。原生二进制无模块系统（vm_boundary：
    /// java/lang/Module 整体手写）——JDK 类全部落在 java.base，用户类落在
    /// 未命名模块；消费面（Files.writeString 的调用方模块一致性检查等）只做
    /// 相等比较，单一单例即可承载（JDK 类侧与 JVM 行为一致：java.base 类
    /// 同模块恒真）。模块名/层级的完整语义不在档 A 面内。
    #[jvm_boundary]
    pub fn getModule(&self) -> Result<Module> {
        crate::__process_static! {
            static THE_MODULE: RefCell<Option<Module>> = const { RefCell::new(None) };
        }
        Ok(THE_MODULE.with(|cell| {
            if cell.borrow().is_none() {
                let mut m = Module::default();
                m._init_not_null();
                *cell.borrow_mut() = Some(m);
            }
            Clone::clone(cell.borrow().as_ref().unwrap())
        }))
    }

    /// native getPrimitiveClass(String)：每个基本类型名对应唯一的 Class 对象
    /// （`Integer.TYPE == int.class` 的身份语义），首次请求时创建。
    #[jvm_native]
    pub fn getPrimitiveClass(name: String) -> Result<Class> {
        crate::__process_static! {
            static PRIMITIVES: RefCell<HashMap<std::string::String, Class>> = RefCell::new(HashMap::new());
        }
        let key = format!("{}", name);
        Ok(PRIMITIVES.with(|cache| {
            Clone::clone(cache.borrow_mut().entry(key).or_insert_with(|| {
                let mut c = Class::default();
                c._init_not_null();
                c.__set_name(Clone::clone(&name));
                c
            }))
        }))
    }

    /// `ldc` 装载的类字面量（`X.class` / `X[].class`）：每个 binary name 对应唯一
    /// 的 Class 对象，首次请求时创建并在线程内缓存 —— 保证 `X.class == X.class`
    /// 的身份语义（JVMS §5.1 运行时常量池的类引用只解析一次）。
    ///
    /// `getName()` 返回 Java 形式的二进制名：斜线换点（`java/util/List` →
    /// `java.util.List`），数组类型保持 JVM 描述符形态（`[Ljava.lang.String;`）。
    /// isAssignableFrom 的层次查询在运行时经 build.rs 生成的层次表进行，
    /// 此处不再携带/登记超类型数据。
    pub fn for_class(binary_name: String) -> Class {
        crate::__process_static! {
            static CLASSES: RefCell<HashMap<std::string::String, Class>> =
                RefCell::new(HashMap::new());
        }
        let key = format!("{}", binary_name);
        if let Some(c) = CLASSES.with(|cache| cache.borrow().get(&key).cloned()) {
            return c;
        }
        let mut c = Class::default();
        c._init_not_null();
        c.__set_name(String::from(key.replace('/', ".").as_str()));
        // 数组类的 componentType 字段由 VM 在建镜像时填充（HotSpot set_component_mirror）：
        // 字节码翻译的 `componentType()` / `arrayType` 链直接读该字段（MethodHandleImpl
        // .makeCollector 的 nCopies(n, arrayType.componentType())）。元素 Class 经 for_class
        // 解析——须在缓存借用之外（递归入本函数）
        if key.starts_with('[') {
            if let Ok(comp) = c.__impl_getComponentType() {
                c.__set_componentType(comp);
            }
        }
        CLASSES.with(|cache| Clone::clone(cache.borrow_mut().entry(key).or_insert(c)))
    }


    /// native `Class.isArray()`：数组类判定。数组类的名字是 JVM 描述符形态
    /// （`[I`、`[Ljava.lang.String;`——for_class 的存储形态），首字符 `[`
    /// 即数组（JLS：数组的运行时类是 JVM 创建的 Array 类型）。
    /// 消费方：MethodHandles 链的 checkSymbolicClass / findVarHandle 类型检查。
    #[jvm_native]
    pub fn isArray(&self) -> Result<bool> {
        Ok(format!("{}", self.__get_name()).starts_with('['))
    }

    /// native `Class.isPrimitive()`：基本类型类判定。基本类型的 Class 经
    /// getPrimitiveClass 创建，名字是基本类型字面量（int / boolean / …，
    /// 无包前缀）；按显式名单判定（八种基本类型 + void，JDK `void.class.
    /// isPrimitive()` 为 true），非基本类型（含数组）→ false。
    /// 消费方：VarHandles.makeFieldHandle 的字段类型分派链。
    #[jvm_native]
    pub fn isPrimitive(&self) -> Result<bool> {
        let name = format!("{}", self.__get_name());
        Ok(matches!(name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double" | "void"))
    }

    /// native `Class.getComponentType()`：数组类返回元素 Class，非数组返回 null。
    ///
    /// 数组类的名字是 JVM 描述符形态（`[I`、`[Ljava.lang.String;`、`[[I`——
    /// for_class 的存储形态，内层点分隔）。解析：剥一层 `[` 后按首字符分派
    /// ——`L` → 引用类、`[` → 元素仍是数组（描述符形态递归）、基本类型字符 →
    /// getPrimitiveClass 的唯一实例（`int.class` 身份语义）。非数组（闭包外类
    /// / 接口 / 基本类型自身）→ null（JLS：componentType 只对数组类非 null）。
    /// 消费方：`Arrays.copyOf(orig, len, newType)` 链（`Array.newInstance(
    /// newType.getComponentType(), n)`，TestCollectionFactory 的 toArray 卡点）、
    /// MethodHandles 的数组访问器构造。
    pub fn __impl_getComponentType(&self) -> Result<Class> {
        let name = format!("{}", self.__get_name());
        let Some(rest) = name.strip_prefix('[') else {
            return Ok(Class::default());
        };
        Ok(class_for_descriptor(rest))
    }

    /// 反射族内部：按字段名查本类声明元数据（描述符 / static 标志 / 修饰位 /
    /// ConstantValue 整数值）。getDeclaredField 与 Field.get/set（reflect 邻域
    /// 伴生 field_impl.rs）共用同一张 build.rs 字段表；未声明 → None。
    pub fn __declared_field_meta(&self, name: &str) -> Option<(&'static str, bool, i32, Option<i64>)> {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        __fields::CLASS_FIELDS.iter()
            .find(|(n, _)| *n == cls_key)
            .and_then(|(_, fs)| fs.iter().find(|f| f.name == name))
            .map(|f| (f.descriptor, f.is_static, f.modifiers, f.constant))
    }

    /// 反射族内部：按 (name, descriptor) 二元组查本类声明方法元数据（修饰位 /
    /// static / native / abstract）。方法重载使 name 不唯一，命中判定必须
    /// 名与描述符配对（JDK getDeclaredMethod 语义——参数类型还原成描述符后
    /// 比对）。消费方：MethodHandleNatives.resolve 的方法/构造器 kind
    /// （method_handle_natives_impl.rs，MemberName 解析内核）；未声明 → None。
    pub fn __declared_method_meta(&self, name: &str, descriptor: &str) -> Option<(i32, bool, bool, bool)> {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        __methods::CLASS_METHODS.iter()
            .find(|(n, _)| *n == cls_key)
            .and_then(|(_, ms)| ms.iter().find(|m| m.name == name && m.descriptor == descriptor))
            .map(|m| (m.modifiers, m.is_static, m.is_native, m.is_abstract))
    }

    /// 反射族内部：本类声明方法行（构造器/类初始化器行含在内，消费方按
    /// JDK 语义过滤）。消费方：getDeclaredMethod / getDeclaredMethods。
    fn __declared_method_rows(&self) -> &'static [__methods::MethodMeta] {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        __methods::CLASS_METHODS.iter()
            .find(|(n, _)| *n == cls_key)
            .map(|(_, ms)| *ms)
            .unwrap_or(&[])
    }


    /// `getDeclaredMethods()`：本类全部声明方法的构造序列（声明序；JDK 语义
    /// 不含构造器与类初始化器——`<init>`/`<clinit>` 行过滤）。
    pub(crate) fn __table_declared_methods(&self) -> Result<JArray<crate::java::lang::reflect::Method>> {
        // VM 创建反射对象前类已初始化（HotSpot Reflection::new_method 同）：超类 AccessibleObject 的
        // <clinit> 先于 ReflectionFactory 运行（登记 ReflectAccess、取 soleInstance）
        crate::java::lang::reflect::Method::__class_init()?;
        let mut out: Vec<crate::java::lang::reflect::Method> = Vec::new();
        for (slot, meta) in self.__declared_method_rows().iter().enumerate() {
            if meta.name == "<init>" || meta.name == "<clinit>" {
                continue;
            }
            out.push(Self::__method_from_meta(Clone::clone(self), meta, slot as i32));
        }
        Ok(JArray::from(out))
    }

    /// 方法元数据行 → Method（查询即构造）。parameterTypes / returnType 从
    /// 描述符还原（class_for_descriptor 的数组形态：`[...` 直接 for_class），
    /// exceptionTypes 从 throws 子句列表还原。
    fn __method_from_meta(clazz: Class, meta: &'static __methods::MethodMeta, slot: i32) -> crate::java::lang::reflect::Method {
        let params: Vec<Class> = descriptor_params(meta.descriptor)
            .into_iter()
            .map(|p| class_for_descriptor(&p))
            .collect();
        let ret_start = meta.descriptor.find(')').map(|i| i + 1).unwrap_or(meta.descriptor.len());
        let ret = class_for_descriptor(&meta.descriptor[ret_start..]);
        let excs: Vec<Class> = meta.exceptions.iter()
            .map(|e| Class::for_class(String::from(*e)))
            .collect();
        let mut m = crate::java::lang::reflect::Method::default();
        m._init_not_null();
        m.__set_clazz(clazz);
        m.__set_name(String::from(meta.name));
        m.__set_modifiers(meta.modifiers);
        m.__set_slot(slot);
        m.__set_returnType(ret);
        m.__set_parameterTypes(JArray::from(params));
        m.__set_exceptionTypes(JArray::from(excs));
        m.__set_annotations(__anno_bytes(meta.annotations));
        m.__set_parameterAnnotations(__anno_bytes(meta.param_annotations));
        m.__set_annotationDefault(__anno_bytes(meta.annotation_default));
        m
    }

    /// 反射族内部：描述符 → Class 对象（class_for_descriptor 的类型面）。
    /// MethodHandleNatives.resolve 的字段 kind 类型核对共用（描述符还原的
    /// Class 与 MemberName 携带的 Class 按名相等）。
    pub fn __class_for_descriptor(desc: &str) -> Class {
        class_for_descriptor(desc)
    }

    /// `Class.isAssignableFrom(Class)`：`X.isAssignableFrom(Y)` 即 Y 的类型闭包
    /// 包含 X（含 X == Y）。超类型闭包由生成器在类字面量处静态推导
    /// （父类链 + 全部接口，见 codegen ldc 类字面量的第二参数），随 `for_class`
    /// 登记到线程内侧表；未登记的目标（闭包外类、数组、基本类型）仅同名相等。
    pub fn isAssignableFrom(&self, cls: Class) -> Result<bool> {
        let self_name = format!("{}", self.__get_name()).replace('.', "/");
        let cls_name = format!("{}", cls.__get_name()).replace('.', "/");
        if self_name == cls_name {
            return Ok(true);
        }
        // cls 的超类型闭包（含自身）包含 self 即可赋值。层次表由 build.rs 从
        // java_class! 的 all_supertypes 属性生成（class 元数据的唯一表达）；
        // 未生成/接口载体的 cls 不在表中，退化为同名相等（与 JVM 语义的差异
        // 仅影响"参数侧从未进入闭包"的场景）。
        Ok(__hierarchy::CLASS_HIERARCHY.iter()
            .find(|(n, _)| *n == cls_name)
            .map(|(_, supers)| supers.iter().any(|s| *s == self_name))
            .unwrap_or(false))
    }

    /// 按 binary name（斜线形态；数组为描述符形态 `[I`、`[Ljava/lang/String;`）判定
    /// 「source 类型的值可赋给 target 类型」（JLS §5.2 引用赋值、§4.10.3 数组协变）：
    /// 同名；数组对数组按组件递归（基本组件须相同）；数组可赋给 Object / Cloneable /
    /// Serializable；类经层次表（build.rs 生成的超类型闭包）。数组运行时类判定
    /// （反射创建数组的组件标签，FS-R6）与 `isAssignableFrom` 同一真源。
    pub fn __name_assignable(target: &str, source: &str) -> bool {
        if target == source || target == "java/lang/Object" {
            return true;
        }
        if let Some(src_elem) = source.strip_prefix('[') {
            let Some(tgt_elem) = target.strip_prefix('[') else {
                return matches!(target, "java/lang/Cloneable" | "java/io/Serializable");
            };
            let unwrap = |d: &str| -> Option<std::string::String> {
                if d.starts_with('[') {
                    Some(d.to_string())
                } else {
                    d.strip_prefix('L').and_then(|x| x.strip_suffix(';')).map(|x| x.to_string())
                }
            };
            return match (unwrap(tgt_elem), unwrap(src_elem)) {
                (Some(t), Some(s)) => Self::__name_assignable(&t, &s),
                _ => tgt_elem == src_elem, // 基本组件：描述符字符相同
            };
        }
        __hierarchy::CLASS_HIERARCHY.iter()
            .find(|(n, _)| *n == source)
            .map(|(_, supers)| supers.iter().any(|s| *s == target))
            .unwrap_or(false)
    }


    /// native `Class.getSuperclass()`：直接父类的 Class 对象。
    ///
    /// 查询经 build.rs 从 `java_class!` 的 super_class 属性生成的直接父类表
    /// （与 isAssignableFrom 的层次表同源）。Object 自身 / 接口 / 基本类型 /
    /// 未登记类（闭包外、数组）→ null（JLS 对接口与 Object 返回 null 的语义）。
    #[jvm_native]
    pub fn getSuperclass(&self) -> Result<Class> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        match __direct_super::CLASS_DIRECT_SUPER.iter().find(|(n, _)| *n == name) {
            // for_class 的缓存键是斜线形态（与 ldc 类字面量同一调用形态）——身份语义
            //（`zuper == Enum.class`）依赖同一缓存条目
            Some((_, sup)) => Ok(Class::for_class(String::from(*sup))),
            None => Ok(Class::default()),
        }
    }



    /// `Class.getModule()`：类所属模块。单二进制无模块层——全类集归属
    /// 无名模块单例（module_impl::unnamed_module，isNamed 恒 false）。
    pub fn __impl_getModule(&self) -> Result<crate::java::lang::Module> {
        Ok(super::module_impl::unnamed_module())
    }

    /// native `Class.getModifiers()`：类修饰符位集（Modifier 协议）。
    /// 查询经 build.rs 从 java_class! 的 access/super_class 属性生成的修饰符
    /// 表；未登记形态按 JVM 语义：数组/基本类型类恒 PUBLIC|FINAL|ABSTRACT，
    /// 其余（闭包外类）同款位集（语料合法程序跨包引用必经 public）。
    #[jvm_native]
    pub fn getModifiers(&self) -> Result<i32> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        if name.starts_with('[') {
            return Ok(0x0001 | 0x0010 | 0x0400);
        }
        Ok(__modifiers::CLASS_MODIFIERS.iter()
            .find(|(n, _)| *n == name)
            .map(|(_, m)| *m)
            .unwrap_or(0x0001 | 0x0010 | 0x0400))
    }

    /// `getDeclaredFields()`：本类全部声明字段的构造序列（字段表驱动，
    /// getDeclaredField 的复数形态——同一张 build.rs 字段表循环输出）。
    pub(crate) fn __table_declared_fields(&self) -> Result<JArray<Field>> {
        Field::__class_init()?;
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        let mut out: Vec<Field> = Vec::new();
        if let Some((_, fs)) = __fields::CLASS_FIELDS.iter().find(|(n, _)| *n == cls_key) {
            for (slot, meta) in fs.iter().enumerate() {
                let mut f = Field::default();
                f._init_not_null();
                f.__set_clazz(Clone::clone(self));
                f.__set_name(String::from(meta.name));
                f.__set_modifiers(meta.modifiers);
                f.__set_slot(slot as i32);
                f.__set_type_(class_for_descriptor(meta.descriptor));
                f.__set_annotations(__anno_bytes(meta.annotations));
                out.push(f);
            }
        }
        Ok(JArray::from(out))
    }


    /// `getDeclaredConstructors()`：本类全部声明构造器（方法表 `<init>` 行；
    /// 构造器身份键 = (类, 描述符)——参数还原同 __method_from_meta）。
    pub(crate) fn __table_declared_ctors(&self) -> Result<JArray<crate::java::lang::reflect::Constructor<Object>>> {
        crate::java::lang::reflect::Constructor::<Object>::__class_init()?;
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        let mut out: Vec<crate::java::lang::reflect::Constructor<Object>> = Vec::new();
        if let Some((_, ms)) = __methods::CLASS_METHODS.iter().find(|(n, _)| *n == cls_key) {
            for (slot, meta) in ms.iter().enumerate() {
                if meta.name != "<init>" {
                    continue;
                }
                let mut c = crate::java::lang::reflect::Constructor::<Object>::default();
                c._init_not_null();
                c.__set_clazz(Clone::clone(self));
                c.__set_modifiers(meta.modifiers);
                c.__set_slot(slot as i32);
                let params: Vec<Class> = descriptor_params(meta.descriptor).into_iter()
                    .map(|p| class_for_descriptor(&p)).collect();
                c.__set_parameterTypes(JArray::from(params));
                let excs: Vec<Class> = meta.exceptions.iter()
                    .map(|e| Class::for_class(String::from(*e))).collect();
                c.__set_exceptionTypes(JArray::from(excs));
                c.__set_annotations(__anno_bytes(meta.annotations));
                c.__set_parameterAnnotations(__anno_bytes(meta.param_annotations));
                out.push(c);
            }
        }
        Ok(JArray::from(out))
    }



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
    #[jvm_native(upcalls = "jdk/internal/reflect/ConstantPool.<init>:()V")]
    pub fn getConstantPool(&self) -> Result<crate::jdk::internal::reflect::ConstantPool> {
        let cp = crate::jdk::internal::reflect::ConstantPool::new()?;
        cp.__set_constantPoolOop(Object::from(Clone::clone(self)));
        Ok(cp)
    }


    /// `Class.isRecord()`：record 类判定（JVMS §4.7.30 Record 属性在场；
    /// 发射侧 is_record 属性 → build.rs record 表）。数组 / 基本类型类恒 false。
    /// `Class.getClassLoader()`：类加载器不建模（单一静态链接映像，无运行期
    /// 加载），一律返回 null——即 JDK 对引导类的返回形态。消费方以「null 或
    /// 安全管理器缺席」短路（ObjectStreamClass.getProtectionDomains：
    /// `cl.getClassLoader() != null && System.getSecurityManager() != null`，
    /// JDK 21 安全管理器恒 null，两种返回可观测等价）。
    pub fn getClassLoader(&self) -> Result<crate::java::lang::ClassLoader> {
        Ok(crate::java::lang::ClassLoader::default())
    }


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
    pub fn getInterfaces0(&self) -> Result<JArray<Class>> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        let list: Vec<&str> = if name.starts_with('[') {
            vec!["java/lang/Cloneable", "java/io/Serializable"]
        } else if self.isPrimitive()? {
            Vec::new()
        } else {
            __interfaces::CLASS_INTERFACES.iter()
                .find(|(n, _)| *n == name)
                .map(|(_, l)| l.to_vec())
                .unwrap_or_default()
        };
        let out: Vec<Class> = list.into_iter()
            .map(|i| Class::for_class(String::from(i)))
            .collect();
        Ok(JArray::from(out))
    }

    /// native `Class.isInterface()`：接口（含注解类型）判定，读 build.rs 修饰符表的
    /// INTERFACE 位（与 getModifiers 同源）。数组类 / 基本类型类 → false
    /// （JLS：数组类型与基本类型都不是接口）；闭包外类（表中缺席）→ false。
    /// 消费方：ObjectStreamClass 构造链（Result.<clinit> 的序列化元数据查询）。
    #[jvm_native]
    pub fn isInterface(&self) -> Result<bool> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        if name.starts_with('[') || self.isPrimitive()? {
            return Ok(false);
        }
        Ok(__modifiers::CLASS_MODIFIERS.iter()
            .find(|(n, _)| *n == name)
            .map(|(_, m)| (*m & 0x0200) != 0)
            .unwrap_or(false))
    }


    /// native `Class.forName0(name, initialize, loader, caller)`：按 binary name 取 Class 对象。
    /// 原生镜像的「可加载类」= 生成闭包内的类（build.rs 修饰符表，含用户类）；数组名
    /// （`[I` / `[Ljava.lang.String;`）直接构造。未知类名 → `ClassNotFoundException(name)`
    /// （JDK 同消息）。initialize=true（`Class.forName(String)` 的缺省）立即执行类初始化
    /// （JLS §12.4.1 / FS-C5）：经 main 启动时登记的类初始化钩子（有 `<clinit>` 的用户类）
    /// 按名代调 `__class_init`，初始化异常按状态机语义传播（ExceptionInInitializerError /
    /// 其后 NoClassDefFoundError）；initialize=false 只取 Class 对象（init-passive）。
    #[jvm_native(upcalls = "java/lang/ClassNotFoundException.<init>:(Ljava/lang/String;)V")]
    pub fn forName0(name: String, initialize: bool, _loader: crate::java::lang::ClassLoader,
                    _caller: Class) -> Result<Class> {
        let dotted = format!("{}", name);
        let slash = dotted.replace('.', "/");
        let known = slash.starts_with('[')
            || __modifiers::CLASS_MODIFIERS.iter().any(|(n, _)| *n == slash);
        if !known {
            let ex = crate::java::lang::ClassNotFoundException::new_str(String::from(dotted.as_str()))?;
            return Err(ex.into());
        }
        if initialize && !slash.starts_with('[') {
            crate::ensure_class_initialized(&slash)?;
        }
        Ok(Class::for_class(String::from(slash.as_str())))
    }

    /// native `Class.getRecordComponents0()`：record 分量反射（声明序）。数据源是
    /// java_class! 块的 `record_components` 属性（classfile Record 属性：名字 /
    /// 描述符 / 泛型签名），build.rs 汇总为 RECORD_COMPONENTS 表。查询即构造
    /// RecordComponent：clazz=本类、type=描述符还原、accessor=同名无参声明方法、
    /// signature=泛型签名（无则 null）。非 record（表中缺席）→ null（JDK 语义）。
    /// 消费方：ObjectStreamClass 的 record 序列化（规范构造器 / 分量取值）。
    /// upcalls：RecordComponent 由此处构造（无 `new` 指令可见）→ 声明其构造器使 BFS
    /// 记为已实例化（toString 等覆盖经 Object 视图可达）；访问器经 getDeclaredMethod 查询。
    #[jvm_native(upcalls = "java/lang/reflect/RecordComponent.<init>:()V java/lang/Class.getDeclaredMethod:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;")]
    pub fn getRecordComponents0(&self) -> Result<JArray<crate::java::lang::reflect::RecordComponent>> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        let comps: &[(&str, &str, &str)] = match __record::RECORD_COMPONENTS.iter().find(|(c, _)| *c == name) {
            Some((_, comps)) => comps,
            // 无分量的 record（`record Empty()`）不产生 record_components 属性 → 空数组
            None if __record::RECORD_CLASSES.contains(&name.as_str()) => &[],
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
            rc.__set_signature(if g.is_empty() { String::default() } else { String::from(*g) });
            out.push(rc);
        }
        Ok(JArray::from(out))
    }


}

/// 描述符 → Class 对象（getDeclaredField 的 type 填充与 getComponentType 的
/// 元素解析共用）：`L<类>;` / `[<描述符>` 经 for_class（斜线键与 ldc 类字面量
/// 同一缓存条目，身份一致）；基本类型描述符经 getPrimitiveClass 的唯一实例。
/// 点形式（Class 名存储形态）先归一为斜线。无法识别 → null Class。
impl Class {
    /// 字段描述符（含 `V`）→ Class，引用类型须在类宇宙内（修饰符表有行），否则
    /// TypeNotPresentException（JDK 经 Class.forName 失败时的同一异常）。消费方：
    /// AnnotationParser.parseSig（注解签名恒为描述符形态，FS-R R4b）。
    pub(crate) fn __from_descriptor_checked(desc: &str) -> Result<Class> {
        let elem = desc.trim_start_matches('[');
        if let Some(name) = elem.strip_prefix('L').and_then(|x| x.strip_suffix(';')) {
            let known = __modifiers::CLASS_MODIFIERS.iter().any(|(n, _)| *n == name)
                || name == "java/lang/Object";
            if !known {
                let ex = crate::java::lang::TypeNotPresentException::new(
                    String::from(name.replace('/', ".").as_str()), Default::default())?;
                return Err(crate::error::JvmError::from(ex));
            }
        }
        Ok(class_for_descriptor(desc))
    }
}

fn class_for_descriptor(desc: &str) -> Class {
    let slashes: std::string::String = desc.replace('.', "/");
    let prim = |java_name: &str| {
        Class::getPrimitiveClass(String::from(java_name)).unwrap_or_default()
    };
    match slashes.as_bytes().first() {
        Some(b'L') if slashes.ends_with(';') =>
            Class::for_class(String::from(&slashes[1..slashes.len() - 1])),
        Some(b'[') => Class::for_class(String::from(slashes.as_str())),
        Some(b'B') => prim("byte"),
        Some(b'C') => prim("char"),
        Some(b'D') => prim("double"),
        Some(b'F') => prim("float"),
        Some(b'I') => prim("int"),
        Some(b'J') => prim("long"),
        Some(b'S') => prim("short"),
        Some(b'Z') => prim("boolean"),
        // 返回描述符 V：void.class（与 Void.TYPE = getPrimitiveClass("void") 同一
        // 缓存实例——JUnit validatePublicVoid 的 `getReturnType() != Void.TYPE`
        // 身份比较依赖此；缺席时 void 方法返回类型为 null，@Test 校验全部失败）
        Some(b'V') => prim("void"),
        _ => Class::default(),
    }
}

/// 方法描述符的参数段 → 逐参数描述符序列（`"(ILjava/lang/String;[I)V"` →
/// ["I", "Ljava/lang/String;", "[I"]；顶层右括号定界，嵌套 `[` 前缀整体归
/// 当前参数）。消费方：getDeclaredMethod 的参数配对（重载语义：JDK 查询
/// 不含返回类型）与 Method.parameterTypes 还原。
fn descriptor_params(descriptor: &str) -> Vec<std::string::String> {
    let Some(open) = descriptor.find('(') else { return Vec::new() };
    let Some(close) = descriptor[open..].find(')').map(|i| i + open) else { return Vec::new() };
    let body = &descriptor[open + 1..close];
    let mut out = Vec::new();
    let mut cur = std::string::String::new();
    for ch in body.chars() {
        cur.push(ch);
        if cur.starts_with('L') {
            // 类描述符：累积到 ';' 闭合
            if ch == ';' { out.push(std::mem::take(&mut cur)); }
            continue;
        }
        if ch == '[' {
            // 数组前缀：归当前参数继续累积（[[I / [Ljava/lang/String; 均整体）
            continue;
        }
        // 基本类型字符（Z B C S I J F D）自成一段
        out.push(std::mem::take(&mut cur));
    }
    out
}

/// 反射族内部：直接父类表查询（L3 分派协议的上溯数据面，
/// reflect_dispatch::reflect_invoke 消费）。
pub fn __direct_super_lookup(class_slash: &str) -> Option<&'static str> {
    __direct_super::CLASS_DIRECT_SUPER.iter()
        .find(|(n, _)| *n == class_slash)
        .map(|(_, s)| *s)
}

/// 反射族内部：方法元数据表的 static 判定（L3 分派协议的 static/虚分派
/// 判别面，reflect_dispatch 消费）。未声明 → false（按虚方法处理，上溯）。
pub fn __method_is_static(class_slash: &str, name: &str, descriptor: &str) -> bool {
    __methods::CLASS_METHODS.iter()
        .find(|(n, _)| *n == class_slash)
        .and_then(|(_, ms)| ms.iter().find(|m| m.name == name && m.descriptor == descriptor))
        .map(|m| m.is_static)
        .unwrap_or(false)
}

/// build.rs 生成的类层次表（OUT_DIR/hierarchy_table.rs，含模块级 static）。
mod __hierarchy {
    include!(concat!(env!("OUT_DIR"), "/hierarchy_table.rs"));
}

/// build.rs 生成的直接父类表（OUT_DIR/direct_super_table.rs）。
mod __direct_super {
    include!(concat!(env!("OUT_DIR"), "/direct_super_table.rs"));
}

/// build.rs 生成的字段元数据表（OUT_DIR/field_table.rs，含 FieldMeta 与
/// CLASS_FIELDS static）。
mod __fields {
    include!(concat!(env!("OUT_DIR"), "/field_table.rs"));
}

/// build.rs 生成的方法元数据表（OUT_DIR/method_table.rs，含 MethodMeta 与
/// CLASS_METHODS static）。
mod __methods {
    include!(concat!(env!("OUT_DIR"), "/method_table.rs"));
}

/// build.rs 生成的类修饰符表（OUT_DIR/modifiers_table.rs）。
mod __modifiers {
    include!(concat!(env!("OUT_DIR"), "/modifiers_table.rs"));
}

/// build.rs 生成的 record 类集（OUT_DIR/record_table.rs）。
mod __record {
    include!(concat!(env!("OUT_DIR"), "/record_table.rs"));
}

/// 注解原始属性体 → Java byte[]（空 = 属性缺席 → null，与 HotSpot 同）。
fn __anno_bytes(raw: &'static [u8]) -> JArray<i8> {
    if raw.is_empty() {
        return JArray::default();
    }
    JArray::from(raw.iter().map(|b| *b as i8).collect::<Vec<i8>>())
}

/// build.rs 生成的直接超接口表（OUT_DIR/interfaces_table.rs）。
mod __interfaces {
    include!(concat!(env!("OUT_DIR"), "/interfaces_table.rs"));
}

/// build.rs 生成的嵌套元数据表（OUT_DIR/nest_table.rs，FS-R R1）。
mod __nest {
    include!(concat!(env!("OUT_DIR"), "/nest_table.rs"));
}

// ── FS-R R1：类级 native（数据源 = 元数据表，JDK 公开方法体回到字节码）────────────
//
// HotSpot 从 class 文件的 InnerClasses / EnclosingMethod / Record 属性取值；原生二进制
// 以 build.rs 表承载同一数据（docs/plans/2026-09-27-reflection-metadata-table.md）。
impl Class {
    fn __slash_name(&self) -> std::string::String {
        format!("{}", self.__get_name()).replace('.', "/")
    }

    fn __nest(&self) -> Option<&'static __nest::NestMeta> {
        let key = self.__slash_name();
        __nest::CLASS_NEST.iter().find(|(n, _)| *n == key).map(|(_, m)| m)
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
        Ok(!key.starts_with('[') && __record::RECORD_CLASSES.contains(&key.as_str()))
    }

    /// static native `desiredAssertionStatus0(Class)`：断言恒关（`-ea` 缺省，JVM 同）。
    #[jvm_native]
    pub fn desiredAssertionStatus0(_c: Class) -> Result<bool> {
        Ok(false)
    }

    /// native `isHidden()`：原生二进制无运行期定义的隐藏类（lambda 代理为编译期合成类）。
    #[jvm_native]
    pub fn isHidden(&self) -> Result<bool> {
        Ok(false)
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

// ── FS-R R2：成员查询 native（元数据表构造 Field / Method / Constructor，JDK 查询族回到字节码）──
impl Class {
    /// native `getDeclaredFields0(boolean publicOnly)`：本类声明字段（声明序 = slot）。
    /// trustedFinal 与 HotSpot 同判定：static final，或 record 类的 final 实例字段。
    #[jvm_native]
    pub fn getDeclaredFields0(&self, public_only: bool) -> Result<JArray<Field>> {
        let all = self.__table_declared_fields()?;
        let is_record = self.isRecord0()?;
        let mut out: Vec<Field> = Vec::new();
        for i in 0..all.len()? {
            let mut f = all.get(i)?;
            let mods = f.__get_modifiers();
            if public_only && mods & 0x0001 == 0 {
                continue;
            }
            let fin = mods & 0x0010 != 0;
            f.__set_trustedFinal(fin && (mods & 0x0008 != 0 || is_record));
            out.push(f);
        }
        Ok(JArray::from(out))
    }

    /// native `getDeclaredMethods0(boolean publicOnly)`：本类声明方法（不含 `<init>` / `<clinit>`）。
    #[jvm_native]
    pub fn getDeclaredMethods0(&self, public_only: bool) -> Result<JArray<crate::java::lang::reflect::Method>> {
        let all = self.__table_declared_methods()?;
        let mut out = Vec::new();
        for i in 0..all.len()? {
            let m = all.get(i)?;
            if !public_only || m.__get_modifiers() & 0x0001 != 0 {
                out.push(m);
            }
        }
        Ok(JArray::from(out))
    }

    /// native `getDeclaredConstructors0(boolean publicOnly)`：本类声明构造器。
    #[jvm_native]
    pub fn getDeclaredConstructors0(&self, public_only: bool)
        -> Result<JArray<crate::java::lang::reflect::Constructor<Object>>>
    {
        let all = self.__table_declared_ctors()?;
        let mut out = Vec::new();
        for i in 0..all.len()? {
            let c = all.get(i)?;
            if !public_only || c.__get_modifiers() & 0x0001 != 0 {
                out.push(c);
            }
        }
        Ok(JArray::from(out))
    }
}

impl Class {
    /// 本类按 (名字, 描述符) 声明的方法（VM 直取反射对象：动态代理的接口方法对象，
    /// HotSpot 同样经方法元数据构造）。未声明 → null。
    pub(crate) fn __table_method(&self, name: &str, descriptor: &str) -> Result<crate::java::lang::reflect::Method> {
        crate::java::lang::reflect::Method::__class_init()?;
        for (slot, meta) in self.__declared_method_rows().iter().enumerate() {
            if meta.name == name && meta.descriptor == descriptor {
                return Ok(Self::__method_from_meta(Clone::clone(self), meta, slot as i32));
            }
        }
        Ok(crate::java::lang::reflect::Method::default())
    }

    /// 本类声明的无参方法（record 组件访问器）：元数据表直构，不经公开查询族
    /// （getRecordComponents0 是 native，JDK 侧同样由 VM 直接取方法对象）。
    pub(crate) fn __table_method_noargs(&self, name: &str) -> Result<crate::java::lang::reflect::Method> {
        let all = self.__table_declared_methods()?;
        for i in 0..all.len()? {
            let m = all.get(i)?;
            if format!("{}", m.__get_name()) == name && m.__get_parameterTypes().len()? == 0 {
                return Ok(m);
            }
        }
        Ok(crate::java::lang::reflect::Method::default())
    }
}
