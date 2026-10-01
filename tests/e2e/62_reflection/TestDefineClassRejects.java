import java.nio.ByteBuffer;

/**
 * ClassLoader.defineClass1 / defineClass2 的类文件前置校验（HotSpot ClassFileParser 次序）：
 * 截断、魔数、主版本过高、主版本过低；越界与 null 参数检查发生在 native 之前或入口处。
 */
public class TestDefineClassRejects extends ClassLoader {
    static byte[] header(int major, int extra) {
        byte[] b = new byte[8 + extra];
        b[0] = (byte) 0xCA; b[1] = (byte) 0xFE; b[2] = (byte) 0xBA; b[3] = (byte) 0xBE;
        b[6] = (byte) (major >> 8); b[7] = (byte) major;
        for (int i = 0; i < extra; i++) b[8 + i] = (byte) (i + 1);
        return b;
    }

    public static void main(String[] args) {
        TestDefineClassRejects loader = new TestDefineClassRejects();
        byte[][] cases = {
            {1, 2, 3},
            {1, 2, 3, 4, 5, 6, 7, 8, 9, 10},
            header(66, 2),
            header(44, 0),
            header(65, 0),
            header(65, 5),
        };
        for (byte[] b : cases) {
            try {
                loader.defineClass("a.b.Foo", b, 0, b.length);
                System.out.println("defined?");
            } catch (Throwable t) {
                System.out.println("array : " + t);
            }
            try {
                loader.defineClass(null, b, 0, b.length);
                System.out.println("defined?");
            } catch (Throwable t) {
                System.out.println("noname: " + t);
            }
            try {
                ByteBuffer bb = ByteBuffer.allocateDirect(b.length);
                bb.put(b).flip();
                loader.defineClass("a.b.Foo", bb, null);
                System.out.println("defined?");
            } catch (Throwable t) {
                System.out.println("direct: " + t);
            }
        }
        try {
            loader.defineClass("a.b.Foo", new byte[4], 2, 5);
        } catch (Throwable t) {
            System.out.println("bounds: " + t.getClass().getName());
        }
        try {
            loader.defineClass("java.lang.Evil", header(65, 0), 0, 8);
        } catch (Throwable t) {
            System.out.println("prohibited: " + t);
        }
    }
}
