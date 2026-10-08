//! 引擎：静态字段的初值（计划 2026-10-05-boot-image-evaluator §5.5.6 机制 ②「静态字段非空折叠」）。
//!
//! 静态字段的常量格 = 初值 ⊔ 档案内全部可达 `putstatic` 写入值（`Ctx::fvals`）：
//! - 构建期初始化类（映像 `build_time`）：运行期从映像状态开始，`<clinit>` 不再执行，初值是映像值
//!   （映像只导出非缺省值，缺席即缺省值；污点 / 占位值为 Top）；
//! - 其余类：初值是缺省值（`<clinit>` 写入之前的读、从未写入的字段）。
//!
//! 写入值按「非空」入格（[`write_pv`]）：确定非空、无标签的引用记为非空引用，于是映像值非空、且全部可达写入
//! 都非空时，读到的值恒非空，`ifnull` / `ifnonnull` 按非空折叠（空分支不可达）。只依赖 JDK 侧事实（映像值 +
//! 档案内写点集合）；写入来源超出字节码的字段（反射 / Unsafe / VarHandle / 手写写入）由 `field_open` 先行排除。
//! 规则只看字段与映像，不看类名。

use super::*;
use crate::image::IVal;

/// 映像中构建期初始化类的静态字段初值（装入映像时由 `install_image` 建立）
#[derive(Default)]
pub(super) struct ImgStatics {
    pub(super) build_time: HashSet<String>,
    /// （声明类, 字段名）→ 映像值的常量格；缺席 = 缺省值
    pub(super) vals: HashMap<(String, String), PV>,
}

impl ImgStatics {
    /// 构建期初始化类的静态字段初值；None = 声明类不在构建期初始化
    pub(super) fn initial(&self, key: &MemberRef) -> Option<PV> {
        if !self.build_time.contains(&key.owner) {
            return None;
        }
        Some(self.vals.get(&(key.owner.clone(), key.name.clone())).cloned().unwrap_or_else(|| default_pv(&key.desc)))
    }
}

/// 静态字段的初值（见模块说明）
pub(super) fn static_initial(img: Option<&ImgStatics>, key: &MemberRef) -> PV {
    img.and_then(|s| s.initial(key)).unwrap_or_else(|| default_pv(&key.desc))
}

/// 字段写入值入常量格：静态字段的确定非空、无标签引用记为非空引用（[`PV::of_ret`]），其余同 [`PV::of`]。
/// 实例字段不取非空引用：非字节码分配（`allocateInstance` / 反序列化等）的对象不并入缺省值
pub(super) fn write_pv(is_static: bool, v: Option<&V>) -> PV {
    match v {
        None => PV::Top,
        Some(v) if is_static => PV::of_ret(v),
        Some(v) => PV::of(v),
    }
}

impl Ctx<'_> {
    /// 静态字段的初值（映像中构建期初始化类取映像值）
    pub(super) fn static_initial(&self, key: &MemberRef) -> PV {
        static_initial(self.img_statics.borrow().as_ref(), key)
    }

    /// 构建期初始化类的 static final 引用字段：`<clinit>` 推不出常量时取映像值（映像对象 / 字符串 / null；
    /// 污点与占位为 None）。`putstatic` 只能在 `<clinit>` 中写 final 字段，而 `<clinit>` 已在构建期执行完
    pub(super) fn image_final(&self, key: &MemberRef) -> Option<V> {
        if !matches!(key.desc.as_bytes().first(), Some(b'L' | b'[')) {
            return None;
        }
        self.img_statics.borrow().as_ref()?.initial(key)?.value()
    }
}

impl Engine<'_> {
    /// 建立构建期初始化类的静态字段初值表（`install_image` 登记映像状态后调用）
    pub(super) fn image_statics_install(&mut self, statics: &HashMap<(String, String), IVal>, build_time: &[String]) {
        let build_time: HashSet<String> = build_time.iter().cloned().collect();
        let mut vals = HashMap::default();
        for ((c, n), v) in statics {
            if build_time.contains(c) {
                vals.insert((c.clone(), n.clone()), self.image_pv(*v));
            }
        }
        *self.ctx.img_statics.borrow_mut() = Some(ImgStatics { build_time, vals });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(owner: &str, desc: &str) -> MemberRef {
        MemberRef { owner: owner.into(), name: "f".into(), desc: desc.into() }
    }

    fn tagged(ty: &str) -> V {
        V::Ref { ty: Some(Rc::from(ty)), nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Fields(vec![]))) }
    }

    fn fresh(ty: &str) -> V {
        V::Ref { ty: Some(Rc::from(ty)), nonnull: true, src: Rc::from([].as_slice()), obj: None }
    }

    fn img() -> ImgStatics {
        let mut s = ImgStatics::default();
        s.build_time.insert("p/B".into());
        s.vals.insert(("p/B".into(), "f".into()), PV::Const(tagged("p/X")));
        s
    }

    fn fold(init: PV, writes: &[Option<V>]) -> PV {
        writes.iter().fold(init, |acc, w| PV::join(Some(&acc), &write_pv(true, w.as_ref())))
    }

    /// 映像值非空、全部可达写入非空（新建对象 / 另一标签对象）：值恒非空
    #[test]
    fn image_nonnull_and_nonnull_writes_stay_nonnull() {
        let s = img();
        let k = key("p/B", "Lp/X;");
        let init = static_initial(Some(&s), &k);
        assert_eq!(init.value().and_then(|v| v.nonnull()), Some(true));
        let v = fold(init, &[Some(fresh("p/X")), Some(tagged("p/Y"))]);
        assert_eq!(v.value().and_then(|v| v.nonnull()), Some(true));
    }

    /// 任一可达写入可空（null / 未知）：不折叠
    #[test]
    fn nullable_write_breaks_nonnull() {
        let s = img();
        let k = key("p/B", "Lp/X;");
        for w in [Some(V::Null), Some(V::Ref { ty: Some(Rc::from("p/X")), nonnull: false, src: Rc::from([].as_slice()), obj: None }), None] {
            let v = fold(static_initial(Some(&s), &k), &[Some(fresh("p/X")), w]);
            assert_ne!(v.value().and_then(|v| v.nonnull()), Some(true));
        }
    }

    /// 非构建期初始化类：初值为缺省 null，写入非空也不折叠；映像中缺席的构建期字段即缺省值
    #[test]
    fn default_initial_outside_image() {
        let s = img();
        let v = fold(static_initial(Some(&s), &key("p/C", "Lp/X;")), &[Some(fresh("p/X"))]);
        assert_ne!(v.value().and_then(|v| v.nonnull()), Some(true));
        assert_eq!(static_initial(None, &key("p/B", "Lp/X;")), PV::Const(V::Null));
        let mut s = img();
        s.vals.clear();
        assert_eq!(static_initial(Some(&s), &key("p/B", "I")), PV::Const(V::Int(0)));
    }

    /// 基本类型静态字段：构建期初始化类取映像值（而非缺省 0），与可达写入取并
    #[test]
    fn primitive_image_value_is_initial() {
        let mut s = ImgStatics::default();
        s.build_time.insert("p/B".into());
        s.vals.insert(("p/B".into(), "f".into()), PV::Const(V::Int(4)));
        let k = key("p/B", "I");
        assert_eq!(static_initial(Some(&s), &k), PV::Const(V::Int(4)));
        let v = PV::join(Some(&static_initial(Some(&s), &k)), &write_pv(true, Some(&V::Int(4))));
        assert_eq!(v, PV::Const(V::Int(4)));
    }

    /// 不同标签的两个非空对象、不同字符串常量合流：仍是非空引用；与 null 合流：可空
    #[test]
    fn distinct_nonnull_values_join_nonnull() {
        let empty = V::Ref { ty: Some(Rc::from("p/X")), nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Empty)) };
        let j = PV::join(Some(&PV::Const(tagged("p/X"))), &PV::Const(empty));
        assert_eq!(j.value().and_then(|v| v.nonnull()), Some(true));
        let j = PV::join(Some(&PV::Const(V::Str(Rc::from("a"), Default::default()))), &PV::Const(V::Str(Rc::from("b"), Default::default())));
        assert_eq!(j.value().and_then(|v| v.nonnull()), Some(true));
        let j = PV::join(Some(&PV::Const(V::Null)), &PV::Const(V::Str(Rc::from("b"), Default::default())));
        assert_ne!(j.value().and_then(|v| v.nonnull()), Some(true));
    }

    /// 实例字段写入不取非空引用（保持 `PV::of` 口径）
    #[test]
    fn instance_writes_keep_plain_lattice() {
        assert_eq!(write_pv(false, Some(&fresh("p/X"))), PV::Top);
        assert_eq!(write_pv(true, Some(&fresh("p/X"))), PV::Const(super::super::facts::nonnull_ref()));
    }
}
