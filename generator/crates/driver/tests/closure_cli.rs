//! `rava closure` 端到端：记录型 `--flows` 查询（`@grow:` / `@trace:` / `@edge:`）分析前登记、传播中记录，
//! 结果随查询输出并实时写 stderr；不带记录型查询时不产生任何记录。找不到 JDK 21 时跳过。

use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

/// 同一进程内的 rava 子进程串行（同 `build_cli.rs`）
static RAVA: Mutex<()> = Mutex::new(());

/// 当前工作区的包目录：取运行期 `CARGO_MANIFEST_DIR`（cargo 按本次调用设置）。编译期 `env!` 在全机共享的
/// CARGO_TARGET_DIR 下可能指向另一工作区——cargo 对路径包按工作区相对路径算 metadata，源码相同时不重编，
/// 测试二进制里嵌的就是首次编译它的（可能已删除的）worktree
fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

/// 一次 `rava closure`：返回 (stdout, stderr)；缺 JDK → None
fn closure(java: &str, extra: &[&str]) -> Option<(String, String)> {
    closure_at(&manifest_dir().join("tests/fixtures").join(java), extra)
}

/// 同上，Java 源文件取给定路径
fn closure_at(java: &std::path::Path, extra: &[&str]) -> Option<(String, String)> {
    let dir = manifest_dir();
    let _guard = RAVA.lock().unwrap_or_else(|e| e.into_inner());
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg("closure")
        .arg(java)
        .args(["--jdk", "21", "--runtime"])
        .arg(dir.join("../../../runtime/java_runtime"))
        .args(extra)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        if stderr.contains("未找到 JDK") {
            eprintln!("[closure_cli] 跳过：无 JDK 21");
            return None;
        }
        panic!("rava closure 失败：\n{stderr}");
    }
    Some((String::from_utf8_lossy(&o.stdout).to_string(), stderr))
}

/// 查询结果段：从标题行起到空行止
fn section<'s>(out: &'s str, query: &str) -> Vec<&'s str> {
    let head = format!("  {query}：");
    let lines: Vec<&str> = out.lines().skip_while(|l| !l.starts_with(&head)).take_while(|l| !l.is_empty()).collect();
    assert!(!lines.is_empty(), "缺查询段 {query}：{out}");
    lines
}

#[test]
fn recording_flow_queries() {
    const PASS: &str = "FlowProbe.pass:(Ljava/lang/Object;)Ljava/lang/Object;";
    let grow = "@grow:R FlowProbe.pass";
    let trace = "@trace:FlowProbe$Box";
    let edge = "@edge:R FlowProbe.pass";
    let Some((out, err)) = closure("FlowProbe.java", &["--flows", grow, "--flows", trace, "--flows", edge, "--flows", "@array"]) else {
        return;
    };
    // @grow：返回节点经形参 P0 获得 StringBuilder
    let g = section(&out, grow);
    assert!(g[0].ends_with("：1 条"), "{g:?}");
    assert!(g[1].contains(&format!("R {PASS} ← P0 {PASS} +{{java/lang/StringBuilder}}")), "{g:?}");
    // @edge：P0 → R 流边
    let e = section(&out, edge);
    assert!(e.iter().any(|l| l.contains(&format!("P0 {PASS} → R {PASS} [java/lang/Object]"))), "{e:?}");
    // @trace：Box 在 main 的分配站点直接注入，再流入构造器接收者；序号按到达先后递增
    let t = section(&out, trace);
    assert!(t[1].contains("← 直接 FlowProbe.main:([Ljava/lang/String;)V@"), "{t:?}");
    assert!(t.iter().any(|l| l.contains("P0 FlowProbe$Box.<init>:()V ← ")), "{t:?}");
    let seqs: Vec<u64> = t[1..].iter().map(|l| l.trim_start()[1..].split(' ').next().unwrap().parse().unwrap()).collect();
    assert!(seqs.windows(2).all(|w| w[0] < w[1]), "{seqs:?}");
    // 实时记录与查询结果逐条一致
    for l in g[1..].iter().chain(&t[1..]).chain(&e[1..]) {
        assert!(err.contains(l.trim_start()), "stderr 缺实时记录 {l}");
    }
    // 非记录型查询照常在分析后求值
    assert!(out.contains("  array = {"), "{out}");
}

#[test]
fn no_recording_without_queries() {
    let Some((out, err)) = closure("FlowProbe.java", &["--flows", "FlowProbe.pass"]) else {
        return;
    };
    assert!(!err.contains("[flows "), "未登记记录型查询却有记录：{err}");
    assert!(out.contains("FlowProbe.pass:(Ljava/lang/Object;)Ljava/lang/Object;"), "{out}");
}

/// 一次 `rava closure -o`：闭包 JSON 的类 / 方法 / 反射成员集合
fn closure_sets(java: &std::path::Path, seed: u64) -> Option<[std::collections::BTreeSet<String>; 3]> {
    // 测试并行运行：输出文件按进程内序号区分
    static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let out = std::env::temp_dir().join(format!("rava_closure_seed_{}_{n}_{seed}.json", std::process::id()));
    let seed = seed.to_string();
    closure_at(java, &["--hash-seed", &seed, "-o", out.to_str().unwrap()])?;
    let text = std::fs::read_to_string(&out).expect("读闭包 JSON");
    let _ = std::fs::remove_file(&out);
    let d: serde_json::Value = serde_json::from_str(&text).expect("闭包 JSON");
    let strs = |v: &serde_json::Value, key: Option<&str>| -> std::collections::BTreeSet<String> {
        v.as_array()
            .expect("数组")
            .iter()
            .map(|x| match key {
                Some(k) => x[k].as_str().expect("字符串字段").to_string(),
                None => x.as_str().map(str::to_string).unwrap_or_else(|| x["name"].as_str().expect("类名").to_string()),
            })
            .collect()
    };
    Some([strs(&d["classes"], None), strs(&d["methods"], Some("id")), strs(&d["reflect"]["members"], Some("member"))])
}

/// 闭包与哈希顺序无关：同一程序换哈希种子，类 / 方法 / 反射成员集合完全一致
/// （形参常量格的中间态不得留下不可撤回的反射登记）。用例取 e2e 的序列化例：序列化辅助方法按形参取名、
/// 按形参取类，形参字符串常量曾随调用点接入先后在种子 0 / 1 间多出或缺少 `writeObject` 回调；
/// 包装方法按调用点配对点名后，各种子下序列化回调都进反射成员（运行期按名查到的回调不得是存根）
#[test]
fn closure_independent_of_hash_seed() {
    // （用例, 是否序列化 ArrayList）
    const CASES: [(&str, bool); 7] = [
        ("23_algorithms/StockTrans.java", true),
        ("23_algorithms/DeepCopy.java", false),
        ("35_io/TestSerialDefaultSuid.java", true),
        ("35_io/TestSerialProxyForm.java", true),
        ("35_io/TestSerialUserGenericCallbacks.java", false),
        ("35_io/TestSerialLookupPairing.java", true),
        // V9：反射调用实参池去冗余（RN 涵盖判定）与透传摘要的过时边曾使 JNDI 闭包随种子差 20 个方法
        ("73_jndi_script/TestJndiNoProvider.java", false),
    ];
    const CALLBACK: &str = "java/util/ArrayList.writeObject:(Ljava/io/ObjectOutputStream;)V";
    for (case, list) in CASES {
        let Some(base) = seeds_agree(case) else { return };
        assert!(!list || base[2].contains(CALLBACK), "{case} 种子 0 的反射成员缺 {CALLBACK}");
    }
}

/// e2e 用例 case 在种子 0 / 1 / 2 下的闭包集合逐项相同；返回种子 0 的集合（缺 JDK → None）
fn seeds_agree(case: &str) -> Option<[std::collections::BTreeSet<String>; 3]> {
    let java = manifest_dir().join("../../../tests/e2e").join(case);
    let base = closure_sets(&java, 0)?;
    for seed in [1, 2] {
        let other = closure_sets(&java, seed).expect("同一 JDK");
        for (i, what) in ["类", "方法", "反射成员"].iter().enumerate() {
            let only_base: Vec<_> = base[i].difference(&other[i]).take(10).collect();
            let only_other: Vec<_> = other[i].difference(&base[i]).take(10).collect();
            assert!(only_base.is_empty() && only_other.is_empty(), "{case} 种子 0 与 {seed} 的{what}集合不同：{only_base:?} / {only_other:?}");
        }
    }
    Some(base)
}

/// 大用例的顺序无关性（单次闭包约 5 分钟 / 6 GB，缺省不跑）：
/// `CARGO_BUILD_JOBS=2 python3 <heavy_lock.py> cargo test --release -p driver --test closure_cli -- --ignored`；
/// 换运行期探针（访问器宏形态）见 `scripts/diag/probe_order.sh`
#[test]
#[ignore]
fn closure_independent_of_hash_seed_large() {
    seeds_agree("72_http/TestHttpLoopbackSync.java");
}

/// 反射数组分配（`Arrays.copyOf(T[], int, Class)` → `Array.newArray`）按调用点逐类型建分配点：`ArrayList.elementData` 的元素
/// 只来自实际写入，`ModuleDescriptor$Version.compareTokens` 的 `toString` / `compareTo` 不派发到全体活类型，`KeyFactory.nextSpi`
/// 的 `Provider$Service.newInstance` 不派发到其余 provider 的服务实现。放行判定在不动点上做，三个种子结果一致（首版即时饱和时种子 2 多 180 类）
#[test]
fn reflect_new_array_element_precision() {
    let Some([classes, ..]) = seeds_agree("62_reflection/TestModuleLayerDefine.java") else { return };
    for c in ["com/sun/org/apache/xml/internal/security/Init", "org/jcp/xml/dsig/internal/dom/ApacheCanonicalizer", "java/util/concurrent/ArrayBlockingQueue"] {
        assert!(!classes.contains(c), "反射数组元素退回 open(Object)：闭包含 {c}");
    }
}

/// 形参字符串常量进形参常量格：URL 构造器把协议名常量传给 URL$DefaultFactory.createURLStreamHandler，
/// 其字符串 switch（String.hashCode / equals 折叠）只取 file 臂。形参字符串一律置 Top 时 switch 不折叠，
/// 经 jrt 处理器、类路径 JarLoader、服务加载与反射池把 HelloWorld 闭包撑到约 2856 类（正常约 500 类）
#[test]
fn param_string_constants_fold_switch() {
    let java = manifest_dir().join("../../../tests/e2e/01_basics/HelloWorld.java");
    let Some([classes, ..]) = closure_sets(&java, 0) else { return };
    assert!(!classes.contains("sun/net/www/protocol/jrt/Handler"), "URL 协议名 switch 未按形参常量折叠");
    assert!(classes.len() < 1000, "HelloWorld 闭包 {} 类", classes.len());
}

/// HelloWorld 级程序：栈耗尽 VM 规则（stack-check）把 StackOverflowError 带入闭包（a3-T1b）
#[test]
fn stack_overflow_error_in_minimal_closure() {
    let Some((out, _)) = closure("MinimalMain.java", &["--why", "java/lang/StackOverflowError"]) else {
        return;
    };
    assert!(out.contains("java/lang/StackOverflowError（"), "{out}");
    assert!(out.contains("[vm-rule] 根 stack-check"), "{out}");
}
