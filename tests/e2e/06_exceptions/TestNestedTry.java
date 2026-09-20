public class TestNestedTry {

    static void innerThrow(String tag, int kind) {
        try {
            if (kind == 0) {
                throw new IllegalArgumentException(tag + "-iae");
            } else {
                throw new IllegalStateException(tag + "-ise");
            }
        } catch (IllegalArgumentException e) {
            System.out.println("inner caught " + e.getMessage());
            throw new RuntimeException("wrapped:" + e.getMessage());
        } finally {
            System.out.println("inner finally " + tag);
        }
    }

    static int level(int depth) {
        try {
            if (depth == 0) {
                return 0;
            }
            return level(depth - 1) + 1;
        } finally {
            System.out.println("unwind level " + depth);
        }
    }

    static String rethrowChain() {
        try {
            try {
                try {
                    throw new IllegalStateException("deep");
                } catch (IllegalStateException e) {
                    System.out.println("level3 caught");
                    throw new RuntimeException("from3");
                }
            } catch (RuntimeException e) {
                System.out.println("level2 caught " + e.getMessage());
                throw new IllegalArgumentException("from2");
            }
        } catch (IllegalArgumentException e) {
            return "level1 caught " + e.getMessage();
        } finally {
            System.out.println("outermost finally");
        }
    }

    public static void main(String[] args) {
        // 内层抛出、外层捕获
        try {
            innerThrow("A", 0);
        } catch (RuntimeException e) {
            System.out.println("outer caught " + e.getMessage());
        }

        // 未重写的异常照常冒泡
        try {
            innerThrow("B", 1);
        } catch (RuntimeException e) {
            System.out.println("outer caught ise -> " + e.getMessage());
        }

        // try-finally 无 catch：异常继续向上，finally 仍执行
        try {
            try {
                throw new RuntimeException("no-catch");
            } finally {
                System.out.println("finally without catch");
            }
        } catch (RuntimeException e) {
            System.out.println("caught after finally " + e.getMessage());
        }

        // 递归 + finally：异常表多层 unwind
        System.out.println("level result=" + level(3));

        // 跨三层的捕获链
        System.out.println(rethrowChain());

        // 嵌套 finally 的执行顺序（由内到外）
        try {
            try {
                System.out.println("deep body");
            } finally {
                System.out.println("deep finally");
            }
        } finally {
            System.out.println("shallow finally");
        }

        // catch 内部再抛，被同层 finally 观察
        try {
            try {
                throw new IllegalArgumentException("first");
            } catch (IllegalArgumentException e) {
                System.out.println("catch then rethrow");
                throw new IllegalStateException("second");
            } finally {
                System.out.println("finally after rethrow");
            }
        } catch (IllegalStateException e) {
            System.out.println("final handler " + e.getMessage());
        }

        System.out.println("done");
    }
}
