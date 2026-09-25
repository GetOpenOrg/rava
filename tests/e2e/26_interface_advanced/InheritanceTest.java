/**
 * Month 3 测试：继承 — 子类、方法覆盖、super 调用、多态
 */
class Animal {
    int age;

    public Animal(int age) {
        this.age = age;
    }

    public int getAge() {
        return this.age;
    }

    public String speak() {
        return "...";
    }
}

class Dog extends Animal {
    String name;

    public Dog(String name, int age) {
        super(age);
        this.name = name;
    }

    public String speak() {
        return "Woof!";
    }

    public String getName() {
        return this.name;
    }
}

public class InheritanceTest {
    public static void main(String[] args) {
        Dog d = new Dog("Rex", 5);
        System.out.println(d.getName());
        System.out.println(d.getAge());
        System.out.println(d.speak());

        Animal a = new Animal(10);
        System.out.println(a.getAge());
        System.out.println(a.speak());
    }
}
