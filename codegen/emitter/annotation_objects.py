"""反射注解代理合成（L3 段 1）：getAnnotation 的实例形态。

JDK 侧 `getAnnotation` 返回实现了注解接口的动态代理（AnnotationParser 在
运行时合成代理类）；本架构没有运行时代码生成，对应物是**翻译期合成的最小
注解实例**——与 sam_objects 的 `<Iface>__Lambda` 同一合成模式：

    #[derive(Clone, Default)]
    pub struct <Short>__AnnotationProxy { <元素字段（默认值 baked）> }

- `from_values(&[(String, AnnotationValue)]) -> Result<Object>`：注解工厂
  （元素缺省 → AnnotationDefault 默认值；运行时经 java_runtime::
  annotation_meta::annotation_instance 按名代调，生成项目 main 启动时登记）；
- `impl ObjectVTable`：`__class_name` = 注解类型 binary name（getClass() /
  instanceof 正确）、`is_instance_of`（接口闭包 + Object）、`__interface`
  （接口载体视图应答——checkcast 到注解类型、经载体调用元素方法由此成立）；
- `impl <I>__VTable`（本接口元素访问器 + `java/lang/annotation/Annotation`
  的 `annotationType()`）。

合成谓词（证据驱动，零猜测）：注解类型接口（ACC_ANNOTATION）**以 ldc 类
字面量出现**于本轮语料——getAnnotation/isAnnotationPresent 的参数是 Class
对象，其唯一构造通道是类字面量（`X.class` → ldc，Class.forName 字符串形态
的注解查询缺席于语料消费面）；从未 ldc 的注解类型在运行时不可命名，不付
合成成本。

元素承载面（本批）：无参访问器 + 基本类型 / String / Class 返回形态
（JUnit 的 @Test.timeout():J / .expected():Class 即此形态）。enum / 数组 /
嵌套注解元素不合成字段——访问器落接口 vtable 的缺省 panic stub（如实定性，
语料出现时再扩）。

共置伴生形态：代理追加在注解接口自身的翻译文件尾部（与 sam_objects 同一
形态——vtable 签名与接口发射同源，同文件保证原子生成）。
"""

from ..type_map import jvm_to_rust, short_cls, parse_descriptor_return
from ..constants import OBJECT_CLASS, ANNOTATION_IFACE

_ACC_ANNOTATION = 0x2000
_OBJECT_CLASS = OBJECT_CLASS
_ANNOTATION_IFACE = ANNOTATION_IFACE

# 已合成的注解类型（{iface_bin: 工厂登记行}）——project_writer 写 main 时经
# registration_lines 消费。与 SAM_LEDGER 同生命周期（每轮 reset）。
LEDGER: dict[str, str] = {}


def reset() -> None:
    LEDGER.clear()


def _ldc_class_candidates(registry: dict) -> set:
    """本轮语料全部 ldc 类字面量的 binary name 集合。"""
    out: set = set()
    for ci in registry.values():
        for m in ci.methods or []:
            for ins in m.instrs or []:
                if ins.opcode in ('ldc', 'ldc_w') and ins.comment \
                        and ins.comment.startswith('class '):
                    out.add(ins.comment[6:].strip())
    return out


def _iface_closure(iface_bin: str, registry: dict) -> list:
    """{I} + 传递超接口（registry 内，广度优先发现序去重）。"""
    out, seen, queue = [], set(), [iface_bin]
    while queue:
        name = queue.pop(0)
        if name in seen:
            continue
        seen.add(name)
        out.append(name)
        ci = registry.get(name)
        if ci is not None:
            queue.extend(ci.interfaces or [])
    return out


def _contract_methods(jci) -> list:
    """接口自身声明的契约实例方法（与 sam_objects._contract_methods 同过滤）。"""
    from ..instr.member_owner import _root_virtual_methods
    root_keys = _root_virtual_methods()
    out = []
    for m in jci.methods:
        if m.is_static or m.is_synthetic or m.name.startswith('<'):
            continue
        if m.access_flags & 0x0002:
            continue
        if (m.name, m.descriptor[:m.descriptor.find(')') + 1]) in root_keys:
            continue
        out.append(m)
    return out


def _anno_unesc(s: str) -> str:
    """载荷百分号解码（与 classfile._anno_esc 互逆）。"""
    import urllib.parse
    return urllib.parse.unquote(s, errors='strict')


def _rust_str_lit(s: str) -> str:
    return '"' + s.replace('\\', '\\\\').replace('"', '\\"') + '"'


def _value_conv(rust_ty: str, anno_val: str, rt: str, cls_ty: str) -> 'str | None':
    """元素值 AnnotationValue → 字段类型的转换表达式（`Some(x)` 形态）。

    按字段 Rust 类型生成 match（宽容宽化：J/I/S/B 数值族互通，D/F 互通）；
    不匹配的变体 → None（保持默认值——JDK 对注解记录的值类型与访问器类型
    恒一致，宽容仅为防御）。
    """
    m = f'{anno_val}::'
    if rust_ty == 'i64':
        return (f'match v {{ {m}J(x) => Some(*x), {m}I(x) => Some(*x as i64), '
                f'{m}S(x) => Some(*x as i64), {m}B(x) => Some(*x as i64), _ => None }}')
    if rust_ty == 'i32':
        return (f'match v {{ {m}I(x) => Some(*x), {m}S(x) => Some(*x as i32), '
                f'{m}B(x) => Some(*x as i32), {m}J(x) => Some(*x as i32), _ => None }}')
    if rust_ty == 'i16':
        return (f'match v {{ {m}S(x) => Some(*x), {m}I(x) => Some(*x as i16), '
                f'{m}B(x) => Some(*x as i16), _ => None }}')
    if rust_ty == 'i8':
        return (f'match v {{ {m}B(x) => Some(*x), {m}I(x) => Some(*x as i8), '
                f'{m}S(x) => Some(*x as i8), _ => None }}')
    if rust_ty == 'bool':
        return f'match v {{ {m}Z(x) => Some(*x), _ => None }}'
    if rust_ty == 'u16':
        return f'match v {{ {m}C(x) => Some(*x), {m}I(x) => Some(*x as u16), _ => None }}'
    if rust_ty == 'f32':
        return f'match v {{ {m}F(x) => Some(*x), {m}D(x) => Some(*x as f32), _ => None }}'
    if rust_ty == 'f64':
        return f'match v {{ {m}D(x) => Some(*x), {m}F(x) => Some(*x as f64), _ => None }}'
    if rust_ty == 'String':
        return (f'match v {{ {m}Str(x) => Some(String::from(x.as_str())), _ => None }}')
    if rust_ty == 'Class':
        # class 元素载荷是 JVM 描述符形态（L..; / [..）——归一后 for_class
        return (f'match v {{ {m}Cls(d) => Some({cls_ty}::for_class(String::from('
                'if d.starts_with(\'L\') && d.ends_with(\';\') { &d[1..d.len()-1] } '
                'else { d.as_str() }))), _ => None }')
    return None


def _default_rust_literal(tag: str, value: str, rust_ty: str) -> 'str | None':
    """AnnotationDefault 的 (tag, 编码载荷) → Rust 字段默认值表达式。"""
    try:
        if tag in 'BIS' and rust_ty in ('i8', 'i16', 'i32'):
            v = int(value)
            suf = {'i8': 'i8', 'i16': 'i16', 'i32': ''}[rust_ty]
            if abs(v) < (1 << 31):
                return f'{v}{suf}'
        if tag == 'Z' and rust_ty == 'bool':
            return 'true' if value == 'true' else 'false'
        if tag == 'J' and rust_ty == 'i64':
            return f'{int(value)}i64'
        if tag == 'C' and rust_ty == 'u16':
            return f'{int(value)}u16'
        if tag == 'F' and rust_ty == 'f32':
            f = float(value)
            return f'{f}f32' if f == f and abs(f) != float('inf') else '0.0f32'
        if tag == 'D' and rust_ty == 'f64':
            f = float(value)
            return f'{f}' if f == f and abs(f) != float('inf') else '0.0'
        if tag == 's' and rust_ty == 'String':
            return f'String::from({_rust_str_lit(_anno_unesc(value))})'
        if tag == 'c' and rust_ty == 'Class':
            desc = _anno_unesc(value)
            if desc.startswith('L') and desc.endswith(';'):
                desc = desc[1:-1]
            return f'Class::for_class(String::from("{desc}"))'
    except (ValueError, OverflowError):
        pass
    return None


_ZERO_LITERALS = {'i64': '0i64', 'i32': '0', 'i16': '0', 'i8': '0', 'bool': 'false',
                  'u16': '0u16', 'f32': '0.0f32', 'f64': '0.0', 'String': 'String::from("")'}


def synthesize(emissions: dict, registry: dict) -> None:
    """对谓词命中的每个注解类型，把代理文本追加到接口发射尾部。

    在全部类文本生成之后、落盘之前调用（条目签名取自发射记录——与落盘
    内容同源）。同时登记工厂路径（main 启动登记用）。
    """
    from .inherited_gen import class_use_path

    candidates = _ldc_class_candidates(registry)
    targets = sorted(
        n for n, ci in registry.items()
        if n in candidates and getattr(ci, 'is_interface', False)
        and (ci.access_flags & _ACC_ANNOTATION)
        and n in emissions and not emissions[n].handwritten
    )
    for iface_bin in targets:
        em = emissions[iface_bin]
        jci = registry[iface_bin]
        short = short_cls(iface_bin)
        proxy = f'{short}__AnnotationProxy'
        in_rt = getattr(em, 'crate_name', '') == 'java_runtime'
        rt = 'crate' if in_rt else 'java_runtime'
        anno_val = f'{rt}::annotation_meta::AnnotationValue'
        cls_ty = f'{rt}::java::lang::Class'

        closure = _iface_closure(iface_bin, registry)
        lines: list[str] = []
        lines.append('// ── 反射注解代理（getAnnotation 的实例形态；JDK 动态代理的翻译期同构物）──')
        lines.append(f'// {iface_bin} 的最小注解实例：元素字段 + AnnotationDefault 默认值 baked，')
        lines.append('// 实现本接口（及闭包内 Annotation）的 __VTable；工厂经 from_values 登记。')
        lines.append('')

        # 元素字段：注解接口自身声明的无参访问器（注解元素的 JVMS 约束）
        fields: list[tuple] = []  # (字段名, rust 名, 类型, 默认值, Java 元素名)
        for m in _contract_methods(jci):
            pdesc = m.descriptor[1:m.descriptor.find(')')]
            if pdesc != '':
                continue
            em_m = em.find(m.name, '()')
            if em_m is None:
                continue
            ret = parse_descriptor_return(m.descriptor)
            if ret == 'V':
                continue
            try:
                rust_ty = jvm_to_rust(ret, registry)
            except (ValueError, IndexError):
                continue
            conv = _value_conv(rust_ty, anno_val, rt, cls_ty)
            if conv is None:
                continue  # 形态不承载：访问器落 trait 缺省 panic stub
            default = None
            if m.annotation_default is not None:
                tag, value = m.annotation_default
                default = _default_rust_literal(tag, value, rust_ty)
                if default is None:
                    continue  # 默认值形态不承载：同上
            else:
                default = _ZERO_LITERALS.get(rust_ty)
                if rust_ty == 'Class':
                    default = f'{cls_ty}::default()'
                elif default is None:
                    continue
            fname = 'v_' + m.name if m.name in ('type', 'name', 'self') else m.name
            fields.append((fname, em_m.rust_name, rust_ty, default, m.name))

        lines.append('#[derive(Clone, Default)]')
        lines.append(f'pub struct {proxy} {{')
        for fname, _rn, fty, _dv, _jn in fields:
            lines.append(f'    pub {fname}: {fty},')
        lines.append('}')
        lines.append('')
        lines.append(f'impl {proxy} {{')
        lines.append('    /// 注解工厂：元素值序列 → 实例（缺省元素回落 AnnotationDefault 默认）。')
        lines.append(f'    pub fn from_values(')
        lines.append(f'        vals: &[(std::string::String, {anno_val})],')
        lines.append(f'    ) -> {rt}::error::Result<{rt}::java::lang::Object> {{')
        lines.append(f'        let mut p = {proxy} {{')
        for fname, _rn, _ty, dv, _jn in fields:
            lines.append(f'            {fname}: {dv},')
        lines.append('        };')
        lines.append('        for (k, v) in vals {')
        lines.append('            match k.as_str() {')
        for fname, _rn, ty, _dv, jname in fields:
            conv = _value_conv(ty, anno_val, rt, cls_ty)
            lines.append(f'                "{jname}" => if let Some(x) = {conv} {{ p.{fname} = x; }},')
        lines.append('                _ => {}')
        lines.append('            }')
        lines.append('        }')
        lines.append(f'        Ok({rt}::java::lang::Object::from(p))')
        lines.append('    }')
        lines.append('}')
        lines.append('')

        # instanceof 名单：接口闭包 + Object（注解类型不实现 Serializable）
        patterns = sorted({iface_bin, *closure, _OBJECT_CLASS})
        lines.append(f'impl {rt}::java::lang::ObjectVTable for {proxy} {{')
        lines.append('    fn as_any(&self) -> &dyn std::any::Any { self }')
        lines.append(f'    fn __obj_str(&self) -> std::string::String {{ std::format!("@{iface_bin}") }}')
        lines.append(f'    fn __class_name(&self) -> &\'static str {{ "{iface_bin}" }}')
        lines.append('    fn is_instance_of(&self, type_id: &str) -> bool {')
        lines.append('        matches!(type_id, ' + ' | '.join(f'"{p}"' for p in patterns) + ')')
        lines.append('    }')
        lines.append('    fn __interface(self: __Shared<Self>, slot: &mut dyn std::any::Any) {')
        impl_targets: list[tuple[str, str]] = []
        for jbin in closure:
            jci2 = registry.get(jbin)
            if jci2 is None or not _contract_methods(jci2):
                continue
            jpath = class_use_path(jbin, em.crate_prefix, emissions,
                                   getattr(em, 'crate_name', ''))
            lines.append('        if let Some(s) = slot.downcast_mut::'
                         f'<Option<__Shared<dyn {jpath}__VTable>>>() {{ *s = Some(self); return; }}')
            impl_targets.append((jbin, jpath))
        lines.append('    }')
        lines.append('}')
        lines.append('')

        # 闭包内各接口的 vtable 实现：本接口元素访问器 + Annotation 的
        # annotationType（equals/hashCode/toString 是根类方法重声明，不进
        # vtable 契约——根分派承载）
        for jbin, jpath in impl_targets:
            jci2 = registry.get(jbin)
            jem2 = emissions.get(jbin)
            entries: list[str] = []
            if jbin == iface_bin:
                # 元素访问器条目：签名头取发射记录，体返回字段克隆
                for m in _contract_methods(jci):
                    pdesc = m.descriptor[1:m.descriptor.find(')')]
                    if pdesc != '':
                        continue
                    em_m = em.find(m.name, '()')
                    if em_m is None:
                        continue
                    hit = next((f for f in fields if f[1] == em_m.rust_name), None)
                    if hit is None:
                        continue  # 形态未承载 → trait 缺省 panic stub
                    fname = hit[0]
                    sig = em_m.signature
                    head = sig[len('pub fn '):].rstrip()
                    head = head.split(' {')[0].rstrip(';').rstrip()
                    entries.append(f'    fn {head} {{ Ok(Clone::clone(&self.{fname})) }}')
            if jbin == _ANNOTATION_IFACE and jem2 is not None:
                em_m = jem2.find('annotationType', '()')
                if em_m is not None:
                    # 返回类型的全路径改写：trait 条目签名在 Annotation 的文件
                    # 上下文解析（Class / Result），代理所在文件按全路径书写
                    entries.append(
                        f'    fn {em_m.rust_name}(&self) -> {rt}::error::Result<{cls_ty}> '
                        f'{{ Ok({cls_ty}::for_class(String::from("{iface_bin}"))) }}')
            # 槽位已登记（__interface 应答 Rc<dyn Iface__VTable>）则 impl 块必须存在：
            # 未承载的元素（enum 返回等形态）不生成条目，落到 trait 的缺省 panic
            # 体（AbstractMethodError 语义，与 stub 发射口径一致）——空 impl 缺席
            # 会让 __interface 的 `*s = Some(self)` 无处收窄（E0277）
            lines.append(f'impl {jpath}__VTable for {proxy} {{')
            lines.extend(entries)
            lines.append('}')
            lines.append('')

        em.text = em.text.rstrip('\n') + '\n\n' + '\n'.join(lines) + '\n'
        # 工厂登记路径（从用户 main 视角——user bin 依赖全部 lib crate 与
        # java_runtime；同 class_use_path 的 user 接收者语义）
        path = class_use_path(iface_bin, 'java_runtime', emissions, 'user')
        LEDGER[iface_bin] = f'    ("{iface_bin}", java_runtime::sync_model::__Shared::new(' \
            f'|v| {path}__AnnotationProxy::from_values(v))),'


def registration_lines() -> list:
    """main 启动时的注解工厂登记行（synthesize 之后调用）。"""
    return [LEDGER[bin] for bin in sorted(LEDGER)]
