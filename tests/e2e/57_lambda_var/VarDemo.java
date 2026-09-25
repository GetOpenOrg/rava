import java.util.ArrayList;

public class VarDemo {
    public static void main(String[] args) {
        // var is a Java 10+ feature; javac infers and emits the concrete type in bytecode.
        // ruva sees no 'var' at all -- only the inferred types -- so no special handling needed.
        var message = "Hello, var!";
        var numbers = new ArrayList<Integer>();
        numbers.add(1);
        numbers.add(2);
        numbers.add(3);

        var sum = 0;
        for (int i = 0; i < numbers.size(); i++) {
            sum += (int) (Integer) numbers.get(i);
        }

        System.out.println(message);
        System.out.println(sum);

        var greeting = "var with concat: " + message;
        System.out.println(greeting);
    }
}
