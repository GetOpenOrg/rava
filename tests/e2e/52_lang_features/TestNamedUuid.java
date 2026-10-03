import java.util.UUID;

/**
 * 确定性 UUID 通道（框架稳定 ID 的地基，此前零覆盖）：nameUUIDFromBytes 的 MD5 v3
 * 形态、fromString 往返、version/variant 位、随机族不打印值只验证形态不变量。
 */
public class TestNamedUuid {

    public static void main(String[] args) {
        UUID v3 = UUID.nameUUIDFromBytes("rava-e2e".getBytes());
        System.out.println("v3=" + v3);
        System.out.println("version=" + v3.version() + " variant=" + v3.variant());

        // 确定性：同字节同 UUID
        System.out.println("deterministic=" + v3.equals(
                UUID.nameUUIDFromBytes("rava-e2e".getBytes())));
        System.out.println("input-sensitive=" + !v3.equals(
                UUID.nameUUIDFromBytes("rava-e2f".getBytes())));

        // fromString 往返（含大小写不敏感）
        UUID parsed = UUID.fromString(v3.toString());
        System.out.println("parse-roundtrip=" + v3.equals(parsed));
        UUID upper = UUID.fromString(v3.toString().toUpperCase());
        System.out.println("case-insensitive=" + v3.equals(upper));

        // 固定字面量的低位高位
        UUID fixed = UUID.fromString("00000000-0000-0000-0000-000000000001");
        System.out.println("fixed-msb=0x" + Long.toHexString(fixed.getMostSignificantBits())
                + " lsb=0x" + Long.toHexString(fixed.getLeastSignificantBits()));

        // fromString 非法形态
        try {
            UUID.fromString("not-a-uuid");
        } catch (IllegalArgumentException e) {
            System.out.println("bad-form-ex=" + e.getClass().getSimpleName());
        }
        try {
            UUID.fromString("00000000-0000-0000-0000-0000000000zz");
        } catch (IllegalArgumentException e) {
            System.out.println("bad-hex-ex=" + e.getClass().getSimpleName());
        }

        // 随机族：不打印值，验证版本位不变量
        UUID rnd = UUID.randomUUID();
        System.out.println("random-v4=" + (rnd.version() == 4));
        System.out.println("clock-seq-var=" + (rnd.variant() == 2));
    }
}
