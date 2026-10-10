import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import org.springframework.util.CollectionUtils;
import org.springframework.util.LinkedCaseInsensitiveMap;
import org.springframework.util.LinkedMultiValueMap;
import org.springframework.util.MultiValueMap;
import org.springframework.util.StreamUtils;

/**
 * spring-core 切片 sc4（矩阵 #11）：StreamUtils 与 MultiValueMap。
 *
 * 覆盖面：StreamUtils（copyToByteArray / copyToString / copy 三形态 / copyRange / drain /
 * emptyInput / nonClosing）、LinkedMultiValueMap（add / addAll / getFirst / set /
 * toSingleValueMap / addIfAbsent / deepCopy / putAll）、CollectionUtils.toMultiValueMap /
 * unmodifiableMultiValueMap、LinkedCaseInsensitiveMap。
 */
public class SpringStreamMultiValueMain {

    /** 记录 close 调用，验证 nonClosing 包装。 */
    static class TrackingOutput extends ByteArrayOutputStream {
        boolean closed;

        @Override
        public void close() throws IOException {
            closed = true;
            super.close();
        }
    }

    public static void main(String[] args) throws IOException {
        // ── StreamUtils ──
        byte[] data = "Grüße, spring-core! 第一行\nsecond line".getBytes(StandardCharsets.UTF_8);
        byte[] copied = StreamUtils.copyToByteArray(new ByteArrayInputStream(data));
        System.out.println("copyToByteArray len=" + copied.length + " same=" + Arrays.equals(data, copied));
        System.out.println("copyToString=" + StreamUtils.copyToString(new ByteArrayInputStream(data), StandardCharsets.UTF_8));
        System.out.println("copyToString(null)='" + StreamUtils.copyToString((InputStream) null, StandardCharsets.UTF_8) + "'");

        ByteArrayOutputStream out = new ByteArrayOutputStream();
        int n = StreamUtils.copy(new ByteArrayInputStream(data), out);
        System.out.println("copy(in,out)=" + n + " out=" + out.size());
        StreamUtils.copy("abc", StandardCharsets.US_ASCII, out);
        StreamUtils.copy(new byte[] {1, 2, 3}, out);
        System.out.println("after copies out=" + out.size());
        System.out.println("copyToString(baos)=" + StreamUtils.copyToString(out, StandardCharsets.UTF_8).length());

        ByteArrayOutputStream range = new ByteArrayOutputStream();
        long r = StreamUtils.copyRange(new ByteArrayInputStream("0123456789".getBytes(StandardCharsets.US_ASCII)), range, 2, 6);
        System.out.println("copyRange=" + r + " '" + range.toString(StandardCharsets.US_ASCII) + "'");

        // 大于缓冲区（8192）的输入：多轮读写
        byte[] big = new byte[20000];
        for (int i = 0; i < big.length; i++) {
            big[i] = (byte) (i % 251);
        }
        ByteArrayOutputStream bigOut = new ByteArrayOutputStream();
        System.out.println("copy big=" + StreamUtils.copy(new ByteArrayInputStream(big), bigOut)
                + " same=" + Arrays.equals(big, bigOut.toByteArray()));
        System.out.println("drain=" + StreamUtils.drain(new ByteArrayInputStream(big)));
        System.out.println("emptyInput.read=" + StreamUtils.emptyInput().read());

        TrackingOutput tracking = new TrackingOutput();
        OutputStream guarded = StreamUtils.nonClosing(tracking);
        guarded.write('x');
        guarded.close();
        System.out.println("nonClosing closed=" + tracking.closed + " size=" + tracking.size());
        tracking.close();
        System.out.println("direct closed=" + tracking.closed);

        // ── LinkedMultiValueMap ──
        MultiValueMap<String, String> headers = new LinkedMultiValueMap<>();
        headers.add("Accept", "text/html");
        headers.add("Accept", "application/json");
        headers.add("Host", "example.org");
        headers.addAll("Cookie", Arrays.asList("a=1", "b=2"));
        headers.addIfAbsent("Host", "ignored.org");
        headers.addIfAbsent("Origin", "https://example.org");
        System.out.println("headers=" + headers);
        System.out.println("getFirst Accept=" + headers.getFirst("Accept") + " missing=" + headers.getFirst("X"));
        headers.set("Cookie", "c=3");
        System.out.println("after set Cookie=" + headers.get("Cookie"));
        Map<String, String> single = headers.toSingleValueMap();
        System.out.println("toSingleValueMap=" + single);
        System.out.println("asSingleValueMap.Accept=" + headers.asSingleValueMap().get("Accept"));

        LinkedMultiValueMap<String, String> copy = ((LinkedMultiValueMap<String, String>) headers).deepCopy();
        copy.add("Accept", "*/*");
        System.out.println("deepCopy Accept=" + copy.get("Accept") + " original=" + headers.get("Accept"));

        MultiValueMap<String, String> other = new LinkedMultiValueMap<>();
        other.add("Accept", "text/plain");
        other.add("X-Trace", "t1");
        headers.addAll(other);
        System.out.println("addAll(map)=" + headers);
        System.out.println("size=" + headers.size() + " keys=" + headers.keySet());

        Map<String, List<String>> raw = new LinkedHashMap<>();
        raw.put("k", new ArrayList<>(Arrays.asList("v1")));
        MultiValueMap<String, String> view = CollectionUtils.toMultiValueMap(raw);
        view.add("k", "v2");
        System.out.println("toMultiValueMap view=" + view + " raw=" + raw);
        MultiValueMap<String, String> ro = CollectionUtils.unmodifiableMultiValueMap(view);
        try {
            ro.add("k", "v3");
            System.out.println("unmodifiable add -> no exception");
        } catch (UnsupportedOperationException e) {
            System.out.println("unmodifiable add -> UnsupportedOperationException");
        }
        System.out.println("unmodifiable getFirst=" + ro.getFirst("k"));

        // ── LinkedCaseInsensitiveMap ──
        LinkedCaseInsensitiveMap<Integer> ci = new LinkedCaseInsensitiveMap<>();
        ci.put("Content-Type", 1);
        ci.put("content-length", 2);
        ci.put("CONTENT-TYPE", 3);
        System.out.println("ci=" + ci + " get=" + ci.get("content-type") + " contains=" + ci.containsKey("Content-Length"));
        ci.remove("CONTENT-length");
        System.out.println("ci after remove=" + ci);
        System.out.println("done");
    }
}
