#!/bin/bash
# 方法句柄对象化收益上界的反事实实验（docs/plans/2026-10-06-mh-objectify.md §五）。服务器作业用，逐例调用
# closure_probe_job.sh；`--cut` 不健全，只量上界。产物在 build/cj/<实验>/<Test>.*（取回：--fetch 'build/cj/**'）。
# 用法：scripts/diag/mh_bound_job.sh <实验> [Test...]（缺省 HelloWorld StockTrans DeepCopy TestSerialDefaultSuid）
# 实验名为若干组件以 + 连接（如 mh+4b+b1+nos），组件：
#   base  无切除
#   mh    方法句柄通道实参池 RP(1) 不收值：句柄成员形参与接收者只剩「对象化完美成立」后的真实来源之下界（乐观）
#   mhp   只切 RN(1)：句柄成员形参不接池，接收者派发保留整池
#   4b    反射对象通道成员形参不接池 RN(0)（§7.4 按成员分池的乐观下界，同 §8.1 t2b-exp1）
#   b1    FieldReflector 字段视图环（§9.1 cutfr）
#   jar   §29 路线一 D（URLClassPath$3.run 两处 new JarLoader + jar Handler.openConnection）
#   jca   §29 路线二 P（ProviderConfig.doLoadProvider）
#   rb    §29 路线三 S（ResourceBundle.getServiceLoader@16）
#   nos   DeepCopy.deepCopy 整体（完全不走序列化，只对 DeepCopy 有意义）
set -u
REPO=$(cd "$(dirname "$0")/../.." && pwd)
exp=$1; shift
tests=("$@")
[ ${#tests[@]} -eq 0 ] && tests=(HelloWorld StockTrans DeepCopy TestSerialDefaultSuid)
args=()
IFS='+' read -r -a parts <<< "$exp"
for p in "${parts[@]}"; do
  case $p in
    base) ;;
    mh) args+=(--cut '@node:reflect-call 实参池 方法句柄') ;;
    mhp) args+=(--cut '@node:reflect-call 实参池（去冗余） 方法句柄') ;;
    4b) args+=(--cut '@node:reflect-call 实参池（去冗余） 反射对象') ;;
    b1) args+=(--cut 'java/io/ObjectStreamClass$FieldReflector.getObjFieldValues:(Ljava/lang/Object;[Ljava/lang/Object;)V'
               --cut 'java/io/ObjectStreamClass$FieldReflector.setObjFieldValues:(Ljava/lang/Object;[Ljava/lang/Object;Z)V') ;;
    jar) args+=(--cut 'jdk/internal/loader/URLClassPath$3.run:()Ljdk/internal/loader/URLClassPath$Loader;@97'
                --cut 'jdk/internal/loader/URLClassPath$3.run:()Ljdk/internal/loader/URLClassPath$Loader;@139'
                --cut 'sun/net/www/protocol/jar/Handler.openConnection:(Ljava/net/URL;)Ljava/net/URLConnection;') ;;
    jca) args+=(--cut 'sun/security/jca/ProviderConfig.doLoadProvider:()Ljava/security/Provider;') ;;
    rb) args+=(--cut 'java/util/ResourceBundle.getServiceLoader:(Ljava/lang/Module;Ljava/lang/String;)Ljava/util/ServiceLoader;@16') ;;
    nos) args+=(--cut 'DeepCopy.deepCopy:(Ljava/lang/Object;)Ljava/lang/Object;') ;;
    *) echo "未知组件 $p" >&2; exit 2 ;;
  esac
done
for t in "${tests[@]}"; do
  bash "$REPO/scripts/diag/closure_probe_job.sh" "$exp" "$t" ${args[@]+"${args[@]}"} --flows '@rcall'
done
