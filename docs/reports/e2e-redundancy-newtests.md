# 新增用例查重报告（任务 4：只出清单不删除）

> 生成：`python3 scripts/e2e_redundancy_scan.py --report docs/reports/e2e-redundancy-scan.md`
> 后与「本轮新增 133 例」清单求交（新增 = `git diff --diff-filter=A 3152f3f4^..8f0121c5 -- tests/e2e/` 现存集）。
> 口径：同目录子集判定（名字级启发式）+ Jaccard ≥0.75 相似对；63_junit 10 例因
> 需要 junit classpath 无法裸 javac，落在编译失败节（预期内，另有 1 例既有基线
> TestUnnamedVar），不参与本分析。

## 结论速览

- **§一 强冗余候选 ∩ 新增 = 5 例**（下表，全部建议**保留**，逐条论证见后）；
- **§二 相似对 ∩ 新增 = 0 对**（新增 133 例之间、与存量之间无 J≥0.75）；
- 因此**本轮无建议删除项**；名字级"被覆盖"不构成删除依据的五条例证本身就是
  方法论文档 §六 第 3 条（名字级覆盖 ≠ 分支语义覆盖）的活教材。

## 五个候选与保留论证

| 候选 | 面大小 | 覆盖者（名字级） | 保留理由（独有语义） |
|---|---|---|---|
| 62_reflection/TestReflectStateWriteBack | 6 | TestInvokeNullArgs 等 3 例单点全覆盖 | **m5 缺陷（@Before 字段写丢失）的定向回归网**——反射写回单文件形态是翻译侧唯一已验证绿的面，删除即失去缺陷定位锚点 |
| 62_reflection/TestReflectOverloadResolution | 14 | 最高 9/14（无单点） | 独有固化点：**反射 invoke 对 int... 散参不装箱**（varargs-spread-ex 边界）、泛型方法 getGenericParameterTypes——名字级重叠来自 getMethod/invoke 等公共面 |
| 62_reflection/TestClassForNameInit | 9 | 最高 6/9（无单点） | 独有固化点：**forName 三参 initialize=true/false 的 <clinit> 触发计数**（懒初始化语义），测试类独有 static 计数器路径 |
| 64_charsets_ext/TestCharsetCjkFamily | 5 | TestCharsetGbk 单点全覆盖 | jmod 覆盖计划 64_charsets_ext 的 A 档三例之一，**Big5/Shift_JIS/EUC-JP/EUC-KR 四字符集的实跑往返**不在 GBK 例内（GB18030 欧元四字节亦独有） |
| 70_crypto_ec/TestEcSignVerify | 19 | TestRsaSignVerify 17/19（非单点） | **ECDSA** 的 DER 签名确定性/篡改拒绝与 RSA 例不同算法族；签名头字节逐字可比的期望独有 |

## 后续

若未来确需压缩语料，上述五例中仅 `TestCharsetCjkFamily` 可在"接受 jmod A 档
charsets 家族覆盖收窄"的前提下讨论；其余四例的独有语义（定向回归网 / 边界固化 /
算法族）在现有语料中无等价承载，删除会造成真实缺口。删除任何一例前建议重跑
`jdk_method_scan.py --report` 复核方法面零收缩。
