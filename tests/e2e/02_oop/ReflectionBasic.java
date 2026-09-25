public class ReflectionBasic {
    public static void main(String[] args) {
        ReflectionBasic obj = new ReflectionBasic();
        String simpleName = obj.getClass().getSimpleName();
        System.out.println(simpleName); // ReflectionBasic
    }
}
