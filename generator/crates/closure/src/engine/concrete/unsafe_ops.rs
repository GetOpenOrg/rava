//! Unsafe 字段偏移族的具体语义：偏移即字段身份（与运行时 `objectFieldOffset` 的不透明 id 同口径——
//! 偏移只经同一 Unsafe 的访问原语消费，数值本身不可观测），按字段身份读写实例字段。

use super::vm::*;
use super::*;

/// 字段偏移的编码基址：偏移 = 基址 + 字段键（`Vm::fkey`），与数组基址 / 下标偏移不相交
const FIELD_OFFSET_BASE: i64 = 1 << 40;

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
                vm.traced_put_field(o, &fr, arg(4)?)?;
            }
            Ok(Some(CV::I(i32::from(hit))))
        })(),
        _ => return None,
    };
    Some(r)
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
