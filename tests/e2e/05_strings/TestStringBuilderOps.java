public class TestStringBuilderOps {

    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder("Hello");

        // append 各种类型
        sb.append(' ').append("World").append(42).append(true).append(3.5);
        System.out.println(sb);

        // insert
        StringBuilder ins = new StringBuilder("abcdef");
        ins.insert(0, "[");
        ins.insert(ins.length(), "]");
        ins.insert(4, "-X-");
        System.out.println(ins);

        // insert 原生类型
        StringBuilder ins2 = new StringBuilder("v=");
        ins2.insert(2, 123);
        System.out.println(ins2);

        // delete / deleteCharAt
        StringBuilder del = new StringBuilder("0123456789");
        del.delete(2, 5);
        System.out.println("after delete(2,5)=" + del);
        del.deleteCharAt(0);
        System.out.println("after deleteCharAt(0)=" + del);

        // replace
        StringBuilder rep = new StringBuilder("a-b-c");
        rep.replace(1, 2, ":");
        System.out.println(rep);
        StringBuilder rep2 = new StringBuilder("foo bar foo");
        rep2.replace(0, 3, "baz");
        System.out.println(rep2);

        // reverse
        System.out.println(new StringBuilder("abcd").reverse());

        // setCharAt / setLength
        StringBuilder sc = new StringBuilder("hello");
        sc.setCharAt(0, 'H');
        System.out.println(sc);
        sc.setLength(3);
        System.out.println("truncated=" + sc);
        sc.setLength(5);
        System.out.println("padded len=" + sc.length() + " val=[" + sc + "]");

        // indexOf / lastIndexOf / charAt / substring
        StringBuilder find = new StringBuilder("one two one three");
        System.out.println("indexOf(one)=" + find.indexOf("one"));
        System.out.println("lastIndexOf(one)=" + find.lastIndexOf("one"));
        System.out.println("indexOf(two,5)=" + find.indexOf("two", 5));
        System.out.println("charAt(4)=" + find.charAt(4));
        System.out.println("substring(4,7)=" + find.substring(4, 7));

        // capacity / ensureCapacity / length
        StringBuilder cap = new StringBuilder();
        System.out.println("default capacity>=16 -> " + (cap.capacity() >= 16));
        cap.ensureCapacity(100);
        System.out.println("capacity after ensure>=100 -> " + (cap.capacity() >= 100));
        cap.append("0123456789");
        System.out.println("length=" + cap.length());

        // 链式调用与 toString 复用
        StringBuilder chain = new StringBuilder();
        String result = chain.append("a").append("b").append("c").toString();
        chain.append("d");
        System.out.println("result=" + result + " chain=" + chain);

        // 循环拼接
        StringBuilder loop = new StringBuilder();
        for (int i = 0; i < 5; i++) {
            loop.append(i);
            if (i < 4) loop.append(",");
        }
        System.out.println("loop=" + loop);

        // 初始值与扩容：追加长字符串
        StringBuilder grow = new StringBuilder(1);
        grow.append("0123456789".repeat(3));
        System.out.println("grow len=" + grow.length() + " tail=" + grow.substring(25));

        System.out.println("done");
    }
}
