//! 抽象值：引用 = 抽象对象位集；`int` 类 = 常量格；`long` / `double` 为单个宽值。

/// 抽象对象编号
pub type NodeId = u32;
/// 区域开始前已存在的一切对象（含静态字段可达的、native / `<clinit>` 期间分配的）
pub const OLD: NodeId = 0;
/// 区域内分配、类型未知的对象（捕获的异常）
pub const FRESH_UNKNOWN: NodeId = 1;

/// 抽象对象位集
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Refs(Vec<u64>);

impl Refs {
    /// 规范化的位字（去掉末尾零字）
    pub fn words(&self) -> &[u64] {
        let mut n = self.0.len();
        while n > 0 && self.0[n - 1] == 0 {
            n -= 1;
        }
        &self.0[..n]
    }

    pub fn single(n: NodeId) -> Refs {
        let mut r = Refs::default();
        r.insert(n);
        r
    }

    pub fn insert(&mut self, n: NodeId) -> bool {
        let (w, b) = ((n / 64) as usize, n % 64);
        if self.0.len() <= w {
            self.0.resize(w + 1, 0);
        }
        let old = self.0[w];
        self.0[w] |= 1 << b;
        old != self.0[w]
    }

    pub fn contains(&self, n: NodeId) -> bool {
        let (w, b) = ((n / 64) as usize, n % 64);
        self.0.get(w).is_some_and(|x| x & (1 << b) != 0)
    }

    /// 并入 `o`，返回是否变化
    pub fn union_with(&mut self, o: &Refs) -> bool {
        if self.0.len() < o.0.len() {
            self.0.resize(o.0.len(), 0);
        }
        let mut changed = false;
        for (a, b) in self.0.iter_mut().zip(&o.0) {
            let n = *a | *b;
            changed |= n != *a;
            *a = n;
        }
        changed
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|w| *w == 0)
    }

    pub fn iter(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.0.iter().enumerate().flat_map(|(i, w)| {
            (0..64u32).filter(move |b| w & (1u64 << b) != 0).map(move |b| i as u32 * 64 + b)
        })
    }

    /// 含区域内分配的对象（`OLD` 以外的任一节点）
    pub fn has_fresh(&self) -> bool {
        self.iter().any(|n| n != OLD)
    }
}

/// `int` 类值的常量格
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Prim {
    Bot,
    Const(i32),
    Top,
}

impl Prim {
    pub fn join(self, o: Prim) -> Prim {
        match (self, o) {
            (Prim::Bot, x) | (x, Prim::Bot) => x,
            (Prim::Const(a), Prim::Const(b)) if a == b => Prim::Const(a),
            _ => Prim::Top,
        }
    }
}

/// 栈 / 局部变量槽的抽象值
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Val {
    Ref(Refs),
    Int(Prim),
    /// long / double（占一个栈项，计算类型 2）
    Wide,
    /// 未初始化或合流冲突（校验器保证不被读取）
    Void,
    /// 区域入口的局部变量：按读取指令解释（引用读为 `OLD`、基本类型读为 ⊤）
    Live,
}

impl Val {
    pub fn int_top() -> Val {
        Val::Int(Prim::Top)
    }

    pub fn old() -> Val {
        Val::Ref(Refs::single(OLD))
    }

    pub fn null() -> Val {
        Val::Ref(Refs::default())
    }

    /// 计算类型 2（long / double）
    pub fn is_wide(&self) -> bool {
        matches!(self, Val::Wide)
    }

    /// 引用视图（`Live` 读为 `OLD`，其余非引用为空）
    pub fn refs(&self) -> Refs {
        match self {
            Val::Ref(r) => r.clone(),
            Val::Live => Refs::single(OLD),
            _ => Refs::default(),
        }
    }

    pub fn prim(&self) -> Prim {
        match self {
            Val::Int(p) => *p,
            _ => Prim::Top,
        }
    }

    /// 合流，返回是否变化
    pub fn join_in(&mut self, o: &Val) -> bool {
        let n = match (&*self, o) {
            (Val::Ref(a), Val::Ref(b)) => {
                let mut a = a.clone();
                if !a.union_with(b) {
                    return false;
                }
                Val::Ref(a)
            }
            (Val::Int(a), Val::Int(b)) => Val::Int(a.join(*b)),
            (Val::Wide, Val::Wide) => Val::Wide,
            (Val::Live, Val::Live) => Val::Live,
            (a, b) if a == b => return false,
            _ => Val::Void,
        };
        let changed = n != *self;
        *self = n;
        changed
    }
}

/// 一条指令入口的抽象帧
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Frame {
    pub locals: Vec<Val>,
    pub stack: Vec<Val>,
}

impl Frame {
    /// 合流，返回是否变化；栈深不一致返回 None（字节码不合法）
    pub fn join_in(&mut self, o: &Frame) -> Option<bool> {
        if self.stack.len() != o.stack.len() {
            return None;
        }
        let mut changed = false;
        if self.locals.len() < o.locals.len() {
            self.locals.resize(o.locals.len(), Val::Void);
            changed = true;
        }
        for (k, a) in self.locals.iter_mut().enumerate() {
            changed |= a.join_in(o.locals.get(k).unwrap_or(&Val::Void));
        }
        for (a, b) in self.stack.iter_mut().zip(&o.stack) {
            changed |= a.join_in(b);
        }
        Some(changed)
    }
}
