import java.nio.ByteBuffer;
import java.nio.CharBuffer;
import java.nio.charset.CharacterCodingException;
import java.nio.charset.CoderResult;
import java.nio.charset.CodingErrorAction;
import java.nio.charset.StandardCharsets;

/**
 * CoderResult 谓词与 throwException（方法级实测：isError 9 / throwException 8 /
 * isOverflow 8 / isUnderflow 7，此前零覆盖）：OVERFLOW/UNDERFLOW/MALFORMED 三态、
 * 手动编码循环驱动真实结果。
 */
public class TestCoderResultFaces {

    public static void main(String[] args) throws Exception {
        System.out.println("static-overflow=" + CoderResult.OVERFLOW.isOverflow()
                + " underflow=" + CoderResult.UNDERFLOW.isUnderflow()
                + " neither-error=" + (!CoderResult.OVERFLOW.isError()
                        && !CoderResult.UNDERFLOW.isError()));

        // malformedForLength 构造：isError true
        CoderResult malformed = CoderResult.malformedForLength(2);
        System.out.println("malformed-is-error=" + malformed.isError()
                + " len=" + malformed.length());
        CoderResult unmappable = CoderResult.unmappableForLength(1);
        System.out.println("unmappable-is-error=" + unmappable.isError());

        // 手动循环：小输出缓冲 → OVERFLOW；throwException → BufferOverflowException
        var enc = StandardCharsets.US_ASCII.newEncoder();
        enc.onUnmappableCharacter(CodingErrorAction.REPORT);
        ByteBuffer small = ByteBuffer.allocate(1);
        CoderResult r = enc.encode(CharBuffer.wrap("hi"), small, true);
        System.out.println("loop-overflow=" + r.isOverflow()
                + " not-error=" + !r.isError());
        try {
            r.throwException();
        } catch (java.nio.BufferOverflowException e) {
            System.out.println("overflow-throws=" + e.getClass().getSimpleName());
        }

        // 全部消费（endOfInput=true）→ UNDERFLOW；UNDERFLOW.throwException() 抛 BufferUnderflowException
        var enc2 = StandardCharsets.US_ASCII.newEncoder();
        enc2.onUnmappableCharacter(CodingErrorAction.REPORT);
        CoderResult r2 = enc2.encode(CharBuffer.wrap("hi"), ByteBuffer.allocate(8), true);
        System.out.println("loop-underflow=" + r2.isUnderflow());
        try {
            r2.throwException();
        } catch (java.nio.BufferUnderflowException e) {
            System.out.println("underflow-throws=" + e.getClass().getSimpleName());
        }
        CoderResult fr = enc2.flush(ByteBuffer.allocate(8));
        System.out.println("flush-underflow=" + fr.isUnderflow());

        // 不可映射 → isError + UnmappableCharacterException
        var enc3 = StandardCharsets.US_ASCII.newEncoder();
        enc3.onUnmappableCharacter(CodingErrorAction.REPORT);
        CoderResult r3 = enc3.encode(CharBuffer.wrap("中"), ByteBuffer.allocate(8), true);
        System.out.println("unmappable-result=" + r3.isError() + " len=" + r3.length());
        try {
            r3.throwException();
        } catch (CharacterCodingException e) {
            System.out.println("unmappable-throws=" + e.getClass().getSimpleName());
        }
    }
}
