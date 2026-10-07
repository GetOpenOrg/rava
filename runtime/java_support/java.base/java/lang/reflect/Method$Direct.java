/*
 * VM 支持类（rava）：直连反射调用——Method.invoke 在「目标不是 @CallerSensitive」时的特化入口。
 *
 * Method.invoke 对任意反射对象都要判 isCallerSensitive（解析方法注解）、取访问器、按 CS 与否两路调用访问器，
 * 这些分支的取舍取决于运行期的反射对象，闭包分析器无从折叠。分析器证明某个调用点的反射对象只可能是
 * 非 CS 的静态目标时（vm_intrinsics.toml [facts.reflect.direct_invokers]，generator engine/reflect_direct.rs），
 * 把该调用指令改写为对本类 invoke 的 invokestatic（栈形不变：Method、obj、args → Object）。
 *
 * 语义与 Method.invoke 逐句一致：isCallerSensitive 恒为假；未 override 时以调用者类做访问检查
 * （本方法标注 @CallerSensitive，生成器在改写后的调用点压入调用方所在类，getCallerClass 取到的即原
 * Method.invoke 的调用者）；访问器恒为本地访问器（jdk.reflect.useNativeAccessorOnly），其非 CS 调用即
 * invoke0(method, obj, args)。clazz / modifiers 是 Method 的私有字段，本类经等值的 getDeclaringClass /
 * getModifiers 取得。
 *
 * 编译：转译时以当前 JDK 的 javac --patch-module java.base 编入 java.lang.reflect 包
 * （generator/crates/resolve/src/image.rs VM 支持类目录）。
 * native invoke0：runtime/java_runtime/src/java/lang/reflect/method_direct_impl.rs
 */
package java.lang.reflect;

import jdk.internal.reflect.CallerSensitive;
import jdk.internal.reflect.Reflection;

final class Method$Direct {
    private Method$Direct() {}

    /** Method.invoke(obj, args) 在目标非 @CallerSensitive 时的等价形式。 */
    @CallerSensitive
    static Object invoke(Method m, Object obj, Object[] args)
        throws IllegalAccessException, InvocationTargetException
    {
        if (!m.override) {
            Class<?> caller = Reflection.getCallerClass();
            int modifiers = m.getModifiers();
            m.checkAccess(caller, m.getDeclaringClass(),
                    Modifier.isStatic(modifiers) ? null : obj.getClass(),
                    modifiers);
        }
        return invoke0(m, obj, args);
    }

    /** 本地访问器的调用（同 DirectMethodHandleAccessor$NativeAccessor.invoke0：按声明键分派）。 */
    private static native Object invoke0(Method m, Object obj, Object[] args)
        throws InvocationTargetException;
}
