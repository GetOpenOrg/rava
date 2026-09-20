import java.util.ArrayList;
import java.util.List;

public class TestTryInLoop {

    static int maybeFail(int i) {
        if (i % 3 == 0) {
            throw new IllegalArgumentException("bad " + i);
        }
        return i * 10;
    }

    public static void main(String[] args) {
        // try-catch 在循环里，异常被吞掉继续下一轮
        StringBuilder kept = new StringBuilder();
        for (int i = 0; i < 6; i++) {
            try {
                kept.append(maybeFail(i)).append(" ");
            } catch (IllegalArgumentException e) {
                kept.append("skip ");
            } finally {
                kept.append("|");
            }
        }
        System.out.println("loop1=" + kept);

        // try 内 continue：finally 仍执行
        StringBuilder trace = new StringBuilder();
        for (int i = 0; i < 4; i++) {
            try {
                if (i % 2 == 0) {
                    continue;
                }
                trace.append("body" + i + " ");
            } finally {
                trace.append("f" + i + " ");
            }
        }
        System.out.println("continueTrace=" + trace);

        // try 内 break：finally 仍执行并且在退出前完成
        StringBuilder breakTrace = new StringBuilder();
        for (int i = 0; i < 5; i++) {
            try {
                if (i == 2) {
                    break;
                }
                breakTrace.append(i);
            } finally {
                breakTrace.append("[" + i + "]");
            }
        }
        System.out.println("breakTrace=" + breakTrace);

        // while 循环 + return
        System.out.println("foundFirstEven=" + findFirstEven());

        // 嵌套循环：内层 try-finally + 标签 break
        StringBuilder labeled = new StringBuilder();
        outer:
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                try {
                    if (i + j == 3) {
                        break outer;
                    }
                    labeled.append(i).append(j).append(" ");
                } finally {
                    labeled.append("(" + i + "," + j + ")");
                }
            }
        }
        System.out.println("labeled=" + labeled);

        // 循环里累积 List，异常时回滚最后一项
        List<String> results = new ArrayList<>();
        for (int i = 0; i < 5; i++) {
            results.add("start" + i);
            try {
                results.add(String.valueOf(maybeFail(i)));
            } catch (IllegalArgumentException e) {
                results.add("rollback" + i);
                results.remove(results.size() - 2);
            }
        }
        System.out.println("results=" + results);

        System.out.println("done");
    }

    static int findFirstEven() {
        int cursor = 0;
        while (true) {
            try {
                if (cursor % 2 == 0 && cursor > 0) {
                    return cursor;
                }
                cursor++;
            } finally {
                System.out.print("#" + cursor + " ");
            }
        }
    }
}
