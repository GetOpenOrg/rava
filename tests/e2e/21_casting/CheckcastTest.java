public class CheckcastTest {
    // Helper: store array in Object variable, then cast back
    static Object asObject(Object o) {
        return o;
    }

    public static void main(String[] args) {
        // 1. String[] checkcast
        String[] names = {"Alice", "Bob", "Charlie"};
        Object obj1 = names;
        String[] back1 = (String[]) obj1;
        System.out.println("String array length: " + back1.length);
        System.out.println("String array[0]: " + back1[0]);
        System.out.println("String array[2]: " + back1[2]);

        // 2. int[] checkcast
        int[] nums = {10, 20, 30};
        Object obj2 = nums;
        int[] back2 = (int[]) obj2;
        System.out.println("Int array length: " + back2.length);
        System.out.println("Int array[1]: " + back2[1]);

        // 3. Object[] checkcast
        Object[] objs = {"Hello", "World"};
        Object obj3 = objs;
        Object[] back3 = (Object[]) obj3;
        System.out.println("Object array length: " + back3.length);
        System.out.println("Object array[0]: " + back3[0]);

        // 4. Boxing cast regression — (Integer) and (String)
        Object boxed = Integer.valueOf(42);
        Integer unboxed = (Integer) boxed;
        System.out.println("Unboxed Integer: " + unboxed);

        Object strObj = "hello";
        String str = (String) strObj;
        System.out.println("Cast String: " + str);

        System.out.println("All checkcast tests passed!");
    }
}
