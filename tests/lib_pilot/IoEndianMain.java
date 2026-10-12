
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;

import org.apache.commons.io.EndianUtils;
import org.apache.commons.io.input.CharSequenceReader;
import org.apache.commons.io.output.WriterOutputStream;

/**
 * commons-io pilot A18：字节序读写与 Reader→OutputStream 桥（W1 #5）。
 */
public class IoEndianMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws IOException {
        // EndianUtils：小端写读
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        EndianUtils.writeSwappedInteger(bos, 0x12345678);
        byte[] le = bos.toByteArray();
        check("swap-write-bytes", "78563412",
                java.util.HexFormat.of().formatHex(le));
        check("swap-read", 0x12345678,
                EndianUtils.readSwappedInteger(new ByteArrayInputStream(le)));

        // long 的swap
        ByteArrayOutputStream bosL = new ByteArrayOutputStream();
        EndianUtils.writeSwappedLong(bosL, 0x0102030405060708L);
        check("swap-long", "0807060504030201",
                java.util.HexFormat.of().formatHex(bosL.toByteArray()));

        // swapInteger：值级字节交换（不写流）
        check("swap-value", 0x78563412, EndianUtils.swapInteger(0x12345678));
        check("swap-short-value", 0x3412, EndianUtils.swapShort((short) 0x1234));

        // 与 DataOutputStream 大端对照
        ByteArrayOutputStream be = new ByteArrayOutputStream();
        try (DataOutputStream out = new DataOutputStream(be)) {
            out.writeInt(0x12345678);
        }
        check("big-endian-bytes", "12345678", java.util.HexFormat.of().formatHex(be.toByteArray()));
        try (DataInputStream in = new DataInputStream(new ByteArrayInputStream(be.toByteArray()))) {
            check("big-endian-read", 0x12345678, in.readInt());
        }

        // WriterOutputStream：字符按字符集编码回流（内存往返）
        ByteArrayOutputStream encoded = new ByteArrayOutputStream();
        try (WriterOutputStream wos = new WriterOutputStream(
                new java.io.OutputStreamWriter(encoded, java.nio.charset.StandardCharsets.UTF_8),
                java.nio.charset.StandardCharsets.UTF_8)) {
            wos.write("桥接".getBytes(java.nio.charset.StandardCharsets.UTF_8));
        }
        check("writer-output-stream", "桥接",
                encoded.toString(java.nio.charset.StandardCharsets.UTF_8));

        // CharSequenceReader：直接读字符串片段
        CharSequenceReader reader = new CharSequenceReader("abcdef");
        char[] buf = new char[3];
        reader.read(buf);
        check("charseq-read", "abc", new String(buf));
        reader.skip(1);
        reader.read(buf);
        check("charseq-after-skip", "ef", new String(buf, 0, 2));
        System.out.println("DONE IoEndianMain");
    }
}
