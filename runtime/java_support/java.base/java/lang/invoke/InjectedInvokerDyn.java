/*
 * VM 支持类（rava）：@CallerSensitive 方法的注入调用器模板。
 *
 * JDK 的 MethodHandleImpl.BindCaller 对无 @CallerSensitiveAdapter 的 CS 方法（如 Field.get），
 * 以 ASM 模板（generateInvokerTemplate）为每个调用者类 H 定义隐藏巢成员类
 * H$$InjectedInvoker/0x…（加载器、包同 H，超类 Object），经其静态方法转调目标句柄，
 * 使 CS 方法取到的调用者为该注入类。原生二进制不能在运行期定义类：注入类由 VM 按宿主登记
 * （runtime/java_runtime/src/injected_invoker.rs），其两个静态方法的体取本类（字节码翻译），
 * 调用时以注入类为调用者压栈。
 * 方案：docs/plans/2026-10-02-c1d-reflect-narrow.md §3.3「CallerSensitive 方法句柄路径」
 *
 * 编译：转译时以当前 JDK 的 javac --patch-module java.base 编入 java.lang.invoke 包，
 * 经常规字节码翻译进入类宇宙（generator/crates/resolve/src/image.rs VM 支持类目录）。
 */
package java.lang.invoke;

import jdk.internal.vm.annotation.Hidden;

final class InjectedInvokerDyn {
    private InjectedInvokerDyn() {
    }

    /** 包装 CS 方法的直接句柄（BindCaller.bindCallerWithInjectedInvoker）。 */
    @Hidden
    static Object invoke_V(MethodHandle vamh, Object[] args) throws Throwable {
        return vamh.invokeExact(args);
    }

    /** CS 方法的反射访问器（DirectMethodHandleAccessor / NativeAccessor 经 reflectiveInvoker）。 */
    @Hidden
    static Object reflect_invoke_V(MethodHandle vamh, Object target, Object[] args) throws Throwable {
        return vamh.invokeExact(target, args);
    }
}
