
import java.util.Arrays;
import java.util.TreeSet;

import jakarta.persistence.CascadeType;
import jakarta.persistence.FetchType;
import jakarta.persistence.LockModeType;
import jakarta.persistence.PessimisticLockScope;
import jakarta.persistence.spi.PersistenceUnitTransactionType;

/**
 * JPA pilot C10：枚举与公共语义面（免 Provider）。
 */
public class JpaEnumMetaMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        // FetchType：两种取载语义
        check("fetch-values", "[LAZY, EAGER]",
                Arrays.toString(FetchType.values()));
        check("fetch-default-hint", FetchType.EAGER, FetchType.valueOf("EAGER"));

        // CascadeType 全集
        check("cascade-values", "[ALL, PERSIST, MERGE, REMOVE, REFRESH, DETACH]",
                Arrays.toString(CascadeType.values()));

        // LockModeType 全集（含乐观/悲观族）
        check("lock-values", "[READ, WRITE, OPTIMISTIC, OPTIMISTIC_FORCE_INCREMENT, "
                        + "PESSIMISTIC_READ, PESSIMISTIC_WRITE, PESSIMISTIC_FORCE_INCREMENT, NONE]",
                Arrays.toString(LockModeType.values()));

        // PessimisticLockScope 常量
        check("plscope-normal", PessimisticLockScope.NORMAL,
                jakarta.persistence.PessimisticLockScope.NORMAL);
        check("plscope-extended", PessimisticLockScope.EXTENDED,
                jakarta.persistence.PessimisticLockScope.EXTENDED);

        // 事务类型（spi 包）
        check("txtype-values", "[JTA, RESOURCE_LOCAL]",
                Arrays.toString(PersistenceUnitTransactionType.values()));

        // 拼接全集（字节面稳定）
        TreeSet<String> all = new TreeSet<>();
        for (FetchType f : FetchType.values()) {
            all.add("Cascade." + f);
        }
        for (CascadeType c : CascadeType.values()) {
            all.add("Cascade." + c);
        } // 统一前缀避免混排
        check("combined", "[Cascade.ALL, Cascade.DETACH, Cascade.EAGER, Cascade.LAZY, "
                        + "Cascade.MERGE, Cascade.PERSIST, Cascade.REFRESH, Cascade.REMOVE]",
                all.toString());
        System.out.println("DONE JpaEnumMetaMain");
    }
}
