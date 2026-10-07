//! 引擎：调用点上的反射式写入与点名（按字节码形状，见 `reflective_writes`）。

use super::*;

impl<'a> Engine<'a> {
    /// 反射式字段写入（按字节码形状）：
    /// - 同一调用里有字符串常量，且形参含 Class 或接收者是 Class：点名字段不折叠
    ///   （所属类取 Class 常量实参 / 接收者，取不到时同名字段全部不折叠）；
    /// - 清单 `[facts.field_writes] enumerators`（返回字段句柄数组）：句柄写入口（`handle_writers`）也可达时
    ///   接收者类的全部字段不折叠，推不出时全部字段（调用方是清单 `serial_enumerators` 时为可序列化字段）；
    /// - 清单 `[facts.reflect] method_lookups`：字符串常量登记为 Class 常量所指类的方法点名；名字是本方法形参时
    ///   取各调用点在该形参上的字符串常量（如按名构造 MemberName 的辅助方法），常量集增长时本站点重跑；
    /// - 清单 `deserializers` 可达：非 static、非 transient 字段全部不折叠
    pub(super) fn reflective_writes(&mut self, m: usize, off: u32, mref: &MemberRef, opcode: u8, args: &[V]) {
        self.rcall_conversion(m, off, mref, opcode, args);
        let class_param = parse_method(&mref.desc)
            .is_some_and(|md| md.params.iter().any(|p| matches!(p, FieldType::Object(c) if c == CLASS)));
        let class_recv = opcode != classfile::op::INVOKESTATIC && mref.owner == CLASS;
        let classes: Vec<String> = args
            .iter()
            .filter_map(|a| match a {
                V::Class(c, _) => Some(c.to_string()),
                _ => None,
            })
            .collect();
        let k = self.mref_key(mref);
        if self.man.is_constructor_lookup(&k) {
            self.constructor_lookup(m, off, &k, mref, opcode, args);
        }
        if let Some(idx) = self.man.serial_allocator(&k) {
            self.serial_alloc_site(m, off, &k, opcode, args, idx);
        }
        if self.man.is_method_lookup(&k) {
            // 查找结果经哪条反射调用通道调用（按查找结果的类型，见 `reflect_call.rs`）
            let ch = self.rcall_lookup_channel(&mref.desc);
            let mut names: BTreeSet<Rc<str>> = BTreeSet::new();
            // 本调用点上的字面量名（常量实参 / 合流前的各字面量）；形参透传来的名字不在此列
            let mut site_names: BTreeSet<Rc<str>> = BTreeSet::new();
            // 拼接出的名字按目标类逐个解析（只保留该类上声明的方法）；目标类另取 Class 实参值集里类镜像所指的类
            // （如取自 static final Class 字段）。形参透传的名字不与镜像类相乘：其类同样来自形参，交叉组合会失真
            let mut per_class: Vec<(String, Rc<str>)> = vec![];
            let mut targets: Option<Vec<String>> = None;
            for a in args {
                match a {
                    V::Str(name, _) if !a.derived_str() => {
                        names.insert(name.clone());
                        site_names.insert(name.clone());
                    }
                    // 常量格给出的名字（形参 / 字段 / 返回常量）按其来源处理，同常量格推不出时：
                    // 不算本点字面量，不与接收者镜像相乘（中间态常量与终态给出同样的点名，见 `V::Str`）
                    V::Ref { .. } | V::Str(..) => {
                        // 合流前的各字面量（如按条件二选一的名字）与形参上流入的字符串常量
                        site_names.extend(a.site_lits());
                        names.extend(a.site_lits());
                        names.extend(self.param_strs(m, off, a));
                        names.extend(self.field_strs(m, a));
                        let Some(parts) = self.method_name_parts(m, a) else { continue };
                        if targets.is_none() {
                            let mut ts = classes.clone();
                            for c in self.class_arg_mirrors(m, mref, opcode, args) {
                                if !ts.contains(&c) {
                                    ts.push(c);
                                }
                            }
                            targets = Some(ts);
                        }
                        for c in targets.iter().flatten() {
                            for n in self.declared_matching(c, &parts) {
                                per_class.push((c.clone(), n));
                            }
                        }
                    }
                    _ => {}
                }
            }
            // 常量名的查找目标：Class 常量实参；本调用点的字面量名另对 Class 接收者值集里类镜像所指的类点名
            // （如 `this.getMethod("values")`：接收者是流到该方法的类镜像）。
            // 形参透传的名字不与接收者镜像相乘：名字与接收者各自来自全部调用点，交叉组合会把任意镜像类上的
            // 同名方法拉进反射面（如序列化辅助方法按形参取名、按形参取类）；拼段名同理只按常量类 / Class 形参定目标
            for name in &names {
                for c in &classes {
                    self.reflect_name(c, name, ch);
                }
            }
            if class_recv && !site_names.is_empty() {
                for c in args.first().map(|r| self.recv_mirrors(m, r)).unwrap_or_default() {
                    if classes.contains(&c) {
                        continue;
                    }
                    for name in &site_names {
                        self.reflect_name(&c, name, ch);
                    }
                }
            }
            // 查找类与名字都来自本方法形参：登记为包装方法，各调用点按本点实参配对点名（`lookup_pair.rs`）
            let wrapped = class_recv && classes.is_empty() && self.lookup_wraps(m, mref, opcode, args, ch);
            if class_recv && !wrapped && !names.is_empty() && classes.is_empty() && site_names.is_empty() {
                // 名字只经形参流入、接收者非常量：查找目标推不出，记为反射缺口
                self.reflect_gaps.insert(format!("{} <- recv(param-name)", self.methods[m].key));
            }
            // 本调用点的字面量名另对 Class 形参值集里类镜像所指的类点名（如 `findStatic(invokerClass, "invoke_V", …)`：
            // 类取自字段 / 类定义点返回的镜像）。与接收者镜像同一口径：只乘本调用点字面量，不乘形参透传的名字
            if class_param && !site_names.is_empty() {
                for c in self.class_arg_mirrors(m, mref, opcode, args) {
                    if classes.contains(&c) {
                        continue;
                    }
                    for name in &site_names {
                        self.reflect_name(&c, name, ch);
                    }
                }
            }
            for (c, name) in &per_class {
                self.reflect_name(c, name, ch);
            }
        }
        if class_param || class_recv {
            // 按名放开字段：字面量与常量格给出的名字，另取按来源给出的名字（形参上各调用点的字符串常量、字段写入的
            // 字面量集）。形参常量窗口内的文本是终态形参字符串集的子集，后者在形参抬为 Top 后仍给出同样的名字
            // 类值（Class 接收者 / Class 形参）与名字都来自本方法形参时登记字段配对，形参名字不在此汇合放开，
            // 由各调用点按本点实参配对点名（`lookup_pair.rs`）
            let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
            let mut cpos: Vec<usize> = if class_recv { vec![0] } else { vec![] };
            // 名字位：声明为 String 的形参。Object 等其他引用形参不是字段名（如 `compareComparables(Class, Object, Object)`
            // 的键），不取名字、不登记配对——否则映射键上流入的全部字符串常量都会按名放开字段
            let mut spos: Vec<usize> = vec![];
            if let Some(md) = parse_method(&mref.desc) {
                cpos.extend(md.params.iter().enumerate().filter(|(_, p)| matches!(p, FieldType::Object(c) if c == CLASS)).map(|(i, _)| i + skip));
                spos.extend(md.params.iter().enumerate().filter(|(_, p)| matches!(p, FieldType::Object(c) if c == STRING)).map(|(i, _)| i + skip));
            }
            let mut fnames: BTreeSet<Rc<str>> = BTreeSet::new();
            for (i, a) in args.iter().enumerate() {
                if !spos.contains(&i) {
                    continue;
                }
                // 常量格给出的名字（`V::derived_str`）按来源取：形参（配对或各调用点常量）、字段写入字面量集、
                // 辅助方法返回常量。中间态常量若当字面量按名放开，形参配对后的终态不再给出该名，放开不撤回（D1）
                fnames.extend(a.site_lits());
                if matches!(a, V::Ref { .. }) || a.derived_str() {
                    let mut paired = false;
                    for &c in &cpos {
                        paired |= self.lookup_wrap_site(m, &args[c], a, &[], 0, true);
                    }
                    if !paired {
                        fnames.extend(self.param_strs(m, off, a));
                    }
                    fnames.extend(self.field_strs(m, a));
                    fnames.extend(self.site_strs(m, a));
                }
            }
            // 形状规则把 Class 字面量实参当作字段所属类：所属类上查不到该名时按名兜底放开。唯一目标按字节码
            // 建模时不兜底——该字面量不是所属类（如 `getFieldOffset(name, Object.class)` 的字段类型实参），
            // 被调体内真正按名取字段的点由其自身的站点规则处理（形参名字取各调用点常量 / 字段配对）
            let analyzed = !classes.is_empty() && self.analyzed_exact(opcode, mref);
            for name in &fnames {
                let mut hit = false;
                for c in &classes {
                    if let Some((decl, desc)) = self.field_by_name(c, name) {
                        self.open_field(MemberRef { owner: decl, name: name.to_string(), desc });
                        hit = true;
                    }
                }
                if !hit && !analyzed {
                    self.open_field_name(name);
                }
            }
        }
        self.field_name_site(m, off, &k, opcode, args);
        self.handle_writer_site(m, mref, opcode, args);
        self.mirror_init_site(m, off, mref, &k, opcode, args);
        if (class_param || class_recv) && !self.man.is_method_lookup(&k) {
            self.field_lookup(m, off, mref, opcode, args, &classes, class_recv);
        }
        if self.man.is_field_enumerator(&k) {
            // 接收者 Class 值集里的类镜像逐类放开（值集增长时本站点重跑）；含所指未知的 Class 时全部放开，记为缺口
            // 放开与登记都幂等：只取值集中尚未处理的部分（站点重跑由值集增长驱动）
            let mut cs = BTreeSet::new();
            let unknown = match args.first() {
                None => true,
                Some(v) => {
                    let mut seen = self.refl_seen.entry(m).or_default().remove(&off).unwrap_or_default();
                    let u = self.class_values_new(m, v, &mut seen.fenum, &mut cs);
                    self.refl_seen.entry(m).or_default().insert(off, seen);
                    u
                }
            };
            // 序列化口径的调用方只用可序列化字段：已知的类与推不出的接收者都按可序列化字段放开。
            // 它们不取静态字段的值（computeDefaultSUID 只读名字与修饰符，默认序列化字段滤掉 static），
            // 枚举本身不初始化类（Class.getDeclaredFields 不触发 `<clinit>`），所以不按静态字段句柄初始化声明类；
            // computeDefaultSUID 对可序列化类的初始化由 hasStaticInitializer（class_initializers）建模
            let serial = self.man.is_serial_enumerator(&self.methods[m].key.to_string());
            if !serial {
                self.enumerated_static_owners(m, off, &cs);
            }
            let mut scopes: Vec<field_handles::EnumScope> = cs.into_iter().map(|c| (serial, Some(c))).collect();
            if unknown {
                scopes.push((serial, None));
                if !serial {
                    self.field_enum_gaps.insert(format!("{}@{off}", self.methods[m].key));
                }
            }
            let statics = !serial && !self.man.is_instance_field_user(&self.methods[m].key.to_string());
            for (_, c) in &scopes {
                if serial {
                    self.enumerate_serial_fields(c.clone());
                } else {
                    if statics {
                        self.fenum_static.insert(c.clone());
                    }
                    self.enumerate_fields(c.clone());
                }
            }
            // 结果句柄带各口径的来源标记：流到句柄写入口时才放开（`field_handles.rs`）
            self.mark_enumeration(m, off, &mref.desc, &scopes);
        }
        if self.man.is_deserializer(&k) && !self.ctx.deser.replace(true) {
            self.open_fields_all(self.ctx.fopen_all.get(), false);
        }
    }

    /// 调用的唯一目标按字节码建模（非 native / 手写承载 / 抽象）：其体内的按名取字段点由分析器直接看到
    fn analyzed_exact(&self, opcode: u8, mref: &MemberRef) -> bool {
        let iface = self.h.class(&mref.owner).is_some_and(|c| c.is_interface());
        let Some((cf, t)) = self.ctx.exact_target(opcode, mref, iface) else { return false };
        cf.methods.iter().find(|x| x.name == t.name && x.desc == t.desc).is_some_and(|rm| self.kind_of(&cf, rm) == Kind::Bytecode)
    }
}
