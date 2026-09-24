import java.lang.annotation.*;
import java.lang.reflect.Method;
import java.lang.reflect.Field;

@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD})
@interface Level {
    int value() default 1;
    String name() default "n";
}

@Level(value = 3, name = "cls")
public class TestAnnoReflect {

    @Level(value = 7, name = "m1")
    public void tagged() {}

    @Level
    public int field;

    public static void main(String[] args) throws Exception {
        Class<?> c = TestAnnoReflect.class;
        System.out.println("cls-present=" + c.isAnnotationPresent(Level.class));
        Level cl = c.getAnnotation(Level.class);
        System.out.println("cls-value=" + cl.value() + ",name=" + cl.name());
        Method m = c.getDeclaredMethod("tagged");
        System.out.println("m-present=" + m.isAnnotationPresent(Level.class));
        Level ml = m.getAnnotation(Level.class);
        System.out.println("m-value=" + ml.value() + ",name=" + ml.name());
        Method[] ms = c.getDeclaredMethods();
        int annotated = 0;
        for (Method mm : ms) {
            if (mm.isAnnotationPresent(Level.class)) annotated++;
        }
        System.out.println("annotated-methods=" + annotated);
        Field f = c.getDeclaredField("field");
        System.out.println("f-present=" + f.isAnnotationPresent(Level.class));
        Level fl = f.getAnnotation(Level.class);
        System.out.println("f-default=" + fl.value() + "," + fl.name());
    }
}
