import java.io.ByteArrayOutputStream;
import java.io.StringWriter;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.nio.charset.StandardCharsets;

import com.fasterxml.jackson.core.Base64Variants;
import com.fasterxml.jackson.core.JsonEncoding;
import com.fasterxml.jackson.core.JsonFactory;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.core.io.SerializedString;
import com.fasterxml.jackson.core.util.DefaultIndenter;
import com.fasterxml.jackson.core.util.DefaultPrettyPrinter;
import com.fasterxml.jackson.core.util.MinimalPrettyPrinter;

/**
 * jackson-core 切片 jc2（矩阵 #3）：JsonGenerator 往返。
 *
 * 覆盖面：Writer（WriterBasedJsonGenerator）与 OutputStream（UTF8JsonGenerator）两路生成器的
 * 全部 write* 族、DefaultPrettyPrinter / 自定义缩进 / MinimalPrettyPrinter、Base64 二进制、
 * writeRaw / writeRawValue、SerializedString 预编码名、copyCurrentEvent / copyCurrentStructure
 * 经 parser → generator 往返后逐字一致、写上下文 / 输出位置，以及非法写序报错。
 */
public class JacksonGeneratorMain {

    static final JsonFactory F = new JsonFactory();

    static void writeSample(JsonGenerator g) throws Exception {
        g.writeStartObject();
        g.writeStringField("str", "hello \"world\"\n\t\\/");
        g.writeNumberField("i", Integer.MIN_VALUE);
        g.writeNumberField("l", Long.MAX_VALUE);
        g.writeNumberField("d", 3.141592653589793);
        g.writeNumberField("f", 1.5f);
        g.writeNumberField("bd", new BigDecimal("12345678901234567890.000123"));
        g.writeFieldName("bi");
        g.writeNumber(new BigInteger("-98765432109876543210987654321"));
        g.writeFieldName("short");
        g.writeNumber((short) -12);
        g.writeBooleanField("t", true);
        g.writeNullField("n");
        g.writeFieldName(new SerializedString("pre:compiled"));
        g.writeString(new SerializedString("seré"));
        g.writeArrayFieldStart("arr");
        g.writeNumber(1);
        g.writeString("two");
        g.writeStartArray();
        g.writeEndArray();
        g.writeStartObject();
        g.writeEndObject();
        g.writeString(new char[] {'c', 'h', 'a', 'r', 's', 'X'}, 0, 5);
        g.writeNumber("123.4500");
        g.writeEndArray();
        g.writeFieldName("bin");
        g.writeBinary(new byte[] {0, 1, 2, (byte) 0xfe, (byte) 0xff, 65, 66});
        g.writeFieldName("raw");
        g.writeRawValue("{\"embedded\":[1,2]}");
        g.writeObjectFieldStart("nested");
        g.writeArrayFieldStart("deep");
        g.writeStartObject();
        g.writeStringField("k", "中文 😀");
        g.writeEndObject();
        g.writeEndArray();
        g.writeEndObject();
        g.writeEndObject();
    }

    static String viaWriter(JsonFactory f, java.util.function.Consumer<JsonGenerator> cfg) throws Exception {
        StringWriter sw = new StringWriter();
        try (JsonGenerator g = f.createGenerator(sw)) {
            cfg.accept(g);
            writeSample(g);
        }
        return sw.toString();
    }

    static String viaStream(JsonFactory f, java.util.function.Consumer<JsonGenerator> cfg) throws Exception {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (JsonGenerator g = f.createGenerator(bos, JsonEncoding.UTF8)) {
            cfg.accept(g);
            writeSample(g);
        }
        return bos.toString(StandardCharsets.UTF_8);
    }

    static void basic() throws Exception {
        System.out.println("── 紧凑输出");
        String w = viaWriter(F, g -> { });
        String s = viaStream(F, g -> { });
        System.out.println("  writer " + w);
        System.out.println("  stream " + s);
        int diff = 0;
        while (diff < Math.min(w.length(), s.length()) && w.charAt(diff) == s.charAt(diff)) {
            diff++;
        }
        System.out.println("  same=" + w.equals(s) + " firstDiff=" + diff);
        System.out.println("── DefaultPrettyPrinter");
        System.out.println(viaWriter(F, JsonGenerator::useDefaultPrettyPrinter));
        System.out.println("── 自定义缩进（tab + 无空格分隔）");
        DefaultPrettyPrinter pp = new DefaultPrettyPrinter()
                .withObjectIndenter(new DefaultIndenter("\t", "\n"))
                .withoutSpacesInObjectEntries();
        System.out.println(viaStream(F, g -> g.setPrettyPrinter(pp)));
        System.out.println("── MinimalPrettyPrinter 根分隔");
        StringWriter sw = new StringWriter();
        try (JsonGenerator g = F.createGenerator(sw)) {
            g.setPrettyPrinter(new MinimalPrettyPrinter("|"));
            g.writeNumber(1);
            g.writeString("x");
            g.writeStartArray();
            g.writeBoolean(false);
            g.writeEndArray();
        }
        System.out.println("  " + sw);
    }

    static String roundTrip(String json, boolean bytes, boolean structure, boolean pretty) throws Exception {
        return roundTrip(json, bytes, structure, pretty, true);
    }

    static String roundTrip(String json, boolean bytes, boolean structure, boolean pretty, boolean exact)
            throws Exception {
        StringWriter sw = new StringWriter();
        try (JsonParser p = bytes ? F.createParser(json.getBytes(StandardCharsets.UTF_8)) : F.createParser(json);
             JsonGenerator g = F.createGenerator(sw)) {
            if (pretty) {
                g.useDefaultPrettyPrinter();
            }
            if (structure) {
                p.nextToken();
                if (exact) {
                    g.copyCurrentStructureExact(p);
                } else {
                    g.copyCurrentStructure(p);
                }
            } else {
                while (p.nextToken() != null) {
                    if (exact) {
                        g.copyCurrentEventExact(p);
                    } else {
                        g.copyCurrentEvent(p);
                    }
                }
            }
        }
        return sw.toString();
    }

    static void roundTrips() throws Exception {
        System.out.println("── 往返");
        String compact = viaWriter(F, g -> { });
        for (int mode = 0; mode < 4; mode++) {
            boolean bytes = (mode & 1) != 0;
            boolean structure = (mode & 2) != 0;
            String back = roundTrip(compact, bytes, structure, false);
            System.out.println("  bytes=" + bytes + " structure=" + structure + " equal=" + back.equals(compact)
                    + (back.equals(compact) ? "" : " got " + back));
        }
        System.out.println("  lossy " + roundTrip(compact, false, false, false, false));
        String pretty = roundTrip(compact, true, true, true);
        String again = roundTrip(pretty, false, false, false);
        System.out.println("  pretty→compact equal=" + again.equals(compact));
        // Base64 往返
        try (JsonParser p = F.createParser(compact)) {
            while (p.nextToken() != null) {
                if ("bin".equals(p.currentName()) && p.currentToken() == JsonToken.VALUE_STRING) {
                    String txt = p.getText();
                    byte[] b = p.getBinaryValue();
                    StringBuilder sb = new StringBuilder();
                    for (byte x : b) {
                        sb.append(x & 0xff).append(',');
                    }
                    System.out.println("  binary " + txt + " -> " + sb);
                    System.out.println("  mime " + Base64Variants.MIME_NO_LINEFEEDS.encode(b) + " url "
                            + Base64Variants.MODIFIED_FOR_URL.encode(b) + " pem " + Base64Variants.PEM.encode(b, true));
                }
            }
        }
    }

    static void contexts() throws Exception {
        System.out.println("── 写上下文");
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (JsonGenerator g = F.createGenerator(bos)) {
            g.writeStartObject();
            g.writeFieldName("a");
            System.out.println("  after name: ctx=" + g.getOutputContext().typeDesc() + " name="
                    + g.getOutputContext().getCurrentName() + " out=" + g.getOutputBuffered());
            g.writeStartArray(null, 3);
            g.writeNumber(1);
            System.out.println("  in array: ctx=" + g.getOutputContext().typeDesc() + " idx="
                    + g.getOutputContext().getCurrentIndex() + " path=" + g.getOutputContext().pathAsPointer());
            g.writeArray(new int[] {5, 6, 7, 8}, 1, 2);
            g.writeArray(new long[] {9L, 10L}, 0, 2);
            g.writeArray(new double[] {0.5, -0.25}, 0, 2);
            g.writeEndArray();
            g.writeEndObject();
            g.flush();
            System.out.println("  written=" + bos.size() + " " + bos.toString(StandardCharsets.UTF_8));
        }
    }

    static void errors() {
        System.out.println("── 非法写序");
        String[] cases = {"value-in-object", "name-in-array", "end-array-in-object", "double-name", "end-root"};
        for (String c : cases) {
            for (int mode = 0; mode < 2; mode++) {
                StringWriter sw = new StringWriter();
                ByteArrayOutputStream bos = new ByteArrayOutputStream();
                try (JsonGenerator g = mode == 0 ? F.createGenerator(sw) : F.createGenerator(bos)) {
                    switch (c) {
                        case "value-in-object" -> { g.writeStartObject(); g.writeNumber(1); }
                        case "name-in-array" -> { g.writeStartArray(); g.writeFieldName("x"); }
                        case "end-array-in-object" -> { g.writeStartObject(); g.writeEndArray(); }
                        case "double-name" -> { g.writeStartObject(); g.writeFieldName("a"); g.writeFieldName("b"); }
                        default -> g.writeEndObject();
                    }
                    System.out.println("  " + c + " no error");
                } catch (JsonProcessingException e) {
                    System.out.println("  " + (mode == 0 ? "w " : "s ") + c + " -> " + e.getClass().getSimpleName()
                            + ": " + e.getOriginalMessage());
                } catch (Exception e) {
                    System.out.println("  " + c + " other " + e);
                }
            }
        }
        // AUTO_CLOSE_JSON_CONTENT：未闭合结构在 close 时补齐
        StringWriter sw = new StringWriter();
        try (JsonGenerator g = F.createGenerator(sw)) {
            g.writeStartObject();
            g.writeArrayFieldStart("open");
            g.writeStartObject();
            g.writeStringField("k", "v");
        } catch (Exception e) {
            System.out.println("  auto-close error " + e);
        }
        System.out.println("  auto-close " + sw);
    }

    public static void main(String[] args) throws Exception {
        basic();
        roundTrips();
        contexts();
        errors();
    }
}
