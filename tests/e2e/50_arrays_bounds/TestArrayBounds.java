import java.util.Arrays;

public class TestArrayBounds {
    public static void main(String[] args) {
        int[] a = {1, 2, 3};
        try {
            int x = a[5];
            System.out.println(x);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("oob=" + e.getClass().getSimpleName());
        }
        try {
            int[] bad = new int[-1];
            System.out.println(bad);
        } catch (NegativeArraySizeException e) {
            System.out.println("negSize=" + e.getClass().getSimpleName());
        }
        int[] src = {1, 2, 3, 4, 5};
        int[] dst = new int[5];
        System.arraycopy(src, 0, dst, 0, 5);
        System.out.println("copy=" + Arrays.toString(dst));
        int[] ov = {1, 2, 3, 4, 5};
        System.arraycopy(ov, 0, ov, 1, 4);
        System.out.println("overlap=" + Arrays.toString(ov));
        int[] rng = Arrays.copyOfRange(src, 1, 4);
        System.out.println("range=" + Arrays.toString(rng));
        int[] clone = src.clone();
        System.out.println("clone=" + Arrays.toString(clone));
        System.out.println("asList=" + Arrays.asList("a", "b"));
    }
}
