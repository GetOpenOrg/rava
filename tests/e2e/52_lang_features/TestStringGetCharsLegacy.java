import java.lang.reflect.InvocationTargetException;
import java.util.Calendar;
import java.util.Date;
import java.util.TimeZone;

/**
 * 杂项遗留通道（方法级实测补全）：String.getChars（16 jar）、
 * InvocationTargetException.getTargetException（26 jar，getCause 的老 API）、
 * Class.getPackage（22 jar）、System.getSecurityManager 恒 null（19 jar）、
 * Calendar.setTime（16 jar）。
 */
public class TestStringGetCharsLegacy {

    public static void main(String[] args) throws Exception {
        // getChars 区间拷贝与三处越界
        String s = "abcdef";
        char[] dst = new char[4];
        s.getChars(1, 5, dst, 0);
        System.out.println("copied=" + new String(dst));
        s.getChars(2, 4, dst, 2);
        System.out.println("offset-copy=" + new String(dst));
        try {
            s.getChars(0, 99, dst, 0);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("range-ex=" + e.getClass().getSimpleName());
        }
        try {
            s.getChars(0, 3, dst, 3);   // dst 空间不足
        } catch (IndexOutOfBoundsException e) {
            System.out.println("dst-ex=" + e.getClass().getSimpleName());
        }
        System.out.println("tochararray-eq=" + java.util.Arrays.equals(s.toCharArray(),
                new char[] { 'a', 'b', 'c', 'd', 'e', 'f' }));

        // ITE 老式解包 API 与 getCause 同一性
        try {
            throwMakeItThrow();
        } catch (InvocationTargetException ite) {
            System.out.println("legacy-target=" + ite.getTargetException().getMessage()
                    + " same-as-cause=" + (ite.getTargetException() == ite.getCause()));
        }

        // Class.getPackage 形态
        Package pkg = TestStringGetCharsLegacy.class.getPackage();
        System.out.println("pkg-null=" + (pkg == null) + " name-defined="
                + (pkg == null || pkg.getName() != null));
        System.out.println("string-pkg=" + String.class.getPackage().getName());

        // System.getSecurityManager：JDK17+ 恒 null（框架的存在性检查路径）
        System.out.println("security-null=" + (System.getSecurityManager() == null));

        // Calendar.setTime 与 getTime 往返
        Calendar c = Calendar.getInstance(TimeZone.getTimeZone("GMT"));
        c.clear();
        c.set(2026, Calendar.OCTOBER, 3);
        Date d = c.getTime();
        System.out.println("epoch-pos=" + (d.getTime() > 0));
        Calendar c2 = Calendar.getInstance(TimeZone.getTimeZone("GMT"));
        c2.setTime(d);
        System.out.println("roundtrip-md=" + (c2.get(Calendar.MONTH) + 1) + "-" + c2.get(Calendar.DAY_OF_MONTH));
    }

    static void throwMakeItThrow() throws InvocationTargetException {
        throw new InvocationTargetException(new IllegalStateException("via-legacy"));
    }
}
