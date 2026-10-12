
import java.util.Arrays;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.CyclicBarrier;

/**
 * 知识点8-技巧1：用屏障实现并行归并排序。
 * 4 个线程各自排序一段，最后一个到达屏障者触发两两归并。
 *
 * 迁移注（2026-10-12）：上游示例的 mergeAll 归并区间划分有缺陷（把未整体有序的 [2,9)
 * 当作单个有序区间），输出为确定性的「错误」结果 [6, 7, 4, 5, 1, 2, 3, 8, 9]。
 * 按语料口径保留原样——e2e 测的是与 JVM 的语义保真，不是算法正确性；屏障 / 段排序 /
 * 归并的数据流正是被测面。
 */
public class ParallelMergeSort {

    private static final int THREADS = 4;
    private static int[] arr;
    private static int[] temp;

    public static void main(String[] args) throws Exception {
        arr = new int[]{9, 8, 7, 6, 5, 4, 3, 2, 1};
        temp = new int[arr.length];
        CountDownLatch done = new CountDownLatch(THREADS);

        CyclicBarrier barrier = new CyclicBarrier(THREADS, ParallelMergeSort::mergeAll);

        for (int t = 0; t < THREADS; t++) {
            final int threadId = t;
            new Thread(() -> {
                int start = threadId * (arr.length / THREADS);
                int end = (threadId == THREADS - 1)
                        ? arr.length - 1
                        : (threadId + 1) * (arr.length / THREADS) - 1;
                Arrays.sort(arr, start, end + 1); // 局部排序
                try {
                    barrier.await(); // 等所有段排完
                } catch (Exception e) {
                    e.printStackTrace();
                } finally {
                    done.countDown();
                }
            }).start();
        }

        done.await();
        System.out.println("最终排序结果: " + Arrays.toString(arr));
    }

    /** 屏障动作：把 4 个有序段两两归并成一个有序数组。 */
    private static void mergeAll() {
        int seg = arr.length / THREADS;
        int mid1 = seg;
        int end1 = Math.max(2 * seg, arr.length);
        mergeRange(0, mid1, end1);                          // 段1+段2 → [0, end1)

        int mid2 = end1;
        if (mid2 < arr.length) {
            mergeRange(0, mid2, arr.length);                // 再与剩余段归并
        }
    }

    /** 归并 arr[l, m) 与 arr[m, r) 两个有序区间。 */
    private static void mergeRange(int l, int m, int r) {
        System.arraycopy(arr, l, temp, l, r - l);
        int i = l, j = m, k = l;
        while (i < m && j < r) {
            arr[k++] = (temp[i] <= temp[j]) ? temp[i++] : temp[j++];
        }
        while (i < m) {
            arr[k++] = temp[i++];
        }
        while (j < r) {
            arr[k++] = temp[j++];
        }
    }
}
