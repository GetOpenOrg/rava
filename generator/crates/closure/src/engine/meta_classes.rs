//! 元数据裁剪口径：哪些闭包类需要反射成员表（方法 / 构造器表、字段表）。
//!
//! 运行期读成员表的路径只有反射（`Class.getDeclared*` / `getMethod` / `getField`）、方法句柄解析
//! （`MethodHandleNatives.resolve` / `expand`）、注解解析与动态代理、序列化字段描述。这些路径的
//! 目标类在分析期已由反射事实给出；集合外的类在档案侧成员表里记 0 行（运行期视同「不声明成员」，
//! 沿超类链的查找仍然正确落到声明类——集合按超类型闭包）。
//!
//! 方法表口径：成员枚举所指类、按名查方法 / 构造器的类、按名取类得到的类（构造器）、反射成员面的
//! 声明类、补种点名类与整类放开类、注解类型（元素面）、序列化分配目标。注解解析入口可达时，闭包内全部注解类型
//! （`ACC_ANNOTATION`，含 JDK 侧：方法 / 字段 / 类上的 JDK 注解同样在运行期解析，如反射调用判调用者敏感读方法注解，
//! 元注解 `@Retention` 决定其保留策略）都有方法表：AnnotationParser 经注解类型的方法表求元素面，动态代理按接口
//! 方法表（含超接口 `Annotation`）取 Method。
//! 字段表口径：按名查字段的声明类、按名查字段目标推不出时声明该名字段的闭包类、字段枚举与整类放开
//! 字段的类、可序列化字段枚举的类（推不出时取全部可序列化闭包类）、补种整类放开类。
//!
//! 反射缺口（`reflect_gaps` / `field_enum_gaps`）不扩大集合：缺口处成员本就不在调用链上（存根），
//! 表行只影响枚举结果的完整性；缺口数随分析精度归零。

use std::collections::BTreeSet;

use super::*;

impl Engine<'_> {
    /// 需要方法 / 构造器表的闭包类（含超类型）
    pub fn meta_method_classes(&self) -> BTreeSet<String> {
        let mut out: BTreeSet<String> = BTreeSet::new();
        let name = |id: u32| self.names[id as usize].to_string();
        out.extend(self.enumerated.iter().map(|(_, c)| name(*c)));
        out.extend(self.reflect_names.keys().map(|c| name(*c)));
        out.extend(self.named_ctors.iter().map(|c| name(*c)));
        out.extend(self.reflect_members.iter().map(|(_, m)| m.owner.clone()));
        out.extend(self.seeds.reflect_names.keys().cloned());
        out.extend(self.seeds.reflect_all.iter().cloned());
        out.extend(self.seeds.annotation_types.iter().cloned());
        if self.seeds.anno_done {
            out.extend(self.annotation_classes());
        }
        out.extend(self.serial_allocs.iter().cloned());
        self.close_supertypes(out)
    }

    /// 需要字段表的闭包类（含超类型）
    pub fn meta_field_classes(&self) -> BTreeSet<String> {
        let mut out: BTreeSet<String> = BTreeSet::new();
        out.extend(self.reflect_fields.iter().map(|(c, _)| c.clone()));
        out.extend(self.seeds.reflect_all.iter().cloned());
        let all_scope = self.fenum_scopes.contains(&None);
        out.extend(self.fenum_scopes.iter().flatten().cloned());
        let all_serial = self.fenum_serial.contains(&None);
        out.extend(self.fenum_serial.iter().flatten().cloned());
        let names = &self.reflect_field_names;
        if all_scope || all_serial || !names.is_empty() {
            for c in self.classes.keys() {
                if all_scope || (all_serial && self.class_serializable(c)) {
                    out.insert(c.clone());
                } else if !names.is_empty()
                    && self.h.class(c).is_some_and(|cf| cf.fields.iter().any(|f| names.contains(&f.name)))
                {
                    out.insert(c.clone());
                }
            }
        }
        self.close_supertypes(out)
    }

    /// 运行期可经反射 Field / 方法句柄按名读写的静态字段中，按名查字段（`reflect_fields` / `reflect_field_names`）
    /// 以外的来源：可取到静态字段句柄的枚举口径（`fenum_static`）所指类及其超类型声明的全部静态字段（推不出
    /// 所指类时为闭包全部类），与静态字段句柄常量解析到的字段。可序列化字段口径与清单 `instance_field_users`
    /// 内取到的句柄只用于实例字段，不计入。发射层据此（并上按名查字段事实）只为这些静态字段生成按名访问
    /// 表项（`__STATICS`）
    pub fn static_field_handles(&self) -> BTreeSet<(String, String)> {
        let mut out = self.static_mh_fields.clone();
        let classes: BTreeSet<String> = if self.fenum_static.contains(&None) {
            self.classes.keys().cloned().collect()
        } else {
            self.close_supertypes(self.fenum_static.iter().flatten().cloned().collect())
        };
        for c in &classes {
            let Some(cf) = self.h.class(c) else { continue };
            out.extend(cf.fields.iter().filter(|f| f.is_static()).map(|f| (c.clone(), f.name.clone())));
        }
        out
    }

    /// 闭包内的注解类型
    fn annotation_classes(&self) -> Vec<String> {
        self.classes
            .keys()
            .filter(|c| self.h.class(c).is_some_and(|cf| cf.access & acc::ANNOTATION != 0))
            .cloned()
            .collect()
    }

    /// 超类型闭包，限于闭包类（数组 / 未入闭包的类无表）
    fn close_supertypes(&self, seed: BTreeSet<String>) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for c in &seed {
            for s in self.h.supertypes(c).iter() {
                if self.classes.contains_key(s) {
                    out.insert(s.clone());
                }
            }
        }
        out
    }
}
