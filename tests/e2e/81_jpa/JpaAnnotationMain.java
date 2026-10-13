
import java.lang.reflect.Field;
import java.util.TreeSet;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.EnumType;
import jakarta.persistence.Enumerated;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

/**
 * JPA pilot C9：注解元数据面（免 Provider，W2 #7 无状态件）。
 */
public class JpaAnnotationMain {

    @Entity(name = "account")
    @Table(name = "t_account", schema = "billing",
            uniqueConstraints = @jakarta.persistence.UniqueConstraint(
                    name = "uk_account_no", columnNames = {"account_no"}))
    static class Account {

        @Id
        @GeneratedValue(strategy = GenerationType.IDENTITY)
        private Long id;

        @Column(name = "account_no", nullable = false, length = 32, unique = true)
        private String accountNo;

        @Enumerated(EnumType.STRING)
        @Column(length = 16)
        private Status status;

        @jakarta.persistence.Version
        private long version;

        @jakarta.persistence.Transient
        private String cachedLabel;
    }

    enum Status {
        ACTIVE, FROZEN, CLOSED
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        // @Entity / @Table
        Entity entity = Account.class.getAnnotation(Entity.class);
        check("entity-name", "account", entity.name());
        Table table = Account.class.getAnnotation(Table.class);
        check("table-name", "t_account", table.name());
        check("table-schema", "billing", table.schema());
        check("unique-constraint", "uk_account_no",
                table.uniqueConstraints()[0].name());
        check("unique-columns", "[account_no]",
                java.util.Arrays.toString(table.uniqueConstraints()[0].columnNames()));

        // @Id + @GeneratedValue
        Field id = Account.class.getDeclaredField("id");
        check("id-present", true, id.isAnnotationPresent(Id.class));
        check("id-strategy", GenerationType.IDENTITY,
                id.getAnnotation(GeneratedValue.class).strategy());

        // @Column 属性
        Column col = Account.class.getDeclaredField("accountNo").getAnnotation(Column.class);
        check("column-name", "account_no", col.name());
        check("column-nullable", false, col.nullable());
        check("column-length", 32, col.length());
        check("column-unique", true, col.unique());

        // @Enumerated
        check("enum-type", EnumType.STRING,
                Account.class.getDeclaredField("status")
                        .getAnnotation(Enumerated.class).value());

        // @Version / @Transient
        check("version-present", true,
                Account.class.getDeclaredField("version")
                        .isAnnotationPresent(jakarta.persistence.Version.class));
        check("transient-present", true,
                Account.class.getDeclaredField("cachedLabel")
                        .isAnnotationPresent(jakarta.persistence.Transient.class));

        // 注解字段全集（TreeSet 排序）
        TreeSet<String> annotated = new TreeSet<>();
        for (Field f : Account.class.getDeclaredFields()) {
            if (f.getAnnotations().length > 0) {
                annotated.add(f.getName());
            }
        }
        check("annotated-fields", "[accountNo, cachedLabel, id, status, version]",
                annotated.toString());
        System.out.println("DONE JpaAnnotationMain");
    }
}
