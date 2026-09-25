import java.util.ArrayList;
import java.util.List;

interface Printable {
    String toPrintString();
}

class Dog implements Printable {
    String name;
    int age;

    Dog(String name, int age) {
        this.name = name;
        this.age = age;
    }

    public String toPrintString() {
        return "Dog(" + name + "," + age + ")";
    }
}

class Cat implements Printable {
    String name;

    Cat(String name) {
        this.name = name;
    }

    public String toPrintString() {
        return "Cat(" + name + ")";
    }
}

class AnimalShelter {
    List<Printable> animals;

    AnimalShelter() {
        animals = new ArrayList<>();
    }

    void admit(Printable animal) {
        animals.add(animal);
    }

    void printAll() {
        for (int i = 0; i < animals.size(); i++) {
            Printable p = animals.get(i);
            System.out.println("  " + p.toPrintString());
        }
    }

    int count() {
        return animals.size();
    }
}

public class InterfacePolyTest {
    public static void main(String[] args) {
        Dog d1 = new Dog("Rex", 5);
        Dog d2 = new Dog("Spot", 3);
        Cat c1 = new Cat("Whiskers");

        System.out.println(d1.toPrintString());
        System.out.println(c1.toPrintString());

        AnimalShelter shelter = new AnimalShelter();
        shelter.admit(d1);
        shelter.admit(d2);
        shelter.admit(c1);

        System.out.println("Shelter (" + shelter.count() + " animals):");
        shelter.printAll();

        // instanceof
        System.out.println("d1 is Printable: " + (d1 instanceof Printable));

        System.out.println("Done.");
    }
}
