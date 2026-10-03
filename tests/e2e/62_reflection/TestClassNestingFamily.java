/**
 * 类嵌套/密封/记录家族：isMember/isLocal/isAnonymous/isSynthetic、getEnclosingClass/
 * Method、getDeclaredClasses、isSealed/getPermittedSubclasses、getRecordComponents
 * （spring 6 / hibernate 的类形态判定面）。
 */
public class TestClassNestingFamily {

    static class Member {
    }

    sealed interface Shape permits Circle, Square {
    }

    static final class Circle implements Shape {
    }

    static final class Square implements Shape {
    }

    record Point(int x, String label) {
    }

    public static void main(String[] args) throws Exception {
        System.out.println("member=" + Member.class.isMemberClass()
                + " enclosing=" + Member.class.getEnclosingClass().getSimpleName());

        Object local = makeLocal();
        System.out.println("local=" + local.getClass().isLocalClass()
                + " enclosing-method=" + (local.getClass().getEnclosingMethod() != null));

        Object anon = new Runnable() {
            @Override
            public void run() {
            }
        };
        System.out.println("anon=" + anon.getClass().isAnonymousClass()
                + " local-false=" + !anon.getClass().isLocalClass()
                + " enclosing-method=" + (anon.getClass().getEnclosingMethod() != null));

        // getDeclaredClasses 只含直接成员
        Class<?>[] members = TestClassNestingFamily.class.getDeclaredClasses();
        System.out.println("declared-count=" + members.length);

        // 密封家族
        System.out.println("sealed=" + Shape.class.isSealed());
        Class<?>[] permitted = Shape.class.getPermittedSubclasses();
        System.out.println("permitted=" + permitted.length
                + " first=" + permitted[0].getSimpleName());
        System.out.println("circle-not-sealed=" + !Circle.class.isSealed());

        // 记录组件
        System.out.println("record=" + Point.class.isRecord());
        java.lang.reflect.RecordComponent[] comps = Point.class.getRecordComponents();
        System.out.println("comp-count=" + comps.length);
        for (java.lang.reflect.RecordComponent rc : comps) {
            System.out.println("comp=" + rc.getName() + " type=" + rc.getType().getSimpleName()
                    + " accessor=" + rc.getAccessor().getName());
        }
        Object p = Point.class.getDeclaredConstructor(int.class, String.class)
                .newInstance(3, "pt");
        System.out.println("via-accessor=" + comps[0].getAccessor().invoke(p));
    }

    static Object makeLocal() {
        class Local {
        }
        return new Local();
    }
}
