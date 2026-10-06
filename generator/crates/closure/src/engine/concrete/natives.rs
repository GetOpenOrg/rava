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
        c if c.starts_with("const:") => match c["const:".len()..].parse::<i64>() {
            Ok(x) if desc.ends_with(")J") => ret(CV::J(x)),
            Ok(x) => ret(CV::I(x as i32)),
            Err(_) => fail(format!("未知常量操作 {c}")),
        },
        "self" => ret(arg(0)?),
        "arg1" => ret(arg(1)?),
        // VM 持有的单例（如 Unsafe.getUnsafe 的实例）：返回类型的唯一映像对象，无实例字段状态
        "vm_singleton" => {
            let Some(t) = desc.rsplit(')').next().and_then(|d| d.strip_prefix('L')).and_then(|d| d.strip_suffix(';')) else {
                return fail("vm_singleton 返回类型非类");
            };
            if let Some(&o) = vm.singletons.get(t) {
                return ret(CV::R(o));
            }
            let image = vm.image;
            vm.image += 1;
            let o = vm.alloc(t, Body::Inst(Vec::new()));
            vm.image = image;
            vm.singletons.insert(Rc::from(t), o);
            ret(CV::R(o))
        }
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
        // ── 构建期引导求值专用操作 ──
        // 运行期副作用（信号、线程启动、OS 环境）：登记为运行期按序重放的 native；有返回值的即宿主相关值
        "defer" => {
            if !desc.ends_with(")V") {
                return defer(format!("延迟值参与求值：宿主相关的返回值 {}", info.key));
            }
            vm.push_rec(env, super::journal::Rec::Native { callee: info.key.clone(), args: args.clone(), ph: None });
            Ok(None)
        }
        // 宿主标量（机器资源 / 描述符状态）：返回污点值（`host_scalar:<下界>:<上界>`，计划 §3.2），
        // 写入映像的位置物化为启动重算槽（concrete/taint.rs）
        h if h == "host_scalar" || h.starts_with("host_scalar:") => {
            let caller = vm.frames.last().map_or_else(String::new, |f| f.to_string());
            vm.bj.host_scalars.insert((info.key.to_string(), caller));
            let r = desc.rsplit(')').next().unwrap_or("I");
            ret(vm.tsrc(&info.key.to_string(), args.clone(), r, h)?)
        }
        "boot_current_thread" => match env.cfg().boot.current_thread.and_then(|i| vm.boot_objs.get(i)) {
            Some(&o) => ret(CV::R(o)),
            None => fail("初始线程尚未构造"),
        },
        // 构建期无回收：引用对象按强引用语义读其所指（VM 布局字段 `reference_referent`）
        "refers_to" => {
            let o = arg(0)?.obj()?;
            let cur = ref_field(vm, env, o)?;
            ret(CV::I(i32::from(cur == arg(1)?)))
        }
        "clear_referent" => {
            let o = arg(0)?.obj()?;
            let spec = env.cfg().vm_fields.get("reference_referent").cloned().map_or_else(|| fail("清单缺 VM 布局字段 reference_referent"), Ok)?;
            let (owner, name) = spec.rsplit_once('.').unwrap_or((&spec, ""));
            let fr = vm.field_res(env, &MemberRef { owner: owner.into(), name: name.into(), desc: "Ljava/lang/Object;".into() })?;
            vm.put_field(o, &fr, CV::N)?;
            Ok(None)
        }
        // Unsafe.ensureClassInitialized0 / shouldBeInitialized0（实参 0 为 Unsafe 接收者，1 为镜像）
        "unsafe_ensure_init" => {
            let t = mirror_type(vm, arg(1)?)?;
            if !t.starts_with('[') && !is_prim(&t) {
                vm.ensure_init(env, &t)?;
            }
            Ok(None)
        }
        "unsafe_should_be_init" => {
            let t = mirror_type(vm, arg(1)?)?;
            ret(CV::I(i32::from(!t.starts_with('[') && !is_prim(&t) && !matches!(vm.init.get(&t), Some(Init::Done)))))
        }
        // 宿主相关的返回值（文件系统查询等）：字符串为内容延迟的非空串，其余即宿主相关值参与求值
        "defer_value" => {
            if desc.ends_with(")Ljava/lang/String;") {
                let k = info.key.to_string();
                return Ok(Some(boot_string(vm, env, &k, "@deferred", Some((&k, None)))?));
            }
            defer(format!("延迟值参与求值：宿主相关的返回值 {}", info.key))
        }
        // 延迟调用：结果依赖宿主（如当前目录），构建期只登记调用、返回非空占位对象；占位对象只许被存放、
        // 判空，读写其状态、比较身份即「延迟值参与求值」。运行期重放该调用得到真值（清单登记即承诺非空）
        "defer_call" => {
            let Some(t) = desc.rsplit(')').next().and_then(|d| d.strip_prefix('L')).and_then(|d| d.strip_suffix(';')) else {
                return fail("defer_call 返回类型非类");
            };
            let o = vm.alloc(t, Body::Inst(Vec::new()));
            vm.mark_placeholder(o, &format!("延迟调用 {}", info.key));
            vm.bj.nonnull.insert(o);
            vm.push_rec(env, super::journal::Rec::Native { callee: info.key.clone(), args: args.clone(), ph: Some(o) });
            ret(CV::R(o))
        }
        // VM 侧状态登记（模块定义、导出、读边等）：构建期记入 VM 表，物化为运行期 VM 表的初值
        "vm_record" => {
            vm.bj.vm_effects += 1;
            *vm.vm_tables.entry(info.key.name.clone()).or_default() += 1;
            // defineModule0(Module, isOpen, version, location, String[] 包名)：包 → 模块登记，
            // 已有与此后新建的类镜像按包填 Class.module（HotSpot 的 java.base 修补与 create_mirror 同义）
            if info.key.name == "defineModule0" {
                let m = arg(0)?.obj()?;
                vm.base_module.get_or_insert(m);
                if let Some(pns) = args.last().copied().and_then(|v| v.r().ok().flatten()) {
                    let names: Vec<CV> = vm.arr(pns)?.clone();
                    *vm.vm_tables.entry("packages".into()).or_default() += names.len();
                    for n in names {
                        let p = vm.rust_string(env, n.obj()?)?.replace('.', "/");
                        vm.pkg_module.insert(Rc::from(p.as_str()), m);
                    }
                }
                let mut ms: Vec<(Rc<str>, u32)> = vm.mirrors.iter().map(|(t, &o)| (t.clone(), o)).collect();
                ms.sort_unstable();
                for (t, o) in ms {
                    vm.mirror_module(env, &t, o)?;
                }
            }
            Ok(zero())
        }
        // 向文件描述符写出是运行期副作用：所在的根帧调用 / 区段残差化
        "boot_write" => defer(format!("延迟值参与求值：构建期输出 {}", info.key)),
        "props:vm" => {
            let kv: Vec<(String, String)> = env.cfg().boot.vm_props.clone();
            let a = vm.new_array(&array_of(STRING), (kv.len() * 2) as i32)?;
            let src = info.key.to_string();
            for (i, (k, v)) in kv.iter().enumerate() {
                let ko = boot_string(vm, env, k, k, None)?;
                let vo = boot_string(vm, env, k, v, Some((&src, Some(2 * i as u32 + 1))))?;
                vm.arr_mut(a)?[2 * i] = ko;
                vm.arr_mut(a)?[2 * i + 1] = vo;
            }
            ret(CV::R(a))
        }
        // 平台属性按名给出：下标取自 native 所在类的 `_<名>_NDX` 常量（各 JDK 版本的下标不同），
        // 数组长度 = 最大下标 + 1；清单未给的名字为 null，清单有而本版本无的名字不出现
        "props:platform" => {
            let cf = info.site.class.clone();
            let ndx: HashMap<&str, usize> = cf
                .fields
                .iter()
                .filter_map(|f| {
                    let n = f.name.strip_prefix('_')?.strip_suffix("_NDX")?;
                    match f.constant_value {
                        Some(Const::Int(i)) if i >= 0 => Some((n, i as usize)),
                        _ => None,
                    }
                })
                .collect();
            let Some(len) = ndx.values().max().map(|m| m + 1) else { return fail(format!("{} 无 _<名>_NDX 下标常量", cf.name)) };
            let a = vm.new_array(&array_of(STRING), len as i32)?;
            let vs: Vec<(String, String)> = env.cfg().boot.platform_props.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            let src = info.key.to_string();
            for (k, v) in vs {
                let Some(&i) = ndx.get(k.as_str()) else { continue };
                let o = boot_string(vm, env, &k, &v, Some((&src, Some(i as u32))))?;
                vm.arr_mut(a)?[i] = o;
            }
            ret(CV::R(a))
        }
        // 数组元素 stride（目标布局常量，与运行时 vm_constants::array_index_scale 同口径）
        "array_index_scale" => {
            let t = mirror_type(vm, arg(1)?)?;
            ret(CV::I(match t.as_bytes().get(1) {
                Some(b'Z' | b'B') => 1,
                Some(b'C' | b'S') => 2,
                Some(b'J' | b'D') => 8,
                _ => 4,
            }))
        }
        s if s.starts_with("set_static:") => {
            let spec = &s["set_static:".len()..];
            let (owner, name) = spec.rsplit_once('.').map_or_else(|| fail("set_static 操作数"), Ok)?;
            let key = vm.fkey(owner, name);
            vm.jlog_static(key);
            vm.statics.insert(key, arg(0)?);
            Ok(None)
        }
        _ => match super::unsafe_ops::call(vm, env, op, &args) {
            Some(r) => r,
            None => class_op(vm, env, op, &args),
        },
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
        // Reflection.getClassAccessFlags(Class)：类文件 access_flags（实参 0 即镜像）
        "class_access_flags" => match &cf {
            Some(c) => Ok(Some(CV::I((c.access & WRITTEN_FLAGS) as i32))),
            None => fail("数组 / 基本类型的访问标志"),
        },
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

/// 引导属性值：`@null` = null，`@deferred` = 宿主相关（内容数组登记为延迟值），否则为字面量。
/// `src`：宿主值的运行期来源（native 键，结果数组下标），随内容数组导出（`IObj::host`）
fn boot_string(vm: &mut Vm, env: &Env, key: &str, v: &str, src: Option<(&str, Option<u32>)>) -> R<CV> {
    match v {
        "@null" => Ok(CV::N),
        "@deferred" => {
            let s = vm.make_string(env, &format!("<{key}>").encode_utf16().collect::<Vec<_>>())?;
            let a = vm.get_vm_field(env, s, "string_value")?.obj()?;
            vm.deferred.insert(a, Rc::from(key));
            if let Some((n, i)) = src {
                vm.host_src.insert(a, (Rc::from(n), i));
            }
            Ok(CV::R(s))
        }
        "@jdk_feature" => {
            let f = vm.jdk_feature(env)?.to_string();
            Ok(CV::R(vm.make_string(env, &f.encode_utf16().collect::<Vec<_>>())?))
        }
        _ => Ok(CV::R(vm.make_string(env, &v.encode_utf16().collect::<Vec<_>>())?)),
    }
}

fn ref_field(vm: &mut Vm, env: &Env, o: u32) -> R<CV> {
    let spec = env.cfg().vm_fields.get("reference_referent").cloned().map_or_else(|| fail("清单缺 VM 布局字段 reference_referent"), Ok)?;
    let (owner, name) = spec.rsplit_once('.').unwrap_or((&spec, ""));
    let fr = vm.field_res(env, &MemberRef { owner: owner.into(), name: name.into(), desc: "Ljava/lang/Object;".into() })?;
    vm.get_field(env, o, &fr)
}
