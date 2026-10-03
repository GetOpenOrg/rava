#!/usr/bin/env python3
"""JDK 方法级使用透视（jdk_method_scan）——pilot jar 调了 JDK 的哪些方法，e2e 盖住了没。

与 dep_scan.py 的分工：dep_scan 答「依赖了哪些类」（CONSTANT_Class 一跳引用，选型透视）；
本脚本答「调用了哪些 JDK 方法 / 字段」（CONSTANT_Methodref / InterfaceMethodref / Fieldref
+ NameAndType 解析），并可与 tests/e2e 语料做**方法名级**覆盖对照——框架 pilot 的地基
补齐（e2e）从人工枚举升级为实测驱动：92 jar 的常量池就是「框架真实踩面」的完备清单。

用法：
    python3 scripts/jdk_method_scan.py                      # 全部 pilot-libs：包/类/方法汇总
    python3 scripts/jdk_method_scan.py --jar h2-2.5.252.jar # 单 jar
    python3 scripts/jdk_method_scan.py --methods 60         # 方法级清单深度（默认 40）
    python3 scripts/jdk_method_scan.py --cover              # 叠加：与 tests/e2e 方法名级对照
    python3 scripts/jdk_method_scan.py --report docs/reports/jdk-method-scan.md  # 报告落盘

口径与注意：
  - 引用 = 常量池显式符号引用（invokevirtual/special/static/interface 与字段访问的解析目标）；
    不含反射字符串、JNI、MethodHandle 指针式调用（那属于 dep_scan 透视 + pilot 实测域）。
  - JDK 前缀：java/ javax/ jdk/ sun/ com/sun/ org/xml/ org/w3c/ org/ietf/。
  - --cover 是**启发式**：方法名在 e2e 语料全文零命中 = 强缺口信号；命中 ≠ 语义已覆盖
    （同名方法可能测的是另一族）。零命中清单供人工裁决，不是自动判定。
"""

import argparse
import glob
import struct
import zipfile
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LIBS = ROOT / "tests/lib_pilot/deps/target/pilot-libs"
E2E = ROOT / "tests/e2e"

JDK_PREFIXES = ("java/", "javax/", "jdk/", "sun/", "com/sun/", "org/xml/", "org/w3c/", "org/ietf/")

# 常量池 tag（字节数为 tag 之后部分；1 长度可变、5/6 占双槽在推进时处理）
_TAG_SIZE = {1: -1, 3: 4, 4: 4, 5: 8, 6: 8, 7: 2, 8: 2, 9: 4, 10: 4,
             11: 4, 12: 4, 15: 3, 16: 2, 17: 4, 18: 4, 19: 2, 20: 2}


def parse_constant_pool(data: bytes):
    """解析常量池 → (utf8 表, class 名表, method/field 引用列表)。

    引用元素为 (tag, class_name, member_name, descriptor)；class_name 取自
    CONSTANT_Class 的 name_index，member/descriptor 取自 NameAndType。
    """
    if data[:4] != b"\xca\xfe\xba\xbe":
        return {}, {}, []
    pos = 8
    count = struct.unpack(">H", data[pos:pos + 2])[0]
    pos += 2
    utf8, classes, nats, refs = {}, {}, {}, []
    i = 1
    while i < count:
        tag = data[pos]
        pos += 1
        if tag == 1:
            length = struct.unpack(">H", data[pos:pos + 2])[0]
            pos += 2
            utf8[i] = data[pos:pos + length].decode("utf-8", "replace")
            pos += length
        elif tag == 7:
            classes[i] = struct.unpack(">H", data[pos:pos + 2])[0]
            pos += 2
        elif tag in (9, 10, 11):
            cls_idx, nat_idx = struct.unpack(">HH", data[pos:pos + 4])
            refs.append((tag, cls_idx, nat_idx))
            pos += 4
        elif tag == 12:
            nats[i] = struct.unpack(">HH", data[pos:pos + 4])
            pos += 4
        else:
            pos += _TAG_SIZE[tag]
        i += 1
        if tag in (5, 6):
            i += 1

    def u(idx):
        return utf8.get(idx, "?")

    cls_names = {u(v) for v in classes.values()}
    out = []
    for tag, cls_idx, nat_idx in refs:
        cls_name = u(classes.get(cls_idx, 0))
        name, desc = nats.get(nat_idx, (0, 0))
        out.append((tag, cls_name, u(name), u(desc)))
    return utf8, cls_names, out


def scan_jar(jar: Path):
    """单 jar → (类名集合, 方法引用 {(类,名): 次数}, 字段引用同构, 描述符表)。"""
    cls_refs = set()
    methods = defaultdict(int)
    fields = defaultdict(int)
    member_desc = {}
    with zipfile.ZipFile(jar) as z:
        for name in z.namelist():
            if not name.endswith(".class"):
                continue
            try:
                _, cls_names, refs = parse_constant_pool(z.read(name))
            except Exception:
                continue
            cls_refs.update(cls_names)
            for tag, cls, mname, desc in refs:
                if cls.startswith(JDK_PREFIXES) and mname not in ("?", ""):
                    if tag == 9:
                        fields[(cls, mname)] += 1
                    else:
                        methods[(cls, mname)] += 1
                    member_desc[(cls, mname)] = desc
    return cls_refs, methods, fields, member_desc


def load_e2e_text() -> str:
    chunks = []
    for f in E2E.rglob("*.java"):
        try:
            chunks.append(f.read_text(encoding="utf-8", errors="replace"))
        except Exception:
            pass
    return "\n".join(chunks)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("jars", nargs="*", help="jar 路径（默认 pilot-libs 全部）")
    ap.add_argument("--methods", type=int, default=40, help="方法级清单深度（默认 40）")
    ap.add_argument("--cover", action="store_true", help="与 tests/e2e 做方法名级覆盖对照")
    ap.add_argument("--report", metavar="PATH", help="汇总报告落盘（markdown）")
    args = ap.parse_args()

    jar_paths = [Path(p) for p in args.jars] or sorted(Path(p) for p in glob.glob(str(LIBS / "*.jar")))
    if not jar_paths:
        raise SystemExit(f"[jdk-method-scan] 未找到 jar：{LIBS}（先跑 fetch_pilot_deps.sh）")

    pkg_jars = defaultdict(set)          # 三段包 → jars
    cls_jars = defaultdict(set)          # JDK 类 → jars
    method_jars = defaultdict(set)       # (类, 方法) → jars
    field_jars = defaultdict(set)        # (类, 静态字段) → jars
    member_desc = {}

    for jar in jar_paths:
        cls_refs, methods, fields, descs = scan_jar(jar)
        short = jar.name
        for ref in cls_refs:
            if not ref.startswith(JDK_PREFIXES) or "[" in ref:
                continue
            parts = ref.split("/")
            if len(parts) >= 3:
                pkg_jars["/".join(parts[:3])].add(short)
            cls_jars[ref].add(short)
        for key in methods:
            method_jars[key].add(short)
            member_desc[key] = descs[key]
        for key in fields:
            field_jars[key].add(short)
            member_desc.setdefault(key, descs[key])

    lines = []
    lines.append(f"# JDK 方法级使用透视（{len(jar_paths)} jar，2026-10-03 实测）")
    lines.append("")
    lines.append("> 生成：`python3 scripts/jdk_method_scan.py --cover --report <本文件>`；"
                 "口径见脚本 docstring（常量池符号引用；--cover 为方法名级启发式）。")
    lines.append("")

    lines.append("## 一、JDK 包级（引用 jar 数前 25）")
    for pkg, jars in sorted(pkg_jars.items(), key=lambda kv: -len(kv[1]))[:25]:
        lines.append(f"- {len(jars):3d} jars — {pkg.replace('/', '.')}")
    lines.append("")

    lines.append("## 二、被 ≥5 jar 引用的 JDK 类")
    for cls, jars in sorted(cls_jars.items(), key=lambda kv: (-len(kv[1]), kv[0])):
        if len(jars) >= 5:
            lines.append(f"- {len(jars):3d} jars — {cls.replace('/', '.')}")
    lines.append("")

    lines.append(f"## 三、方法级（被最多 jar 调用的前 {args.methods}）")
    for (cls, mname), jars in sorted(method_jars.items(), key=lambda kv: (-len(kv[1]), kv[0]))[:args.methods]:
        desc = member_desc[(cls, mname)]
        lines.append(f"- {len(jars):3d} jars — {cls.replace('/', '.')}.{mname}{desc}")
    lines.append("")

    print("\n".join(lines))

    if args.cover:
        corpus = load_e2e_text()
        zero_hit = []
        seen_names = set()
        for (cls, mname), jars in sorted(method_jars.items(), key=lambda kv: -len(kv[1])):
            if mname in seen_names or mname in ("<init>", "<clinit>"):
                continue
            seen_names.add(mname)
            if f"{mname}(" not in corpus:
                zero_hit.append((cls, mname, len(jars), member_desc[(cls, mname)]))
        cover_lines = ["", "## 四、方法名级覆盖对照（tests/e2e 全文零命中，启发式）", "",
                       "> 零命中 = 强缺口信号（人工裁决）；命中 ≠ 语义已覆盖。", ""]
        if not zero_hit:
            cover_lines.append("- （无零命中——高频方法名全部在语料中出现）")
        for cls, mname, jars, desc in zero_hit:
            cover_lines.append(f"- {jars:3d} jars — {cls.replace('/', '.')}.{mname}{desc}")
        lines.extend(cover_lines)
        print("\n".join(cover_lines))

        # 静态字段引用（无括号，裸名匹配）：只列真正零命中的
        zero_fields = []
        seen_f = set()
        for (cls, fname), jars in sorted(field_jars.items(), key=lambda kv: -len(kv[1])):
            if fname in seen_f:
                continue
            seen_f.add(fname)
            if fname not in corpus:
                zero_fields.append((cls, fname, len(jars), member_desc[(cls, fname)]))
        field_lines = ["", "## 五、静态字段引用对照（tests/e2e 全文零命中）", ""]
        if not zero_fields:
            field_lines.append("- （无零命中）")
        for cls, fname, jars, desc in zero_fields:
            field_lines.append(f"- {jars:3d} jars — {cls.replace('/', '.')}.{fname}{desc}")
        lines.extend(field_lines)
        print("\n".join(field_lines))

    if args.report:
        out = Path(args.report)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text("\n".join(lines) + "\n", encoding="utf-8")
        try:
            where = out.resolve().relative_to(ROOT)
        except ValueError:
            where = out
        print(f"\n[jdk-method-scan] 报告已落盘：{where}")


if __name__ == "__main__":
    main()
