use crate::prelude::*;
use super::*;
use super::reflect::Field;
use crate::sync_model::__RefSlot as RefCell;
use std::collections::HashMap;
mod attrs;
use attrs::*;
mod members;
use members::*;
mod nest;
use nest::*;

impl Class {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
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
    /// `java.util.List`），数组类型保持 JVM 描述符形态（`[Ljava.lang.String;`）；隐藏类
    /// （lambda 调用点类 `p/C$$Lambda/0x…`）只换调用者类部分，后缀前保留 `/`（`p.C$$Lambda/0x…`，
    /// JVM 隐藏类命名），反向换回斜线即元数据表键。
    /// isAssignableFrom 的层次查询在运行时经 java_meta 生成的层次表进行，
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
        let dotted = crate::meta::java_name(&key);
        c.__set_name(String::from(dotted.as_str()));
        // 数组类的 componentType 字段由 VM 在建镜像时填充（HotSpot set_component_mirror）：
        // 字节码翻译的 `componentType()` / `arrayType` 链直接读该字段（MethodHandleImpl
        // .makeCollector 的 nCopies(n, arrayType.componentType())）。元素 Class 经 for_class
        // 解析——须在缓存借用之外（递归入本函数）
        if let Some(rest) = key.strip_prefix('[') {
            c.__set_componentType(class_for_descriptor(rest));
        }
        CLASSES.with(|cache| Clone::clone(cache.borrow_mut().entry(key).or_insert(c)))
    }

    /// VM 注入状态 `classLoader` 的落地（vm_intrinsics.toml `[vm_state.field_hooks]`，准入 ③）：HotSpot
    /// `java_lang_Class::create_mirror` 建镜像时写入定义加载器；原生镜像在首次访问该字段前按定义加载器表
    /// （java_meta `CLASS_DEFINING_LOADER`，生成器按类所在模块求出）填充。数组类取元素类的加载器，基本类型、
    /// `void` 与引导加载器定义的类为 null。加载器对象取自 `ClassLoaders` 的字节码访问器（同一层级实例）。
    pub fn __vm_defining_loader(&self) -> Result<&Self> {
        if !Object::from(self.__get_classLoader()).0.is_jvm_null() {
            return Ok(self);
        }
        let name = format!("{}", self.__get_name());
        let elem = name.trim_start_matches('[');
        let elem = if elem.len() < name.len() {
            match elem.strip_prefix('L').and_then(|e| e.strip_suffix(';')) {
                Some(e) => e,
                None => return Ok(self),
            }
        } else {
            elem
        };
        let loader = match crate::meta::class_defining_loader(&elem.replace('.', "/")) {
            Some("app") => crate::jdk::internal::loader::ClassLoaders::appClassLoader()?,
            Some("platform") => crate::jdk::internal::loader::ClassLoaders::platformClassLoader()?,
            _ => return Ok(self),
        };
        self.__set_classLoader(loader);
        Ok(self)
    }


    /// VM 注入状态 `module` 的落地（vm_intrinsics.toml `[vm_state.field_hooks]`，准入 ③）：HotSpot 建镜像时
    /// （`java_lang_Class::create_mirror`）按类的包条目写入所属模块。按「定义加载器 + 包」查 VM 模块表
    /// （映像中的引导层模块与运行期 defineModule0 登记的模块）；未登记的包归定义加载器的无名模块
    /// （引导加载器为 `BootLoader.getUnnamedModule()`）。数组类取元素类型的模块，基本类型与其数组属 java.base。
    pub fn __vm_module(&self) -> Result<&Self> {
        if !Object::from(self.__get_module()).0.is_jvm_null() {
            return Ok(self);
        }
        let name = format!("{}", self.__get_name()).replace('.', "/");
        let elem = name.trim_start_matches('[');
        let pkg = if elem.len() < name.len() {
            elem.strip_prefix('L').and_then(|e| e.strip_suffix(';')).map(|e| e.rsplit_once('/').map_or("", |(p, _)| p))
        } else if name.contains('/') || !is_primitive_name(&name) {
            Some(name.rsplit_once('/').map_or("", |(p, _)| p))
        } else {
            None
        };
        let loader = self.__vm_defining_loader()?.__get_classLoader();
        let module = match Module::__vm_package_module(Object::from(Clone::clone(&loader)), pkg) {
            Some(m) => m,
            None if Object::from(Clone::clone(&loader)).0.is_jvm_null() => crate::jdk::internal::loader::BootLoader::getUnnamedModule()?,
            None => loader.getUnnamedModule()?,
        };
        self.__set_module(module);
        Ok(self)
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

    /// 反射族内部：按字段名查本类声明元数据（描述符 / static 标志 / 修饰位 /
    /// ConstantValue 整数值）。getDeclaredField 与 Field.get/set（reflect 邻域
    /// 伴生 field_impl.rs）共用同一张 java_meta 字段表；未声明 → None。
    pub fn __declared_field_meta(&self, name: &str) -> Option<(&'static str, bool, i32, Option<i64>)> {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        crate::meta::class_fields().iter()
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
        crate::meta::class_methods().iter()
            .find(|(n, _)| *n == cls_key)
            .and_then(|(_, ms)| ms.iter().find(|m| m.name == name && m.descriptor == descriptor))
            .map(|m| (m.modifiers, m.is_static, m.is_native, m.is_abstract))
    }

    /// 反射族内部：本类声明方法行（构造器/类初始化器行含在内，消费方按
    /// JDK 语义过滤）。消费方：getDeclaredMethod / getDeclaredMethods。
    fn __declared_method_rows(&self) -> &'static [crate::meta::MethodMeta] {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        crate::meta::class_methods().iter()
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
            if meta.name == "<init>" || meta.name == "<clinit>" || meta.inherited {
                continue;
            }
            out.push(self.__method_from_meta(meta, slot as i32));
        }
        Ok(JArray::from(out))
    }

    /// 方法元数据行 → Method（查询即构造）。parameterTypes / returnType 从
    /// 描述符还原（class_for_descriptor 的数组形态：`[...` 直接 for_class），
    /// exceptionTypes 从 throws 子句列表还原。
    /// 声明类即接收者本身（`clazz` = `self`，分析器据此把 Method.clazz 接为接收者镜像）。
    fn __method_from_meta(&self, meta: &'static crate::meta::MethodMeta, slot: i32) -> crate::java::lang::reflect::Method {
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
        m.__set_clazz(Clone::clone(self));
        m.__set_name(String::from(meta.name));
        m.__set_modifiers(meta.modifiers);
        m.__set_slot(slot);
        m.__set_returnType(ret);
        m.__set_parameterTypes(JArray::from(params));
        m.__set_exceptionTypes(JArray::from(excs));
        m.__set_annotations(__anno_bytes(meta.annotations));
        m.__set_parameterAnnotations(__anno_bytes(meta.param_annotations));
        m.__set_annotationDefault(__anno_bytes(meta.annotation_default));
        m.__set_signature(__signature(meta.signature));
        m
    }

    /// 反射族内部：描述符 → Class 对象（class_for_descriptor 的类型面）。
    /// MethodHandleNatives.resolve 的字段 kind 类型核对共用（描述符还原的
    /// Class 与 MemberName 携带的 Class 按名相等）。
    pub fn __class_for_descriptor(desc: &str) -> Class {
        class_for_descriptor(desc)
    }

    /// native `Class.isAssignableFrom(Class)`：`X.isAssignableFrom(Y)` 即 Y 类型的值可赋给 X
    /// （JVMS §6.5 checkcast / instanceof 的类型兼容规则）。基本类型类只与自身相容；
    /// 引用类型（含数组：元素引用类型协变、基本元素须相同、多维逐层、数组可赋给
    /// Object / Cloneable / Serializable）统一经 `__name_assignable` 判定（与 checkcast /
    /// 反射数组组件标签同一真源）。null 实参 → NPE（JDK 同）。
    pub fn isAssignableFrom(&self, cls: Class) -> Result<bool> {
        if Object::from(Clone::clone(&cls)).0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let self_name = format!("{}", self.__get_name()).replace('.', "/");
        let cls_name = format!("{}", cls.__get_name()).replace('.', "/");
        if self_name == cls_name {
            return Ok(true);
        }
        if self.isPrimitive()? || cls.isPrimitive()? {
            return Ok(false);
        }
        Ok(Self::__name_assignable(&self_name, &cls_name))
    }

    /// 按 binary name（斜线形态；数组为描述符形态 `[I`、`[Ljava/lang/String;`）判定
    /// 「source 类型的值可赋给 target 类型」（JLS §5.2 引用赋值、§4.10.3 数组协变）：
    /// 同名；数组对数组按组件递归（基本组件须相同）；数组可赋给 Object / Cloneable /
    /// Serializable；类经层次表（java_meta 生成的超类型闭包）。数组运行时类判定
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
        if crate::meta::class_hierarchy().iter()
            .find(|(n, _)| *n == source)
            .map(|(_, supers)| supers.iter().any(|s| *s == target))
            .unwrap_or(false)
        {
            return true;
        }
        // 接口块不携带 all_supertypes（层次表无行）：沿直接超接口表（class 文件 interfaces
        // 项）传递查找——接口 → 超接口（注解类型 → Annotation 等）
        let mut stack: Vec<&str> = vec![source];
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        while let Some(cur) = stack.pop() {
            if !seen.insert(cur) {
                continue;
            }
            if let Some((_, ifaces)) = crate::meta::class_interfaces().iter().find(|(n, _)| *n == cur) {
                for i in ifaces.iter() {
                    if *i == target {
                        return true;
                    }
                    stack.push(i);
                }
            }
        }
        false
    }


    /// native `Class.getSuperclass()`：直接父类的 Class 对象。
    ///
    /// 查询经 java_meta 从 `java_class!` 的 super_class 属性生成的直接父类表
    /// （与 isAssignableFrom 的层次表同源）。数组类 → Object（JLS §10.8）；
    /// Object 自身 / 接口 / 基本类型 / 未登记类（闭包外）→ null。
    #[jvm_native]
    pub fn getSuperclass(&self) -> Result<Class> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        if name.starts_with('[') {
            return Ok(Class::for_class(String::from("java/lang/Object")));
        }
        match crate::meta::class_direct_super().iter().find(|(n, _)| *n == name) {
            // for_class 的缓存键是斜线形态（与 ldc 类字面量同一调用形态）——身份语义
            //（`zuper == Enum.class`）依赖同一缓存条目
            Some((_, sup)) => Ok(Class::for_class(String::from(*sup))),
            // @CallerSensitive 注入调用器（运行期登记的隐藏类）：超类 Object（JDK 模板同）
            None if crate::injected_invoker::is_injected(&name) => Ok(Class::for_class(String::from("java/lang/Object"))),
            None => Ok(Class::default()),
        }
    }



    /// native `Class.getModifiers()`：类修饰符位集（Modifier 协议）。
    /// 查询经 java_meta 从 java_class! 的 access/super_class 属性生成的修饰符
    /// 表；未登记形态按 JVM 语义：数组/基本类型类恒 PUBLIC|FINAL|ABSTRACT，
    /// 其余（闭包外类）同款位集（语料合法程序跨包引用必经 public）。
    #[jvm_native]
    pub fn getModifiers(&self) -> Result<i32> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        if name.starts_with('[') {
            return Ok(0x0001 | 0x0010 | 0x0400);
        }
        Ok(crate::meta::class_modifiers().iter()
            .find(|(n, _)| *n == name)
            .map(|(_, m)| *m)
            .unwrap_or(0x0001 | 0x0010 | 0x0400))
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
            let known = crate::meta::class_modifiers().iter().any(|(n, _)| *n == name)
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
    Class::__descriptor_params(descriptor)
}

impl Class {
    /// 方法描述符的逐参数描述符序列（见 [`descriptor_params`]；反射调用的实参校验共用）
    pub fn __descriptor_params(descriptor: &str) -> Vec<std::string::String> {
        let Some(open) = descriptor.find('(') else { return Vec::new() };
        let Some(close) = descriptor[open..].find(')').map(|i| i + open) else { return Vec::new() };
        let body = &descriptor[open + 1..close];
        let mut out = Vec::new();
        let mut cur = std::string::String::new();
        for ch in body.chars() {
            cur.push(ch);
            if cur.trim_start_matches('[').starts_with('L') {
                // 类描述符（含对象数组 `[Ljava/lang/String;`）：累积到 ';' 闭合
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
}

/// 反射族内部：直接父类表查询（L3 分派协议的上溯数据面，
/// reflect_dispatch::reflect_invoke 消费）。
pub fn __direct_super_lookup(class_slash: &str) -> Option<&'static str> {
    crate::meta::class_direct_super().iter()
        .find(|(n, _)| *n == class_slash)
        .map(|(_, s)| *s)
}

/// 反射族内部：方法元数据表的 static 判定（L3 分派协议的 static/虚分派
/// 判别面，reflect_dispatch 消费）。未声明 → false（按虚方法处理，上溯）。
pub fn __method_is_static(class_slash: &str, name: &str, descriptor: &str) -> bool {
    crate::meta::class_methods().iter()
        .find(|(n, _)| *n == class_slash)
        .and_then(|(_, ms)| ms.iter().find(|m| m.name == name && m.descriptor == descriptor))
        .map(|m| m.is_static)
        .unwrap_or(false)
}








/// 注解原始属性体 → Java byte[]（空 = 属性缺席 → null，与 HotSpot 同）。
fn __anno_bytes(raw: &'static [u8]) -> JArray<i8> {
    if raw.is_empty() {
        return JArray::default();
    }
    JArray::from(raw.iter().map(|b| *b as i8).collect::<Vec<i8>>())
}

/// 元数据表的 Signature 属性 → 反射对象的 `signature` 字段（无 Signature 属性时为 null，
/// 与 HotSpot `Reflection::new_method` / `new_field` / `new_constructor` 同义）。
fn __signature(sig: &str) -> String {
    if sig.is_empty() { String::default() } else { String::from(sig) }
}

/// 基本类型名（类镜像名为 Java 关键字形态：`int`、`void` …）
fn is_primitive_name(n: &str) -> bool {
    matches!(n, "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double" | "void")
}
