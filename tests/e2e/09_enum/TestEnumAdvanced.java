interface Describable {
    String describe();
}

// 枚举实现接口
enum Level implements Describable {
    LOW(1), MEDIUM(5), HIGH(10);

    private final int weight;

    Level(int weight) {
        this.weight = weight;
    }

    public int getWeight() {
        return weight;
    }

    @Override
    public String describe() {
        return name().toLowerCase() + "(" + weight + ")";
    }
}

// 逐常量抽象方法（constant-specific class body）
enum Operation {
    PLUS {
        int apply(int x, int y) {
            return x + y;
        }

        String symbol() {
            return "+";
        }
    },
    MINUS {
        int apply(int x, int y) {
            return x - y;
        }

        String symbol() {
            return "-";
        }
    },
    TIMES {
        int apply(int x, int y) {
            return x * y;
        }

        String symbol() {
            return "*";
        }
    };

    abstract int apply(int x, int y);

    abstract String symbol();
}

enum Status {
    NEW, ACTIVE, CLOSED;

    static Status parse(String s) {
        return Status.valueOf(s.toUpperCase());
    }

    Status next() {
        int idx = ordinal() + 1;
        if (idx >= values().length) {
            return NEW;
        }
        return values()[idx];
    }
}

public class TestEnumAdvanced {

    public static void main(String[] args) {
        // values() / valueOf / ordinal
        Level[] all = Level.values();
        for (Level l : all) {
            System.out.println(l.ordinal() + ":" + l.name() + ":" + l.getWeight());
        }
        System.out.println("valueOf HIGH=" + Level.valueOf("HIGH").getWeight());
        try {
            Level.valueOf("NOPE");
        } catch (IllegalArgumentException e) {
            System.out.println("bad valueOf -> " + e.getClass().getSimpleName());
        }

        // 枚举实现接口
        Describable d = Level.MEDIUM;
        System.out.println("describe=" + d.describe());

        // 逐常量方法（匿名子类字节码）
        for (Operation op : Operation.values()) {
            System.out.println(op.name() + " " + op.symbol() + " 6" + op.symbol() + "3=" + op.apply(6, 3));
        }

        // switch 枚举
        Level lv = Level.MEDIUM;
        switch (lv) {
            case LOW:
                System.out.println("low branch");
                break;
            case MEDIUM:
                System.out.println("medium branch");
                break;
            case HIGH:
                System.out.println("high branch");
                break;
            default:
                System.out.println("default branch");
        }

        // 比较与排序
        System.out.println("compareTo=" + Level.LOW.compareTo(Level.HIGH));
        System.out.println("equals=" + Level.HIGH.equals(Level.valueOf("HIGH")));
        System.out.println("==" + (Level.HIGH == Level.valueOf("HIGH")));

        // toString / hashCode / getDeclaringClass 规避 getClass
        System.out.println("toString=" + Status.ACTIVE);
        System.out.println("hashCode stable size=" + Status.values().length);

        // 自定义静态/实例方法
        System.out.println("parse(active)=" + Status.parse("active"));
        System.out.println("next chain=" + Status.NEW.next() + "->" + Status.NEW.next().next() + "->" + Status.CLOSED.next());

        // 作为 Map 语义替代：用 name 索引
        System.out.println("join=" + String.join(",", Level.LOW.name(), Level.HIGH.name()));

        System.out.println("done");
    }
}
