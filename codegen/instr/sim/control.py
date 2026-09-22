# 从 codegen/instr/sim.py 中拆出

from ...type_map import short_cls as _short_cls_g
from ...stack import BOOL
from ...rs_ir import CastExpr, InstanceOfExpr, Lit, RawExpr, RsNamed
from ...render import render_expr, render_type
from ...type_map import jvm_to_rust, short_cls, effective_class_type_params as _effective_class_type_params
from ..coerce import _coerce_to_object
from ..hierarchy import _is_subtype, _rust_type_to_binary
from ...constants import OBJECT_CLASS as _OBJECT_CLASS


def _erased_shape(rust_ty: str) -> str:
    """去掉类类型实参后的形态（数组保留元素形态）：`JArray<Entry<K, V>>` → `JArray<Entry>`。"""
    import re
    prev = None
    while prev != rust_ty:
        prev = rust_ty
        rust_ty = re.sub(r'\b(?!JArray\b)(\w+)<[^<>]*>', r'\1', rust_ty)
    return rust_ty


def sim_control(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # 跳转指令（if* / goto / switch）由 method/blocks.py 作为块终结解释，不经过本分发

    # ── checkcast / instanceof ──
    if op == 'checkcast':
        # 更新栈顶类型为 cast 目标类型；若源类型为 Object，插入运行时 checkcast。
        # 发射一律为 CastExpr 节点（A-3）：checked 形态失败返回 Err(JvmError::class_cast)
        # 可被 java_try 捕获（S-1），消费方按节点分派（不再匹配 downcast 字符串）。
        if comment and sim.stack:
            if comment.startswith('['):
                cast_rust = jvm_to_rust(comment, registry)
            else:
                cast_rust = jvm_to_rust(f'L{comment};', registry)
            # 目标类与当前类共享类型变量作用域（内部类沿用外层类的类型变量，其有效类型形参
            # 全部是当前类的类型形参）→ 擦除实例化按目标类自身的形参顺序还原为 <K, V>；
            # 显式类型参数不会像 _ 那样在无标注赋值处引发 E0283
            if '<' in cast_rust and sim.class_type_params and registry:
                _cast_elem = comment.lstrip('[')
                if _cast_elem != comment:
                    _cast_elem = _cast_elem[1:-1] if _cast_elem.startswith('L') else ''
                _cast_ci = registry.get(_cast_elem)
                _cast_tps = _effective_class_type_params(_cast_ci, registry) if _cast_ci else []
                from ...jvm_type import carrier_type as _cast_carrier
                _cast_is_carrier = (_cast_ci is not None and _cast_ci.is_interface
                                    and _cast_carrier(_cast_elem, registry) is not None)
                if (_cast_tps and not _cast_is_carrier
                        and all(_tp in sim.class_type_params for _tp in _cast_tps)):
                    # 接口载体不参与共享类型变量的实例化还原：载体是擦除运行时形态
                    # （I<Object> 即 itable 视图），按 T 重新实例化会得到非法类型
                    _erased = f"{short_cls(_cast_elem)}<{', '.join(['Object'] * len(_cast_tps))}>"
                    cast_rust = cast_rust.replace(_erased, f"{short_cls(_cast_elem)}<{', '.join(_cast_tps)}>")
            expr, src_ty = sim.pop()
            src_name = getattr(src_ty, 'name', str(src_ty))
            # A-4 批次 1：checkcast 到接口（目标 jvm_to_rust 擦除为 Object）。此前
            # 静默丢弃（cast_rust=='Object' 落到通用 else）→ 跨接口强转的 CCE 空档；
            # 现发射 interface_target 形态的 CastExpr（try_cast_iface：null 通过 /
            # is_instance_of 按运行时类接口闭包判定 / 失败 Err 可捕获，S-1），
            # 栈类型保持擦除记录 Object——载体进类型位置后翻转为目标载体形态。
            # 批次 3+：已铺设载体化的接口不再走本分支——cast_rust 已是载体
            # `I<Object>`，下方通用 checked 臂发射 try_cast::<I<Object>>（null 还原
            # + downcast 快路径 + is_instance_of 名单，命中经 From<Object> 包装），
            # 栈类型记录为载体。
            _tgt_ci = registry.get(comment) if (registry and not comment.startswith('[')) else None
            from ...jvm_type import carrier_type as _carrier_type
            if (src_name == 'Object' and _tgt_ci is not None and _tgt_ci.is_interface
                    and _carrier_type(comment, registry) is None):
                expr = CastExpr(expr, 'Object', binary_name=comment, checked=True,
                                interface_target=True)
                sim.push(expr, RsNamed('Object'))
                return True
            if src_name == 'Object' and cast_rust not in ('Object', '()'):
                # 源是 Object（擦除边界）：checkcast 语义（null 还原 / `__view_into` 视图 /
                # is_instance_of + 擦除部件重建由 try_cast 按序判定，数组目标按元素类型驱动）
                expr = CastExpr(expr, cast_rust, binary_name=comment, checked=True)
            elif (src_name != 'Object' and cast_rust not in ('Object', '()', src_name)
                  and _erased_shape(src_name) == _erased_shape(cast_rust)):
                # 同一擦除类型、仅类型实参不同（`(Entry<K,V>[]) new Entry<?,?>[n]`）：JVM 上类型实参
                # 不参与 checkcast，值不变；源侧待推断的 `_` 由目标类型给出
                import re as _re_cast
                if '_' in _re_cast.findall(r'\w+', src_name):
                    sim.push(expr, RsNamed(cast_rust))
                else:
                    sim.push(expr, src_ty)
                return True
            elif (src_name != 'Object' and cast_rust not in ('Object', '()')
                  and (sim.type_var_bounds.get(src_name) or '').split('<')[0].strip()
                  == cast_rust.split('<')[0].strip()):
                # 类型变量值转型到其上界的擦除类（`K c = task.makeChild(..)`，javac 按 K 的擦除
                # 补 checkcast）：恒成立，值与静态类型都不变
                sim.push(expr, src_ty)
                return True
            elif src_name != 'Object' and cast_rust not in ('Object', '()', src_name):
                if _is_subtype(cast_rust.split('<')[0], src_name.split('<')[0], registry):
                    # 合法向下转型（源静态类型是目标的父类，如 Node → TreeNode）：checkcast 语义 ——
                    # 经 Object 边界（保持对象标识与运行时类）按目标类取回子类视图，null 原样通过；
                    # 失败（运行时类不是目标族）抛 ClassCastException（S-1：Err 而非 panic）
                    expr = CastExpr(expr, cast_rust, binary_name=comment, checked=True, box_first=True)
                else:
                    # 静态类型互不为子类型（擦除泛型数组 `(E[][]) Arrays.copyOf(..)`、交叉转型）：
                    # checkcast 是运行时校验 → 经 Object 边界按目标类型取回
                    _boxed = _coerce_to_object(render_expr(expr), src_name, registry,
                                               sim.class_type_params)
                    expr = CastExpr(RawExpr(_boxed), cast_rust, binary_name=comment, checked=True)
            if cast_rust == 'Object' and src_name not in ('Object', '()'):
                # 目标擦除为 Object（接口 / 根类）而值有更精确的静态类型（类型变量 T_NODE、
                # 具体类）：向上转型不改变值，表达式的 Rust 类型仍是源类型 → 记录源类型，
                # 由使用点按真实类型转换（记成 Object 会让使用点漏掉装箱，E0277）
                # 例外：源是具体类且静态上并未实现目标接口（交叉转型，运行时子类才实现，
                # `(DirectBuffer) byteBuffer`）——接口视图只能经对象身份取得 → 装箱为 Object
                _src_base = src_name.split('<')[0]
                _tgt_short = _short_cls_g(comment)
                if (comment != _OBJECT_CLASS and not comment.startswith('[')
                        and _rust_type_to_binary(_src_base, registry)
                        and _src_base != _tgt_short
                        and not _is_subtype(_src_base, _tgt_short, registry)):
                    _se = render_expr(expr)
                    if not _se.startswith('Clone::clone('):
                        _se = f"Clone::clone(&{_se})"
                    sim.push(RawExpr(f"Into::<Object>::into({_se})"), RsNamed('Object'))
                else:
                    sim.push(expr, src_ty)
            else:
                sim.push(expr, RsNamed(cast_rust))
    elif op == 'instanceof':
        # JVM 语义: pop objectref, push int(0/1)
        # 静态类型分析（registry 继承链）：
        #   obj 静态类型 IS-A target  → true（子类一定是父类，编译期可证）
        #   obj 静态类型 == target    → true
        #   obj 静态类型是 target 的超类（Animal 变量 instanceof Dog）→ 运行时判定：
        #     装箱为 Object（超类包装持有具体子类的 vtable），ObjectVTable::is_instance_of
        #     按 all_supertypes 匹配 binary name（宏静态展开 patterns，含类自身）
        #   其余（互不为子类型）→ 编译期 false，计入 instanceof_fold 审计
        # 槽位定型配套：同一 slot 在兄弟分支赋不同引用类型时按公共祖先 widening
        # （vars.py 的合并 pass），运行时化复活的分支（HashMap.putVal 的
        # `p instanceof TreeNode`）才能与首赋值类型共存。
        # 运行时判定统一发射 InstanceOfExpr 节点（A-3），经擦除类（vtable 静态
        # 超类型名单）判定，消费方按节点分派。
        val_expr_inst, val_ty_inst = sim.pop() if sim.stack else (None, None)
        if comment and val_ty_inst is not None:
            if comment.startswith('['):
                target_rust = jvm_to_rust(comment, registry)
            else:
                target_rust = jvm_to_rust(f'L{comment};', registry)
            # jvm_to_rust 对接口返回 'Object'；instanceof 子类型判断需要接口的实际 Rust 短名
            # 用二进制名末段（去路径后 $ → _）还原接口 Rust 短名，供 _is_subtype 正确匹配
            if target_rust == 'Object' and not comment.startswith('[') and comment != _OBJECT_CLASS:
                target_for_subtype = _short_cls_g(comment)
            else:
                target_for_subtype = target_rust
            obj_ty_str = render_type(val_ty_inst)
            if obj_ty_str == 'Object':
                # 运行时多态：通过 ObjectVTable fn 指针（Arch-2）检查类型继承链
                # comment 本身就是 JVM 二进制名（如 java/util/List）
                sim.push(InstanceOfExpr(val_expr_inst, comment), BOOL)
            elif obj_ty_str == target_for_subtype:
                sim.push(Lit('true'), BOOL)
            elif _is_subtype(obj_ty_str.split('<')[0], target_for_subtype.split('<')[0], registry):
                sim.push(Lit('true'), BOOL)
            elif _is_subtype(target_for_subtype.split('<')[0], obj_ty_str.split('<')[0], registry):
                # obj 静态类型是 target 的超类：装箱后按运行时类判定（is_instance_of 按
                # vtable 的 all_supertypes 匹配 binary name，含类自身）
                val_s_inst = render_expr(val_expr_inst)
                _boxed_inst = _coerce_to_object(val_s_inst, obj_ty_str.split('<')[0], registry,
                                                sim.class_type_params)
                sim.push(InstanceOfExpr(RawExpr(_boxed_inst), comment), BOOL)
            else:
                from ...cfg import STATS as _STATS_INST
                _STATS_INST.record_instanceof_fold()
                sim.push(Lit('false'), BOOL)
        else:
            from ...cfg import STATS as _STATS_INST
            _STATS_INST.record_instanceof_fold()
            sim.push(Lit('false'), BOOL)
    else:
        return False
    return True
