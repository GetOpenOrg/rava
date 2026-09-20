public class TestShiftOps {

    public static void main(String[] args) {
        int x = 1;

        // int 移位：位移量取低 5 位（& 31）
        System.out.println("1<<0=" + (x << 0));
        System.out.println("1<<31=" + (x << 31));
        System.out.println("1<<32=" + (x << 32));   // 等价于 <<0
        System.out.println("1<<33=" + (x << 33));   // 等价于 <<1
        System.out.println("1<<-1=" + (x << -1));   // 等价于 <<31

        // 算术右移保持符号位
        System.out.println("-8>>1=" + (-8 >> 1));
        System.out.println("-8>>2=" + (-8 >> 2));

        // 无符号右移：高位补 0
        System.out.println("-8>>>1=" + (-8 >>> 1));
        System.out.println("-1>>>28=" + (-1 >>> 28));
        System.out.println("-1>>>32=" + (-1 >>> 32)); // 等价于 >>>0

        // long 移位：位移量取低 6 位（& 63）
        long L = 1L;
        System.out.println("1L<<62=" + (L << 62));
        System.out.println("1L<<63=" + (L << 63));
        System.out.println("1L<<64=" + (L << 64));   // 等价于 <<0
        System.out.println("1L<<65=" + (L << 65));   // 等价于 <<1
        System.out.println("-1L>>>60=" + (-1L >>> 60));
        System.out.println("-8L>>1=" + (-8L >> 1));

        // 移位参与表达式：提取高低位
        int packed = 0x1234;
        System.out.println("hi=" + ((packed >> 8) & 0xFF));
        System.out.println("lo=" + (packed & 0xFF));

        // 复合赋值移位
        int v = 3;
        v <<= 4;
        System.out.println("v<<=4 -> " + v);
        v >>= 2;
        System.out.println("v>>=2 -> " + v);
        v >>>= 1;
        System.out.println("v>>>=1 -> " + v);

        // long 复合赋值
        long lv = -16L;
        lv >>>= 2;
        System.out.println("lv>>>=2 -> " + lv);

        System.out.println("done");
    }
}
