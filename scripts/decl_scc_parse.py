#!/usr/bin/env python3
"""声明层 D8 图的文本解析：D8 同款路径 / 字面量提取 + 引用出现处的上下文分类（`decl_scc_graph.py` 用）。"""
import re

# D8 分段预算（与 generator/crates/emit/src/project/decl_segments.rs DECL_SEGMENT_BYTES / UPPER_SEGMENT_PEAK、
# decl_side.rs SEGMENT_FIXED_BYTES / SEGMENT_SUFFIX_BYTES 同值）
CAP_BYTES = 5_500_000  # 不分段判定
MB = 1_000_000
# 上段峰值（字节单位）= PEAK_INTERCEPT + PEAK_PER_OWN × 本段字节 + PEAK_PER_UPSTREAM × 上游字节 ≤ PEAK_LIMIT
PEAK_INTERCEPT = -9 * MB
PEAK_PER_OWN = 240
PEAK_PER_UPSTREAM = 13
PEAK_LIMIT = (1300 - 64) * MB
SEGMENT_FIXED_BYTES = 1024
SEGMENT_SUFFIX_BYTES = 6
KINDS = ["inherit", "hidden", "sig", "field", "body", "nest", "exc", "attr", "macro", "top"]
INHERIT_KEYS = {"super_class", "interfaces", "all_supertypes", "declared_by", "superclass", "all_superclasses",
                "ancestor_fields_layout", "to_string_vtable", "hash_code_vtable", "equals_vtable", "binary_name"}
NEST_KEYS = {"nest_members", "inner_classes", "permitted_subclasses", "enclosing_method", "nest_host"}
CLASS_MACROS = ("java_class!", "java_class_opaque!", "hidden_class!")
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")


def is_ident_char(c):
    return c.isalnum() or c == "_"


def stem(ident):
    p = ident.find("__")
    return ident[:p] if p > 0 else ident


# ── D8 同款文本解析 ────────────────────────────────────────────────────────────

def matching_brace(text, open_):
    depth = 0
    for i in range(open_, len(text)):
        c = text[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i
    return max(len(text) - 1, open_)


def parse_tree(text, pos, prefix, limit=None):
    end_text = len(text) if limit is None else limit
    while True:
        if text.startswith("{", pos):
            close = matching_brace(text, pos)
            inner = text[pos + 1:close]
            items, depth, start = [], 0, 0
            for i, c in enumerate(inner):
                if c == "{":
                    depth += 1
                elif c == "}":
                    depth = max(depth - 1, 0)
                elif c == "," and depth == 0:
                    items.append((start, i))
                    start = i + 1
            items.append((start, len(inner)))
            out = []
            for s, e in items:
                item = inner[s:e]
                if not item.strip():
                    continue
                lead = len(item) - len(item.lstrip())
                paths, _ = parse_tree(text, pos + 1 + s + lead, list(prefix), pos + 1 + e)
                out.extend(paths)
            return out, close + 1
        rest_at = pos + 2 if text.startswith("r#", pos) else pos
        j = rest_at
        while j < end_text and is_ident_char(text[j]):
            j += 1
        if j == rest_at:
            return ([prefix] if prefix else []), pos
        prefix = prefix + [text[rest_at:j]]
        pos = j
        if text.startswith("::", pos) and pos + 2 <= end_text:
            pos += 2
            continue
        return [prefix], pos


def crate_paths(text):
    """[(位置, 路径)]：与 D8 `crate_paths` 同一集合，另带出现位置"""
    out, frm = [], 0
    while True:
        at = text.find("crate::", frm)
        if at < 0:
            return out
        frm = at + len("crate::")
        prev = text[at - 1] if at > 0 else ""
        if prev and (is_ident_char(prev) or prev == "$"):
            continue
        paths, end = parse_tree(text, frm, [])
        out.extend((at, p) for p in paths)
        frm = max(end, frm)


def string_literals(text):
    """[(位置, 内容)]：含 `/` 的字符串字面量（D8 `string_literals`）"""
    out, i, n = [], 0, len(text)
    while True:
        p = text.find('"', i)
        if p < 0:
            return out
        j, esc, end = p + 1, False, None
        while j < n:
            c = text[j]
            if c == "\\" and not esc:
                esc = True
            elif c == '"' and not esc:
                end = j
                break
            else:
                esc = False
            j += 1
        if end is None:
            return out
        body = text[p + 1:end]
        if "/" in body:
            out.append((p, body))
        i = end + 1


def hw_idents(text):
    """手写文本标识符（去注释与字符串，D8 `idents`）"""
    out, i, n, cur = set(), 0, len(text), []

    def flush():
        if cur and not cur[0].isdigit():
            out.add("".join(cur))
        cur.clear()
    while i < n:
        c = text[i]
        nx = text[i + 1] if i + 1 < n else ""
        if c == "/" and nx == "/":
            flush()
            while i < n and text[i] != "\n":
                i += 1
            continue
        if c == "/" and nx == "*":
            flush()
            i += 2
            while i + 1 < n and not (text[i] == "*" and text[i + 1] == "/"):
                i += 1
            i += 2
            continue
        if c == '"':
            flush()
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            continue
        if is_ident_char(c):
            cur.append(c)
        else:
            flush()
        i += 1
    flush()
    return out


# ── 上下文：给文本每个位置标种类 ───────────────────────────────────────────────

class Ctx:
    """逐字符扫描声明文件，把位置区间标成种类（sig / field / body / inherit / attr:<键> / use / top）"""

    KIND_IDS = {}

    def __init__(self, text):
        self.text = text
        self.spans = []  # (起, 止, 种类)
        self._scan()
        # 由外到内涂色：长区间先涂，内层覆盖外层
        self.paint = bytearray(len(text))
        for s, e, kind in sorted(self.spans, key=lambda x: x[0] - x[1]):
            k = Ctx.KIND_IDS.setdefault(kind, len(Ctx.KIND_IDS) + 1)
            self.paint[s:e] = bytes([k]) * (e - s)
        self.names = {v: k for k, v in Ctx.KIND_IDS.items()}
        self.noncode = self._noncode()

    def _noncode(self):
        """字符串字面量与注释的位置掩码（标识符出现在其中不算引用）"""
        t, n, i = self.text, len(self.text), 0
        mask = bytearray(n)
        while i < n:
            c = t[i]
            if c == '"':
                j = self._skip_str(i)
                mask[i:j] = b"\x01" * (j - i)
                i = j
                continue
            if c == "'" and i + 2 < n and (t[i + 2] == "'" or (t[i + 1] == "\\" and i + 3 < n and t[i + 3] == "'")):
                i += 3 if t[i + 2] == "'" else 4
                continue
            if t.startswith("//", i) or t.startswith("/*", i):
                j = t.find("\n", i) if t[i + 1] == "/" else t.find("*/", i) + 2
                j = n if j <= i else j
                mask[i:j] = b"\x01" * (j - i)
                i = j
                continue
            i += 1
        return mask

    def kind_at(self, pos):
        return self.names.get(self.paint[pos], "top") if pos < len(self.paint) else "top"

    def _skip_str(self, i):
        t, n = self.text, len(self.text)
        j = i + 1
        while j < n and t[j] != '"':
            j += 2 if t[j] == "\\" else 1
        return j + 1

    def _match(self, i, open_c, close_c):
        """i 处为 open_c，返回配对 close_c 之后的位置（跳过字符串与注释）"""
        t, n, depth = self.text, len(self.text), 0
        while i < n:
            c = t[i]
            if c == '"':
                i = self._skip_str(i)
                continue
            if c == "/" and t.startswith("//", i):
                i = t.find("\n", i)
                i = n if i < 0 else i
                continue
            if c == open_c:
                depth += 1
            elif c == close_c:
                depth -= 1
                if depth == 0:
                    return i + 1
            i += 1
        return n

    def _attr(self, i):
        """`#[...]` 属性：按键名给其中字符串标 attr:<键>"""
        end = self._match(i + 1, "[", "]")
        body = self.text[i:end]
        name = IDENT.search(body)
        outer = name.group(0) if name else "?"
        for m in re.finditer(r'([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"', body):
            s = i + m.end() - 1
            e = self._skip_str(s)
            self.spans.append((s, e, "attr:" + m.group(1)))
        kind = "field" if outer == "superclass_fields" else "attr:" + outer
        self.spans.append((i, end, kind))
        return end

    def _items(self, i, end, in_impl):
        """impl / struct 体内条目：属性、fn（签名 + 体）、static / const、字段"""
        t = self.text
        while i < end:
            c = t[i]
            if c.isspace():
                i += 1
                continue
            if t.startswith("//", i):
                j = t.find("\n", i)
                i = end if j < 0 else j
                continue
            if t.startswith("#[", i):
                i = self._attr(i)
                continue
            m = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|unsafe\s+|extern\s+\"[^\"]*\"\s+)*fn\b").match(t, i)
            if in_impl and m:
                j = i
                while j < end and t[j] not in "{;":
                    if t[j] == '"':
                        j = self._skip_str(j)
                        continue
                    j += 1
                self.spans.append((i, j, "sig"))
                if j < end and t[j] == "{":
                    k = self._match(j, "{", "}")
                    self.spans.append((j, k, "body"))
                    i = k
                else:
                    i = j + 1
                continue
            # static / const / type / 字段：到行末分号或逗号
            j = i
            depth = 0
            while j < end:
                ch = t[j]
                if ch == '"':
                    j = self._skip_str(j)
                    continue
                if ch in "{(<[":
                    depth += 1
                elif ch in "})>]":
                    depth -= 1
                elif (ch == ";" or ch == ",") and depth <= 0:
                    j += 1
                    break
                j += 1
            self.spans.append((i, j, "field"))
            i = j

    def _class_block(self, i, end):
        t = self.text
        while i < end:
            c = t[i]
            if c.isspace():
                i += 1
                continue
            if t.startswith("//", i):
                j = t.find("\n", i)
                i = end if j < 0 else j
                continue
            if t.startswith("#[", i):
                i = self._attr(i)
                continue
            m = re.compile(r"pub\s+struct\b|impl\b").match(t, i)
            if not m:
                i += 1
                continue
            brace = i
            while brace < end and t[brace] not in "{;":
                brace += 1
            head = t[i:brace]
            if brace >= end or t[brace] == ";":
                self.spans.append((i, brace + 1, "field"))
                i = brace + 1
                continue
            close = self._match(brace, "{", "}")
            if head.startswith("impl") and re.search(r"\bfor\b", head):
                self.spans.append((i, brace, "inherit"))  # trait impl 头
            else:
                self.spans.append((i, brace, "field" if head.startswith("pub") else "sig"))
            self._items(brace + 1, close - 1, head.startswith("impl"))
            i = close

    def _scan(self):
        t, n, i = self.text, len(self.text), 0
        while i < n:
            if t.startswith("use ", i) and (i == 0 or t[i - 1] == "\n"):
                j = t.find(";", i)
                j = n if j < 0 else j + 1
                self.spans.append((i, j, "use"))
                i = j
                continue
            if t.startswith("//", i):
                j = t.find("\n", i)
                i = n if j < 0 else j
                continue
            if t[i] == "\n" or t[i].isspace():
                i += 1
                continue
            line_end = t.find("\n", i)
            line_end = n if line_end < 0 else line_end
            head = t[i:line_end]
            brace = t.find("{", i)
            if brace >= 0 and "hidden_class!" in head and brace <= line_end:
                close = self._match(brace, "{", "}")
                self.spans.append((i, close, "hidden"))
                i = close
                continue
            if brace >= 0 and any(m in head for m in CLASS_MACROS) and brace <= line_end:
                close = self._match(brace, "{", "}")
                self._class_block(brace + 1, close - 1)
                i = close
                continue
            if "iface_upcasts!" in head and brace >= 0:
                close = self._match(brace, "{", "}")
                self.spans.append((i, close, "inherit"))
                i = close
                continue
            if brace >= 0 and brace <= line_end and "!" in head:
                close = self._match(brace, "{", "}")
                self.spans.append((i, close, "top"))
                i = close
                continue
            self.spans.append((i, line_end, "top"))
            i = line_end


def kind_of(ctx_kind):
    if ctx_kind.startswith("attr:"):
        key = ctx_kind[5:]
        if key in INHERIT_KEYS:
            return "inherit", None
        if key in NEST_KEYS:
            return "nest", None
        if key == "exceptions":
            return "exc", None
        return "attr", key
    return ctx_kind, None
