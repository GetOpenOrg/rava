public class TestCustomException {

    // 受检异常
    static class InvalidInputException extends Exception {
        private final int code;

        InvalidInputException(String message, int code) {
            super(message);
            this.code = code;
        }

        InvalidInputException(String message, Throwable cause) {
            super(message, cause);
            this.code = -1;
        }

        int getCode() {
            return code;
        }
    }

    // 非受检异常 + cause 链
    static class BusinessException extends RuntimeException {
        BusinessException(String message) {
            super(message);
        }

        BusinessException(String message, Throwable cause) {
            super(message, cause);
        }
    }

    // 继承层次：子类先捕获
    static class SubFineException extends InvalidInputException {
        SubFineException(String message) {
            super(message, 999);
        }
    }

    static void validate(int value) throws InvalidInputException {
        if (value < 0) {
            throw new InvalidInputException("negative value", 400);
        }
        if (value == 999) {
            throw new SubFineException("special");
        }
    }

    static void wrap() {
        try {
            int r = 10 / 0;
            System.out.println(r);
        } catch (ArithmeticException e) {
            throw new BusinessException("calc failed", e);
        }
    }

    static void deep(String stage) {
        try {
            if (stage.equals("fail")) {
                throw new IllegalStateException("root cause");
            }
        } catch (IllegalStateException e) {
            throw new BusinessException("outer:" + stage, new BusinessException("middle", e));
        }
    }

    public static void main(String[] args) {
        // 受检异常必须声明或捕获
        try {
            validate(-1);
            System.out.println("valid");
        } catch (InvalidInputException e) {
            System.out.println("caught " + e.getMessage() + " code=" + e.getCode());
        }

        try {
            validate(42);
            System.out.println("valid input accepted");
        } catch (InvalidInputException e) {
            System.out.println("unexpected " + e.getMessage());
        }

        // 子类异常先被捕获
        try {
            validate(999);
        } catch (SubFineException e) {
            System.out.println("sub caught " + e.getMessage() + " code=" + e.getCode());
        } catch (InvalidInputException e) {
            System.out.println("parent caught " + e.getMessage());
        }

        // cause 链
        try {
            wrap();
        } catch (BusinessException e) {
            System.out.println("message=" + e.getMessage());
            System.out.println("cause=" + e.getCause().getClass().getSimpleName());
            System.out.println("cause msg=" + e.getCause().getMessage());
        }

        // 多层 cause 遍历
        try {
            deep("fail");
        } catch (BusinessException e) {
            Throwable cur = e;
            StringBuilder chain = new StringBuilder();
            while (cur != null) {
                if (chain.length() > 0) chain.append(" <- ");
                chain.append(cur.getMessage());
                cur = cur.getCause();
            }
            System.out.println("chain=" + chain);
        }

        // 用 cause 构造函数
        try {
            Throwable root = new IllegalStateException("origin");
            throw new InvalidInputException("wrapped checked", root);
        } catch (Exception e) {
            System.out.println("msg=" + e.getMessage() + " causeIsThrowable=" + (e.getCause() != null));
        }

        // 自定义异常被当作 RuntimeException 使用
        try {
            throw new BusinessException("direct");
        } catch (RuntimeException e) {
            System.out.println("as runtime=" + e.getMessage());
        }

        // fillInStackTrace / getStackTrace 长度 > 0
        BusinessException stack = new BusinessException("trace");
        System.out.println("has stack frames=" + (stack.getStackTrace().length > 0));

        System.out.println("done");
    }
}
