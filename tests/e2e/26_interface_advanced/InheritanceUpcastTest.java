// Test: user class inheritance upcasting (subclass → parent class parameter)

class Animal2 {
    String name;

    Animal2(String name) {
        this.name = name;
    }

    String describe() {
        return "Animal(" + name + ")";
    }
}

class Dog2 extends Animal2 {
    String breed;

    Dog2(String name, String breed) {
        super(name);
        this.breed = breed;
    }

    String describe() {
        return "Dog(" + name + "," + breed + ")";
    }
}

class Cat2 extends Animal2 {
    boolean indoor;

    Cat2(String name, boolean indoor) {
        super(name);
        this.indoor = indoor;
    }

    String describe() {
        return "Cat(" + name + "," + indoor + ")";
    }
}

class Shelter2 {
    Animal2[] animals;
    int count;

    Shelter2() {
        animals = new Animal2[10];
        count = 0;
    }

    void admit(Animal2 animal) {
        animals[count] = animal;
        count++;
    }

    void printAll() {
        for (int i = 0; i < count; i++) {
            System.out.println("  " + animals[i].describe());
        }
    }
}

public class InheritanceUpcastTest {
    static String greet(Animal2 a) {
        return "Hello, " + a.describe();
    }

    public static void main(String[] args) {
        Dog2 d = new Dog2("Rex", "Labrador");
        Cat2 c = new Cat2("Whiskers", true);

        // Upcasting: pass subclass to parent class parameter
        System.out.println(greet(d));
        System.out.println(greet(c));

        // Upcasting in collection-like context
        Shelter2 shelter = new Shelter2();
        shelter.admit(d);
        shelter.admit(c);
        System.out.println("Shelter (" + shelter.count + "):");
        shelter.printAll();

        // instanceof checks
        Animal2 a = d;
        System.out.println("a is Dog2: " + (a instanceof Dog2));
        System.out.println("a is Animal2: " + (a instanceof Animal2));

        System.out.println("Done.");
    }
}
