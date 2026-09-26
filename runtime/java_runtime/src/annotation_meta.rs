//! 反射注解元数据（L3 段 1）：RuntimeVisibleAnnotations 的运行时消费面。
//!
//! 数据流（与层次/字段/方法四表同构的静态注册表路线）：
//!   codegen 解析 class 文件的 RuntimeVisibleAnnotations（类/方法/字段三挂载点）
//!   → java_class! 属性（`annotations = "..."`，载荷编码见 classfile._anno_esc）
//!   → build.rs 造表（OUT_DIR/annotation_table.rs，元素值「tag:载荷」透传）
//!   → 本模块解码 + 查询层（Class/Method/Field 的 isAnnotationPresent /
//!   getAnnotation / getAnnotations，class_impl.rs / method_impl.rs /
//!   field_impl.rs）。
//!
//! 注解实例形态（getAnnotation 的返回值）：JDK 侧 getAnnotation 返回实现了
//! 注解接口的**动态代理**（sun.reflect.annotation.AnnotationParser 产物）；本
//! 架构没有运行时代码生成，对应物是**翻译期合成的最小注解实例**——codegen
//! 为每个注解类型（ACC_ANNOTATION 接口）合成 `<Short>__AnnotationProxy`
//!（共置注解接口翻译文件尾部，sam_objects 的同一合成模式）：实现注解接口的
//! `__VTable`（元素访问器返回字段，默认值 baked）、`ObjectVTable::__interface`
//!（接口载体视图应答——checkcast 到注解类型 / 经载体调用元素方法由此成立）。
//! 生成项目 main 启动时按语料登记工厂（`register_annotation_factories`，与
//! 类初始化钩子同一登记模式），`annotation_instance` 按名代调。

use crate::error::Result;
use crate::java::lang::Object;
use std::collections::HashMap;
use crate::sync_model::__Shared as Rc;

/// 注解元素值（JVMS element_value 的结构化形态；`X` = 未支持形态）。
#[derive(Clone, Debug)]
pub enum AnnotationValue {
    Z(bool),
    B(i32),
    C(u16),
    S(i32),
    I(i32),
    J(i64),
    F(f32),
    D(f64),
    Cls(String),                        // class 元素（JVM 描述符形态载荷）
    Str(String),
    Enum(String, String),               // (枚举类 binary, 常量名)
    Arr(Vec<AnnotationValue>),
    Ann(String, Vec<(String, AnnotationValue)>),  // 嵌套注解
    X,
}

/// 载荷百分号解码（`%XX` 字节序列 → 字符）。与 codegen._anno_esc 互逆
///（保留字符 = 分隔符 `; # =`、引号、反斜杠、控制字符；UTF-8 多字节逐字节
/// 编码，按字节重组后 from_utf8 还原）。
fn unescape(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// 切分（载荷内保留字符已百分号编码——分隔符只以分隔身份出现）。
fn split_escaped(s: &str, sep: char) -> Vec<String> {
    s.split(sep).map(str::to_owned).collect()
}

/// 元素值文本（`tag:载荷`，build.rs 表的透传形态）→ AnnotationValue。
pub fn decode_value(text: &str) -> AnnotationValue {
    let Some(colon) = text.find(':') else { return AnnotationValue::X };
    let (tag, payload) = text.split_at(colon);
    let payload = &payload[1..];
    match tag {
        "Z" => AnnotationValue::Z(payload == "true"),
        "B" => payload.parse::<i32>().map(AnnotationValue::B).unwrap_or(AnnotationValue::X),
        "C" => payload.parse::<u16>().map(AnnotationValue::C).unwrap_or(AnnotationValue::X),
        "S" => payload.parse::<i32>().map(AnnotationValue::S).unwrap_or(AnnotationValue::X),
        "I" => payload.parse::<i32>().map(AnnotationValue::I).unwrap_or(AnnotationValue::X),
        "J" => payload.parse::<i64>().map(AnnotationValue::J).unwrap_or(AnnotationValue::X),
        "F" => payload.parse::<f32>().map(AnnotationValue::F).unwrap_or(AnnotationValue::X),
        "D" => payload.parse::<f64>().map(AnnotationValue::D).unwrap_or(AnnotationValue::X),
        "s" => AnnotationValue::Str(unescape(payload)),
        "c" => AnnotationValue::Cls(unescape(payload)),
        "e" => {
            // 载荷 `binary#常量名`（两段均无转义需求）
            let mut it = payload.splitn(2, '#');
            let (a, b) = (it.next().unwrap_or(""), it.next().unwrap_or(""));
            AnnotationValue::Enum(a.to_owned(), b.to_owned())
        }
        "a" => {
            // 复合载荷整体编码过一层：先解码再按 `;` 切元素
            let inner = unescape(payload);
            let items = split_escaped(&inner, ';')
                .into_iter()
                .map(|it| decode_value(&it))
                .collect();
            AnnotationValue::Arr(items)
        }
        "@" => {
            // 嵌套注解：`binary#name=tag:载荷#...`（整体编码过一层）
            let inner = unescape(payload);
            let segs = split_escaped(&inner, '#');
            let bin = segs.first().map(|s| unescape(s)).unwrap_or_default();
            let mut elems: Vec<(String, AnnotationValue)> = Vec::new();
            for seg in segs.iter().skip(1) {
                if let Some(eq) = seg.find('=') {
                    elems.push((seg[..eq].to_owned(), decode_value(&seg[eq + 1..])));
                }
            }
            AnnotationValue::Ann(bin, elems)
        }
        _ => AnnotationValue::X,
    }
}

/// 注解工厂：元素值序列 → 注解实例（合成代理的 from_values）。
pub type AnnotationFactory = Rc<crate::__DynFn!((&[(String, AnnotationValue)]) -> Result<Object>)>;

crate::__process_static! {
    static FACTORIES: crate::sync_model::__RefSlot<HashMap<String, AnnotationFactory>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

/// 生成项目 main 启动时登记注解工厂（binary name 斜线形态；同名重登记幂等）。
pub fn register_annotation_factories(factories: &[(&str, AnnotationFactory)]) {
    FACTORIES.with(|f| {
        let mut f = f.borrow_mut();
        for (name, fac) in factories {
            f.insert((*name).to_owned(), Clone::clone(fac));
        }
    });
}

/// 按注解类型名 + 元素序列构造注解实例（getAnnotation 的实例面）。
/// 工厂未登记（语料外注解类型 / 独立构建 java_runtime）→ panic stub
///（如实报出缺口，与反射族其他未覆盖面的语义一致）。
pub fn annotation_instance(anno_bin: &str, elements: &[(&str, &str)]) -> Result<Object> {
    let factory = FACTORIES.with(|f| f.borrow().get(anno_bin).map(Clone::clone));
    let Some(factory) = factory else {
        panic!("stub: 注解工厂未登记（{}）——该注解类型的代理未合成或未登记", anno_bin);
    };
    let vals: Vec<(String, AnnotationValue)> = elements.iter()
        .map(|(n, v)| ((*n).to_owned(), decode_value(v)))
        .collect();
    factory(&vals)
}

// ── build.rs 注解元数据表（OUT_DIR/annotation_table.rs）─────────────────────

pub(crate) mod __anno_table {
    include!(concat!(env!("OUT_DIR"), "/annotation_table.rs"));
}

/// 类挂载点注解条目（binary name 斜线形态；无记录 → 空片）。
pub fn class_annotation_entries(cls_slash: &str) -> &'static [__anno_table::AnnotationEntry] {
    __anno_table::CLASS_ANNOTATIONS.iter()
        .find(|(n, _)| *n == cls_slash)
        .map(|(_, e)| *e)
        .unwrap_or(&[])
}

/// 方法挂载点注解条目（键 = (类, 方法名, 描述符)，与 method_table 同一身份键）。
pub fn method_annotation_entries(cls_slash: &str, name: &str, desc: &str)
    -> &'static [__anno_table::AnnotationEntry] {
    __anno_table::METHOD_ANNOTATIONS.iter()
        .find(|(c, n, d, _)| *c == cls_slash && *n == name && *d == desc)
        .map(|(_, _, _, e)| *e)
        .unwrap_or(&[])
}

/// 字段挂载点注解条目（键 = (类, 字段名)）。
pub fn field_annotation_entries(cls_slash: &str, name: &str)
    -> &'static [__anno_table::AnnotationEntry] {
    __anno_table::FIELD_ANNOTATIONS.iter()
        .find(|(c, n, _)| *c == cls_slash && *n == name)
        .map(|(_, _, e)| *e)
        .unwrap_or(&[])
}

/// 条目序列中按注解类型名取命中条目（getAnnotation 语义；首个命中）。
pub fn find_annotation<'a>(entries: &'a [__anno_table::AnnotationEntry], anno_slash: &str)
    -> Option<&'a __anno_table::AnnotationEntry> {
    entries.iter().find(|e| e.anno == anno_slash)
}

/// 条目序列中按注解类型名判存在（isAnnotationPresent 语义——纯名匹配，
/// 不构造实例，无需工厂登记）。
pub fn has_annotation(entries: &[__anno_table::AnnotationEntry], anno_slash: &str) -> bool {
    entries.iter().any(|e| e.anno == anno_slash)
}
