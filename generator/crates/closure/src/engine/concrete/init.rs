//! 类初始化（JVMS §5.5）与隐式异常对象。

use super::vm::*;
use super::*;

impl Vm {
    /// 类初始化（JVMS §5.5：超类、声明默认方法的超接口先于本类；`<clinit>` 在映像纪元执行）
    pub(super) fn ensure_init(&mut self, env: &Env, c: &str) -> R<()> {
        if c.starts_with('[') {
            return Ok(());
        }
        if self.tracing() {
            self.trace.touched.insert(c.to_string());
        }
        match self.init.get(c) {
            Some(Init::Done | Init::Running) => return self.ext_inited(c),
            Some(Init::Failed(w)) => return fail(format!("类初始化失败 {c}：{w}")),
            None => {}
        }
        let key: Rc<str> = Rc::from(c);
        if self.boot {
            if self.ext.is_some() {
                return self.ext_init(env, key);
            }
            return self.boot_init(env, key);
        }
        self.init.insert(key.clone(), Init::Running);
        let (floor, foreign, done) = (self.heap.len(), self.foreign, self.done_log.len());
        self.clinit_floor.push(floor);
        let mut r = self.do_init(env, c);
        self.clinit_floor.pop();
        // 初始化中途失败（不可建模；含占位对象参与求值）且未改写此前已有的映像状态：本类与期间完成初始化的类改为静态不可读，
        // 部分执行留下的对象只经这些类的静态字段可达
        if matches!(r, Err(Flow::Fail(_) | Flow::Defer(_))) && self.foreign == foreign {
            self.opaque.insert(key.clone());
            let later: Vec<Rc<str>> = self.done_log[done..].to_vec();
            self.opaque.extend(later);
            r = Ok(());
        }
        if r.is_ok() {
            self.done_log.push(key.clone());
        }
        self.init.insert(key, match &r {
            Ok(()) => Init::Done,
            Err(Flow::Fail(w)) => Init::Failed(Rc::from(w.as_str())),
            Err(Flow::Throw(o)) => Init::Failed(Rc::from(format!("抛出 {}", self.ty(*o)).as_str())),
            Err(Flow::Implicit(k)) => Init::Failed(Rc::from(format!("隐式异常 {k}").as_str())),
            Err(Flow::Defer(w)) => Init::Failed(Rc::from(w.as_str())),
        });
        if r.is_ok() && self.tracing() {
            self.trace.inited.insert(c.to_string());
        }
        r.map_or_else(|e| match e {
            Flow::Fail(w) | Flow::Defer(w) => fail(format!("类初始化失败 {c}：{w}")),
            Flow::Throw(_) | Flow::Implicit(_) => fail(format!("类初始化抛出异常 {c}")),
        }, Ok)
    }

    /// 引导求值的类初始化：`<clinit>` 在日志标记内执行；延迟值参与求值即撤回其全部效果，该类转为
    /// 运行期初始化（静态字段构建期不可读）。其余失败与异常即构建失败
    pub(super) fn boot_init(&mut self, env: &Env, key: Rc<str>) -> R<()> {
        self.jlog_init(&key);
        self.init.insert(key.clone(), Init::Running);
        let m = self.jmark();
        self.bj.clinits.push(key.clone());
        let r = self.do_init(env, &key);
        self.bj.clinits.pop();
        match r {
            Ok(()) => {
                self.jpop(m);
                self.done_log.push(key.clone());
                self.init.insert(key, Init::Done);
                Ok(())
            }
            Err(Flow::Defer(w)) => {
                self.jrollback_for(m, Some(&key))?;
                self.fail_frames = None;
                self.mark_opaque(key.clone());
                let why = w.split(" @ ").next().unwrap_or(&w).to_string();
                self.bj.rt_attempts.push((key.clone(), why.clone()));
                self.push_rec(env, super::journal::Rec::RuntimeInit { class: key.clone(), why });
                self.war_capture(m.rl(), m.heap());
                self.init.insert(key, Init::Done);
                Ok(())
            }
            Err(f) => {
                self.jpop(m);
                let w = match &f {
                    Flow::Fail(w) | Flow::Defer(w) => w.clone(),
                    Flow::Throw(o) => format!("抛出 {}", self.ty(*o)),
                    Flow::Implicit(k) => format!("隐式异常 {k}"),
                };
                self.init.insert(key.clone(), Init::Failed(Rc::from(w.as_str())));
                fail(format!("类初始化失败 {key}：{w}"))
            }
        }
    }

    fn do_init(&mut self, env: &Env, c: &str) -> R<()> {
        let cf = self.class(env, c)?;
        if !cf.is_interface() {
            if let Some(s) = &cf.super_name {
                self.ensure_init(env, s)?;
            }
            for i in env.h().all_superinterfaces(&cf) {
                if i.methods.iter().any(|m| !m.is_abstract() && !m.is_static()) {
                    self.ensure_init(env, &i.name)?;
                }
            }
        }
        let Some(idx) = cf.methods.iter().position(|m| m.is_clinit()) else { return Ok(()) };
        let site = MethodSite { class: cf, index: idx };
        let info = self.info(env, &site);
        // 静态状态由 VM / 手写层承载的类：初始化不执行，其静态字段不可读
        if info.op.as_deref() == Some("opaque") {
            self.mark_opaque(Rc::from(c));
            return Ok(());
        }
        if !info.bytecode && info.op.is_none() {
            return fail(format!("类初始化器无字节码语义 {c}"));
        }
        self.image += 1;
        let r = self.call(env, &site, Vec::new());
        self.image -= 1;
        r.map(|_| ())
    }

    /// 隐式异常（`Flow::Implicit` 的种类）的异常对象：类型取自清单 `[concrete.implicit]`，按 VM 方式分配
    /// （不执行构造器：消息由 VM 惰性给出）
    pub(super) fn implicit(&mut self, env: &Env, kind: &str) -> R<u32> {
        let Some(t) = env.cfg().implicit.get(kind) else { return fail(format!("清单未声明隐式异常 {kind}")) };
        self.ensure_init(env, t)?;
        Ok(self.alloc(t, Body::Inst(Vec::new())))
    }
}
