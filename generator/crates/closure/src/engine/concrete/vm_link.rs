//! 具体求值器的驻留与链接：字符串字面量、类镜像（含 Class.module）、方法解析 / 虚方法选择 / 方法信息（按引用缓存）。

use std::sync::Arc;

use resolve::hierarchy::MethodSite;

use super::vm::*;
use super::*;

impl Vm {
    // ── 字符串与类镜像 ──────────────────────────────────────────────────────

    /// 字符串字面量（按内容驻留进映像）：紧凑字符串布局——全部码元 ≤ 0xFF 时 LATIN1（coder 0），
    /// 否则 UTF16（coder 1，小端码元）
    pub(super) fn string(&mut self, env: &Env, units: &[u16]) -> R<u32> {
        if let Some(&o) = self.strings.get(units) {
            return Ok(o);
        }
        let image = self.image;
        self.image += 1;
        let r = self.make_string(env, units);
        self.image = image;
        let o = r?;
        self.strings.insert(units.to_vec(), o);
        if self.ext.is_some() {
            let key = String::from_utf16(units).map_or_else(|_| format!("s:#{}", units.iter().map(|u| format!("{u:04x}")).collect::<String>()), |s| format!("s:{s}"));
            let arr = self.get_vm_field(env, o, "string_value")?.obj()?;
            self.ext_share(&[o, arr], key);
        }
        Ok(o)
    }

    /// 求值纪元内新建的字符串（不驻留）
    pub(super) fn make_string(&mut self, env: &Env, units: &[u16]) -> R<u32> {
        let latin1 = units.iter().all(|&u| u <= 0xFF);
        let bytes: Vec<CV> = if latin1 {
            units.iter().map(|&u| CV::I(u as u8 as i8 as i32)).collect()
        } else {
            units.iter().flat_map(|&u| [CV::I((u & 0xFF) as u8 as i8 as i32), CV::I((u >> 8) as u8 as i8 as i32)]).collect()
        };
        let arr = self.alloc("[B", Body::Arr(bytes));
        self.frozen.insert(arr);
        let s = self.alloc(STRING, Body::Inst(Vec::new()));
        self.put_vm_field(env, s, "string_value", CV::R(arr))?;
        self.put_vm_field(env, s, "string_coder", CV::I(i32::from(!latin1)))?;
        Ok(s)
    }

    /// 字符串对象的 UTF-16 内容
    pub(super) fn units(&mut self, env: &Env, s: u32) -> R<Vec<u16>> {
        let arr = self.get_vm_field(env, s, "string_value")?.obj()?;
        let coder = self.get_vm_field(env, s, "string_coder")?;
        let bytes: Vec<u8> = self.arr(arr)?.iter().map(|v| v.i().map(|b| b as u8)).collect::<R<_>>()?;
        Ok(if coder == CV::I(0) {
            bytes.iter().map(|&b| b as u16).collect()
        } else {
            bytes.chunks(2).map(|c| c[0] as u16 | (c.get(1).copied().unwrap_or(0) as u16) << 8).collect()
        })
    }

    pub(super) fn rust_string(&mut self, env: &Env, s: u32) -> R<String> {
        let u = self.units(env, s)?;
        String::from_utf16(&u).map_or_else(|_| fail("字符串含孤立代理项"), Ok)
    }

    /// 类镜像（按所指类型驻留进映像；类型取 binary name / 数组描述符 / 基本类型描述符字符）
    pub(super) fn mirror(&mut self, env: &Env, t: &str) -> R<u32> {
        if let Some(&o) = self.mirrors.get(t) {
            return Ok(o);
        }
        let image = self.image;
        self.image += 1;
        let o = self.alloc(CLASS, Body::Inst(Vec::new()));
        self.image = image;
        let t: Rc<str> = Rc::from(t);
        self.mirrors.insert(t.clone(), o);
        self.mirror_of.insert(o, t.clone());
        if self.ext.is_some() {
            self.ext_share(&[o], format!("m:{t}"));
        }
        if let Some(c) = t.strip_prefix('[') {
            let ct = c.strip_prefix('L').and_then(|x| x.strip_suffix(';')).unwrap_or(c);
            let cm = self.mirror(env, ct)?;
            self.put_vm_field(env, o, "component_type", CV::R(cm))?;
        }
        // 镜像名（HotSpot `JVM_InitClassName` 的驻留结果）：映像镜像是运行期的规范镜像（零拷贝，计划 §5.10），
        // 名字随镜像入映像，运行期不再按需建名
        if (self.boot || self.ext.is_some()) && env.cfg().vm_fields.contains_key("class_name") {
            let u: Vec<u16> = super::natives::java_name(&t).encode_utf16().collect();
            let s = self.string(env, &u)?;
            self.put_vm_field(env, o, "class_name", CV::R(s))?;
        }
        if self.boot {
            self.mirror_module(env, &t, o)?;
        }
        Ok(o)
    }

    /// 类镜像的 Class.module（引导求值）：按所在包查 defineModule0 登记；数组取元素类型的模块
    pub(super) fn mirror_module(&mut self, env: &Env, t: &str, o: u32) -> R<()> {
        let base = t.trim_start_matches('[');
        let base = base.strip_prefix('L').and_then(|x| x.strip_suffix(';')).unwrap_or(base);
        let pkg = base.rsplit_once('/').map_or("", |(p, _)| p);
        let m = match self.pkg_module.get(pkg) {
            Some(&m) => m,
            // 基本类型与其数组属于基础模块（VM 规定首个 defineModule0 即基础模块）
            None if !base.contains('/') => match self.base_module {
                Some(m) => m,
                None => return Ok(()),
            },
            None => return Ok(()),
        };
        if env.cfg().vm_fields.contains_key("class_module") && self.get_vm_field(env, o, "class_module")? == CV::N {
            self.put_vm_field(env, o, "class_module", CV::R(m))?;
        }
        Ok(())
    }

    // ── 方法解析与选择（按引用缓存）──────────────────────────────────────────

    pub(super) fn resolve(&mut self, env: &Env, m: &MemberRef, iface: bool) -> R<MethodSite> {
        let k = (m.clone(), iface);
        if let Some(r) = self.mres.get(&k) {
            return r.clone().map_or_else(|| fail(format!("方法解析失败 {m}")), Ok);
        }
        let r = env.h().resolve_method(&m.owner, &m.name, &m.desc, iface);
        self.mres.insert(k, r.clone());
        r.map_or_else(|| fail(format!("方法解析失败 {m}")), Ok)
    }

    pub(super) fn select(&mut self, env: &Env, recv: &Rc<str>, site: &MethodSite) -> R<MethodSite> {
        let (o, n, d) = site.key();
        let k = (recv.clone(), MemberRef { owner: o, name: n, desc: d });
        if let Some(r) = self.sel.get(&k) {
            return r.clone().map_or_else(|| fail(format!("虚方法选择失败 {} on {recv}", k.1)), Ok);
        }
        let r = env.h().select(recv, site);
        self.sel.insert(k.clone(), r.clone());
        r.map_or_else(|| fail(format!("虚方法选择失败 {} on {recv}", k.1)), Ok)
    }

    pub(super) fn info(&mut self, env: &Env, site: &MethodSite) -> Rc<MInfo> {
        let (o, n, d) = site.key();
        let key = MemberRef { owner: o, name: n, desc: d };
        if let Some(i) = self.minfo.get(&key) {
            return i.clone();
        }
        let index = site.method().code.as_ref().map(|c| c.insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect()).unwrap_or_default();
        // 显式操作优先；其次清单的返回值事实（`[facts.returns]` / `[vm_constants] null_returns`：原生二进制里恒定的返回值）。
        // 引导求值中有字节码的方法不取返回值事实：事实描述的是进入 main 之后的值（如 `VM.isBooted` 恒 true），
        // 引导期间按字节码读档位等静态状态，与 JVM 引导期次序一致
        let ks = key.to_string();
        let boot_op = if self.boot { env.cfg().boot.natives.get(&ks).cloned() } else { None };
        let fact_ok = !self.boot || site.method().code.is_none() || self.ext.is_some();
        let op = boot_op.or_else(|| env.cfg().natives.get(&ks).cloned()).or_else(|| {
            if !fact_ok {
                return None;
            }
            env.man().return_fact(&ks).map(|f| match f {
                crate::manifest::Fact::Null => "const:null".to_string(),
                crate::manifest::Fact::Int(x) => format!("const:{x}"),
            })
        });
        // 引导求值执行 JDK 字节码本身（手写替换是运行期承载，不是构建期语义）
        let bytecode = site.method().code.is_some() && (self.boot || matches!(env.ctx.kind_of(&site.class, site.method()), Kind::Bytecode));
        let i = Rc::new(MInfo { key: key.clone(), site: site.clone(), index, op, bytecode });
        self.minfo.insert(key, i.clone());
        i
    }

    pub(super) fn class(&self, env: &Env, name: &str) -> R<Arc<ClassFile>> {
        env.h().class(name).map_or_else(|| fail(format!("类缺失 {name}")), Ok)
    }
}
