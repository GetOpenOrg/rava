class ShadowParent {
    String value = "parent-value";
    int number = 1;

    String getValue() {
        return value;
    }

    int getNumber() {
        return number;
    }

    void printParentView() {
        System.out.println("parent view: value=" + value + " number=" + number);
    }
}

class ShadowChild extends ShadowParent {
    String value = "child-value";   // 隐藏父类同名字段
    int number = 2;

    String getChildValue() {
        return value;
    }

    String getSuperValue() {
        return super.value;
    }

    void printChildView() {
        System.out.println("child view: value=" + value + " number=" + number);
        System.out.println("via super: value=" + super.value + " number=" + super.number);
    }

    void localShadow() {
        String value = "local-value";   // 局部变量遮蔽字段
        System.out.println("local=" + value + " field=" + this.value + " super=" + super.value);
        int number = 3;
        System.out.println("local number=" + number + " this.number=" + this.number);
    }
}

public class TestFieldShadow {

    static String field = "static-field";

    static void staticShadow(String field) {
        System.out.println("param=" + field + " static=" + TestFieldShadow.field);
    }

    String instance;

    TestFieldShadow(String instance) {
        // 构造器参数与字段同名：需要 this.x = x 显式赋值
        this.instance = instance;
        String instance2 = instance + "-local";
        System.out.println("ctor: this=" + this.instance + " local=" + instance2);
    }

    public static void main(String[] args) {
        ShadowChild child = new ShadowChild();

        // 字段访问是静态绑定的：按引用类型解析
        System.out.println("child.getValue()=" + child.getValue());      // 父类方法 → 父类字段
        System.out.println("child.getChildValue()=" + child.getChildValue());
        System.out.println("child.getSuperValue()=" + child.getSuperValue());

        child.printChildView();
        child.printParentView();
        child.localShadow();

        // 通过父类引用访问 → 看到父类字段
        ShadowParent asParent = child;
        System.out.println("as parent value=" + asParent.value + " number=" + asParent.number);
        System.out.println("as child value=" + ((ShadowChild) asParent).value);

        // 方法多态不受影响
        ShadowParent poly = child;
        System.out.println("poly getValue()=" + poly.getValue());

        staticShadow("param-value");

        // 实例字段与局部变量同名
        TestFieldShadow obj = new TestFieldShadow("outer-instance");
        String instance = "shadowing-local";
        System.out.println("local=" + instance + " field=" + obj.instance);

        System.out.println("done");
    }
}
