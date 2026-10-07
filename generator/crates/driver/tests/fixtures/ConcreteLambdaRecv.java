// 具体求值（Pattern.compile 字面量）的轨迹执行过字符类并集 lambda 实现方法里捕获谓词上的接口调用；
// 运行期同一段字节码照常执行，该调用点不得因抽象克隆的捕获值为空折叠为 null_recv。
import java.util.regex.Pattern;

public class ConcreteLambdaRecv {
    static final Pattern P = Pattern.compile("[a-zA-Z%]");

    public static void main(String[] args) {
        System.out.println(P.pattern());
    }
}
