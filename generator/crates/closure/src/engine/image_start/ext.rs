//! 引擎：构建期初始化扩展接入映像起点（计划 2026-10-05-boot-image-evaluator §5.8）。
//!
//! 类首次初始化且不在映像的构建期初始化集合中：交给保留的引导求值器尝试构建期初始化（`concrete/ext_init.rs`）。
//! 成功即追加的扩展组接入映像状态——与引导映像同一口径：类按构建期初始化处理（不展开 `<clinit>`），
//! 其静态字段取映像值，已出现的字段节点补传播。

use super::*;

impl<'a> Engine<'a> {
    /// 尝试构建期初始化 cls；返回 cls 是否成为构建期初始化类
    pub(in crate::engine) fn image_ext(&mut self, cls: &str) -> bool {
        let (Some(x), Some(s)) = (self.ext_vm.as_deref_mut(), self.img.as_deref_mut()) else { return false };
        if s.build_time.contains(cls) {
            return true;
        }
        if !s.tried.insert(cls.to_string()) {
            return false;
        }
        let d = Rc::make_mut(&mut s.data);
        let (n0, s0) = (d.objs.len(), d.statics.len());
        let Some(classes) = x.attempt(&self.ctx, self.cp, cls, d) else { return false };
        let fresh: Vec<(String, String, IVal)> = d.statics[s0..].to_vec();
        for (c, n, v) in &fresh {
            s.statics.insert((c.clone(), n.clone()), *v);
        }
        s.build_time.extend(classes.iter().cloned());
        let vals: Vec<((String, String), PV)> = fresh.iter().map(|(c, n, v)| ((c.clone(), n.clone()), self.image_pv(*v))).collect();
        if let Some(st) = self.ctx.img_statics.borrow_mut().as_mut() {
            st.build_time.extend(classes.iter().cloned());
            st.vals.extend(vals);
        }
        self.image_appended(n0);
        // 辅助分析（无方法上下文，不经 getstatic 预先定论）在尝试之前读过这些类的静态字段：记忆作废、取用者重算
        let set: HashSet<&str> = classes.iter().map(String::as_str).collect();
        let deps = self.ctx.ceval_drop(|inp| inp.reads.iter().any(|r| set.contains(r.owner.as_str())));
        if !deps.is_empty() {
            self.invalidate_all(Some(deps), Why::FieldPut);
        }
        let keys: Vec<(MemberRef, usize)> = self.fields.keys().enumerate().filter(|(_, k)| classes.contains(&k.owner)).map(|(i, k)| (k.clone(), i)).collect();
        for (k, fi) in keys {
            self.image_field(&k, fi);
        }
        classes.iter().any(|c| c == cls)
    }

    /// 方法体分析之前：其 getstatic 所读静态字段的声明类先确定构建期初始化结局。字段答复（映像值 / `<clinit>`
    /// 常量 / 缺省值）取决于声明类是否构建期初始化；结局未定时答复只能是未知，定论后变精确——格只升不降，
    /// 先前的未知已并入形参常量等汇合格无法撤回，闭包随处理次序变化。读点执行即触发声明类初始化（JVMS §5.5），
    /// 尝试只是提前；只在死代码中读到的类多一次尝试，结局与次序无关
    pub(in crate::engine) fn image_settle_reads(&mut self, code: &classfile::Code) {
        if self.ext_vm.is_none() {
            return;
        }
        for x in &code.insns {
            let (classfile::op::GETSTATIC, classfile::Operand::Field(f)) = (x.opcode, &x.operand) else { continue };
            let Some(fi) = self.ctx.field_info(f) else { continue };
            if fi.access & acc::STATIC != 0 && !self.inited.contains_key(fi.key.owner.as_str()) {
                self.image_ext(&fi.key.owner);
            }
        }
    }

    /// 追加的映像对象（下标 ≥ n0：扩展组、镜像缓存组）接入映像状态：活标记扩容、新建的类镜像登记；
    /// 程序已取过的类镜像随即成为活对象（与引导镜像在程序取镜像时成为活对象同一口径）
    pub(in crate::engine) fn image_appended(&mut self, n0: usize) {
        let Some(s) = self.img.as_deref_mut() else { return };
        s.live.resize(s.data.objs.len(), false);
        let mut taken: Vec<u32> = Vec::new();
        for (i, o) in s.data.objs.iter().enumerate().skip(n0) {
            if let Some(m) = &o.mirror {
                s.mirror_obj.insert(m.clone(), i as u32);
                if self.ids.contains_key(format!("{CLASS}#{m}").as_str()) {
                    taken.push(i as u32);
                }
            }
        }
        for o in taken {
            self.image_ref(o);
        }
        self.image_drain();
    }

    /// closure.json `summary.build_time_init`
    pub fn ext_report(&self) -> serde_json::Value {
        self.ext_vm.as_deref().map_or(serde_json::Value::Null, |x| x.report())
    }

    /// 分析结束时的映像数据：引导映像 + 扩展组（构建期初始化扩展与镜像缓存），活对象登记后规范化（按键排序重编号）
    pub fn image_final(&self) -> Option<Result<ImageData, String>> {
        let s = self.img.as_ref()?;
        let mut d = (*s.data).clone();
        d.live = self.image_live();
        Some(d.canonicalize().map(|()| d))
    }
}
