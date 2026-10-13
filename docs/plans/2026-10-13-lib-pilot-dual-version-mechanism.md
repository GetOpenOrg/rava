# lib_pilot 同 GA 双版本并存机制议题（2026-10-13）

> 状态：**待用户裁决**——本文只摆事实与两案利弊，落选前不动 `tests/lib_pilot/deps/pom.xml`。
> 背景：依赖锁是「每坐标一份 jar」的扁平表（条目名 = artifactId），而 Maven 单 pom 对同一
> GA 坐标经依赖调解只产出一个版本，双版本不可表达。当前两处在撞墙：

- **esper 7.1.0 声明 `antlr4-runtime 4.7`**（esper-parent 7.1.0 属性实测值），但锁里是 **4.13.2**
  ——经 hibernate-core 7.4.6 传递（同为 depth 2，pom 声明序在前，nearest + first 调解胜出）。
  antlr 4.10 起生成代码与运行时二进制不兼容（runtime 与生成器须同版线），4.7 生成的 esper
  parser 跑 4.13.2 运行时会 NoSuchFieldError/NoSuchMethodError 级失配。
- **thymeleaf 3.1.3.RELEASE 托管 `ognl 3.3.4`**（thymeleaf-parent 属性实测值），但锁钉
  **ognl 3.4.14**（2026-10-03 矩阵钉值，且 OgnlCollection/Context/Eval 三个单文件 pilot
  golden 已按 3.4.14 存档）——不能改钉降级，只能并存。

即：esper 项目 form.toml 需要 antlr4-runtime@4.7，templates 项目需要 ognl@3.3.4，与主锁并存。

## 案 A：deps/pom 拆多模块

`tests/lib_pilot/deps/` 改父 pom + 子模块（如 deps-core / deps-esper / deps-thymeleaf），每模块
独立 `target/pilot-libs/` 子目录 + 独立 `deps.lock.toml`；冲突项目的 form.toml 把 `deps` 指到
对应模块锁（如 `tests/lib_pilot/deps-esper/target/deps.lock.toml`）。

跟进项：`fetch_pilot_deps.sh` 从单 pom 改遍历模块（或加模块参数，rm -rf 与锁生成都按模块做）；
`lib_pilot_golden.sh` 的 LOCK 缺省从「单一 pilot-libs 兄弟位」改为按 form.toml 的 `deps` 字段取。
**e2e 侧基本免改**：`e2e_form.py` 本就按 form.toml 的 `deps` 路径取锁、锁内 jar 路径相对锁目录
解析，`--pilot-libs DIR`（run_tests）也已支持换资产根（锁取 DIR 上级同名文件）——目录级寻址
对 form 机制是现成能力，不是新机制。

## 案 B：手工改名 jar 入库

如 `ognl-3.3.4-as-thymeleaf.jar` 手工放进 pilot-libs，锁条目名带后缀区分（条目名口径「无坐标取
文件名 stem」，机制上可行）；form.toml cp 直接引后缀条目名。

## 利弊对比

| 维度 | 案 A（多模块） | 案 B（改名 jar） |
|---|---|---|
| 缓存 | ~/.m2 共享，模块化不重复下载；模块间共享 jar 在各自 pilot-libs 各存一份（gitignore 生成物，磁盘冗余可接受） | 脱离 Maven 坐标系：~/.m2 不登记，升级与换机复现全靠手工 |
| 分发 | 全程 Maven 坐标系，锁 coordinate/sha256 语义不变，dep_scan 透视照用 | sha256 只保身份不保出处，coordinate 字段名存实亡；来源无坐标背书 |
| 维护 | 钉值仍声明在 pom（一处事实）；再撞同 GA 冲突 = 加子模块 | **fetch 每轮先 `rm -rf pilot-libs`，手工 jar 必被清掉**，需加「保留手工件」特例或每轮重放；ognl 3.3.x 传递依赖（javassist 线）也须手工对齐 |
| e2e 兼容 | form 机制天然兼容（per-form deps 路径 + --pilot-libs 已支持）；成本集中在 fetch/golden 两脚本小改 | cp 条目名机制不用改，表面改动最小，但 rm -rf 冲突是结构性缺陷 |

## 推荐：案 A

理由三条：① form 机制的目录级寻址已存在，案 A 的跟进成本是两个脚本的小改，而案 B 与 fetch
脚本的 rm -rf 清目录结构性相抵，要靠特例补丁长期背着；② 锁 / dep_scan / 取包全链路建立在
Maven 坐标系上，案 B 摘出一个 jar 就丢掉出处、传递、升级三重凭据；③ 双版本需求按项目分布
（esper、templates），与「每模块服务一组项目」的切分天然同构，未来 jakarta 线再分裂是加模块
而非改机制。

**待用户裁决**。确认案 A 后的落地序：pom 拆模块 → fetch 脚本模块化 → golden 脚本 LOCK 寻址
按 form.toml → esper/templates 两个 form.toml 切锁重钉。
