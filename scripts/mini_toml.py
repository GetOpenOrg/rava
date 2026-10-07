"""TOML 读取兼容层：Python ≥3.11 用标准库 tomllib；更老的解释器（部分云服务器 python3 为 3.10）回落到
本文件的最小解析器——只支持本仓库数据文件用到的子集：注释、[表]、[[表数组]]、`键 = "基本字符串"` /
整数 / 浮点 / 布尔 / 字符串数组（可跨行、可尾逗号）。"""

import json
import re

try:
    import tomllib  # noqa: F401

    def loads(text: str) -> dict:
        return tomllib.loads(text)
except ModuleNotFoundError:
    def _value(raw: str):
        raw = raw.strip()
        if raw in ("true", "false"):
            return raw == "true"
        if raw.startswith("["):
            return json.loads(re.sub(r",\s*\]$", "]", raw))
        if raw.startswith('"'):
            return json.loads(raw)
        return float(raw) if any(c in raw for c in ".eE") else int(raw)

    def _strip_comment(line: str) -> str:
        out, in_str, esc = [], False, False
        for ch in line:
            if in_str:
                esc = ch == "\\" and not esc
                if ch == '"' and not esc:
                    in_str = False
            elif ch == '"':
                in_str = True
            elif ch == "#":
                break
            out.append(ch)
        return "".join(out).strip()

    def loads(text: str) -> dict:
        root: dict = {}
        cur = root
        pending = None  # (键, 已累积文本) —— 跨行数组
        for line in text.splitlines():
            line = _strip_comment(line)
            if pending:
                pending = (pending[0], pending[1] + " " + line)
                if line.endswith("]"):
                    cur[pending[0]] = _value(pending[1])
                    pending = None
                continue
            if not line:
                continue
            if line.startswith("[["):
                cur = {}
                root.setdefault(line[2:-2].strip(), []).append(cur)
            elif line.startswith("["):
                cur = root.setdefault(line[1:-1].strip(), {})
            else:
                key, _, val = line.partition("=")
                key, val = key.strip(), val.strip()
                if val.startswith("[") and not val.endswith("]"):
                    pending = (key, val)
                else:
                    cur[key] = _value(val)
        return root
