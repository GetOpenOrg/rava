import java.util.StringJoiner;
import java.util.StringTokenizer;

/**
 * 缺口 A2：字段「声明类型」是否作为独立的类型依赖进入闭包。
 *
 * 下面两个字段的声明类型（StringJoiner / StringTokenizer）在 main 里从未被
 * new、也从未被 getstatic/putstatic 触达，唯一的出现位置就是字段描述符。
 * 若引用收集只覆盖「方法描述符 + 字段访问指令」，这两个类型会缺失，
 * 生成代码里字段的类型无处可指。
 */
public class TestFieldTypeRef {
    static StringJoiner staticJoiner;
    StringTokenizer instanceTokenizer;

    public static void main(String[] args) {
        System.out.println("start");
        System.out.println(new TestFieldTypeRef() != null);
        System.out.println("end");
    }
}
