public class TestDoWhile {

    public static void main(String[] args) {
        // 条件一开始为假仍执行一次
        boolean flag = false;
        do {
            System.out.println("body executed once");
        } while (flag);

        // 正常计数
        int i = 0;
        do {
            System.out.println("i=" + i);
            i++;
        } while (i < 3);
        System.out.println("after loop i=" + i);

        // continue 跳到条件判断而非循环体顶部（避免死循环）
        int n = 0;
        int iterations = 0;
        do {
            System.out.println("top n=" + n);
            n++;
            iterations++;
            if (n % 2 == 1) {
                continue;
            }
            System.out.println("even n=" + n);
        } while (n < 6);
        System.out.println("iterations=" + iterations + " n=" + n);

        // break 退出
        int k = 0;
        do {
            if (k == 2) {
                System.out.println("break at k=2");
                break;
            }
            System.out.println("k=" + k);
            k++;
        } while (k < 10);

        // 嵌套 do-while
        int outer = 0;
        do {
            int inner = 0;
            do {
                System.out.print(outer + "" + inner + " ");
                inner++;
            } while (inner < 2);
            System.out.println();
            outer++;
        } while (outer < 3);

        // do-while 中的 return
        System.out.println("found=" + findFirst());

        // do-while(false) 当作 goto-block 用
        int v = 5;
        do {
            if (v < 0) break;
            v = v * 2;
        } while (false);
        System.out.println("v=" + v);

        System.out.println("done");
    }

    static int findFirst() {
        int i = 0;
        do {
            if (i * i > 20) {
                return i;
            }
            i++;
        } while (i < 100);
        return -1;
    }
}
