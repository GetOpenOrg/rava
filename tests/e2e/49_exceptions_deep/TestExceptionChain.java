public class TestExceptionChain {
    public static void main(String[] args) {
        Throwable cause = new IllegalArgumentException("root");
        Exception wrapped = new Exception("wrap");
        wrapped.initCause(cause);
        System.out.println("msg=" + wrapped.getMessage());
        System.out.println("cause=" + wrapped.getCause().getMessage());
        System.out.println("causeType=" + wrapped.getCause().getClass().getSimpleName());
        Throwable t = new Throwable("base");
        Throwable back = t.fillInStackTrace();
        System.out.println("sameRef=" + (back == t));
        System.out.println("msg2=" + t.getMessage());
    }
}
