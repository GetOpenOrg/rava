//! 区域对象图的成环判定：只有成环（引用计数回收不了）的区域才值得整组释放。

use crate::analysis::{Analysis, NodeKind};

impl Analysis<'_, '_> {
    /// 结果中区域对象之间是否成环（引用计数无法回收、需整组释放）
    pub fn has_cycle(&self) -> bool {
        let exact =
            |n: usize| matches!(self.nodes[n], NodeKind::Alloc(_) | NodeKind::Lambda(_) | NodeKind::Made(_) | NodeKind::Native(_));
        let n = self.nodes.len();
        // 0 未访问 / 1 在栈 / 2 完成
        let mut color = vec![0u8; n];
        for s in 0..n {
            if !exact(s) || color[s] != 0 {
                continue;
            }
            let mut stack = vec![(s, self.succ(s).iter().collect::<Vec<_>>(), 0usize)];
            color[s] = 1;
            while let Some((v, succ, i)) = stack.last_mut() {
                if *i < succ.len() {
                    let u = succ[*i] as usize;
                    *i += 1;
                    if !exact(u) {
                        continue;
                    }
                    if color[u] == 1 {
                        return true;
                    }
                    if color[u] == 0 {
                        color[u] = 1;
                        let next = self.succ(u).iter().collect();
                        stack.push((u, next, 0));
                    }
                } else {
                    color[*v] = 2;
                    stack.pop();
                }
            }
        }
        false
    }
}
