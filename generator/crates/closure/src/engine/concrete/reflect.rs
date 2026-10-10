//! 反射 native 的具体语义（HotSpot `Reflection::new_method` / `Reflection::invoke_method` 同义）：
//! - `class_declared_methods`：`getDeclaredMethods0(publicOnly)`——本类声明的方法（不含 `<init>` / `<clinit>`），
//!   按类文件声明序；反射对象不经构造器，VM 直接填写其布局字段（清单 `[concrete.vm_fields]` 的 `method_*`），
//!   `slot` 为类文件方法表下标；注解三项取类文件属性原始字节（无 → null）；
//! - `class_declared_constructors`：`getDeclaredConstructors0(publicOnly)`——同上，只取 `<init>`（布局字段 `ctor_*`）；
//! - `class_constant_pool`：`getConstantPool()`——新建常量池对象，`constant_pool_oop` 指向所属类镜像；
//! - `reflect_invoke`：本地访问器 `invoke0(Method, obj, args)`——按 (声明类, slot) 取方法，静态方法先初始化
//!   声明类，实例方法除私有外按接收者类型虚选择。实参 / 返回值只支持引用类型（基本类型的装箱拆箱不建模，
//!   求值失败）；目标抛出异常时 HotSpot 包装为 InvocationTargetException，这里不建模、求值失败；
//! - `reflect_new`：本地访问器 `newInstance0(Constructor, args)`——按 (声明类, slot) 取构造器，初始化声明类、
//!   分配实例并执行 `<init>`（HotSpot `Reflection::invoke_constructor`）；实参与异常的建模口径同 `reflect_invoke`。
//!
//! 构建期产生的反射对象经类镜像的反射数据缓存（软引用）可达；导出映像时软引用按「可随时清除」语义清除
//! （`export.rs`），不入映像。

use classfile::acc;

use super::interp::array_of;
use super::vm::*;
use super::*;

/// HotSpot 写入 Method.modifiers 的方法访问标志（`JVM_RECOGNIZED_METHOD_MODIFIERS`）
const METHOD_MODIFIERS: u16 = 0x1DFF;

/// 字段描述符 → 类镜像驻留键（基本类型取描述符字符，类取内部名，数组原样）
fn mirror_key(d: &str) -> &str {
    d.strip_prefix('L').and_then(|x| x.strip_suffix(';')).unwrap_or(d)
}

/// 方法描述符的形参描述符与返回描述符
fn split_desc(desc: &str) -> R<(Vec<&str>, &str)> {
    let Some((ps, ret)) = desc.strip_prefix('(').and_then(|d| d.split_once(')')) else { return fail(format!("方法描述符 {desc}")) };
    let b = ps.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let s = i;
        while b[i] == b'[' {
            i += 1;
        }
        if b[i] == b'L' {
            while b[i] != b';' {
                i += 1;
            }
        }
        i += 1;
        out.push(&ps[s..i]);
    }
    Ok((out, ret))
}

fn is_ref(d: &str) -> bool {
    d.starts_with('L') || d.starts_with('[')
}

impl Vm {
    fn mirror_array(&mut self, env: &Env, ds: &[&str]) -> R<CV> {
        let a = self.new_array(&array_of(CLASS), ds.len() as i32)?;
        for (i, d) in ds.iter().enumerate() {
            let m = self.mirror(env, mirror_key(d))?;
            self.arr_mut(a)?[i] = CV::R(m);
        }
        Ok(CV::R(a))
    }

    fn byte_array(&mut self, bytes: &[u8]) -> CV {
        if bytes.is_empty() {
            return CV::N;
        }
        CV::R(self.alloc("[B", Body::Arr(bytes.iter().map(|&b| CV::I(b as i8 as i32)).collect())))
    }

    fn java_string(&mut self, env: &Env, s: &str) -> R<CV> {
        Ok(CV::R(self.string(env, &s.encode_utf16().collect::<Vec<_>>())?))
    }
}

pub(super) fn call(vm: &mut Vm, env: &Env, op: &str, info: &MInfo, args: &[CV]) -> Option<R<Option<CV>>> {
    Some(match op {
        "class_declared_methods" => declared_members(vm, env, info, args, "method"),
        "class_declared_constructors" => declared_members(vm, env, info, args, "ctor"),
        "class_constant_pool" => constant_pool(vm, env, info, args),
        "reflect_invoke" => invoke(vm, env, args),
        "reflect_new" => construct(vm, env, args),
        _ => return None,
    })
}

fn arg(args: &[CV], i: usize) -> R<CV> {
    args.get(i).copied().map_or_else(|| fail("native 实参个数"), Ok)
}

/// 返回类型（类或类的数组）的元素类内部名
fn ret_class(desc: &str) -> R<&str> {
    let r = desc.rsplit(')').next().unwrap_or("");
    let r = r.trim_start_matches('[');
    r.strip_prefix('L').and_then(|x| x.strip_suffix(';')).map_or_else(|| fail(format!("返回类型非类 {desc}")), Ok)
}

/// `getDeclaredMethods0` / `getDeclaredConstructors0`：`kind` 为布局字段键前缀（`method` / `ctor`），
/// 构造器只取 `<init>`、不写名字 / 返回类型 / 注解默认值
fn declared_members(vm: &mut Vm, env: &Env, info: &MInfo, args: &[CV], kind: &str) -> R<Option<CV>> {
    let ctor = kind == "ctor";
    let this = arg(args, 0)?;
    let public_only = arg(args, 1)?.i()? != 0;
    let t = vm.mirror_of.get(&this.obj()?).cloned().map_or_else(|| fail("非类镜像"), Ok)?;
    let mty = ret_class(&info.key.desc)?.to_string();
    let arr_ty = array_of(&mty);
    if t.starts_with('[') || t.len() == 1 {
        return Ok(Some(CV::R(vm.new_array(&arr_ty, 0)?)));
    }
    let cf = vm.class(env, &t)?;
    let extras = match env.cp.bytes(&t).map(|b| classfile::extras::parse_extras(&b)) {
        Some(Ok(x)) if x.methods.len() == cf.methods.len() => x,
        _ => return fail(format!("类文件注解属性不可得 {t}")),
    };
    vm.ensure_init(env, &mty)?;
    let mut out = Vec::new();
    for (slot, m) in cf.methods.iter().enumerate() {
        if (m.name == "<init>") != ctor || m.name == "<clinit>" || public_only && m.access & acc::PUBLIC == 0 {
            continue;
        }
        let (ps, ret) = split_desc(&m.desc)?;
        let o = vm.alloc(&mty, Body::Inst(Vec::new()));
        let mut fs = vec![
            ("clazz", this),
            ("parameter_types", vm.mirror_array(env, &ps)?),
            ("exception_types", {
                let ex: Vec<String> = m.exceptions.iter().map(|e| format!("L{e};")).collect();
                vm.mirror_array(env, &ex.iter().map(String::as_str).collect::<Vec<_>>())?
            }),
            ("modifiers", CV::I((m.access & METHOD_MODIFIERS) as i32)),
            ("slot", CV::I(slot as i32)),
            ("signature", match &m.signature {
                Some(s) => vm.java_string(env, s)?,
                None => CV::N,
            }),
            ("annotations", vm.byte_array(&extras.methods[slot].raw_annotations)),
            ("parameter_annotations", vm.byte_array(&extras.methods[slot].raw_param_annotations)),
        ];
        if !ctor {
            fs.push(("name", vm.java_string(env, &m.name)?));
            fs.push(("return_type", CV::R(vm.mirror(env, mirror_key(ret))?)));
            fs.push(("annotation_default", vm.byte_array(&extras.methods[slot].raw_annotation_default)));
        }
        for (k, v) in fs {
            vm.put_vm_field(env, o, &format!("{kind}_{k}"), v)?;
        }
        out.push(CV::R(o));
    }
    let a = vm.new_array(&arr_ty, out.len() as i32)?;
    vm.arr_mut(a)?.copy_from_slice(&out);
    Ok(Some(CV::R(a)))
}

fn constant_pool(vm: &mut Vm, env: &Env, info: &MInfo, args: &[CV]) -> R<Option<CV>> {
    let this = arg(args, 0)?;
    let ty = ret_class(&info.key.desc)?.to_string();
    vm.ensure_init(env, &ty)?;
    let o = vm.alloc(&ty, Body::Inst(Vec::new()));
    vm.put_vm_field(env, o, "constant_pool_oop", this)?;
    Ok(Some(CV::R(o)))
}

fn invoke(vm: &mut Vm, env: &Env, args: &[CV]) -> R<Option<CV>> {
    let m = arg(args, 0)?.obj()?;
    let clazz = vm.get_vm_field(env, m, "method_clazz")?.obj()?;
    let slot = vm.get_vm_field(env, m, "method_slot")?.i()?;
    let t = vm.mirror_of.get(&clazz).cloned().map_or_else(|| fail("非类镜像"), Ok)?;
    let cf = vm.class(env, &t)?;
    let Some(meth) = usize::try_from(slot).ok().and_then(|i| cf.methods.get(i)) else { return fail(format!("反射方法 slot 越界 {t}#{slot}")) };
    let (name, desc, is_static, private) = (meth.name.clone(), meth.desc.clone(), meth.is_static(), meth.is_private());
    let (ps, ret) = split_desc(&desc)?;
    if !ps.iter().all(|p| is_ref(p)) || !(is_ref(ret) || ret == "V") {
        return fail(format!("反射调用的基本类型实参 / 返回值 {t}.{name}{desc}"));
    }
    let actual: Vec<CV> = match arg(args, 2)?.r()? {
        Some(a) => vm.arr(a)?.clone(),
        None => Vec::new(),
    };
    if actual.len() != ps.len() {
        return fail(format!("反射调用实参个数 {t}.{name}{desc}"));
    }
    for (v, p) in actual.iter().zip(&ps) {
        if let Some(o) = v.r()? {
            let ty = vm.ty(o);
            if !vm.instance_of(env, &ty, o, mirror_key(p)) {
                return fail(format!("反射调用实参类型不符 {t}.{name}{desc}"));
            }
        }
    }
    let mref = MemberRef { owner: t.to_string(), name, desc };
    let resolved = vm.resolve(env, &mref, cf.is_interface())?;
    let mut call_args = Vec::with_capacity(actual.len() + 1);
    let target = if is_static {
        vm.ensure_init(env, &t)?;
        resolved
    } else {
        let Some(recv) = arg(args, 1)?.r()? else { return implicit("null") };
        let ty = vm.ty(recv);
        if !vm.instance_of(env, &ty, recv, &t) {
            return fail(format!("反射调用接收者类型不符 {}", mref));
        }
        call_args.push(CV::R(recv));
        if private { resolved } else { vm.select(env, &ty, &resolved)? }
    };
    call_args.extend(actual);
    match vm.call(env, &target, call_args) {
        Ok(r) => Ok(Some(r.unwrap_or(CV::N))),
        Err(Flow::Throw(_) | Flow::Implicit(_)) => fail(format!("反射调用目标抛出异常（InvocationTargetException 包装未建模）{mref}")),
        Err(e) => Err(e),
    }
}

/// 反射调用的引用实参：实参数组按形参类型核对（基本类型形参不建模）
fn ref_args(vm: &Vm, env: &Env, arr: CV, ps: &[&str], what: &str) -> R<Vec<CV>> {
    if !ps.iter().all(|p| is_ref(p)) {
        return fail(format!("反射调用的基本类型实参 {what}"));
    }
    let actual: Vec<CV> = match arr.r()? {
        Some(a) => vm.arr(a)?.clone(),
        None => Vec::new(),
    };
    if actual.len() != ps.len() {
        return fail(format!("反射调用实参个数 {what}"));
    }
    for (v, p) in actual.iter().zip(ps) {
        vm.check_identity(*v)?;
        if let Some(o) = v.r()? {
            if !vm.instance_of(env, &vm.ty(o), o, mirror_key(p)) {
                return fail(format!("反射调用实参类型不符 {what}"));
            }
        }
    }
    Ok(actual)
}

fn construct(vm: &mut Vm, env: &Env, args: &[CV]) -> R<Option<CV>> {
    let c = arg(args, 0)?.obj()?;
    let clazz = vm.get_vm_field(env, c, "ctor_clazz")?.obj()?;
    let slot = vm.get_vm_field(env, c, "ctor_slot")?.i()?;
    let t = vm.mirror_of.get(&clazz).cloned().map_or_else(|| fail("非类镜像"), Ok)?;
    let cf = vm.class(env, &t)?;
    let Some(meth) = usize::try_from(slot).ok().and_then(|i| cf.methods.get(i)).filter(|m| m.name == "<init>") else {
        return fail(format!("反射构造器 slot 越界 {t}#{slot}"));
    };
    let desc = meth.desc.clone();
    if cf.is_interface() || cf.is_abstract() {
        return fail(format!("反射实例化抽象类 {t}"));
    }
    let mref = MemberRef { owner: t.to_string(), name: "<init>".into(), desc };
    let (ps, _) = split_desc(&mref.desc)?;
    let actual = ref_args(vm, env, arg(args, 1)?, &ps, &mref.to_string())?;
    vm.ensure_init(env, &t)?;
    let target = vm.resolve(env, &mref, false)?;
    let o = vm.alloc(&t, Body::Inst(Vec::new()));
    let mut call_args = Vec::with_capacity(actual.len() + 1);
    call_args.push(CV::R(o));
    call_args.extend(actual);
    match vm.call(env, &target, call_args) {
        Ok(_) => Ok(Some(CV::R(o))),
        Err(Flow::Throw(_) | Flow::Implicit(_)) => fail(format!("反射构造目标抛出异常（InvocationTargetException 包装未建模）{mref}")),
        Err(e) => Err(e),
    }
}
