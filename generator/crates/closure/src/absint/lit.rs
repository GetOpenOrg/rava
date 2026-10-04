//! 字符串字面量序号：合流后的引用值按字面量区分来源（[`super::Src::Str`]），
//! 下游（按名查找、形参字符串常量集）据此取回合流前的各个字面量。
//! 分析器单线程，序号表按线程保存；序号只在进程内使用，不进入输出。

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
struct Table {
    ids: HashMap<Rc<str>, u32>,
    strs: Vec<Rc<str>>,
}

thread_local! {
    static TABLE: RefCell<Table> = RefCell::new(Table::default());
}

/// 字面量 s 的序号
pub fn lit_id(s: &Rc<str>) -> u32 {
    TABLE.with(|t| {
        let mut t = t.borrow_mut();
        if let Some(&id) = t.ids.get(s) {
            return id;
        }
        let id = t.strs.len() as u32;
        t.strs.push(s.clone());
        t.ids.insert(s.clone(), id);
        id
    })
}

/// 序号 id 的字面量
pub fn lit_str(id: u32) -> Rc<str> {
    TABLE.with(|t| t.borrow().strs[id as usize].clone())
}

#[cfg(test)]
mod tests {
    use super::super::V;
    use std::rc::Rc;

    #[test]
    fn joined_literals_stay_recoverable() {
        let a = V::lit(Rc::from("invoke"));
        let b = V::lit(Rc::from("invokeExact"));
        let j = a.join(&b);
        assert!(matches!(j, V::Ref { .. }));
        let mut ls: Vec<String> = j.lits().iter().map(|s| s.to_string()).collect();
        ls.sort();
        assert_eq!(ls, ["invoke", "invokeExact"]);
        // 同一字面量两次合流不重复
        assert_eq!(a.join(&a).lits().len(), 1);
    }
}
