public class ArrayCovarianceDemo {
    public static void main(String[] args) {
        // Array covariance: String[] is a subtype of Object[]
        String[] strings = {"a", "b", "c"};
        Object[] objects = strings;  // widening reference conversion

        System.out.println(objects.length);
        System.out.println(objects[0]);
        System.out.println(objects[1] instanceof String);

        // Reading through the supertype array reference
        for (Object o : objects) {
            System.out.println(o.getClass().getSimpleName());
        }

        // ArrayStoreException when storing incompatible type
        try {
            objects[0] = Integer.valueOf(42);  // Integer is not a String
            System.out.println("no exception (unexpected)");
        } catch (ArrayStoreException e) {
            System.out.println("ArrayStoreException caught");
        }

        // Safe read after attempted store
        System.out.println(objects[0]); // still "a"

        // Multidimensional covariance
        Integer[][] intMatrix = {{1, 2}, {3, 4}};
        Number[][] numMatrix = intMatrix;
        System.out.println(numMatrix[0][0]);
        System.out.println(numMatrix[1][1]);

        // int[] is NOT a subtype of Object[] (primitive arrays are not covariant)
        int[] ints = {1, 2, 3};
        Object obj = ints;  // int[] itself is an Object
        System.out.println(obj instanceof int[]);
        System.out.println(!(obj instanceof Object[]));

        System.out.println("done");
    }
}
