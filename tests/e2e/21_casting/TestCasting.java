public class TestCasting {

    static class Animal {
        String name;
        Animal(String name) { this.name = name; }
        String speak() { return "..."; }
    }

    static class Dog extends Animal {
        Dog(String name) { super(name); }
        @Override
        String speak() { return "Woof"; }
        void fetch() { System.out.println(name + " fetches!"); }
    }

    static class Cat extends Animal {
        Cat(String name) { super(name); }
        @Override
        String speak() { return "Meow"; }
        void purr() { System.out.println(name + " purrs!"); }
    }

    public static void main(String[] args) {
        // widening (upcasting)
        Animal a1 = new Dog("Rex");
        Animal a2 = new Cat("Whiskers");
        System.out.println(a1.speak());   // Woof
        System.out.println(a2.speak());   // Meow

        // instanceof check before downcast
        if (a1 instanceof Dog) {
            Dog d = (Dog) a1;
            d.fetch();  // Rex fetches!
        }

        Animal[] animals = { new Dog("Buddy"), new Cat("Luna"), new Dog("Max") };
        for (Animal a : animals) {
            System.out.println(a.speak());
            if (a instanceof Dog) {
                ((Dog) a).fetch();
            } else if (a instanceof Cat) {
                ((Cat) a).purr();
            }
        }
        // Woof, Buddy fetches!, Meow, Luna purrs!, Woof, Max fetches!

        // numeric casts
        double pi = 3.14159;
        int truncated = (int) pi;
        System.out.println(truncated);      // 3

        long big = 100000L;
        int small = (int) big;
        System.out.println(small);          // 100000

        int val = 65;
        char ch = (char) val;
        System.out.println(ch);             // A

        // widening numeric
        int x = 42;
        double d = x;
        System.out.println(d);              // 42.0

        // instanceof with false
        System.out.println(a2 instanceof Dog);  // false
        System.out.println(a1 instanceof Cat);  // false
    }
}
