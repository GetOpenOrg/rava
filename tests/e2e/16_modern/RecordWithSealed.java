// Java 16: record 实现 sealed interface，组合使用
public class RecordWithSealed {
    public static void main(String[] args) {
        Expr expr = new TimesExpr(new ConstantExpr(6), new ConstantExpr(7));
        System.out.println("Answer: " + expr.evaluate());

        Expr sum = new PlusExpr(new ConstantExpr(3), new ConstantExpr(4));
        System.out.println("Sum: " + sum.evaluate());
    }
}

sealed interface Expr permits ConstantExpr, PlusExpr, TimesExpr, NegExpr {
    Integer evaluate();
}

record ConstantExpr(int i) implements Expr {
    public Integer evaluate() { return i; }
}

record PlusExpr(Expr a, Expr b) implements Expr {
    public Integer evaluate() { return a.evaluate() + b.evaluate(); }
}

record TimesExpr(Expr a, Expr b) implements Expr {
    public Integer evaluate() { return a.evaluate() * b.evaluate(); }
}

record NegExpr(Expr e) implements Expr {
    public Integer evaluate() { return -e.evaluate(); }
}
