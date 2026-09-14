public class TestInheritance {

    static class Animal {
        String name;

        Animal(String name) {
            this.name = name;
        }

        String speak() {
            return name + " makes a sound";
        }

        String getName() {
            return name;
        }
    }

    static class Dog extends Animal {
        Dog(String name) {
            super(name);
        }

        @Override
        String speak() {
            return name + " barks";
        }
    }

    static class Cat extends Animal {
        Cat(String name) {
            super(name);
        }

        @Override
        String speak() {
            return name + " meows";
        }
    }

    static class GuideDog extends Dog {
        GuideDog(String name) {
            super(name);
        }

        @Override
        String speak() {
            return super.speak() + " (guide)";
        }
    }

    public static void main(String[] args) {
        Animal a = new Animal("Animal");
        Dog d = new Dog("Rex");
        Cat c = new Cat("Whiskers");
        GuideDog g = new GuideDog("Buddy");

        // 直接调用
        System.out.println(a.speak());
        System.out.println(d.speak());
        System.out.println(c.speak());
        System.out.println(g.speak());

        // 继承的方法
        System.out.println(d.getName());

        // 多态：父类引用调用子类方法
        Animal[] animals = { a, d, c, g };
        for (Animal animal : animals) {
            System.out.println(animal.speak());
        }

        // instanceof
        System.out.println(d instanceof Animal);
        System.out.println(g instanceof Dog);
        System.out.println(a instanceof Dog);
    }
}
