public class TernaryExpr {
    public static void main(String[] args) {
        int x = 5;
        // 三元表达式：选择字符串
        String result = x > 3 ? "big" : "small";
        System.out.println(result);
        // 三元表达式：选择 boolean
        boolean flag = x == 5 ? true : false;
        System.out.println(flag);
        // 三元表达式：数值计算
        int abs = x < 0 ? -x : x;
        System.out.println(abs);
        // 嵌套三元（两次）
        int y = 2;
        String grade = y >= 3 ? "high" : (y >= 1 ? "mid" : "low");
        System.out.println(grade);
    }
}
