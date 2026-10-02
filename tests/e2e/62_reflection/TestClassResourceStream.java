import java.io.InputStream;

/**
 * 类路径资源读取：getResource / getResourceAsStream 读自身 class 文件（魔数校验）、
 * 未命中 null、系统资源（mybatis Resources、logback 配置查找、typesafe config 的
 * K10 前奏——资源内容不可静态枚举，但命中/未命中/内容流三态可测）。
 */
public class TestClassResourceStream {

    public static void main(String[] args) throws Exception {
        String self = "TestClassResourceStream.class";

        InputStream in = TestClassResourceStream.class.getResourceAsStream(self);
        System.out.println("self-stream=" + (in != null));
        byte[] head = new byte[4];
        int read = in.read(head);
        in.close();
        System.out.println("magic-read=" + read
                + " cafebabe=" + String.format("%02X%02X%02X%02X", head[0], head[1], head[2], head[3]));

        System.out.println("self-url=" + (TestClassResourceStream.class.getResource(self) != null));
        System.out.println("miss=" + (TestClassResourceStream.class.getResource("no-such.txt") == null));
        System.out.println("miss-stream="
                + (TestClassResourceStream.class.getResourceAsStream("no-such.txt") == null));

        // 绝对/相对名解析：不带斜杠相对本类包；带斜杠从类路径根
        System.out.println("fqcn-abs=" + (ClassLoader.getSystemResource("TestClassResourceStream.class") != null));

        // 系统资源：JDK 自身类可命中
        System.out.println("jdk-res=" + (ClassLoader.getSystemResource("java/lang/String.class") != null));
        System.out.println("jdk-res-miss="
                + (ClassLoader.getSystemResource("java/lang/Nope.class") == null));

        // 同名资源经类加载器与经 Class 取得的一致性
        System.out.println("loader-eq=" + (TestClassResourceStream.class.getClassLoader()
                .getResource(self) != null));
    }
}
