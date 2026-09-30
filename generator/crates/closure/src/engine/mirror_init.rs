//! 引擎：按类镜像强制类初始化（`[facts.reflect] class_initializers`，如 `Unsafe.ensureClassInitialized`）。
//!
//! JDK 以「初始化某类以运行其 `<clinit>` 的副作用」登记跨包访问器（`SharedSecrets.javaUtilJarAccess()`：
//! `ensureClassInitialized(JarFile.class)` 后读 `javaUtilJarAccess`，由 `JarFile.<clinit>` 写入）。
//! 不建模时 `<clinit>` 的写入缺席，字段按初值 null 折叠——属未建模写入来源。
//!
//! 两类调用点：
//! - 一般调用点：Class 实参所指类（类字面量、值集里的类镜像）按 JVMS §5.5 初始化；值集含推不出所指类的
//!   镜像记为反射缺口。
//! - 成员声明类初始化点（调用方登记在 `member_owner_initializers`：方法句柄 / VarHandle / 反射访问器链接
//!   静态成员或构造器时初始化其声明类）：Class 实参恒为正被链接的成员（getStatic / putStatic / invokeStatic /
//!   newInvokeSpecial）的声明类。结构不变量「静态方法 / 构造器可达 ⇒ 声明类初始化」（JVMS §5.5 invokestatic /
//!   new，`member_owner_init`，任何进入路径都成立）与「按反射 / 句柄取得静态字段 ⇒ 声明类初始化」
//!   （字段句柄入口，engine/field_names.rs）已覆盖这些类的 `<clinit>`，分析上无需按值集求目标——
//!   值集是 MemberName.clazz 一类跨全部成员的汇合，按它求目标只会引入无关类。运行期钩子目标取按反射 /
//!   方法句柄链接到的成员声明类（`linked_owners`），不取值集。

use super::*;

/// 经反射 / 方法句柄链接成员的溯源种类 → 可能送达的链接路径（与 reflect.rs / lambda.rs / field_names.rs 的
/// Via 种类一致；按名取字段的入口同时服务句柄与核心反射，两路都记）
fn link_routes(kind: &str) -> &'static [LinkRoute] {
    match kind {
        "method-handle" => &[LinkRoute::Handle],
        "reflect" => &[LinkRoute::Reflect],
        "field-name" => &[LinkRoute::Handle, LinkRoute::Reflect],
        _ => &[],
    }
}

impl Engine<'_> {
    pub(super) fn mirror_init_site(&mut self, m: usize, off: u32, mref: &MemberRef, k: &str, opcode: u8, args: &[V]) {
        if !self.man.is_class_initializer(k) {
            return;
        }
        if let Some(route) = self.man.member_owner_route(&self.methods[m].key.to_string()) {
            if self.seeds.live_routes.insert(route) {
                let owners: Vec<String> =
                    self.seeds.linked_owners.iter().filter(|(r, _)| *r == route).map(|(_, c)| c.clone()).collect();
                self.seeds.mirror_inits.extend(owners);
            }
            return;
        }
        let Some(md) = parse_method(&mref.desc) else { return };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let mut targets: Vec<String> = vec![];
        for (p, a) in md.params.iter().zip(args.iter().skip(skip)) {
            if matches!(p, FieldType::Object(c) if c == CLASS) {
                targets.extend(self.mirror_classes(m, off, k, a));
            }
        }
        for c in targets {
            if !c.starts_with('[') {
                self.seeds.mirror_inits.insert(c.clone());
            }
            self.init(&c, Via::method("mirror-init", m, Some(off)));
        }
    }

    /// 方法边经反射 / 方法句柄链接（每条边都判：首个到达路径可能是普通调用）：静态方法 / 构造器的声明类
    /// 记为成员声明类初始化点的运行期目标（分析上的初始化由 method_ctx 的不变量完成）
    pub(super) fn linked_member(&mut self, key: &MemberRef, via: &Via) {
        let routes = link_routes(via.kind);
        if routes.iter().all(|r| self.seeds.linked_owners.contains(&(*r, key.owner.clone()))) {
            return;
        }
        // 反射暴露的方法（任意种类）由 reflect.rs 的 expose 初始化声明类；句柄链接只对静态方法 / 构造器初始化
        let owner_init = via.kind == "reflect"
            || key.name == "<init>"
            || self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).map(|mm| mm.is_static())).unwrap_or(false);
        if owner_init && key.name != "<clinit>" {
            self.link_owner(&key.owner, routes);
        }
    }

    /// 按反射 / 句柄取得静态字段（字段名入口解析到的静态字段、ldc 静态字段句柄）：声明类初始化（JVMS §5.5
    /// getstatic / putstatic 的等价路径），并记为运行期目标
    pub(super) fn static_field_owner(&mut self, decl: &str, via: Via) {
        let routes = link_routes(via.kind);
        self.init(decl, via);
        self.link_owner(decl, routes);
    }

    /// 字段枚举（Field 句柄数组）：接收者镜像所指类及其超类中声明静态字段的类按静态字段句柄处理；
    /// 推不出所指类的镜像记为反射缺口
    pub(super) fn enumerated_static_owners(&mut self, m: usize, off: u32, k: &str, recv: &V) {
        let classes = self.mirror_classes(m, off, k, recv);
        let via = Via::method("field-name", m, Some(off));
        for c in classes {
            let mut cur = Some(c);
            while let Some(cls) = cur {
                let Some(cf) = self.h.class(&cls) else { break };
                if cf.fields.iter().any(|f| f.access & acc::STATIC != 0 && f.constant_value.is_none()) {
                    self.static_field_owner(&cls, via.clone());
                }
                cur = cf.super_name.clone();
            }
        }
    }

    /// Class 值所指的类：类字面量，或值集里类镜像所指的类；推不出所指类的镜像记为本站点的反射缺口
    pub(super) fn mirror_classes(&mut self, m: usize, off: u32, k: &str, v: &V) -> Vec<String> {
        self.mirror_classes_of(m, off, k, v, true).0
    }

    /// 同 [`Self::mirror_classes`]，另报所指类是否齐全（值集里没有推不出的镜像 / open）。
    /// `record_gaps = false`：调用方对不齐全另有兜底（按名静态字段的闭包扫描），不记反射缺口
    pub(super) fn mirror_classes_of(&mut self, m: usize, off: u32, k: &str, v: &V, record_gaps: bool) -> (Vec<String>, bool) {
        if let V::Class(c, _) = v {
            return (vec![c.to_string()], true);
        }
        let class = self.id(CLASS);
        let fs = self.feeds(m, v, class);
        let s = self.value_set(&fs);
        let site = format!("{k} @ {}@{off}", self.methods[m].key);
        let mut out = vec![];
        let mut complete = true;
        for x in s.classes.iter() {
            match self.mirrors.get(&x) {
                Some(&c) => out.push(self.names[c as usize].to_string()),
                None => {
                    complete = false;
                    if record_gaps {
                        let what = self.names[x as usize].to_string();
                        self.reflect_gaps.insert(format!("{site} <- {what}"));
                    }
                }
            }
        }
        for o in &s.open {
            complete = false;
            if record_gaps {
                self.reflect_gaps.insert(format!("{site} <- open({})", self.names[o as usize]));
            }
        }
        (out, complete)
    }

    /// 按名取静态字段、所属类推不出（Class 值集含 open / 推不出的镜像）：运行期镜像只能指向闭包里的类，
    /// 闭包中声明该名静态字段的类都按静态字段句柄处理（补种阶段逐轮扫描新增的类）
    pub(super) fn static_owner_name_open(&mut self, name: &str) {
        self.seeds.owner_names.entry(name.to_string()).or_insert(0);
    }

    /// 补种：`static_owner_name_open` 登记的名字对闭包新增类的扫描
    pub(super) fn seed_static_owner_names(&mut self) {
        let names: Vec<(String, usize)> = self.seeds.owner_names.iter().map(|(n, i)| (n.clone(), *i)).collect();
        for (name, from) in names {
            let n = self.classes.len();
            let hits: Vec<String> = (from..n)
                .filter_map(|i| {
                    let (c, _) = self.classes.get_index(i)?;
                    let cf = self.h.class(c)?;
                    cf.fields.iter().any(|f| f.name == name && f.access & acc::STATIC != 0).then(|| c.clone())
                })
                .collect();
            self.seeds.owner_names.insert(name, n);
            for c in hits {
                self.static_field_owner(&c, Via::root("field-name", &c));
            }
        }
    }

    fn link_owner(&mut self, owner: &str, routes: &[LinkRoute]) {
        if owner.starts_with('[') {
            return;
        }
        for &r in routes {
            if self.seeds.linked_owners.insert((r, owner.to_string())) && self.seeds.live_routes.contains(&r) {
                self.seeds.mirror_inits.insert(owner.to_string());
            }
        }
    }
}
