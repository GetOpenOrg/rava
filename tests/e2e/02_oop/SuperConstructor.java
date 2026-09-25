public class SuperConstructor {
    static class Animal {
        String name;
        int age;
        Animal(String name, int age) {
            this.name = name;
            this.age = age;
        }
        String describe() { return name + ":" + age; }
    }

    static class Dog extends Animal {
        String breed;
        Dog(String name, int age, String breed) {
            super(name, age);
            this.breed = breed;
        }
        String describe() { return super.describe() + "/" + breed; }
    }

    static class Puppy extends Dog {
        Puppy(String name, String breed) {
            super(name, 1, breed);
        }
    }

    public static void main(String[] args) {
        Animal a = new Animal("Cat", 3);
        System.out.println(a.describe());     // Cat:3

        Dog d = new Dog("Rex", 5, "Husky");
        System.out.println(d.describe());     // Rex:5/Husky

        Puppy p = new Puppy("Spot", "Dalmatian");
        System.out.println(p.name);           // Spot
        System.out.println(p.age);            // 1
        System.out.println(p.breed);          // Dalmatian
    }
}
