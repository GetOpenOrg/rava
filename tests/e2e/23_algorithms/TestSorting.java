public class TestSorting {

    static void bubbleSort(int[] arr) {
        int n = arr.length;
        for (int i = 0; i < n - 1; i++) {
            for (int j = 0; j < n - i - 1; j++) {
                if (arr[j] > arr[j + 1]) {
                    int tmp = arr[j];
                    arr[j] = arr[j + 1];
                    arr[j + 1] = tmp;
                }
            }
        }
    }

    static void selectionSort(int[] arr) {
        int n = arr.length;
        for (int i = 0; i < n - 1; i++) {
            int minIdx = i;
            for (int j = i + 1; j < n; j++) {
                if (arr[j] < arr[minIdx]) minIdx = j;
            }
            int tmp = arr[minIdx];
            arr[minIdx] = arr[i];
            arr[i] = tmp;
        }
    }

    static void insertionSort(int[] arr) {
        int n = arr.length;
        for (int i = 1; i < n; i++) {
            int key = arr[i];
            int j = i - 1;
            while (j >= 0 && arr[j] > key) {
                arr[j + 1] = arr[j];
                j--;
            }
            arr[j + 1] = key;
        }
    }

    static int binarySearch(int[] arr, int target) {
        int lo = 0, hi = arr.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (arr[mid] == target) return mid;
            if (arr[mid] < target) lo = mid + 1;
            else hi = mid - 1;
        }
        return -1;
    }

    static void printArray(int[] arr) {
        for (int i = 0; i < arr.length; i++) {
            if (i > 0) System.out.print(" ");
            System.out.print(arr[i]);
        }
        System.out.println();
    }

    public static void main(String[] args) {
        int[] a = {5, 2, 8, 1, 9, 3};
        bubbleSort(a);
        printArray(a);  // 1 2 3 5 8 9

        int[] b = {64, 34, 25, 12, 22, 11, 90};
        selectionSort(b);
        printArray(b);  // 11 12 22 25 34 64 90

        int[] c = {12, 11, 13, 5, 6};
        insertionSort(c);
        printArray(c);  // 5 6 11 12 13

        // binary search
        System.out.println(binarySearch(a, 5));   // 3
        System.out.println(binarySearch(a, 7));   // -1
        System.out.println(binarySearch(c, 11));  // 2

        // already sorted
        int[] d = {1, 2, 3, 4, 5};
        bubbleSort(d);
        printArray(d);  // 1 2 3 4 5

        // reverse sorted
        int[] e = {5, 4, 3, 2, 1};
        insertionSort(e);
        printArray(e);  // 1 2 3 4 5
    }
}
