import java.io.ByteArrayOutputStream;
import java.io.StringWriter;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;

import com.fasterxml.jackson.core.JsonEncoding;
import com.fasterxml.jackson.core.JsonFactory;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.core.StreamReadFeature;
import com.fasterxml.jackson.core.StreamWriteFeature;
import com.fasterxml.jackson.core.io.JsonStringEncoder;
import com.fasterxml.jackson.core.json.JsonReadFeature;
import com.fasterxml.jackson.core.json.JsonWriteFeature;

/**
 * jackson-core 切片 jc3（矩阵 #3）：数字边角与编码。
 *
 * 覆盖面：数字——int/long/BigInteger 分级（getNumberType）、long 与 int 边界、溢出强转报错
 * （InputCoercionException）、BigDecimal 精度、科学计数、-0 / 指数大小写、非数值 NaN / Infinity 特性、
 * 快速 double 解析开关、WRITE_BIGDECIMAL_AS_PLAIN；编码——byte[] 输入的 UTF-8 / UTF-16 BE/LE /
 * UTF-32 自动探测、各 JsonEncoding 生成、\\u 转义与代理对、ESCAPE_NON_ASCII、非法 UTF-8 报错、
 * JsonStringEncoder 引号与 UTF-8 编码；末尾为数字密集吞吐基准（耗时写 stderr）。
 */
public class JacksonNumbersEncodingMain {

    static final JsonFactory F = new JsonFactory();

    static void describeNumbers(String label, JsonFactory f, String json) throws Exception {
        System.out.println("── " + label);
        try (JsonParser p = f.createParser(json)) {
            p.nextToken();
            while (p.nextToken() != JsonToken.END_ARRAY) {
                StringBuilder sb = new StringBuilder();
                sb.append(p.getText()).append(" tok=").append(p.currentToken());
                if (p.currentToken().isNumeric()) {
                    sb.append(" type=").append(p.getNumberType());
                    sb.append(" nan=").append(p.isNaN());
                    sb.append(" num=").append(p.getNumberValue());
                    sb.append(" dbl=").append(p.getDoubleValue());
                    sb.append(" flt=").append(p.getFloatValue());
                    try {
                        sb.append(" dec=").append(p.getDecimalValue());
                    } catch (Exception e) {
                        sb.append(" dec!").append(e.getClass().getSimpleName());
                    }
                    try {
                        sb.append(" big=").append(p.getBigIntegerValue());
                    } catch (Exception e) {
                        sb.append(" big!").append(e.getClass().getSimpleName());
                    }
                    try {
                        sb.append(" long=").append(p.getLongValue());
                    } catch (JsonProcessingException e) {
                        sb.append(" long!").append(e.getClass().getSimpleName()).append(':')
                                .append(e.getOriginalMessage());
                    }
                    try {
                        sb.append(" int=").append(p.getIntValue());
                    } catch (JsonProcessingException e) {
                        sb.append(" int!").append(e.getClass().getSimpleName()).append(':')
                                .append(e.getOriginalMessage());
                    }
                }
                System.out.println("  " + sb);
            }
        }
    }

    static void numbers() throws Exception {
        describeNumbers("整数分级", F, "[0,-0,2147483647,2147483648,-2147483648,-2147483649,"
                + "9223372036854775807,9223372036854775808,-9223372036854775808,-9223372036854775809,"
                + "123456789012345678901234567890]");
        describeNumbers("小数与科学计数", F, "[0.1,-0.0,1e3,1E-3,1.5e+10,6.02214076E23,4.9e-324,1e-400,"
                + "1.7976931348623157e308,1e309,3.14159265358979323846264338327950288,0.30000000000000004,"
                + "1234567890.0987654321e-5]");
        JsonFactory lax = JsonFactory.builder()
                .enable(JsonReadFeature.ALLOW_NON_NUMERIC_NUMBERS)
                .enable(JsonReadFeature.ALLOW_LEADING_PLUS_SIGN_FOR_NUMBERS)
                .enable(JsonReadFeature.ALLOW_LEADING_DECIMAL_POINT_FOR_NUMBERS)
                .enable(JsonReadFeature.ALLOW_TRAILING_DECIMAL_POINT_FOR_NUMBERS)
                .enable(JsonReadFeature.ALLOW_LEADING_ZEROS_FOR_NUMBERS)
                .build();
        describeNumbers("非标准数字", lax, "[NaN,Infinity,-Infinity,+Infinity,+5,.5,5.,007,-.25]");
        JsonFactory slow = JsonFactory.builder()
                .disable(StreamReadFeature.USE_FAST_DOUBLE_PARSER)
                .disable(StreamReadFeature.USE_FAST_BIG_NUMBER_PARSER)
                .build();
        JsonFactory fast = JsonFactory.builder()
                .enable(StreamReadFeature.USE_FAST_DOUBLE_PARSER)
                .enable(StreamReadFeature.USE_FAST_BIG_NUMBER_PARSER)
                .build();
        String hard = "[2.2250738585072011e-308,9007199254740993,0.1e1,1.0000000000000002,"
                + "123456789012345678901234567890.123456789e-20,8.41e21,5e-324]";
        describeNumbers("慢速解析器", slow, hard);
        describeNumbers("快速解析器", fast, hard);
    }

    static void writeNumbers() throws Exception {
        System.out.println("── 数字生成");
        for (int mode = 0; mode < 2; mode++) {
            JsonFactory f = mode == 0 ? F
                    : JsonFactory.builder().enable(StreamWriteFeature.WRITE_BIGDECIMAL_AS_PLAIN)
                            .enable(JsonWriteFeature.WRITE_NAN_AS_STRINGS).build();
            StringWriter sw = new StringWriter();
            try (JsonGenerator g = f.createGenerator(sw)) {
                g.writeStartArray();
                g.writeNumber(new BigDecimal("1E+20"));
                g.writeNumber(new BigDecimal("1.2300E-7"));
                g.writeNumber(new BigDecimal("-0.000"));
                g.writeNumber(new BigInteger("2").pow(100));
                g.writeNumber(Double.MIN_VALUE);
                g.writeNumber(Double.MAX_VALUE);
                g.writeNumber(1e7);
                g.writeNumber(1e-3);
                g.writeNumber(0.1f + 0.2f);
                g.writeNumber(0.1 + 0.2);
                g.writeNumber(Float.NaN);
                g.writeNumber(Double.NEGATIVE_INFINITY);
                g.writeNumber(-0.0);
                g.writeEndArray();
            }
            System.out.println("  " + (mode == 0 ? "default " : "plain+nanstr ") + sw);
        }
        StringWriter sw = new StringWriter();
        try (JsonGenerator g = JsonFactory.builder().enable(JsonWriteFeature.WRITE_NUMBERS_AS_STRINGS).build()
                .createGenerator(sw)) {
            g.writeStartArray();
            g.writeNumber(5);
            g.writeNumber(5L << 40);
            g.writeNumber(2.5);
            g.writeNumber(new BigDecimal("9.99"));
            g.writeEndArray();
        }
        System.out.println("  numbers-as-strings " + sw);
    }

    static String hex(byte[] b, int max) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < Math.min(b.length, max); i++) {
            sb.append(String.format("%02x", b[i] & 0xff));
        }
        return sb.toString();
    }

    static final String TEXT = "{\"k\u00e9y\":\"A\u00e9\u4e2d\ud83d\ude00\\u0041\\ud83d\\ude03\\n\\\"\\/\\u00FF\"}";

    static void detection() throws Exception {
        System.out.println("── 编码探测");
        String[] cs = {"UTF-8", "UTF-16BE", "UTF-16LE", "UTF-32BE", "UTF-32LE"};
        for (String c : cs) {
            for (int bom = 0; bom < 2; bom++) {
                byte[] body = TEXT.getBytes(Charset.forName(c));
                byte[] in = body;
                if (bom == 1) {
                    byte[] mark = "\ufeff".getBytes(Charset.forName(c));
                    in = new byte[mark.length + body.length];
                    System.arraycopy(mark, 0, in, 0, mark.length);
                    System.arraycopy(body, 0, in, mark.length, body.length);
                }
                StringBuilder sb = new StringBuilder();
                try (JsonParser p = F.createParser(in)) {
                    while (p.nextToken() != null) {
                        if (p.currentToken() == JsonToken.VALUE_STRING || p.currentToken() == JsonToken.FIELD_NAME) {
                            String t = p.getText();
                            sb.append(t.length()).append(':');
                            t.codePoints().forEach(cp -> sb.append(Integer.toHexString(cp)).append(' '));
                        }
                    }
                    System.out.println("  " + c + (bom == 1 ? "+BOM " : " ") + p.getClass().getSimpleName() + " " + sb);
                } catch (Exception e) {
                    System.out.println("  " + c + (bom == 1 ? "+BOM " : " ") + "-> " + e.getClass().getSimpleName()
                            + ": " + e.getMessage().replace("\n", " | "));
                }
            }
        }
    }

    static void generateEncodings() throws Exception {
        System.out.println("── 编码生成");
        for (JsonEncoding enc : JsonEncoding.values()) {
            ByteArrayOutputStream bos = new ByteArrayOutputStream();
            try (JsonGenerator g = F.createGenerator(bos, enc)) {
                g.writeStartObject();
                g.writeStringField("s\u00e9", "\u00e9\u4e2d\ud83d\ude00\u0001\u001f\u007f");
                g.writeEndObject();
            }
            byte[] b = bos.toByteArray();
            System.out.println("  " + enc + " name=" + enc.getJavaName() + " bigEndian=" + enc.isBigEndian()
                    + " bits=" + enc.bits() + " len=" + b.length + " hex=" + hex(b, 24)
                    + " back=" + new String(b, Charset.forName(enc.getJavaName())));
        }
        for (int mode = 0; mode < 4; mode++) {
            JsonFactory f = JsonFactory.builder()
                    .configure(JsonWriteFeature.ESCAPE_NON_ASCII, (mode & 1) != 0)
                    .configure(JsonWriteFeature.WRITE_HEX_UPPER_CASE, (mode & 2) == 0)
                    .build();
            StringWriter sw = new StringWriter();
            ByteArrayOutputStream bos = new ByteArrayOutputStream();
            String v = "\u00e9\u4e2d\ud83d\ude00</script>\u2028\u0000";
            try (JsonGenerator g = f.createGenerator(sw)) {
                g.writeString(v);
            }
            try (JsonGenerator g = f.createGenerator(bos)) {
                g.writeString(v);
            }
            System.out.println("  escapeNonAscii=" + ((mode & 1) != 0) + " upper=" + ((mode & 2) == 0) + " writer="
                    + sw + " stream=" + bos.toString(StandardCharsets.UTF_8));
        }
    }

    static void badEncodings() {
        System.out.println("── 非法编码");
        byte[][] bad = {
            {'[', '"', (byte) 0xc3, '"', ']'},
            {'[', '"', (byte) 0xff, '"', ']'},
            {'[', '"', (byte) 0xed, (byte) 0xa0, (byte) 0x80, '"', ']'},
            {'[', '"', (byte) 0xf0, (byte) 0x9f, (byte) 0x98, '"', ']'},
            {'[', '"', 'a', 0x01, '"', ']'},
        };
        for (byte[] b : bad) {
            StringBuilder sb = new StringBuilder();
            try (JsonParser p = F.createParser(b)) {
                while (p.nextToken() != null) {
                    if (p.currentToken() == JsonToken.VALUE_STRING) {
                        p.getText().codePoints().forEach(cp -> sb.append(Integer.toHexString(cp)).append(' '));
                    }
                }
                System.out.println("  " + hex(b, 16) + " OK " + sb);
            } catch (Exception e) {
                System.out.println("  " + hex(b, 16) + " -> " + e.getClass().getSimpleName() + ": "
                        + e.getMessage().replace("\n", " | "));
            }
        }
        String[] badEsc = {"[\"\\ud83d\"]", "[\"\\ude00x\"]", "[\"\\u12\"]", "[\"\\uZZZZ\"]"};
        for (String s : badEsc) {
            for (int mode = 0; mode < 2; mode++) {
                StringBuilder sb = new StringBuilder();
                try (JsonParser p = mode == 0 ? F.createParser(s) : F.createParser(s.getBytes(StandardCharsets.UTF_8))) {
                    while (p.nextToken() != null) {
                        if (p.currentToken() == JsonToken.VALUE_STRING) {
                            p.getText().chars().forEach(cp -> sb.append(Integer.toHexString(cp)).append(' '));
                        }
                    }
                    System.out.println("  " + (mode == 0 ? "c " : "b ") + s + " OK " + sb);
                } catch (Exception e) {
                    System.out.println("  " + (mode == 0 ? "c " : "b ") + s + " -> " + e.getClass().getSimpleName()
                            + ": " + e.getMessage().replace("\n", " | "));
                }
            }
        }
    }

    static void stringEncoder() {
        System.out.println("── JsonStringEncoder");
        JsonStringEncoder enc = JsonStringEncoder.getInstance();
        String s = "q\"b\\s/\t\u0007\u00e9\ud83d\ude00";
        System.out.println("  quote=" + new String(enc.quoteAsString(s)));
        System.out.println("  quoteUtf8=" + hex(enc.quoteAsUTF8(s), 64));
        System.out.println("  utf8=" + hex(enc.encodeAsUTF8(s), 64));
        StringBuilder sb = new StringBuilder("pre:");
        enc.quoteAsString("x\ny", sb);
        System.out.println("  sb=" + sb);
    }

    // 数字密集吞吐：大数组的 int / double / BigDecimal 生成与解析
    static void bench() throws Exception {
        System.out.println("── 数字吞吐基准");
        long t0 = System.nanoTime();
        ByteArrayOutputStream bos = new ByteArrayOutputStream(1 << 22);
        long seed = 12345;
        try (JsonGenerator g = F.createGenerator(bos)) {
            g.writeStartArray();
            for (int i = 0; i < 200000; i++) {
                seed = seed * 6364136223846793005L + 1442695040888963407L;
                switch (i % 4) {
                    case 0 -> g.writeNumber(seed >> 20);
                    case 1 -> g.writeNumber((seed >>> 11) * 0x1.0p-53 * 1e6);
                    case 2 -> g.writeNumber(BigDecimal.valueOf(seed % 100000000, 4));
                    default -> g.writeNumber((int) seed);
                }
            }
            g.writeEndArray();
        }
        byte[] bytes = bos.toByteArray();
        long t1 = System.nanoTime();
        double dsum = 0;
        long lsum = 0;
        BigDecimal bsum = BigDecimal.ZERO;
        for (int r = 0; r < 3; r++) {
            try (JsonParser p = F.createParser(bytes)) {
                p.nextToken();
                int i = 0;
                while (p.nextToken() != JsonToken.END_ARRAY) {
                    switch (i++ % 4) {
                        case 1 -> dsum += p.getDoubleValue();
                        case 2 -> bsum = bsum.add(p.getDecimalValue());
                        default -> lsum += p.getLongValue();
                    }
                }
            }
        }
        long t2 = System.nanoTime();
        System.out.println("  bytes=" + bytes.length + " lsum=" + lsum + " dsum=" + dsum + " bsum=" + bsum);
        System.err.println("JC_BENCH gen_ms=" + (t1 - t0) / 1_000_000 + " parse_ms=" + (t2 - t1) / 1_000_000);
    }

    public static void main(String[] args) throws Exception {
        numbers();
        writeNumbers();
        detection();
        generateEncodings();
        badEncodings();
        stringEncoder();
        bench();
    }
}
