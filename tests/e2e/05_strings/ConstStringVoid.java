public class ConstStringVoid {
    public static void main(String[] args) {
        // InvokeVoid on string literal: void methods on string constants
        String s = "  hello  ";
        System.out.println(s.trim().isEmpty()); // false
        System.out.println("".isEmpty());        // true
        System.out.println("abc".length());      // 3
        StringBuilder sb = new StringBuilder();
        sb.append("world");
        System.out.println(sb.toString());       // world
    }
}
