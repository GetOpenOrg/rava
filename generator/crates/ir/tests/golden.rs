//! IR 渲染 golden 对照：`scripts/golden/dump_ir.py` 录下 Python 各渲染入口的
//! （IR 输入, 输出）对，这里转换成 Rust IR 后渲染并逐字节比对。
//!
//! golden 不在仓库里（`build/golden/ir/`，gitignore），不存在时跳过并打印生成命令。
//! 已知不可对齐的条目登记在 `GOLDEN_DIFF.md`。

mod golden_support;

use golden_support::run;

const TESTS: [&str; 2] = ["TestHashMapOps", "TestStreamBasic"];

fn check(test: &str) {
    let Some(rep) = run(test) else {
        println!(
            "[golden] 跳过 {test}：{} 下无 golden。生成：python3 scripts/golden/dump_ir.py <{test}.java> --clean",
            golden_support::golden_dir().display()
        );
        return;
    };
    println!("[golden] {test}: {}/{} 全等", rep.equal, rep.total);
    for (k, (n, eq)) in &rep.by_kind {
        println!("  {k}: {eq}/{n}");
    }
    println!("  结构化：{:?}", rep.fallbacks.structured);
    println!("  退回 Raw：{:?}", rep.fallbacks.raw);
    for (cat, text) in &rep.fallbacks.raw_samples {
        println!("  Raw ← [{cat}] {text:?}");
    }
    for m in rep.mismatches.iter().take(20) {
        println!("  ≠ [{}] {}\n    py: {}\n    rs: {}", m.kind, m.input, m.python, m.rust);
    }
    for e in rep.errors.iter().take(20) {
        println!("  ! {e}");
    }
    for a in rep.atomic_disagree.iter().take(20) {
        println!("  原子性不一致：{a}");
    }
    assert!(rep.errors.is_empty(), "{test}: {} 条转换错误", rep.errors.len());
    assert!(rep.mismatches.is_empty(), "{test}: {} 条输出不一致", rep.mismatches.len());
    assert!(rep.atomic_disagree.is_empty(), "{test}: {} 处原子性判定不一致", rep.atomic_disagree.len());
}

#[test]
fn golden_hash_map_ops() {
    check(TESTS[0]);
}

#[test]
fn golden_stream_basic() {
    check(TESTS[1]);
}
