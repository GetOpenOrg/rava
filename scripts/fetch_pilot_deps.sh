#!/usr/bin/env bash
# lib pilot 语料取包 + 透视：mvn 解析依赖闭包 → jar 导出 → 逐 jar 跑 dep_scan。
#
# 依赖清单（版本钉死）：tests/lib_pilot/deps/pom.xml
# jar 导出位：tests/lib_pilot/deps/target/pilot-libs/（gitignore；scripts/lib_pilot_golden.sh 默认从此取）
#
# 用法：scripts/fetch_pilot_deps.sh            # 取包 + 逐 jar 透视
#       scripts/fetch_pilot_deps.sh --no-scan  # 只取包（golden 对账前置）
# 依赖树全貌：mvn -B -f tests/lib_pilot/deps/pom.xml dependency:tree
# 前置：mvn 在 PATH；JDK 经 rava jdk 选择（与 rava build / run_tests.py 同一实现）。
set -euo pipefail
REPO="$(cd "$(dirname "$0")/.." && pwd)"
DEPS="$REPO/tests/lib_pilot/deps"
SCAN=1; [[ "${1:-}" == "--no-scan" ]] && SCAN=0

. "$REPO/scripts/rava_env.sh" "$REPO"
JAVA_HOME="$("$RAVA" jdk --home-only)" || { echo "未找到可用 JDK" >&2; exit 2; }
export JAVA_HOME

# 清残留：copy-dependencies 不清目录，升版后旧 jar 会并存污染语料口径
rm -rf "$DEPS/target/pilot-libs"
mvn -B -q -f "$DEPS/pom.xml" package
echo "─── jar 已导出：$DEPS/target/pilot-libs/ ───"
ls -lh "$DEPS/target/pilot-libs/"

# mvn 解析坐标清单（g:a:type[:classifier]:version:scope:绝对路径）——锁条目名的真源；
# jar 内 pom.properties 只作回退（junit / hamcrest 等 5 个 jar 不带）
mvn -B -q -f "$DEPS/pom.xml" dependency:list -DoutputFile="$DEPS/target/deps-list.txt" \
    -DoutputAbsoluteArtifactFilename=true >/dev/null

# 依赖锁 deps.lock.toml（V12 §3.2：release + [[jar]] 类路径序，coordinate/path/sha256；
# 生成物不入库）。坐标优先取 mvn 解析（按文件名配对）；release 取参考 JDK tag 主版本；
# sha256 = jar 身份（坐标只作索引与诊断）；条目名 = 坐标 artifactId（无坐标时文件名 stem）
python3 - "$DEPS/target" "$REPO/tools/refjdk.toml" <<'PY'
import hashlib, re, sys, zipfile
from pathlib import Path

target, refjdk = Path(sys.argv[1]), Path(sys.argv[2])
tag = next((l.split('"')[1] for l in refjdk.read_text().splitlines() if l.startswith("tag =")), "")
major = re.match(r"jdk-(\d+)", tag)
assert major, f"参考 JDK tag 不可解析：{tag}"

# mvn 坐标清单 → 文件名 → 坐标（g:a:v）。行形如
# `g:a:jar[:classifier]:version:scope:/abs/path.jar -- module … [auto]`：先剥 ` -- ` 后缀；
# path = 末段（.jar 结尾判据），version = 倒数第三段（含 classifier 的 7 段与不含的 6 段同位置）
mvn_coords = {}
for ln in (target / "deps-list.txt").read_text().splitlines():
    parts = ln.split(" -- ")[0].strip().split(":")
    if len(parts) >= 6 and parts[-1].endswith(".jar"):
        g, a, ver, path = parts[0], parts[1], parts[-3], parts[-1]
        mvn_coords[Path(path).name] = f"{g}:{a}:{ver}"

lines = [f"release = {major.group(1)}"]
for jar in sorted((target / "pilot-libs").glob("*.jar")):
    coord = mvn_coords.get(jar.name, "")
    if not coord:
        with zipfile.ZipFile(jar) as z:
            props = [n for n in z.namelist() if re.fullmatch(r"META-INF/maven/[^/]+/[^/]+/pom.properties", n)]
            if props:
                kv = dict(l.split("=", 1) for l in z.read(props[0]).decode().splitlines() if "=" in l and not l.startswith("#"))
                g, a, v = kv.get("groupId", ""), kv.get("artifactId", ""), kv.get("version", "")
                if g and a and v:
                    coord = f"{g}:{a}:{v}"
    sha = hashlib.sha256(jar.read_bytes()).hexdigest()
    lines.append("[[jar]]")
    if coord:
        lines.append(f'coordinate = "{coord}"')
    lines.append(f'path = "pilot-libs/{jar.name}"')
    lines.append(f'sha256 = "{sha}"')
(target / "deps.lock.toml").write_text("\n".join(lines) + "\n")
print(f"deps.lock.toml：{lines.count('[[jar]]')} 条（坐标 {sum(1 for l in lines if l.startswith('coordinate'))}）")
PY

[[ "$SCAN" == 1 ]] || exit 0
for jar in "$DEPS"/target/pilot-libs/*.jar; do
    echo
    echo "════ $(basename "$jar") ════"
    python3 "$REPO/scripts/dep_scan.py" "$jar"
done
