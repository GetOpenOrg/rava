public class TestLabeledBreak {

    public static void main(String[] args) {
        // 标签 break：从内层直接跳出外层循环
        outer:
        for (int i = 0; i < 5; i++) {
            for (int j = 0; j < 5; j++) {
                if (i * j > 6) break outer;
                System.out.println("pair " + i + "," + j);
            }
        }

        // 标签 continue：跳过外层本次迭代剩余部分
        rows:
        for (int i = 0; i < 4; i++) {
            System.out.println("row-start " + i);
            for (int j = 0; j < 4; j++) {
                if (j == 2) continue rows;
                System.out.println("  col " + j);
            }
            System.out.println("row-end " + i);
        }

        // 标签 while
        int k = 0;
        scan:
        while (k < 10) {
            int m = 0;
            while (m < 10) {
                if (m == 3) {
                    k += 2;
                    continue scan;
                }
                if (k >= 8) break scan;
                m++;
            }
            System.out.println("k=" + k);
        }
        System.out.println("after scan k=" + k);

        // 三层嵌套跳到最外层
        deep:
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                for (int l = 0; l < 3; l++) {
                    if (i + j + l == 4) break deep;
                    System.out.println("d " + i + j + l);
                }
            }
        }
        System.out.println("deep done");

        // 标签 break 与 switch 混用
        search:
        for (int i = 0; i < 3; i++) {
            switch (i) {
                case 0:
                    System.out.println("case0");
                    break;
                case 1:
                    System.out.println("case1 hit");
                    break search;
                default:
                    System.out.println("case" + i);
            }
        }
        System.out.println("search done");

        System.out.println("end");
    }
}
