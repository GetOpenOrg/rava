//! Unsafe 字段偏移族的具体语义：偏移即字段身份（与运行时 `objectFieldOffset` 的不透明 id 同口径——
//! 偏移只经同一 Unsafe 的访问原语消费，数值本身不可观测），按字段身份读写实例字段。

use super::vm::*;
use super::*;

/// 字段偏移的编码基址：偏移 = 基址 + 字段键（`Vm::fkey`），与数组基址 / 下标偏移不相交
const FIELD_OFFSET_BASE: i64 = 1 << 40;
/// VM 原生单元（如线程 id 计数器）的地址编码基址：null 基址 + 该区间偏移即按单元读写；
/// 物化时单元终值成为运行期原生单元的初值（地址本身是运行期重定位值，不进映像）
const VM_CELL_BASE: i64 = 1 << 48;

/// 构建期编码的重定位值（字段偏移 / VM 单元地址）：映像导出时转为运行期口径的重定位槽
pub(super) fn reloc_of(vm: &Vm, v: CV) -> Option<crate::image::IReloc> {
    let CV::J(x) = v else { return None };
    if x >= VM_CELL_BASE {
        let off = x - VM_CELL_BASE;
        let i = usize::try_from(off / 8).ok()?;
        return (off % 8 == 0).then(|| vm.cells.get(i)).flatten().map(|(n, _)| crate::image::IReloc::Cell(n.to_string()));
    }
    if x >= FIELD_OFFSET_BASE {
        let k = usize::try_from(x - FIELD_OFFSET_BASE).ok()?;
        return vm.fnames.get(k).map(|(d, n)| crate::image::IReloc::FieldOffset(d.to_string(), n.to_string()));
    }
    None
}

pub(super) fn call(vm: &mut Vm, env: &Env, op: &str, args: &[CV]) -> Option<R<Option<CV>>> {
    let arg = |i: usize| args.get(i).copied().map_or_else(|| fail("native 实参个数"), Ok);
    let r = match op {
        // objectFieldOffset(Class, String)：按名在类及其超类中找实例字段（找不到时 JDK 抛 InternalError）
        "field_offset" => (|| {
            let c = arg(1)?.obj()?;
            let Some(t) = vm.mirror_of.get(&c).cloned() else { return fail("非类镜像") };
            let name = vm.rust_string(env, arg(2)?.obj()?)?;
            let mut cur = env.h().class(&t);
            while let Some(cf) = cur {
                if cf.fields.iter().any(|f| !f.is_static() && f.name == name) {
                    let k = vm.fkey(&cf.name, &name);
                    return Ok(Some(CV::J(FIELD_OFFSET_BASE + i64::from(k))));
                }
                cur = cf.super_name.as_deref().and_then(|s| env.h().class(s));
            }
            fail(format!("objectFieldOffset 找不到字段 {t}.{name}"))
        })(),
        // compareAndSetReference(o, offset, expected, x)：引用相等时写入
        "cas_reference" => (|| {
            let o = arg(1)?.obj()?;
            let fr = field_at(vm, env, arg(2)?.j()?)?;
            let cur = vm.get_field(env, o, &fr)?;
            let hit = cur == arg(3)?;
            if hit {
                vm.traced_put_field(env, o, &fr, arg(4)?)?;
            }
            Ok(Some(CV::I(i32::from(hit))))
        })(),
        _ => match op.split_once(':') {
            Some(("vm_cell", name)) => {
                let i = match vm.cells.iter().position(|(n, _)| &**n == name) {
                    Some(i) => i,
                    None => {
                        vm.cells.push((Rc::from(name), 0));
                        vm.cells.len() - 1
                    }
                };
                Ok(Some(CV::J(VM_CELL_BASE + i as i64 * 8)))
            }
            Some(("unsafe_get", k)) => (|| Ok(Some(mem_get(vm, env, arg(1)?, arg(2)?.j()?, k)?)))(),
            Some(("unsafe_put", k)) => (|| {
                mem_put(vm, env, arg(1)?, arg(2)?.j()?, k, arg(3)?)?;
                Ok(None)
            })(),
            // compareAndSet / compareAndExchange（`unsafe_cas` 返回是否命中，`unsafe_cax` 返回旧值）
            Some((cas @ ("unsafe_cas" | "unsafe_cax"), k)) => (|| {
                let (o, off) = (arg(1)?, arg(2)?.j()?);
                let wide = matches!(k, "J" | "D");
                let (exp, x) = if wide { (arg(3)?, arg(4)?) } else { (arg(3)?, arg(4)?) };
                let cur = mem_get(vm, env, o, off, k)?;
                if cur != exp && (matches!(cur, CV::T(..)) || matches!(exp, CV::T(..))) {
                    return defer("延迟值参与求值：宿主标量（污点）参与 CAS 比较");
                }
                let hit = cur == exp;
                if hit {
                    mem_put(vm, env, o, off, k, x)?;
                }
                Ok(Some(if cas == "unsafe_cas" { CV::I(i32::from(hit)) } else { cur }))
            })(),
            _ => return None,
        },
    };
    Some(r)
}

/// 访问宽度（字节）
fn width(k: &str) -> usize {
    match k {
        "Z" | "B" => 1,
        "C" | "S" => 2,
        "J" | "D" => 8,
        "L" => 4,
        _ => 4,
    }
}

/// 数组元素宽度（按数组描述符）
fn elem_width(ty: &str) -> usize {
    width(&ty[1..2])
}

/// 偏移 → 数组下标（基址与运行时 `vm_constants::array_base_offset` 同口径）
fn arr_index(off: i64, w: usize) -> R<usize> {
    let rel = off - 16;
    if rel < 0 || rel % w as i64 != 0 {
        return fail(format!("非对齐数组偏移 {off}"));
    }
    Ok((rel / w as i64) as usize)
}

/// 取值的位模式（整数类）
fn bits(v: CV) -> R<i64> {
    Ok(match v {
        CV::I(x) => i64::from(x),
        CV::J(x) => x,
        CV::F(x) => i64::from(x.to_bits()),
        CV::D(x) => x.to_bits() as i64,
        CV::T(..) => return defer("延迟值参与求值：宿主标量（污点）按位写入"),
        _ => return fail("非数值"),
    })
}

fn of_bits(k: &str, b: i64) -> CV {
    match k {
        "J" => CV::J(b),
        "F" => CV::F(f32::from_bits(b as u32)),
        "D" => CV::D(f64::from_bits(b as u64)),
        "Z" => CV::I((b & 1) as i32),
        "B" => CV::I(b as i8 as i32),
        "S" => CV::I(b as i16 as i32),
        "C" => CV::I(b as u16 as i32),
        _ => CV::I(b as i32),
    }
}

fn cell(vm: &Vm, o: CV, off: i64) -> R<Option<usize>> {
    if o != CV::N || off < VM_CELL_BASE {
        return Ok(None);
    }
    let i = ((off - VM_CELL_BASE) / 8) as usize;
    if i >= vm.cells.len() {
        return fail(format!("未分配的 VM 单元 {off}"));
    }
    Ok(Some(i))
}

fn mem_get(vm: &mut Vm, env: &Env, o: CV, off: i64, k: &str) -> R<CV> {
    if let Some(i) = cell(vm, o, off)? {
        if vm.bj.dirty_cells.contains(&i) {
            return defer(format!("延迟值参与求值：运行期重放会改写的 VM 单元 {}", vm.cells[i].0));
        }
        vm.war_read(super::war::Loc::C(i as u32));
        return Ok(of_bits(k, vm.cells[i].1));
    }
    let o = o.obj()?;
    let ty = vm.ty(o);
    if ty.starts_with('[') {
        let ew = elem_width(&ty);
        let w = width(k);
        if k == "L" || ew == w {
            let i = arr_index(off, ew)?;
            let a = vm.arr(o)?;
            return a.get(i).copied().map_or_else(|| fail("Unsafe 数组越界"), Ok);
        }
        if ew == 1 {
            let i = arr_index(off, 1)?;
            let a = vm.arr(o)?;
            let mut b: i64 = 0;
            for j in (0..w).rev() {
                b = (b << 8) | (a.get(i + j).copied().map_or_else(|| fail("Unsafe 数组越界"), Ok)?.i()? as i64 & 0xFF);
            }
            return Ok(of_bits(k, b));
        }
        return fail(format!("Unsafe 跨宽度读 {ty} as {k}"));
    }
    let fr = field_at(vm, env, off)?;
    vm.get_field(env, o, &fr)
}

fn mem_put(vm: &mut Vm, env: &Env, o: CV, off: i64, k: &str, v: CV) -> R<()> {
    if let Some(i) = cell(vm, o, off)? {
        let old = vm.cells[i].1;
        let b = bits(v)?;
        vm.jlog(super::journal::JEnt::Cell(i, old));
        vm.war_write(super::war::Loc::C(i as u32));
        vm.cells[i].1 = b;
        return Ok(());
    }
    let o = o.obj()?;
    let ty = vm.ty(o);
    if ty.starts_with('[') {
        let ew = elem_width(&ty);
        let w = width(k);
        if k == "L" || ew == w {
            let i = arr_index(off, ew)?;
            let a = vm.arr_mut(o)?;
            let slot = a.get_mut(i).map_or_else(|| fail("Unsafe 数组越界"), Ok)?;
            *slot = v;
            return Ok(());
        }
        if ew == 1 {
            let i = arr_index(off, 1)?;
            let b = bits(v)?;
            let a = vm.arr_mut(o)?;
            for j in 0..w {
                let slot = a.get_mut(i + j).map_or_else(|| fail("Unsafe 数组越界"), Ok)?;
                *slot = CV::I(((b >> (8 * j)) & 0xFF) as u8 as i8 as i32);
            }
            return Ok(());
        }
        return fail(format!("Unsafe 跨宽度写 {ty} as {k}"));
    }
    let fr = field_at(vm, env, off)?;
    vm.traced_put_field(env, o, &fr, v)
}

/// 偏移所指字段的解析结果
fn field_at(vm: &mut Vm, env: &Env, off: i64) -> R<Rc<FRes>> {
    let k = off - FIELD_OFFSET_BASE;
    let Some((decl, name)) = usize::try_from(k).ok().and_then(|k| vm.fnames.get(k)).cloned() else {
        return fail(format!("非字段偏移 {off}"));
    };
    let Some(desc) = env.h().class(&decl).and_then(|c| c.fields.iter().find(|f| *f.name == *name).map(|f| f.desc.clone())) else {
        return fail(format!("字段缺失 {decl}.{name}"));
    };
    vm.field_res(env, &MemberRef { owner: decl.to_string(), name: name.to_string(), desc })
}
