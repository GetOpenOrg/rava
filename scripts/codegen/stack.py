"""
JVM 操作数栈模拟器：将基于栈的字节码转换为具名局部变量（SSA 前驱）。
"""


class StackSim:
    def __init__(self, param_rust_types: list[str], is_static: bool, class_name: str):
        self.stack:      list[tuple[str, str]] = []
        self._ctr:       int                   = 0
        self.locals:     dict[int, tuple]      = {}
        self.stmts:      list[str]             = []
        self.is_static   = is_static
        self.class_name  = class_name

        if is_static:
            for slot, rt in enumerate(param_rust_types):
                self.locals[slot] = (f"arg_{slot}", rt, False)
        else:
            # slot 0 = this（Rc<RefCell<ClassName>>）
            this_ty = f"Rc<RefCell<{class_name}>>" if class_name else "JvmObject"
            self.locals[0] = ("this", this_ty, False)
            for slot, rt in enumerate(param_rust_types):
                self.locals[slot + 1] = (f"arg_{slot}", rt, False)

    def fresh(self, prefix: str = '_t') -> str:
        v = f"{prefix}{self._ctr}"
        self._ctr += 1
        return v

    def push(self, expr: str, ty: str = 'i32'):
        self.stack.append((expr, ty))

    def pop(self) -> tuple[str, str]:
        return self.stack.pop() if self.stack else ('/* UNDERFLOW */', 'i32')

    def emit(self, stmt: str):
        self.stmts.append(stmt)

    def store_local(self, slot: int, expr: str, ty: str):
        if slot in self.locals:
            name, _, _ = self.locals[slot]
            self.emit(f"    {name} = {expr};")
        else:
            name = f"local_{slot}"
            self.locals[slot] = (name, ty, True)
            self.emit(f"    let mut {name}: {ty} = {expr};")

    def load_local(self, slot: int) -> tuple[str, str]:
        if slot in self.locals:
            name, ty, _ = self.locals[slot]
            return (name, ty)
        return (f"local_{slot}", 'i32')
