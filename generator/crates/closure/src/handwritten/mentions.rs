//! 手写层是否点名某个 Java 字段（边界类字段的建模口径，见 `engine/bytecode.rs` `boundary_field`）。
//!
//! 边界类（类镜像等）的 struct 承载 VM 注入状态，其字段可能由手写层直接读写；但其中的纯 Java 字段（缓存、
//! 惰性表等）在手写层里不出现时，运行期的值只来自字节码写入与构建期映像，与普通类字段同一口径。判定按名字、
//! 保守取：全部非生成手写文件的标识符与字符串字面量切成词，词本身及其任一 `_` 之后的后缀（覆盖
//! `__get_<名>` / `__set_<名>` / `set_<名>` 等访问器名）都算点名；字段名的 snake_case 形式同样计入。

use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

use super::{Handwritten, GENERATED_MARK};

impl Handwritten {
    /// 手写层（非生成文件）以任意形式点名了字段 `name`
    pub fn mentions_field(&self, name: &str) -> bool {
        let words = self.mention_words();
        words.contains(name) || words.contains(&super::to_snake(name))
    }

    fn mention_words(&self) -> Rc<HashSet<String>> {
        if let Some(w) = self.mentions.borrow().as_ref() {
            return w.clone();
        }
        let mut out = HashSet::new();
        walk(&self.src, &mut |content| {
            if content.contains(GENERATED_MARK) {
                return;
            }
            for t in content.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$')) {
                if t.is_empty() {
                    continue;
                }
                out.insert(t.to_string());
                for (i, _) in t.match_indices('_') {
                    let rest = &t[i + 1..];
                    if !rest.is_empty() {
                        out.insert(rest.to_string());
                    }
                }
            }
        });
        let w = Rc::new(out);
        *self.mentions.borrow_mut() = Some(w.clone());
        w
    }
}

fn walk(dir: &Path, f: &mut dyn FnMut(&str)) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, f);
        } else if p.extension().is_some_and(|e| e == "rs") {
            if let Ok(c) = std::fs::read_to_string(&p) {
                f(&c);
            }
        }
    }
}
