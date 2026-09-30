# emit crate 与 Python 生成器的差异登记

golden：`scripts/golden/dump_emit.py` 采集 → `build/golden/emit/<Test>/`；对照测试 `tests/golden.rs`
（TestHashMapOps / TestStreamBasic / TestCompletableFuture）。

## 当前对照状态

| 阶段 | 类文件对照范围 | 结果 |
|---|---|---|
| (b) 类块头 / struct / 字段 / 导入 | 文件头（cross_imports，去继承导入插入位后为 py 前缀）+ `java_class!` 块到 impl 头 | 3 例 2132 个文件全部一致 |
| (b) 非类文件 | Cargo.toml / mod.rs / lib.rs / 资源文件全文 | 一致；`user/src/main.rs` 反射分派注册表待步骤 (d) |
| (c) 方法块（clinit / 声明方法 / 手写覆盖 / 接口 lambda / 接口补全） | `java_class!` 块到继承段插入位（方法体经 bodies.jsonl 回放替换） | 除 2 个 record 类外全部一致；record 访问器补丁（`_patch_record_method_blocks`）属步骤 (d) |
| (c) 方法体请求 | bodies.jsonl 回放记录消费 | TestStreamBasic 25 条、TestCompletableFuture 235 条未请求——继承段（接口 default / special / 超类虚方法）的方法体请求属步骤 (d) |
| (d1) 继承展开段 + record 补丁 | 同上（方法块全段） | 3 例全部一致；bodies.jsonl 回放记录全部被请求（0 未用） |
| (d2) 第二阶段：接口实现 / 协变 upcast / 接口接收者超接口成员 / 类接收者继承成员 + use 行 | 全部类文件全文 | 3 例除 d3 尾段外逐字节一致（522 个文件的差异均为 py 多出的尾段：L3 反射字段 473、L3 反射分派 9、A-5 SAM 合成对象 40） |

| (d3) SAM 合成对象（A-5）+ L3 反射分派 / 字段闭包 + main 反射登记 | 全部文件全文 | 3 例 2132 个文件逐字节一致（失配 0）；回放方法体中 SAM 站点的构造路径与 `SamLedger::site_ctor_path` 一致 |

`tests/golden.rs` 全文对照，无待接入项（`PENDING` 已删除）。

## 一、未移植分支（显式 `EmitError::Unported`）

| 位置 | 分支 | 说明 |
|---|---|---|
| `project::write_project` | lib crate 模式（jar 输入） | `lib_crates` 非空即报错 |
| `imports::base_fn::base_member_name` | 调用描述符只命中 synthetic bridge 的 `__base` 命名（桥接重定向） | 三例未触发 |
| `imports::base_fn::base_member_name` | 接口接收者的 `__base` 命名（声明接口定位） | 三例未触发 |

未移植的 Python 形参：`java_visibility`（仅 lib crate 用）、`full_impl_classes`（`_impl.rs` 内含 `pub struct` 的
全量手写类；runtime 现无此类文件，按空集处理）、`stub_bodies`（project_writer 恒为 False）。

## 二、有意差异（Rust 取确定性 / 结构化口径，golden 三例无可见影响）

1. **conflict_map 包序**：Python `list(set)` 顺序随哈希；Rust 排序后迭代。
2. **invokedynamic 实现句柄**：Python 在指令注释文本里找 `' impl:'` 子串，字符串拼接模板 / 常量文本含该子串时
   会误命中；Rust 取结构化 bootstrap 方法（lambda 类引导、≥2 实参、第 2 实参为方法句柄）。
3. **手写伴生依赖**：Python 手维 `IMPL_FILE_DEPS` 表；Rust 由 `closure::handwritten` 的 use 类型引用推导（4c783fa7）。
4. **scratch 包版本**：`scratch_pkg_version` 按 `os.path.abspath` 口径词法消去 `..`（`std::path::absolute`
   不消去，路径含 `..` 时与 Python 不一致——已修正为一致，记录以免回退）。

5. **Fallback 副作用**：Python 方法体翻译失败回落存根时，已登记的部分副作用（lambda 定义、跨类请求）残留在
   项目状态中；Rust 在 `BodyError::Fallback` 时整体丢弃该方法体的副作用（`ProjectState::absorb` 仅在成功时调用）。
   golden 三例的回落方法无残留副作用，故无可见差异。
6. **lambda 命名为 emit 私有最小移植**：根 API 重载判断 → `hierarchy_overloaded_names` 改名 → `safe_ident`，
   与 `codegen/instr` 同口径；P4c method crate 落地后统一到其公共实现。
7. **FS-H0 手写审计**：`ProjectState.hw_audit` 已按 VmBoundary / Intrinsic / Override 分类记录，尚未写入
   raw_audit 输出（P0 driver 接入时落盘）。

## 三、Python 行为照搬（疑似缺陷，按原样移植，不在 P5a 修）

0. **record 方法文本补丁**：`record.rs` 按方法体文本中的 `/* TODO: invokedynamic` 占位识别 record 的
   toString / hashCode / equals 并整体替换（Python `_patch_record_method_blocks`）。终态应由方法体生成器
   直接翻译 `ObjectMethods` 引导点，届时删除该模块。

1. **空串 ConstantValue**：`""` 字符串常量被当作「无常量」，走 clinit 抽取而非常量初始化。
2. **用户类模块名**：layout 的用户类 mod 名取简单名，而 vtable / `__base` 导入的模块路径用 `to_snake(binary)`
   （完整 binary name）；包内用户类两者不一致（Python 同样如此）。
3. **手写覆盖引用不过滤**：`_handwritten_inherited_overrides` 合成声明的签名引用在生成集过滤**之后**加入
   referenced，JDK 类中可能引用生成集外的类型。
4. **注册表插入序依赖**：`scan_used_vtable_imports` 与 `__base` 短名反查取注册表插入序首个短名命中者；
   Rust 注册表插入序与 Python dict 序一致时才等价（`ShortNames.index` 为后写覆盖，故手工按插入序迭代）。
5. **浮点常量边界**：NaN payload 与代理对字符的常量文本按 Python repr 近似，非规范 NaN 可能与 Python 不同。

6. **SAM 站点断言时机**：Python `record_site` 在站点生成时即抛错；Rust 由方法体生成器经 `BodyEffects::sam_sites`
   登记，发射层在第二阶段开头统一 `SamLedger::check_sites`（断言内容相同，报错点后移）。
7. **SAM 预扫描账本**：Python 模块级 `SAM_LEDGER`；Rust 为 `EmitCtx::sam()`（首次查询时预扫描，只读缓存）。
   预扫描读规范化方法体（`EmitInput::code`），与 Python 发射前已折叠的指令同源。
