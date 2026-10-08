/*
 * VM 支持类（rava）：直连反射调用——Method.invoke 在「目标不是 @CallerSensitive」时的特化入口。
 *
 * Method.invoke 对任意反射对象都要判 isCallerSensitive（解析方法注解）、取访问器、按 CS 与否两路调用访问器，
 * 这些分支的取舍取决于运行期的反射对象，闭包分析器无从折叠。分析器把查找结果建模为携带所指方法的标记，
 * 证明某个调用点的反射对象只可能是标记所指的非 CS 方法时（vm_intrinsics.toml [facts.reflect.direct_invokers]，
 * generator engine/method_marks.rs、engine/reflect_direct.rs），把该调用指令改写为对本类 invoke 的 invokestatic
 * （栈形不变：Method、obj、args → Object）。
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

import java.util.AbstractList;
import java.util.Iterator;
import java.util.List;
import java.util.NoSuchElementException;

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

    /**
     * 列表形查找结果的分析模型（清单 direct_invokers 的 list）：标记数组的只读列表视图。分析器把列表形查找
     * （JavaLangAccess.getDeclaredPublicMethods）的结果建模为本方法对标记数组的返回值；运行期不调用。
     * 不借用 ArrayList：其 elementData 经 Arrays.copyOf 的全程序共享分配点扩容，元素会混入别处的值，
     * 使经列表取出的接收者不再全是标记；本视图的元素只来自传入的标记数组。
     */
    static List<Method> list(Method[] ms) {
        return new Marks(ms);
    }

    /** 标记数组的只读列表视图（含自有迭代器，不经 AbstractList$Itr 的共享字段）。 */
    private static final class Marks extends AbstractList<Method> {
        private final Method[] ms;

        Marks(Method[] ms) {
            this.ms = ms;
        }

        @Override
        public Method get(int i) {
            return ms[i];
        }

        @Override
        public int size() {
            return ms.length;
        }

        @Override
        public Iterator<Method> iterator() {
            return new It(ms);
        }
    }

    private static final class It implements Iterator<Method> {
        private final Method[] ms;
        private int i;

        It(Method[] ms) {
            this.ms = ms;
        }

        @Override
        public boolean hasNext() {
            return i < ms.length;
        }

        @Override
        public Method next() {
            if (i >= ms.length) {
                throw new NoSuchElementException();
            }
            return ms[i++];
        }
    }

    /** 本地访问器的调用（同 DirectMethodHandleAccessor$NativeAccessor.invoke0：按声明键分派）。 */
    private static native Object invoke0(Method m, Object obj, Object[] args)
        throws InvocationTargetException;
}
