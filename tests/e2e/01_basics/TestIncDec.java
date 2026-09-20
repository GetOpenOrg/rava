public class TestIncDec {

    static int[] arr = {1, 2, 3};

    static int nextIndex = 0;

    static int consume() {
        return arr[nextIndex++];
    }

    public static void main(String[] args) {
        // 后缀自增：先取值再自增
        int a = 5;
        int b = a++;
        System.out.println("a=" + a + " b=" + b);

        // 前缀自增：先自增再取值
        int c = 5;
        int d = ++c;
        System.out.println("c=" + c + " d=" + d);

        // 经典陷阱 i = i++
        int i = 0;
        i = i++;
        System.out.println("i = i++ => " + i);

        int j = 0;
        j = ++j;
        System.out.println("j = ++j => " + j);

        // 表达式中混用
        int x = 3;
        int r = x++ + ++x + x++;
        System.out.println("x=" + x + " r=" + r);

        // 前后缀自减
        int y = 5;
        System.out.println("y--=" + (y--) + " then y=" + y);
        System.out.println("--y=" + (--y) + " then y=" + y);

        // 数组下标中的自增（顺序求值）
        int k = 0;
        arr[k++] = 10;
        System.out.println("arr=" + arr[0] + "," + arr[1] + "," + arr[2] + " k=" + k);

        // 数组访问自增
        System.out.println("consume=" + consume() + " idx=" + nextIndex);
        System.out.println("consume=" + consume() + " idx=" + nextIndex);

        // 循环中的 ++ / --（含 Java 无 ++i / i++ 差异的场景）
        StringBuilder up = new StringBuilder();
        for (int n = 0; n < 3; n++) {
            up.append(n);
        }
        StringBuilder down = new StringBuilder();
        for (int n = 3; n > 0; n--) {
            down.append(n);
        }
        System.out.println("up=" + up + " down=" + down);

        // for 循环里用后缀 vs 前缀结果相同
        StringBuilder viaPre = new StringBuilder();
        for (int n = 0; n < 3; ++n) {
            viaPre.append(n);
        }
        System.out.println("viaPre=" + viaPre);

        // byte / char 上的自增（隐式窄化）
        byte bb = 1;
        bb++;
        System.out.println("byte=" + bb);
        char ch = 'a';
        ch++;
        System.out.println("char=" + ch);
        long lg = 5L;
        lg--;
        System.out.println("long=" + lg);
        double dd = 2.0;
        dd++;
        System.out.println("double=" + dd);

        // 复合赋值：隐含强制转换
        byte cb = 100;
        cb += 100;
        System.out.println("byte +=100 -> " + cb);
        int ci = 10;
        ci -= 3;
        ci *= 4;
        ci /= 7;
        ci %= 3;
        System.out.println("compound chain=" + ci);

        int mask = 0b1100;
        mask <<= 2;
        mask >>= 1;
        mask |= 1;
        mask &= 0b111;
        System.out.println("bitwise compound=" + mask);

        // += 与 float 的混合
        int base = 5;
        base += 2.7;
        System.out.println("int += 2.7 -> " + base);

        System.out.println("done");
    }
}
