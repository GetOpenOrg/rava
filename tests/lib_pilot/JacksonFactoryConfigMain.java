import java.io.ByteArrayOutputStream;
import java.io.FilterOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.io.Reader;
import java.io.StringReader;
import java.io.StringWriter;
import java.io.Writer;
import java.nio.charset.StandardCharsets;

import com.fasterxml.jackson.core.JsonFactory;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.core.SerializableString;
import com.fasterxml.jackson.core.StreamReadConstraints;
import com.fasterxml.jackson.core.StreamReadFeature;
import com.fasterxml.jackson.core.StreamWriteConstraints;
import com.fasterxml.jackson.core.StreamWriteFeature;
import com.fasterxml.jackson.core.io.CharacterEscapes;
import com.fasterxml.jackson.core.io.IOContext;
import com.fasterxml.jackson.core.io.InputDecorator;
import com.fasterxml.jackson.core.io.OutputDecorator;
import com.fasterxml.jackson.core.io.SerializedString;
import com.fasterxml.jackson.core.json.JsonReadFeature;
import com.fasterxml.jackson.core.json.JsonWriteFeature;

/**
 * jackson-core 切片 jc4（矩阵 #3）：JsonFactory 配置与特性开关。
 *
 * 覆盖面：各特性枚举的缺省值与位掩码、builder / rebuild / copy 的配置继承、宽松读特性
 * （注释 / 单引号 / 无引号名 / 尾逗号 / 缺失值 / 控制字符 / 任意转义）、重复键检测、
 * 读写约束（嵌套深度 / 数字 / 字符串 / 名字长度 / 文档长度 / token 数）、根值分隔符、
 * 用户子类回调库（自定义 CharacterEscapes、Input/OutputDecorator），以及工厂能力查询。
 */
public class JacksonFactoryConfigMain {

    /** 自定义转义：'<' '>' 转为 \\u 形式，'#' 转为自定义序列，非 ASCII 走标准 \\u 转义。 */
    static final class HtmlEscapes extends CharacterEscapes {
        private final int[] codes;

        HtmlEscapes() {
            codes = standardAsciiEscapesForJSON();
            codes['<'] = ESCAPE_STANDARD;
            codes['>'] = ESCAPE_STANDARD;
            codes['#'] = ESCAPE_CUSTOM;
        }

        @Override
        public int[] getEscapeCodesForAscii() {
            return codes;
        }

        @Override
        public SerializableString getEscapeSequence(int ch) {
            if (ch == '#') {
                return new SerializedString("<HASH>");
            }
            if (ch == 0x2028 || ch == 0x2029) {
                return new SerializedString(String.format("\\u%04x", ch));
            }
            return null;
        }
    }

    /** 输出装饰：在每个生成器输出前加前缀，统计写出字节。 */
    static final class CountingDecorator extends OutputDecorator {
        int created;

        @Override
        public OutputStream decorate(IOContext ctxt, OutputStream out) throws IOException {
            created++;
            out.write("/*os*/".getBytes(StandardCharsets.US_ASCII));
            return new FilterOutputStream(out) {
                @Override
                public void write(int b) throws IOException {
                    super.write(Character.toUpperCase((char) b));
                }

                @Override
                public void write(byte[] b, int off, int len) throws IOException {
                    for (int i = off; i < off + len; i++) {
                        write(b[i]);
                    }
                }
            };
        }

        @Override
        public Writer decorate(IOContext ctxt, Writer w) throws IOException {
            created++;
            w.write("/*w*/");
            return w;
        }
    }

    /** 输入装饰：把输入中的单引号替换为双引号。 */
    static final class QuoteFixDecorator extends InputDecorator {
        @Override
        public InputStream decorate(IOContext ctxt, InputStream in) throws IOException {
            byte[] all = in.readAllBytes();
            return new java.io.ByteArrayInputStream(new String(all, StandardCharsets.UTF_8).replace('\'', '"')
                    .getBytes(StandardCharsets.UTF_8));
        }

        @Override
        public InputStream decorate(IOContext ctxt, byte[] src, int offset, int length) throws IOException {
            return decorate(ctxt, new java.io.ByteArrayInputStream(src, offset, length));
        }

        @Override
        public Reader decorate(IOContext ctxt, Reader r) throws IOException {
            StringBuilder sb = new StringBuilder();
            int c;
            while ((c = r.read()) >= 0) {
                sb.append(c == '\'' ? '"' : (char) c);
            }
            return new StringReader(sb.toString());
        }
    }

    static String tokens(JsonFactory f, String json, boolean bytes) {
        StringBuilder sb = new StringBuilder();
        try (JsonParser p = bytes ? f.createParser(json.getBytes(StandardCharsets.UTF_8)) : f.createParser(json)) {
            JsonToken t;
            while ((t = p.nextToken()) != null) {
                sb.append(t == JsonToken.FIELD_NAME ? p.currentName() + ":" : t.isScalarValue() ? p.getText() : t.asString());
                sb.append(' ');
            }
            return "OK " + sb.toString().trim();
        } catch (Exception e) {
            return "after [" + sb.toString().trim() + "] " + e.getClass().getSimpleName() + ": "
                    + e.getMessage().replace("\n", " | ");
        }
    }

    static void defaults() {
        System.out.println("── 缺省特性");
        JsonFactory f = new JsonFactory();
        for (JsonFactory.Feature x : JsonFactory.Feature.values()) {
            System.out.println("  factory " + x + " def=" + x.enabledByDefault() + " mask=" + x.getMask() + " on="
                    + f.isEnabled(x));
        }
        for (StreamReadFeature x : StreamReadFeature.values()) {
            System.out.println("  sread " + x + " def=" + x.enabledByDefault() + " on=" + f.isEnabled(x));
        }
        for (StreamWriteFeature x : StreamWriteFeature.values()) {
            System.out.println("  swrite " + x + " def=" + x.enabledByDefault() + " on=" + f.isEnabled(x));
        }
        for (JsonReadFeature x : JsonReadFeature.values()) {
            System.out.println("  jread " + x + " def=" + x.enabledByDefault() + " mask=" + x.getMask() + " mapped="
                    + x.mappedFeature());
        }
        for (JsonWriteFeature x : JsonWriteFeature.values()) {
            System.out.println("  jwrite " + x + " def=" + x.enabledByDefault() + " mask=" + x.getMask());
        }
        System.out.println("  masks factory=" + f.getFactoryFeatures() + " parser=" + f.getParserFeatures()
                + " generator=" + f.getGeneratorFeatures() + " fparser=" + f.getFormatParserFeatures()
                + " fgen=" + f.getFormatGeneratorFeatures());
        System.out.println("  format=" + f.getFormatName() + " async=" + f.canParseAsync() + " chars="
                + f.canUseCharArrays() + " binary=" + f.canHandleBinaryNatively() + " ordering="
                + f.requiresPropertyOrdering() + " codec=" + f.getCodec() + " sep=[" + f.getRootValueSeparator() + "]");
        System.out.println("  version=" + f.version().getMajorVersion() + "." + f.version().getMinorVersion()
                + " group=" + f.version().getGroupId() + " artifact=" + f.version().getArtifactId());
        StreamReadConstraints rc = f.streamReadConstraints();
        System.out.println("  readConstraints depth=" + rc.getMaxNestingDepth() + " num=" + rc.getMaxNumberLength()
                + " str=" + rc.getMaxStringLength() + " name=" + rc.getMaxNameLength() + " doc="
                + rc.getMaxDocumentLength() + " tokens=" + rc.getMaxTokenCount());
        System.out.println("  writeConstraints depth=" + f.streamWriteConstraints().getMaxNestingDepth());
    }

    static void lenient() {
        System.out.println("── 宽松读特性");
        String messy = "// head\n{ unquoted: 'single', # yaml\n /* block */ \"arr\": [1,,3,], \"ctl\":\"a\tb\","
                + " \"esc\": \"\\q\", }";
        JsonFactory strict = new JsonFactory();
        JsonFactory lax = JsonFactory.builder()
                .enable(JsonReadFeature.ALLOW_JAVA_COMMENTS, JsonReadFeature.ALLOW_YAML_COMMENTS,
                        JsonReadFeature.ALLOW_SINGLE_QUOTES, JsonReadFeature.ALLOW_UNQUOTED_FIELD_NAMES,
                        JsonReadFeature.ALLOW_MISSING_VALUES, JsonReadFeature.ALLOW_TRAILING_COMMA,
                        JsonReadFeature.ALLOW_UNESCAPED_CONTROL_CHARS,
                        JsonReadFeature.ALLOW_BACKSLASH_ESCAPING_ANY_CHARACTER)
                .build();
        for (int bytes = 0; bytes < 2; bytes++) {
            System.out.println("  strict " + (bytes == 1 ? "b " : "c ") + tokens(strict, messy, bytes == 1));
            System.out.println("  lax " + (bytes == 1 ? "b " : "c ") + tokens(lax, messy, bytes == 1));
        }
        // 逐项开启：每个特性单独打开时对同一输入的影响
        JsonReadFeature[] each = {JsonReadFeature.ALLOW_JAVA_COMMENTS, JsonReadFeature.ALLOW_SINGLE_QUOTES,
            JsonReadFeature.ALLOW_UNQUOTED_FIELD_NAMES, JsonReadFeature.ALLOW_TRAILING_COMMA};
        String[] inputs = {"/*c*/[1]", "['x']", "{a:1}", "[1,2,]"};
        for (int i = 0; i < each.length; i++) {
            JsonFactory one = JsonFactory.builder().enable(each[i]).build();
            System.out.println("  " + each[i] + " " + tokens(one, inputs[i], false) + " | strict "
                    + tokens(strict, inputs[i], false));
        }
        JsonFactory rs = JsonFactory.builder().enable(JsonReadFeature.ALLOW_RS_CONTROL_CHAR).build();
        System.out.println("  rs " + tokens(rs, "\u001e{\"a\":1}\u001e[2]", false));
    }

    static void duplicates() {
        System.out.println("── 重复键");
        String dup = "{\"a\":1,\"b\":{\"a\":2,\"c\":3},\"a\":4}";
        JsonFactory det = JsonFactory.builder().enable(StreamReadFeature.STRICT_DUPLICATE_DETECTION).build();
        System.out.println("  off " + tokens(new JsonFactory(), dup, false));
        System.out.println("  on c " + tokens(det, dup, false));
        System.out.println("  on b " + tokens(det, dup, true));
        StringWriter sw = new StringWriter();
        JsonFactory wdet = JsonFactory.builder().enable(StreamWriteFeature.STRICT_DUPLICATE_DETECTION).build();
        try (JsonGenerator g = wdet.createGenerator(sw)) {
            g.writeStartObject();
            g.writeNumberField("x", 1);
            g.writeNumberField("x", 2);
        } catch (Exception e) {
            System.out.println("  write dup " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    static void constraints() {
        System.out.println("── 读写约束");
        JsonFactory f = JsonFactory.builder()
                .streamReadConstraints(StreamReadConstraints.builder()
                        .maxNestingDepth(3).maxNumberLength(5).maxStringLength(8).maxNameLength(4).build())
                .build();
        String[] in = {"[[[1]]]", "[[[[1]]]]", "[12345]", "[123456]", "[1.2345]", "[\"12345678\"]",
            "[\"123456789\"]", "{\"abcd\":1}", "{\"abcde\":1}"};
        for (String s : in) {
            System.out.println("  " + s + " c " + tokens(f, s, false));
            System.out.println("  " + s + " b " + tokens(f, s, true));
        }
        JsonFactory doc = JsonFactory.builder()
                .streamReadConstraints(StreamReadConstraints.builder().maxDocumentLength(10).maxTokenCount(4).build())
                .build();
        System.out.println("  doclen " + tokens(doc, "[1,2,3,4,5,6,7,8]", true));
        System.out.println("  tokens " + tokens(doc, "[1,2,3]", false));
        JsonFactory w = JsonFactory.builder()
                .streamWriteConstraints(StreamWriteConstraints.builder().maxNestingDepth(2).build()).build();
        StringWriter sw = new StringWriter();
        try (JsonGenerator g = w.createGenerator(sw)) {
            g.writeStartArray();
            g.writeStartArray();
            g.writeStartArray();
        } catch (Exception e) {
            System.out.println("  write depth " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
        try {
            StreamReadConstraints.builder().maxNestingDepth(-1);
        } catch (IllegalArgumentException e) {
            System.out.println("  bad builder " + e.getMessage());
        }
    }

    static void writeFeatures() throws Exception {
        System.out.println("── 写特性 / 根分隔 / 自定义转义 / 装饰器");
        JsonFactory base = JsonFactory.builder()
                .disable(JsonWriteFeature.QUOTE_FIELD_NAMES)
                .enable(JsonWriteFeature.ESCAPE_FORWARD_SLASHES)
                .build();
        base.setRootValueSeparator(" ;; ");
        base.setCharacterEscapes(new HtmlEscapes());
        for (int mode = 0; mode < 2; mode++) {
            ByteArrayOutputStream bos = new ByteArrayOutputStream();
            StringWriter sw = new StringWriter();
            try (JsonGenerator g = mode == 0 ? base.createGenerator(sw) : base.createGenerator(bos)) {
                g.writeStartObject();
                g.writeStringField("html", "<b>#1</b> \u2028 \u00e9 a/b");
                g.writeEndObject();
                g.writeNumber(2);
                g.writeString("x");
            }
            System.out.println("  " + (mode == 0 ? "writer " + sw : "stream " + bos.toString(StandardCharsets.UTF_8)));
        }
        // copy / rebuild 继承配置
        JsonFactory copy = base.copy();
        JsonFactory rebuilt = base.rebuild().enable(JsonWriteFeature.QUOTE_FIELD_NAMES).build();
        System.out.println("  copy sep=[" + copy.getRootValueSeparator() + "] escapes="
                + (copy.getCharacterEscapes() != null) + " quote=" + copy.isEnabled(JsonGenerator.Feature.QUOTE_FIELD_NAMES));
        System.out.println("  rebuilt sep=[" + rebuilt.getRootValueSeparator() + "] escapes="
                + (rebuilt.getCharacterEscapes() != null) + " quote="
                + rebuilt.isEnabled(JsonGenerator.Feature.QUOTE_FIELD_NAMES));
        // 生成器级覆写
        StringWriter sw = new StringWriter();
        try (JsonGenerator g = new JsonFactory().createGenerator(sw)) {
            g.configure(JsonGenerator.Feature.WRITE_BIGDECIMAL_AS_PLAIN, true);
            g.writeStartArray();
            g.writeNumber(new java.math.BigDecimal("1e5"));
            g.disable(JsonGenerator.Feature.WRITE_BIGDECIMAL_AS_PLAIN);
            g.writeNumber(new java.math.BigDecimal("1e5"));
            g.setHighestNonEscapedChar(0x7f);
            g.writeString("\u00e9");
            g.writeEndArray();
            System.out.println("  per-gen features=" + g.getFeatureMask() + " highest=" + g.getHighestEscapedChar());
        }
        System.out.println("  per-gen " + sw);
        // 装饰器
        CountingDecorator od = new CountingDecorator();
        JsonFactory dec = JsonFactory.builder().outputDecorator(od).inputDecorator(new QuoteFixDecorator()).build();
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (JsonGenerator g = dec.createGenerator(bos)) {
            g.writeStartObject();
            g.writeStringField("k", "value");
            g.writeEndObject();
        }
        StringWriter dw = new StringWriter();
        try (JsonGenerator g = dec.createGenerator(dw)) {
            g.writeString("w");
        }
        System.out.println("  out " + bos.toString(StandardCharsets.UTF_8) + " | " + dw + " created=" + od.created);
        System.out.println("  in bytes " + tokens(dec, "{'a':['b']}", true));
        System.out.println("  in chars " + tokens(dec, "{'a':['b']}", false));
        try (JsonParser p = dec.createParser(new StringReader("['r']"))) {
            p.nextToken();
            p.nextToken();
            System.out.println("  in reader " + p.getText());
        }
    }

    static void parserLevel() throws Exception {
        System.out.println("── 解析器级开关");
        JsonFactory f = new JsonFactory();
        try (JsonParser p = f.createParser("{\"a\":[1,2]}")) {
            System.out.println("  default allowComments=" + p.isEnabled(JsonParser.Feature.ALLOW_COMMENTS)
                    + " autoClose=" + p.isEnabled(JsonParser.Feature.AUTO_CLOSE_SOURCE) + " mask=" + p.getFeatureMask());
            p.enable(JsonParser.Feature.ALLOW_COMMENTS);
            p.overrideStdFeatures(JsonParser.Feature.AUTO_CLOSE_SOURCE.getMask(), 0);
            System.out.println("  changed allowComments=" + p.isEnabled(JsonParser.Feature.ALLOW_COMMENTS)
                    + " autoClose=" + p.isEnabled(JsonParser.Feature.AUTO_CLOSE_SOURCE) + " mask=" + p.getFeatureMask());
            System.out.println("  readCaps=" + p.getReadCapabilities().asBitmask() + " canReadTypeId="
                    + p.canReadTypeId() + " canReadObjectId=" + p.canReadObjectId());
        }
        JsonFactory legacy = new JsonFactory();
        legacy.configure(JsonParser.Feature.ALLOW_SINGLE_QUOTES, true);
        legacy.enable(JsonParser.Feature.ALLOW_UNQUOTED_FIELD_NAMES);
        System.out.println("  legacy " + tokens(legacy, "{a:'b'}", false) + " | " + tokens(legacy, "{a:'b'}", true));
    }

    public static void main(String[] args) throws Exception {
        defaults();
        lenient();
        duplicates();
        constraints();
        writeFeatures();
        parserLevel();
    }
}
