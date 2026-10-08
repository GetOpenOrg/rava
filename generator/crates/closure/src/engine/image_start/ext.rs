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
        let keys: Vec<(MemberRef, usize)> = self.fields.keys().enumerate().filter(|(_, k)| classes.contains(&k.owner)).map(|(i, k)| (k.clone(), i)).collect();
        for (k, fi) in keys {
            self.image_field(&k, fi);
        }
        classes.iter().any(|c| c == cls)
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
