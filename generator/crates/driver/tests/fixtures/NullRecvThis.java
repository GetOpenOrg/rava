// 实例方法自身的 this 按 JVMS 恒非 null：编码器来自手写边界（类型集里没有可派发的类、值流不可追溯），
// CharsetEncoder 方法体里 this 上的虚调用（encodeLoop / implFlush）及其结果上的调用都不得折叠为 null_recv。
import java.nio.charset.StandardCharsets;

public class NullRecvThis {
    public static void main(String[] args) {
        System.out.println("abc".getBytes(StandardCharsets.ISO_8859_1).length);
    }
}
