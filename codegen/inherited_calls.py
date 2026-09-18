"""继承成员调用需求登记（调用侧 → 定义侧的唯一通道）。

Java 在具体类接收者上调用继承自祖先的实例方法（`dog.speak()`，speak 声明在祖先 Animal）时，
调用点一律生成与 Java 一致的 `obj.method(args)`，并在此登记
「接收者类需要该继承成员」。全部方法体生成完毕后，emitter 据此在接收者类的
java_class! 块里补上继承成员声明（见 emitter/inherited_gen.py），
由宏展开为 wrapper 上的转发方法 —— vtable 分派细节不出现在方法体里。

按需登记而非为每个类声明全部继承成员：与「只分析调用链」原则一致，
生成规模只随实际调用点增长。
"""

# {接收者类 binary name → {(Java 方法名, 参数描述符部分 '(..)')}}
_requests: dict[str, set[tuple[str, str]]] = {}


def reset() -> None:
    """开始一次新的工程生成前清空登记。"""
    _requests.clear()


def request(receiver_binary: str, method_name: str, param_descriptor: str) -> None:
    """登记：receiver_binary 的实例上调用了它自身未声明的继承方法。

    param_descriptor 只含参数部分（`(I)`），返回类型不参与匹配（协变返回）。
    """
    if receiver_binary and method_name:
        _requests.setdefault(receiver_binary, set()).add((method_name, param_descriptor))


def requests() -> dict[str, set[tuple[str, str]]]:
    return _requests
