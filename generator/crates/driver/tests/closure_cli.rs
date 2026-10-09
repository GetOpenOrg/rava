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

/// 一次 `rava closure -o`（附加参数 extra）的规范化闭包 JSON：去掉 `via` 与 `summary`，数组按规范文本排序
fn closure_doc(java: &std::path::Path, extra: &[&str]) -> Option<serde_json::Value> {
    static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let out = std::env::temp_dir().join(format!("rava_closure_order_{}_{n}.json", std::process::id()));
    let mut args = extra.to_vec();
    args.extend(["-o", out.to_str().unwrap()]);
    closure_at(java, &args)?;
    let text = std::fs::read_to_string(&out).expect("读闭包 JSON");
    let _ = std::fs::remove_file(&out);
    let mut d: serde_json::Value = serde_json::from_str(&text).expect("闭包 JSON");
    d.as_object_mut().expect("对象").remove("summary");
    Some(norm_doc(d))
}

fn norm_doc(v: serde_json::Value) -> serde_json::Value {
    use serde_json::Value;
    match v {
        Value::Object(m) => Value::Object(m.into_iter().filter(|(k, _)| k != "via").map(|(k, x)| (k, norm_doc(x))).collect()),
        Value::Array(xs) => {
            let mut xs: Vec<Value> = xs.into_iter().map(norm_doc).collect();
            xs.sort_by_cached_key(|x| x.to_string());
            Value::Array(xs)
        }
        x => x,
    }
}

/// 两份规范化闭包 JSON 逐键（reflect 等再下一层）对照，返回差异键与示例
fn doc_diffs(a: &serde_json::Value, b: &serde_json::Value) -> Vec<String> {
    let flat = |d: &serde_json::Value| -> std::collections::BTreeMap<String, Vec<String>> {
        let mut out = std::collections::BTreeMap::new();
        for (k, v) in d.as_object().expect("对象") {
            match v {
                serde_json::Value::Object(m) => {
                    for (k2, v2) in m {
                        out.insert(format!("{k}.{k2}"), elems(v2));
                    }
                }
                _ => {
                    out.insert(k.clone(), elems(v));
                }
            }
        }
        out
    };
    fn elems(v: &serde_json::Value) -> Vec<String> {
        match v {
            serde_json::Value::Array(xs) => xs.iter().map(|x| x.to_string()).collect(),
            x => vec![x.to_string()],
        }
    }
    let (fa, fb) = (flat(a), flat(b));
    let keys: std::collections::BTreeSet<&String> = fa.keys().chain(fb.keys()).collect();
    let mut out = vec![];
    for k in keys {
        let empty = vec![];
        let (xa, xb) = (fa.get(k).unwrap_or(&empty), fb.get(k).unwrap_or(&empty));
        if xa != xb {
            let only_a: Vec<_> = xa.iter().filter(|x| !xb.contains(x)).take(3).collect();
            let only_b: Vec<_> = xb.iter().filter(|x| !xa.contains(x)).take(3).collect();
            out.push(format!("{k}：仅基准 {only_a:?} / 仅本例 {only_b:?}"));
        }
    }
    out
}

/// D1：闭包与处理顺序无关——流传播批量（`--flow-batch`）× 哈希种子（`--hash-seed`）的各组合下闭包 JSON
/// （类、方法含 kind / 截断标记、反射成员与缺口、折叠等全部键，via 除外）逐项相同。
/// HelloWorld 跑全矩阵；DeepCopy（按名查方法 / 反射调用池 / 形参常量格窗口的回归例）跑对角组合
#[test]
fn closure_independent_of_order() {
    const BATCHES: [&str; 5] = ["1", "7", "64", "512", "4096"];
    const SEEDS: [&str; 4] = ["0", "1", "2", "12345"];
    let e2e = manifest_dir().join("../../../tests/e2e");
    let hello = e2e.join("01_basics/HelloWorld.java");
    let Some(base) = closure_doc(&hello, &[]) else { return };
    for b in BATCHES {
        for s in SEEDS {
            let cur = closure_doc(&hello, &["--flow-batch", b, "--hash-seed", s]).expect("同一 JDK");
            let d = doc_diffs(&base, &cur);
            assert!(d.is_empty(), "HelloWorld batch {b} seed {s} 与缺省不同：\n{}", d.join("\n"));
        }
    }
    let deep = e2e.join("23_algorithms/DeepCopy.java");
    let base = closure_doc(&deep, &[]).expect("同一 JDK");
    for (b, s) in [("1", "1"), ("7", "2"), ("512", "12345"), ("4096", "0")] {
        let cur = closure_doc(&deep, &["--flow-batch", b, "--hash-seed", s]).expect("同一 JDK");
        let d = doc_diffs(&base, &cur);
        assert!(d.is_empty(), "DeepCopy batch {b} seed {s} 与缺省不同：\n{}", d.join("\n"));
    }
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

/// 按名取类站点名字含推不出的支时，已知名字仍在不动点上放行：工厂查找（`FactoryFinder.find`）先读系统属性 / 配置文件
/// （任意串），再回落到调用方传入的缺省实现类名——缺省实现类必须入闭包，否则运行期 `Class.forName` 找不到
/// （FactoryConfigurationError: Provider … not found）。三个种子结果一致
#[test]
fn unsure_lookup_releases_known_names() {
    let Some([classes, ..]) = seeds_agree("71_xml/TestSaxLocatorAttributes.java") else { return };
    for c in ["com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl", "com/sun/org/apache/xerces/internal/jaxp/SAXParserImpl"] {
        assert!(classes.contains(c), "按名取类的缺省实现类未入闭包：{c}");
    }
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

/// 容器元素按对象（计划 c1d §30 B2）：两个 ConcurrentHashMap 各存一种元素，`m1.get` 再 cast 的结果只含 m1 的元素，
/// `Square.name` 不经 `Shape.name` 派发入链。依赖两项：Unsafe 访问序包装（`getReferenceAcquire`）继承调用方上下文
/// （`relay.rs`，否则 `tabAt` 读出全部 map 的节点），树箱在自身方法里分配的树节点沿用属主 map 的堆上下文
/// （`classes.rs::internal_alloc`，否则截断后各 map 共用树节点、`val` 汇合全部 map 的值）
#[test]
fn container_elements_per_object() {
    let java = manifest_dir().join("tests/fixtures/ElemTrack.java");
    let Some([_, methods, _]) = closure_sets(&java, 0) else { return };
    assert!(methods.contains("ElemTrack$Circle.name:()Ljava/lang/String;"), "缺 Circle.name");
    assert!(!methods.contains("ElemTrack$Square.name:()Ljava/lang/String;"), "m1.get 的结果混入 m2 的元素：Square.name 入链");
}

/// 按接收者对象的返回值（计划 c1d §30.9 的 P3）：`use(b1)` 中 `b.tag()` 按形参对象取 b1 的返回值，
/// 构造器必然写 `tag`（P2）不并入初值；按成员汇合时含 b2 的 null，`Rare.go` 入链
#[test]
fn returns_per_receiver_object() {
    let java = manifest_dir().join("tests/fixtures/ObjFacts.java");
    let Some([_, methods, _]) = closure_sets(&java, 0) else { return };
    assert!(methods.contains("ObjFacts$Box.tag:()Ljava/lang/String;"), "缺 Box.tag");
    assert!(!methods.contains("ObjFacts$Rare.go:()V"), "use(b1) 的 b.tag() 混入 b2 的 null：Rare.go 入链");
}

/// 按站点值集的接收者（计划 c1d §30.9 的 P4）：`Holder.run` 中 `box.tag()` 的接收者来自字段读，
/// 站点节点值集只有 b1，按对象取返回值，`Rare2.go` 不入链
#[test]
fn returns_per_site_receiver() {
    let java = manifest_dir().join("tests/fixtures/ObjFacts.java");
    let Some([_, methods, _]) = closure_sets(&java, 0) else { return };
    assert!(methods.contains("ObjFacts$Holder.run:()V"), "缺 Holder.run");
    assert!(!methods.contains("ObjFacts$Rare2.go:()V"), "box.tag() 混入 b2 的 null：Rare2.go 入链");
}

/// 一次 `rava closure -o` 的 summary.sysprops_unstable
fn sysprops_unstable(java: &str) -> Option<serde_json::Value> {
    Some(closure_json(java)?["summary"]["sysprops_unstable"].clone())
}

/// 一次 `rava closure -o` 的完整 JSON
fn closure_json(java: &str) -> Option<serde_json::Value> {
    let out = std::env::temp_dir().join(format!("rava_cjson_{}_{java}.json", std::process::id()));
    closure(java, &["-o", out.to_str().unwrap()])?;
    let text = std::fs::read_to_string(&out).expect("读闭包 JSON");
    let _ = std::fs::remove_file(&out);
    Some(serde_json::from_str(&text).expect("闭包 JSON"))
}

/// 启动快照读取（`snapshot = true`）不受属性表逃逸影响：本例把 System.props 存入静态字段（全部不折叠），
/// ThreadLocalRandom.<clinit> 经 VM.getSavedProperty 读 java.util.secureRandomSeed 仍折叠为 null →
/// parseBoolean 为 false，SecureRandom.getSeed 分支（@185）为死区
#[test]
fn snapshot_read_ignores_props_escape() {
    let Some(d) = closure_json("SyspropsLambdaLeak.java") else { return };
    assert_eq!(d["summary"]["sysprops_unstable"]["all"], serde_json::Value::Bool(true), "本例应全部不折叠");
    let folds = d["folds"].as_array().expect("folds");
    let f = folds
        .iter()
        .find(|f| f["method"] == "java/util/concurrent/ThreadLocalRandom.<clinit>:()V")
        .expect("ThreadLocalRandom.<clinit> 应有折叠点");
    let dead = f["dead_pcs"].as_array().unwrap().iter().any(|r| r[0].as_u64() <= Some(185) && r[1].as_u64() > Some(185));
    assert!(dead, "快照读取未折叠：{f}");
}

/// lambda 返回系统属性表：lambda 封闭于创建方法的一次调用、结果原路返回后只读 → 不逃逸；
/// 结果存入字段 → 逃逸，全部不折叠（`sysprops_lambda.rs`）
#[test]
fn sysprops_lambda_return_confined() {
    let Some(base) = sysprops_unstable("MinimalMain.java") else { return };
    let Some(read) = sysprops_unstable("SyspropsLambdaRead.java") else { return };
    let Some(leak) = sysprops_unstable("SyspropsLambdaLeak.java") else { return };
    assert_eq!(read["all"], base["all"], "只读使用不应改变不折叠判定：{read}");
    assert_eq!(leak["all"], serde_json::Value::Bool(true), "存入字段应全部不折叠：{leak}");
    if base["all"] == serde_json::Value::Bool(false) {
        let cause = leak["cause"].as_str().unwrap_or_default();
        assert!(cause.contains("SyspropsLambdaLeak"), "成因应指向本例：{cause}");
    }
}
