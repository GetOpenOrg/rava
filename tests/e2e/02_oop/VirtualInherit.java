// 测试：子类对象调用父类定义（未被覆盖）的虚方法
public class VirtualInherit {
    static class Animal {
        String sound() { return "generic"; }
        int age() { return 1; }
    }

    static class Dog extends Animal {
        @Override
        String sound() { return "woof"; }
        // age() is NOT overridden — must be dispatched to Animal.age()
    }

    static class Cat extends Animal {
        // sound() is NOT overridden — must be dispatched to Animal.sound()
        @Override
        int age() { return 5; }
    }

    public static void main(String[] args) {
        Animal a = new Animal();
        Animal d = new Dog();
        Animal c = new Cat();

        System.out.println(a.sound()); // generic
        System.out.println(d.sound()); // woof
        System.out.println(c.sound()); // generic
        System.out.println(a.age());   // 1
        System.out.println(d.age());   // 1  (Dog inherits Animal.age)
        System.out.println(c.age());   // 5
    }
}
