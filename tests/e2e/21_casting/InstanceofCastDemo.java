public class InstanceofCastDemo {
    public static void main(String[] args) {
        Object obj1 = "Hello";
        Object obj2 = Integer.valueOf(42);
        Object obj3 = null;

        // 基础 instanceof
        System.out.println(obj1 instanceof String);   // true
        System.out.println(obj2 instanceof String);   // false
        System.out.println(obj3 instanceof String);   // false

        // instanceof + cast 模式（字节码是 instanceof + checkcast）
        if (obj1 instanceof String) {
            String s = (String) obj1;
            System.out.println(s.length());  // 5
        }

        // 多类型分支
        printType(obj1);  // String: Hello
        printType(obj2);  // Integer: 42
        printType(obj3);  // null
    }

    static void printType(Object obj) {
        if (obj == null) {
            System.out.println("null");
        } else if (obj instanceof String) {
            System.out.println("String: " + (String) obj);
        } else if (obj instanceof Integer) {
            System.out.println("Integer: " + ((Integer) obj).intValue());
        } else {
            System.out.println("other");
        }
    }
}
