import java.util.Arrays;

public class TestArraysMismatchTypes {
    public static void main(String[] args) {
        System.out.println(Arrays.mismatch(new char[]{'a', 'b', 'c'}, new char[]{'a', 'b', 'd'}));
        System.out.println(Arrays.mismatch(new short[]{1, 2}, new short[]{1, 2}));
        System.out.println(Arrays.mismatch(new long[]{1L, 2L, 3L}, 1, 3, new long[]{2L, 4L}, 0, 2));
        System.out.println(Arrays.mismatch(new boolean[]{true, false}, new boolean[]{true, true}));
        System.out.println(Arrays.mismatch(new int[]{5, 6, 7, 8}, 1, 4, new int[]{6, 7, 9}, 0, 3));
        System.out.println(Arrays.mismatch(new float[]{Float.NaN, 0.0f}, new float[]{Float.NaN, -0.0f}));
        System.out.println(Arrays.mismatch(new double[]{1.5, Double.NaN}, new double[]{1.5, Double.NaN}));
        System.out.println(Arrays.equals(new double[]{0.0}, new double[]{-0.0}));
        System.out.println(Arrays.equals(new char[]{'x', 'y'}, new char[]{'x', 'y'}));
        System.out.println(Arrays.compare(new long[]{1, 2, 3}, new long[]{1, 2, 4}));
        System.out.println(Arrays.equals(new float[]{1f, 2f, 3f}, 0, 2, new float[]{1f, 2f}, 0, 2));
    }
}
