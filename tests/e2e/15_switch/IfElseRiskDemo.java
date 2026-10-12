
/**
 * 文章里的“改造前”写法：long 范围判断只能堆 if/else（Java 27 之前 switch 不支持 long）。
 * 三个业务规则：订单风控分级、会员等级、上传策略。
 *
 * 输出确定性说明：测试输入为固定边界值，无随机/时间成分。
 */
public class IfElseRiskDemo {

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

/** 订单风控等级（金额单位：分）——自上游 RiskLevel.java 并入（e2e 单文件口径）。 */
enum RiskLevel {
    LOW, MEDIUM, HIGH, VERY_HIGH
}

/** 文件上传策略——自上游 UploadMode.java 并入（e2e 单文件口径）。 */
enum UploadMode {
    DIRECT, MULTIPART, LARGE_FILE
}
