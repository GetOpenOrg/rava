import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class TestGenericThrow {
    static <T extends Exception> void throwIt(T t) throws T {
        throw t;
    }

    @SafeVarargs
    static <T> List<T> asList(T... items) {
        List<T> l = new ArrayList<>();
        for (T it : items) l.add(it);
        return l;
    }

    public static void main(String[] args) {
        try {
            throwIt(new IOException("boom"));
        } catch (IOException e) {
            System.out.println("caught=" + e.getClass().getSimpleName() + ":" + e.getMessage());
        }
        System.out.println("varargs=" + asList("a", "b", "c"));
        System.out.println("varargsNum=" + asList(1, 2, 3));
        System.out.println("varargsEmpty=" + asList());
    }
}
