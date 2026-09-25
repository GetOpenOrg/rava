public class InstanceOfInherit {
    static class Animal {}
    static class Dog extends Animal {}
    static class Cat extends Animal {}

    public static void main(String[] args) {
        Animal dog  = new Dog();
        Animal cat  = new Cat();
        Animal base = new Animal();

        // Same type checks
        System.out.println(dog instanceof Dog);    // true
        System.out.println(cat instanceof Cat);    // true
        // Cross-sibling checks via parent variable
        System.out.println(dog instanceof Cat);    // false
        System.out.println(cat instanceof Dog);    // false
        // Parent type check
        System.out.println(dog instanceof Animal); // true
        System.out.println(base instanceof Dog);   // false
    }
}
