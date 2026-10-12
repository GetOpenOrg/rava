
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.exc.UnrecognizedPropertyException;

/**
 * jackson-databind pilot E3：特征开关（FAIL_ON_UNKNOWN_PROPERTIES 双态对照，E 批）。
 */
public class DatabindDeserMain {

    static class Strict {
        public String name;
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    static String outcome(ObjectMapper mapper, String json) {
        try {
            return "ok:" + mapper.readValue(json, Strict.class).name;
        } catch (UnrecognizedPropertyException e) {
            return "unrecognized:" + e.getPropertyName();
        } catch (Exception e) {
            return "other:" + e.getClass().getSimpleName();
        }
    }

    public static void main(String[] args) throws Exception {
        String withExtra = "{\"name\":\"甲\",\"extra\":1}";

        // 原生默认：FAIL_ON_UNKNOWN_PROPERTIES 开启 → 未知属性抛异常
        ObjectMapper strict = new ObjectMapper();
        check("default-strict", "unrecognized:extra", outcome(strict, withExtra));

        // Spring Boot 默认配置：关闭后容忍未知属性
        ObjectMapper lenient = new ObjectMapper()
                .disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES);
        check("boot-style-lenient", "ok:甲", outcome(lenient, withExtra));

        // FAIL_ON_NULL_FOR_PRIMITIVES
        String nullToInt = "{\"n\":null}";
        ObjectMapper strictPrim = new ObjectMapper()
                .enable(DeserializationFeature.FAIL_ON_NULL_FOR_PRIMITIVES);
        try {
            strictPrim.readValue(nullToInt, IntBox.class);
            System.out.println("FAIL null-primitive :: no error");
        } catch (Exception e) {
            System.out.println("PASS null-primitive :: " + e.getClass().getSimpleName());
        }
        IntBox lenientBox = new ObjectMapper().readValue(nullToInt, IntBox.class);
        check("null-primitive-lenient", 0, lenientBox.n);

        // USE_BIG_DECIMAL_FOR_FLOATS：数值精度面
        var reader = new ObjectMapper()
                .enable(DeserializationFeature.USE_BIG_DECIMAL_FOR_FLOATS)
                .readerFor(new com.fasterxml.jackson.core.type.TypeReference<
                        java.util.Map<String, Object>>() {
                });
        java.util.Map<String, Object> m = reader.readValue("{\"v\":0.1}");
        check("bigdecimal-for-float", "java.math.BigDecimal", m.get("v").getClass().getName());

        // ACCEPT_EMPTY_STRING_AS_NULL_OBJECT
        ObjectMapper emptyOk = new ObjectMapper()
                .enable(DeserializationFeature.ACCEPT_EMPTY_STRING_AS_NULL_OBJECT);
        Strict s = emptyOk.readValue("{\"name\":\"\"}", Strict.class);
        check("empty-string-kept", "", s.name); // String 场景空串仍是空串
        System.out.println("DONE DatabindDeserMain");
    }

    static class IntBox {
        public int n;
    }
}
