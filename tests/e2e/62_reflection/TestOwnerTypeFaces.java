import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.util.List;
import java.util.Map;

/**
 * ParameterizedType.getOwnerType（方法级实测：10 jar，此前零覆盖）：
 * 成员类型（Map.Entry）有 owner，顶层泛型无 owner（null）。
 */
public class TestOwnerTypeFaces {

    interface EntryImpl extends Map.Entry<String, Integer> {
    }

    static class WithField {
        Map.Entry<String, Long> entryField;
        List<String> listField;
    }

    public static void main(String[] args) throws Exception {
        Type[] ifaces = EntryImpl.class.getGenericInterfaces();
        System.out.println("iface-count=" + ifaces.length
                + " parameterized=" + (ifaces[0] instanceof ParameterizedType));
        ParameterizedType pt = (ParameterizedType) ifaces[0];
        System.out.println("raw=" + pt.getRawType().getTypeName());
        System.out.println("owner-null=" + (pt.getOwnerType() == null));

        // 真正带 owner 的形态：以成员类型 Map.Entry 直接参数化时 owner = Map
        Type field = WithField.class.getDeclaredField("entryField").getGenericType();
        System.out.println("field-type=" + field.getTypeName()
                + " parameterized=" + (field instanceof ParameterizedType));
        ParameterizedType fpt = (ParameterizedType) field;
        System.out.println("field-owner=" + fpt.getOwnerType().getTypeName()
                + " owner-is-map=" + (fpt.getOwnerType() == Map.class));

        // 顶层泛型无 owner
        Type listT = WithField.class.getDeclaredField("listField").getGenericType();
        System.out.println("list-owner-null="
                + (((ParameterizedType) listT).getOwnerType() == null));

        // 泛型接口作为超接口的实参递归
        System.out.println("entry-args=" + pt.getActualTypeArguments()[0].getTypeName()
                + "," + pt.getActualTypeArguments()[1].getTypeName());
    }
}
