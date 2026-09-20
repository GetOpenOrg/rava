interface Calc {
    int eval(int x);

    String name();
}

public class TestLocalClass {

    static Calc makeAdder(final int base) {
        // 方法内的局部类，捕获 effectively final 的局部变量
        class Adder implements Calc {
            @Override
            public int eval(int x) {
                return x + base;
            }

            @Override
            public String name() {
                return "adder(" + base + ")";
            }
        }
        return new Adder();
    }

    static Calc makeMultiplier(int factor) {
        class Multiplier implements Calc {
            private final int f;

            Multiplier(int f) {
                this.f = f;
            }

            @Override
            public int eval(int x) {
                return x * f;
            }

            @Override
            public String name() {
                return "mul(" + f + ")";
            }
        }
        return new Multiplier(factor);
    }

    static void localClassWithState() {
        class Accumulator {
            private int sum = 0;

            void add(int v) {
                sum += v;
            }

            int getSum() {
                return sum;
            }
        }
        Accumulator acc = new Accumulator();
        for (int i = 1; i <= 4; i++) {
            acc.add(i);
        }
        System.out.println("accumulated=" + acc.getSum());
    }

    static int useInLoop() {
        int total = 0;
        for (int i = 0; i < 3; i++) {
            // 每次进入循环体都重新定义同一个局部类（每次持有不同的捕获值）
            final int frozen = i;
            class Step implements Calc {
                @Override
                public int eval(int x) {
                    return x + frozen * 100;
                }

                @Override
                public String name() {
                    return "step" + frozen;
                }
            }
            Step s = new Step();
            total += s.eval(1);
            System.out.println(s.name() + " -> " + s.eval(1));
        }
        return total;
    }

    static void conditionalLocalClass(boolean flag) {
        if (flag) {
            class TrueCase implements Calc {
                @Override
                public int eval(int x) {
                    return x;
                }

                @Override
                public String name() {
                    return "true-case";
                }
            }
            System.out.println(new TrueCase().eval(9) + " " + new TrueCase().name());
        } else {
            class FalseCase implements Calc {
                @Override
                public int eval(int x) {
                    return -x;
                }

                @Override
                public String name() {
                    return "false-case";
                }
            }
            System.out.println(new FalseCase().eval(9) + " " + new FalseCase().name());
        }
    }

    public static void main(String[] args) {
        Calc add5 = makeAdder(5);
        Calc add100 = makeAdder(100);
        System.out.println(add5.name() + " " + add5.eval(3));
        System.out.println(add100.name() + " " + add100.eval(3));

        Calc mul = makeMultiplier(7);
        System.out.println(mul.name() + " " + mul.eval(6));

        localClassWithState();

        System.out.println("loop total=" + useInLoop());

        conditionalLocalClass(true);
        conditionalLocalClass(false);

        // 局部类与匿名内部类行为对照
        Calc anon = new Calc() {
            @Override
            public int eval(int x) {
                return x - 1;
            }

            @Override
            public String name() {
                return "anon";
            }
        };
        System.out.println(anon.name() + " " + anon.eval(10));

        System.out.println("done");
    }
}
