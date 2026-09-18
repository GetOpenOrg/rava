# block.rs 重构方案

> 日期：2026-09-18  
> 背景：`runtime/java_rta_macros/src/block.rs` 当前 2016 行，单文件承担全部 proc-macro 逻辑，所有改动集中于此。本文档记录分析结论与推荐重构路径。

---

## 一、现状分析

### 1.1 段落结构

| 行范围 | 职责 |
|--------|------|
| 1–115 | 输入解析：`FnItem`、`ClassInput`、`parse_impl_fns` |
| 117–217 | 类级属性：`ClassMeta`、`lit_str`、`lit_bool` |
| 219–319 | 分类工具：`is_basic`、`attr_str`、`MethodKind`、`classify_method` |
| 321–567 | 字段重写器：`Rewriter`（`self.field → self.__get_field()`） |
| 519–567 | Self/this 重写器：`SelfToConcrete`、`SelfToThis`（dead code） |
| 569–673 | vtable 专用重写器：`rewrite_base_calls_for_wrapper` 等 |
| 675–722 | 方法体分类：`VTableBodyKind`、`classify_vtable_body` |
| 724–847 | JVM 泛型签名解析：`parse_generic_method_sig` 等 |
| 856–2016 | **`expand_inner`**：~1100 行单函数，11 个生成阶段全部在内 |

### 1.2 核心问题

#### 问题 1：`expand_inner` 是 1100 行的 God Function

11 个生成阶段（vtable trait → inner struct → vtable impls → wrapper → From impls → base 函数）全部共享局部变量，无法跨文件拆分，无法独立测试，改一处容易踩另一处。

#### 问题 2：`classify_vtable_body` 用字符串匹配 token（脆弱）

```rust
// 当前实现
let s = quote!(#block).to_string();
if s.contains("Clone :: clone (this)")
    || s.contains("Clone :: clone(this)")
    || s.contains("Clone::clone (this)")
    || s.contains("Clone::clone(this)") { ... }
```

`proc_macro2` 生成的 token 空格不稳定，已经导致过 bug（`Ok(Clone::clone(this))` 漏匹配）。字符串匹配无法区分"作为返回值的 `Ok(Clone::clone(this))`"和"作为函数参数的 `Clone::clone(this)`"，导致 E0308 编译错误。

#### 问题 3：base 函数生成缺少非 vtable 方法检查

`NeedsWrapper` 体若调用了 native non-vtable 方法（如 `fillInStackTrace_i`、`isArray`），在 base 函数的 `&(impl VTable + ?Sized)` 上下文中无法编译（E0599）。根本原因：生成 base 函数时无法访问 vtable 方法名集合，无法判断 `this.xyz()` 是否安全。

#### 问题 4：dead code 噪音

`SelfToConcrete`、`SelfToThis`、`rewrite_block_for_base` 均未被调用，产生 `#[warn(dead_code)]`。

#### 问题 5：错误处理不统一

`expand_inner` 签名是 `-> TokenStream2`，内部通过 `return e.to_compile_error()` 提前退出，无法使用 `?` 传播错误。

---

## 二、设计原则

### 原则 1：以 Java 语义特性为边界切分 `gen/`，而非以 Rust 产物类型

**错误方向（按 Rust 产物）：**
```
gen/vtable_trait.rs
gen/inner_struct.rs
gen/vtable_impls.rs
gen/base_fns.rs
```
问题：`NeedsWrapper` 判断逻辑横跨 vtable_impls、base_fns、wrapper 三个文件；改一个 Java 特性要动多个文件。

**正确方向（按 Java 语义）：**
```
gen/virtual_dispatch.rs   ← Java 虚方法分派（vtable trait + vtable impl + base fn 在同一模块）
gen/struct_layout.rs      ← Java 字段布局（__inner struct + field accessors）
gen/wrapper.rs            ← Java 类型包装（wrapper struct + method delegation）
gen/type_conversions.rs   ← Java 类型转换（From<Object>, From<Child> for Parent）
```

改 `NeedsWrapper` 逻辑：只动 `virtual_dispatch.rs`；  
改字段访问器逻辑：只动 `struct_layout.rs`。

### 原则 2：`GenContext` 是唯一的跨阶段数据传递机制

当前 `expand_inner` 里的所有局部变量打包进 `GenContext`，每个生成函数只接受 `&GenContext`，不接受散装参数。

### 原则 3：`classify_vtable_body` 改为 AST 遍历

用 `syn::visit::Visit` trait 做精确的 AST 节点检查，彻底消除字符串匹配。

---

## 三、目标文件结构

```
runtime/java_rta_macros/src/
├── lib.rs                   # 只保留 #[proc_macro] 入口，~5 行
├── block.rs                 # expand() + expand_inner() 调用序列，~80 行
├── parse.rs                 # FnItem, ClassInput, ClassMeta, parse_impl_fns
├── util.rs                  # is_basic, attr_str, strip_meta_attrs, MethodKind, classify_method
├── rewrite.rs               # Rewriter, rewrite_block, rewrite_base_calls_for_wrapper,
│                            #   rewrite_virtual_calls_for_wrapper, replace_clone_this_in_ok
├── classify.rs              # VTableBodyKind, classify_vtable_body（AST 实现），is_vtable_safe_body
├── generic_sig.rs           # parse_generic_method_sig, parse_jvm_type_at, rebuild_sig_with_generics
└── gen/
    ├── mod.rs               # pub use，GenContext 定义
    ├── context.rs           # GenContext struct
    ├── virtual_dispatch.rs  # vtable trait + vtable impl for __inner + base 自由函数
    ├── struct_layout.rs     # __inner struct + field accessors + impl ObjectVTable for __inner
    ├── wrapper.rs           # wrapper struct + wrapper impl（method delegation）
    └── type_conversions.rs  # From<Object>, From<Child> for Parent, BINARY_NAME 常量
```

---

## 四、关键数据结构

### GenContext

> **2026-09-18 审计补充**：初版 GenContext 有 5 处缺失和 2 处错误，详见下方说明。

```rust
// gen/context.rs
pub(crate) struct GenContext<'a> {
    // ── 标识符 ────────────────────────────────────────────────
    pub struct_ident: &'a Ident,
    pub self_name: String,              // struct_ident.to_string()，base fn 命名用
    pub inner_ident: Ident,             // format_ident!("{}__inner", struct_ident)
    pub vtable_trait_ident: Ident,      // format_ident!("{}__VTable", struct_ident)

    // ── 泛型（同时保留完整对象和 split 结果）──────────────────
    pub gen: syn::Generics,             // 完整对象，§11 需要 gen.clone() 追加 __BT 参数
    pub impl_g: TokenStream2,           // 便捷引用，各处 quote! 使用
    pub ty_g: TokenStream2,
    pub where_c: TokenStream2,
    pub class_type_params: Vec<String>, // 类型参数名，§1 generic_sig 重建
    pub class_ty_idents: Vec<Ident>,    // 类型参数 Ident，§1 turbofish ::<K, V, Self>

    // ── 元数据 ────────────────────────────────────────────────
    pub meta: ClassMeta,                // binary_name, superclass, superclass_fields 等
    pub superclass_vtable_args: TokenStream2, // 父类泛型参数（如 <Object>），§1 §4 §10

    // ── 字段 ──────────────────────────────────────────────────
    pub fields: Vec<(Ident, Type)>,     // own fields，§2 inner struct / §4 accessor / §7 wrapper
    pub basic_names: HashSet<String>,   // 值类型字段名（供 Rewriter 使用）
    pub ref_names: HashSet<String>,     // 引用类型字段名（供 Rewriter 使用）
    pub phantom_fields: Vec<TokenStream2>, // 未被字段使用的泛型参数，§2 PhantomData 字段

    // ── 方法分类 ──────────────────────────────────────────────
    pub vtable_defines: Vec<&'a FnItem>,
    pub vtable_overrides: HashMap<String, Vec<&'a FnItem>>,
    pub non_virtual: Vec<&'a FnItem>,   // Constructor + NonVirtual 合并（代码中不分离）

    // ── 方法名集合（两个不同语义，不可合并为一个）────────────
    pub own_method_names: HashSet<String>,    // 全部方法名，rewrite_virtual_calls_for_wrapper 用
    pub vtable_define_names: HashSet<String>, // 仅 VirtualDefine 方法名，§11 base fn 安全检查用
}
```

**初版 GenContext 审计发现的问题（2026-09-18）：**

| 问题 | 说明 |
|------|------|
| `self_name` 缺失 | `self_name = struct_ident.to_string()`，用于 §1 vtable default impl 和 §11 base fn 命名 |
| `gen: syn::Generics` 缺失 | 初版只有 `impl_g/ty_g/where_c: TokenStream2`，§11 需要 `gen.clone()` 追加 `__BT` 泛型参数，TokenStream2 无法做到 |
| `fields` 缺失 | own fields 列表，§2 inner struct / §4 Self__VTable accessor / §7 wrapper accessor 均需要 |
| `phantom_fields` 缺失 | §2 inner struct 的 PhantomData 字段生成 |
| `class_ty_idents` 缺失 | §1 vtable default impl turbofish `::<ClassTypeParams, Self>` 避免 E0282 |
| `constructors` 错误 | 代码中 Constructor 和 NonVirtual 合并到 `non_virtual`，不单独存在 |
| `vtable_method_names` 概念模糊 | 初版只有一个集合，但代码实际有两个语义不同的集合：`own_method_names`（全部方法，rewrite 用）和 `vtable_define_names`（仅 VirtualDefine，base fn 安全检查用），不能合并 |

### expand_inner 简化后

```rust
fn expand_inner(input: ClassInput) -> syn::Result<TokenStream2> {
    let meta = ClassMeta::from_attrs(&input.attrs)?;

    if meta.is_interface {
        let name = &input.struct_ident;
        return Ok(quote! { pub type #name = Object; });
    }

    let ctx = GenContext::build(&input, &meta)?;

    let layout      = gen::struct_layout::generate(&ctx);
    let dispatch    = gen::virtual_dispatch::generate(&ctx);
    let wrapper     = gen::wrapper::generate(&ctx);
    let conversions = gen::type_conversions::generate(&ctx);

    Ok(quote! { #layout #dispatch #wrapper #conversions })
}

pub fn expand(input: TokenStream2) -> TokenStream2 {
    syn::parse2::<ClassInput>(input)
        .and_then(expand_inner)
        .unwrap_or_else(|e| e.to_compile_error())
}
```

---

## 五、关键算法改进

### 5.1 `classify_vtable_body` 改为 AST 遍历

```rust
// classify.rs
pub enum VTableBodyKind {
    Safe,         // body 中无 Clone::clone(this)，无 Self::，this.xyz() 均为 vtable 方法
    NeedsWrapper, // body 中有 Ok(Clone::clone(this)) 作为返回，其余 this.xyz() 均为 vtable 方法
    Skip,         // body 含 Self::，或 Clone::clone(this) 作为非返回参数，无法在 vtable 上下文运行
}

struct BodyScanner {
    has_self_path: bool,              // Self:: 出现
    clone_this_positions: Vec<CloneThisPos>,
}
enum CloneThisPos { InOkReturn, AsArg }

impl<'ast> Visit<'ast> for BodyScanner {
    fn visit_expr_call(&mut self, e: &ExprCall) {
        let is_ok = path_ident(&e.func).map_or(false, |id| id == "Ok");
        let is_clone = is_clone_call(&e.func);
        let has_bare_this = e.args.iter().any(|a| is_bare_ident(a, "this"));

        if is_clone && has_bare_this {
            // 检查上下文：是否在 Ok(...) 的直接子节点
            self.clone_this_positions.push(CloneThisPos::AsArg);
        }
        if is_ok && e.args.len() == 1 {
            if let Expr::Call(inner) = &e.args[0] {
                if is_clone_call(&inner.func) && inner.args.iter().any(|a| is_bare_ident(a, "this")) {
                    // 覆盖为 InOkReturn
                    // ...
                }
            }
        }
        visit::visit_expr_call(self, e);
    }
    fn visit_expr_path(&mut self, e: &ExprPath) {
        if e.path.segments.first().map(|s| s.ident == "Self").unwrap_or(false) {
            self.has_self_path = true;
        }
    }
}

pub fn classify_vtable_body(block: &Block, vtable_names: &HashSet<String>) -> VTableBodyKind {
    let mut scanner = BodyScanner::default();
    scanner.visit_block(block);

    if scanner.has_self_path { return VTableBodyKind::Skip; }
    if scanner.clone_this_positions.iter().any(|p| matches!(p, CloneThisPos::AsArg)) {
        return VTableBodyKind::Skip;  // Clone::clone(this) 作为普通参数，无法安全替换
    }

    // 检查 this.xyz() 调用是否全部为 vtable 方法
    let has_non_vtable_call = has_non_vtable_this_call(block, vtable_names);
    if has_non_vtable_call { return VTableBodyKind::Skip; }

    if scanner.clone_this_positions.iter().any(|p| matches!(p, CloneThisPos::InOkReturn)) {
        return VTableBodyKind::NeedsWrapper;
    }
    VTableBodyKind::Safe
}
```

**重要变化**：`classify_vtable_body` 现在接受 `vtable_names` 参数，可以检测 `this.fillInStackTrace_i()` 等非 vtable 方法调用，直接返回 `Skip`，避免 E0599。

### 5.2 base 函数安全检查

**两种 base 函数上下文的 `__xxx` 规则不同，必须分开处理：**

| 上下文 | `this` 类型约束 | `__get_xxx` 安全条件 |
|--------|----------------|---------------------|
| VirtualDefine base | `&__BT where __BT: CurrentClass__VTable` | 始终安全（当前类 vtable 包含所有字段 accessor） |
| VirtualOverride base | `&__BT where __BT: AncestorClass__VTable` | 仅当 `xxx` 是超类字段时安全；自有字段不在祖先 vtable 里 |

```rust
// gen/virtual_dispatch.rs

// VirtualDefine base fn 安全检查
// this: &__BT where __BT: CurrentClass__VTable
// __ 前缀 accessor 始终安全（均在 CurrentClass__VTable 中）
fn is_define_body_safe_for_base(block: &Block, vtable_define_names: &HashSet<String>) -> bool {
    let bs = quote!(#block).to_string();
    // 检查 this.method()（非 __ 前缀），是否全在 vtable_define_names 中
    for part in bs.split("this .").chain(bs.split("this.")).skip(1) {
        let trimmed = part.trim_start();
        let mname: String = trimmed.chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if !mname.is_empty() && !mname.starts_with("__")
            && trimmed[mname.len()..].trim_start().starts_with('(')
            && !vtable_define_names.contains(&mname)
        {
            return false;
        }
    }
    true
}

// VirtualOverride base fn 安全检查
// this: &__BT where __BT: AncestorClass__VTable
// __ 前缀 accessor 只有超类字段才安全；自有字段 accessor 不在祖先 vtable 里
fn is_override_body_safe_for_base(
    block: &Block,
    superclass_field_names: &HashSet<String>,  // meta.superclass_fields 的字段名集合
) -> bool {
    let bs = quote!(#block).to_string();
    for part in bs.split("this .").chain(bs.split("this.")).skip(1) {
        let trimmed = part.trim_start();
        let mname: String = trimmed.chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if mname.is_empty() { continue; }
        if !trimmed[mname.len()..].trim_start().starts_with('(') { continue; }
        if mname.starts_with("__") {
            // accessor：提取字段名，只有超类字段才在祖先 vtable 中
            let field = mname.strip_prefix("__get_")
                .or_else(|| mname.strip_prefix("__set_"))
                .or_else(|| mname.strip_prefix("__borrow_mut_"))
                .unwrap_or("");
            if !field.is_empty() && !superclass_field_names.contains(field) {
                return false;  // 自有字段 accessor，祖先 vtable 不含 → unsafe
            }
        } else {
            return false;  // 普通方法调用，祖先 vtable 不一定含 → unsafe
        }
    }
    true
}
```

> **注**：上述仍为字符串扫描实现（与现有 block.rs 保持一致）。改为 AST Visit 实现时，`ExprMethodCall` 处理逻辑相同，但 `VisitMut` 需要上下文感知地判断 `__` accessor 中的字段名是否在超类字段集合中。
```

---

## 六、执行计划

### 阶段 0（当前阻塞，P0）：修复 classify 逻辑

不拆文件，直接在现有 `block.rs` 内修复：

- [ ] `classify_vtable_body` 增加 `vtable_names` 参数，改为 AST 扫描
- [ ] `ClassContext` 构建时将 vtable_method_names 传入 base 函数生成逻辑
- [ ] 清理 dead code：`SelfToConcrete`、`SelfToThis`、`rewrite_block_for_base`

**验收**：HelloWorld + TestInterfaces e2e 通过，无 E0277/E0308/E0599。

### 阶段 1（独立层拆出，P1）：零依赖模块先行

每个模块独立 PR，不触碰 `expand_inner`：

- [ ] 新建 `generic_sig.rs`，搬移 `parse_generic_method_sig` 等（724–847 行）
- [ ] 新建 `classify.rs`，搬移 `VTableBodyKind`、`classify_vtable_body`（已是 AST 版本）
- [ ] 新建 `rewrite.rs`，搬移所有 `rewrite_*` 函数（321–673 行）
- [ ] `block.rs` 缩减到解析 + 属性 + 方法分类 + `expand_inner`

**验收**：`cargo check` 通过，行为不变。

### 阶段 2（GenContext + gen/ 拆分，P2）

- [ ] 新建 `gen/context.rs`，定义 `GenContext`（含 `vtable_method_names`）
- [ ] 新建 `gen/struct_layout.rs`，迁移 __inner struct 生成逻辑（Section 2–3）
- [ ] 新建 `gen/virtual_dispatch.rs`，迁移 vtable trait + vtable impls + base fn（Section 1, 4, 11）
- [ ] 新建 `gen/wrapper.rs`，迁移 wrapper struct + impl（Section 5–7）
- [ ] 新建 `gen/type_conversions.rs`，迁移 From impls + BINARY_NAME（Section 8–10）
- [ ] `expand_inner` 改为调用序列，缩减到 ~80 行

**验收**：`cargo check` 通过，`expand_inner` ≤ 100 行，单个 `gen/` 模块 ≤ 400 行。

### 阶段 3（错误处理统一，P3）

- [ ] `expand_inner` 改为 `syn::Result<TokenStream2>`，内部使用 `?`
- [ ] `GenContext::build` 改为 `syn::Result<GenContext>`

---

## 七、不改变的内容

以下内容**不在本次重构范围内**，不修改：

- 生成的 Rust 代码语义（vtable 两指针架构本身不变）
- `java_class!` 宏的调用接口
- `runtime/java_runtime/src/` 下的手写代码
- Python codegen 生成的 `.rs` 文件内容

重构是纯粹的代码组织优化，对 Java→Rust 翻译结果零影响。

---

## 八、度量指标

| 指标 | 当前 | 目标（阶段 2 后） |
|------|------|-----------------|
| `block.rs` 行数 | 2016 | ≤ 150 |
| `expand_inner` 行数 | ~1100 | ≤ 100 |
| 单个模块最大行数 | 2016 | ≤ 400 |
| string-based token match | 4 处变种 | 0 |
| dead code warnings | 3 个 | 0 |
| `classify_vtable_body` E0599 误判 | 有（fillInStackTrace_i 等） | 0 |
