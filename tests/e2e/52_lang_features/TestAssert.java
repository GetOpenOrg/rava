public class TestAssert {
    public static void main(String[] args) {
        int x = 5;
        assert x == 5 : "x should be 5";
        System.out.println("assertTrue-ok");
        // 该断言仅在开启 -ea 时抛错；默认（无 -ea）为 no-op，保证与转译器运行一致。
        assert x == 6 : "this only fails with -ea";
        System.out.println("afterAssert-false");
        String msg = "computed";
        assert true : msg;
        System.out.println("msg=" + msg);
        System.out.println("done");
    }
}
