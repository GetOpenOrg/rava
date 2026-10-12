
import java.util.List;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * jackson-databind pilot E1：注解驱动的序列化（排 jackson-core green 后的增量件）。
 */
public class DatabindAnnotationMain {

    @JsonIgnoreProperties(ignoreUnknown = true)
    static class UserDTO {
        @JsonProperty("user_name")
        public String userName;

        @JsonIgnore
        public String password = "secret";

        public String role = "member";
    }

    @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "type")
    @JsonSubTypes({
            @JsonSubTypes.Type(value = Circle.class, name = "circle"),
            @JsonSubTypes.Type(value = Square.class, name = "square")
    })
    static abstract class Shape {
    }

    static class Circle extends Shape {
        public double radius = 2.0;
    }

    static class Square extends Shape {
        public double side = 3.0;
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        ObjectMapper mapper = new ObjectMapper();

        // @JsonProperty 改名 + @JsonIgnore 忽略
        UserDTO user = new UserDTO();
        user.userName = "张三";
        String json = mapper.writeValueAsString(user);
        check("rename-and-ignore", "{\"role\":\"member\",\"user_name\":\"张三\"}", json); // 公有字段按名排序输出

        // @JsonIgnoreProperties：未知属性不报错
        UserDTO back = mapper.readValue(
                "{\"user_name\":\"李四\",\"extra\":1,\"role\":\"vip\"}", UserDTO.class);
        check("unknown-tolerated", "李四/vip", back.userName + "/" + back.role);

        // 多态：@JsonTypeInfo 自动带 type 标签
        String circleJson = mapper.writeValueAsString((Shape) new Circle());
        check("polymorphic-ser", "{\"type\":\"circle\",\"radius\":2.0}", circleJson);
        Shape shape = mapper.readValue("{\"type\":\"square\",\"side\":4.0}", Shape.class);
        check("polymorphic-deser", "Square(4.0)",
                shape.getClass().getSimpleName() + "(" + ((Square) shape).side + ")");
        check("polymorphic-deser-type", Square.class, shape.getClass());
        System.out.println("DONE DatabindAnnotationMain");
    }
}
