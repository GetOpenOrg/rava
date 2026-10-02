//! invokedynamic：lambda（LambdaMetafactory）与字符串拼接（StringConcatFactory）。
//! 其余引导方法（switch 引导、record 对象方法等）求值失败。

use std::sync::Arc;

use super::interp::nparams;
use super::vm::*;
use super::*;

/// altMetafactory 标志位（`LambdaMetafactory.FLAG_*`）
const FLAG_SERIALIZABLE: i32 = 1;
const FLAG_MARKERS: i32 = 2;
/// 拼接配方中的实参 / 常量占位符
const TAG_ARG: u16 = 1;
const TAG_CONST: u16 = 2;

/// 方法描述符的形参描述符与返回描述符
fn split_desc(desc: &str) -> (Vec<&str>, &str) {
    let b = desc.as_bytes();
    let mut out = Vec::new();
    let mut i = 1;
    while i < b.len() && b[i] != b')' {
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
        out.push(&desc[s..i]);
    }
    (out, &desc[i + 1..])
}

fn is_ref(d: &str) -> bool {
    d.starts_with('L') || d.starts_with('[')
}

impl Vm {
    pub(super) fn indy(&mut self, env: &Env, cf: &Arc<ClassFile>, bsm: u16, name: &str, desc: &str, args: Vec<CV>) -> R<CV> {
        let Some(bm) = cf.bootstrap_methods.get(bsm as usize) else { return fail("引导方法下标") };
        let bkey = format!("{}.{}", bm.handle.member.owner, bm.handle.member.name);
        match env.man().indy_kind(&bkey) {
            Some(IndyKind::Lambda) => self.make_lambda(name, desc, &bm.args, args),
            Some(IndyKind::Concat) => self.concat(env, desc, &bm.args, args),
            _ => fail(format!("不支持的引导方法 {bkey}")),
        }
    }

    fn make_lambda(&mut self, name: &str, desc: &str, bargs: &[Const], captured: Vec<CV>) -> R<CV> {
        let (Some(Const::MethodType(_)), Some(Const::MethodHandle(imp)), Some(Const::MethodType(_))) = (bargs.first(), bargs.get(1), bargs.get(2)) else {
            return fail("lambda 引导实参");
        };
        let (_, ret) = split_desc(desc);
        let Some(iface) = ret.strip_prefix('L').and_then(|r| r.strip_suffix(';')) else { return fail("lambda 接口类型") };
        let mut markers = Vec::new();
        if let Some(Const::Int(flags)) = bargs.get(3) {
            if flags & FLAG_SERIALIZABLE != 0 {
                return fail("可序列化 lambda");
            }
            if flags & FLAG_MARKERS != 0 {
                let Some(Const::Int(n)) = bargs.get(4) else { return fail("lambda 标记接口计数") };
                for k in 0..*n as usize {
                    match bargs.get(5 + k) {
                        Some(Const::Class(c)) => markers.push(c.clone()),
                        _ => return fail("lambda 标记接口"),
                    }
                }
            }
        }
        let lam = Lam { iface: iface.to_string(), markers, sam: name.to_string(), imp: imp.clone(), captured };
        Ok(CV::R(self.alloc(iface, Body::Lam(Rc::new(lam)))))
    }

    /// 调用 lambda 对象上的接口方法：SAM → 实现方法；默认方法 → 按接口选择；其余（Object 方法）失败
    pub(super) fn call_lambda(&mut self, env: &Env, l: &Lam, resolved: &MethodSite, args: Vec<CV>) -> R<Option<CV>> {
        let rm = resolved.method();
        if !rm.is_abstract() {
            if !resolved.class.is_interface() {
                return fail("lambda 对象上的 Object 方法");
            }
            let iface: Rc<str> = Rc::from(l.iface.as_str());
            let site = self.select(env, &iface, resolved)?;
            return self.call(env, &site, args);
        }
        if rm.name != l.sam {
            return fail(format!("lambda 上调用非 SAM 方法 {}", rm.name));
        }
        let mut rest = l.captured.clone();
        rest.extend_from_slice(&args[1..]);
        let imp = &l.imp.member;
        let (mut ptys, iret) = split_desc(&imp.desc);
        if matches!(l.imp.kind, 5 | 7 | 9) {
            ptys.insert(0, "L;");
        }
        let (_, sret) = split_desc(&rm.desc);
        if ptys.len() != rest.len() {
            return fail("lambda 实参个数");
        }
        for (p, v) in ptys.iter().zip(&rest) {
            if is_ref(p) != matches!(v, CV::N | CV::R(_)) {
                return fail("lambda 实参需装箱适配");
            }
        }
        let site = self.resolve(env, imp, l.imp.interface)?;
        let r = match l.imp.kind {
            6 => {
                self.ensure_init(env, &site.class.name)?;
                self.call(env, &site, rest)?
            }
            7 => self.call(env, &site, rest)?,
            5 | 9 => {
                let recv = rest[0].obj()?;
                if matches!(self.heap[recv as usize].body, Body::Lam(_)) {
                    return fail("lambda 实现方法的接收者是 lambda");
                }
                let ty = self.ty(recv);
                let t = self.select(env, &ty, &site)?;
                self.call(env, &t, rest)?
            }
            8 => {
                self.ensure_init(env, &imp.owner)?;
                let o = self.alloc(&imp.owner, Body::Inst(Vec::new()));
                rest.insert(0, CV::R(o));
                self.call(env, &site, rest)?;
                Some(CV::R(o))
            }
            k => return fail(format!("lambda 实现句柄种类 {k}")),
        };
        let iret = if l.imp.kind == 8 { "L;" } else { iret };
        match (r, sret == "V") {
            (_, true) => Ok(None),
            (Some(v), false) if is_ref(iret) == is_ref(sret) => Ok(Some(v)),
            (Some(v), false) if !is_ref(iret) && is_ref(sret) => self.box_value(env, iret, v).map(Some),
            _ => fail("lambda 返回值需拆箱适配"),
        }
    }

    /// 基本类型值装箱（`[boxing]` 装箱类的 `valueOf`）
    fn box_value(&mut self, env: &Env, prim: &str, v: CV) -> R<CV> {
        let Some(cls) = env.man().boxed_class(prim.as_bytes()[0]) else { return fail("无装箱类") };
        let m = MemberRef { owner: cls.to_string(), name: "valueOf".into(), desc: format!("({prim})L{cls};") };
        let site = self.resolve(env, &m, false)?;
        self.ensure_init(env, cls)?;
        self.call(env, &site, vec![v])?.map_or_else(|| fail("装箱无返回值"), Ok)
    }

    fn concat(&mut self, env: &Env, desc: &str, bargs: &[Const], args: Vec<CV>) -> R<CV> {
        let (ptys, _) = split_desc(desc);
        let recipe: Vec<u16> = match bargs.first() {
            Some(Const::String(s)) => s.encode_utf16().collect(),
            Some(Const::StringUtf16(u)) => u.clone(),
            None => vec![TAG_ARG; nparams(desc)],
            _ => return fail("拼接配方"),
        };
        let (mut ai, mut ci) = (0, 1);
        let mut out: Vec<u16> = Vec::new();
        for &u in &recipe {
            match u {
                TAG_ARG => {
                    let (Some(p), Some(v)) = (ptys.get(ai), args.get(ai)) else { return fail("拼接实参") };
                    ai += 1;
                    let s = self.stringify(env, p, *v)?;
                    out.extend(s);
                }
                TAG_CONST => {
                    match bargs.get(ci) {
                        Some(Const::String(s)) => out.extend(s.encode_utf16()),
                        Some(Const::Int(i)) => out.extend(i.to_string().encode_utf16()),
                        _ => return fail("拼接常量"),
                    }
                    ci += 1;
                }
                c => out.push(c),
            }
        }
        Ok(CV::R(self.make_string(env, &out)?))
    }

    /// `String.valueOf` 语义的字符串化（浮点格式化不建模）
    fn stringify(&mut self, env: &Env, p: &str, v: CV) -> R<Vec<u16>> {
        let s = match (p, v) {
            ("Z", CV::I(x)) => (if x != 0 { "true" } else { "false" }).to_string(),
            ("C", CV::I(x)) => return Ok(vec![x as u16]),
            (_, CV::I(x)) => x.to_string(),
            (_, CV::J(x)) => x.to_string(),
            (_, CV::N) => "null".to_string(),
            (_, CV::R(o)) => {
                if &*self.ty(o) == STRING {
                    return self.units(env, o);
                }
                if matches!(self.heap[o as usize].body, Body::Lam(_)) {
                    return fail("lambda 对象字符串化");
                }
                let m = MemberRef { owner: OBJECT.into(), name: TO_STRING.0.into(), desc: TO_STRING.1.into() };
                let site = self.resolve(env, &m, false)?;
                let ty = self.ty(o);
                let t = self.select(env, &ty, &site)?;
                return match self.call(env, &t, vec![v])? {
                    Some(CV::R(s)) => self.units(env, s),
                    _ => Ok("null".encode_utf16().collect()),
                };
            }
            _ => return fail("浮点字符串化"),
        };
        Ok(s.encode_utf16().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::split_desc;

    #[test]
    fn splits_descriptor() {
        let (p, r) = split_desc("(I[JLa/B;[[La/C;)La/D;");
        assert_eq!(p, ["I", "[J", "La/B;", "[[La/C;"]);
        assert_eq!(r, "La/D;");
    }
}
