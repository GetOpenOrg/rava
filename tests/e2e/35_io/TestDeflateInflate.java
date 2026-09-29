import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.util.HexFormat;
import java.util.zip.DataFormatException;
import java.util.zip.Deflater;
import java.util.zip.GZIPInputStream;
import java.util.zip.GZIPOutputStream;
import java.util.zip.Inflater;

public class TestDeflateInflate {
    public static void main(String[] args) throws Exception {
        byte[] input = "The quick brown fox jumps over the lazy dog. The quick brown fox again!".getBytes("UTF-8");

        // Deflater 默认级别：压缩字节与 JVM（zlib）逐字节一致
        Deflater def = new Deflater();
        def.setInput(input);
        def.finish();
        byte[] buf = new byte[256];
        int clen = def.deflate(buf);
        System.out.println("deflated " + input.length + " -> " + clen + " finished=" + def.finished());
        System.out.println(HexFormat.of().formatHex(buf, 0, clen));
        System.out.println("adler=" + Integer.toHexString(def.getAdler()) + " in=" + def.getTotalIn() + " out=" + def.getTotalOut());
        def.end();

        // Inflater 还原
        Inflater inf = new Inflater();
        inf.setInput(buf, 0, clen);
        byte[] out = new byte[256];
        int n = inf.inflate(out);
        System.out.println("inflated " + n + " finished=" + inf.finished() + " ok=" + new String(out, 0, n, "UTF-8").equals(new String(input, "UTF-8")));
        inf.end();

        // 最佳压缩 + 原始 deflate（nowrap）
        Deflater raw = new Deflater(Deflater.BEST_COMPRESSION, true);
        raw.setInput(input);
        raw.finish();
        int rlen = raw.deflate(buf);
        System.out.println("raw " + rlen + " " + HexFormat.of().formatHex(buf, 0, Math.min(rlen, 16)));
        raw.end();
        Inflater rinf = new Inflater(true);
        rinf.setInput(buf, 0, rlen);
        System.out.println("raw inflated " + rinf.inflate(out));
        rinf.end();

        // 小输出缓冲逐块 inflate
        Deflater d2 = new Deflater();
        d2.setInput(input);
        d2.finish();
        int l2 = d2.deflate(buf);
        d2.end();
        Inflater i2 = new Inflater();
        i2.setInput(buf, 0, l2);
        byte[] small = new byte[10];
        StringBuilder sb = new StringBuilder();
        while (!i2.finished()) {
            int k = i2.inflate(small);
            sb.append(new String(small, 0, k, "UTF-8"));
        }
        System.out.println("chunked ok=" + sb.toString().equals(new String(input, "UTF-8")) + " remaining=" + i2.getRemaining());
        i2.end();

        // GZIP 往返
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (GZIPOutputStream gz = new GZIPOutputStream(bos)) {
            for (int i = 0; i < 20; i++) gz.write(input);
        }
        byte[] gzBytes = bos.toByteArray();
        System.out.println("gzip size=" + gzBytes.length + " head=" + HexFormat.of().formatHex(gzBytes, 0, 4));
        try (GZIPInputStream gin = new GZIPInputStream(new ByteArrayInputStream(gzBytes))) {
            byte[] all = gin.readAllBytes();
            System.out.println("gunzip " + all.length);
        }

        // 坏数据 → DataFormatException
        Inflater bad = new Inflater();
        bad.setInput(new byte[]{1, 2, 3, 4, 5, 6});
        try {
            bad.inflate(out);
            System.out.println("no exception");
        } catch (DataFormatException e) {
            System.out.println("DataFormatException: " + e.getMessage());
        }
        bad.end();
    }
}
