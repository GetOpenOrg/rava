import java.lang.reflect.Array;

/** java.lang.reflect.Array get/set 边界：拓宽、拆箱、类型不符、协变存储检查、null / 非数组 / 越界。 */
public class TestReflectArrayAccessEdges {
    interface Op { void run() throws Exception; }

    static void attempt(String label, Op op) {
        try {
            op.run();
            System.out.println(label + "=ok");
        } catch (Exception e) {
            String msg = e.getMessage();
            System.out.println(label + "=" + e.getClass().getSimpleName()
                    + (e instanceof IllegalArgumentException ? ":" + msg : ""));
        }
    }

    public static void main(String[] args) {
        int[] ints = {7, -3, 42};
        long[] longs = new long[2];
        double[] doubles = new double[2];
        char[] chars = {'A', 'z'};
        byte[] bytes = {(byte) -5};
        boolean[] flags = new boolean[1];
        Object[] objs = new Integer[2];
        Number[] nums = new Number[2];

        System.out.println("get=" + Array.get(ints, 1) + "," + Array.get(chars, 1) + "," + Array.get(bytes, 0));
        System.out.println("getLong(int[])=" + Array.getLong(ints, 2));
        System.out.println("getDouble(char[])=" + Array.getDouble(chars, 0));
        System.out.println("getInt(byte[])=" + Array.getInt(bytes, 0));
        attempt("getInt(long[])", () -> Array.getInt(longs, 0));
        attempt("getChar(int[])", () -> Array.getChar(ints, 0));
        attempt("getInt(Integer[])", () -> Array.getInt(objs, 0));

        Array.setInt(longs, 0, 9);
        Array.setChar(doubles, 1, 'B');
        Array.set(longs, 1, Integer.valueOf(5));
        System.out.println("longs=" + longs[0] + "," + longs[1] + " doubles[1]=" + doubles[1]);
        attempt("setLong(int[])", () -> Array.setLong(ints, 0, 1L));
        attempt("set(int[],Long)", () -> Array.set(ints, 0, Long.valueOf(1)));
        attempt("set(int[],null)", () -> Array.set(ints, 0, null));
        attempt("set(int[],String)", () -> Array.set(ints, 0, "x"));
        attempt("setBoolean(int[])", () -> Array.setBoolean(ints, 0, true));
        Array.setBoolean(flags, 0, true);
        System.out.println("flags[0]=" + flags[0] + " get=" + Array.get(flags, 0));

        Array.set(objs, 0, Integer.valueOf(11));
        Array.set(objs, 1, null);
        System.out.println("objs=" + objs[0] + "," + objs[1]);
        attempt("set(Integer[],String)", () -> Array.set(objs, 0, "s"));
        Array.set(nums, 0, Long.valueOf(3));
        Array.set(nums, 1, Double.valueOf(2.5));
        System.out.println("nums=" + nums[0] + "," + nums[1]);
        attempt("set(Number[],String)", () -> Array.set(nums, 0, "s"));
        attempt("setInt(Number[])", () -> Array.setInt(nums, 0, 1));

        attempt("get(null)", () -> Array.get(null, 0));
        attempt("get(String)", () -> Array.get("abc", 0));
        attempt("set(Integer)", () -> Array.set(Integer.valueOf(1), 0, null));
        attempt("get(int[],3)", () -> Array.get(ints, 3));
        attempt("set(Integer[],-1)", () -> Array.set(objs, -1, null));
        attempt("setInt(int[],5)", () -> Array.setInt(ints, 5, 0));
    }
}
