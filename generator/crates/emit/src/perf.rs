//! 发射性能观测（计划 docs/plans/2026-09-30-emitter-performance.md P0）：分阶段墙钟、
//! 阶段末进程峰值 RSS、逐类 / 逐方法耗时 Top-N。只做记录，不影响生成输出。

use std::time::{Duration, Instant};

/// 一个已结束的阶段
#[derive(Debug, Clone)]
pub struct PhaseMark {
    pub name: &'static str,
    pub elapsed: Duration,
    /// 阶段结束时的进程峰值内存占用（MB，单调不减；口径见 `closure::engine::peak_mem_mb`）
    pub peak_mem_mb: u64,
}

/// 分阶段计时器 + 逐项耗时表
#[derive(Debug)]
pub struct Perf {
    since: Instant,
    pub phases: Vec<PhaseMark>,
    /// 逐类发射耗时（类 binary 名；含其方法体生成）
    pub classes: Vec<(String, Duration)>,
}

impl Default for Perf {
    fn default() -> Perf {
        Perf { since: Instant::now(), phases: Vec::new(), classes: Vec::new() }
    }
}

impl Perf {
    pub fn new() -> Perf {
        Perf::default()
    }

    /// 结束当前阶段（自上一次 `mark` / 创建起计）
    pub fn mark(&mut self, name: &'static str) {
        let now = Instant::now();
        self.phases.push(PhaseMark { name, elapsed: now - self.since, peak_mem_mb: closure::engine::peak_mem_mb() });
        self.since = now;
    }

    /// 记录一个在别处计时、已结束的子阶段（紧接当前阶段起点之后；起点随之后移）
    pub fn mark_sub(&mut self, name: &'static str, elapsed: Duration) {
        self.phases.push(PhaseMark { name, elapsed, peak_mem_mb: closure::engine::peak_mem_mb() });
        self.since += elapsed;
    }

    /// 重置阶段起点（丢弃未计入的时间段）
    pub fn restart(&mut self) {
        self.since = Instant::now();
    }

    /// 并入另一个计时器的阶段与逐类记录（阶段名加前缀不可行时原样追加）
    pub fn absorb(&mut self, other: Perf) {
        self.phases.extend(other.phases);
        self.classes.extend(other.classes);
        self.since = Instant::now();
    }

    pub fn total(&self) -> Duration {
        self.phases.iter().map(|p| p.elapsed).sum()
    }
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// 按耗时降序取前 n 项（同耗时按名，确定性）
pub fn top_n(items: &[(String, Duration)], n: usize) -> Vec<(String, Duration)> {
    let mut v: Vec<(String, Duration)> = items.to_vec();
    v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    v.truncate(n);
    v
}

/// `[perf]` 报告行：阶段表 + 逐类 / 逐方法 Top-N
pub fn report_lines(perf: &Perf, methods: &[(String, Duration)], n: usize) -> Vec<String> {
    let mut out = Vec::new();
    out.push(format!("[perf] 合计 {:.1} ms，峰值占用 {} MB（RSS {} MB）", ms(perf.total()), closure::engine::peak_mem_mb(), closure::engine::peak_rss_mb()));
    for p in &perf.phases {
        out.push(format!("[perf] 阶段 {:<16} {:>10.1} ms  峰值占用 {:>6} MB", p.name, ms(p.elapsed), p.peak_mem_mb));
    }
    let class_sum: Duration = perf.classes.iter().map(|c| c.1).sum();
    out.push(format!("[perf] 类 {} 个，合计 {:.1} ms；Top {n}：", perf.classes.len(), ms(class_sum)));
    for (c, d) in top_n(&perf.classes, n) {
        out.push(format!("[perf]   {:>9.2} ms  {c}", ms(d)));
    }
    let method_sum: Duration = methods.iter().map(|m| m.1).sum();
    out.push(format!("[perf] 方法体 {} 次，合计 {:.1} ms；Top {n}：", methods.len(), ms(method_sum)));
    for (m, d) in top_n(methods, n) {
        out.push(format!("[perf]   {:>9.2} ms  {m}", ms(d)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_n_is_sorted_and_deterministic() {
        let d = Duration::from_millis;
        let items = vec![("b".to_string(), d(2)), ("a".to_string(), d(2)), ("c".to_string(), d(5)), ("d".to_string(), d(1))];
        let t = top_n(&items, 3);
        assert_eq!(t.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), ["c", "a", "b"]);
    }

    #[test]
    fn marks_accumulate() {
        let mut p = Perf::new();
        p.mark("a");
        p.mark("b");
        assert_eq!(p.phases.len(), 2);
        assert!(report_lines(&p, &[], 5).iter().any(|l| l.contains("阶段 a")));
    }
}
