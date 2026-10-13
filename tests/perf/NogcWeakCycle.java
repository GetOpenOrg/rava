// 无 GC「逃逸到旧对象但成环」判据探针（docs/plans/2026-10-13-nogc-phase2-design.md §一）。
//
// 构造：next 前向强引用 + prev 回指强引用——相邻节点互持成 2-环，引用计数无法回收；
// 整链经静态锚（head/tail）逃逸到旧对象，区域整组释放不适用（存入旧对象即判否）。
// 每轮迭代建一条链、摘锚（不可达），现状下成环链全部泄漏。
//
// 判据（分档）：
//   - 现状（无弱引用）：cyclic 模式 RSS 随迭代线性增长；acyclic 模式（prev 恒 null）平稳；
//   - 所有权推断 Weak 落地后（prev 弱化）：两模式都平稳，checksum 逐字不变。
// JDK 参照：两模式都平稳（GC 回收）。
//
// 用法：java NogcWeakCycle [迭代数=500000] [链长=8] [cyclic|acyclic=cyclic]
// 输出确定性 checksum（回归判据）；RSS 由外部 /usr/bin/time -f maxrss 测量。
public class NogcWeakCycle {
    static class Node {
        Node prev; // 回指（弱化目标）
        Node next; // 前向
        final int payload;
        Node(Node p, int v) { prev = p; payload = v; }
    }

    // 旧锚：静态字段（区域外已存在的一切对象 O 的代表）
    static Node head, tail;

    public static void main(String[] args) {
        int iters = args.length > 0 ? Integer.parseInt(args[0]) : 500_000;
        int len = args.length > 1 ? Integer.parseInt(args[1]) : 8;
        boolean cyclic = args.length <= 2 || !args[2].equals("acyclic");
        long checksum = 0;
        for (int i = 0; i < iters; i++) {
            Node h = new Node(null, i);
            Node t = h;
            for (int k = 1; k < len; k++) {
                t.next = new Node(cyclic ? t : null, i * 100 + k);
                t = t.next;
            }
            head = h; // 逃逸到旧对象（静态字段）
            tail = t;
            checksum += head.payload + tail.payload;
            head = null; // 摘锚：链不可达；cyclic 成环 → 泄漏，acyclic → 引用计数回收
            tail = null;
        }
        System.out.println("iters=" + iters + " len=" + len + " cyclic=" + cyclic + " checksum=" + checksum);
    }
}
