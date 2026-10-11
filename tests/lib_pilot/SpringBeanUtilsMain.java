import java.beans.PropertyDescriptor;
import java.lang.reflect.Constructor;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import org.springframework.beans.BeanInstantiationException;
import org.springframework.beans.BeanUtils;
import org.springframework.beans.FatalBeanException;

/**
 * spring-beans 切片 sb1（矩阵 #12）：BeanUtils。
 *
 * 覆盖面：instantiateClass（无参 / 指定构造器带参 / 抽象类与接口的 BeanInstantiationException /
 * 构造器抛异常的包装）、getResolvableConstructor、copyProperties（全量 / ignoreProperties /
 * editable 限定 / 类型不匹配跳过）、findMethod / findDeclaredMethod /
 * findMethodWithMinimalParameters / resolveSignature、getPropertyDescriptors（按名排序）/
 * getPropertyDescriptor / findPropertyForMethod / getWriteMethodParameter、
 * isSimpleProperty / isSimpleValueType。无 cglib、无容器。
 */
public class SpringBeanUtilsMain {

    public static class Person {
        private String name;
        private int age;
        private boolean active;
        private List<String> tags = new ArrayList<>();
        private String nickname;

        public Person() {
            this("anonymous", 0);
        }

        public Person(String name, int age) {
            this.name = name;
            this.age = age;
        }

        public String getName() { return name; }
        public void setName(String name) { this.name = name; }
        public int getAge() { return age; }
        public void setAge(int age) { this.age = age; }
        public boolean isActive() { return active; }
        public void setActive(boolean active) { this.active = active; }
        public List<String> getTags() { return tags; }
        public void setTags(List<String> tags) { this.tags = tags; }
        public String getNickname() { return nickname; }
        public void setNickname(String nickname) { this.nickname = nickname; }

        public String greet(String other) { return "hi " + other + " from " + name; }
        public String greet(String other, int times) { return greet(other) + " x" + times; }

        @Override
        public String toString() {
            return "Person{name=" + name + ", age=" + age + ", active=" + active + ", tags=" + tags
                    + ", nickname=" + nickname + "}";
        }
    }

    /** 与 Person 同名属性但 age 为 long（类型不可赋值，copyProperties 跳过）。 */
    public static class PersonDto {
        private String name;
        private long age;
        private boolean active;
        private List<String> tags;
        private String extra = "dto-only";

        public String getName() { return name; }
        public void setName(String name) { this.name = name; }
        public long getAge() { return age; }
        public void setAge(long age) { this.age = age; }
        public boolean isActive() { return active; }
        public void setActive(boolean active) { this.active = active; }
        public List<String> getTags() { return tags; }
        public void setTags(List<String> tags) { this.tags = tags; }
        public String getExtra() { return extra; }
        public void setExtra(String extra) { this.extra = extra; }

        @Override
        public String toString() {
            return "PersonDto{name=" + name + ", age=" + age + ", active=" + active + ", tags=" + tags
                    + ", extra=" + extra + "}";
        }
    }

    /** copyProperties 的 editable 限定：只写 Named 声明的属性。 */
    public interface Named {
        String getName();
        void setName(String name);
    }

    public static class Employee extends Person implements Named {
        private String dept;

        public String getDept() { return dept; }
        public void setDept(String dept) { this.dept = dept; }

        @Override
        public String toString() {
            return "Employee{" + super.toString() + ", dept=" + dept + "}";
        }
    }

    public abstract static class Shape {
        public abstract double area();
    }

    public static class Exploding {
        public Exploding() {
            throw new IllegalStateException("boom in ctor");
        }
    }

    /** 唯一公开构造器带参：getResolvableConstructor 选它。 */
    public static class Point {
        final int x;
        final int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public String toString() { return "Point(" + x + "," + y + ")"; }
    }

    enum Color { RED, GREEN }

    static String sig(Method m) {
        if (m == null) {
            return "null";
        }
        StringBuilder sb = new StringBuilder(m.getDeclaringClass().getSimpleName()).append('.').append(m.getName()).append('(');
        Class<?>[] ps = m.getParameterTypes();
        for (int i = 0; i < ps.length; i++) {
            if (i > 0) {
                sb.append(',');
            }
            sb.append(ps[i].getSimpleName());
        }
        return sb.append(')').toString();
    }

    public static void main(String[] args) throws Exception {
        // ── instantiateClass ──
        Person p = BeanUtils.instantiateClass(Person.class);
        System.out.println("instantiateClass(Person) = " + p);
        Constructor<Person> ctor = Person.class.getConstructor(String.class, int.class);
        Person q = BeanUtils.instantiateClass(ctor, "Alice", 30);
        System.out.println("instantiateClass(ctor,Alice,30) = " + q);
        Constructor<Point> pc = BeanUtils.getResolvableConstructor(Point.class);
        System.out.println("getResolvableConstructor(Point) params=" + pc.getParameterCount());
        System.out.println("instantiateClass(Point ctor) = " + BeanUtils.instantiateClass(pc, 3, 4));
        StringBuilder sb = BeanUtils.instantiateClass(StringBuilder.class);
        System.out.println("instantiateClass(StringBuilder).length = " + sb.append("ok").length());

        for (Class<?> bad : Arrays.asList(Shape.class, Runnable.class, Exploding.class)) {
            try {
                BeanUtils.instantiateClass(bad);
                System.out.println("instantiateClass(" + bad.getSimpleName() + ") unexpectedly succeeded");
            } catch (BeanInstantiationException e) {
                Throwable cause = e.getCause();
                System.out.println("instantiateClass(" + bad.getSimpleName() + ") -> BeanInstantiationException beanClass="
                        + e.getBeanClass().getSimpleName() + " msg=" + e.getMessage()
                        + " cause=" + (cause == null ? "null" : cause.getClass().getSimpleName() + ":" + cause.getMessage()));
            }
        }
        try {
            BeanUtils.instantiateClass(ctor, "too", 1, 2);
        } catch (BeanInstantiationException e) {
            System.out.println("instantiateClass(wrong arg count) -> " + e.getClass().getSimpleName()
                    + " isFatal=" + (e instanceof FatalBeanException));
        }

        // ── copyProperties ──
        Person src = new Person("Bob", 41);
        src.setActive(true);
        src.setTags(new ArrayList<>(Arrays.asList("a", "b")));
        src.setNickname("bobby");
        PersonDto dto = new PersonDto();
        BeanUtils.copyProperties(src, dto);
        System.out.println("copyProperties(Person->Dto) = " + dto);
        System.out.println("tags shared = " + (dto.getTags() == src.getTags()));

        Person target = new Person();
        BeanUtils.copyProperties(src, target, "age", "nickname");
        System.out.println("copyProperties(ignore age,nickname) = " + target);

        Employee emp = new Employee();
        emp.setDept("R&D");
        BeanUtils.copyProperties(src, emp, Named.class);
        System.out.println("copyProperties(editable=Named) = " + emp);

        Person back = new Person();
        BeanUtils.copyProperties(dto, back);
        System.out.println("copyProperties(Dto->Person) = " + back);

        // ── findMethod 族 ──
        System.out.println("findMethod(greet,String) = " + sig(BeanUtils.findMethod(Person.class, "greet", String.class)));
        System.out.println("findMethod(greet,String,int) = " + sig(BeanUtils.findMethod(Person.class, "greet", String.class, int.class)));
        System.out.println("findMethod(greet,int) = " + sig(BeanUtils.findMethod(Person.class, "greet", int.class)));
        System.out.println("findMethod(Employee,getName) = " + sig(BeanUtils.findMethod(Employee.class, "getName")));
        System.out.println("findMethod(Object,toString) = " + sig(BeanUtils.findMethod(Object.class, "toString")));
        System.out.println("findDeclaredMethod(Employee,getDept) = " + sig(BeanUtils.findDeclaredMethod(Employee.class, "getDept")));
        System.out.println("findDeclaredMethod(Employee,getAge) = " + sig(BeanUtils.findDeclaredMethod(Employee.class, "getAge")));
        System.out.println("findMethodWithMinimalParameters(greet) = " + sig(BeanUtils.findMethodWithMinimalParameters(Person.class, "greet")));
        System.out.println("findMethodWithMinimalParameters(nope) = " + sig(BeanUtils.findMethodWithMinimalParameters(Person.class, "nope")));
        System.out.println("resolveSignature(greet(java.lang.String,int)) = "
                + sig(BeanUtils.resolveSignature("greet(java.lang.String,int)", Person.class)));
        System.out.println("resolveSignature(getName) = " + sig(BeanUtils.resolveSignature("getName", Person.class)));
        Method greet = Person.class.getMethod("greet", String.class, int.class);
        System.out.println("invoke found = " + greet.invoke(q, "Carol", 2));

        // ── 属性描述符 ──
        List<String> props = new ArrayList<>();
        for (PropertyDescriptor pd : BeanUtils.getPropertyDescriptors(Employee.class)) {
            props.add(pd.getName() + ":" + pd.getPropertyType().getSimpleName()
                    + (pd.getReadMethod() != null ? ":r" : "") + (pd.getWriteMethod() != null ? ":w" : ""));
        }
        props.sort(null);
        System.out.println("getPropertyDescriptors(Employee) = " + props);
        PropertyDescriptor agePd = BeanUtils.getPropertyDescriptor(Person.class, "age");
        System.out.println("getPropertyDescriptor(age) read=" + sig(agePd.getReadMethod()) + " write=" + sig(agePd.getWriteMethod()));
        System.out.println("getPropertyDescriptor(missing) = " + BeanUtils.getPropertyDescriptor(Person.class, "missing"));
        PropertyDescriptor found = BeanUtils.findPropertyForMethod(Person.class.getMethod("isActive"));
        System.out.println("findPropertyForMethod(isActive) = " + (found == null ? "null" : found.getName()));
        System.out.println("getWriteMethodParameter(tags) type = "
                + BeanUtils.getWriteMethodParameter(BeanUtils.getPropertyDescriptor(Person.class, "tags")).getGenericParameterType().getTypeName());

        // ── 简单类型判定 ──
        for (Class<?> c : Arrays.asList(int.class, Integer.class, String.class, Color.class, Class.class,
                java.util.Date.class, java.net.URI.class, Person.class, int[].class, String[].class, Person[].class,
                Object.class, void.class, Void.class, CharSequence.class, java.util.Locale.class)) {
            System.out.println("simple " + c.getSimpleName() + " property=" + BeanUtils.isSimpleProperty(c)
                    + " value=" + BeanUtils.isSimpleValueType(c));
        }
    }
}
