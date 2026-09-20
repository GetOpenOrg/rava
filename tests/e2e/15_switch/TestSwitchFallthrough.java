public class TestSwitchFallthrough {

    static String classify(int v) {
        StringBuilder sb = new StringBuilder();
        switch (v) {
            case 0:
                sb.append("zero,");
            case 1:
                sb.append("low,");
            case 2:
            case 3:
                sb.append("mid,");
                break;
            case 4:
                sb.append("four,");
            default:
                sb.append("def,");
        }
        return sb.toString();
    }

    static int hoursPrice(int h) {
        int price = 0;
        switch (h) {
            case 1:
                price += 10;
            case 2:
                price += 20;
            case 3:
                price += 30;
                break;
            default:
                price = -1;
        }
        return price;
    }

    public static void main(String[] args) {
        // fall-through 累积
        for (int i = 0; i <= 5; i++) {
            System.out.println(i + " -> " + classify(i));
        }

        // 缺 break 的价格累加
        System.out.println("h1=" + hoursPrice(1));
        System.out.println("h2=" + hoursPrice(2));
        System.out.println("h3=" + hoursPrice(3));
        System.out.println("h9=" + hoursPrice(9));

        // default 放在中间
        int k = 7;
        switch (k) {
            case 1:
                System.out.println("one");
                break;
            default:
                System.out.println("default-first");
                break;
            case 7:
                System.out.println("seven");
                break;
        }

        // 没有 default 且无匹配 → 什么都不做
        int m = 100;
        switch (m) {
            case 1:
                System.out.println("hit-1");
            case 2:
                System.out.println("hit-2");
        }
        System.out.println("after-no-match");

        // switch 体内声明变量并跨 case 使用
        int n = 2;
        switch (n) {
            case 2: {
                int tmp = n * 10;
                System.out.println("tmp=" + tmp);
                break;
            }
            case 3: {
                int tmp = n * 20;
                System.out.println("tmp3=" + tmp);
                break;
            }
            default:
                break;
        }

        // char switch（本质是 int tableswitch）
        char grade = 'B';
        switch (grade) {
            case 'A':
                System.out.println("excellent");
                break;
            case 'B':
                System.out.println("good");
                break;
            case 'C':
                System.out.println("pass");
                break;
            default:
                System.out.println("fail");
        }

        // 稀疏 case 值触发 lookupswitch
        int sparse = 1000;
        switch (sparse) {
            case 1:
                System.out.println("s1");
                break;
            case 100:
                System.out.println("s100");
                break;
            case 1000:
                System.out.println("s1000");
                break;
            case 100000:
                System.out.println("s100000");
                break;
            default:
                System.out.println("sdefault");
        }

        System.out.println("done");
    }
}
