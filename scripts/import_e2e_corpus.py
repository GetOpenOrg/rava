#!/usr/bin/env python3
"""jnc / ruva Java 测试语料导入工具（一次性；方案见 docs/plans/2026-09-25-import-jnc-ruva-e2e-tests.md）。

范围（2026-09-25 用户裁定）：只做文件导入 + 内容修正，不生成期望输出、不跑 run_tests.py。

用法：
    python3 scripts/import_e2e_corpus.py manifest          # 生成 build/import-manifest.tsv（不写任何测试文件）
    python3 scripts/import_e2e_corpus.py import B1 [B2 B3] # 执行拷贝 + 内容修正（package 剥离/改名/合并/剔除）
    python3 scripts/import_e2e_corpus.py verify            # 对已导入文件做隔离 javac 编译核验
"""

import re
import shutil
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).parent.parent
E2E = ROOT / "tests" / "e2e"
JNC = Path("/Users/yuwei/dev/workspace/jnc/tests/java-sources")
RUVA = Path("/Users/yuwei/dev/workspace/ruva/tests/e2e")
MANIFEST = ROOT / "build" / "import-manifest.tsv"

# ── 剔除清单（理由均为违反框架契约或纯重复，见方案 §4.1）──────────────────
SKIPS = {
    "jnc:HelloWorld.java": "与既有 01_basics/HelloWorld 同名，内容为一行打印，语义已覆盖",
    "jnc:ComprehensiveTestV1.java": "与 ComprehensiveTest.java 逐字节相同，纯重复",
    "jnc:bootstrap/Wrapper.java": "无 main 的 bootstrap 支持类，非独立测试",
    "jnc:bootstrap/SimpleNode.java": "无 main 的 bootstrap 支持类，非独立测试",
    "jnc:bootstrap/SimpleContainer.java": "无 main 的 bootstrap 支持类，非独立测试",
    "ruva:basic/HelloWorld.java": "与既有 01_basics/HelloWorld 同名，内容为一行打印，语义已覆盖",
    "ruva:io/ScannerBasic.java": "Scanner(System.in) 读 stdin，headless 必 EOF 失败",
    "ruva:io/ProcessBuilderDemo.java": "派生外部进程且输出含 $HOME，机器相关",
    "ruva:misc/UUIDDemo.java": "UUID.randomUUID 输出不可复现",
    "ruva:algorithms/KnightsTour.java": "Math.random 驱动输出",
    "ruva:algorithms/BalancedBrackets.java": "Math.random 驱动输出",
    "ruva:algorithms/VersCheck.java": "打印 java.version，JDK 版本相关",
    "ruva:algorithms/ScriptName.java": "读 sun.java.command，调 java 与调 cargo run 输出必然不同",
    "ruva:basic/MultiFileDemo/Greeter.java": "无 main 支持类，并入 MultiFileDemo.java 单文件化",
    "jnc:PackageTest.java": "package 声明使类落子包，run_tests 按裸类名加载必 CNFE——测试目标在本框架不可表达",
    "ruva:algorithms/Narcissist.java": "从 stdin 读自身源码做 quine 校验，headless 下 readLine 永久阻塞至超时",
}

# ── package 声明剥离（22 个，其中 KnightsTour 已剔除，实际 21）────────────
STRIP_PACKAGE = {
    "ruva:algorithms/HeapsAlgorithm.java",
    "ruva:algorithms/KnightsTour.java",
    "ruva:algorithms/KroneckerProduct.java",
    "ruva:algorithms/KroneckerProductFractals.java",
    "ruva:algorithms/OIDListSorting.java",
    "ruva:algorithms/RailwayCircuit.java",
    "ruva:algorithms/SumTo100.java",
    "ruva:algorithms/WheelController.java",
    "ruva:algorithms/Zebra.java",
    "ruva:collections/DoubleLinkedListTraversing.java",
    "ruva:features/Base64Example.java",
    "ruva:features/DiamondProblem.java",
    "ruva:features/ExampleConsumers.java",
    "ruva:features/InterfaceTest.java",
    "ruva:features/PipeStreamTest.java",
    "ruva:features/ShadowingTest.java",
    "ruva:features/SimpleReductions.java",
    "ruva:features/StreamTest.java",
    "ruva:features/UnnecessaryOne.java",
    "ruva:features/UrlEncoderExample.java",
    "ruva:string/ReportStringLengths.java",
    "ruva:string/WordWrap.java",
}

# ── 改名（类名跨库撞名 / 语义过弱；词边界全文件替换标识符）──────────────────
# key=(来源, 相对路径) → (新类名, 目标目录)
RENAMES = {
    "ruva:features/InterfaceTest.java": ("InterfaceDefaultChainTest", "26_interface_advanced"),
    "ruva:features/StreamTest.java": ("StreamOfNullableFilteringTest", "30_streams"),
    "ruva:features/InstanceofPattern/Main.java": ("InstanceofCastDemo", "21_casting"),
}

# ── jnc 目录映射：显式覆盖优先，其余按规则序匹配文件名 ─────────────────────
JNC_OVERRIDES = {
    "ControlFlowTest": "01_basics", "DoWhileAndLabelTest": "01_basics",
    "ForEachTest": "01_basics", "EdgeCaseTest": "01_basics",
    "SimpleClass": "01_basics", "ComprehensiveTest": "01_basics",
    "ComprehensiveTestV2": "01_basics", "IincRunner": "01_basics",
    "VarTest": "57_lambda_var",
    "RegexTest": "27_string_regex",
    "StaticFieldTest": "10_static", "StaticInitTest": "10_static",
    "DivByZeroTest": "40_numeric_edge",
    "IntegerToStringRadixTest": "08_numbers",
    "SetOperationsTest": "44_collection_api",
    "PackageTest": "52_lang_features", "RuntimeMethodsTest": "52_lang_features",
    "CheckcastTest": "21_casting", "FunctionalCompositionTest": "14_functional",
    "BSTRecursiveGeneric": "23_algorithms",
    "MultiClassOopTest": "02_oop",
    "java_base_e2e_reflect_invoke": "24_object_methods",
    "JavaBaseObjectsTest": "24_object_methods", "UserObjectHashTest": "24_object_methods",
    "JavaBaseOptionalTest": "43_optional_api", "JavaBaseOptionalPrimitiveTest": "43_optional_api",
    "JavaBaseStatisticsTest": "30_streams", "JavaBaseLongStatisticsTest": "30_streams",
    "JavaBaseLinkTest": "58_misc", "JavaBaseComboTest": "58_misc",
    "JavaBaseMultiTest": "58_misc", "JavaBaseAlphaTest": "58_misc",
}
JNC_RULES = [
    (r"Switch", "15_switch"),
    (r"Enum", "09_enum"),
    (r"Record|Sealed", "16_modern"),
    (r"Thread|Concurrent", "34_concurrency"),
    (r"LocalDate|LocalTime|DateTime", "37_datetime"),
    (r"Stream|Lambda|MethodRef|Anon", "07_lambdas"),
    (r"Exception|MultiCatch|Finally|TryWith|CrossMethod", "06_exceptions"),
    (r"GZIP|FileIO|Scanner", "35_io"),
    (r"Big", "36_bignum"),
    (r"HashMap|HashSet|TreeMap|TreeSet|Map|Linked", "33_maps"),
    (r"List|Queue|Stack|Collection|Iterator", "04_collections"),
    (r"GenericBasic", "03_generics"),
    (r"Generic", "12_generics_advanced"),
    (r"String", "05_strings"),
    (r"Cast|InstanceOf", "21_casting"),
    (r"Boxing", "22_autoboxing"),
    (r"Varargs", "20_varargs"),
    (r"Abstract", "19_abstract"),
    (r"Interface|Poly|Inherit|DefaultMethod", "26_interface_advanced"),
    (r"Inner", "11_inner_classes"),
    (r"Clone|Null|Objects|ObjectHash", "24_object_methods"),
    (r"Math|Number", "38_math"),
    (r"Recursion|Recursive|BST|Checksum", "23_algorithms"),
    (r"Array", "18_arrays_advanced"),
    (r"Pattern", "41_patterns_advanced"),
]

# ── ruva 目录映射：目录 → 类目，名字级覆盖修正错位 ─────────────────────────
RUVA_DIR_MAP = {
    "algorithms": "23_algorithms",
    "basic": "01_basics",
    "collections": "04_collections",
    "exceptions": "06_exceptions",
    "generics": "03_generics",
    "inheritance": "02_oop",
    "io": "35_io",
    "lambda": "07_lambdas",
    "oop": "02_oop",
    "streams": "30_streams",
    "string": "05_strings",
    "time": "37_datetime",
}
RUVA_NAME_OVERRIDES = {
    # collections 内错位
    "ExceptionHierarchy": "06_exceptions", "ExceptionPropagation": "06_exceptions",
    "AtomicBasic": "34_concurrency", "CompletableFutureDemo": "34_concurrency",
    "ConcurrentMapDemo": "34_concurrency",
    # features
    "Base64Example": "53_io_api", "UrlEncoderExample": "53_io_api",
    "ShadowingTest": "52_lang_features", "DiamondProblem": "26_interface_advanced",
    "PipeStreamTest": "35_io", "RecordsSerializationTest": "35_io",
    "SimpleReductions": "14_functional", "ExampleConsumers": "14_functional",
    "HelpfulNPEMessages": "16_modern",
    "RecordPatternTest": "16_modern", "RecordPatternsTest": "16_modern",
    "RecordStaticInnerMemberTest": "16_modern", "RecordWithSealed": "16_modern",
    "RecordWithSealedInterface": "16_modern",
    "SealedTypes": "16_modern", "SealedTypesExample": "16_modern",
    "TextBlockDemo": "16_modern", "TextBlocksExample": "16_modern",
    "PatternMatchingForInstanceOf": "41_patterns_advanced",
    "SwitchExpressionTests": "15_switch", "SwitchExpressions": "15_switch",
    "SwitchPatternSealed": "15_switch", "SwitchWithPatternMatchingThirdPreview": "15_switch",
    "VarDemo": "57_lambda_var", "RegexMatcherDemo": "27_string_regex",
    "SequencedCollectionExample": "55_sequenced",
    # features/InstanceofPattern/Main.java 与 misc/InstanceofPattern.java 由 RENAMES 处理
    # misc
    "EnumDemo": "09_enum", "AbstractEnumDemo": "45_enum_deep",
    "EnumAbstractMethod": "09_enum", "EnumSwitch": "09_enum",
    "SwitchExpression": "15_switch", "SwitchPattern": "15_switch",
    "InstanceofPattern": "41_patterns_advanced",
    "StringBuilderAdv": "17_string_advanced", "NestedClasses": "11_inner_classes",
    "InnerClassBasic": "11_inner_classes", "LocalClassHelper": "11_inner_classes",
    "StaticNestedClass": "11_inner_classes",
    "WeakRefDemo": "48_refs", "BigIntegerBasic": "36_bignum",
    "BigDecimalBasic": "36_bignum",
    "StrictfpDemo": "52_lang_features", "JavaDocMarkdownExample": "52_lang_features",
    "CompactNumberFormatExample": "56_random_format",
    "RandomDemo": "56_random_format", "SerializableDemo": "35_io",
    "ByteBufferDemo": "35_io", "Base64Demo": "53_io_api",
    "ThreadStub": "34_concurrency", "SynchronizedTest": "34_concurrency",
    "ArrayCovarianceDemo": "18_arrays_advanced",
    "IntOverflow": "40_numeric_edge",
    "IntegerMethodsDemo": "08_numbers", "NumberDemo": "08_numbers",
    "ObjectsDemo": "24_object_methods", "InterfaceDefaultMethod": "26_interface_advanced",
    "ClinitOrder": "10_static", "StringInternDemo": "05_strings",
    "RecordDemo": "16_modern", "SealedDemo": "16_modern",
    "VarargsDemo": "20_varargs",
    # stdlib 按名分投
    "BigDecimalDemo": "36_bignum", "BigIntegerDemo": "36_bignum",
    "RegexDemo": "27_string_regex", "AtomicDemo": "34_concurrency",
    "RandomLCG": "56_random_format", "JavaTimeDemo": "37_datetime",
}
RUVA_FALLBACK_DIRS = {"features", "misc"}  # 未命中覆盖时落 58_misc


def jnc_target(stem: str) -> str:
    if stem in JNC_OVERRIDES:
        return JNC_OVERRIDES[stem]
    for pat, d in JNC_RULES:
        if re.search(pat, stem):
            return d
    return "58_misc"


def ruva_target(rel: Path) -> str:
    top = rel.parts[0]
    stem = rel.stem
    if stem in RUVA_NAME_OVERRIDES:
        return RUVA_NAME_OVERRIDES[stem]
    if top in RUVA_DIR_MAP:
        return RUVA_DIR_MAP[top]
    if top in RUVA_FALLBACK_DIRS:
        return "58_misc"
    raise SystemExit(f"ruva 未知目录: {rel}")


class Op:
    def __init__(self, batch, src_key, src_path, dst_dir, dst_name, actions, note=""):
        self.batch = batch
        self.src_key = src_key          # jnc:xxx / ruva:xxx
        self.src_path = src_path
        self.dst_dir = dst_dir
        self.dst_name = dst_name
        self.actions = actions          # [copy/strip-package/rename/merge 之一或多]
        self.note = note

    def row(self):
        dst = f"tests/e2e/{self.dst_dir}/{self.dst_name}"
        return f"{self.batch}\t{self.src_key}\t{'+'.join(self.actions)}\t{dst}\t{self.note}"


def build_ops() -> list[Op]:
    ops = []
    # jnc：仅顶层 *.java（bootstrap/ 已剔除）
    for f in sorted(JNC.glob("*.java")):
        key = f"jnc:{f.name}"
        if key in SKIPS:
            continue
        ops.append(Op("B1", key, f, jnc_target(f.stem), f.name, ["copy"]))
    # ruva：全树 *.java
    for f in sorted(RUVA.rglob("*.java")):
        rel = f.relative_to(RUVA)
        key = f"ruva:{rel}"
        if key in SKIPS:
            continue
        if key == "ruva:basic/MultiFileDemo/MultiFileDemo.java":
            ops.append(Op("B2", key, f, "01_basics", "MultiFileDemo.java",
                          ["merge-greeter"], "Greeter.java 并入为非 public 第二类"))
            continue
        actions = ["copy"]
        if key in STRIP_PACKAGE:
            actions = ["strip-package"]
        if key in RENAMES:
            new_name, dst_dir = RENAMES[key]
            actions.append("rename")
            ops.append(Op("B2", key, f, dst_dir, f"{new_name}.java", actions,
                          f"类改名 {rel.stem}→{new_name}"))
            continue
        dst_dir = ruva_target(rel)
        ops.append(Op("B2" if "algorithms" not in rel.parts else "B3", key, f, dst_dir, f.name, actions))
    return ops


def strip_package(text: str) -> str:
    return re.sub(r"(?m)^\s*package\s+[\w.]+;\s*\n", "", text)


def rename_class(text: str, old: str, new: str) -> str:
    return re.sub(rf"\b{old}\b", new, text)


def apply_op(op: Op) -> Path:
    dst = E2E / op.dst_dir / op.dst_name
    if dst.exists():
        print(f"  [exists-skip] {dst.relative_to(ROOT)}")
        return dst
    dst.parent.mkdir(parents=True, exist_ok=True)
    text = op.src_path.read_text()
    if "strip-package" in op.actions:
        text = strip_package(text)
    if "rename" in op.actions:
        old_name = Path(op.src_key.split(":", 1)[1]).stem
        new = op.dst_name[:-len(".java")]
        text = rename_class(text, old_name, new)
    if "merge-greeter" in op.actions:
        greeter = op.src_path.parent / "Greeter.java"
        g = greeter.read_text().replace("public class Greeter", "class Greeter")
        text = text.rstrip() + "\n\n" + g.lstrip()
    dst.write_text(text)
    return dst


def cmd_manifest(ops: list[Op]) -> None:
    MANIFEST.parent.mkdir(exist_ok=True)
    MANIFEST.write_text("batch\tsource\taction\ttarget\tnote\n" + "\n".join(o.row() for o in ops) + "\n")
    from collections import Counter
    by_batch = Counter(o.batch for o in ops)
    by_dir = Counter(o.dst_dir for o in ops)
    print(f"manifest → {MANIFEST}")
    print("按批次:", dict(by_batch))
    print("按目标目录:")
    for d, c in sorted(by_dir.items()):
        print(f"  {d:22s} {c}")
    fallbacks = [o for o in ops if o.dst_dir == "58_misc"]
    print(f"58_misc 落位 {len(fallbacks)} 个（manifest 可查）")
    # 目标撞名自检
    seen = {}
    for o in ops:
        k = f"{o.dst_dir}/{o.dst_name}"
        if k in seen:
            raise SystemExit(f"目标撞名: {k} ← {seen[k]} 与 {o.src_key}")
        seen[k] = o.src_key
    existing = [o for o in ops if (E2E / o.dst_dir / o.dst_name).exists()]
    print(f"目标已存在(将 exists-skip): {len(existing)}")
    print("OK")


def cmd_import(ops: list[Op], batches: list[str]) -> None:
    n = 0
    for o in ops:
        if o.batch not in batches:
            continue
        apply_op(o)
        n += 1
    print(f"已处理 {n} 个文件（批次 {batches}）")


def cmd_verify(ops: list[Op], batches: list[str]) -> None:
    targets = [o for o in ops if o.batch in batches]
    files = [E2E / o.dst_dir / o.dst_name for o in targets]

    def check(p: Path) -> tuple[Path, str]:
        d = Path(tempfile.mkdtemp(prefix="import_verify_"))
        try:
            r = subprocess.run(["javac", "-nowarn", "-d", str(d / "out"), str(p)],
                               capture_output=True, text=True, timeout=120, cwd=d)
            if r.returncode == 0:
                return p, "ok"
            err = (r.stderr.strip().splitlines() or [""])[-1]
            return p, f"FAIL: {err[:120]}"
        except subprocess.TimeoutExpired:
            return p, "FAIL: timeout"
        finally:
            shutil.rmtree(d, ignore_errors=True)

    with ThreadPoolExecutor(max_workers=8) as ex:
        results = list(ex.map(check, files))
    bad = [(p, s) for p, s in results if s != "ok"]
    print(f"verify: {len(results) - len(bad)}/{len(results)} ok")
    for p, s in bad:
        print(f"  {s}  {p.relative_to(ROOT)}")
    if bad:
        sys.exit(1)


def main():
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    ops = build_ops()
    cmd = sys.argv[1]
    if cmd == "manifest":
        cmd_manifest(ops)
    elif cmd == "import":
        cmd_import(ops, sys.argv[2:])
    elif cmd == "verify":
        cmd_verify(ops, sys.argv[2:] or ["B1", "B2", "B3"])
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
