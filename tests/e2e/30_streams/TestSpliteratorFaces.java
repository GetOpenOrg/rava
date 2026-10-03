import java.util.Arrays;
import java.util.List;
import java.util.Spliterator;
import java.util.stream.Collectors;
import java.util.stream.StreamSupport;

/**
 * Spliterator 族（方法级实测：spliterator 12 jar / characteristics 5 /
 * spliteratorUnknownSize 10，streams 语料未直接触达）：特征位、tryAdvance/forEachRemaining、
 * trySplit 二分、estimateSize/getExactSizeIfKnown、未知大小形态。
 */
public class TestSpliteratorFaces {

    public static void main(String[] args) {
        List<String> data = Arrays.asList("a", "b", "c", "d");
        Spliterator<String> sp = data.spliterator();

        System.out.println("exact-size=" + sp.getExactSizeIfKnown() + " estimate=" + sp.estimateSize());
        int chars = sp.characteristics();
        System.out.println("ordered=" + sp.hasCharacteristics(Spliterator.ORDERED)
                + " sized=" + sp.hasCharacteristics(Spliterator.SIZED)
                + " subsized=" + sp.hasCharacteristics(Spliterator.SUBSIZED));

        // tryAdvance 逐个
        StringBuilder adv = new StringBuilder();
        while (sp.tryAdvance(s -> adv.append(s).append(";"))) {
            // 消费
        }
        System.out.println("advance=" + adv);

        // forEachRemaining
        Spliterator<String> sp2 = data.spliterator();
        sp2.tryAdvance(s -> {
        });                      // 先吃一个
        StringBuilder rest = new StringBuilder();
        sp2.forEachRemaining(s -> rest.append(s));
        System.out.println("remaining=" + rest);

        // trySplit：前半留在原，后半给新分裂器
        Spliterator<String> sp3 = data.spliterator();
        Spliterator<String> half = sp3.trySplit();
        System.out.println("half-size=" + half.estimateSize() + " orig-size=" + sp3.estimateSize());

        // 未知大小形态（迭代器包装，spliteratorUnknownSize 的公开通道）
        java.util.Iterator<String> it = Arrays.asList("x", "y").iterator();
        Spliterator<String> unknown = Spliterators8.spliteratorUnknownSize(it);
        System.out.println("unknown-exact=" + unknown.getExactSizeIfKnown()
                + " max=" + (unknown.estimateSize() == Long.MAX_VALUE));

        // 组装回 stream
        String joined = StreamSupport.stream(data.spliterator(), false)
                .collect(Collectors.joining("-"));
        System.out.println("stream-join=" + joined);
    }

    static class Spliterators8 {
        static <T> Spliterator<T> spliteratorUnknownSize(java.util.Iterator<? extends T> it) {
            return java.util.Spliterators.spliteratorUnknownSize(it,
                    Spliterator.ORDERED | Spliterator.NONNULL);
        }
    }
}
