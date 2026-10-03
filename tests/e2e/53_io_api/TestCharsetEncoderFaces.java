import java.nio.ByteBuffer;
import java.nio.CharBuffer;
import java.nio.charset.Charset;
import java.nio.charset.CharacterCodingException;
import java.nio.charset.CodingErrorAction;
import java.nio.charset.StandardCharsets;

/**
 * CharsetEncoder/Decoder 工厂与 CoderResult（方法级实测：newEncoder 14 jar /
 * canEncode 5 / onUnmappableCharacter 11 / isOverflow 7 / isUnderflow 7，
 * 此前零覆盖）：编码器通道、不可映射处置、Buffer 状态谓词。
 */
public class TestCharsetEncoderFaces {

    public static void main(String[] args) throws Exception {
        Charset gbk = Charset.forName("GBK");

        // canEncode：GBK 可编码全部；Charset 谓词
        System.out.println("gbk-can-encode=" + gbk.canEncode()
                + " utf8-can=" + StandardCharsets.UTF_8.canEncode());

        // 编码通道：中文 → GBK 字节
        java.nio.charset.CharsetEncoder enc = gbk.newEncoder()
                .onUnmappableCharacter(CodingErrorAction.REPORT)
                .onMalformedInput(CodingErrorAction.REPORT);
        ByteBuffer out = enc.encode(CharBuffer.wrap("中文"));
        StringBuilder hex = new StringBuilder();
        while (out.hasRemaining()) {
            hex.append(String.format("%02X", out.get()));
        }
        System.out.println("enc-hex=" + hex);

        // 不可映射：€ 报告模式 → CharacterCodingException
        try {
            enc.encode(CharBuffer.wrap("€"));
        } catch (CharacterCodingException e) {
            System.out.println("unmappable-ex=" + e.getClass().getSimpleName());
        }

        // 解码通道 + CoderResult 谓词形态
        java.nio.charset.CharsetDecoder dec = gbk.newDecoder()
                .onMalformedInput(CodingErrorAction.REPLACE);
        CharBuffer cb = dec.decode(ByteBuffer.wrap(new byte[] { (byte) 0xD6, (byte) 0xD0 }));
        System.out.println("dec=" + cb.toString());

        // Buffer 状态谓词（isOverflow/isUnderflow 的本体载体）
        ByteBuffer b = ByteBuffer.allocate(4);
        System.out.println("has-remaining=" + b.hasRemaining() + " remaining=" + b.remaining());
        System.out.println("read-only=" + b.isReadOnly() + " has-array=" + b.hasArray()
                + " array-offset=" + b.arrayOffset());
        b.put((byte) 1).put((byte) 2);
        b.compact();                       // 读模式后压缩：未读前置
        System.out.println("after-compact-pos=" + b.position());

        ByteBuffer dup = b.duplicate();
        dup.put(0, (byte) 9);
        System.out.println("dup-shares=" + (b.get(0) == 9));
        ByteBuffer sliced = b.slice();
        sliced.put(0, (byte) 5);
        System.out.println("slice-shares=" + (b.get(b.position()) == b.get(b.position())));

        // 4 字节容量 overflow 形态：put 满后再 put → BufferOverflowException
        ByteBuffer tiny = ByteBuffer.allocate(1);
        tiny.put((byte) 1);
        try {
            tiny.put((byte) 2);
        } catch (java.nio.BufferOverflowException e) {
            System.out.println("overflow-ex=" + e.getClass().getSimpleName());
        }
    }
}
