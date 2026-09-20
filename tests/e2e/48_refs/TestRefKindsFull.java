import java.util.*;
import java.util.function.*;

public class TestRefKindsFull {
    static String make(String s) { return "M:" + s; }
    static int len(String s) { return s.length(); }

    static class Parent {
        String who() { return "parent"; }
    }

    static class Child extends Parent {
        String who() { return "child"; }
        String useSuper() { Supplier<String> f = super::who; return "super:" + f.get(); }
        String useThis() { Supplier<String> f = this::who; return "this:" + f.get(); }
    }

    public static void main(String[] args) {
        Function<String, String> ctor = String::new;
        System.out.println("ctor=" + ctor.apply("hi"));
        Function<String, Integer> mref = TestRefKindsFull::len;
        System.out.println("static=" + mref.apply("abc"));
        Function<String, Integer> inst = String::length;
        System.out.println("inst=" + inst.apply("abcd"));
        IntFunction<String[]> arrRef = String[]::new;
        System.out.println("arrLen=" + arrRef.apply(3).length);
        IntFunction<int[]> intArr = int[]::new;
        System.out.println("intArrLen=" + intArr.apply(4).length);
        Supplier<ArrayList<String>> listCtor = ArrayList<String>::new;
        System.out.println("listCtor=" + listCtor.get().size());
        Child c = new Child();
        System.out.println("superRef=" + c.useSuper());
        System.out.println("thisRef=" + c.useThis());
    }
}
