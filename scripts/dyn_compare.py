#!/usr/bin/env python3
"""动态对照（闭包计划 C5 / §3.8）：真实 JVM 类加载轨迹 vs 静态闭包（closure.json）。

只用于验证，不参与闭包计算。每个测试跑一次原始 Java 程序：

    java -Xshare:off -agentpath:<load_trace>=<轨迹> -Xlog:class+load,class+init:file=<日志> <主类>

- `-Xlog`：JVM 实际加载的类全集（静态多出的判据），以及主类初始化（main 即将执行）之后加载的
  「程序期」类集合（漏覆盖的判据；启动器装载与反射找 main 均在此之前，自然排除）。
- `scripts/dyn_agent/load_trace.c`（原生 JVMTI agent，首次使用时以 `$CC`/cc 编译并缓存于
  build/dyn_agent/）：记录每个类加载时的 Java 调用栈。原生 agent 不执行 Java 代码，不扰动加载序列
  （java.lang.instrument agent 的装载会提前加载整套 MethodHandle 基础设施，已弃用）。

程序期加载、不在闭包内的类按以下规则分类（无类名特判；规则数据全部来自清单与 closure.json）：

- 隐藏类（名字含 `/`：lambda 代理、LambdaForm 等运行期定义的类）没有 class 文件，静态闭包按定义不含
  → 计数 `hidden`，不参与对照。
- 类所属域按 closure.toml 判定（与 generator/crates/closure/src/manifest.rs `Manifest::domain` 同口径）：
  边界域类 → `boundary`（翻译域外，由手写层负责，不计漏覆盖）。
- 翻译域 / 用户域类按调用栈自入口帧（栈底）起逐帧上溯：
  - 帧是闭包内按字节码建模的方法 → 继续；
  - 帧是闭包内的手写 / native / 根域方法 → `handwritten`（加载发生在手写实现内部）；
  - 帧的类属边界域 → `boundary-code`；
  - 帧的类列在 closure.toml `[dynamic] vm_upcall_classes`（JVM 链接期直接调用的入口）→ `vm-upcall`；
  - 帧的 `方法:描述符` 与闭包 refs 里某个边界域方法一致（经边界接口 / 类虚派发进入其翻译域实现，
    闭包按边界手写建模、不展开实现）→ `boundary-dispatch`；
  - 栈底帧不在闭包（VM 自行启动的线程 / 入口）→ `vm-entry`；
  - 帧停在闭包 `indy_models` 列出的 invokedynamic 调用点（引导方法由运行模型替换：lambda / 字符串拼接 /
    record 方法 / native 引导），加载发生在该调用点的 JVM 链接期（解析引导方法句柄、执行引导方法）→
    `indy-model`：原生程序不执行引导方法，这些类不属翻译程序；
  - 其余不在闭包的帧（调用方已建模，被调方未建模）→ **漏覆盖**，记录该帧（静态分析漏掉的方法）；
  - 全部帧已建模而类不在闭包 → **漏覆盖**（漏掉的是类引用边）。
- 隐藏类帧（lambda 代理、LambdaForm 编译体：JVMTI 类名含 `.`）透明跳过。
- 无加载事件（agent 盲区）→ `unattributed`。

静态多出 = 闭包内、而 JVM 全程未加载的类；provenance 说明 = 该类的 `via` 边能解析到闭包内的
来源（根 / 闭包内的类 / 闭包内的方法）。

用法（独立运行；run_tests.py 在转译后自动调用）：
    python3 scripts/dyn_compare.py build/jdk21/test_array_list [-o out.json]
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import time
from collections import Counter
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
AGENT_SRC = ROOT / "scripts" / "dyn_agent"
# 与 Manifest::domain 同口径：公开 API 前缀属翻译域，根类单列
PUBLIC_API = ("java/", "javax/")
ROOT_CLASS = "java/lang/Object"
BYTECODE = "bytecode"

# 程序期加载类的分类
MISS = "miss"
UNATTRIBUTED = "unattributed"
ATTRIBUTED = ("handwritten", "boundary-code", "boundary-dispatch", "vm-upcall", "vm-entry")
BOUNDARY = "boundary"


# ── 域判定 ──────────────────────────────────────────────────────────────────────

def entry_matches(entry: str, cls: str) -> bool:
    """清单条目匹配：包前缀（`/` 结尾）或类（含 `$` 嵌套类）。"""
    if entry.endswith("/"):
        return cls.startswith(entry)
    return cls == entry or cls.startswith(entry + "$")


@dataclass
class DomainRules:
    """闭包域判定规则（数据来自 closure.toml / seeds.toml）。"""
    boundary_packages: list[str]
    vm_boundary: set[str]
    release: list[str]
    vm_upcalls: list[str]
    user: set[str] = field(default_factory=set)

    @classmethod
    def from_manifest(cls, user: set[str]) -> "DomainRules":
        sys.path.insert(0, str(ROOT))
        from codegen import runtime_manifest as rm
        return cls(boundary_packages=rm.boundary_packages(),
                   vm_boundary=set(rm.vm_boundary_classes()),
                   release=rm.release_entries() + rm.jca_release_entries(),
                   vm_upcalls=rm.dynamic_vm_upcall_classes(),
                   user=set(user))

    def domain(self, cls: str) -> str:
        if cls in self.user:
            return "user"
        if cls == ROOT_CLASS:
            return "root"
        if any(entry_matches(r, cls) for r in self.release):
            return "translate"
        if any(cls.startswith(p) for p in self.boundary_packages):
            return BOUNDARY
        if cls.split("$", 1)[0] in self.vm_boundary:
            return BOUNDARY
        return "translate" if cls.startswith(PUBLIC_API) else BOUNDARY

    def is_vm_upcall(self, cls: str) -> bool:
        return any(entry_matches(e, cls) for e in self.vm_upcalls)


# ── 轨迹解析 ────────────────────────────────────────────────────────────────────

def _binary_name(dotted: str) -> str:
    return dotted.replace(".", "/")


def parse_xlog(text: str, main_class: str) -> tuple[set[str], list[str], set[str]]:
    """`-Xlog:class+load,class+init` → (加载全集, 程序期加载序列, 隐藏类)。

    程序期 = 主类初始化（`Initializing '<主类>'`，JNI 调 main 前）之后；主类没有初始化行时退回主类
    加载行之后。隐藏类名含 `/`（`Foo$$Lambda/0x…`），单独返回。"""
    loaded: set[str] = set()
    hidden: set[str] = set()
    seq: list[tuple[int, str]] = []
    init_at = load_at = None
    init_marker = f"Initializing '{main_class}'"
    for i, ln in enumerate(text.splitlines()):
        if "[class,load]" in ln:
            rest = ln.split("[class,load]", 1)[1].strip()
            dotted = rest.split(" ", 1)[0]
            if "/" in dotted:
                hidden.add(dotted)
                continue
            name = _binary_name(dotted)
            loaded.add(name)
            seq.append((i, name))
            if name == main_class and load_at is None:
                load_at = i
        elif "[class,init]" in ln and init_at is None and init_marker in ln:
            init_at = i
    start = init_at if init_at is not None else load_at
    program = [n for i, n in seq if start is not None and i > start]
    hidden_program = {h for h in hidden}  # 隐藏类只计数，不区分阶段
    return loaded, program, hidden_program


@dataclass
class LoadEvent:
    thread: str
    frames: list[tuple[str, str, str, int]]   # 栈顶在前：(类, 方法, 描述符, bci)


def parse_agent(text: str) -> dict[str, LoadEvent]:
    """agent 轨迹 → 类 → 首个加载事件（栈顶在前；隐藏类的帧已剔除）。"""
    events: dict[str, LoadEvent] = {}
    cur: LoadEvent | None = None
    for ln in text.splitlines():
        tag, _, rest = ln.partition(" ")
        if tag == "L":
            name, _, thread = rest.partition(" ")
            cur = LoadEvent(thread, [])
            if name not in events:
                events[name] = cur
        elif tag == "F" and cur is not None:
            parts = rest.split(" ")
            if len(parts) == 4 and not is_hidden_frame(parts[0]):
                cur.frames.append((parts[0], parts[1], parts[2], int(parts[3])))
    return events


def is_hidden_frame(cls: str) -> bool:
    """隐藏类（JVMTI 类签名形如 `a/B$$Lambda.0x…`）的帧：运行期生成代码，静态闭包按定义不含，透明跳过。"""
    return "." in cls


# ── 对照 ────────────────────────────────────────────────────────────────────────

def _frame_id(f: tuple[str, str, str, int]) -> str:
    return f"{f[0]}.{f[1]}:{f[2]}"


def _frame_str(f: tuple[str, str, str, int]) -> str:
    return f"{_frame_id(f)}@{f[3]}"


def _sig(mid: str) -> str:
    """`类.方法:描述符` → `方法:描述符`。"""
    head, _, desc = mid.partition(":")
    return head.rsplit(".", 1)[-1] + ":" + desc


def boundary_ref_sigs(refs: list[str], rules: DomainRules) -> set[str]:
    """闭包引用到的边界域方法签名（`方法:描述符`）：经这些方法虚派发进入的实现不在翻译范围。"""
    out = set()
    for r in refs:
        cls = r.partition(":")[0].rsplit(".", 1)[0]
        if rules.domain(cls) == BOUNDARY:
            out.add(_sig(r))
    return out


def attribute(ev: LoadEvent, methods: dict[str, str], rules: DomainRules,
              boundary_sigs: set[str] = frozenset(),
              indy_sites: set[str] = frozenset()) -> tuple[str, str | None]:
    """按调用栈归因一次程序期加载 → (分类, 负责帧)。"""
    frames = list(reversed(ev.frames))           # 栈底在前
    if not frames:
        return "vm-entry", None                  # 无 Java 帧：VM 内部加载
    depth = 0
    while depth < len(frames):
        f = frames[depth]
        mid = _frame_id(f)
        kind = methods.get(mid)
        if kind == BYTECODE:
            if _frame_str(f) in indy_sites:
                # 运行模型替换的 indy：其上方是 JVM 链接期 / 引导产物的执行帧。模型再次进入的已建模方法
                # （拼接时的 toString、lambda 实现方法）从该帧起照常归因；其上全是模型外帧 → 链接期加载
                above = [j for j in range(depth + 1, len(frames)) if methods.get(_frame_id(frames[j])) == BYTECODE]
                if not above:
                    return "indy-model", _frame_str(f)
                depth = above[0]
                continue
            depth += 1
            continue
        if kind is not None:
            return "handwritten", _frame_str(f)
        if rules.domain(f[0]) == BOUNDARY:
            return "boundary-code", _frame_str(f)
        if rules.is_vm_upcall(f[0]):
            return "vm-upcall", _frame_str(f)
        if depth > 0 and _sig(mid) in boundary_sigs:
            return "boundary-dispatch", _frame_str(f)
        if depth == 0:
            return "vm-entry", _frame_str(f)
        return MISS, _frame_str(f)               # 调用方已建模、被调方不在闭包：漏掉的方法
    return MISS, None                            # 全栈已建模：漏掉的是类引用边


def explain(via: dict | None, classes: set[str], methods: dict[str, str]) -> bool:
    """via 边能否解析到闭包内的来源。"""
    if not via or not via.get("kind"):
        return False
    src = via.get("from") or {}
    if "root" in src:
        return True
    if "class" in src:
        return src["class"] in classes
    if "method" in src:
        return src["method"] in methods
    return False


def compare(closure: dict, xlog: str, agent: str, rules: DomainRules,
            main_class: str) -> dict:
    classes = {c["name"]: c for c in closure.get("classes", [])}
    methods = {m["id"]: m.get("kind", "") for m in closure.get("methods", [])}
    loaded, program, hidden = parse_xlog(xlog, main_class)
    events = parse_agent(agent)
    bsigs = boundary_ref_sigs(closure.get("refs", []), rules)
    indy_sites = {x["site"] for x in closure.get("indy_models", [])}

    miss: list[dict] = []
    unattributed: list[dict] = []
    attributed: dict[str, list[dict]] = {}
    seen: set[str] = set()
    for name in program:
        if name in classes or name in seen:
            continue
        seen.add(name)
        dom = rules.domain(name)
        ev = events.get(name)
        cat, frame = attribute(ev, methods, rules, bsigs, indy_sites) if ev is not None else (UNATTRIBUTED, None)
        item = {"class": name, "domain": dom, "frame": frame,
                "thread": ev.thread if ev else None,
                "stack": [_frame_str(f) for f in ev.frames[:16]] if ev else []}
        if dom == BOUNDARY:
            item["attribution"] = cat
            attributed.setdefault(BOUNDARY, []).append(item)
        elif cat == MISS:
            miss.append(item)
        elif cat == UNATTRIBUTED:
            unattributed.append(item)
        else:
            attributed.setdefault(cat, []).append(item)

    extra_names = sorted(n for n in classes if n not in loaded)
    unexplained = [n for n in extra_names
                   if not explain(classes[n].get("via"), set(classes), methods)]
    return {
        "main": main_class,
        "loaded": len(loaded),
        "program_loaded": len(set(program)),
        "hidden": len(hidden),
        "static": len(classes),
        "miss": miss,
        "unattributed": unattributed,
        "attributed": {k: v for k, v in sorted(attributed.items())},
        "extra": {
            "count": len(extra_names),
            "explained": len(extra_names) - len(unexplained),
            "unexplained": unexplained,
            "by_level": dict(Counter(classes[n].get("level") for n in extra_names)),
            "by_via": dict(Counter((classes[n].get("via") or {}).get("kind") for n in extra_names)),
            "classes": extra_names,
        },
    }


# ── 运行 ────────────────────────────────────────────────────────────────────────

def _agent_lib_name() -> str:
    return {"darwin": "libload_trace.dylib", "win32": "load_trace.dll"}.get(sys.platform,
                                                                          "libload_trace.so")


def ensure_agent(java_home: Path, cache_root: Path) -> Path:
    """编译 JVMTI agent（按源码 + JDK 取哈希缓存；并发安全：临时文件构建后原子改名）。"""
    src = AGENT_SRC / "load_trace.c"
    h = hashlib.sha1(str(java_home).encode())
    h.update(src.read_bytes())
    lib = cache_root / h.hexdigest()[:12] / _agent_lib_name()
    if lib.exists():
        return lib
    lib.parent.mkdir(parents=True, exist_ok=True)
    inc = java_home / "include"
    plat = [d for d in inc.iterdir() if d.is_dir()] if inc.is_dir() else []
    cc = os.environ.get("CC", "cc")
    fd, part = tempfile.mkstemp(dir=lib.parent, suffix=".part")
    os.close(fd)
    try:
        r = subprocess.run([cc, "-shared", "-fPIC", "-O1", f"-I{inc}", *(f"-I{d}" for d in plat),
                            "-o", part, str(src)], capture_output=True, text=True)
        if r.returncode != 0:
            raise RuntimeError(f"agent 编译失败（{cc}）：{r.stderr.strip()[-400:]}")
        os.replace(part, lib)
    finally:
        if os.path.exists(part):
            os.unlink(part)
    return lib


def main_class_of(closure: dict) -> str | None:
    for c in closure.get("classes", []):
        if (c.get("via") or {}).get("kind") == "main":
            return c["name"]
    return None


def user_classes(classes_dir: Path) -> set[str]:
    return {p.relative_to(classes_dir).with_suffix("").as_posix()
            for p in classes_dir.rglob("*.class")}


def _java_run(cmd: list[str], cwd: Path, timeout: float) -> str | None:
    try:
        r = subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                           stderr=subprocess.PIPE, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return "timeout"
    return None if r.returncode == 0 else f"exit {r.returncode}"


def run(ws: Path, java_home: Path, agent_cache: Path, cwd: Path,
        timeout: float = 120.0) -> dict:
    """对一个转译 scratch（含 closure_input/）做动态对照，返回结果字典（`error` 键表示未完成）。"""
    t0 = time.perf_counter()
    cin = ws / "closure_input"
    cj = cin / "closure.json"
    classes_dir = cin / "classes"
    if not cj.exists() or not classes_dir.is_dir():
        return {"error": f"缺 {cj} 或 {classes_dir}"}
    closure = json.loads(cj.read_text())
    main = main_class_of(closure)
    if main is None:
        return {"error": "closure.json 无 main 根"}
    java = str(java_home / "bin" / "java")
    try:
        lib = ensure_agent(java_home, agent_cache)
    except RuntimeError as e:
        return {"error": str(e)}
    with tempfile.TemporaryDirectory() as tmp:
        xlog_p = Path(tmp) / "load.log"
        agent_p = Path(tmp) / "agent.txt"
        # 基准与归因同一次运行：JVMTI agent 不执行 Java 代码，不改变加载序列
        err = _java_run([java, "-Xshare:off", f"-agentpath:{lib}={agent_p}",
                         f"-Xlog:class+load=info,class+init=info:file={xlog_p}",
                         "-cp", str(classes_dir), main.replace("/", ".")], cwd, timeout)
        xlog = xlog_p.read_text(errors="replace") if xlog_p.exists() else ""
        agent = agent_p.read_text(errors="replace") if agent_p.exists() else ""
    if not xlog:
        return {"error": f"基准轨迹为空（java {err or '无输出'}）"}
    rules = DomainRules.from_manifest(user_classes(classes_dir))
    res = compare(closure, xlog, agent, rules, main)
    res["java_status"] = err or "ok"
    res["elapsed_s"] = round(time.perf_counter() - t0, 2)
    return res


# ── 输出 ────────────────────────────────────────────────────────────────────────

def provenance_pct(res: dict) -> str:
    ex = res["extra"]
    return "100%" if ex["count"] == 0 else f"{100 * ex['explained'] // ex['count']}%"


def summary_tag(res: dict) -> str:
    """结果行的简短指标：`dyn miss N / extra M prov P%[ / unattr K]`。"""
    if "error" in res:
        return "dyn ERR"
    tag = f"dyn miss {len(res['miss'])} / extra {res['extra']['count']} prov {provenance_pct(res)}"
    if res["unattributed"]:
        tag += f" / unattr {len(res['unattributed'])}"
    return tag


def print_summary(per_test: dict[str, dict]) -> None:
    """汇总：漏覆盖总数与列表、未归因、静态多出的 provenance 覆盖率。"""
    if not per_test:
        return
    ok = {k: v for k, v in per_test.items() if "error" not in v}
    errs = {k: v["error"] for k, v in per_test.items() if "error" in v}
    n_miss = sum(len(v["miss"]) for v in ok.values())
    n_unattr = sum(len(v["unattributed"]) for v in ok.values())
    n_extra = sum(v["extra"]["count"] for v in ok.values())
    n_expl = sum(v["extra"]["explained"] for v in ok.values())
    attr = Counter()
    for v in ok.values():
        for k, items in v["attributed"].items():
            attr[k] += len(items)
    print(f"\n[dyn-compare] {len(ok)} 例：翻译域漏覆盖 {n_miss}，未归因 {n_unattr}，"
          f"静态多出 {n_extra}（provenance {n_expl}/{n_extra}）"
          + (f"，归因 " + " ".join(f"{k}={c}" for k, c in sorted(attr.items())) if attr else "")
          + (f"，对照失败 {len(errs)}" if errs else ""))
    for name, v in sorted(ok.items()):
        for m in v["miss"]:
            print(f"  [miss] {name}: {m['class']}  ← {m['frame'] or '全栈已建模（类引用边）'}")
    for name, v in sorted(ok.items()):
        for m in v["unattributed"]:
            print(f"  [unattr] {name}: {m['class']}")
    for name, v in sorted(ok.items()):
        for n in v["extra"]["unexplained"]:
            print(f"  [no-provenance] {name}: {n}")
    for name, e in sorted(errs.items()):
        print(f"  [error] {name}: {e}")


def main() -> int:
    ap = argparse.ArgumentParser(description="真实 JVM 类加载轨迹 vs 静态闭包（C5 动态对照）")
    ap.add_argument("workspace", help="转译 scratch（含 closure_input/closure.json 与 classes/）")
    ap.add_argument("-o", "--out", help="明细 JSON 输出路径（缺省打印到 stdout）")
    ap.add_argument("--java-home", default=os.environ.get("JAVA_HOME"),
                    help="JDK home（缺省 JAVA_HOME）")
    args = ap.parse_args()
    if not args.java_home:
        sys.exit("需要 --java-home 或 JAVA_HOME")
    ws = Path(args.workspace).resolve()
    res = run(ws, Path(args.java_home), ROOT / "build" / "dyn_agent", ROOT)
    text = json.dumps(res, ensure_ascii=False, indent=1)
    if args.out:
        Path(args.out).write_text(text)
    else:
        print(text)
    print(summary_tag(res), file=sys.stderr)
    print_summary({ws.name: res})
    return 0


if __name__ == "__main__":
    sys.exit(main())
