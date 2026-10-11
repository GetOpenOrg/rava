import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.StringReader;
import java.nio.charset.StandardCharsets;

import com.fasterxml.jackson.core.JsonFactory;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonLocation;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonPointer;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.core.async.ByteArrayFeeder;

/**
 * jackson-core 切片 jc1（矩阵 #3）：JsonParser 流式解析与 token 状态机。
 *
 * 覆盖面：三种输入形态（String / Reader → ReaderBasedJsonParser，byte[] / InputStream →
 * UTF8StreamJsonParser，非阻塞 feed → NonBlockingJsonParser）的 token 序列、当前名与解析上下文路径、
 * 位置信息、nextFieldName / nextTextValue 等快速路径、skipChildren、多根值、畸形输入报错消息，
 * 以及吞吐基准（约 4 MB 文档生成 + 多轮解析；耗时写 stderr，stdout 只留校验和）。
 */
public class JacksonParserMain {

    static final JsonFactory F = new JsonFactory();

    static final String DOC = "{\"id\":42,\"name\":\"rava\",\"tags\":[\"a\",\"b\",{\"deep\":[1,2.5,true,null]}],"
            + "\"empty\":{},\"none\":[],\"flag\":false,\"neg\":-7,\"esc\":\"x\\ny\\u0041\"}";

    static String tokenLine(JsonParser p) throws Exception {
        JsonToken t = p.currentToken();
        StringBuilder sb = new StringBuilder();
        sb.append(t).append(" id=").append(p.currentTokenId());
        sb.append(" name=").append(p.currentName());
        sb.append(" path=").append(p.getParsingContext().pathAsPointer());
        sb.append(" depth=").append(p.getParsingContext().getNestingDepth());
        if (t.isScalarValue()) {
            sb.append(" text=").append(p.getText());
            sb.append(" num=").append(t.isNumeric());
        }
        sb.append(" struct=").append(t.isStructStart()).append('/').append(t.isStructEnd());
        return sb.toString();
    }

    static void dumpTokens(String label, JsonParser p) throws Exception {
        System.out.println("── " + label + " (" + p.getClass().getSimpleName() + ")");
        int n = 0;
        while (p.nextToken() != null) {
            System.out.println("  " + tokenLine(p));
            n++;
        }
        System.out.println("  tokens=" + n + " closed=" + p.isClosed() + " last=" + p.getLastClearedToken());
        p.close();
    }

    static void locations() throws Exception {
        System.out.println("── 位置");
        String src = "{\n  \"a\" : 1,\n  \"b\" : [ true ,\n false ]\n}";
        try (JsonParser p = F.createParser(src)) {
            while (p.nextToken() != null) {
                JsonLocation loc = p.currentTokenLocation();
                System.out.println("  " + p.currentToken() + " line=" + loc.getLineNr() + " col=" + loc.getColumnNr()
                        + " char=" + loc.getCharOffset());
            }
        }
        try (JsonParser p = F.createParser(src.getBytes(StandardCharsets.UTF_8))) {
            while (p.nextToken() != null) {
                JsonLocation loc = p.currentTokenLocation();
                System.out.println("  bytes " + p.currentToken() + " line=" + loc.getLineNr() + " col="
                        + loc.getColumnNr() + " byte=" + loc.getByteOffset());
            }
        }
    }

    static void fastPaths() throws Exception {
        System.out.println("── 快速路径");
        String src = "{\"k1\":\"v1\",\"k2\":7,\"k3\":true,\"k4\":{\"x\":[1,2,3]},\"k5\":\"tail\"}";
        for (int mode = 0; mode < 2; mode++) {
            try (JsonParser p = mode == 0 ? F.createParser(src) : F.createParser(src.getBytes(StandardCharsets.UTF_8))) {
                p.nextToken();
                String name;
                StringBuilder sb = new StringBuilder();
                while ((name = p.nextFieldName()) != null) {
                    JsonToken v = p.nextToken();
                    sb.append(name).append(':').append(v);
                    if (v.isStructStart()) {
                        p.skipChildren();
                        sb.append("(skipped→").append(p.currentToken()).append(')');
                    } else {
                        sb.append('=').append(p.getValueAsString());
                    }
                    sb.append(' ');
                }
                System.out.println("  " + (mode == 0 ? "chars " : "bytes ") + sb.toString().trim());
            }
        }
        try (JsonParser p = F.createParser("[\"s1\",\"s2\",3,\"s4\"]")) {
            p.nextToken();
            String s;
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < 4; i++) {
                s = p.nextTextValue();
                sb.append(s == null ? "<" + p.currentToken() + ">" : s).append(' ');
            }
            System.out.println("  nextTextValue " + sb.toString().trim());
        }
        try (JsonParser p = F.createParser("[10,\"20\",3.9,true,null,\"x\"]")) {
            p.nextToken();
            StringBuilder sb = new StringBuilder();
            while (p.nextToken() != JsonToken.END_ARRAY) {
                sb.append(p.getValueAsInt(-1)).append('/').append(p.getValueAsLong(-2)).append('/')
                        .append(p.getValueAsDouble(-3.0)).append('/').append(p.getValueAsBoolean(false)).append(' ');
            }
            System.out.println("  coerce " + sb.toString().trim());
        }
    }

    static void multiRoot() throws Exception {
        System.out.println("── 多根值");
        try (JsonParser p = F.createParser("1 \"two\" [3] {\"four\":4} true null")) {
            StringBuilder sb = new StringBuilder();
            JsonToken t;
            while ((t = p.nextToken()) != null) {
                if (p.getParsingContext().inRoot()) {
                    sb.append(t).append(' ');
                }
            }
            System.out.println("  " + sb.toString().trim());
        }
    }

    static void errors() {
        System.out.println("── 畸形输入");
        String[] bad = {
            "{\"a\":1,}", "[1,2", "{\"a\" 1}", "[tru]", "\"unterminated", "{\"a\":01}", "[1]]", "{'a':1}",
            "// c\n1", "[\"\\q\"]", "{\"a\":1}x", "[-]", "[1.]", "[NaN]",
        };
        for (String b : bad) {
            for (int mode = 0; mode < 2; mode++) {
                StringBuilder sb = new StringBuilder();
                try (JsonParser p = mode == 0 ? F.createParser(b) : F.createParser(b.getBytes(StandardCharsets.UTF_8))) {
                    while (p.nextToken() != null) {
                        sb.append(p.currentToken()).append(' ');
                    }
                    System.out.println("  " + (mode == 0 ? "c " : "b ") + b.replace("\n", "\\n") + " OK " + sb);
                } catch (JsonProcessingException e) {
                    System.out.println("  " + (mode == 0 ? "c " : "b ") + b.replace("\n", "\\n") + " -> "
                            + e.getClass().getSimpleName() + " after [" + sb.toString().trim() + "]: "
                            + e.getMessage().replace("\n", " | "));
                } catch (Exception e) {
                    System.out.println("  other " + e);
                }
            }
        }
    }

    static void nonBlocking() throws Exception {
        System.out.println("── 非阻塞 feed");
        byte[] all = DOC.getBytes(StandardCharsets.UTF_8);
        for (int chunk : new int[] {1, 7, all.length}) {
            JsonParser p = F.createNonBlockingByteArrayParser();
            ByteArrayFeeder feeder = (ByteArrayFeeder) p.getNonBlockingInputFeeder();
            int off = 0;
            int notAvail = 0;
            StringBuilder sb = new StringBuilder();
            while (true) {
                JsonToken t = p.nextToken();
                if (t == JsonToken.NOT_AVAILABLE) {
                    notAvail++;
                    if (off < all.length) {
                        int end = Math.min(all.length, off + chunk);
                        feeder.feedInput(all, off, end);
                        off = end;
                    } else {
                        feeder.endOfInput();
                    }
                    continue;
                }
                if (t == null) {
                    break;
                }
                sb.append(t.asString() != null ? t.asString() : t.isScalarValue() ? p.getText() : p.currentName());
                sb.append(' ');
            }
            System.out.println("  chunk=" + chunk + " notAvail>0=" + (notAvail > 0) + " " + sb.toString().trim());
            p.close();
        }
    }

    static void pointers() throws Exception {
        System.out.println("── JsonPointer");
        JsonPointer ptr = JsonPointer.compile("/tags/2/deep/1");
        System.out.println("  ptr=" + ptr + " head=" + ptr.getMatchingProperty() + " tail=" + ptr.tail()
                + " last=" + ptr.last() + " append=" + ptr.append(JsonPointer.compile("/z")));
        try (JsonParser p = F.createParser(DOC)) {
            while (p.nextToken() != null) {
                if (p.getParsingContext().pathAsPointer().equals(ptr)) {
                    System.out.println("  hit " + p.currentToken() + " " + p.getText());
                }
            }
        }
        System.out.println("  escape " + JsonPointer.compile("/a~1b/c~0d").getMatchingProperty() + " "
                + JsonPointer.compile("/a~1b/c~0d").tail().getMatchingProperty());
    }

    // 吞吐基准：生成约 4 MB 文档，按 chars / bytes 两路各解析若干轮；stdout 只打确定性校验和
    static void bench() throws Exception {
        System.out.println("── 吞吐基准");
        long t0 = System.nanoTime();
        ByteArrayOutputStream bos = new ByteArrayOutputStream(1 << 22);
        try (JsonGenerator g = F.createGenerator(bos)) {
            g.writeStartArray();
            for (int i = 0; i < 40000; i++) {
                g.writeStartObject();
                g.writeNumberField("id", i);
                g.writeStringField("name", "item-" + i + "-\u00e9\u4e2d");
                g.writeNumberField("price", i * 1.25 + 0.5);
                g.writeBooleanField("active", (i & 1) == 0);
                g.writeArrayFieldStart("vals");
                for (int k = 0; k < 5; k++) {
                    g.writeNumber((long) i * 31 + k);
                }
                g.writeEndArray();
                g.writeObjectFieldStart("meta");
                g.writeStringField("note", "line\n\"quoted\"\t" + (i % 97));
                g.writeNullField("nothing");
                g.writeEndObject();
                g.writeEndObject();
            }
            g.writeEndArray();
        }
        byte[] bytes = bos.toByteArray();
        String text = new String(bytes, StandardCharsets.UTF_8);
        long t1 = System.nanoTime();
        long sum = 0;
        long tokens = 0;
        int rounds = 5;
        for (int r = 0; r < rounds; r++) {
            try (JsonParser p = (r & 1) == 0 ? F.createParser(new ByteArrayInputStream(bytes))
                    : F.createParser(new StringReader(text))) {
                JsonToken t;
                while ((t = p.nextToken()) != null) {
                    tokens++;
                    switch (t) {
                        case VALUE_NUMBER_INT -> sum += p.getLongValue();
                        case VALUE_NUMBER_FLOAT -> sum += (long) (p.getDoubleValue() * 4);
                        case VALUE_STRING -> sum += p.getTextLength();
                        case FIELD_NAME -> sum += p.currentName().length();
                        default -> sum += 1;
                    }
                }
            }
        }
        long t2 = System.nanoTime();
        System.out.println("  bytes=" + bytes.length + " rounds=" + rounds + " tokens=" + tokens + " sum=" + sum);
        System.err.println("JC_BENCH gen_ms=" + (t1 - t0) / 1_000_000 + " parse_ms=" + (t2 - t1) / 1_000_000
                + " mb_per_s=" + String.format("%.1f", bytes.length * (double) rounds / 1e6 / ((t2 - t1) / 1e9)));
    }

    public static void main(String[] args) throws Exception {
        dumpTokens("String", F.createParser(DOC));
        dumpTokens("byte[]", F.createParser(DOC.getBytes(StandardCharsets.UTF_8)));
        dumpTokens("Reader", F.createParser(new StringReader(DOC)));
        dumpTokens("InputStream", F.createParser(new ByteArrayInputStream(DOC.getBytes(StandardCharsets.UTF_8))));
        locations();
        fastPaths();
        multiRoot();
        errors();
        nonBlocking();
        pointers();
        bench();
    }
}
