import java.util.Arrays;

/**
 * SecurityManager.getClassContext 在接口 / 抽象方法派发下的帧口径：抽象方法没有方法体，不成帧，
 * 帧只属于实际执行的实现方法；接口 default 方法有方法体，以接口为声明类成帧。
 */
public class TestClassContextDispatch {
    @SuppressWarnings("removal")
    static class Probe extends SecurityManager {
        String context() {
            return Arrays.stream(getClassContext()).map(Class::getName)
                    .filter(n -> !n.contains("$$Lambda")).toList().toString();
        }
    }

    interface Action { String act(Probe p); }

    static class Named implements Action {
        public String act(Probe p) { return p.context(); }
    }

    interface WithDefault {
        default String viaDefault(Probe p) { return p.context(); }
    }

    static class UsesDefault implements WithDefault {}

    static class OverridesDefault implements WithDefault {
        public String viaDefault(Probe p) { return p.context(); }
    }

    static abstract class Base {
        abstract String run(Probe p);
        String template(Probe p) { return run(p); }
    }

    static class Derived extends Base {
        String run(Probe p) { return p.context(); }
    }

    interface Chain extends Action {}

    static class ChainImpl implements Chain {
        public String act(Probe p) { return new Named().act(p); }
    }

    public static void main(String[] args) {
        Probe p = new Probe();
        Action a = new Named();
        System.out.println("interface = " + a.act(p));
        WithDefault d = new UsesDefault();
        System.out.println("default = " + d.viaDefault(p));
        WithDefault o = new OverridesDefault();
        System.out.println("override default = " + o.viaDefault(p));
        Base b = new Derived();
        System.out.println("abstract = " + b.run(p));
        System.out.println("template = " + b.template(p));
        Chain c = new ChainImpl();
        System.out.println("sub-interface = " + c.act(p));
        Action l = q -> q.context();
        System.out.println("lambda = " + l.act(p));
    }
}
