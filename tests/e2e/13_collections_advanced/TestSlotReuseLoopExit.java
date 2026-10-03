// 边界：循环变量与循环后同名新变量复用同一槽（LVT 两段、类型不同），循环出口代码结构化进循环体
// 的 return 分支（同 Hashtable.reconstitutionPut）；循环尾 `e = e.next` 须写回外层绑定
public class TestSlotReuseLoopExit {
    static class Node<K> {
        final int hash;
        final K key;
        Node<K> next;

        Node(int hash, K key, Node<K> next) {
            this.hash = hash;
            this.key = key;
            this.next = next;
        }
    }

    static class Table<K> {
        int count;
        int probes;

        void put(Node<?>[] tab, K key) {
            int hash = key.hashCode();
            int index = (hash & 0x7FFFFFFF) % tab.length;
            for (Node<?> e = tab[index]; e != null; e = e.next) {
                probes++;
                if ((e.hash == hash) && e.key.equals(key)) {
                    throw new IllegalStateException("duplicate " + key);
                }
            }
            @SuppressWarnings("unchecked")
            Node<K> e = (Node<K>) tab[index];
            tab[index] = new Node<>(hash, key, e);
            count++;
        }
    }

    public static void main(String[] args) {
        Node<?>[] tab = new Node<?>[2];
        Table<String> t = new Table<>();
        String[] keys = {"a", "b", "c", "d", "e"};
        for (String k : keys) {
            t.put(tab, k);
        }
        System.out.println("count=" + t.count + " probes=" + t.probes);
        try {
            t.put(tab, "c");
        } catch (IllegalStateException ex) {
            System.out.println(ex.getMessage() + " probes=" + t.probes);
        }
        for (int i = 0; i < tab.length; i++) {
            StringBuilder sb = new StringBuilder();
            for (Node<?> n = tab[i]; n != null; n = n.next) {
                sb.append(n.key);
            }
            System.out.println(i + ":" + sb);
        }
    }
}
