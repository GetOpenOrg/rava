import java.io.ByteArrayOutputStream;
import java.io.StringWriter;

/**
 * StringBuffer 遗留通道与 StringWriter 别名（方法级实测：StringWriter.getBuffer 8 jar /
 * ByteArrayOutputStream.writeTo 5，StringBuilder 已有 149 处而 StringBuffer 独立类零专项）：
 * getBuffer 返回活缓冲（写入穿透）、变更操作族。
 */
public class TestStringBufferAliases {

    public static void main(String[] args) throws Exception {
        StringWriter sw = new StringWriter();
        sw.write("base");
        StringBuffer live = sw.getBuffer();
        live.append("-via-buffer");            // 别名写入穿透到 writer 输出
        System.out.println("alias-through=" + sw.toString());
        live.insert(0, "[");
        System.out.println("alias-insert=" + sw.toString());
        System.out.println("alias-same=" + (sw.getBuffer() == live));

        // StringBuffer 变更族
        StringBuffer sb = new StringBuffer("abcdef");
        sb.setCharAt(0, 'X');
        sb.deleteCharAt(6 - 1);
        System.out.println("mutated=" + sb);
        sb.setLength(3);
        System.out.println("trimmed=" + sb + " len=" + sb.length());
        sb.setLength(5);
        System.out.println("padded=[" + sb + "] len=" + sb.length());
        StringBuffer rv = new StringBuffer("abc");
        System.out.println("reversed=" + rv.reverse());
        System.out.println("capacity-grow=" + (new StringBuffer(4).append("12345").capacity() >= 5));
        System.out.println("substring=" + new StringBuffer("hello").substring(1, 3));

        // ByteArrayOutputStream.writeTo：内容转投他处
        ByteArrayOutputStream src = new ByteArrayOutputStream();
        src.write("copy-me".getBytes("UTF-8"));
        ByteArrayOutputStream dst = new ByteArrayOutputStream();
        src.writeTo(dst);
        System.out.println("written-to=" + dst.toString("UTF-8"));
        System.out.println("src-intact=" + src.toString("UTF-8"));
    }
}
