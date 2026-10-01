//! 引擎：手写字段访问器（如标准流 `System::out()`：HotSpot 经 native 写入、不经 `<clinit>` 的静态字段）。
//!
//! 访问器体是一段独立的手写代码，其回调实参取自它自己的值池（手写体产出 / 读到的静态字段）。
//! 按读取方法的上下文建模会让回调实参落到读取方的值池——字节码方法的值池没有产出汇入，实参为空集，
//! 进而被 `null_recv` 折叠为恒 null（C1d：`PrintStream.<init>` 的 Charset 实参 → StreamEncoder 编码器恒 null）。
//! 故每个访问器建一个伪方法节点（不进输出），与实现对象的伪方法同样接线。

use super::*;

/// 伪方法节点的种类标签
pub(super) const HWFIELD_KIND: &str = "field-accessor";

/// 伪方法名：`<get:字段名>`，不可能与 Java 方法重名
fn accessor_name(field: &str) -> String {
    format!("<get:{field}>")
}

pub(super) fn field_of(accessor: &str) -> &str {
    accessor.strip_prefix("<get:").and_then(|s| s.strip_suffix('>')).unwrap_or(accessor)
}

impl<'a> Engine<'a> {
    /// 字段 `decl.name` 的手写访问器伪方法节点（首次创建时入队）
    pub(super) fn hwfield_method(&mut self, decl: &str, name: &str, fdesc: &str, via: Via) -> usize {
        let key = MemberRef { owner: decl.to_string(), name: accessor_name(name), desc: format!("(){fdesc}") };
        if let Some(i) = self.methods.get_index_of(&(key.clone(), NOCTX)) {
            return i;
        }
        let rtype = parse_field(fdesc).and_then(|t| self.ptype(&t));
        let idx = self.methods.len();
        self.methods.insert(
            (key.clone(), NOCTX),
            MNode {
                key: key.clone(),
                kind: Kind::Handwritten(HWFIELD_KIND),
                via,
                is_static: true,
                ptypes: Vec::new(),
                rtype,
                analysis: None,
                applied: None,
                returned: None,
                hw_fns: Vec::new(),
                ctx: NOCTX,
                ret_model: RetModel::Plain,
                aseq: 0,
                applied_seq: 0,
            },
        );
        self.mbase.entry(key).or_insert(idx);
        self.push_m(idx);
        idx
    }

    /// 访问器节点：手写体产出汇入值池，回调 / 分配 / 字段效果按访问器所在类的手写文件建模
    pub(super) fn process_hwfield_method(&mut self, m: usize) {
        let via = Via::method("handwritten", m, None);
        let obj = self.id(OBJECT);
        self.flow(Node::S(m, PROD), Node::S(m, POOL), obj);
        let Some((host, mh)) = self.hw_body(m) else { return };
        let rt = self.methods[m].rtype;
        for t in self.hw_exports(&host, &mh, rt, true) {
            self.flow(Node::S(m, POOL), Node::Esc, t);
        }
        self.apply_hw(m, &host, &mh, &via);
    }
}
