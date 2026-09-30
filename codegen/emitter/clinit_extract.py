"""static 字段声明与 <clinit> 方法块生成（从 class_writer._gen_class_rs 迁出）。

前身（p0 计划的 _extract_clinit_consts/_extract_clinit_arrays/<clinit> 常量扫描）
已随 2ca1d4a「<clinit> 终态类初始化语义」删除——<clinit> 改为整段字节码翻译，
常量初值回归 ConstantValue 属性。本模块承接现役两段：
- _gen_static_field_blocks：static 字段声明。闭包变量 class_type_params /
  _type_only / _nf_entry / _registry_short_names 改为显式参数；
  method_blocks 闭包追加改为返回块列表。
- _gen_clinit_block：<clinit> → `fn __clinit()` 方法块（完整翻译，失败回退
  panic 存根）。闭包变量改为显式参数；原 continue 控制流改为返回
  None（跳过）/ str（追加）。
函数体逻辑零改动（迁移后统一去一层缩进）。
依赖方向 class_writer → clinit_extract → field_gen 单向。
"""

import re as _re

from ..method import gen_method_body
from .. import fallback_audit as _fallback_audit

from ..type_map import jvm_to_rust
from ..sig_parse import parse_field_type
from ..constants import safe_ident, PRIMITIVE_RUST_TYPES
from .attrs import _java_field_attr, _java_method_attr
from .field_gen import _validate_field_type

_safe_field_name = safe_ident

# <clinit> 翻译函数在 java_class! 块内的固定名字（与宏 block/class_init.rs 的约定一致）
_CLINIT_FN = '__clinit'


def _gen_static_field_blocks(ci, registry, class_type_params: list,
                             _type_only: bool, _nf_entry: dict | None,
                             _registry_short_names: set[str]) -> list[str]:
    """static 字段声明（JVMS §5.5 类初始化的事实层）。

    codegen 只声明事实，存储 / 访问器 / 初始化触发全部由 java_class! 宏展开：
      ConstantValue 属性 → `pub const NAME: T = 值;`（编译期常量，访问不触发初始化）
      其余 static 字段   → `pub static NAME: T;`（初值由 <clinit> 字节码翻译写入）
    类型存根（没有任何方法在调用链上：内部边界类 / 仅签名引用的类）不会被初始化：
    未被共置手写文件覆盖的 static 字段保持 panic 存根，命中时精确报出字段。"""
    blocks: list[str] = []
    static_fields = [f for f in ci.fields if f.is_static]
    existing_method_names: set[str] = {m.name for m in ci.methods}
    _nf_covered_sf = (_nf_entry or {}).get('methods', set())
    _nf_cores = (_nf_entry or {}).get('method_cores', {}) or {}
    for sf in static_fields:
        # 字段名与方法名冲突：改用 `<原名>_field` 后缀再做标识符转义（读写侧 fields.py 同规则）
        safe_fname = _safe_field_name(sf.name + '_field' if sf.name in existing_method_names else sf.name)
        # 优先用 generic_signature 确定字段类型（包含泛型参数信息）
        if sf.generic_signature:
            _gs_ret = parse_field_type(sf.generic_signature, class_type_params, registry)
            # 校验引用的类型存在，否则回退到描述符
            if (_gs_ret and _gs_ret != 'Object'
                    and not _validate_field_type(_gs_ret, class_type_params,
                                                 _registry_short_names)):
                _gs_ret = ''
            # static 字段不在类型参数作用域内（Java 同样禁止），引用类型变量时回退描述符
            if _gs_ret and any(_tp in _re.findall(r'[A-Za-z_][A-Za-z0-9_]*', _gs_ret)
                               for _tp in class_type_params):
                _gs_ret = ''
        else:
            _gs_ret = ''
        rust_ret = _gs_ret if _gs_ret else jvm_to_rust(sf.descriptor, registry=registry)
        field_meta = _java_field_attr(sf)
        cv = sf.constant_value
        if cv:
            if rust_ret == 'String':
                body = f'String::from("{cv}")'
            elif rust_ret == 'f32':
                if cv == 'inf':      body = 'f32::INFINITY'
                elif cv == '-inf':   body = 'f32::NEG_INFINITY'
                elif cv == 'NaN':    body = 'f32::NAN'
                else:                body = f'{cv}f32'
            elif rust_ret == 'f64':
                if cv == 'inf':      body = 'f64::INFINITY'
                elif cv == '-inf':   body = 'f64::NEG_INFINITY'
                elif cv == 'NaN':    body = 'f64::NAN'
                else:                body = f'{cv}f64'
            elif rust_ret == 'i64':
                body = f'{cv}i64'
            elif rust_ret == 'bool':
                body = 'true' if cv == '1' else 'false'
            else:
                body = cv
            blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub const {safe_fname}: {rust_ret} = {body};')
        elif _type_only:
            if safe_fname in _nf_covered_sf:
                continue
            _sf_core = _nf_cores.get(safe_fname)
            if _sf_core is not None:
                # 静态字段伴生核心（`core_<字段名>()`，无接收者）：内部边界类的常量
                # 字段 <clinit> 不翻译，值由手写层给出；字段类型随 JDK 演化时
                # （Unsafe.ARRAY_INT_BASE_OFFSET：JDK21 `I` → JDK25 `J`）核心按当前
                # JDK 形态书写，此处按**当前模型类型**发 getter 转发并显式 as 还原宽度
                #（与实例方法 core_ 适配同一约定）。setter 保持存根（final 常量）。
                _core_name, _core_ret = _sf_core
                _cm = _re.match(r'^Result<(.*)>$', _core_ret or '')
                _cr = _cm.group(1) if _cm else None
                _cast = (f' as {rust_ret}' if (_cr and _cr != rust_ret
                         and _cr in PRIMITIVE_RUST_TYPES and rust_ret in PRIMITIVE_RUST_TYPES)
                         else '')
                blocks.append(
                    f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                    f'pub fn {safe_fname}() -> Result<{rust_ret}> {{\n'
                    f'    Ok(Self::{_core_name}()?{_cast})\n}}\n'
                    f'pub fn set_{safe_fname}(v: {rust_ret}) -> Result<()> {{\n'
                    f'    panic!("stub-set: {ci.name}.{sf.name}:{sf.descriptor}")\n}}')
                continue
            blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub fn {safe_fname}() -> Result<{rust_ret}> {{\n'
                f'    panic!("stub: {ci.name}.{sf.name}:{sf.descriptor}")\n}}\n'
                f'pub fn set_{safe_fname}(v: {rust_ret}) -> Result<()> {{\n'
                f'    panic!("stub-set: {ci.name}.{sf.name}:{sf.descriptor}")\n}}')
        else:
            blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub static {safe_fname}: {rust_ret};')
    return blocks


def _gen_clinit_block(m, ci, registry: dict | None, class_type_params: list,
                      overloaded_names: set, call_chain: set | None,
                      _type_only: bool) -> 'str | None':
    """<clinit> → `fn __clinit()`：宏生成的 __class_init() 状态机在首次主动使用时调用它。
    类型存根不会被初始化；不在调用链上的 <clinit> 与其它方法同规则生成 panic 存根
    （内部边界类例外，见下方分支——初始化为手写层职责）。
    返回 None 表示跳过不生成；返回 str 为待追加的方法块（含 @java_method 属性行）。"""
    if _type_only:
        return None
    from ..callchain import _is_vm_boundary_class
    if _is_vm_boundary_class(ci.name):
        # VM 契约边界类的 <clinit> 不翻译（按方法划分的类初始化口径：静态状态由 VM / 手写层
        # 承载）；不生成 → 宏 has_clinit=false → __class_init() 为 no-op
        return None
    attr_line = _java_method_attr(m)
    _clinit_stub = (f'pub fn {_CLINIT_FN}() -> Result<()> {{\n'
                    f'    panic!("stub: {ci.name}.<clinit>:()V")\n}}')
    if call_chain is not None and (ci.name, m.name, m.descriptor) not in call_chain:
        return attr_line + '\n' + _clinit_stub
    try:
        clinit_body = gen_method_body(
            m, ci, registry=registry,
            class_type_params=class_type_params,
            overloaded_names=overloaded_names,
            rust_name=_CLINIT_FN,
        )
        return attr_line + '\n' + clinit_body
    except _fallback_audit.fallback_exc() as e:
        # A 组白名单兜底（fallback-audit 方案 §4.1）九点之一：只兜 CfgError
        # 家族，代码 bug（CfgAuditError / ImportError / NameError / …）穿透
        _fallback_audit.stub_fallback(
            f"{ci.name}.{m.name}:{m.descriptor}", 'clinit', e)
        return attr_line + '\n' + _clinit_stub
