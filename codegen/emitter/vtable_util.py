# 从 codegen/emitter/class_writer.py 中拆出

from ..type_map import short_cls as _short_cls_g
from ..constants import safe_ident, OBJECT_CLASS as _OBJECT_CLASS
from ..type_map import mangle_name

_ACC_PRIVATE = 0x0002


def _bin_to_rust(binary_name: str) -> str:
    """将 JVM binary 名（含 / 和 $）转为 Rust 类型名。"""
    return _short_cls_g(binary_name)


def _descriptor_param_part(descriptor: str) -> str:
    """描述符的参数部分（含右括号）—— JVM 方法解析的签名键（返回类型可协变覆盖）。"""
    idx = descriptor.find(')')
    return descriptor[:idx + 1] if idx >= 0 else descriptor


def _same_vtable_slot(am, m) -> bool:
    """两个声明是否占据同一 vtable 槽位：同名且参数描述符一致（JVMS §5.4.5：
    覆盖按 name + 参数类型判定，返回类型允许协变收窄）。"""
    return (am.name == m.name
            and _descriptor_param_part(am.descriptor) == _descriptor_param_part(m.descriptor))


def _descriptor_return_part(descriptor: str) -> str:
    """描述符的返回类型部分。"""
    idx = descriptor.find(')')
    return descriptor[idx + 1:] if idx >= 0 else ''


def _slot_return_is_object(decl_ci, decl_m, registry) -> bool:
    """声明者 decl_m 的 vtable 槽位返回是否为 Object 位（A-1 按声明类判定）。

    声明者发射签名的返回（emitted_method_sig_types，与其文件逐字同源）提及
    声明者自身类型形参 → 宏擦除后槽位该位是 Object；返回字面即 Object 同理。
    协变覆盖条目只有落在 Object 位上才能经 vtable_erasure 名单对齐（宏侧既有
    机制只做 Object 化）；具体返回的槽位（如 MethodHandle.copyWith →
    MethodHandle，非泛型链）无法用该机制表达，覆盖者须留在中间声明者的
    自开槽位（既有行为，分派缺口另行立项）。"""
    from ..sig_types import emitted_method_sig_types
    from ..type_map import effective_class_type_params
    import re as _re
    decl_params = effective_class_type_params(decl_ci, registry)
    ret = emitted_method_sig_types(decl_ci, decl_m, decl_params, registry)[1] or ''
    mm = _re.match(r'^Result<(.*)>$', ret)
    inner = mm.group(1) if mm else ret
    if not inner or inner == 'Object':
        return True
    return any(_re.search(r'\b' + _re.escape(p) + r'\b', inner) for p in decl_params)


def _covariant_landing_slot(am, anc_ci, registry, handwritten_methods):
    """声明 am（属 anc_ci）的最终槽位落点 (owner_bin, 声明方法)：am 自身经协变
    模型逃逸到的声明者；无处逃逸 → (anc_ci.name, am)（自开槽位）。"""
    owner_short = _covariant_virtual_owner(am, anc_ci, registry, handwritten_methods)
    if not owner_short or owner_short == _bin_to_rust(anc_ci.name):
        return anc_ci.name, am
    cur = anc_ci.super_class
    seen: set[str] = {anc_ci.name}
    while cur and cur not in seen and cur != _OBJECT_CLASS and cur in registry:
        seen.add(cur)
        if _bin_to_rust(cur) == owner_short:
            hit = next((x for x in registry[cur].methods
                        if not x.is_synthetic and _same_vtable_slot(x, am)), None)
            if hit is not None:
                return cur, hit
        cur = registry[cur].super_class
    return anc_ci.name, am


def _covariant_virtual_owner(m: 'ParsedMethod', ci: 'ClassInfo',
                             registry: 'dict | None',
                             handwritten_methods: 'dict | None' = None) -> str:
    """同名 + 同参数描述符（返回类型可协变）的最远 vtable 槽位声明者（K-6a）。

    javac 为协变覆盖生成 ACC_BRIDGE 桥接方法（描述符 = 声明方的擦除描述符），
    它与真实方法同名 → ``_find_virtual_in`` 的同名方法计数含桥接时两侧不等，
    精确描述符匹配也找不到祖先声明 → 覆盖被判为本类新槽位（virtual_in=self），
    父槽位落空成 trait default 的 stub panic（构造器链 / super 视图调用命中）。

    本函数按槽位的精确签名模型重走超类链，判定不改 ``_find_virtual_in`` 的
    重载计数：
      1. 槽位声明 = 同名 + 同参数描述符的非私有真实声明（synthetic / bridge 是
         编译器产物，其描述符复述更早声明者的擦除形态，不作槽位签名源）；
         手写 / 私有声明不作槽位，链继续上溯（与 _find_virtual_in 同语义）；
      2. 名字一致性守卫（与 _find_virtual_in 守卫同一目的）：mangle 状态取
         hierarchy_overloaded_names —— 唯一权威（synthetic/bridge 不计、按参数
         列表计数、单调继承）。子类 mangle 而祖先未 mangle → 名字将不一致，
         停止追踪（其余形态名字必然一致：覆盖条目与槽位的 mangle 后缀同源，
         均取自参数描述符）；
      3. 逐跳可对齐校验：相邻两级声明的返回描述符不同（协变跳转）时，该跳的
         **落点槽位**（声明者自身经协变模型逃逸后的最终声明，多级协变链传递）
         的返回必须是 Object 位（``_slot_return_is_object``）——协变覆盖条目经
         vtable_erasure 名单（Object 化）才能与槽位签名对齐；具体返回槽位
         （如 CalendarSystem.getCalendarDate → CalendarDate，非泛型链）无法用
         该机制表达 → 停在上一级声明者（其自开槽位 / 回落 _find_virtual_in
         结果，既有行为）。逐跳校验对链上每个声明者一致生效 → 全链各文件的
         槽位归属汇于同一点。"""
    # 与 _find_virtual_in 同源的非虚守卫：构造器（含匿名类转发构造器，其参数
    # 描述符常与父类构造器逐字相同——`<init>(Writer)` 会同名同参误中父类构造器）、
    # static / native 方法不参与 vtable
    if m.is_constructor or m.is_static or m.is_native:
        return ''
    if not registry or ci.is_interface:
        return ''
    from ..sig_types import hierarchy_overloaded_names
    cur_mangled = m.name in hierarchy_overloaded_names(ci, registry)
    prev_return = _descriptor_return_part(m.descriptor)
    oldest: 'str | None' = None
    cur = ci.super_class
    seen: set[str] = set()
    while cur and cur not in seen and cur != _OBJECT_CLASS and cur in registry:
        seen.add(cur)
        anc = registry[cur]
        _anc_hand = ((handwritten_methods or {}).get(cur) or {}).get('methods', ())
        am = next((x for x in anc.methods
                   if not x.is_synthetic and _same_vtable_slot(x, m)), None)
        if am is not None:
            if (safe_ident(am.name) in _anc_hand
                    or safe_ident(mangle_name(am.name, am.descriptor)) in _anc_hand):
                pass  # 祖先的该方法由共置 _impl.rs 手写 → 不在其 vtable，不作槽位
            elif am.access_flags & _ACC_PRIVATE:
                pass  # 私有声明不参与覆盖解析，链继续上溯
            else:
                if cur_mangled and m.name not in hierarchy_overloaded_names(anc, registry):
                    break  # mangle 状态不一致 → 名字将不一致，停止追踪
                am_return = _descriptor_return_part(am.descriptor)
                if am_return != prev_return:
                    land_bin, land_m = _covariant_landing_slot(
                        am, anc, registry, handwritten_methods)
                    if not _slot_return_is_object(
                            registry.get(land_bin) or anc, land_m, registry):
                        break  # 落点槽位返回具体类型，无法对齐 → 停在上一级
                prev_return = am_return
                oldest = cur
        cur = anc.super_class
    return _bin_to_rust(oldest) if oldest is not None else ''


def _chain_depth(owner_rust: str, ci: 'ClassInfo', registry: 'dict | None') -> int:
    """owner_rust 在 ci 超类链上的位置（直接父类 = 0，越远越大；不在链上 = -1）。"""
    cur = ci.super_class
    seen: set[str] = {ci.name}
    depth = 0
    while cur and cur not in seen:
        seen.add(cur)
        if _bin_to_rust(cur) == owner_rust:
            return depth
        if not registry or cur not in registry:
            return -1
        cur = registry[cur].super_class
        depth += 1
    return -1


def resolve_virtual_slot(m: 'ParsedMethod', ci: 'ClassInfo',
                         registry: 'dict | None',
                         handwritten_methods: 'dict | None' = None) -> str:
    """虚方法槽位归属的完整解析。

    两条路径可以给出不同（都非本类）的归属：
      - ``_find_virtual_in``：精确描述符（同名同参同返回）的最远声明者；
      - 槽位精确签名模型（K-6a，``_covariant_virtual_owner``）：同名同参数描述符
        （返回可协变、含 bridge 计数误伤）的最远声明者。
    不一致时取超类链上**更远**的声明者：精确描述符命中即 JVMS §5.4.5 的同一
    槽位，协变模型只在精确匹配失败（bridge 改写返回）时才给出不同的（更远的）
    归属；更近的结果源于协变模型的重载 mangle 守卫在 mangle 边界提前截断。
    取更远者保证链上各类对同一方法归属同一点（覆盖传递性）。

    归属只决定 trait 槽位（vtable impl 填充位置），wrapper 名按本类重载态
    （method_name_is_mangled / receiver_member_name），两者不同名时经
    vtable_name 属性解耦（slot_member_rust_name）。"""
    slot = _find_virtual_in(m, ci, registry, handwritten_methods)
    if not slot or not registry or ci.is_interface:
        return slot
    self_rust = _bin_to_rust(ci.name)
    cov = _covariant_virtual_owner(m, ci, registry, handwritten_methods)
    if cov and cov != slot:
        if slot == self_rust:
            return cov
        return slot if _chain_depth(slot, ci, registry) >= _chain_depth(cov, ci, registry) else cov
    return cov or slot


def slot_member_rust_name(m: 'ParsedMethod', ci: 'ClassInfo',
                          registry: 'dict | None',
                          handwritten_methods: 'dict | None' = None) -> str:
    """覆盖条目对应的祖先 vtable trait 槽位成员名（槽位声明者视角下的 Rust 名）。

    槽位声明者由 resolve_virtual_slot 的归属结果定位；其 (m.name, 参数描述符) 声明
    的名字按声明者自身重载态判定（method_name_is_mangled 纯接收者态）——与该声明者
    文件里实际发射的名字同源。声明不可解析（链断裂 / 手写）时返回空串（调用方
    不解耦，维持旧形态）。"""
    from ..sig_types import method_name_is_mangled
    from ..type_map import mangle_name as _mangle
    owner_rust = resolve_virtual_slot(m, ci, registry, handwritten_methods)
    if not owner_rust or owner_rust == _bin_to_rust(ci.name):
        return ''
    cur = ci.super_class
    seen: set[str] = {ci.name}
    while cur and cur not in seen:
        seen.add(cur)
        if _bin_to_rust(cur) == owner_rust:
            decl = next((x for x in registry[cur].methods
                         if not x.is_synthetic and _same_vtable_slot(x, m)), None)
            if decl is None:
                return ''
            return (_mangle(decl.name, decl.descriptor)
                    if method_name_is_mangled(registry[cur], decl, registry)
                    else decl.name)
        if cur not in registry:
            return ''
        cur = registry[cur].super_class
    return ''


def _find_virtual_in(m: 'ParsedMethod', ci: 'ClassInfo',
                     registry: 'dict | None',
                     handwritten_methods: 'dict | None' = None) -> str:
    """确定虚方法归属的 vtable 类 Rust 名。
    返回空串表示非虚方法；返回当前类 Rust 名表示新定义；返回祖先类 Rust 名表示覆盖。

    必须找到最远祖先（原始 virtual_in），而非第一个出现的祖先：
      - A 定义 foo() → A__VTable 含 abstract fn foo()
      - B extends A 覆盖 foo() → impl A__VTable for B__inner { fn foo() }
      - C extends B 覆盖 foo() → impl A__VTable for C__inner { fn foo() }
    C.foo 的 virtual_in 应为 A（最远定义者），不是 B。
    """
    # 构造器、静态方法、native 方法不参与 vtable（私有非 native 方法仍可参与）
    if m.is_constructor or m.is_static or m.is_native:
        return ''
    # 接口 default 方法：视为新定义（放在接口自己的 vtable）
    if ci.is_interface:
        return _bin_to_rust(ci.name)

    # 沿祖先链找到最远（最上层）定义了该 name+descriptor 的非私有方法的类
    # final 方法也需要追踪，它覆盖祖先虚方法时仍属于该 vtable
    #
    # 重载 mangle 状态差异（祖先与当前类同名方法数量不同）不中止追踪：名字一致性
    # 由槽位名解耦保证——wrapper 名按本类重载态（method_name_is_mangled 纯判定 /
    # receiver_member_name），trait 槽位 impl 名按槽位声明者态，不同名时经
    # vtable_name 属性改写（与继承成员的 K-6 机制统一）。旧计数守卫会把「子类因
    # 实现接口方法新增同名重载」的覆盖（如 WhileOps$1Op.opWrapSink：覆盖
    # AbstractPipeline 槽位 + 实现 WhileOps$DropWhileOp 接口方法）误判为本类新
    # 槽位 → 祖先槽位无人覆盖，虚分派落到 trait default 的 stub panic（S-18 家族）。
    if registry:
        oldest: str | None = None
        cur = ci.super_class
        while cur and cur != _OBJECT_CLASS and cur in registry:
            anc = registry[cur]
            # 祖先的该方法由共置 _impl.rs 手写（codegen 跳过生成）→ 它不在祖先的
            # vtable trait 里，不能作为覆盖目标（否则 impl 出 trait 没有的方法，E0407）。
            _anc_hand = ((handwritten_methods or {}).get(cur) or {}).get('methods', ())
            for am in anc.methods:
                if am.name == m.name and am.descriptor == m.descriptor:
                    if (safe_ident(am.name) in _anc_hand
                            or safe_ident(mangle_name(am.name, am.descriptor)) in _anc_hand):
                        pass
                    elif not (am.access_flags & _ACC_PRIVATE):
                        oldest = cur
                    break
            cur = anc.super_class
        if oldest is not None:
            return _bin_to_rust(oldest)

    # 未在祖先中找到 → 当前类新定义
    return _bin_to_rust(ci.name)
