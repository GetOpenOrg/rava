class GenericBasic {
    static class Pair<A, B> {
        Object first;
        Object second;
        Pair(Object a, Object b) { first = a; second = b; }
        Object getFirst() { return first; }
        Object getSecond() { return second; }
    }
    public static void main(String[] args) {
        Pair<String, Integer> p = new Pair<>("hello", 42);
        System.out.println((String) p.getFirst());
        System.out.println((Integer) p.getSecond());
    }
}
