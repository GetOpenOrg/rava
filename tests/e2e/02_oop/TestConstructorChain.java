class ShapeBase {
    String name;
    int sides;

    ShapeBase() {
        this("unnamed", 0);
        System.out.println("ShapeBase no-arg tail");
    }

    ShapeBase(String name, int sides) {
        this.name = name;
        this.sides = sides;
        System.out.println("ShapeBase(name,sides) " + name + " " + sides);
        describe();
    }

    void describe() {
        System.out.println("  base describe: " + name + "/" + sides);
    }
}

class RectDerived extends ShapeBase {
    int w;
    int h = 5;

    RectDerived() {
        this("rect", 4, 3, 2);
        System.out.println("RectDerived no-arg tail");
    }

    RectDerived(String name, int sides, int w, int h) {
        super(name, sides);
        this.w = w;
        this.h = h;
        System.out.println("RectDerived(w,h) " + w + " " + h + " h-field=" + this.h);
    }

    @Override
    void describe() {
        System.out.println("  derived describe: " + name + " sides=" + sides + " w=" + w + " h=" + h);
    }
}

class InitInCtor {
    int a = 1;
    int b;

    InitInCtor() {
        b = a * 10;
        System.out.println("a=" + a + " b=" + b);
        a = 100;
        System.out.println("after mutate a=" + a + " b=" + b);
    }
}

class FieldInitOrder {
    int x = compute("x");
    int y = compute("y");

    static int compute(String tag) {
        System.out.println("compute " + tag);
        return tag.equals("x") ? 1 : 2;
    }

    FieldInitOrder() {
        System.out.println("ctor sees x=" + x + " y=" + y);
    }
}

public class TestConstructorChain {

    public static void main(String[] args) {
        System.out.println("--- no-arg chain ---");
        ShapeBase s = new ShapeBase();
        System.out.println("name=" + s.name + " sides=" + s.sides);

        System.out.println("--- super() 链 + 虚方法回调 ---");
        RectDerived r = new RectDerived();
        System.out.println("w=" + r.w + " h=" + r.h + " name=" + r.name);

        System.out.println("--- 构造器内字段初始化顺序 ---");
        new InitInCtor();

        System.out.println("--- 字段按文本顺序初始化 ---");
        new FieldInitOrder();

        System.out.println("done");
    }
}
