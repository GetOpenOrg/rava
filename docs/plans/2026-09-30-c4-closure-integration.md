# C4：Python 生成器接入 `rava closure`（2026-09-30）

> 上级计划：[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md) §六 C4（含 C2 中种子部分）。
> 终态：Python 生成器**只有一套**闭包来源——`rava closure` 输出的 `closure.json`；`callchain.py` 的发现逻辑整体删除。

## 一、下游从 Python BFS 取用的全部内容（盘点）

`_discover_jdk_classes_method_level` 返回 `(jdk_class_infos, visited_methods, field_discover_classes)`，另有全局与副作用：

| 产物 | 下游消费 | closure.json 来源 |
|---|---|---|
| `jdk_class_infos`（ClassInfo 列表，含库类） | `write_cargo_project` 注册表 / 每类一个 `.rs` / `_user_gen_jdk` / 注解枚举钩子与引导初始化门控；`--lib` 按 crate 拆分；`closure_report`；jdk-scan 报告 | `classes`（非 user 域）→ 按名装载 ClassInfo |
| `visited_methods`（`(类, 名, 描述符)`） | 每方法「翻译 / 存根」门控；`_type_only`；`<clinit>` 门控；接口 default / 超类虚方法槽位继承（`(实现类或祖先, m, d)` 键 + `_slot_demanded_on_chain` 按**调用点常量池类**键索引）；`import_gen` 祖先 `__base` 导入 | `methods`（已解析声明）∪ **新增 `refs`**（活代码调用点的符号引用键）∪ `clinit` |
| `field_discover_classes` | 只用于打印行与 jdk-scan 报告的「Field-only Stub」表 | `classes` 中 `level ∈ {type}` |
| `REFLECT_CONSTS` | `dispatch_gen` 反射分派臂（按名） | `reflect.members`（方法 / 构造器）+ 种子登记 |
| `REFLECT_FIELD_NAMES` | 静态字段反射臂 | 发射期启发（链上 JDK 方法的标识符形 `ldc` 字符串），改由接入层按 `methods` 计算 |
| `REFLECT_ALL_MEMBERS` | 镜像独有 / 物种类的全量字段反射面 | **新增** `seeds.image`（镜像独有 / VM 支持类种子） |
| `DATA_BUNDLE_SEEDS` | `register_data_bundles` | **新增** `seeds.data_bundles` |
| `ANNOTATION_ENUM_SEEDS` | 注解枚举类初始化钩子 | **新增** `seeds.annotation_enums` |
| `JCA_SEEDS` | `jca::register_services / register_providers` | **新增** `seeds.jca`（type / algorithm / impl / provider） |
| `MODULE_RESOURCES` | `jdk_resources/module/*` | 与闭包无关：接入层直接按 `seeds.toml [module_resources]` 从 jmod 提取 |
| `PRECHECK_CHAIN` | `precheck_from_tree`（native 缺失 / 边界存根命中） | `methods` 中 `handwritten:*` 与 `instantiated` |
| `_DATA_BUNDLE_LOADER` 注册 | `_is_boundary_class` 的纯数据束判定（发射期 `clinit_extract` / `class_writer` 审计 / `closure_report`） | 接入层在装载 ClassInfo 时注册同一装载器（与发现解耦） |
| `jdk_resolver._CORPUS_HOME` + 「语料选择」打印 | `jdk_feature.txt`、`run_tests.py` MIX 检查 | 接入层照旧按用户类 class 版本选 JDK，并以 `--java-home` 传给分析器 |
| `[fallback-audit]` / `[bfs-audit]` / `[upcall-audit]` 打印 | `run_tests.py`、`gen_trees.sh` 解析 | `summary`（missing / unresolved / reflect_gaps）按同格式打印；`cc-*` 桶随发现删除而消失 |

## 二、分析器侧补齐（Rust，接入前置）

| # | 缺口 | 实现 |
|---|---|---|
| A1 | 调用点符号键 | 活代码（折叠后）每条 invoke 的常量池 `(owner, name, desc)` 导出为 `refs`；emitter 的槽位需求门控按此消费 |
| A2 | 镜像独有类 / VM 支持类 | `--image <目录>`（`Origin::Image`，接入层把 jimage 抽取物与 `runtime/java_support` 编译物给出）；规则：父类在闭包内的非接口 Image 类 → 实例化 + 全部方法入链 + 全量反射面；同直接父类的闭包类（物种族）同取全量反射面。导出 `seeds.image` |
| A3 | 注解种子 | `seeds.toml [annotation]`：triggers 可达时，按用户类 RuntimeVisibleAnnotations 传递收集注解类型（方法入链）、枚举元素类型（`<clinit>` + 钩子）、Class 元素类型（L1 类型） |
| A4 | Locale 资源束种子 | `seeds.toml [locale]`：consts / factories / tags 推出 locale 集（含父链），族 triggers 可达时入选 `<包>/<基名>_<locale>` 束类：实例化 + `<init>()V` + 载体方法 |
| A5 | JCA 服务种子 | `seeds.toml [jca]`：provider 注册类三连常量提取服务，候选算法名 = 用户字符串常量 ∪ 链上 JDK 方法 `getInstance` 前字符串实参（含 alias_sources 同义名、defaults），engine 类在链上即入选；实现类全部构造器 + provider 类实例化 |
| A6 | 纯数据束判定 | `[data_bundle] carriers`：边界前缀内的纯数据类按翻译域处理（与 Python `_is_data_bundle` 同判定） |
| A7 | 库模式 | `--lib <jar>`（`Origin::Lib`，可多次）+ `--seed-class <类>`（public 方法为根，排除 `main`） |
| A8 | 额外根 | `--root <类.方法:描述符>`（gap_scan API 模式）：方法入链 + 类初始化 + 构造器实例化 |

## 三、接入层（Python）

新模块 `codegen/closure_input.py`（≤ 600 行）：

1. 选 JDK（沿用 `JdkResolver(prefer_major=…)` 与打印行）→ 调 `cargo run --release -q --manifest-path generator/Cargo.toml -- closure <用户类目录> --java-home … --runtime <scratch>/java_runtime [--lib …] [--image …] -o <scratch>/closure.json`。
2. 读 `closure.json`：非 user 域类按名装载 ClassInfo（库类取 jar 注册表同一对象）；`visited_methods` = `methods` ∪ `refs` ∪ `clinit`；种子全局按 `seeds` 填充；`field_stubs` = `level=type` 的类。
3. `transpile.py` 第 3 步改调 `closure_input.discover(...)`，返回形状不变，下游零改动。

## 四、验收

1. 验收集 27 例 `gen_trees.sh` 生成成功、`cargo build` 全过（与 BFS 树的差异只用于定位，不要求逐字节一致——闭包本身变精确了）。
2. 全量 e2e：JDK 21 + 25 全绿（基线失败项不新增）；`gap_scan` precheck 无新增缺口。

## 五、删除清单（验收通过后执行）

`callchain.py` 发现部分整体（`_discover_jdk_classes_method_level` 及其全部内部函数、`_collect_method_refs`、`_scan_reflect_consts`、`_fill_precheck` 中依赖发现内部状态的部分），`locale_seed.py` / `jca_services.py` 中只服务发现的函数，脚本 `trace_callchain*.py`、`rta.py`、`analyze_callchain.py`、`dep_scan.py` 闭包部分、`scan_jdk_boundary.py`；`transpile.py` 中的死导入。保留：`_is_boundary_class` 等清单判定（发射期仍用）、`precheck_from_tree` / `print_precheck`。

## 六、进度

| 步 | 内容 | 状态 |
|---|---|---|
| 1 | A1 refs | ✅ `engine/invoke.rs` 活代码调用点符号键；手写 `use` / 路径类型引用（`handwritten/type_refs.rs`）按类型级入闭包 |
| 2 | A2 镜像 / VM 支持类 | ✅ `--image`（`JdkResolver.image_class_dirs()`）+ `engine/seeds.rs` `seed_image`；导出 `seeds.reflect_all / reflect_names` |
| 3 | A3–A6 种子 | ✅ `seeds/{annotation,locale,jca,data_bundle}.rs`（纯判定）+ `engine/seeds.rs`（工作队列排空时补种，外层不动点）；A6 在 `Ctx::domain` |
| 4 | A7–A8 库模式 / 额外根 | ✅ `--lib`（`Origin::Lib` 一律翻译域）/ `--seed-class` / `--root`（`Engine::root_seed`） |
| 5 | 接入层 + transpile 切换 | ✅ `codegen/closure_input.py`；visited 按方法 kind 过滤（vm_boundary 类按方法划分，不能按类域过滤） |
| 6 | 验收 27 例 + 全量 e2e | ⏳ |
| 7 | 删除 Python 发现机制（含 `closure.toml [vm_boundary] whole_class`：只有 Python BFS 读） | ⏳ |
