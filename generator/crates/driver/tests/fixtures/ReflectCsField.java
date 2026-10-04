import java.lang.reflect.Field;
import java.lang.reflect.Method;

// 反射调用调用者敏感方法读私有静态字段：Method.invoke(Field.get)
public class ReflectCsField {
    private static String secret = "s";

    public static void main(String[] args) throws Exception {
        Field f = ReflectCsField.class.getDeclaredField("secret");
        Method get = Field.class.getMethod("get", Object.class);
        System.out.println(get.invoke(f, (Object) null));
    }
}
