//! BoundMethodHandle 动态物种的 VM 侧（N11，方案 `docs/plans/2026-09-27-bmh-dynamic-species.md`）。
//!
//! jlink 未预生成的物种 key 在 JDK 上由 ASM 现场生成专用类；原生二进制把全部动态 key
//! 统一承载于 VM 支持类 `BoundMethodHandle$Species_Dyn`（Java 源码，字节码翻译）：
//! - `ClassSpecializer$Factory.generateConcreteSpeciesCode` 返回 Species_Dyn，并登记
//!   key → SpeciesData（`register`）；
//! - ClassSpecializer 随后按 key 形态查找的成员由本模块应答：
//!   `make(MethodType, LambdaForm, T0..Tn-1)`（findStatic）→ 转调规范工厂
//!   `make(MethodType, LambdaForm, SpeciesData, Object[])`；`arg<T><i>`（findGetter）→
//!   实例 `args[i]`。
//! 静态字段 `BMH_SPECIES` 是 Species_Dyn 的真实字段（全部动态 key 共用写入落点），
//! 走既有字段闭包，不经本模块。

use crate::error::Result;
use crate::java::lang::Object;
use crate::JArray;

/// VM 支持类 binary name。
pub const SPECIES_DYN: &str = "java/lang/invoke/BoundMethodHandle$Species_Dyn";

/// 规范工厂描述符（Species_Dyn.make 的真实声明）。
const CANONICAL_MAKE: &str = "(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;\
Ljava/lang/invoke/BoundMethodHandle$SpeciesData;[Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;";

/// key 形态工厂的描述符前缀与返回（`make(MethodType, LambdaForm, …)BoundMethodHandle`）。
const MAKE_PREFIX: &str = "(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;";
const MAKE_RET: &str = ")Ljava/lang/invoke/BoundMethodHandle;";

crate::__process_static! {
    /// 动态物种登记：key（basic type 字符串，如 "LII"）→ SpeciesData。
    static REGISTRY: crate::sync_model::__RefSlot<std::collections::HashMap<std::string::String, Object>> =
        crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
}

/// `generateConcreteSpeciesCode` 登记（key → SpeciesData）。
pub fn register(key: std::string::String, species_data: Object) {
    REGISTRY.with(|r| { r.borrow_mut().insert(key, species_data); });
}

fn lookup(key: &str) -> Option<Object> {
    REGISTRY.with(|r| r.borrow().get(key).cloned())
}

/// 字段描述符 → basic type 字符（LambdaForm.BasicType 的归并：subword / boolean → I）。
fn basic_char(desc: &str) -> Option<char> {
    Some(match desc.as_bytes().first()? {
        b'L' | b'[' => 'L',
        b'I' | b'Z' | b'B' | b'S' | b'C' => 'I',
        b'J' => 'J',
        b'F' => 'F',
        b'D' => 'D',
        _ => return None,
    })
}

/// 描述符参数表拆分（顺序字段描述符）。
fn split_params(params: &str) -> Vec<&str> {
    let b = params.as_bytes();
    let (mut out, mut i) = (Vec::new(), 0usize);
    while i < b.len() {
        let start = i;
        while b[i] == b'[' {
            i += 1;
        }
        if b[i] == b'L' {
            while b[i] != b';' {
                i += 1;
            }
        }
        i += 1;
        out.push(&params[start..i]);
    }
    out
}

/// key 形态工厂描述符 → 物种 key（非本形态 / 规范描述符 → None）。
fn make_key(descriptor: &str) -> Option<std::string::String> {
    if descriptor == CANONICAL_MAKE {
        return None;
    }
    let rest = descriptor.strip_prefix(MAKE_PREFIX)?.strip_suffix(MAKE_RET)?;
    split_params(rest).into_iter().map(basic_char).collect()
}

/// `arg<T><i>` 字段名 → (basic type, 下标)。
fn arg_field(name: &str) -> Option<(char, usize)> {
    let rest = name.strip_prefix("arg")?;
    let t = rest.chars().next()?;
    if !matches!(t, 'L' | 'I' | 'J' | 'F' | 'D') {
        return None;
    }
    rest[1..].parse::<usize>().ok().map(|i| (t, i))
}

/// basic type → 字段描述符（L 形态为 Object）。
fn field_descriptor(t: char) -> &'static str {
    match t {
        'I' => "I",
        'J' => "J",
        'F' => "F",
        'D' => "D",
        _ => "Ljava/lang/Object;",
    }
}

/// MethodHandleNatives.resolve 的方法面：Species_Dyn 上 key 形态的 `make`（static）。
/// 返回 (修饰位, static, native, abstract)，与声明方法元数据同形。
pub fn method_meta(owner: &str, name: &str, descriptor: &str) -> Option<(i32, bool, bool, bool)> {
    if owner != SPECIES_DYN || name != "make" {
        return None;
    }
    make_key(descriptor).map(|_| (0x0008, true, false, false))
}

/// MethodHandleNatives.resolve 的字段面：Species_Dyn 上的 `arg<T><i>`（final 实例字段）。
/// 返回 (描述符, static, 修饰位)。
pub fn field_meta(owner: &str, name: &str) -> Option<(&'static str, bool, i32)> {
    if owner != SPECIES_DYN {
        return None;
    }
    arg_field(name).map(|(t, _)| (field_descriptor(t), false, 0x0010))
}

/// reflect_invoke 前置：key 形态 `make` → 规范工厂（登记表取 SpeciesData，其余实参装入 Object[]）。
pub fn invoke(declaring: &str, name: &str, descriptor: &str, args: &JArray<Object>) -> Option<Result<Object>> {
    if declaring != SPECIES_DYN || name != "make" {
        return None;
    }
    let key = make_key(descriptor)?;
    Some((|| {
        let Some(sd) = lookup(&key) else {
            panic!("stub: BoundMethodHandle$Species_Dyn.make 物种 {} 未登记（generateConcreteSpeciesCode 未经过）", key);
        };
        let n = args.len()?;
        let mut rest: Vec<Object> = Vec::with_capacity(n.max(2) as usize - 2);
        for i in 2..n {
            rest.push(args.get(i)?);
        }
        let canonical = JArray::from(vec![
            args.get(0)?,
            args.get(1)?,
            sd,
            Object::from(JArray::from(rest)),
        ]);
        crate::reflect_dispatch::reflect_invoke(SPECIES_DYN, "make", CANONICAL_MAKE,
                                                Object::default(), &canonical)
    })())
}

/// reflect_field 前置：`arg<T><i>` 读 → 实例 `args[i]`（只读：物种字段为 final）。
pub fn field(declaring: &str, name: &str, recv: &Object, value: &Option<Object>) -> Option<Result<Object>> {
    if declaring != SPECIES_DYN || value.is_some() {
        return None;
    }
    let (_, idx) = arg_field(name)?;
    Some((|| {
        let arr = crate::reflect_dispatch::reflect_field(SPECIES_DYN, "args", Clone::clone(recv), None)
            .unwrap_or_else(|| panic!("stub: BoundMethodHandle$Species_Dyn.args 无字段闭包"))?;
        JArray::<Object>::from(arr).get(idx as i32)
    })())
}
