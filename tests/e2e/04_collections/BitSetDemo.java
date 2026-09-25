import java.util.BitSet;

public class BitSetDemo {
    public static void main(String[] args) {
        BitSet bs = new BitSet(16);
        bs.set(0);
        bs.set(3);
        bs.set(7);
        bs.set(15);

        System.out.println(bs.get(0));
        System.out.println(bs.get(1));
        System.out.println(bs.get(3));
        System.out.println(bs.get(15));
        System.out.println(bs.cardinality());
        System.out.println(bs.size() >= 16);
        System.out.println(bs.length());

        // and / or / xor
        BitSet a = new BitSet();
        a.set(1); a.set(2); a.set(3);
        BitSet b = new BitSet();
        b.set(2); b.set(3); b.set(4);

        BitSet andResult = (BitSet) a.clone();
        andResult.and(b);
        System.out.println(andResult.cardinality());
        System.out.println(andResult.get(2));
        System.out.println(andResult.get(3));

        BitSet orResult = (BitSet) a.clone();
        orResult.or(b);
        System.out.println(orResult.cardinality());

        BitSet xorResult = (BitSet) a.clone();
        xorResult.xor(b);
        System.out.println(xorResult.cardinality());

        // clear / flip
        BitSet c = new BitSet();
        c.set(0, 5);
        System.out.println(c.cardinality());
        c.clear(2);
        System.out.println(c.cardinality());
        c.flip(0);
        System.out.println(c.cardinality());

        // nextSetBit / nextClearBit
        BitSet d = new BitSet();
        d.set(2); d.set(5); d.set(8);
        System.out.println(d.nextSetBit(0));
        System.out.println(d.nextSetBit(3));
        System.out.println(d.nextClearBit(0));

        System.out.println("done");
    }
}
