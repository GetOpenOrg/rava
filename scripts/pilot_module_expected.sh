#!/usr/bin/env bash
# 生成 pilot jar 的模块名期望表（resolve 集成测试 tests/pilot_module_names.rs 的数据源）。
#
# 名字真源 = 参考 JDK 的 java.lang.module.ModuleFinder（与运行期 JPMS 同一实现）：
#   - 具名模块（含多版本 jar 的 META-INF/versions/N 描述符，按运行时版本取视图）
#   - 自动模块（Automatic-Module-Name 或 JPMS 文件名推导）
# 输出 TSV：jar 文件名 <TAB> 模块名 <TAB> kind(named|automatic)，按 jar 名排序。
# pom 变更取包后须重跑本脚本刷新期望表。
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LIBS_DIR="${1:-$REPO_ROOT/tests/lib_pilot/deps/target/pilot-libs}"
OUT="${2:-$REPO_ROOT/generator/crates/resolve/tests/pilot_module_names.tsv}"

JAVA_HOME="$(ls -d "$REPO_ROOT"/tools/refjdk/*/ 2>/dev/null | head -1 || true)"
if [[ -z "$JAVA_HOME" ]]; then
  JAVA_HOME="$(bash "$REPO_ROOT/scripts/fetch_reference_jdk.sh" 2>/dev/null | tail -1 || true)"
fi
[[ -x "$JAVA_HOME/bin/javac" ]] || { echo "找不到参考 JDK（先跑 scripts/fetch_reference_jdk.sh）" >&2; exit 1; }

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$(dirname "$OUT")"
cat > "$tmp/Describe.java" <<'EOF'
import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReference;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collections;
import java.util.stream.Stream;

public class Describe {
    public static void main(String[] args) throws Exception {
        var lines = new ArrayList<String>();
        // 逐 jar 建 finder：目录级 findAll 会因同名模块（多版本坐标并存）抛 FindException
        try (Stream<Path> jars = Files.list(Path.of(args[0]))) {
            for (Path jar : jars.filter(p -> p.toString().endsWith(".jar")).sorted().toList()) {
                for (ModuleReference mr : ModuleFinder.of(jar).findAll()) {
                    var d = mr.descriptor();
                    lines.add(jar.getFileName() + "\t" + d.name() + "\t" + (d.isAutomatic() ? "automatic" : "named"));
                }
            }
        }
        Collections.sort(lines);
        lines.forEach(System.out::println);
    }
}
EOF
"$JAVA_HOME/bin/javac" -d "$tmp" "$tmp/Describe.java" >&2
"$JAVA_HOME/bin/java" -cp "$tmp" Describe "$LIBS_DIR" > "$OUT"
echo "$(wc -l < "$OUT" | tr -d ' ') 行 @ $JAVA_HOME -> $OUT"
