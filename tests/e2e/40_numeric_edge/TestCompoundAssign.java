public class TestCompoundAssign {
    public static void main(String[] args) {
        byte b = 0;
        b += 200;
        System.out.println("byteAdd=" + b);
        short s = 0;
        s += 40000;
        System.out.println("shortAdd=" + s);
        char c = 0;
        c += 70000;
        System.out.println("charAdd=" + (int) c);
        int i = 10;
        i += 2.9;
        System.out.println("intAddDouble=" + i);
        byte b2 = 5;
        System.out.println("explicit=" + (byte) (b2 + 200));
        float f = 1.0f;
        f += 0.5;
        System.out.println("floatAdd=" + f);
    }
}
