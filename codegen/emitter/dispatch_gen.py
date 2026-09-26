"""L3 反射分派闭包发射（Method.invoke / Constructor.newInstance 的按名协议）。

协议（见 runtime/java_runtime/src/reflect_dispatch.rs 头注——架构定稿）：
codegen 为**用户树全部（非泛型、非接口）类**在类文件尾部发射

    impl <Class> {
        pub fn __reflect_dispatch(name: &str, descriptor: &str,
                                  recv: Object, args: &JArray<Object>)
            -> Option<Result<Object>> {
            match (name, descriptor) {
                ("<init>", "()V") => Some((|| Self::new().map(Object::from))()),
                ("m", "(I)V") => Some((|| { ...; Ok(Object::default()) })()),
                ...
                _ => None,   // 本类未声明（上溯继续）
            }
        }
    }

臂的调用形态按成员类别：
  - `<init>` → `Self::new*(..)`（构造器系列，结果装箱）；
  - static → `Self::m(..)`（命名空间函数直调）；
  - 实例方法 → `recv.try_cast::<Self>(binary)` 取视图后调用——视图共享运行时
    vtable，子类覆盖自动按虚分派生效（与 JVM invoke 的虚语义一致）。

实参/返回值 marshalling 按发射签名的 Rust 类型：基本类型经
reflect_dispatch::unbox_* 双路径拆箱（原生盒 / 翻译包装）；引用类型经
try_cast（String 经 valueOf_obj）；Object 直传；数组/复杂形态不承载（臂体
panic stub——如实定性）。生成 main 启动时登记（register_method_dispatch，
与类初始化钩子 / 注解工厂同一登记模式）。

数据源：类发射文本的 java_method / java_native 属性行 + 其后 pub fn 签名行
（与 build.rs 方法表扫描同一属性协议——零二次推导）。
"""

import re

# 已发射分派闭包的类（{binary: 登记行}）。与 SAM_LEDGER 同生命周期（每轮 reset）。
LEDGER: dict[str, str] = {}

_ATTR_RE = re.compile(r'#\[java_(?:method|native)\(')
_FN_RE = re.compile(r'^\s*pub fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*(.+?))?\s*[;{]')

_PRIM_UNBOX = {
    'i64': ('unbox_i64', ''),
    'i32': ('unbox_i32', ''),
    'bool': ('unbox_bool', ''),
    'f64': ('unbox_f64', ''),
    'f32': ('unbox_f32', ''),
    'i16': ('unbox_i32', ' as i16'),
    'i8': ('unbox_i32', ' as i8'),
    'u16': ('unbox_char', ''),
}


def reset() -> None:
    LEDGER.clear()
    FIELD_LEDGER.clear()


def _extract(window: str, key: str) -> 'str | None':
    m = re.search(rf'\b{re.escape(key)}\s*=\s*"([^"]*)"', window)
    return m.group(1) if m else None


def _flag(window: str, key: str) -> bool:
    return re.search(rf'\b{re.escape(key)}\s*=\s*true', window) is not None


def _param_bin(descriptor: str, index: int) -> 'str | None':
    """描述符第 index 个参数的二进制名（L..; / 数组原样；基本类型 None）。"""
    from ..type_map import parse_descriptor_params
    try:
        p = parse_descriptor_params(descriptor)[index]
    except (ValueError, IndexError):
        return None
    if p.startswith('L') and p.endswith(';'):
        return p[1:-1]
    if p.startswith('['):
        return p
    return None


def _split_params(sig_params: str) -> list:
    parts, depth, cur = [], 0, []
    for ch in sig_params:
        if ch in '<([':
            depth += 1
        elif ch in '>)]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(''.join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    if ''.join(cur).strip():
        parts.append(''.join(cur).strip())
    return parts


def _emit_for(class_bin: str, short: str, em, only: 'set[str] | None' = None) -> 'str | None':
    """类发射文本 → __reflect_dispatch 实现（无方法 / 泛型 / 接口 → None）。

    only：只发射这些方法名的臂（JDK 类的常量反射引用面，MH-native）；None = 全部。"""
    lines = em.text.split('\n')
    # 泛型类不承载（try_cast::<Self> 的擦除视图还原未定案）
    if re.search(rf'pub struct {re.escape(short)}<', em.text):
        return None
    rt = 'crate' if getattr(em, 'crate_name', '') == 'java_runtime' else 'java_runtime'
    disp = f'{rt}::reflect_dispatch'

    arms: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if not _ATTR_RE.search(line):
            i += 1
            continue
        # 找后续最近的 pub fn 签名行
        j = i + 1
        fn_m = None
        while j < len(lines) and j <= i + 4:
            fn_m = _FN_RE.match(lines[j])
            if fn_m:
                break
            if _ATTR_RE.search(lines[j]):
                fn_m = None
                break
            j += 1
        i += 1
        if fn_m is None:
            continue
        mname = _extract(line, 'name')
        descriptor = _extract(line, 'descriptor')
        if not mname or not descriptor:
            continue
        if only is not None and mname not in only:
            continue
        is_static = _flag(line, 'is_static')
        rust_name, params, ret = fn_m.group(1), fn_m.group(2), fn_m.group(3) or '()'
        ret = ret.strip()
        params = [p for p in _split_params(params) if p not in ('&self', 'self', 'mut self', '&mut self')]

        # 形参名与类型解构
        pdefs = []
        for p in params:
            name, _, ty = p.partition(':')
            pdefs.append((name.strip().removeprefix('mut ').strip(), ty.strip()))

        # 实参 marshalling 表达式（None = 形态不承载）
        arg_exprs: list['str | None'] = []
        for idx, (_pn, ty) in enumerate(pdefs):
            if ty == 'Object':
                arg_exprs.append(f'args.get({idx})?')
            elif ty == 'String':
                arg_exprs.append(f'String::valueOf_obj(args.get({idx})?)?')
            elif ty in _PRIM_UNBOX:
                fn, cast = _PRIM_UNBOX[ty]
                # unbox 双路径返回 Option——错型实参按 IllegalArgumentException
                # 语义（JDK 对包装类型不符的反射实参同型）
                arg_exprs.append(
                    f'({disp}::{fn}(&args.get({idx})?)'
                    f'.ok_or_else({disp}::bad_arg)?{cast})')
            elif ty.startswith('JArray') or '<' in ty:
                # 数组 / 泛型载体（接口 `List<E>` 等）：Object → 载体的 From 转换
                # （数组元素类型、接口视图由运行时对象承载，与字节码 checkcast 同语义）
                arg_exprs.append(f'<{ty} as ::std::convert::From<Object>>::from(args.get({idx})?)')
            else:
                pbin = _param_bin(descriptor, idx)
                if pbin and not pbin.startswith('['):
                    arg_exprs.append(
                        f'args.get({idx})?.try_cast::<{ty}>("{pbin}")?')
                else:
                    arg_exprs.append(None)

        # 返回值装箱（None = 形态不承载）——按 Result<内层> 统一处理
        inner = ret[len('Result<'):-1] if ret.startswith('Result<') and ret.endswith('>') else None
        if inner == '()':
            ret_box = 'Ok(Object::default())'
        elif inner == 'Object':
            ret_box = 'Ok(__v)'
        elif inner in _PRIM_UNBOX or inner in ('String',):
            ret_box = 'Ok(Object::from(__v))'
        elif inner and inner[0].isupper() and '&' not in inner:
            # 类 / 接口载体（含泛型实参形态 `List<Object>`）：载体与 Object 双向互转
            ret_box = 'Ok(Object::from(__v))'
        elif inner and inner.startswith('JArray'):
            ret_box = 'Ok(Object::from(__v))'
        else:
            ret_box = None

        unsupported = (None in arg_exprs) or ret_box is None

        call = ', '.join(f'{{{n}}}' for n, _t in pdefs) if pdefs else ''
        if unsupported:
            body = (f'panic!("stub: L3 分派未支持的签名形态 {class_bin}.{mname}:{descriptor}")')
            arms.append(f'        ({mname!r}, {descriptor!r}) => Some({body}),'.replace("'", '"'))
            continue

        if mname == '<init>':
            call_expr = f'Self::{rust_name}({call})?'
            tail = f'Ok(Object::from(__r))'
            inner_body = f'let __r = {call_expr}; {tail}'
        elif is_static:
            # void 静态方法同样须 `?` 传播被调方法的异常（缺失即静默吞失——
            # Method.invoke 的 InvocationTargetException 包装面拿不到异常）
            inner_body = f'Self::{rust_name}({call})?; Ok(Object::default())' if inner == '()' \
                else f'let __v = Self::{rust_name}({call})?; {ret_box}'
        else:
            this = f'recv.try_cast::<Self>("{class_bin}")?'
            if inner == '()':
                inner_body = f'{this}.{rust_name}({call})?; Ok(Object::default())'
            else:
                inner_body = f'let __v = {this}.{rust_name}({call})?; {ret_box}'

        # 实参表达式回填
        for (pn, _ty), expr in zip(pdefs, arg_exprs):
            inner_body = inner_body.replace('{' + pn + '}', expr)
        arms.append(f'        ("{mname}", "{descriptor}") => '
                    f'Some((|| {{ {inner_body} }})()),')

    if not arms:
        return None
    out = []
    out.append('')
    out.append('// ── L3 反射分派闭包（Method.invoke / Constructor.newInstance 的按名协议；')
    out.append('//    协议与上溯语义见 java_runtime::reflect_dispatch 头注）──')
    out.append('#[allow(unused_variables, unreachable_patterns)]')
    out.append(f'impl {short} {{')
    out.append('    pub fn __reflect_dispatch(')
    out.append('        name: &str, descriptor: &str,')
    out.append('        recv: Object, args: &JArray<Object>,')
    out.append('    ) -> Option<Result<Object>> {')
    out.append('        match (name, descriptor) {')
    out.extend(arms)
    out.append('            _ => None,')
    out.append('        }')
    out.append('    }')
    out.append('}')
    return '\n'.join(out)


# java_field 属性之后的 Rust 声明行：实例字段 / 静态字段 / 常量
_FIELD_DECL_RE = re.compile(
    r'^\s*pub\s+(?:(static)\s+|(const)\s+)?(\w+)\s*:\s*([^=;,]+?)\s*(?:=[^;]*)?[;,]\s*$')
_FIELD_ATTR_RE = re.compile(r'java_field\(')


def _emit_fields_for(class_bin: str, short: str, em) -> 'str | None':
    """类发射文本 → __reflect_field 实现（Field.get/set 与 MH 字段句柄的按名协议）。

    臂形态：`("字段名", 值) =>`，值 None = 读、Some(v) = 写。实例字段经宏生成的
    `__get_<f>()` / `__set_<f>(v)`（接收者 try_cast 为声明类视图），静态字段经
    `Self::<f>()` / `Self::set_<f>(v)`，编译期常量只读。泛型类的实例字段不承载
    （擦除视图还原未定案，与 __reflect_dispatch 同一边界）。"""
    lines = em.text.split('\n')
    generic = re.search(rf'pub struct {re.escape(short)}<', em.text) is not None
    rt = 'crate' if getattr(em, 'crate_name', '') == 'java_runtime' else 'java_runtime'
    disp = f'{rt}::reflect_dispatch'
    arms: list[str] = []
    for i, line in enumerate(lines):
        if not _FIELD_ATTR_RE.search(line):
            continue
        fname = _extract(line, 'name')
        fdesc = _extract(line, 'descriptor')
        if not fname or not fdesc:
            continue
        decl = None
        for j in range(i + 1, min(i + 4, len(lines))):
            if lines[j].strip().startswith('//'):
                continue
            decl = _FIELD_DECL_RE.match(lines[j])
            break
        if decl is None:
            continue
        is_static_kw, is_const, rname, rty = decl.group(1), decl.group(2), decl.group(3), decl.group(4).strip()
        is_static = bool(is_static_kw or is_const or _flag(line, 'is_static'))
        if generic and not is_static:
            continue
        if is_const:
            read = f'Object::from(Clone::clone(&Self::{rname}))'
        elif is_static:
            read = f'Object::from(Self::{rname}()?)'
        else:
            read = f'Object::from(recv.try_cast::<Self>("{class_bin}")?.__get_{rname}())'
        if rty in _PRIM_UNBOX:
            fn, cast = _PRIM_UNBOX[rty]
            unbox = f'({disp}::{fn}(&v).ok_or_else({disp}::bad_arg)?{cast})'
        elif rty == 'Object':
            unbox = 'v'
        else:
            unbox = f'<{rty} as ::std::convert::From<Object>>::from(v)'
        arms.append(f'            ("{fname}", None) => Some((|| {{ Ok({read}) }})()),')
        if is_const:
            arms.append(f'            ("{fname}", Some(_)) => Some(Err({disp}::final_field("{fname}"))),')
        elif is_static:
            arms.append(f'            ("{fname}", Some(v)) => Some((|| {{ Self::set_{rname}({unbox})?; Ok(Object::default()) }})()),')
        else:
            arms.append(f'            ("{fname}", Some(v)) => Some((|| {{ recv.try_cast::<Self>("{class_bin}")?.__set_{rname}({unbox}); Ok(Object::default()) }})()),')
    if not arms:
        return None
    out = ['', '// ── L3 反射字段闭包（Field.get/set 与 MH 字段句柄的按名协议）──',
           '#[allow(unused_variables, unreachable_patterns, unused_mut)]',
           f'impl {short} {{',
           '    pub fn __reflect_field(',
           '        name: &str, recv: Object, value: Option<Object>,',
           '    ) -> Option<Result<Object>> {',
           '        match (name, value) {']
    out.extend(arms)
    out += ['            _ => None,', '        }', '    }', '}']
    return '\n'.join(out)


FIELD_LEDGER: dict[str, str] = {}


def synthesize(emissions: dict, registry: dict, user_bins: 'set[str]',
               reflect_members: 'dict[str, set[str]] | None' = None) -> None:
    """为用户树类发射分派闭包（追加在类文件尾部）并登记工厂路径。

    reflect_members：JDK / 库类的常量反射引用面（callchain.REFLECT_CONSTS，MH-native）
    ——只为被指名的方法发射臂（无反射调用边的成员不付代码税）。"""
    from .inherited_gen import class_use_path

    for bin_name in sorted(user_bins):
        # 字段闭包：用户树全部类（含接口——接口常量是 public static 字段）
        em = emissions.get(bin_name)
        if em is None or em.handwritten or registry.get(bin_name) is None:
            continue
        from ..type_map import short_cls
        ftext = _emit_fields_for(bin_name, short_cls(bin_name), em)
        if ftext is None:
            continue
        em.text = em.text.rstrip('\n') + '\n' + ftext + '\n'
        fpath = class_use_path(bin_name, 'java_runtime', emissions, 'user')
        FIELD_LEDGER[bin_name] = f'    ("{bin_name}", std::rc::Rc::new(' \
            f'|n, r, v| {fpath}::__reflect_field(n, r, v))),'

    targets = {b: None for b in user_bins}
    for b, names in (reflect_members or {}).items():
        if b not in targets:
            targets[b] = set(names)
    for bin_name in sorted(targets):
        only = targets[bin_name]
        em = emissions.get(bin_name)
        if em is None or em.handwritten:
            continue
        ci = registry.get(bin_name)
        if ci is None or getattr(ci, 'is_interface', False):
            continue
        from ..type_map import short_cls
        text = _emit_for(bin_name, short_cls(bin_name), em, only)
        if text is None:
            continue
        em.text = em.text.rstrip('\n') + '\n' + text + '\n'
        path = class_use_path(bin_name, 'java_runtime', emissions, 'user')
        short = short_cls(bin_name)
        LEDGER[bin_name] = f'    ("{bin_name}", std::rc::Rc::new(' \
            f'|n, d, r, a| {path}::__reflect_dispatch(n, d, r, a))),'


def registration_lines() -> list:
    return [LEDGER[bin] for bin in sorted(LEDGER)]


def field_registration_lines() -> list:
    return [FIELD_LEDGER[bin] for bin in sorted(FIELD_LEDGER)]
