#!/usr/bin/env python3
"""
端到端测试框架（T51）— per-test scratch workspace 版。

用法：
    python3 scripts/run_tests.py                         # 全量运行（顺序）
    python3 scripts/run_tests.py -j 4                    # 并行运行，最多 4 个并发
    python3 scripts/run_tests.py -j 0                    # 并行运行，并发数 = CPU 核数
    python3 scripts/run_tests.py --filter 01_basics          # 只跑指定目录
    python3 scripts/run_tests.py --filter TestArrayList      # 只跑指定类名
    python3 scripts/run_tests.py --filter 01 02 03           # 多个 filter（任意匹配）
    python3 scripts/run_tests.py --failed                    # 只跑失败清单（build/failed_tests.txt），PASS 自动出列
    python3 scripts/run_tests.py --skip-failed               # 跳过清单内已知失败（干净面快速迭代）
    python3 scripts/run_tests.py --update-expected       # 重新生成 expected/*.txt（并行，-j 控制并发）
    python3 scripts/run_tests.py --no-run                # 只生成 Rust，不执行对比
    python3 scripts/run_tests.py --jdk 25                # 指定 JDK 主版本（javac/java/翻译语料同源）
    python3 scripts/run_tests.py --deny equiv            # 任一等价发射点非零 → 整体失败
    python3 scripts/run_tests.py --deny equiv::neg-array # 细粒度拒绝（对齐 rustc lint 模型）
    python3 scripts/run_tests.py --deny stub-hit         # run 失败的 stub 子族 → 整体失败
    python3 scripts/run_tests.py --deny fallback         # 任一静默兜底点非零 → 整体失败（K-6b 防线）
    python3 scripts/run_tests.py --deny equiv --deny stub-hit   # 可叠加

工作区模型（见 docs/plans/2026-09-16-per-test-scratch-workspace.md）：
    build/<test>/   每测试独立 scratch（手写 overlay + 该测试的生成代码）
    build/target/   共享编译缓存（CARGO_TARGET_DIR：syn/quote/宏 crate 指纹稳定，
                    跨测试复用；java_runtime 因生成内容不同各自编译）

流程：
  对每个 tests/e2e/**/*.java：
  1. main.py 转译 → overlay 手写 + 生成 build/<test>/{java_runtime,user}
  2. cargo run --bin <class>（共享 target 缓存）捕获 stdout
  3. 与 tests/expected/<Class>.txt diff
"""

import argparse
import difflib
import os
import re
import subprocess
import sys
import time
import fcntl
import threading
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from jdk_select import resolve_jdk_home, _major_of


def _current_jdk_major() -> 'int | None':
    """当前生效 JDK 的 major（JAVA_HOME 同源解析；无法解析时 None=旧式不分文件）。"""
    home = os.environ.get('JAVA_HOME', '')
    return _major_of(Path(home)) if home else None

ROOT   = Path(__file__).parent.parent
TESTS  = ROOT / "tests"
E2E    = TESTS / "e2e"
EXPECT = TESTS / "expected"
OUT    = ROOT / "build"     # scratch 根（可被 --out-dir 覆盖）
SHARED_TARGET = OUT / "target"


def fmt_dur(sec: float) -> str:
    """格式化耗时：<60s 用秒（两位小数），≥60s 用 m 分 s 秒。"""
    if sec < 60:
        return f"{sec:.2f}s"
    m, s = divmod(sec, 60)
    return f"{int(m)}m{s:04.1f}s"


# 每测试运行阶段超时（秒）；编译阶段用更宽的上限捕获病态构建
RUN_TIMEOUT = 300
# 构建档位目录（debug/release）：--release 开关切换，bin 路径与 build 命令统一读它
PROFILE_DIR = "debug"


def _cargo_profile_args() -> list[str]:
    return ["--release"] if PROFILE_DIR == "release" else []
BUILD_TIMEOUT = 600


def _jdk_tool(name: str) -> str:
    """javac / java 从 JAVA_HOME 同源解析（未设时回退 PATH）。"""
    home = os.environ.get('JAVA_HOME', '')
    if home:
        p = Path(home) / 'bin' / name
        if p.exists():
            return str(p)
    return name


def apply_jdk_choice(major: int) -> None:
    """--jdk N：解析本机安装并写入 JAVA_HOME（javac/java/翻译语料全部同源）。"""
    home = resolve_jdk_home(major)
    if home is None:
        from jdk_select import list_installed_jdks
        installed = '\n'.join(f"  JDK {m}: {h}" for m, h in list_installed_jdks())
        sys.exit(f"未找到 JDK {major}。本机已安装：\n{installed}")
    os.environ['JAVA_HOME'] = str(home)
    print(f"[jdk] JAVA_HOME → {home} (JDK {major})")


def _cargo_env() -> dict:
    """共享编译缓存环境变量。

    CARGO_INCREMENTAL=0：1500+ 类的宽闭包 crate 上，增量编译的元数据
    双份内存是 OOM 的压垮点（服务器 SIGKILL 实证——同树本地 PASS）；
    scratch 语义下每轮重生成源文件，增量命中本就趋零，关闭无损失。"""
    return dict(os.environ, CARGO_TARGET_DIR=str(SHARED_TARGET),
                CARGO_INCREMENTAL='0')


def _run(cmd: list[str], cwd: Path, capture: bool = True,
         env: dict | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=capture, text=True, env=env)


def _discover(filter_str: list[str] | None) -> list[Path]:
    files = sorted(E2E.rglob("*.java"))
    if filter_str:
        files = [f for f in files if any(s in str(f) for s in filter_str)]
    return files


def _class_name(java_file: Path) -> str:
    return java_file.stem


def _to_bin_name(class_name: str) -> str:
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', class_name)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    return s.lower()


# ── 行输出格式化：时间戳 + 列对齐 + 转译辅助列（引用类计数 / 语料混用告警）──
_CLASSES_RE = re.compile(r"完成[，,]\s*(\d+) 个调用链类 \+ (\d+) 个 field stub = (\d+) 个")
_CORPUS_RE = re.compile(r"语料选择：用户类 class 版本 (\d+) → JDK (\d+)")


def _now_hms() -> str:
    return time.strftime("%H:%M:%S")


def _name_width(files: list) -> int:
    """测试名列宽：按发现清单自适应（相对 tests/e2e/ 的路径），夹在 [34, 58]。"""
    lens = [len(str(f.relative_to(E2E))) for f in files] or [34]
    return min(max(max(lens), 34), 58)


def _transpile_aux(log: str) -> str:
    """转译输出的行尾辅助列：BFS 引用类计数（调用链+field stub）+ 语料/JDK
    混用告警（class 版本-44 ≠ 语料 JDK 时——TestListOf 教训：PATH javac 25
    劫持会让同一测试在两个 JDK 语料间漂移）。"""
    parts = []
    m = _CLASSES_RE.search(log)
    if m:
        parts.append(f"cls {m.group(1)}+{m.group(2)}")
    c = _CORPUS_RE.search(log)
    if c and int(c.group(1)) - 44 != int(c.group(2)):
        parts.append(f"MIX! class{c.group(1)}->jdk{c.group(2)}")
    return " | ".join(parts)


def _pline(name_w: int, status: str, name, tail: str = "", aux: str = "",
           prog: str = "") -> None:
    """统一行输出（两模式共用）：
    [HH:MM:SS] [ N/M] [PASS] name<pad> tail  | cls ..+.. | MIX! ..
    状态列宽 5 保持 [ PASS ] / [ FAIL ] 既有 token 不变（下游 grep 兼容）。"""
    head = f"[{_now_hms()}] {prog} " if prog else f"[{_now_hms()}] "
    line = f"{head}[{status:^5}] {str(name):<{name_w}}"
    if tail:
        line += f" {tail}"
    if aux:
        line += f"  | {aux}"
    print(line, flush=True)


def _bin_size_mb(bin_name: str) -> str:
    """构建产物大小（闭包膨胀观测列；TestTernary JDK25 1611 类 30MB 的教训）。"""
    p = SHARED_TARGET / PROFILE_DIR / bin_name
    try:
        return f"{p.stat().st_size / 1048576:.1f}M"
    except OSError:
        return ""


def _fmt_build_dur(dur: float) -> str:
    """构建耗时列：缓存命中（<0.5s，纯指纹比对无重编）加 (cache) 标记。"""
    return fmt_dur(dur) + ("(cache)" if dur < 0.5 else "")


def _aux_full(cls_aux: str, raw_v: int, eq_v: int, bin_name: str = "",
             eta: str = "") -> str:
    """行尾辅助列组装：cls | raw | eq | bin | eta（缺省项自动省略）。"""
    parts = [p for p in (cls_aux,
                         f"raw {raw_v}" if raw_v else "",
                         f"eq {eq_v}" if eq_v else "",
                         f"bin {_bin_size_mb(bin_name)}" if bin_name else "",
                         eta) if p]
    return " | ".join(parts)


# ── 失败清单（棘轮）：全量跑批累积失败，--failed 只跑清单，通过自动出列 ──
def _failed_header(jdk_major: int | None) -> str:
    # 头部 `# jdk: N` 是清单的身份行：同一文件只服务一个 JDK 版本的失败集
    # （不同版本语料/手写覆盖面不同，失败集不可比），读取侧据此校验防混用
    return ("# run_tests.py 失败清单（自动维护，勿手编）：\n"
            "# - 任何一次跑批：FAIL 进列 / 跑到且 PASS 出列 / 未跑的不动\n"
            "# - --failed 按本清单回归；--failed-file 可改路径\n"
            f"# jdk: {jdk_major}\n")


_FAILED_HEADER = _failed_header(None)


def _failed_file_path(cli_path: str | None, jdk_major: int | None = None) -> Path:
    # 默认清单按 JDK 版本分文件：不同版本的失败集互不可比（语料/手写覆盖面
    # 都随版本变化），混在同一清单里 --failed 会在错误版本下重跑假失败
    if cli_path:
        return Path(cli_path)
    if jdk_major is not None:
        return OUT / f"failed_tests_jdk{jdk_major}.txt"
    return OUT / "failed_tests.txt"



def _load_failed(path: Path) -> set:
    if not path.exists():
        return set()
    return {ln.strip() for ln in path.read_text(encoding='utf-8').splitlines()
            if ln.strip() and not ln.startswith('#')}


def _failed_jdk_of(path: Path) -> 'int | None':
    """清单头部 `# jdk: N` 的解析（无头部=旧格式，返回 None 不校验）。"""
    if not path.exists():
        return None
    for ln in path.read_text(encoding='utf-8').splitlines():
        m = re.match(r'#\s*jdk:\s*(\d+)', ln.strip())
        if m:
            return int(m.group(1))
    return None


def _save_failed(path: Path, names: set, jdk_major: 'int | None' = None) -> None:
    """原子写（临时文件 + os.replace）：其他进程任意时刻读到的都是完整文件。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(_failed_header(jdk_major) + "".join(f"{n}\n" for n in sorted(names)),
                   encoding="utf-8")
    os.replace(tmp, path)


class _FailedRatchet:
    """失败清单写穿棘轮：每个测试出结果**立即**落盘（边测边进/出列），
    进程被中断（Ctrl-C/超时杀）也不丢已累积的结果；跑批中可随时 tail
    该文件观察。

    **多进程并发安全**（一个全量 + 一个定向跑批共用同一清单）：
    每次记录在跨进程文件锁（flock 清单同名 .lock）内做「重读 → 合并 → 原子写」
    ——写前重读文件吸收其他进程已落盘的更新，不会用本进程的内存快照覆盖掉
    别人的条目；线程锁负责单进程内并行 worker 的串行化。"""

    def __init__(self, path: Path, prev: set, jdk_major: 'int | None' = None):
        self.path = path
        self.failed = set(prev)
        self.jdk_major = jdk_major
        self.removed = self.added = 0
        self._lock = threading.Lock()

    def record(self, name: str, ok: bool) -> None:
        with self._lock:
            lock_path = self.path.with_suffix(self.path.suffix + ".lock")
            with open(lock_path, "w") as lf:
                fcntl.flock(lf, fcntl.LOCK_EX)
                try:
                    current = _load_failed(self.path)   # 重读：吸收其他进程的写入
                    if ok:
                        if name in current:
                            current.discard(name)
                            self.removed += 1
                    else:
                        if name not in current:
                            self.added += 1
                        current.add(name)
                    self.failed = current
                    _save_failed(self.path, current, self.jdk_major)
                finally:
                    fcntl.flock(lf, fcntl.LOCK_UN)

    def summary(self) -> None:
        # 清单路径可能不在仓库内（--failed-file /tmp/... 或相对路径）——
        # relative_to 会 ValueError，展示用回退原样路径
        try:
            shown = str(self.path.relative_to(ROOT))
        except ValueError:
            shown = str(self.path)
        print(f"[failed-file] {shown}：保留 {len(self.failed)}"
              f"（本次出列 {self.removed}、进列 {self.added}）—— `--failed` 按此回归")


def _apply_failed_filter(files: list, failed_set: set) -> list:
    return [f for f in files if str(f.relative_to(ROOT)) in failed_set]


def _print_env_header() -> None:
    """环境头：跨机器日志可比性（OS/架构 + 工具链版本 + 代码版本 + 影响
    生成/编译行为的开关）。git 哈希与 dirty 标记让每份日志可追溯到确切树
    （dirty=工作树有未提交改动，结果解释需注意）；PYTHONHASHSEED 未设时
    每进程随机（双种子验证需显式固定）；CARGO_INCREMENTAL 影响编译内存
    行为（服务器 OOM 缓解）。"""
    import platform
    try:
        cargo_v = subprocess.run(["cargo", "--version"], capture_output=True,
                                 text=True, timeout=10).stdout.strip()
    except Exception:
        cargo_v = "cargo ?"
    print(f"[env] {platform.platform()} | {cargo_v}")

    def _git_desc() -> str:
        try:
            head = subprocess.run(["git", "rev-parse", "--short", "HEAD"],
                                  capture_output=True, text=True, timeout=10,
                                  cwd=ROOT).stdout.strip()
            dirty = subprocess.run(["git", "status", "--porcelain"],
                                   capture_output=True, text=True, timeout=10,
                                   cwd=ROOT)
            if not head:
                return "git ?"
            return head + (" (dirty)" if dirty.stdout.strip() else "")
        except Exception:
            return "git ?"

    _flag_vars = ("PYTHONHASHSEED", "CARGO_INCREMENTAL", "CARGO_BUILD_JOBS",
                  "JAVA_RTA_DEBUG", "JAVA_RTA_STRICT", "JAVA_RTA_BFS_EDGE_AUDIT")
    _flags = " ".join(f"{k}={os.environ.get(k, '(unset)')}" for k in _flag_vars)
    print(f"[meta] git {_git_desc()} | profile={PROFILE_DIR} | {_flags} | out={OUT}")


def _test_workspace(bin_name: str) -> Path:
    """每测试独立 scratch 工作区。"""
    return OUT / bin_name


def _transpile(java_file: Path, out_dir: Path) -> tuple[bool, str]:
    """运行转译器：overlay 手写代码 + 生成该测试的 Rust 代码。"""
    args = [sys.executable, str(ROOT / "scripts" / "main.py"), str(java_file),
            "--no-run", "--out", str(out_dir)]
    r = _run(args, cwd=ROOT)
    return r.returncode == 0, (r.stdout + r.stderr)


def _cargo_build(class_name: str, out_dir: Path) -> tuple[bool, str]:
    """cargo build --bin <class>（共享 target 缓存）。返回 (ok, 首个 error 行)。"""
    bin_name = _to_bin_name(class_name)
    try:
        r = _run(["cargo", "build", *_cargo_profile_args(), "--bin", bin_name], cwd=out_dir, env=_cargo_env())
    except subprocess.TimeoutExpired:
        return False, f"build timeout ({fmt_dur(BUILD_TIMEOUT)})"
    if r.returncode != 0:
        err = next((ln for ln in r.stderr.splitlines() if ln.startswith("error")),
                   "unknown build error")
        return False, err[:120]
    return True, ""


# ── V-3 可读性审计汇总 ──────────────────────────────────────────────

_READABILITY_RE = re.compile(r"^\[readability-audit\]\s+(.+)$", re.MULTILINE)


def _parse_readability(log: str) -> dict[str, int]:
    """从转译输出解析 [readability-audit] 行的禁用调用计数（A-2 口径）。"""
    m = _READABILITY_RE.search(log)
    if not m:
        return {}
    counts: dict[str, int] = {}
    for part in m.group(1).split():
        k, _, v = part.partition("=")
        try:
            counts[k] = int(v)
        except ValueError:
            pass
    return counts


def _print_readability_summary(per_test: dict[str, dict[str, int]]) -> None:
    """汇总各测试的可读性审计计数；目标全 0（A-2 禁用调用清零的验收口径）。"""
    if not per_test:
        return
    totals: dict[str, int] = {}
    nonzero: list[str] = []
    for name, counts in sorted(per_test.items()):
        if not counts:
            continue
        bad = {k: v for k, v in counts.items() if v}
        for k, v in counts.items():
            totals[k] = totals.get(k, 0) + v
        if bad:
            nonzero.append(f"{name}({', '.join(f'{k}={v}' for k, v in sorted(bad.items()))})")
    line = " ".join(f"{k}={v}" for k, v in sorted(totals.items()))
    print(f"\n[readability] {line}  (合计 {sum(totals.values())}，"
          f"{len(per_test)} 个测试，目标全 0)")
    if nonzero:
        shown = "; ".join(nonzero[:6])
        print(f"  非零: {shown}{'…' if len(nonzero) > 6 else ''}")


# ── 等价发射点审计汇总（compatibility.md §4；对齐 readability 模式） ─────

_EQUIV_RE = re.compile(r"^\[equiv-audit\]\s+(.+)$", re.MULTILINE)
_RAW_RE = re.compile(r"^\[raw-audit\]\s+(.+)$", re.MULTILINE)
_RAW_TOTALS: dict[str, int] = {}

# 可 --deny 的等价 ID（= codegen/equiv_audit.py 的发射口径全集；
# monitor-mt 已随 S-20 落地补埋（2026-09-21）；stacktrace 无 codegen 发射点，不在列）
EQUIV_IDS = (
    'identity-hash', 'intern-identity', 'null-array', 'boxed-null',
    'class-literal', 'record-hash', 'neg-array', 'field-npe', 'class-init',
    'monitor-mt',
)


def _parse_equiv(log: str) -> dict[str, int]:
    """从转译输出解析 [equiv-audit] 行的等价发射点计数（全零行是 `none`）。"""
    m = _EQUIV_RE.search(log)
    if not m:
        return {}
    counts: dict[str, int] = {}
    for part in m.group(1).split():
        k, _, v = part.partition("=")
        try:
            counts[k] = int(v)
        except ValueError:
            pass
    return counts


def _print_equiv_summary(per_test: dict[str, dict[str, int]]) -> None:
    """汇总各测试的等价发射点计数。口径：发射点数而非缺陷数——近似等价允许
    存在，目标是可观测（--deny 可升级），与 readability 的「目标全 0」不同。"""
    if not per_test:
        return
    totals: dict[str, int] = {}
    nonzero: list[str] = []
    for name, counts in sorted(per_test.items()):
        if not counts:
            continue
        bad = {k: v for k, v in counts.items() if v}
        for k, v in counts.items():
            totals[k] = totals.get(k, 0) + v
        if bad:
            nonzero.append(f"{name}({', '.join(f'{k}={v}' for k, v in sorted(bad.items()))})")
    line = " ".join(f"{k}={v}" for k, v in sorted(totals.items())) or "none"
    print(f"\n[equiv] {line}  (合计 {sum(totals.values())}，{len(per_test)} 个测试，"
          f"口径：发射点数非缺陷数，观测即可，--deny 可升级)")
    if nonzero:
        shown = "; ".join(nonzero[:6])
        print(f"  非零: {shown}{'…' if len(nonzero) > 6 else ''}")


# ── 静默兜底审计汇总（fallback-audit 方案 §4.3；对齐 equiv 模式） ──────

_FALLBACK_RE = re.compile(r"^\[fallback-audit\]\s+(.+)$", re.MULTILINE)
_FALLBACK_RUNS = 0   # 带 [fallback-audit] 行的转译次数（全零时汇总也可见）

# 可 --deny 的兜底 ID（= codegen/fallback_audit.py 的 B 组口径全集）
FALLBACK_IDS = (
    'sig-parse-field', 'sig-parse-method', 'type-map-params',
    'vars-render-loop', 'vars-render-if', 'vars-type-decl',
    'vars-type-outer', 'vars-type-later',
    'sam-functional', 'sam-prescan',
    'cc-load-class', 'cc-root-names', 'cc-root-desc',
    'cc-stub-chan', 'cc-parent-queue',
)


def _parse_fallback(log: str) -> dict[str, int]:
    """从转译输出解析 [fallback-audit] 行的静默兜底计数（全零行是 `none`，
    计入 _FALLBACK_RUNS 使全零口径在汇总可见）。"""
    global _FALLBACK_RUNS
    m = _FALLBACK_RE.search(log)
    if not m:
        return {}
    _FALLBACK_RUNS += 1
    counts: dict[str, int] = {}
    for part in m.group(1).split():
        k, _, v = part.partition("=")
        try:
            counts[k] = int(v)
        except ValueError:
            pass
    return counts


def _print_fallback_summary(per_test: dict[str, dict[str, int]]) -> None:
    """汇总各测试的静默兜底计数。目标全 0：B 组收窄是死代码收窄（2026-09-23
    审计实证全语料零触发），任何非零都极可能是真 bug（K-6b / typeir-b3 型
    「安全网吞 bug」）——与 equiv 的「观测即可」不同，非零应当排查。"""
    if not per_test:
        if _FALLBACK_RUNS:
            print(f"\n[fallback] none  (合计 0，{_FALLBACK_RUNS} 次转译零触发——"
                  f"死代码收窄零损失口径成立)")
        return
    totals: dict[str, int] = {}
    nonzero: list[str] = []
    for name, counts in sorted(per_test.items()):
        if not counts:
            continue
        bad = {k: v for k, v in counts.items() if v}
        for k, v in counts.items():
            totals[k] = totals.get(k, 0) + v
        if bad:
            nonzero.append(f"{name}({', '.join(f'{k}={v}' for k, v in sorted(bad.items()))})")
    line = " ".join(f"{k}={v}" for k, v in sorted(totals.items())) or "none"
    print(f"\n[fallback] {line}  (合计 {sum(totals.values())}，{len(per_test)} 个测试，"
          f"目标全 0——非零=收窄后仍触发的兜底，极可能是真 bug)")
    if nonzero:
        shown = "; ".join(nonzero[:6])
        print(f"  非零: {shown}{'…' if len(nonzero) > 6 else ''}")


def _parse_raw(log: str) -> int:
    """累加 [raw-audit] 行（收敛路线 L5-b）并返回本测试的 raw 合计（行尾列用）。"""
    m = _RAW_RE.search(log)
    if not m:
        return 0
    per = 0
    for p in m.group(1).split():
        k, _, v = p.partition('=')
        if k in ('raw_expr', 'raw_stmt'):
            _RAW_TOTALS[k] = _RAW_TOTALS.get(k, 0) + int(v)
            per += int(v)
        elif k == 'type_surgery_sites':
            _RAW_TOTALS['sites'] = int(v)   # 静态度量，取末值
    _RAW_TOTALS['runs'] = _RAW_TOTALS.get('runs', 0) + 1
    return per


def _summarize_raw() -> None:
    """[raw] 汇总：本次跑批的 Raw 发射与类型手术静态位点（收敛路线 L5-b / 阶段 A）。
    终态全 0（Raw 全部类型化 IR、类型查询全经 TypeIR）；趋势只降不升。"""
    if not _RAW_TOTALS.get('runs'):
        return
    e = _RAW_TOTALS.get('raw_expr', 0)
    s = _RAW_TOTALS.get('raw_stmt', 0)
    print(f"\n[raw] raw_expr={e} raw_stmt={s}  "
          f"(合计 {e + s}，{_RAW_TOTALS['runs']} 次转译；"
          f"type_surgery_sites={_RAW_TOTALS.get('sites', 0)} 为源码静态位点)"
          f"——终态全 0，趋势只降不升")


# ── run 失败子族自动分类（直跑二进制抓 stderr，不经 cargo） ────────────
#
# 失败五分类（transpile / compile / run-timeout / run / output）之后，对 run 族
# 失败自动重跑二进制并按 stderr 分类子族：
#   stub-hit       panic 消息以 `stub: ` 开头——调用链内未翻译方法的可见 stub
#   native-hit     panic 消息含 `native: ` —— 手写 native 的显式未实现标记
#   s8-crash       `capacity overflow` —— S-8 负长度直通 Vec 分配的进程崩溃族
#   runtime-panic  其余 panic —— 运行期非预期崩溃
# 子族经 --deny stub-hit 可升级为整体失败；其余子族的 deny 待后续按需扩展。

RUN_SUBFAMILIES = ('stub-hit', 'native-hit', 's8-crash', 'runtime-panic')


def _classify_run_failure(class_name: str) -> tuple[str, str]:
    """run 失败后直跑二进制抓 stderr 分类。返回 (子族, 摘要)；
    子族为空串表示无法分类（如重跑超时 / 无 panic 输出）。"""
    bin_path = SHARED_TARGET / PROFILE_DIR / _to_bin_name(class_name)
    if not bin_path.exists():
        return "", "binary missing on re-run"
    try:
        r = subprocess.run([str(bin_path)], capture_output=True, text=True,
                           timeout=RUN_TIMEOUT)
    except subprocess.TimeoutExpired:
        return "", "timeout on re-run"
    stderr = r.stderr or ""
    if r.returncode == 0:
        return "", "re-run exited 0 (flaky)"
    m = re.search(r"stub: \S[^\n]*", stderr)
    if m:
        return "stub-hit", m.group(0).strip()
    m = re.search(r"native: \S[^\n]*", stderr)
    if m:
        return "native-hit", m.group(0).strip()
    if "capacity overflow" in stderr:
        return "s8-crash", "capacity overflow (S-8 NegativeArraySizeException 缺失)"
    m = re.search(r"panicked at [^\n]*", stderr)
    if m:
        return "runtime-panic", m.group(0).strip()[:120]
    return "", (stderr.strip().splitlines() or ["no stderr"])[-1][:120]


def _print_run_subfamily_summary(sub: dict[str, tuple[str, str]]) -> None:
    """run 族失败的子族标注（追加在失败五分类之后，只新增行）。"""
    if not sub:
        return
    by_fam: dict[str, list[str]] = {}
    for name in sorted(sub):
        fam, detail = sub[name]
        by_fam.setdefault(fam or "unclassified", []).append(f"{name}[{detail}]")
    parts = [f"{fam}={len(names)} ({', '.join(names[:4])}{'…' if len(names) > 4 else ''})"
             for fam, names in sorted(by_fam.items())]
    print(f"[run-classify] run 族失败子族: " + "；".join(parts))


# ── --deny 拒绝升级（compatibility.md §4.3；默认全放行） ──────────────


def _validate_deny(deny: list[str]) -> None:
    """校验 --deny 规格：equiv / equiv::<id> / stub-hit / fallback / fallback::<id>。"""
    valid = ({'equiv', 'stub-hit', 'fallback'}
             | {f"equiv::{i}" for i in EQUIV_IDS}
             | {f"fallback::{i}" for i in FALLBACK_IDS})
    bad = [d for d in deny if d not in valid]
    if bad:
        known = {'equiv': f"equiv::<id>（id ∈ {', '.join(EQUIV_IDS)}）",
                 'fallback': f"fallback::<id>（id ∈ {', '.join(FALLBACK_IDS)}）"}
        sys.exit(f"无效 --deny 规格: {', '.join(bad)}\n"
                 f"可用: {' | '.join(['equiv', *sorted(known.values()), 'stub-hit'])}")


def _deny_violations(deny: list[str],
                     equiv_counts: dict[str, dict[str, int]],
                     run_sub: dict[str, tuple[str, str]],
                     fallback_counts: dict[str, dict[str, int]]) -> list[str]:
    """按 --deny 规格收集违规消息（不改变测试通过判定本身，只影响退出码）。"""
    msgs: list[str] = []
    for spec in deny:
        if spec == 'equiv':
            for name, counts in sorted(equiv_counts.items()):
                bad = {k: v for k, v in counts.items() if v}
                if bad:
                    msgs.append(f"--deny equiv: {name} "
                                + ', '.join(f"{k}={v}" for k, v in sorted(bad.items())))
        elif spec.startswith('equiv::'):
            eid = spec[len('equiv::'):]
            for name, counts in sorted(equiv_counts.items()):
                if counts.get(eid):
                    msgs.append(f"--deny {spec}: {name} {eid}={counts[eid]}")
        elif spec == 'fallback':
            for name, counts in sorted(fallback_counts.items()):
                bad = {k: v for k, v in counts.items() if v}
                if bad:
                    msgs.append(f"--deny fallback: {name} "
                                + ', '.join(f"{k}={v}" for k, v in sorted(bad.items())))
        elif spec.startswith('fallback::'):
            fid = spec[len('fallback::'):]
            for name, counts in sorted(fallback_counts.items()):
                if counts.get(fid):
                    msgs.append(f"--deny {spec}: {name} {fid}={counts[fid]}")
        elif spec == 'stub-hit':
            for name in sorted(run_sub):
                fam, detail = run_sub[name]
                if fam == 'stub-hit':
                    msgs.append(f"--deny stub-hit: {name} [{detail}]")
    return msgs


def _apply_deny(deny: list[str],
                equiv_counts: dict[str, dict[str, int]],
                run_sub: dict[str, tuple[str, str]], failed: int,
                fallback_counts: dict[str, dict[str, int]] | None = None) -> int:
    """打印违规并决定最终退出码：deny 命中时整体失败（即使测试全 PASS）。"""
    if not deny:
        return 1 if failed else 0
    violations = _deny_violations(deny, equiv_counts, run_sub, fallback_counts or {})
    if violations:
        print(f"\n[deny] {len(violations)} 处违规（--deny {' '.join(deny)}）:")
        for msg in violations:
            print(f"  {msg}")
        return 1
    print(f"\n[deny] {' '.join(deny)}: 0 违规")
    return 1 if failed else 0


def _run_bin(class_name: str, timeout: int = RUN_TIMEOUT) -> tuple[str, str]:
    """直接执行 binary。返回 (状态, stdout)：状态 ∈ ok / timeout / error。"""
    bin_name = _to_bin_name(class_name)
    bin_path = SHARED_TARGET / PROFILE_DIR / bin_name
    try:
        r = subprocess.run([str(bin_path)], capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return "timeout", ""
    if r.returncode != 0:
        return "error", r.stdout
    return "ok", r.stdout


def _run_binary(class_name: str) -> tuple[bool, str]:
    """直接执行已编译的 binary（共享 target 目录下，不经 cargo 避免锁竞争）。"""
    bin_name = _to_bin_name(class_name)
    bin_path = SHARED_TARGET / PROFILE_DIR / bin_name
    r = subprocess.run([str(bin_path)], capture_output=True, text=True)
    return r.returncode == 0, r.stdout


def _expected_path(class_name: str) -> Path:
    return EXPECT / f"{class_name}.txt"


def _read_expected(class_name: str) -> str | None:
    p = _expected_path(class_name)
    return p.read_text() if p.exists() else None


def _diff(expected: str, actual: str, class_name: str) -> list[str]:
    return list(difflib.unified_diff(
        expected.splitlines(keepends=True),
        actual.splitlines(keepends=True),
        fromfile=f"expected/{class_name}.txt",
        tofile=f"cargo run --bin {_to_bin_name(class_name)}",
    ))


def _update_expected(java_file: Path) -> tuple[str, str]:
    """生成期望输出。返回 (状态, 摘要)：状态 ∈ updated / javac-fail / java-fail / timeout。

    classes 目录按测试独立（build/expected-classes/<bin>），支持并行无覆盖。"""
    class_name = _class_name(java_file)
    classes_dir = OUT / "expected-classes" / _to_bin_name(class_name)
    classes_dir.mkdir(parents=True, exist_ok=True)
    r = _run([_jdk_tool("javac"), "-g", "-d", str(classes_dir), str(java_file)], cwd=ROOT)
    if r.returncode != 0:
        return "javac-fail", r.stderr.strip().splitlines()[-1] if r.stderr.strip() else "javac error"
    try:
        r = subprocess.run([_jdk_tool("java"), "-cp", str(classes_dir), class_name],
                           cwd=ROOT, capture_output=True, text=True, timeout=RUN_TIMEOUT)
    except subprocess.TimeoutExpired:
        return "timeout", f"exceeded {fmt_dur(RUN_TIMEOUT)}"
    if r.returncode != 0:
        return "java-fail", (r.stderr.strip().splitlines() or ["java error"])[-1][:120]
    _expected_path(class_name).write_text(r.stdout)
    return "updated", f"{r.stdout.count(chr(10))} lines"


# ── 顺序模式 ─────────────────────────────────────────────────────────

def _run_sequential(filter_str: list[str] | None, no_run: bool,
                    deny: list[str], use_failed: bool = False,
                    failed_path: Path | None = None,
                    skip_failed: bool = False,
                    jdk_major: 'int | None' = None) -> int:
    failed_path = failed_path or _failed_file_path(None, jdk_major)
    prev_failed = _load_failed(failed_path)
    files = _discover(filter_str)
    if use_failed:
        files = _apply_failed_filter(files, prev_failed)
        if not files:
            print(f"失败清单为空或与 filter 无交集（{failed_path}）。")
            return 0
        print(f"[failed] 回归模式：清单 {len(prev_failed)} 个，本次运行 {len(files)} 个")
    elif skip_failed:
        files = [f for f in files if str(f.relative_to(ROOT)) not in prev_failed]
        print(f"[skip-failed] 跳过清单内 {len(prev_failed)} 个已知失败，本次运行 {len(files)} 个")
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    print(f"[start] {time.strftime('%Y-%m-%d %H:%M:%S')} | sequential"
          + (f" | filter: {' '.join(filter_str)}" if filter_str else ""))
    _print_env_header()
    name_w = _name_width(files)
    t_seq_start = time.perf_counter()

    passed = failed = skipped = 0
    t_all = time.perf_counter()
    t_transpile_total = t_build_total = t_run_total = 0.0
    readability_counts: dict[str, dict[str, int]] = {}
    equiv_counts: dict[str, dict[str, int]] = {}
    fallback_counts: dict[str, dict[str, int]] = {}
    run_sub: dict[str, tuple[str, str]] = {}
    fail_categories: dict[str, list[str]] = {}

    ratchet = _FailedRatchet(failed_path, prev_failed, jdk_major)

    def _fail(cat: str, rel_str: str) -> None:
        nonlocal failed
        failed += 1
        fail_categories.setdefault(cat, []).append(rel_str)
        ratchet.record(rel_str, False)

    total_files = len(files)
    for idx, java_file in enumerate(files, 1):
        prog = f"[{idx:>3}/{total_files}]"
        _eta = ""
        if idx > 1 and idx < total_files:
            _eta = f"eta {fmt_dur((time.perf_counter() - t_seq_start) / idx * (total_files - idx))}"
        class_name = _class_name(java_file)
        bin_name   = _to_bin_name(class_name)
        rel        = java_file.relative_to(ROOT)
        expected   = _read_expected(class_name)

        if expected is None:
            print(f"{prog} [ SKIP ] {rel}  (no expected/{class_name}.txt)")
            skipped += 1
            continue

        ws = _test_workspace(bin_name)
        t0 = time.perf_counter()
        ok, log = _transpile(java_file, out_dir=ws)
        t_transpile = time.perf_counter() - t0
        t_transpile_total += t_transpile
        if not ok:
            _pline(name_w, "FAIL", java_file.relative_to(E2E),
                   f"— transpile error ({fmt_dur(t_transpile)})",
                   aux=_aux_full("", 0, 0, eta=_eta), prog=prog)
            print(log[-500:])
            _fail("transpile", str(rel))
            continue

        _rc = _parse_readability(log)
        if _rc:
            readability_counts[class_name] = _rc
        _ec = _parse_equiv(log)
        _fc = _parse_fallback(log)
        _raw_per = _parse_raw(log)
        _eq_sum = sum(_ec.values()) if _ec else 0
        _parse_raw(log)
        if _ec:
            equiv_counts[class_name] = _ec
        if _fc:
            fallback_counts[class_name] = _fc

        if no_run:
            _pline(name_w, "NORUN", java_file.relative_to(E2E),
                   f"— transpile OK ({fmt_dur(t_transpile)})",
                   aux=_aux_full(_transpile_aux(log), _raw_per, _eq_sum, eta=_eta), prog=prog)
            skipped += 1
            continue

        t0 = time.perf_counter()
        ok, err = _cargo_build(class_name, out_dir=ws)
        t_build = time.perf_counter() - t0
        t_build_total += t_build
        if not ok:
            _pline(name_w, "FAIL", java_file.relative_to(E2E),
                   f"— compile error ({_fmt_build_dur(t_build)})  {err}",
                   aux=_aux_full(_transpile_aux(log), _raw_per, _eq_sum, eta=_eta), prog=prog)
            _fail("compile", str(rel))
            continue

        t0 = time.perf_counter()
        status, actual = _run_bin(class_name)
        t_run = time.perf_counter() - t0
        t_run_total += t_run
        timing = f"transpile {fmt_dur(t_transpile)}, build {_fmt_build_dur(t_build)}, run {fmt_dur(t_run)}"
        if status == "timeout":
            _pline(name_w, "FAIL", java_file.relative_to(E2E),
                   f"— run timeout (> {fmt_dur(RUN_TIMEOUT)})  ({timing})",
                   aux=_aux_full(_transpile_aux(log), _raw_per, _eq_sum, bin_name, _eta), prog=prog)
            _fail("run-timeout", str(rel))
            continue
        if status == "error":
            # run 族失败：直跑二进制抓 stderr 自动分类子族（stub-hit 等）
            fam, detail = _classify_run_failure(class_name)
            run_sub[class_name] = (fam, detail)
            _pline(name_w, "FAIL", java_file.relative_to(E2E),
                   f"— run error  ({timing})",
                   aux=_aux_full(_transpile_aux(log), _raw_per, _eq_sum, bin_name, _eta), prog=prog)
            _fail("run", str(rel))
            continue

        diff = _diff(expected, actual, class_name)
        if diff:
            _n_diff = sum(1 for l in diff if l[:1] in "+-" and not l.startswith(("+++", "---")))
            _pline(name_w, "FAIL", java_file.relative_to(E2E),
                   f"— output mismatch · diff {_n_diff} 行  ({timing})",
                   aux=_aux_full(_transpile_aux(log), _raw_per, _eq_sum, bin_name, _eta), prog=prog)
            print("".join(diff[:40]))
            if len(diff) > 40:
                print(f"  … ({len(diff) - 40} more lines)")
            _fail("output", str(rel))
        else:
            _pline(name_w, "PASS", java_file.relative_to(E2E),
                   f"({timing})",
                   aux=_aux_full(_transpile_aux(log), _raw_per, _eq_sum, bin_name, _eta), prog=prog)
            passed += 1
            ratchet.record(str(rel), True)

    total = passed + failed + skipped
    elapsed = time.perf_counter() - t_all
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total")
    if fail_categories:
        for cat, names in fail_categories.items():
            print(f"  {cat}: {len(names)}  ({', '.join(names[:6])}{'…' if len(names) > 6 else ''})")
    _print_run_subfamily_summary(run_sub)
    print(f"Elapsed: {fmt_dur(elapsed)}"
          f"  (transpile {fmt_dur(t_transpile_total)}, build {fmt_dur(t_build_total)},"
          f" run {fmt_dur(t_run_total)})")
    print(f"[end] {time.strftime('%Y-%m-%d %H:%M:%S')}")
    ratchet.summary()
    _print_readability_summary(readability_counts)
    _print_equiv_summary(equiv_counts)
    _print_fallback_summary(fallback_counts)
    _summarize_raw()
    return _apply_deny(deny, equiv_counts, run_sub, failed,
                       fallback_counts=fallback_counts)


# ── 并行模式 ─────────────────────────────────────────────────────────

def _run_parallel(filter_str: list[str] | None, jobs: int,
                  deny: list[str], use_failed: bool = False,
                  failed_path: Path | None = None,
                  skip_failed: bool = False,
                  jdk_major: 'int | None' = None) -> int:
    failed_path = failed_path or _failed_file_path(None, jdk_major)
    prev_failed = _load_failed(failed_path)
    files = _discover(filter_str)
    if use_failed:
        files = _apply_failed_filter(files, prev_failed)
        if not files:
            print(f"失败清单为空或与 filter 无交集（{failed_path}）。")
            return 0
        print(f"[failed] 回归模式：清单 {len(prev_failed)} 个，本次运行 {len(files)} 个")
    elif skip_failed:
        files = [f for f in files if str(f.relative_to(ROOT)) not in prev_failed]
        print(f"[skip-failed] 跳过清单内 {len(prev_failed)} 个已知失败，本次运行 {len(files)} 个")
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    print(f"[start] {time.strftime('%Y-%m-%d %H:%M:%S')} | parallel (jobs={jobs})"
          + (f" | filter: {' '.join(filter_str)}" if filter_str else ""))
    ratchet = _FailedRatchet(failed_path, prev_failed, jdk_major)
    _print_env_header()
    name_w = _name_width(files)
    aux_by_file: dict = {}

    # 过滤无 expected 文件的测试
    pending: list[Path] = []
    skipped = 0
    for f in files:
        if _read_expected(_class_name(f)) is None:
            _pline(name_w, "SKIP", f.relative_to(E2E),
                   f"(no expected/{_class_name(f)}.txt)", prog=f"[{files.index(f)+1:>3}/{len(files)}]")
            skipped += 1
        else:
            pending.append(f)

    if not pending:
        print(f"\n{'='*50}")
        print(f"Results: 0 passed, 0 failed, {skipped} skipped / {skipped} total")
        return 0

    # 1. 并行转译（每测试独立 scratch，无共享状态，可安全并发）
    print(f"\n[batch] 并行转译 {len(pending)} 个测试（max_workers={jobs}）…")
    t_all = time.perf_counter()

    def _transpile_one(java_file: Path) -> tuple[Path, bool, str, dict[str, int],
                                                 dict[str, int], dict[str, int]]:
        ws = _test_workspace(_to_bin_name(_class_name(java_file)))
        ok, log = _transpile(java_file, out_dir=ws)
        _parse_raw(log)
        return (java_file, ok, log, _parse_readability(log), _parse_equiv(log),
                _parse_fallback(log))

    transpile_ok: list[Path] = []
    transpile_fail: list[Path] = []
    readability_counts: dict[str, dict[str, int]] = {}
    equiv_counts: dict[str, dict[str, int]] = {}
    fallback_counts: dict[str, dict[str, int]] = {}
    with ThreadPoolExecutor(max_workers=jobs) as executor:
        futures = {executor.submit(_transpile_one, f): f for f in pending}
        for fut in as_completed(futures):
            java_file, ok, log, rc, ec, fc = fut.result()
            rel = java_file.relative_to(ROOT)
            if ok:
                print(f"  [transpile] {rel} OK", flush=True)
                transpile_ok.append(java_file)
                if rc:
                    readability_counts[_class_name(java_file)] = rc
                if ec:
                    equiv_counts[_class_name(java_file)] = ec
                if fc:
                    fallback_counts[_class_name(java_file)] = fc
                aux_by_file[java_file] = (_transpile_aux(log),
                                          sum(ec.values()) if ec else 0,
                                          _parse_raw(log))
            else:
                print(f"  [transpile] {rel} FAIL", flush=True)
                print(log[-300:])
                transpile_fail.append(java_file)
                ratchet.record(str(rel), False)

    if not transpile_ok:
        print("所有转译均失败，退出。")
        return 1

    t_transpile = time.perf_counter() - t_all
    print(f"[time] 转译阶段 {fmt_dur(t_transpile)}（OK {len(transpile_ok)} / FAIL {len(transpile_fail)}）")

    # 2. 逐测试 cargo build --bin <name>
    #    共享 CARGO_TARGET_DIR 下 cargo 以文件锁串行化构建——并发调用只会互相
    #    等待，因此这里顺序构建；syn/quote/宏依赖缓存命中后每个测试只编译自己的
    #    窄语料 java_runtime + user bin。
    print(f"\n[batch] 顺序构建 {len(transpile_ok)} 个测试的 binary（共享 target 缓存）…")
    t_build_start = time.perf_counter()
    build_ok: list[Path] = []
    build_fail: list[Path] = []
    build_durations: dict[str, float] = {}
    for java_file in transpile_ok:
        class_name = _class_name(java_file)
        bin_name = _to_bin_name(class_name)
        ws = _test_workspace(bin_name)
        print(f"  [build] {bin_name}…", end=" ", flush=True)
        t0 = time.perf_counter()
        r = _run(["cargo", "build", *_cargo_profile_args(), "--bin", bin_name], cwd=ws, env=_cargo_env())
        dur = time.perf_counter() - t0
        build_durations[bin_name] = dur
        if r.returncode == 0:
            print(f"OK ({_fmt_build_dur(dur)})", flush=True)
            build_ok.append(java_file)
        else:
            err = next((ln for ln in r.stderr.splitlines() if ln.startswith("error")),
                       "unknown error")
            print(f"FAIL ({fmt_dur(dur)})  {err[:100]}", flush=True)
            build_fail.append(java_file)

    t_build = time.perf_counter() - t_build_start
    slowest = max(build_durations.items(), key=lambda kv: kv[1], default=("", 0.0))
    print(f"[time] 构建阶段 {fmt_dur(t_build)}（OK {len(build_ok)} / FAIL {len(build_fail)}"
          f"，最慢 {slowest[0]} {fmt_dur(slowest[1])}）")

    # 3. 并行运行所有 binary（直接执行 target/debug/<bin>，不经 cargo）
    print(f"\n[batch] 并行运行 {len(build_ok)} 个 binary (max_workers={jobs})…\n")
    t_run_start = time.perf_counter()

    passed = failed = 0
    run_sub: dict[str, tuple[str, str]] = {}

    for java_file in build_fail:
        _a = aux_by_file.get(java_file) or ("", 0, 0)
        _pline(name_w, "FAIL", java_file.relative_to(E2E), "— compile error",
               aux=_aux_full(_a[0], _a[2], _a[1]))
        ratchet.record(str(java_file.relative_to(ROOT)), False)
        failed += 1

    def _run_one(java_file: Path) -> tuple[Path, bool, str, list[str]]:
        class_name = _class_name(java_file)
        ok, actual = _run_binary(class_name)
        if not ok:
            # run 族失败：直跑二进制抓 stderr 自动分类子族（stub-hit 等）
            fam, detail = _classify_run_failure(class_name)
            run_sub[class_name] = (fam, detail)
            return java_file, False, "binary error", []
        expected = _read_expected(class_name)
        diff = _diff(expected, actual, class_name)
        return java_file, len(diff) == 0, "", diff

    with ThreadPoolExecutor(max_workers=jobs) as executor:
        futures = {executor.submit(_run_one, f): f for f in build_ok}
        for fut in as_completed(futures):
            java_file, ok, err_msg, diff = fut.result()
            rel = java_file.relative_to(ROOT)
            _a = aux_by_file.get(java_file) or ("", 0, 0)
            _aux = _aux_full(_a[0], _a[2], _a[1],
                             _to_bin_name(_class_name(java_file)))
            if not ok and err_msg:
                _pline(name_w, "FAIL", java_file.relative_to(E2E), f"— {err_msg}", aux=_aux)
                ratchet.record(str(java_file.relative_to(ROOT)), False)
                failed += 1
            elif diff:
                _n_diff = sum(1 for l in diff if l[:1] in "+-" and not l.startswith(("+++", "---")))
                _pline(name_w, "FAIL", java_file.relative_to(E2E),
                       f"— output mismatch · diff {_n_diff} 行", aux=_aux)
                print("".join(diff[:40]))
                if len(diff) > 40:
                    print(f"  … ({len(diff) - 40} more lines)")
                ratchet.record(str(java_file.relative_to(ROOT)), False)
                failed += 1
            else:
                _pline(name_w, "PASS", java_file.relative_to(E2E), aux=_aux)
                ratchet.record(str(java_file.relative_to(ROOT)), True)
                passed += 1

    for java_file in transpile_fail:
        _pline(name_w, "FAIL", java_file.relative_to(E2E), "— transpile error")
        ratchet.record(str(java_file.relative_to(ROOT)), False)
        failed += 1

    total = passed + failed + skipped
    elapsed = time.perf_counter() - t_all
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total")
    _print_run_subfamily_summary(run_sub)
    print(f"Elapsed: {fmt_dur(elapsed)}"
          f"  (transpile {fmt_dur(t_transpile)}, build {fmt_dur(t_build)},"
          f" run {fmt_dur(time.perf_counter() - t_run_start)})")
    print(f"[end] {time.strftime('%Y-%m-%d %H:%M:%S')}")
    ratchet.summary()
    _print_readability_summary(readability_counts)
    _print_equiv_summary(equiv_counts)
    _print_fallback_summary(fallback_counts)
    _summarize_raw()
    return _apply_deny(deny, equiv_counts, run_sub, failed,
                       fallback_counts=fallback_counts)


# ── 期望输出更新（并行） ─────────────────────────────────────────────

def _update_expected_parallel(files: list[Path], jobs: int) -> int:
    """并行生成期望输出（javac/java 独立进程，classes 目录按测试独立）。

    结尾汇总四类：updated / javac-fail / java-fail / timeout——哪些测试
    没有正常生成、Java 侧报什么错，一目了然。"""
    print(f"Updating expected output for {len(files)} file(s) (max_workers={jobs})…")
    t_all = time.perf_counter()

    def _one(f: Path) -> tuple[Path, str, str]:
        return f, *_update_expected(f)

    results: dict[str, list[tuple[str, str]]] = {}
    with ThreadPoolExecutor(max_workers=jobs) as executor:
        futures = {executor.submit(_one, f): f for f in files}
        done = 0
        for fut in as_completed(futures):
            java_file, status, detail = fut.result()
            done += 1
            rel = str(java_file.relative_to(ROOT))
            results.setdefault(status, []).append((rel, detail))
            mark = {"updated": "OK", "javac-fail": "JAVAC-FAIL",
                    "java-fail": "JAVA-FAIL", "timeout": "TIMEOUT"}.get(status, status)
            print(f"[{done:>3}/{len(files)}] [{mark:>10}] {rel}  {detail}", flush=True)

    print(f"\n{'='*50}")
    updated = results.get("updated", [])
    print(f"updated: {len(updated)} / {len(files)}")
    for status in ("javac-fail", "java-fail", "timeout"):
        items = results.get(status, [])
        if items:
            print(f"\n{status}: {len(items)}")
            for rel, detail in items:
                print(f"  {rel}\n    {detail}")
    print(f"Elapsed: {fmt_dur(time.perf_counter() - t_all)}")
    return 0 if len(updated) == len(files) else 1


# ── 入口 ─────────────────────────────────────────────────────────────

def run_tests(filter_str: list[str] | None, no_run: bool, update_expected: bool, jobs: int,
              deny: list[str], use_failed: bool = False,
              failed_file: str | None = None, skip_failed: bool = False) -> int:
    files = _discover(filter_str)
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    if update_expected:
        return _update_expected_parallel(files, jobs)

    # 清单按 JDK 版本分文件 + 头部身份校验：不同版本语料/手写覆盖面不同，
    # 失败集不可比——错版本下 --failed 会重跑假失败（已发生过：JDK25 的
    # HelloWorld 混入 JDK21 清单）。旧无版本清单一次性迁移（视为 21）。
    jdk_major = _current_jdk_major()
    failed_path = _failed_file_path(failed_file, jdk_major)
    listed_jdk = _failed_jdk_of(failed_path)
    if (listed_jdk is not None and jdk_major is not None and listed_jdk != jdk_major):
        sys.exit(f"[failed-file] 清单 {failed_path} 是 JDK{listed_jdk} 的失败集，"
                 f"当前运行 JDK{jdk_major}——不同版本失败集不可比。"
                 f"请用对应 --jdk 运行，或 --failed-file 指定独立清单。")

    if jobs > 1:
        return _run_parallel(filter_str, jobs, deny, use_failed=use_failed,
                             failed_path=failed_path,
                             skip_failed=skip_failed, jdk_major=jdk_major)
    return _run_sequential(filter_str, no_run, deny, use_failed=use_failed,
                           failed_path=failed_path,
                           skip_failed=skip_failed, jdk_major=jdk_major)


def main():
    global OUT, SHARED_TARGET
    ap = argparse.ArgumentParser(description="java_rta 端到端测试框架")
    ap.add_argument("--filter",          metavar="STR", nargs="+", help="只测试路径中包含任意指定字符串的文件（可传多个）")
    ap.add_argument("--no-run",          action="store_true", help="只生成 Rust，不执行对比（仅顺序模式）")
    ap.add_argument("--update-expected", action="store_true", help="重新生成 expected/*.txt（用 java 运行）")
    ap.add_argument("--out-dir",         metavar="DIR", default=None,
                    help="scratch 根目录（默认 build/；每测试在其下建独立子目录）")
    ap.add_argument("--jobs", "-j",      type=int, default=1, metavar="N",
                    help="并行测试数（默认 1 = 顺序模式；0 = CPU 核数）")
    ap.add_argument("--jdk",             type=int, default=None, metavar="N",
                    help="指定 JDK 主版本（javac/java/翻译语料同源；默认沿用 JAVA_HOME 或自动发现）")
    ap.add_argument("--release",         action="store_true", help="release 档位构建运行（LTO 慢编译/快运行；默认 dev）")
    ap.add_argument("--failed",          action="store_true", help="只运行失败清单（默认 build/failed_tests.txt）里的测试；跑到且 PASS 自动出列")
    ap.add_argument("--skip-failed",     action="store_true", help="跳过失败清单内的已知失败（干净面快速迭代；被跳过的不进出清单）")
    ap.add_argument("--failed-file",     metavar="PATH", default=None, help="失败清单路径（默认 build/failed_tests.txt）")
    ap.add_argument("--deny",            action="append", default=[], metavar="SPEC",
                    help="拒绝升级（默认全放行，可叠加）：equiv = 任一等价发射点非零即整体失败；"
                         "equiv::<id> = 细粒度（id 见 [equiv-audit] 行）；"
                         "fallback / fallback::<id> = 静默兜底点非零（id 见 [fallback-audit] 行，"
                         "收窄后非零极可能是真 bug）；"
                         "stub-hit = run 失败的 stub 子族（二进制 stderr 含 `stub: `）")
    args = ap.parse_args()

    _validate_deny(args.deny)
    if args.failed and args.skip_failed:
        sys.exit("--failed 与 --skip-failed 互斥：前者只跑清单、后者跳过清单。")

    if args.out_dir is not None:
        OUT = Path(args.out_dir)
        if not OUT.is_absolute():
            OUT = ROOT / OUT
        SHARED_TARGET = OUT / "target"

    global PROFILE_DIR
    if args.release:
        PROFILE_DIR = "release"

    if args.jdk is not None:
        apply_jdk_choice(args.jdk)

    jobs = args.jobs
    if jobs == 0:
        jobs = os.cpu_count() or 4

    try:
        sys.exit(run_tests(args.filter, args.no_run, args.update_expected, jobs, args.deny,
                           use_failed=args.failed, failed_file=args.failed_file,
                           skip_failed=args.skip_failed))
    except KeyboardInterrupt:
        # Ctrl-C：失败清单是写穿棘轮（每测即落盘），已完成的结果已保住；
        # 子进程由 SIGINT 直接终止，这里只做安静退出，不打 traceback。
        print(f"\n[interrupt] 用户中断——已完成测试的结果已写入失败清单与日志。")
        sys.exit(130)


if __name__ == "__main__":
    main()
