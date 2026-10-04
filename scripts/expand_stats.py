#!/usr/bin/env python3
"""声明 crate 宏展开体量分类（拆 crate 线 §7.5.4 达标账的度量工具）。

  scripts/expand_stats.py expand <scratch> <out.rs>   # RUSTC_BOOTSTRAP=1 cargo rustc -p java_runtime --lib -- -Z unpretty=expanded
  scripts/expand_stats.py stats <expanded.rs> [--top N] # 按条目归类统计字节，输出 markdown

归类按 rustc 美化输出的缩进结构：模块级条目（`mod` 直接子项）按种类归类；`impl` 块再按其中方法名归入
子类（如 `impl ObjectVTable for X::__view_into`）；属性行与 `use` 行单列。方法名里的类名部分归一
（`__jb_<类>__m` → `__jb_*`），按「种类 / 方法」汇总。
"""

import collections
import os
import re
import subprocess
import sys
from pathlib import Path

ITEM = re.compile(r"^(?:pub(?:\([^)]*\))? )?(?:(?:unsafe|safe|const|async|default|extern \"[^\"]*\") )*"
                  r"(fn|impl|struct|trait|use|static|const|mod|extern|enum|type|macro_rules!|union)\b(.*)$")
FN_NAME = re.compile(r"\bfn ([A-Za-z_][A-Za-z0-9_]*)")


def norm_fn(name):
    # 类名部分归一：__jb_<类>__m / __jbm_<类>__m / <类>__m_base / __rava_... → 模式
    if name.startswith("__jbm_"):
        return "__jbm_*"
    if name.startswith("__jb_"):
        return "__jb_*__" + name.rsplit("__", 1)[-1]
    if name.endswith("_base"):
        return "*_base"
    if name.startswith("__impl_"):
        return "__impl_*"
    if name.startswith("__get_") or name.startswith("__set_"):
        return name[:6] + "*"
    if name.startswith("__init_on"):
        return "__init_on*"
    if name.startswith("new"):
        return "new*"
    return name if name.startswith("__") else "<java 方法>"


def impl_kind(header):
    h = header
    m = re.match(r"impl(?:<.*?>)? (?:(?:::)?[\w:]*::)?(\w+)(?:<[^>]*>)? for ", h)
    if m:
        t = m.group(1)
        if t.endswith("__VTable"):
            return "impl *__VTable for"
        if t.endswith("__AsVTable"):
            return "impl *__AsVTable for"
        return f"impl {t} for"
    return "impl <固有>"


def stats(path, top):
    by_cat = collections.Counter()
    by_sub = collections.Counter()
    cnt = collections.Counter()
    stack = []  # (indent, cat)
    total = 0
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            n = len(line.encode())
            total += n
            s = line.lstrip(" ")
            ind = len(line) - len(s)
            if not s.strip():
                continue
            while stack and ind <= stack[-1][0] and not s.startswith(("}", ")", "]")):
                stack.pop()
            if s.startswith("}") and stack and ind == stack[-1][0]:
                by_cat[stack[-1][1]] += n
                if stack[-1][2]:
                    by_sub[stack[-1][2]] += n
                stack.pop()
                continue
            if s.startswith("#[") or s.startswith("#!["):
                by_cat["属性"] += n
                continue
            if s.startswith("///") or s.startswith("//!"):
                by_cat["文档注释"] += n
                continue
            m = ITEM.match(s)
            parent = stack[-1] if stack else None
            if m and (parent is None or parent[1] in ("mod", "extern 块")):
                kind = m.group(1)
                if kind == "impl":
                    cat = impl_kind(s.strip())
                elif kind == "extern":
                    cat = "extern 块"
                elif kind == "fn":
                    fm = FN_NAME.search(s)
                    cat = "自由 fn " + norm_fn(fm.group(1)) if fm else "自由 fn"
                    if parent and parent[1] == "extern 块":
                        cat = "外部声明 " + norm_fn(fm.group(1)) if fm else "外部声明"
                else:
                    cat = kind
                cnt[cat] += 1
                ends = s.rstrip().endswith((";", "}"))
                if not ends:
                    stack.append((ind, cat, None))
                by_cat[cat] += n
                continue
            if m and parent and parent[1].startswith(("impl", "trait")) and m.group(1) == "fn":
                fm = FN_NAME.search(s)
                sub = parent[1] + " :: " + (norm_fn(fm.group(1)) if fm else "?")
                cnt[sub] += 1
                by_sub[sub] += n
                by_cat[parent[1]] += n
                if not s.rstrip().endswith((";", "}")):
                    stack.append((ind, parent[1], sub))
                continue
            if stack:
                by_cat[stack[-1][1]] += n
                if stack[-1][2]:
                    by_sub[stack[-1][2]] += n
            else:
                by_cat["其他"] += n
    out = [f"展开后 {total / 1e6:.2f} MB（{path}）", "", "| 类别 | MB | 占比 | 条目数 |", "|---|---:|---:|---:|"]
    for c, b in by_cat.most_common(top):
        out.append(f"| {c} | {b / 1e6:.2f} | {100 * b / total:.1f}% | {cnt.get(c, '')} |")
    out += ["", "| impl 内方法（归一） | MB | 个数 |", "|---|---:|---:|"]
    for c, b in by_sub.most_common(top):
        out.append(f"| {c} | {b / 1e6:.2f} | {cnt.get(c, '')} |")
    return "\n".join(out)


def expand(scratch, out):
    env = dict(os.environ, RUSTC_BOOTSTRAP="1")
    with open(out, "w") as f:
        return subprocess.run(["cargo", "rustc", "-q", "-p", "java_runtime", "--lib", "--", "-Z", "unpretty=expanded"],
                              cwd=scratch, env=env, stdout=f).returncode


if __name__ == "__main__":
    if sys.argv[1] == "expand":
        sys.exit(expand(sys.argv[2], sys.argv[3]))
    top = int(sys.argv[sys.argv.index("--top") + 1]) if "--top" in sys.argv else 40
    print(stats(sys.argv[2], top))
