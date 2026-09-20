public class TestFinallyReturn {

    // finally 修改返回值变量：不影响已经算好的返回值
    static int tryReturnFinallyModify() {
        int r = 1;
        try {
            r = 2;
            return r;
        } finally {
            r = 3;
            System.out.println("finally r=" + r);
        }
    }

    // finally 自己 return：覆盖 try 的 return
    static int tryReturnFinallyReturn() {
        try {
            return 1;
        } finally {
            System.out.println("finally overrides");
            return 2;
        }
    }

    // catch return 之后 finally 执行但不改结果
    static int catchReturnFinallyModify() {
        try {
            throw new RuntimeException("x");
        } catch (RuntimeException e) {
            System.out.println("caught " + e.getMessage());
            return 10;
        } finally {
            System.out.println("finally after catch");
        }
    }

    // finally return 吞掉正在抛出的异常
    static int swallowException() {
        try {
            throw new RuntimeException("swallowed");
        } finally {
            return 99;
        }
    }

    // 对象引用：finally 修改引用不影响返回，修改对象内容可见
    static StringBuilder objectCase() {
        StringBuilder sb = new StringBuilder("a");
        try {
            sb.append("b");
            return sb;
        } finally {
            sb.append("c");
            sb = new StringBuilder("replaced");
        }
    }

    // 带资源的 try 也有 finally
    static int nestedFinally() {
        try {
            try {
                return 1;
            } finally {
                System.out.println("inner finally");
                return 2;
            }
        } finally {
            System.out.println("outer finally");
        }
    }

    public static void main(String[] args) {
        System.out.println("case1=" + tryReturnFinallyModify());
        System.out.println("case2=" + tryReturnFinallyReturn());
        System.out.println("case3=" + catchReturnFinallyModify());
        System.out.println("case4=" + swallowException());
        System.out.println("case5=" + objectCase());
        System.out.println("case6=" + nestedFinally());

        // 异常是否真的被吞掉：外层不应捕获到
        try {
            System.out.println("swallow result=" + swallowException());
        } catch (RuntimeException e) {
            System.out.println("should not reach: " + e.getMessage());
        }

        System.out.println("done");
    }
}
