# 从 codegen/instr/sim.py 中拆出

from ...stack import BOOL
from ...rs_ir import Lit, RawExpr, RsNamed
from ...render import render_expr, render_type
from ...type_map import jvm_to_rust
from ..coerce import _is_subtype
from ...constants import OBJECT_CLASS as _OBJECT_CLASS


def sim_control(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── 控制流（循环由 method.py 处理，此处跳过）──
    if op.startswith('if_icmp') or op.startswith('if') or op == 'goto':
        # 未被控制流 map 捕获的分支指令：仍需弹出操作数，防止遗留值污染后续栈状态
        if op.startswith('if_icmp') or op in ('if_acmpeq', 'if_acmpne'):
            if sim.stack: sim.pop()
            if sim.stack: sim.pop()
        elif op != 'goto':
            # 单操作数：ifeq / ifne / iflt / ifge / ifgt / ifle / ifnull / ifnonnull
            if sim.stack: sim.pop()

    # ── checkcast / instanceof ──
    elif op == 'checkcast':
        # 更新栈顶类型为 cast 目标类型；若源类型为 Object，插入运行时 downcast
        if comment and sim.stack:
            if comment.startswith('['):
                cast_rust = jvm_to_rust(comment, registry)
            else:
                cast_rust = jvm_to_rust(f'L{comment};', registry)
            expr, src_ty = sim.pop()
            src_name = getattr(src_ty, 'name', str(src_ty))
            if src_name == 'Object' and cast_rust not in ('Object', '()'):
                expr = RawExpr(f"({render_expr(expr)}).downcast::<{cast_rust}>()")
            elif src_name != 'Object' and cast_rust not in ('Object', '()', src_name):
                if _is_subtype(cast_rust.split('<')[0], src_name.split('<')[0], registry):
                    # 合法向下转型（源静态类型是目标的父类，如 Node → TreeNode，E0282）：
                    # 经 Object::from_any 保留运行时值再 downcast 恢复子类型，
                    # 不能用 Default::default() 占位（会丢失接收者类型导致无法推断）
                    expr = RawExpr(f"Object::from_any({render_expr(expr)}).downcast::<{cast_rust}>()")
                else:
                    # 二次 checkcast（非 Object 源类型）：两种具体类型不兼容，用 Default::default() 占位
                    expr = RawExpr('Default::default()')
            sim.push(expr, RsNamed(cast_rust))
    elif op == 'instanceof':
        # JVM 语义: pop objectref, push int(0/1)
        # 通过 registry 继承链做静态类型分析：
        #   obj 静态类型 IS-A target  → true（子类一定是父类）
        #   obj 静态类型 == target    → true
        #   否则（obj 是 target 超类，或无继承关系）→ false
        #   值语义下 Dog.into()→Animal 后类型信息已丢失，超类变量对子类 instanceof 为 false
        val_expr_inst, val_ty_inst = sim.pop() if sim.stack else (None, None)
        if comment and val_ty_inst is not None:
            if comment.startswith('['):
                target_rust = jvm_to_rust(comment, registry)
            else:
                target_rust = jvm_to_rust(f'L{comment};', registry)
            # jvm_to_rust 对接口返回 'Object'；instanceof 子类型判断需要接口的实际 Rust 短名
            # 用二进制名末段（去路径后 $ → _）还原接口 Rust 短名，供 _is_subtype 正确匹配
            if target_rust == 'Object' and not comment.startswith('[') and comment != _OBJECT_CLASS:
                _last = comment.rsplit('/', 1)[-1]
                target_for_subtype = _last.replace('$', '_')
            else:
                target_for_subtype = target_rust
            obj_ty_str = render_type(val_ty_inst)
            val_s_inst = render_expr(val_expr_inst)
            if obj_ty_str == 'Object':
                # 运行时多态：通过 ObjectVTable fn 指针（Arch-2）检查类型继承链
                # comment 本身就是 JVM 二进制名（如 java/util/List）
                sim.push(RawExpr(f"({val_s_inst}.is_instance_of(\"{comment}\"))"), BOOL)
            elif obj_ty_str == target_for_subtype:
                sim.push(Lit('true'), BOOL)
            elif _is_subtype(obj_ty_str.split('<')[0], target_for_subtype.split('<')[0], registry):
                sim.push(Lit('true'), BOOL)
            else:
                sim.push(Lit('false'), BOOL)
        else:
            sim.push(Lit('false'), BOOL)
    else:
        return False
    return True
