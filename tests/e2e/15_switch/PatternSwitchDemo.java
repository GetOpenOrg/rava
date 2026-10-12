
import java.util.List;

/**
 * 文章的“改造后”目标：把 if/else 范围判断写成 switch 规则表。
 *
 * 版本说明（重要）：
 * - 文章的主角是 Java 27 的 Preview 特性 “Primitive Types in Patterns, instanceof,
 *   and switch”（第 5 次 Preview），可写 case long p when p < 100_000L
 * - 本项目基线是 JDK 21（生产 LTS），原生 long 选择器在 21 下无法编译，
 *   因此这里用 Java 21 已【定型】的引用类型模式 switch 等价实现：case Long p when ...
 *   两者语法几乎一致，Java 27 正式化后只需把 Long 换成 long（去掉装箱）
 * - 生产启用 Java 27 写法需要：javac --release 27 --enable-preview；
 *   Maven 需给 compiler 和 surefire 都配置 --enable-preview
 * - 文章的忠告：switch 能用 double 不代表金额该用 double，
 *   钱继续用 long 分或 BigDecimal，语言表达力和精度是两回事
 *
 * 输出确定性说明：与 IfElseRiskDemo 用同一组固定边界输入做等价校验。
 */
public class PatternSwitchDemo {

    /** 风控分级：switch 规则表（Java 21 引用类型模式版本）。 */
    static RiskLevel calculateRisk(Long amountFen) {
        return switch (amountFen) {
            case null -> throw new IllegalArgumentException("amount must not be null");
            case Long amount when amount < 0 ->  // Java 27 后可写成 case long amount（去掉装箱）
                    throw new IllegalArgumentException("amount must not be negative");
            case Long amount when amount < 10_000L -> RiskLevel.LOW;
            case Long amount when amount < 100_000L -> RiskLevel.MEDIUM;
            case Long amount when amount < 1_000_000L -> RiskLevel.HIGH;
            default -> RiskLevel.VERY_HIGH;
        };
    }

    static String memberLevel(Long points) {
        return switch (points) {
            case null -> "UNKNOWN";
            case Long p when p < 1_000L -> "NORMAL";
            case Long p when p < 10_000L -> "SILVER";
            case Long p when p < 100_000L -> "GOLD";
            default -> "VIP";
        };
    }

    static UploadMode resolveUploadMode(Long fileSize) {
        return switch (fileSize) {
            case null -> throw new IllegalArgumentException("file size must not be null");
            case Long size when size < 0 -> throw new IllegalArgumentException("invalid file size");
            case Long size when size <= 10 * 1024 * 1024L -> UploadMode.DIRECT;
            case Long size when size <= 500 * 1024 * 1024L -> UploadMode.MULTIPART;
            default -> UploadMode.LARGE_FILE;
        };
    }

    public static void main(String[] args) {
        List<Long> amounts = List.of(0L, 9_900L, 9_999L, 10_000L, 99_999L,
                100_000L, 999_999L, 1_000_000L);
        List<Long> points = List.of(0L, 999L, 1_000L, 9_999L, 10_000L, 99_999L, 100_000L);
        List<Long> sizes = List.of(0L, 10 * 1024 * 1024L, 10 * 1024 * 1024L + 1,
                500 * 1024 * 1024L, 500 * 1024 * 1024L + 1);

        System.out.println("[风控分级 switch 规则表]");
        amounts.forEach(a -> System.out.printf("  amountFen=%-9d -> %s%n", a, calculateRisk(a)));

        System.out.println("\n[会员等级 switch 规则表]");
        points.forEach(p -> System.out.printf("  points=%-8d -> %s%n", p, memberLevel(p)));

        System.out.println("\n[上传策略 switch 规则表]");
        sizes.forEach(s -> System.out.printf("  fileSize=%-11d -> %s%n", s, resolveUploadMode(s)));

        // 异常分支：负数与 null
        System.out.println("\n[异常分支]");
        for (Long bad : java.util.Arrays.asList(-1L, null)) { // Arrays.asList 才允许 null 元素
            String msg;
            try {
                calculateRisk(bad);
                msg = "未抛异常(错误)";
            } catch (IllegalArgumentException e) {
                msg = e.getMessage();
            }
            System.out.println("  input=" + bad + " -> " + msg);
        }

        // 与 if/else 旧实现做等价校验（同一批边界输入，两者必须给出相同结果）
        int checked = 0;
        for (long a : amounts) {
            if (IfElseRiskDemo.calculateRisk(a) == calculateRisk(a)) {
                checked++;
            }
        }
        for (long p : points) {
            if (IfElseRiskDemo.memberLevel(p).equals(memberLevel(p))) {
                checked++;
            }
        }
        for (long s : sizes) {
            if (IfElseRiskDemo.resolveUploadMode(s) == resolveUploadMode(s)) {
                checked++;
            }
        }
        System.out.println("\n等价校验: " + checked + "/" + (amounts.size() + points.size() + sizes.size())
                + " 个边界输入与 if/else 实现结果一致");
    }
}

/** 订单风控等级（金额单位：分）——自上游 RiskLevel.java 并入（e2e 单文件口径）。 */
enum RiskLevel {
    LOW, MEDIUM, HIGH, VERY_HIGH
}

/** 文件上传策略——自上游 UploadMode.java 并入（e2e 单文件口径）。 */
enum UploadMode {
    DIRECT, MULTIPART, LARGE_FILE
}


/**
 * 文章里的“改造前”写法：long 范围判断只能堆 if/else（Java 27 之前 switch 不支持 long）。
 * 三个业务规则：订单风控分级、会员等级、上传策略。
 *
 * 输出确定性说明：测试输入为固定边界值，无随机/时间成分。
 */
class IfElseRiskDemo {

    /** 订单状态：int 常量 switch（一直都支持，文章开头的例子）。 */
    static String statusText(int status) {
        return switch (status) {
            case 0 -> "待支付";
            case 1 -> "已支付";
            default -> "未知";
        };
    }

    /** 风控分级：if/else 链，规则越加越长。 */
    static RiskLevel calculateRisk(long amountFen) {
        if (amountFen < 0) {
            throw new IllegalArgumentException("amount must not be negative");
        } else if (amountFen < 10_000L) {
            return RiskLevel.LOW;
        } else if (amountFen < 100_000L) {
            return RiskLevel.MEDIUM;
        } else if (amountFen < 1_000_000L) {
            return RiskLevel.HIGH;
        } else {
            return RiskLevel.VERY_HIGH;
        }
    }

    /** 会员等级：连续多个独立 if。 */
    static String memberLevel(long points) {
        if (points < 1_000L) {
            return "NORMAL";
        }
        if (points < 10_000L) {
            return "SILVER";
        }
        if (points < 100_000L) {
            return "GOLD";
        }
        return "VIP";
    }

    /** 上传策略：按文件大小选模式。 */
    static UploadMode resolveUploadMode(long fileSize) {
        if (fileSize <= 10 * 1024 * 1024L) {
            return UploadMode.DIRECT;
        } else if (fileSize <= 500 * 1024 * 1024L) {
            return UploadMode.MULTIPART;
        } else {
            return UploadMode.LARGE_FILE;
        }
    }

    public static void main(String[] args) {
        System.out.println("[int 常量 switch]");
        for (int status : new int[]{0, 1, 9}) {
            System.out.println("  status=" + status + " -> " + statusText(status));
        }

        System.out.println("\n[风控分级 if/else]（金额单位:分）");
        long[] amounts = {-1L, 0L, 9_900L, 9_999L, 10_000L, 99_999L,
                100_000L, 999_999L, 1_000_000L};
        for (long amount : amounts) {
            String result = amount < 0 ? "IllegalArgumentException" : calculateRisk(amount).name();
            System.out.printf("  amountFen=%-9d -> %s%n", amount, result);
        }

        System.out.println("\n[会员等级 if 链]");
        long[] points = {0L, 999L, 1_000L, 9_999L, 10_000L, 99_999L, 100_000L};
        for (long p : points) {
            System.out.printf("  points=%-8d -> %s%n", p, memberLevel(p));
        }

        System.out.println("\n[上传策略 if/else]");
        long[] sizes = {0L, 10 * 1024 * 1024L, 10 * 1024 * 1024L + 1,
                500 * 1024 * 1024L, 500 * 1024 * 1024L + 1};
        for (long size : sizes) {
            System.out.printf("  fileSize=%-11d -> %s%n", size, resolveUploadMode(size));
        }
    }
}
