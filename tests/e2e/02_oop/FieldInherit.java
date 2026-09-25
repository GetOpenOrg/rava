public class FieldInherit {
    static class Animal {
        int age;
        Animal(int age) { this.age = age; }
    }

    static class Dog extends Animal {
        int tricks;
        Dog(int age, int tricks) {
            super(age);
            this.tricks = tricks;
        }
        int score() { return age + tricks; }
    }

    public static void main(String[] args) {
        Dog d = new Dog(5, 1);
        System.out.println(d.age);    // 5
        d.age = 10;
        System.out.println(d.age);    // 10
        Dog d2 = new Dog(5, 1);
        System.out.println(d2.score()); // 6
    }
}
