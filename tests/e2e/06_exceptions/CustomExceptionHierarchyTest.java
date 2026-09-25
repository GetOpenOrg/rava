public class CustomExceptionHierarchyTest {
    // 多层自定义异常继承链
    static class BusinessException extends RuntimeException {
        public BusinessException(String msg) {
            super(msg);
        }
    }

    static class OrderException extends BusinessException {
        public OrderException(String msg) {
            super(msg);
        }
    }

    static class PaymentException extends OrderException {
        public PaymentException(String msg) {
            super(msg);
        }
    }

    public static void main(String[] args) {
        // Test 1: catch BusinessException should catch OrderException
        try {
            throw new OrderException("order failed");
        } catch (BusinessException e) {
            System.out.println("caught as Business: " + e.getMessage());
        }

        // Test 2: catch BusinessException should catch PaymentException (3 levels deep)
        try {
            throw new PaymentException("payment failed");
        } catch (BusinessException e) {
            System.out.println("caught as Business: " + e.getMessage());
        }

        // Test 3: catch RuntimeException should catch all custom exceptions
        try {
            throw new PaymentException("runtime catch");
        } catch (RuntimeException e) {
            System.out.println("caught as Runtime: " + e.getMessage());
        }

        // Test 4: catch Exception should catch all custom exceptions
        try {
            throw new OrderException("exception catch");
        } catch (Exception e) {
            System.out.println("caught as Exception: " + e.getMessage());
        }

        // Test 5: OrderException should NOT be caught by PaymentException catch
        boolean caught = false;
        try {
            try {
                throw new OrderException("not payment");
            } catch (PaymentException e) {
                System.out.println("ERROR: should not catch");
                caught = true;
            }
        } catch (OrderException e) {
            System.out.println("correctly fell through: " + e.getMessage());
        }

        // Test 6: exact match
        try {
            throw new PaymentException("exact");
        } catch (PaymentException e) {
            System.out.println("exact match: " + e.getMessage());
        }

        System.out.println("Done.");
    }
}
