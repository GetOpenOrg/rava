import java.util.Scanner;

public class ScannerInputTest {
    public static void main(String[] args) {
        // Test 1: Scanner from String - nextLine
        System.out.println("=== Scanner from String ===");
        Scanner sc1 = new Scanner("Hello World\nSecond Line\nThird Line");
        System.out.println(sc1.nextLine());
        System.out.println(sc1.nextLine());
        System.out.println(sc1.nextLine());
        sc1.close();

        // Test 2: Scanner from String - nextInt
        System.out.println("=== nextInt ===");
        Scanner sc2 = new Scanner("42 17 99");
        System.out.println(sc2.nextInt());
        System.out.println(sc2.nextInt());
        System.out.println(sc2.nextInt());
        sc2.close();

        // Test 3: Scanner from String - next (token)
        System.out.println("=== next token ===");
        Scanner sc3 = new Scanner("apple banana cherry");
        System.out.println(sc3.next());
        System.out.println(sc3.next());
        System.out.println(sc3.next());
        sc3.close();

        // Test 4: Scanner from String - hasNext / hasNextLine
        System.out.println("=== hasNext ===");
        Scanner sc4 = new Scanner("one two");
        System.out.println(sc4.hasNext());
        System.out.println(sc4.next());
        System.out.println(sc4.hasNext());
        System.out.println(sc4.next());
        System.out.println(sc4.hasNext());
        sc4.close();

        // Test 5: Scanner from String - nextDouble
        System.out.println("=== nextDouble ===");
        Scanner sc5 = new Scanner("3.14 2.718");
        System.out.println(sc5.nextDouble());
        System.out.println(sc5.nextDouble());
        sc5.close();

        System.out.println("Scanner input tests passed");
    }
}
