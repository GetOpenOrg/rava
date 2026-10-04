//! 引擎：字段枚举与字段句柄写入口（`[facts.field_writes] enumerators / handle_writers`）。
//!
//! 字段句柄只经句柄写入口（Field.set*、Lookup 的 setter / VarHandle、jdk.unsupported 的 Unsafe 偏移取法）
//! 才能写字段。被枚举的字段（字段枚举、名字不可知的按名取句柄）只有其句柄真的流到写入口时才放开——
//! 按名字常量取到的句柄已由名字点名放开，不使别处枚举的字段随之放开。
//!
//! 句柄身份按来源标记：每个枚举口径（类 / 推不出，是否只取可序列化字段）一个标记对象（类型为字段句柄类型，
//! 不是堆抽象：不分配、不作克隆上下文），在枚举调用点并入结果（枚举返回数组时经该调用点的一个标记数组的元素）。
//! 标记随句柄值沿字节码数据流传播；字节码调用点调用写入口时，句柄实参值集里的标记所指口径放开。
//! 句柄实参取不到值（`V::Top`）或含字段句柄相关类型的 open（来自非建模代码），以及经非字节码调用点
//! （反射调用、lambda、手写体）到达写入口时，按保守口径全部放开。

use super::*;

/// 枚举口径：(只取可序列化字段, 类；None = 推不出)
pub(super) type EnumScope = (bool, Option<String>);

/// 标记数组的伪分配偏移（与字节码偏移不相交：字节码偏移 < 2^16）
const MARK_ARRAY: u32 = 0x4000_0000;

impl<'a> Engine<'a> {
    /// 字段枚举（cls = 接收者 Class 值所指的类，None = 推不出）：句柄写入口已按保守口径可达时放开，
    /// 否则挂起到标记流到写入口
    pub(super) fn enumerate_fields(&mut self, cls: Option<String>) {
        if !self.fwriter_live && !self.fh_released.contains(&(false, cls.clone())) {
            if self.fenum_pending.insert(cls) {
                self.offset_reads_ready();
            }
            return;
        }
        self.open_class_fields(cls);
    }

    /// 放开类（含超类）的全部字段；None = 全部字段不折叠
    pub(super) fn open_class_fields(&mut self, cls: Option<String>) {
        match cls {
            Some(c) => {
                let mut cur = Some(c);
                while let Some(cls) = cur {
                    let Some(cf) = self.h.class(&cls) else { break };
                    for f in &cf.fields {
                        self.open_field(MemberRef { owner: cls.clone(), name: f.name.clone(), desc: f.desc.clone() });
                    }
                    cur = cf.super_name.clone();
                }
            }
            None => {
                if !self.ctx.fopen_all.replace(true) {
                    self.open_fields_all(false, self.ctx.deser.get());
                }
            }
        }
    }

    /// 可序列化字段口径的枚举（清单 `serial_enumerators`；cls = 接收者 Class 所指的类，None = 推不出）：
    /// 该类及其超类（None = 全部可序列化类）的非 static、非 transient 字段偏移可得（可按偏移读取）；
    /// 句柄流到写入口时这些字段不折叠（None 同反序列化的字段面）。transient / static 字段不经此放开
    pub(super) fn enumerate_serial_fields(&mut self, cls: Option<String>) {
        if self.fenum_serial.insert(cls.clone()) {
            self.offset_reads_ready();
        }
        if self.fwriter_live || self.fh_released.contains(&(true, cls.clone())) {
            self.open_serial_fields(cls);
        }
    }

    fn open_serial_fields(&mut self, cls: Option<String>) {
        let Some(c) = cls else {
            if !self.ctx.deser.replace(true) {
                self.open_fields_all(self.ctx.fopen_all.get(), false);
            }
            return;
        };
        let mut cur = Some(c);
        while let Some(cls) = cur {
            let Some(cf) = self.h.class(&cls) else { break };
            for f in &cf.fields {
                let key = MemberRef { owner: cls.clone(), name: f.name.clone(), desc: f.desc.clone() };
                if self.ctx.field_info(&key).is_some_and(|i| Ctx::serial_field(&i)) {
                    self.open_field(key);
                }
            }
            cur = cf.super_name.clone();
        }
    }

    /// 方法节点登记时的句柄写入口检查：字节码调用点（`invoke` / `dispatch`）到达的写入口由调用点按句柄实参
    /// 判定（[`Self::handle_writer_site`]）；其它途径（反射调用、lambda、手写体、具体求值）到达时句柄来源不可知，
    /// 按保守口径。调用者是句柄桥（取得的句柄只经 Field.set* 的访问器使用，写入由 Field.set* 计入）时不算；
    /// 每条边都判（首个调用者是桥不代表后续调用者也是）
    pub(super) fn handle_writer_edge(&mut self, key: &MemberRef, via: &Via) {
        if !self.man.is_field_handle_writer(key) {
            return;
        }
        if let From::Method(c) = via.from {
            if self.man.is_field_handle_bridge(&self.methods[c].key.to_string()) {
                return;
            }
            if via.off.is_some() && matches!(via.kind, "invoke" | "dispatch") && self.methods[c].kind == Kind::Bytecode {
                return;
            }
        }
        let from = match via.from {
            From::Method(c) => self.methods[c].key.to_string(),
            From::Root(ref r) | From::Class(ref r) => r.clone(),
        };
        self.field_writer_live(format!("{key} <- {} {from}", via.kind));
    }

    /// 句柄写入口经来源不可知的途径可达：全部枚举口径放开（含此后的枚举）
    pub(super) fn field_writer_live(&mut self, cause: String) {
        if std::mem::replace(&mut self.fwriter_live, true) {
            return;
        }
        self.fwriter_cause = Some(cause);
        for cls in std::mem::take(&mut self.fenum_pending) {
            self.open_class_fields(cls);
        }
        for cls in self.fenum_serial.clone() {
            self.open_serial_fields(cls);
        }
    }

    /// 枚举口径放开：其标记流到了句柄写入口
    fn release_scope(&mut self, scope: EnumScope) {
        if self.fwriter_live || !self.fh_released.insert(scope.clone()) {
            return;
        }
        match scope {
            (true, cls) => {
                if self.fenum_serial.contains(&cls) {
                    self.open_serial_fields(cls);
                }
            }
            (false, cls) => {
                if self.fenum_pending.remove(&cls) {
                    self.open_class_fields(cls);
                }
            }
        }
    }

    /// 口径 scope 的标记对象（类型 ty = 字段句柄类型）
    fn handle_mark(&mut self, ty: &str, scope: &EnumScope) -> u32 {
        let name = format!("{ty}#<enum:{}{}>", scope.1.as_deref().unwrap_or("*"), if scope.0 { ":serial" } else { "" });
        if let Some(&id) = self.ids.get(name.as_str()) {
            return id;
        }
        let tid = self.id(ty);
        let id = self.id(&name);
        self.objs.insert(id, tid);
        self.fh_marks.insert(id, scope.clone());
        id
    }

    /// 字段枚举调用点 (m, off)（返回句柄数组，描述符 desc）：各口径的标记经本点的标记数组元素并入结果
    pub(super) fn mark_enumeration(&mut self, m: usize, off: u32, desc: &str, scopes: &[EnumScope]) {
        let Some(arr) = desc.rsplit_once(')').map(|(_, r)| r.to_string()) else { return };
        let Some(ty) = arr.strip_prefix("[L").and_then(|c| c.strip_suffix(';')).map(str::to_string) else { return };
        if scopes.is_empty() {
            return;
        }
        let id = self.array_site(m, MARK_ARRAY | off, &arr, false, Via::method("field-enum", m, Some(off)));
        for s in scopes {
            let k = self.handle_mark(&ty, s);
            for p in PARITIES {
                self.add_to(Node::E(id, p), &TypeSet::exact(k));
            }
        }
        self.add_to(Node::S(m, off), &TypeSet::exact(id));
    }

    /// 按名取句柄的调用点 (m, off)（名字不可知，描述符 desc）：口径的标记并入结果
    pub(super) fn mark_handle(&mut self, m: usize, off: u32, desc: &str, scope: EnumScope) {
        let Some(ty) = desc.rsplit_once(')').and_then(|(_, r)| r.strip_prefix('L')).and_then(|c| c.strip_suffix(';')) else {
            return;
        };
        let ty = ty.to_string();
        let k = self.handle_mark(&ty, &scope);
        self.add_to(Node::S(m, off), &TypeSet::exact(k));
    }

    /// 字节码调用点 (m, off) 调用句柄写入口（实参 args 含接收者）：句柄类型的实参值集里的标记所指口径放开；
    /// 值集含句柄相关类型的 open、或实参值不可知时按保守口径。值集增长时本站点重跑
    pub(super) fn handle_writer_site(&mut self, m: usize, mref: &MemberRef, opcode: u8, args: &[V]) {
        if self.fwriter_live || !self.man.is_field_handle_writer(mref) {
            return;
        }
        if self.man.is_field_handle_bridge(&self.methods[m].key.to_string()) {
            return;
        }
        let Some(md) = parse_method(&mref.desc) else { return };
        let mut decls: Vec<Option<String>> = vec![];
        if opcode != classfile::op::INVOKESTATIC {
            decls.push(Some(mref.owner.clone()));
        }
        decls.extend(md.params.iter().map(|p| match p {
            FieldType::Object(c) => Some(c.clone()),
            _ => None,
        }));
        let types = self.man.field_handle_types();
        for (d, v) in decls.iter().zip(args) {
            let Some(d) = d else { continue };
            if !types.iter().any(|t| self.h.is_subtype(t, d)) {
                continue;
            }
            match v {
                V::Null | V::Str(..) | V::Class(..) => {}
                V::Ref { .. } => {
                    let tid = self.id(d);
                    let fs = self.feeds(m, v, tid);
                    let s = self.value_set(&fs);
                    let related = s.open.iter().any(|o| {
                        let o = self.names[o as usize].clone();
                        types.iter().any(|t| self.h.is_subtype(t, &o) || self.h.is_subtype(&o, t))
                    });
                    if related {
                        self.field_writer_live(format!("{mref} @ {}: open", self.methods[m].key));
                        return;
                    }
                    let scopes: Vec<EnumScope> = s.classes.iter().filter_map(|x| self.fh_marks.get(&x).cloned()).collect();
                    for sc in scopes {
                        self.release_scope(sc);
                    }
                }
                _ => {
                    self.field_writer_live(format!("{mref} @ {}: unknown", self.methods[m].key));
                    return;
                }
            }
        }
    }
}
