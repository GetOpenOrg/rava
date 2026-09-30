//! 分类与统计：exact（逐行全等）/ layout（只差已登记的版式规则）/ unported（Rust 走到
//! 未移植分支）/ mismatch（其余）。版式规则集中在 [`LAYOUT_RULES`]，逐条在报告中打印。

use std::collections::BTreeMap;

/// 断言 mismatch = 0 的已移植指令集（consts / locals / stack / arith 与两类折叠点）；
/// 其余指令只报告不断言
pub const PORTED_OPS: &[&str] = &[
    // consts
    "aconst_null", "iconst_m1", "iconst_0", "iconst_1", "iconst_2", "iconst_3", "iconst_4", "iconst_5",
    "lconst_0", "lconst_1", "fconst_0", "fconst_1", "fconst_2", "dconst_0", "dconst_1",
    "bipush", "sipush", "ldc", "ldc_w", "ldc2_w",
    // locals
    "iload", "lload", "fload", "dload", "aload",
    "iload_0", "iload_1", "iload_2", "iload_3", "lload_0", "lload_1", "lload_2", "lload_3",
    "fload_0", "fload_1", "fload_2", "fload_3", "dload_0", "dload_1", "dload_2", "dload_3",
    "aload_0", "aload_1", "aload_2", "aload_3",
    "istore", "lstore", "fstore", "dstore", "astore",
    "istore_0", "istore_1", "istore_2", "istore_3", "lstore_0", "lstore_1", "lstore_2", "lstore_3",
    "fstore_0", "fstore_1", "fstore_2", "fstore_3", "dstore_0", "dstore_1", "dstore_2", "dstore_3",
    "astore_0", "astore_1", "astore_2", "astore_3", "iinc",
    // stack
    "pop", "pop2", "dup", "dup_x1", "dup_x2", "dup2", "dup2_x1", "dup2_x2", "swap",
    // arith
    "iadd", "ladd", "fadd", "dadd", "isub", "lsub", "fsub", "dsub", "imul", "lmul", "fmul", "dmul",
    "idiv", "ldiv", "fdiv", "ddiv", "irem", "lrem", "frem", "drem", "ineg", "lneg", "fneg", "dneg",
    "ishl", "lshl", "ishr", "lshr", "iushr", "lushr", "iand", "land", "ior", "lor", "ixor", "lxor",
    "i2l", "i2f", "i2d", "l2i", "l2f", "l2d", "f2i", "f2l", "f2d", "d2i", "d2l", "d2f", "i2b", "i2c", "i2s",
    "lcmp", "fcmpl", "fcmpg", "dcmpl", "dcmpg",
    // 折叠点
    "fold_const", "fold",
];

/// 版式规则：(名称, 说明, 行规范化)。两侧逐行不等、但各行经全部规则规范化后相等 → layout；
/// 命中计数按「该规则改变了任一侧某行」统计
pub type LayoutRule = (&'static str, &'static str, fn(&str) -> String);

pub const LAYOUT_RULES: &[LayoutRule] = &[(
    "float-int-literal",
    "fconst / dconst 的浮点字面量：Python `Lit(f\"{n}f64\")` 发射 `0f64`，Rust `FloatLit` 渲染为 `0.0f64`（同值同类型）",
    float_int_literal,
)];

/// `0f64` / `12f32` → `0.0f64` / `12.0f32`（仅整数位后直接跟 `f32` / `f64` 且前后为词边界的记号）
fn float_int_literal(line: &str) -> String {
    let b = line.as_bytes();
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut out = String::with_capacity(line.len() + 4);
    let mut i = 0;
    while i < b.len() {
        let starts = b[i].is_ascii_digit() && (i == 0 || !(word(b[i - 1]) || b[i - 1] == b'.'));
        if starts {
            let j = (i..b.len()).find(|&j| !b[j].is_ascii_digit()).unwrap_or(b.len());
            let suffix = &line[j..];
            let is_float = (suffix.starts_with("f32") || suffix.starts_with("f64")) && b.get(j + 3).is_none_or(|&c| !word(c));
            out.push_str(&line[i..j]);
            if is_float {
                out.push_str(".0");
            }
            i = j;
            continue;
        }
        let ch = line[i..].chars().next().unwrap_or(' ');
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

#[derive(Default, Clone, Copy)]
pub struct Counts {
    pub exact: u64,
    pub layout: u64,
    pub unported: u64,
    pub mismatch: u64,
}

impl Counts {
    fn total(&self) -> u64 {
        self.exact + self.layout + self.unported + self.mismatch
    }
}

pub enum Class {
    Exact,
    Layout(Vec<&'static str>),
    Unported(String),
    Mismatch,
}

/// 逐行比对 → 分类（Unported 由回放直接给出）
pub fn classify(py: &[String], rs: &[String]) -> Class {
    if py == rs {
        return Class::Exact;
    }
    if py.len() != rs.len() {
        return Class::Mismatch;
    }
    let mut hits = Vec::new();
    for (p, r) in py.iter().zip(rs) {
        if p == r {
            continue;
        }
        let (mut pn, mut rn) = (p.clone(), r.clone());
        for (name, _, f) in LAYOUT_RULES {
            let (p2, r2) = (f(&pn), f(&rn));
            if (p2 != pn || r2 != rn) && !hits.contains(name) {
                hits.push(*name);
            }
            (pn, rn) = (p2, r2);
        }
        if pn != rn {
            return Class::Mismatch;
        }
    }
    Class::Layout(hits)
}

/// 一个测试的统计：记录数（去重后）与调用数（记录 × count）两套口径
#[derive(Default)]
pub struct Tally {
    pub by_op: BTreeMap<String, (Counts, Counts)>,
    pub unported: BTreeMap<String, u64>,
    pub layout_hits: BTreeMap<&'static str, u64>,
    pub ported_mismatch: u64,
}

impl Tally {
    pub fn add(&mut self, op: &str, count: u64, class: &Class) {
        let (rec, calls) = self.by_op.entry(op.to_string()).or_default();
        let bump = |c: &mut Counts, n: u64| match class {
            Class::Exact => c.exact += n,
            Class::Layout(_) => c.layout += n,
            Class::Unported(_) => c.unported += n,
            Class::Mismatch => c.mismatch += n,
        };
        bump(rec, 1);
        bump(calls, count);
        match class {
            Class::Unported(s) => *self.unported.entry(s.clone()).or_default() += 1,
            Class::Layout(hits) => {
                for h in hits {
                    *self.layout_hits.entry(h).or_default() += 1;
                }
            }
            Class::Mismatch if is_ported(op) => self.ported_mismatch += 1,
            _ => {}
        }
    }

    /// 逐指令分类表 + 未移植分支 + 版式规则命中（stderr）
    pub fn report(&self, test: &str, sampled: &str) {
        eprintln!("\n[instr-golden] {test}（{sampled}）逐指令分类（记录数；调用数见括号）");
        eprintln!("  {:<16} {:>6} {:>16} {:>16} {:>16} {:>16}", "op", "ported", "exact", "layout", "unported", "mismatch");
        let mut sum = (Counts::default(), Counts::default());
        for (op, (r, c)) in &self.by_op {
            let cell = |a: u64, b: u64| format!("{a} ({b})");
            eprintln!(
                "  {:<16} {:>6} {:>16} {:>16} {:>16} {:>16}",
                op,
                if is_ported(op) { "✓" } else { "" },
                cell(r.exact, c.exact),
                cell(r.layout, c.layout),
                cell(r.unported, c.unported),
                cell(r.mismatch, c.mismatch)
            );
            for (s, x) in [(&mut sum.0, r), (&mut sum.1, c)] {
                s.exact += x.exact;
                s.layout += x.layout;
                s.unported += x.unported;
                s.mismatch += x.mismatch;
            }
        }
        let (r, c) = sum;
        eprintln!(
            "  {:<16} {:>6} {:>16} {:>16} {:>16} {:>16}   共 {} 记录 / {} 调用",
            "合计",
            "",
            format!("{} ({})", r.exact, c.exact),
            format!("{} ({})", r.layout, c.layout),
            format!("{} ({})", r.unported, c.unported),
            format!("{} ({})", r.mismatch, c.mismatch),
            r.total(),
            c.total()
        );
        eprintln!("[instr-golden] {test} 未移植分支（记录数）：");
        for (s, n) in &self.unported {
            eprintln!("  {n:>7}  {s}");
        }
        eprintln!("[instr-golden] {test} 版式规则命中（记录数）：");
        for (name, desc, _) in LAYOUT_RULES {
            eprintln!("  {:>7}  {name}：{desc}", self.layout_hits.get(name).copied().unwrap_or(0));
        }
    }
}

pub fn is_ported(op: &str) -> bool {
    PORTED_OPS.contains(&op)
}
