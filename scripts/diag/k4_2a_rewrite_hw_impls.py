#!/usr/bin/env python3
"""K4-2a：伴生手写文件的固有 impl 机械改写为 `__XHw{N}` 扩展 trait 实现（S7 计划 §9.9）。

背景：K4 阶段 3 把 wrapper struct 挪入 `java_marker` 后，声明层 crate 里伴生文件
（`<x>_impl.rs` / `_ext.rs`，约 100 份）对 wrapper 的固有 impl 全部 E0390。本脚本把
`impl X {`（含多块）改写为 `impl __XHw{N} for X {`（体不动，仅 impl 头替换 + 块内
顶层条目剥可见性修饰——trait impl 条目不允许 `pub`），并补 `use super::__XHw{N};`
导入行。trait 声明由生成器按本脚本的同一扫描口径发射（`scripts/diag` 本脚本只负责
伴生侧；ordinal 与生成器扫描序一致 = 文件遍历序 × 块位置序）。

规则（与生成器扫描 `input::handwritten` 的 hw 块探测同口径）：
- 只改第 0 列的固有 `impl` 头（无 ` for `）；fn 体内的缩进 impl（如 `impl Cursor<'_>`）不动；
- 头类名在本树（runtime/java_runtime/src）任何地方有定义（struct/enum/trait/type/union）
  的是本地辅助类型（`Prim` / `Instance` / `_BitOp`）或手写根类（`Object`），不改写；
- 每类按遍历序编号：第 1 块 `__XHw`，第 N 块 `__XHw{N}`（N ≥ 2）；
- 块内顶层条目剥 `pub` / `pub(crate)` / `pub(super)`（fn 与 const）；
- 幂等：已改写形态识别为同编号延续，重跑无操作。

用法：python3 scripts/diag/k4_2a_rewrite_hw_impls.py [--check]   # --check 只报告不改
"""

from __future__ import annotations

import argparse
import os
import re
import sys

IMPL_HEAD = re.compile(r'^impl\b')
IDENT = re.compile(r'[A-Za-z_]\w*')


def _balanced(head: str, i: int) -> tuple[str, int]:
    """从 head[i] == '<' 取平衡的尖括号组（内层可再嵌 `<…>`）。"""
    depth = 0
    start = i
    while i < len(head):
        c = head[i]
        if c == "<":
            depth += 1
        elif c == ">":
            depth -= 1
            if depth == 0:
                return head[start:i + 1], i + 1
        elif c == ";":
            break
        i += 1
    raise ValueError(f"尖括号不配对：{head!r}")


def _skip_ws(head: str, i: int) -> int:
    while i < len(head) and head[i].isspace():
        i += 1
    return i


def parse_inherent_head(head: str) -> tuple[str, str, str, str] | None:
    """`impl<GEN> Cls<APP> [where …] {` → (GEN, Cls, APP, WHERE)；泛型组按尖括号平衡取（可嵌套 `From<Object>`）。"""
    i = _skip_ws(head, len("impl"))
    gen = ""
    if i < len(head) and head[i] == "<":
        gen, i = _balanced(head, i)
    i = _skip_ws(head, i)
    m = IDENT.match(head, i)
    if not m:
        return None
    cls = m.group(0)
    i = _skip_ws(head, m.end())
    app = ""
    if i < len(head) and head[i] == "<":
        app, i = _balanced(head, i)
    i = _skip_ws(head, i)
    where = ""
    if head[i:i + 6] == "where ":
        j = head.find("{", i)
        if j < 0:
            raise ValueError(f"固有 impl 头以 {{ 结尾：{head!r}")
        where = " " + " ".join(head[i:j].split()).rstrip(",") + " "
        i = _skip_ws(head, j)
    if i >= len(head) or head[i] != "{":
        raise ValueError(f"固有 impl 头以 {{ 结尾：{head!r}")
    return gen, cls, app, where


def parse_hw_head(head: str) -> tuple[str, str, int, str, str] | None:
    """`impl<GEN> __ClsHw{N} for SelfTy<APP> {` → (GEN, Cls, N, SelfTy, APP)。"""
    i = _skip_ws(head, len("impl"))
    gen = ""
    if i < len(head) and head[i] == "<":
        gen, i = _balanced(head, i)
    i = _skip_ws(head, i)
    if i >= len(head) or head[i] != "_":
        return None
    m = IDENT.match(head, i)
    if not m or not m.group(0).startswith("__"):
        return None
    trait = m.group(0)[2:]
    hm = re.fullmatch(r"(?P<cls>[A-Za-z_]\w*?)Hw(?P<ord>\d*)", trait)
    if not hm:
        return None
    i = _skip_ws(head, m.end())
    # trait 名后的自类型实参组（幂等重入形态）
    if i < len(head) and head[i] == "<":
        _, i = _balanced(head, i)
        i = _skip_ws(head, i)
    if head[i:i + 4] != "for ":
        return None
    i = _skip_ws(head, i + 4)
    sm = IDENT.match(head, i)
    if not sm:
        return None
    selfty = sm.group(0)
    i = _skip_ws(head, sm.end())
    while i < len(head) and head[i] == ":" and IDENT.match(head, i + 2):
        i = _skip_ws(head, i + 2)
        nm = IDENT.match(head, i)
        if not nm:
            break
        selfty = nm.group(0)
        i = _skip_ws(head, nm.end())
    app = ""
    if i < len(head) and head[i] == "<":
        app, i = _balanced(head, i)
    i = _skip_ws(head, i)
    if head[i:i + 6] == "where ":
        j = head.find("{", i)
        if j < 0:
            raise ValueError(f"__XHw impl 头以 {{ 结尾：{head!r}")
        i = _skip_ws(head, j)
    if i >= len(head) or head[i] != "{":
        raise ValueError(f"__XHw impl 头以 {{ 结尾：{head!r}")
    return gen, hm.group("cls"), int(hm.group("ord")) if hm.group("ord") else 1, selfty, app
DEF_ITEM = re.compile(
    r'^(?:pub(?:\s*\([^)]*\))?\s+)?(?:unsafe\s+)?(?:struct|enum|trait|type|union)\s+([A-Za-z_]\w*)'
)
VIS_ITEM = re.compile(r'^(\s*)(pub(?:\s*\([^)]*\))?\s+)((?:unsafe\s+)?(?:const\s+)?(?:extern\s+"[^"]*"\s+)?(?:fn|const)\b)')

## 原 `pub fn` / `pub unsafe fn` 条目的原位标记（trait impl 条目不允许可见性修饰）：
## 闭包分析器（vm 钩子判定）与 impl_methods（成员由手写提供的判定）据此区分「原本就是
## 公开成员」与「原本 pub(crate)/pub(super) 的私有辅助」——两类在改写前的语义不同，
## 后者不是钩子 / 不进成员判定（CollectorsDemo 种子漂移的根因，S7 计划 §9.9 K4-2a）。
HW_PUB_MARK = "#[cfg_attr(any(), rava_hw_pub)]"


def walk_sorted(root: str):
    """与 Rust 侧 `walk` 同序：本目录文件（排序）→ 子目录（排序）递归。产出相对路径。"""
    out = []

    def rec(rel: str):
        d = os.path.join(root, rel) if rel else root
        try:
            entries = sorted(os.listdir(d))
        except OSError:
            return
        files = [e for e in entries if os.path.isfile(os.path.join(d, e))]
        dirs = [e for e in entries if os.path.isdir(os.path.join(d, e))]
        for f in files:
            out.append(os.path.join(rel, f) if rel else f)
        for sub in dirs:
            rec(os.path.join(rel, sub) if rel else sub)

    rec("")
    return out


def defined_types(root: str) -> set[str]:
    """runtime 树内定义的类型名（第 0 列条目；fn 内定义的缩进形式不采——其 impl 同样缩进）。"""
    out: set[str] = set()
    for rel in walk_sorted(root):
        if not rel.endswith(".rs"):
            continue
        try:
            with open(os.path.join(root, rel), encoding="utf-8") as fh:
                for line in fh:
                    m = DEF_ITEM.match(line)
                    if m:
                        out.add(m.group(1))
        except (OSError, UnicodeDecodeError):
            continue
    return out


def find_blocks(lines: list[str]):
    """产出 (head_first_line_idx, head_last_line_idx, body_end_line_idx 含收括号行)。

    只认第 0 列 `impl` 头；头累积到以 `{` 结尾的行；体按花括号深度找齐。
    """
    i, n = 0, len(lines)
    while i < n:
        if not IMPL_HEAD.match(lines[i]):
            i += 1
            continue
        head_lines = [lines[i]]
        j = i
        while not "".join(head_lines).rstrip().endswith("{") and j + 1 < n:
            j += 1
            head_lines.append(lines[j])
        head = " ".join(h.strip() for h in head_lines)
        # 跳过带 ` for ` 的 trait impl（含既有 __XHw 形态——调用方需要识别，这里统一交出原文）
        if " for " in head:
            i = j + 1
            continue
        depth = 1
        k = j + 1
        while k < n and depth > 0:
            depth += lines[k].count("{") - lines[k].count("}")
            k += 1
        yield (i, j, k - 1)
        i = k


def rewrite_file(path: str, text: str, local_types: set[str], counters: dict[str, int],
                 uses_needed: dict[str, list[int]]) -> tuple[str, list[str]]:
    """改写一个伴生文件；返回 (新文本, 报告行)。计数器跨文件延续（同类多文件）。"""
    lines = text.split("\n")
    reports: list[str] = []
    spans: list[tuple[int, int, int, str]] = []  # (head_first, head_last, body_end, new_head)
    own: dict[str, set[int]] = {}

    # 先识别既有改写形态（幂等），延续编号
    i = 0
    n = len(lines)
    while i < n:
        if not IMPL_HEAD.match(lines[i]):
            i += 1
            continue
        head_lines = [lines[i]]
        j = i
        while not "".join(head_lines).rstrip().endswith("{") and j + 1 < n:
            j += 1
            head_lines.append(lines[j])
        head = " ".join(h.strip() for h in head_lines)
        depth = head.count("{") - head.count("}")
        k = j + 1
        while k < n and depth > 0:
            depth += lines[k].count("{") - lines[k].count("}")
            k += 1
        m = None
        try:
            m = parse_hw_head(head)
        except ValueError:
            pass
        if m and m[1] == m[3]:
            _, cls, ordn, _, _ = m
            counters[cls] = max(counters.get(cls, 0), ordn)
            own.setdefault(cls, set()).add(ordn)
            uses_needed.setdefault(cls, set()).add(ordn)
        elif " for " not in head:
            m2 = None
            try:
                m2 = parse_inherent_head(head)
            except ValueError:
                pass
            if m2 and m2[1] not in local_types:
                gen, cls, app, where = m2
                ordn = counters.get(cls, 0) + 1
                counters[cls] = ordn
                suffix = "" if ordn == 1 else str(ordn)
                # 泛型 trait 的 impl 必须应用其形参：`impl<T> __XHw<T> for X<T>`
                new_head = f"impl {gen} __{cls}Hw{suffix}{app} for {cls}{app}{where}{{"
                new_head = re.sub(r"\s+", " ", new_head).replace("{ ", "{").replace(" }", "}")
                new_head = re.sub(r",?\s*\{$", " {", new_head).replace("impl <", "impl<")
                spans.append((i, j, k - 1, new_head))
                own.setdefault(cls, set()).add(ordn)
                uses_needed.setdefault(cls, set()).add(ordn)
                reports.append(f"{path}: impl {cls} -> __{cls}Hw{suffix}")
        i = k

    if not spans:
        return text, reports

    # 块内顶层条目剥可见性（只动 spans 记录的改写块）
    edits: list[tuple[int, str]] = []  # (行号, 新行)
    for (h0, h1, bend, new_head) in spans:
        for ln in range(h0, h1 + 1):
            if ln == h0:
                edits.append((h0, new_head))
            else:
                edits.append((ln, ""))
        depth = 1
        for ln in range(h1 + 1, bend + 1):
            line = lines[ln]
            if depth == 1:
                m = VIS_ITEM.match(line)
                if m:
                    ind, vis, kw = m.group(1), m.group(2).strip(), m.group(3)
                    if vis == "pub":
                        edits.append((ln, f"{ind}{HW_PUB_MARK}\n{ind}{kw}{line[m.end():]}"))
                    else:
                        edits.append((ln, ind + kw + line[m.end():]))
            depth += line.count("{") - line.count("}")
    for ln, new in edits:
        lines[ln] = new

    # 导入行：本文件涉及类的全部 ordinal（跨块 / 跨文件互调都在作用域）
    ins_lines: list[str] = []
    for cls in sorted(own):
        for ordn in sorted(uses_needed.get(cls, set())):
            suffix = "" if ordn == 1 else str(ordn)
            ins_lines.append(f"use super::__{cls}Hw{suffix};")
    existing = set(l.strip() for l in lines)
    ins_lines = [l for l in ins_lines if l not in existing]
    if ins_lines:
        last_use = max((i for i, l in enumerate(lines) if l.startswith("use ") and l.endswith(";")), default=-1)
        insert_at = last_use + 1
        lines[insert_at:insert_at] = ins_lines

    return "\n".join(lines), reports


def scan_hw_items(src: str) -> dict[str, dict[int, set[str]]]:
    """伴生树里 `impl __XHw{N} for X` 块的条目名：类 → {ordinal: 名集}。"""
    out: dict[str, dict[int, set[str]]] = {}
    hwre = re.compile(r"^impl\s*(?:<.*>\s*)?__(\w+?)Hw(\d*)(?:<[^>]*>)?\s+for\s+(\w+)")
    for rel in walk_sorted(src):
        if not (rel.endswith("_impl.rs") or rel.endswith("_ext.rs")):
            continue
        cur, depth, cls = None, 0, None
        for line in open(os.path.join(src, rel), encoding="utf-8"):
            m = hwre.match(line)
            if m and depth == 0:
                cls = m.group(1)
                ordn = int(m.group(2)) if m.group(2) else 1
                out.setdefault(cls, {}).setdefault(ordn, set())
                cur = (cls, ordn)
                depth = 1
                continue
            if depth > 0:
                depth += line.count("{") - line.count("}")
                if cur:
                    fm = re.match(r"^\s*(?:unsafe\s+)?fn\s+(\w+)", line)
                    if fm:
                        out[cur[0]][cur[1]].add(fm.group(1))
                if depth <= 0:
                    cur = None
    return out


KEYWORD_PKGS = {"ref": "r#ref"}

hwre_head = re.compile(r"^impl\s*(?:<.*>\s*)?__\w+Hw\d*(?:<[^>]*>)?\s+for\s")


def fix_callers(src: str, check: bool) -> int:
    """非伴生手写文件里的 hw 方法调用面（`Cls::m(` UFCS 与单主 `.m(` 接收者形态）补
    `use crate::<包>::__ClsHw{N} as _;`。插入在文件头最后一个 `use` 行后，幂等。"""
    items = scan_hw_items(src)
    # 伴生也扫（跨类 hw 调用——如 method_handle_ext 的 Unsafe::__field_of_offset；本类
    # trait 的 use 行已在改写主步加入，existing 集合去重幂等）
    # 伴生文件位置 → 类的包路径（crate::java::lang 一段）
    pkg_of: dict[str, str] = {}
    hwre = re.compile(r"^impl\s*(?:<.*>\s*)?__(\w+?)Hw(\d*)(?:<[^>]*>)?\s+for\s")
    for rel in walk_sorted(src):
        if not (rel.endswith("_impl.rs") or rel.endswith("_ext.rs")):
            continue
        for line in open(os.path.join(src, rel), encoding="utf-8"):
            m = hwre.match(line)
            if m:
                segs = [KEYWORD_PKGS.get(s, s) for s in os.path.dirname(rel).split(os.sep) if s != "."]
                pkg_of.setdefault(m.group(1), "crate::" + "::".join(segs) if segs else "crate")
                break
    # 接收者形态只认白名单（`.m(` 单主匹配的误报率高——str / 迭代器 / 原子等常用名撞车，
    # s7k4d-hw2 实测：`.bytes(`/`.count(`/`.pin(`/`.load(` 全是误报；s7k4d-hw3 实测
    # `MethodHandleImpl$BindCaller.makeInjectedInvoker(H)` 的文档注释同样命中）。新增条目
    # 前先核实接收者确为该 wrapper（文档注释不算）。
    receiver_allow = {
        "isPrimitive": ["Class"],
        "getBoolean": ["Array"],
        "__declared_method_rows": ["Class"],
        "__method_from_meta": ["Class"],
        "__table_declared_methods": ["Class"],
        "__reflect_key": ["Method", "Constructor"],
        "__vh_ref_update": ["Unsafe"],
        "__vh_prim_update": ["Unsafe"],
    }
    cls_ord_of: dict[str, dict[int, set[str]]] = items

    changed = 0
    for rel in walk_sorted(src):
        if not rel.endswith(".rs"):
            continue
        p = os.path.join(src, rel)
        text = open(p, encoding="utf-8").read()
        need: dict[str, set[int]] = {}
        for cls, ords in cls_ord_of.items():
            for ordn, names in ords.items():
                if any(re.search(rf"\b{cls}::{re.escape(n)}\s*[(<]", text) for n in names):
                    need.setdefault(cls, set()).add(ordn)
        for n, clss in receiver_allow.items():
            for cls in clss:
                if cls in need or cls not in cls_ord_of:
                    continue
                if re.search(rf"\.{re.escape(n)}\s*\(", text):
                    for ordn, names in cls_ord_of[cls].items():
                        if n in names:
                            need.setdefault(cls, set()).add(ordn)
        lines: list[str] = []
        for cls in sorted(need):
            segs = KEYWORD_PKGS.get(cls, cls)
            base = pkg_of.get(cls)
            if not base:
                continue
            for ordn in sorted(need[cls]):
                suffix = "" if ordn == 1 else str(ordn)
                lines.append(f"use {base}::__{cls}Hw{suffix} as _;")
        existing = set(l.strip() for l in text.split("\n"))
        lines = [l for l in lines if l not in existing]
        if lines:
            changed += 1
            print(f"{p}: +{len(lines)} use 行 ({', '.join(l.split('::')[-1] for l in lines)})")
            if not check:
                out_lines = text.split("\n")
                last_use = max((i for i, l in enumerate(out_lines) if l.startswith("use ") and l.endswith(";")), default=-1)
                insert_at = last_use + 1
                if last_use < 0:
                    # 无先导 use：跳过文件头的 `//!` 文档与 `#![...]` 内属性（内文档前不得有条目）
                    while insert_at < len(out_lines) and (
                        out_lines[insert_at].startswith("//!") or out_lines[insert_at].startswith("#![")
                    ):
                        insert_at += 1
                    if insert_at < len(out_lines) and out_lines[insert_at].strip() == "":
                        insert_at += 1
                out_lines.insert(insert_at, "\n".join(lines))
                with open(p, "w", encoding="utf-8") as fh:
                    fh.write("\n".join(out_lines))
    print(f"[{'CHECK ' if check else ''}K4-2a fix-callers] 补导入文件 {changed} 份")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true", help="只报告将要做的改写，不落盘")
    ap.add_argument("--fix-callers", action="store_true",
                    help="非伴生手写文件的 hw 调用面补 use 行（改写伴生后的第二步）")
    ap.add_argument("--src", default=None, help="runtime src 根（缺省 <repo>/runtime/java_runtime/src）")
    args = ap.parse_args()

    here = os.path.dirname(os.path.abspath(__file__))
    src = args.src or os.path.join(here, "..", ".. "[:-1], "runtime", "java_runtime", "src")
    src = os.path.normpath(src)
    if not os.path.isdir(src):
        print(f"src 不存在：{src}", file=sys.stderr)
        return 2

    if args.fix_callers:
        return fix_callers(src, args.check)

    local_types = defined_types(src)
    counters: dict[str, int] = {}
    uses_needed: dict[str, set[int]] = {}
    changed = 0
    all_reports: list[str] = []
    # 两遍：第一遍改写头并收集全部类完整块序（跨文件后见块补前文件的 use 行），
    # 第二遍按完整集合插 use 行落盘
    staged: list[tuple[str, str, str]] = []  # (path, old, new)
    for rel in walk_sorted(src):
        if not (rel.endswith("_impl.rs") or rel.endswith("_ext.rs")):
            continue
        p = os.path.join(src, rel)
        with open(p, encoding="utf-8") as fh:
            text = fh.read()
        new, reports = rewrite_file(p, text, local_types, counters, uses_needed)
        all_reports.extend(reports)
        if new != text:
            changed += 1
        staged.append((p, text, new))  # 无头改写的文件也要走 use 补齐遍
    for p, text, new in staged:
        # hw 块签名引用本文件模块级定义的类型 → 定义补 pub（声明文件要 use 它；私有跨模块不可见）
        lines = new.split("\n")
        local_defs = set()
        for i, l in enumerate(lines):
            m = re.match(r"^(?:pub(?:\s*\([^)]*\))?\s+)?(?:unsafe\s+)?(?:struct|enum|trait|type|union)\s+(\w+)", l)
            if m:
                local_defs.add(m.group(1))
        hw_words = set()
        depth = 0
        for l in lines:
            if hwre_head.match(l) and depth == 0:
                depth = 1
                continue
            if depth > 0:
                depth += l.count("{") - l.count("}")
                for w in re.findall(r"\b([A-Z]\w+)\b", l):
                    hw_words.add(w)
        need_pub = local_defs & hw_words - {"Self", "Result", "Option", "Vec", "Clone", "Default", "Into", "From"}
        if need_pub:
            for i, l in enumerate(lines):
                m = re.match(r"^(?:unsafe\s+)?(?:struct|enum|trait|type|union)\s+(\w+)", l)
                if m and m.group(1) in need_pub:
                    lines[i] = "pub " + l
            new = "\n".join(lines)
        # 补齐本文件涉及类的全部 ordinal（首文件后见的块）
        lines = new.split("\n")
        ins: list[str] = []
        for m in re.finditer(r"^impl\s*(?:<.*>\s*)?__(\w+?)Hw(\d*)(?:<[^>]*>)?\s+for\s", "\n".join(
                l for l in lines if l.startswith("impl")), re.M):
            cls, ordn = m.group(1), int(m.group(2)) if m.group(2) else 1
            for o in sorted(uses_needed.get(cls, set())):
                suffix = "" if o == 1 else str(o)
                line = f"use super::__{cls}Hw{suffix};"
                if line not in lines:
                    ins.append(line)
        if ins:
            last_use = max((i for i, l in enumerate(lines) if l.startswith("use ") and l.endswith(";")), default=-1)
            lines.insert(last_use + 1, "\n".join(sorted(set(ins))))
            new = "\n".join(lines)
        if not args.check and new != text:
            with open(p, "w", encoding="utf-8") as fh:
                fh.write(new)
        if args.check and new != text:
            print(f"[check] {p}: 将补 {len(set(ins)) if 'ins' in dir() else '?'} use 行")

    for r in all_reports:
        print(r)
    tail = sorted(f"{cls}: {'/'.join(str(o) if o > 1 else '' for o in sorted(ords)) or '1'}" for cls, ords in uses_needed.items())
    print(f"\n[{'CHECK ' if args.check else ''}K4-2a] 改写文件 {changed} 份，类 {len(uses_needed)} 个：{', '.join(tail)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
