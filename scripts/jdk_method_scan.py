#!/usr/bin/env python3
"""JDK 方法级使用透视（jdk_method_scan）——pilot jar 调了 JDK 的哪些方法，e2e 盖住了没。

与 dep_scan.py 的分工：dep_scan 答「依赖了哪些类」（CONSTANT_Class 一跳引用，选型透视）；
本脚本答「调用了哪些 JDK 方法 / 字段」（CONSTANT_Methodref / InterfaceMethodref / Fieldref
+ NameAndType 解析），并与 tests/e2e 语料做**方法名级**覆盖对照——**默认运行即产出缺口清单**
（分层覆盖数 + gated 归属 + 可行动缺口），框架 pilot 的地基补齐从人工枚举升级为实测驱动。

用法：
    python3 scripts/jdk_method_scan.py                        # 全部 pilot-libs：使用透视 + 缺口对照（默认找缺口）
    python3 scripts/jdk_method_scan.py --jar h2-2.5.252.jar   # 单 jar
    python3 scripts/jdk_method_scan.py --usage-only           # 只看使用面，不做覆盖对照
    python3 scripts/jdk_method_scan.py --min-jars 3           # 可行动缺口的 jar 数阈值（默认 5）
    python3 scripts/jdk_method_scan.py --methods 60           # 方法级使用清单深度（默认 40）
    python3 scripts/jdk_method_scan.py --report docs/reports/jdk-method-scan.md  # 报告落盘

口径与注意：
  - 引用 = 常量池显式符号引用（invokevirtual/special/static/interface 与字段访问的解析目标）；
    不含反射字符串、JNI、MethodHandle 指针式调用（那属于 dep_scan 透视 + pilot 实测域）。
  - JDK 前缀：java/ javax/ jdk/ sun/ com/sun/ org/xml/ org/w3c/ org/ietf/。
  - 覆盖对照是**方法名级启发式**：名字在 e2e 语料全文零命中 = 强缺口信号；命中 ≠ 语义已覆盖
    （同名方法可能测的是另一族）。清单供人工裁决，不是自动判定。
  - gated 归类（GATED_RULES）：零命中但属能力判据项的方法自动归组（K9 网络/SSL、K7 JDBC、
    C-MT 并发调度、indy 运行模型、跨机默认值、K10 资源枚举），不算可行动缺口。
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

# gated 归类：(类名精确/前缀, 方法名集合或 None=整类, 归属说明)。
# 只收「能力判据项」——有明确排期归属、不该由 e2e 追赶的面。
GATED_RULES = [
    ("javax/net/ssl", None, "K9 SSL（证书/握手不可单文件生成）"),
    ("javax/net", {"createSocket", "createServerSocket", "getSocketFactory"}, "K9 SSL 工厂"),
    ("java/net/Socket", None, "K9 socket 选项（SO_LINGER/缓冲/保活等）"),
    ("java/net/ServerSocket", None, "K9 socket 选项"),
    ("java/net/URL", {"openConnection", "setUseCaches", "getContent", "getOutputStream"},
     "K9 网络请求面"),
    ("java/net/URLConnection", {"openConnection", "setUseCaches", "getContent", "getResponseCode",
                                 "setDoOutput", "setRequestProperty", "getOutputStream"},
     "K9 网络请求面"),
    ("java/net/HttpURLConnection", None, "K9 网络请求面"),
    ("java/net/InetAddress", {"getLocalHost", "getHostName", "getCanonicalHostName",
                               "getByAddress", "getAllByName"}, "K9 地址解析"),
    ("java/sql", None, "K7 JDBC（需 H2 驱动落地后随 #9/#10 测）"),
    ("javax/sql", None, "K7 JDBC"),
    ("java/util/concurrent/locks", {"writeLock", "readLock", "tryLock"}, "C-MT 锁真并发语义"),
    ("java/util/concurrent", {"awaitTermination", "shutdownNow", "newThread",
                               "newSingleThreadExecutor", "schedule", "scheduleAtFixedRate",
                               "cancel", "submit", "invokeAll"}, "C-MT 执行器/调度（时序不确定）"),
    ("java/util/Timer", None, "C-MT 定时器"),
    ("java/lang/invoke", {"metafactory", "altMetafactory", "makeConcatWithConstants",
                           "bootstrap", "privateLookupIn", "throwException",
                           "getImplMethodSignature", "getImplMethodName", "getImplMethodKind",
                           "getImplClass", "getFunctionalInterfaceMethodSignature",
                           "getFunctionalInterfaceMethodName", "getFunctionalInterfaceClass",
                           "getCapturedArg"}, "indy 运行模型（闭包/字符串拼接引导，运行模型域）"),
    ("java/util/Locale", {"getDefault", "setDefault"}, "跨机默认 locale（期望文件不可入库）"),
    ("java/util/TimeZone", {"getDefault"}, "跨机默认时区"),
    ("java/time/ZoneId", {"systemDefault"}, "跨机默认时区"),
    ("java/lang/ClassLoader", {"getSystemResources", "getResources"}, "K10 资源枚举"),
    ("java/lang/SecurityManager", None, "安全管理器（JDK17+ 已弃用，检查面归域外）"),
    ("java/lang/System", {"getSecurityManager"}, "安全管理器（JDK17+ 恒 null 已测，检查面归域外）"),
]


def parse_constant_pool(data: bytes):
    """解析常量池 → (utf8 表, 类名集合, 成员引用列表 [(tag, 类名, 名, 描述符)])。"""
    if data[:4] != b"\xca\xfe\xba\xbe":
        return {}, set(), []
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


def gated_label(cls: str, name: str):
    """(类, 方法) → gated 归属说明；非 gated 返回 None。规则按序取首个命中。"""
    for key, names, label in GATED_RULES:
        if cls == key or cls.startswith(key + "/"):
            if names is None or name in names:
                return label
    return None


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("jars", nargs="*", help="jar 路径（默认 pilot-libs 全部）")
    ap.add_argument("--methods", type=int, default=40, help="使用清单深度（默认 40）")
    ap.add_argument("--min-jars", type=int, default=5, help="可行动缺口的 jar 数阈值（默认 5）")
    ap.add_argument("--usage-only", action="store_true", help="只看使用面，跳过缺口对照")
    ap.add_argument("--report", metavar="PATH", help="汇总报告落盘（markdown）")
    args = ap.parse_args()

    jar_paths = [Path(p) for p in args.jars] or sorted(Path(p) for p in glob.glob(str(LIBS / "*.jar")))
    if not jar_paths:
        raise SystemExit(f"[jdk-method-scan] 未找到 jar：{LIBS}（先跑 fetch_pilot_deps.sh）")

    pkg_jars = defaultdict(set)
    cls_jars = defaultdict(set)
    method_jars = defaultdict(set)
    field_jars = defaultdict(set)
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

    lines = [f"# JDK 方法级使用透视（{len(jar_paths)} jar 实测）", "",
             "> 生成：`python3 scripts/jdk_method_scan.py --report <本文件>`；"
             "口径与 gated 归类规则见脚本 docstring。", ""]

    lines.append("## 一、JDK 包级（引用 jar 数前 25）")
    for pkg, jars in sorted(pkg_jars.items(), key=lambda kv: -len(kv[1]))[:25]:
        lines.append(f"- {len(jars):3d} jars — {pkg.replace('/', '.')}")
    lines.append("")

    lines.append("## 二、被 ≥5 jar 引用的 JDK 类")
    for cls, jars in sorted(cls_jars.items(), key=lambda kv: (-len(kv[1]), kv[0])):
        if len(jars) >= 5:
            lines.append(f"- {len(jars):3d} jars — {cls.replace('/', '.')}")
    lines.append("")

    lines.append(f"## 三、方法级使用（被最多 jar 调用的前 {args.methods}）")
    for (cls, mname), jars in sorted(method_jars.items(), key=lambda kv: (-len(kv[1]), kv[0]))[:args.methods]:
        lines.append(f"- {len(jars):3d} jars — {cls.replace('/', '.')}.{mname}{member_desc[(cls, mname)]}")
    lines.append("")

    print("\n".join(lines))

    if not args.usage_only:
        corpus = load_e2e_text()
        gap_start = len(lines)
        # 方法名按最大 jar 数去重（同名跨类取最高使用度）
        best = {}
        best_cls = {}
        for (cls, mname), jars in method_jars.items():
            if mname in ("<init>", "<clinit>"):
                continue
            if len(jars) > best.get(mname, 0):
                best[mname] = len(jars)
                best_cls[mname] = cls

        tiers = [("≥10 jar（核心共用层）", 10, 999), ("5-9 jar（中层）", 5, 9),
                 ("3-4 jar", 3, 4), ("1-2 jar（长尾）", 1, 2)]
        lines.append("## 四、缺口对照（默认产出；方法名级启发式）")
        lines.append("")
        for label, lo, hi in tiers:
            total = sum(1 for n, c in best.items() if lo <= c <= hi)
            zero = sum(1 for n, c in best.items()
                       if lo <= c <= hi and f"{n}(" not in corpus)
            lines.append(f"- {label}：方法名 {total}，零命中 {zero}，覆盖 {100 * (total - zero) // max(total, 1)}%")
        lines.append("")

        # 零命中 → gated 归类 / 可行动
        zero_pairs = [(best[n], best_cls[n], n) for n in best
                      if f"{n}(" not in corpus]
        gated = defaultdict(list)
        actionable = []
        for jars, cls, name in zero_pairs:
            label = gated_label(cls, name)
            if label:
                gated[label].append((jars, cls, name))
            else:
                actionable.append((jars, cls, name))
        actionable = [a for a in actionable if a[0] >= args.min_jars]

        lines.append("### 4a. gated 归属（能力判据项，不由 e2e 追赶）")
        lines.append("")
        if not gated:
            lines.append("- （无）")
        for label in sorted(gated, key=lambda k: -len(gated[k])):
            names = sorted(gated[label], key=lambda x: -x[0])
            head = ", ".join(f"{c.replace('/', '.')}.{n}" for _, c, n in names[:6])
            lines.append(f"- {label}：{len(names)} 个（{head}{'…' if len(names) > 6 else ''}）")
        lines.append("")

        lines.append(f"### 4b. 可行动缺口（≥{args.min_jars} jar 且未归类——按 jar 数排序，供人工裁决补测）")
        lines.append("")
        if not actionable:
            lines.append("- （无——本轮阈值下无可行动缺口）")
        for jars, cls, name in sorted(actionable, key=lambda x: (-x[0], x[1])):
            lines.append(f"- {jars:3d} jars — {cls.replace('/', '.')}.{name}{member_desc.get((cls, name), '')}")
        lines.append("")

        # 静态字段零命中
        zero_fields = []
        seen_f = set()
        for (cls, fname), jars in sorted(field_jars.items(), key=lambda kv: -len(kv[1])):
            if fname in seen_f:
                continue
            seen_f.add(fname)
            if fname not in corpus:
                zero_fields.append((cls, fname, len(jars), member_desc[(cls, fname)]))
        lines.append("### 4c. 静态字段零命中")
        lines.append("")
        if not zero_fields:
            lines.append("- （无）")
        for cls, fname, jars, desc in zero_fields[:30]:
            lines.append(f"- {jars:3d} jars — {cls.replace('/', '.')}.{fname}{desc}")
        lines.append("")

        summary = (f"> 总结：gated {sum(len(v) for v in gated.values())} 个；"
                   f"可行动缺口（≥{args.min_jars} jar）{len(actionable)} 个；"
                   f"静态字段零命中 {len(zero_fields)} 个。")
        lines.append(summary)
        print("\n".join(lines[gap_start:]))

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
