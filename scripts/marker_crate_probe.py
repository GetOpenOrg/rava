#!/usr/bin/env python3
"""类型标记 crate 峰值探针（S7 计划 §9.8 第四问）：按类名清单生成一个只含类型标记的 crate，供服务器实测编译峰值。

用法：
  scripts/marker_crate_probe.py <names.txt> <out_dir>
  (cd <out_dir> && /usr/bin/time -v cargo build)        # 只在服务器上跑

<names.txt>：每行一个 binary name（如 `java/lang/String`；可由 `jimage list` 取 java.base 全部类）。
每类生成的形态即 §9.8 标记路线的标记 crate 内容上限：
  - `#[repr(transparent)] pub struct X { h: Handle, _p: ::std::marker::PhantomData<fn() -> X> }`（句柄 + 零尺寸类型标记）；
  - `Clone` / `PartialEq` / `Debug` 三个 impl（句柄身份语义，与现 wrapper 相同的 std trait 面）；
  - `From<X> for Handle` 与固有 `const BINARY_NAME`。
不含方法、字段访问器、描述符数据与 vtable trait（它们留在各声明 crate）。按包生成嵌套模块，类名按 rava 规则
把 `$` 换成 `_`、`-` 换成 `_`；同名冲突加序号。输出确定。
"""
import os
import re
import sys

KEYWORDS = {"as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl",
            "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
            "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
            "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual",
            "yield", "try", "gen"}

HEADER = """#![allow(non_camel_case_types, non_snake_case, dead_code)]
/// 句柄（与 runtime `__Handle` 同形：可空的共享对象指针）
#[derive(Clone, Default)]
pub struct Handle(pub Option<::std::rc::Rc<dyn std::any::Any>>);
"""


def ident(s):
    s = re.sub(r"[^A-Za-z0-9_]", "_", s)
    if s[0].isdigit():
        s = "_" + s
    return "r#" + s if s in KEYWORDS else s


def item(name, bin_name):
    return f"""
#[repr(transparent)]
pub struct {name} {{ h: crate::Handle, _p: ::std::marker::PhantomData<fn() -> {name}> }}
impl {name} {{ pub const BINARY_NAME: &'static str = "{bin_name}"; }}
impl Clone for {name} {{ fn clone(&self) -> Self {{ Self {{ h: self.h.clone(), _p: ::std::marker::PhantomData }} }} }}
impl PartialEq for {name} {{
    fn eq(&self, o: &Self) -> bool {{
        match (&self.h.0, &o.h.0) {{ (Some(a), Some(b)) => ::std::rc::Rc::ptr_eq(a, b), (None, None) => true, _ => false }}
    }}
}}
impl std::fmt::Debug for {name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{ f.write_str(Self::BINARY_NAME) }}
}}
impl From<{name}> for crate::Handle {{ fn from(x: {name}) -> Self {{ x.h }} }}
"""


def main():
    if len(sys.argv) != 3:
        print(__doc__)
        return 1
    names = sorted({line.strip() for line in open(sys.argv[1]) if line.strip()})
    out = sys.argv[2]
    tree = {}
    for b in names:
        parts = b.split("/")
        node = tree
        for p in parts[:-1]:
            node = node.setdefault(ident(p), {})
        node.setdefault("", []).append(b)
    os.makedirs(os.path.join(out, "src"), exist_ok=True)
    with open(os.path.join(out, "Cargo.toml"), "w") as f:
        f.write('[package]\nname = "marker_probe"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "src/lib.rs"\n')
    lines = [HEADER]

    def emit(node, depth):
        pad = "    " * depth
        used = set()
        for b in node.get("", []):
            n = ident(b.rsplit("/", 1)[-1])
            k, base = 1, n
            while n in used or n in node:
                k += 1
                n = f"{base}_{k}"
            used.add(n)
            for ln in item(n, b).splitlines():
                lines.append(pad + ln if ln else ln)
        for m in sorted(k for k in node if k):
            lines.append(f"{pad}pub mod {m} {{")
            emit(node[m], depth + 1)
            lines.append(f"{pad}}}")
    emit(tree, 0)
    with open(os.path.join(out, "src", "lib.rs"), "w") as f:
        f.write("\n".join(lines) + "\n")
    size = os.path.getsize(os.path.join(out, "src", "lib.rs"))
    print(f"marker_probe: {len(names)} 类，lib.rs {size / 1e6:.2f} MB")
    return 0


if __name__ == "__main__":
    sys.exit(main())
