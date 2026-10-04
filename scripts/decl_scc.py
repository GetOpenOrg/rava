#!/usr/bin/env python3
"""声明层跨类引用图的强连通分量（S7 方案 §九「java.base 声明层能否再拆」的度量工具）。

  scripts/decl_scc.py <scratch> [--module-pkgs <包名清单文件>]

读 <scratch>/java_runtime/src 下生成的声明文件（每个 java_class! / java_interface! 一类），按三种口径建图：
  all：现状声明层全部引用（文件的 use 列表，#3 剪枝后即实际引用）；
  sig：S7 后仍留在声明里的具名类型——继承 / 接口实现 + 方法签名、字段、static、继承字段中的类型；
  inh：再把 vtable 签名擦除为句柄后只剩继承 / 接口实现 / 描述符互指。
输出各口径的最大 SCC；给出模块包清单（每行一个包名，如 `java --describe-module java.base` 的
exports / contains / qualified exports）时另报该模块子图的最大 SCC。
"""
import os,re,sys,collections
root=sys.argv[1]+'/java_runtime/src'
PKGS=sys.argv[sys.argv.index('--module-pkgs')+1] if '--module-pkgs' in sys.argv else None
BIN=re.compile(r'#\[binary_name\s*=\s*"([^"]+)"\]')
SUP=re.compile(r'#\[super_class\s*=\s*"([^"]+)"\]')
ALLS=re.compile(r'#\[all_supertypes\s*=\s*"([^"]*)"\]')
DECL=re.compile(r'^\s*pub (?:struct|trait) ([A-Za-z_][A-Za-z0-9_]*)',re.M)
USE=re.compile(r'^use crate::(?:[a-z_0-9]+::)+([A-Za-z_][A-Za-z0-9_]*);',re.M)
IDENT=re.compile(r'\b([A-Z][A-Za-z0-9_]*)\b')
nodes={}  # ident -> dict
for dp,_,fs in os.walk(root):
    for f in fs:
        if not f.endswith('.rs') or f.endswith('_impl.rs'): continue
        t=open(os.path.join(dp,f),encoding='utf-8',errors='ignore').read()
        if 'java_class!' not in t and 'java_interface!' not in t: continue
        b=BIN.search(t)
        if not b: continue
        # 首个声明名
        m=re.search(r'java_(?:class|interface)!\s*\{.*?^\s*pub (?:struct|trait) ([A-Za-z_][A-Za-z0-9_]*)',t,re.S|re.M)
        if not m: continue
        name=m.group(1)
        sup=SUP.search(t); alls=ALLS.search(t)
        uses=set(u for u in USE.findall(t))
        # 签名 / 字段中的类型名（fn 声明行、字段行、static 行）
        sig=set()
        for line in t.split('\n'):
            s=line.strip()
            if s.startswith('#[superclass_fields') or s.startswith('pub fn') or s.startswith('fn ') or (s.startswith('pub ') and ':' in s and not s.startswith('pub struct') and not s.startswith('pub trait')):
                sig.update(IDENT.findall(s))
        nodes[name]=dict(bin=b.group(1),sup=sup.group(1) if sup else None,
            alls=[x for x in (alls.group(1).split(';') if alls else []) if x],uses=uses,sig=sig)
bin2id={v['bin']:k for k,v in nodes.items()}
def base(u):
    for suf in ('__VTable','__AsVTable','__Lambda'):
        if u.endswith(suf): return u[:-len(suf)]
    if '__' in u:
        return u.split('__')[0]
    return u
def graph(kind):
    g={n:set() for n in nodes}
    for n,v in nodes.items():
        inh=set()
        if v['sup'] and v['sup'] in bin2id: inh.add(bin2id[v['sup']])
        for x in v['alls']:
            if x in bin2id: inh.add(bin2id[x])
        if kind=='inh': e=inh
        elif kind=='sig': e=inh|{x for x in v['sig'] if x in nodes}
        else: e=inh|{base(u) for u in v['uses'] if base(u) in nodes}
        e.discard(n); g[n]=e
    return g
def sccs(g):
    idx={};low={};on=set();st=[];out=[];c=[0]
    for s in g:
        if s in idx: continue
        work=[(s,iter(g[s]))]; idx[s]=low[s]=c[0];c[0]+=1;st.append(s);on.add(s)
        while work:
            v,it=work[-1]; adv=False
            for w in it:
                if w not in idx:
                    idx[w]=low[w]=c[0];c[0]+=1;st.append(w);on.add(w);work.append((w,iter(g[w])));adv=True;break
                elif w in on: low[v]=min(low[v],idx[w])
            if adv: continue
            work.pop()
            if work: low[work[-1][0]]=min(low[work[-1][0]],low[v])
            if low[v]==idx[v]:
                comp=[]
                while True:
                    w=st.pop();on.discard(w);comp.append(w)
                    if w==v: break
                out.append(comp)
    return out
print(f"## {sys.argv[1]}：{len(nodes)} 个类 / 接口")
for kind,label in (('all','现状声明层全部引用（use 剪枝后）'),('sig','S7 后：继承 + 签名 / 字段 / static 中的具名类型'),('inh','S7 + vtable 签名擦除：只剩继承 / 接口实现 / 描述符')):
    g=graph(kind); cs=sorted(sccs(g),key=len,reverse=True)
    big=[len(c) for c in cs if len(c)>1]
    ne=sum(len(e) for e in g.values())
    print(f"- {label}：边 {ne}，最大 SCC {len(cs[0])}（{len(cs[0])*100//len(nodes)}%），>1 的 SCC {len(big)} 个，前 5：{big[:5]}")
    if kind=='sig':
        biggest=set(cs[0])
        pk=collections.Counter(nodes[n]['bin'].rsplit('/',1)[0] for n in biggest)
        print('  - 最大 SCC 按包前 8：',pk.most_common(8))
if not PKGS: sys.exit(0)
pk=set(open(PKGS).read().split())
inbase={n for n,v in nodes.items() if v['bin'].rsplit('/',1)[0].replace('/','.') in pk}
print(f"- 模块内类：{len(inbase)}（{len(inbase)*100//len(nodes)}%）")
for kind in ('all','sig'):
    g=graph(kind); gb={n:{w for w in g[n] if w in inbase} for n in inbase}
    cs=sorted(sccs(gb),key=len,reverse=True)
    print(f"  - 模块子图 {kind}：最大 SCC {len(cs[0])}")
