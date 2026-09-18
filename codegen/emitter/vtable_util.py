# 从 codegen/emitter/class_writer.py 中拆出

from ..constants import safe_ident, OBJECT_CLASS as _OBJECT_CLASS
from ..type_map import mangle_name

_ACC_PRIVATE = 0x0002


def _bin_to_rust(binary_name: str) -> str:
    """将 JVM binary 名（含 / 和 $）转为 Rust 类型名。"""
    return binary_name.rsplit('/', 1)[-1].replace('$', '_')


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
    # 额外约束：若祖先与当前类的同名方法数量不同（mangle 状态不同），Rust 方法名将不一致，
    # 不能继续追踪（否则 vtable trait 声明与 impl 的方法名不同 → E0407）。
    if registry:
        # 当前类中 m.name 的同名非构造/非静态方法数
        cur_overloaded = sum(
            1 for cm in ci.methods
            if cm.name == m.name and not cm.is_constructor and not cm.is_static
        ) > 1

        oldest: str | None = None
        cur = ci.super_class
        while cur and cur != _OBJECT_CLASS and cur in registry:
            anc = registry[cur]
            found = False
            # 祖先的该方法由共置 _impl.rs 手写（codegen 跳过生成）→ 它不在祖先的
            # vtable trait 里，不能作为覆盖目标（否则 impl 出 trait 没有的方法，E0407）。
            _anc_hand = ((handwritten_methods or {}).get(cur) or {}).get('methods', ())
            for am in anc.methods:
                if am.name == m.name and am.descriptor == m.descriptor:
                    if (safe_ident(am.name) in _anc_hand
                            or safe_ident(mangle_name(am.name, am.descriptor)) in _anc_hand):
                        pass
                    elif not (am.access_flags & _ACC_PRIVATE):
                        anc_overloaded = sum(
                            1 for xm in anc.methods
                            if xm.name == m.name and not xm.is_constructor and not xm.is_static
                        ) > 1
                        if cur_overloaded != anc_overloaded:
                            # mangle 状态不同 → Rust 方法名不一致 → 停止追踪
                            cur = None
                        else:
                            oldest = cur
                    found = True
                    break
            if cur is None:
                break
            cur = anc.super_class
        if oldest is not None:
            return _bin_to_rust(oldest)

    # 未在祖先中找到 → 当前类新定义
    return _bin_to_rust(ci.name)
