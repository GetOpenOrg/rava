import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Constructor;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collection;
import java.util.LinkedList;

/**
 * 构造器查找的目标是 JDK 类（非用户类）：
 *   - MethodHandles.Lookup.findConstructor（newInvokeSpecial 句柄：先无构造分配、再调 <init>）；
 *   - Class.getConstructor / getDeclaredConstructor + Constructor.newInstance；
 *   - Class 值来自类字面量、条件合流（值集含两个类）、静态字段；
 *   - JDK 内部的 findConstructor：FileChannel.map 经 ExtendedMapMode.<clinit> 以私有查找构造 MapMode；
 *   - 数组类、接口上的构造器查找抛 NoSuchMethodException；抽象类的 findConstructor 句柄调用抛 InstantiationException。
 */
public class TestJdkConstructorLookup {
    static final Class<?> HELD = StringBuilder.class;

    public static void main(String[] args) throws Throwable {
        // findConstructor：JDK 类的 public 构造器
        MethodHandle sbNew = MethodHandles.publicLookup()
                .findConstructor(StringBuilder.class, MethodType.methodType(void.class, String.class));
        StringBuilder sb = (StringBuilder) sbNew.invoke("handle");
        sb.append("-ctor");
        System.out.println("findConstructor StringBuilder = " + sb);

        MethodHandle listNew = MethodHandles.lookup()
                .findConstructor(ArrayList.class, MethodType.methodType(void.class, int.class));
        @SuppressWarnings("unchecked")
        ArrayList<String> list = (ArrayList<String>) listNew.invoke(4);
        list.add("a");
        list.add("b");
        System.out.println("findConstructor ArrayList = " + list + " size " + list.size());

        // getDeclaredConstructor + newInstance：JDK 类
        Constructor<ArrayList> ac = ArrayList.class.getDeclaredConstructor(Collection.class);
        ArrayList<?> copy = ac.newInstance(list);
        System.out.println("getDeclaredConstructor ArrayList = " + copy);

        // 条件合流：Class 值集含两个 JDK 类
        for (int i = 0; i < 2; i++) {
            Class<?> c = (i == 0) ? LinkedList.class : ArrayDeque.class;
            Object o = c.getConstructor().newInstance();
            System.out.println("getConstructor " + c.getSimpleName() + " -> " + o.getClass().getSimpleName() + " " + o);
        }

        // 静态字段持有的 Class
        Object held = HELD.getConstructor(String.class).newInstance("held");
        System.out.println("static field class = " + held);

        // JDK 内部的私有构造器查找：FileChannel.map → isSync → ExtendedMapMode.<clinit>
        Path tmp = Files.createTempFile("ctorlookup", ".txt");
        try {
            Files.writeString(tmp, "mapped bytes", StandardCharsets.UTF_8);
            try (FileChannel ch = FileChannel.open(tmp, StandardOpenOption.READ)) {
                MappedByteBuffer mb = ch.map(FileChannel.MapMode.READ_ONLY, 0, ch.size());
                byte[] b = new byte[mb.remaining()];
                mb.get(b);
                System.out.println("map READ_ONLY = " + new String(b, StandardCharsets.UTF_8) + " / " + FileChannel.MapMode.READ_ONLY);
            }
        } finally {
            Files.deleteIfExists(tmp);
        }

        // 查找失败的边界
        try {
            int[].class.getDeclaredConstructor();
            System.out.println("array ctor found");
        } catch (NoSuchMethodException e) {
            System.out.println("array ctor -> NoSuchMethodException");
        }
        try {
            Runnable.class.getConstructor();
            System.out.println("interface ctor found");
        } catch (NoSuchMethodException e) {
            System.out.println("interface ctor -> NoSuchMethodException");
        }
        try {
            MethodHandles.publicLookup().findConstructor(Number.class, MethodType.methodType(void.class)).invoke();
            System.out.println("abstract ctor invoked");
        } catch (IllegalAccessException | InstantiationException e) {
            System.out.println("abstract ctor -> " + e.getClass().getSimpleName());
        }
    }
}
