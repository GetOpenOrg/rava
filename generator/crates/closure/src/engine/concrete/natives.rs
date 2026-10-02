//! 白名单 native 的具体语义：清单 `[concrete.natives]` 把成员映射到操作名，这里只按操作名分派。
//! 类镜像相关操作的所指类型取自镜像驻留表（`Vm::mirror_of`）。

use classfile::acc;

use super::interp::{array_of, returns_void};
use super::vm::*;
use super::*;

/// HotSpot 写入 Class 修饰符的类访问标志（`JVM_ACC_WRITTEN_FLAGS`）
const WRITTEN_FLAGS: u16 = 0x7631;

/// 基本类型描述符字符 ↔ 名字（Java 语言关键字）
const PRIMS: [(&str, &str); 9] = [
    ("Z", "boolean"),
    ("B", "byte"),
    ("C", "char"),
    ("S", "short"),
    ("I", "int"),
    ("J", "long"),
    ("F", "float"),
    ("D", "double"),
    ("V", "void"),
];

fn is_prim(t: &str) -> bool {
    PRIMS.iter().any(|(d, _)| *d == t)
}

/// 镜像所指类型的 `Class.getName()` 形态
fn java_name(t: &str) -> String {
    match PRIMS.iter().find(|(d, _)| *d == t) {
        Some((_, n)) => n.to_string(),
        None => t.replace('/', "."),
    }
}

pub(super) fn call(vm: &mut Vm, env: &Env, op: &str, info: &MInfo, args: Vec<CV>) -> R<Option<CV>> {
    if vm.tracing() {
        vm.trace.natives.insert(info.key.clone());
    }
    let desc = &info.key.desc;
    let ret = |v: CV| Ok(Some(v));
    let arg = |i: usize| args.get(i).copied().map_or_else(|| fail("native 实参个数"), Ok);
    let zero = || if returns_void(desc) { None } else { desc.rsplit(')').next().map(CV::zero) };
    match op {
        "noop" => Ok(zero()),
        "const:false" | "const:0" => ret(CV::I(0)),
        "const:true" => ret(CV::I(1)),
        "const:null" => ret(CV::N),
        c if c.starts_with("const:") => match c["const:".len()..].parse::<i32>() {
            Ok(x) => ret(CV::I(x)),
            Err(_) => fail(format!("未知常量操作 {c}")),
        },
        "self" => ret(arg(0)?),
        "bytecode" => vm.run(env, &Rc::new(MInfo { key: info.key.clone(), site: info.site.clone(), index: info.index.clone(), op: None, bytecode: true }), args),
        "identity_hash" => {
            let h = arg(0)?.r()?.map_or(0, |o| vm.identity_hash(o));
            ret(CV::I(h))
        }
        "get_class" => {
            let o = arg(0)?.obj()?;
            if matches!(vm.heap[o as usize].body, Body::Lam(_)) {
                return fail("lambda 对象的类");
            }
            let t = vm.ty(o);
            ret(CV::R(vm.mirror(env, &t)?))
        }
        "clone" => {
            let o = arg(0)?.obj()?;
            let t = vm.ty(o);
            let body = match &vm.heap[o as usize].body {
                Body::Arr(v) => Body::Arr(v.clone()),
                Body::Inst(fs) => Body::Inst(fs.clone()),
                Body::Lam(_) => return fail("克隆 lambda"),
            };
            ret(CV::R(vm.alloc(&t, body)))
        }
        "arraycopy" => {
            let (src, sp, dst, dp, n) = (arg(0)?.obj()?, arg(1)?.i()?, arg(2)?.obj()?, arg(3)?.i()?, arg(4)?.i()?);
            let (st, dt) = (vm.ty(src), vm.ty(dst));
            if !st.starts_with('[') || !dt.starts_with('[') {
                return fail("arraycopy 非数组");
            }
            let refs = |t: &str| t[1..].starts_with('L') || t[1..].starts_with('[');
            if st != dt && !(refs(&st) && refs(&dt)) {
                return fail("arraycopy 元素类型不符");
            }
            let vals = vm.arr(src)?;
            if sp < 0 || dp < 0 || n < 0 || (sp as i64 + n as i64) as usize > vals.len() {
                return implicit("index");
            }
            let part: Vec<CV> = vals[sp as usize..(sp + n) as usize].to_vec();
            if st != dt {
                let ct = &dt[1..];
                let ct = ct.strip_prefix('L').and_then(|x| x.strip_suffix(';')).unwrap_or(ct);
                for v in &part {
                    if let Some(o) = v.r()? {
                        if !vm.instance_of(env, &vm.ty(o), o, ct) {
                            return fail("arraycopy 存储类型不符");
                        }
                    }
                }
            }
            let d = vm.arr_mut(dst)?;
            if (dp as i64 + n as i64) as usize > d.len() {
                return implicit("index");
            }
            d[dp as usize..(dp + n) as usize].copy_from_slice(&part);
            Ok(None)
        }
        "intern" => {
            let s = arg(0)?.obj()?;
            let u = vm.units(env, s)?;
            ret(CV::R(vm.string(env, &u)?))
        }
        "float_to_raw_int_bits" => ret(CV::I(arg(0)?.f()?.to_bits() as i32)),
        "int_bits_to_float" => ret(CV::F(f32::from_bits(arg(0)?.i()? as u32))),
        "double_to_raw_long_bits" => ret(CV::J(arg(0)?.d()?.to_bits() as i64)),
        "long_bits_to_double" => ret(CV::D(f64::from_bits(arg(0)?.j()? as u64))),
        "primitive_class" => {
            let n = vm.rust_string(env, arg(0)?.obj()?)?;
            let Some((d, _)) = PRIMS.iter().find(|(_, x)| *x == n) else { return fail("未知基本类型名") };
            ret(CV::R(vm.mirror(env, d)?))
        }
        "class_for_name" => {
            let n = vm.rust_string(env, arg(0)?.obj()?)?.replace('.', "/");
            let exists = if let Some(c) = n.strip_prefix('[') {
                let base = c.trim_start_matches('[');
                is_prim(base) || base.strip_prefix('L').and_then(|b| b.strip_suffix(';')).is_some_and(|b| env.h().class(b).is_some())
            } else {
                env.h().class(&n).is_some()
            };
            if !exists {
                return fail(format!("Class.forName 找不到类 {n}"));
            }
            if arg(1)? == CV::I(1) {
                vm.ensure_init(env, &n)?;
            }
            ret(CV::R(vm.mirror(env, &n)?))
        }
        "caller_class" => {
            let n = vm.frames.len();
            let Some(c) = n.checked_sub(2).and_then(|i| vm.frames.get(i)) else { return fail("调用方帧缺失") };
            let c = c.owner.clone();
            ret(CV::R(vm.mirror(env, &c)?))
        }
        "new_array" => {
            let t = mirror_type(vm, arg(0)?)?;
            if &*t == "V" {
                return fail("void 数组");
            }
            ret(CV::R(vm.new_array(&array_of(&t), arg(1)?.i()?)?))
        }
        _ => class_op(vm, env, op, &args),
    }
}

fn mirror_type(vm: &Vm, v: CV) -> R<Rc<str>> {
    let o = v.obj()?;
    vm.mirror_of.get(&o).cloned().map_or_else(|| fail("非类镜像"), Ok)
}

fn class_op(vm: &mut Vm, env: &Env, op: &str, args: &[CV]) -> R<Option<CV>> {
    let this = args.first().copied().map_or_else(|| fail("native 实参个数"), Ok)?;
    let t = mirror_type(vm, this)?;
    let arr = t.starts_with('[');
    let prim = is_prim(&t);
    let cf = if arr || prim { None } else { Some(vm.class(env, &t)?) };
    let b = |x: bool| Ok(Some(CV::I(i32::from(x))));
    match op {
        "class_name" => {
            let u: Vec<u16> = java_name(&t).encode_utf16().collect();
            Ok(Some(CV::R(vm.make_string(env, &u)?)))
        }
        "class_is_interface" => b(cf.as_ref().is_some_and(|c| c.is_interface())),
        "class_is_array" => b(arr),
        "class_is_primitive" => b(prim),
        "class_modifiers" => {
            let m = match &cf {
                Some(c) => match c.inner_classes.iter().find(|i| i.inner == c.name) {
                    Some(i) => i.access & !acc::SUPER,
                    None => c.access & WRITTEN_FLAGS & !acc::SUPER,
                },
                None => return fail("数组 / 基本类型的修饰符"),
            };
            Ok(Some(CV::I(m as i32)))
        }
        "class_superclass" => {
            let s = match &cf {
                Some(c) if !c.is_interface() => c.super_name.clone(),
                Some(_) => None,
                None if arr => Some(OBJECT.to_string()),
                None => None,
            };
            Ok(Some(match s {
                Some(s) => CV::R(vm.mirror(env, &s)?),
                None => CV::N,
            }))
        }
        "class_interfaces" => {
            let Some(c) = &cf else { return fail("数组 / 基本类型的接口") };
            let names = c.interfaces.clone();
            let a = vm.new_array(&array_of(CLASS), names.len() as i32)?;
            for (i, n) in names.iter().enumerate() {
                let m = vm.mirror(env, n)?;
                vm.arr_mut(a)?[i] = CV::R(m);
            }
            Ok(Some(CV::R(a)))
        }
        "class_signature" => {
            let sig = cf.as_ref().and_then(|c| c.signature.clone());
            Ok(Some(match sig {
                Some(s) => CV::R(vm.make_string(env, &s.encode_utf16().collect::<Vec<_>>())?),
                None => CV::N,
            }))
        }
        "class_declaring_class" => {
            let outer = cf.as_ref().and_then(|c| c.inner_classes.iter().find(|i| i.inner == c.name).and_then(|i| i.outer.clone()));
            Ok(Some(match outer {
                Some(o) => CV::R(vm.mirror(env, &o)?),
                None => CV::N,
            }))
        }
        "class_enclosing_method" => {
            let Some((c, m)) = cf.as_ref().and_then(|c| c.enclosing_method.clone()) else { return Ok(Some(CV::N)) };
            let a = vm.new_array(&array_of(OBJECT), 3)?;
            let cm = vm.mirror(env, &c)?;
            let (n, d) = match m {
                Some((n, d)) => {
                    let n = vm.make_string(env, &n.encode_utf16().collect::<Vec<_>>())?;
                    let d = vm.make_string(env, &d.encode_utf16().collect::<Vec<_>>())?;
                    (CV::R(n), CV::R(d))
                }
                None => (CV::N, CV::N),
            };
            vm.arr_mut(a)?.copy_from_slice(&[CV::R(cm), n, d]);
            Ok(Some(CV::R(a)))
        }
        "class_is_instance" => {
            let Some(o) = args.get(1).copied().map_or_else(|| fail("native 实参个数"), Ok)?.r()? else { return b(false) };
            if prim {
                return b(false);
            }
            let ty = vm.ty(o);
            b(vm.instance_of(env, &ty, o, &t))
        }
        "class_is_assignable_from" => {
            let other = mirror_type(vm, args.get(1).copied().map_or_else(|| fail("native 实参个数"), Ok)?)?;
            b(if prim || is_prim(&other) { t == other } else { env.h().is_subtype(&other, &t) })
        }
        _ => fail(format!("未知 native 操作 {op}")),
    }
}
