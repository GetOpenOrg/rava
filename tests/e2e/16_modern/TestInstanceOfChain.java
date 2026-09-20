import java.util.ArrayList;
import java.util.List;

interface Marker {
}

class Animal implements Marker {
    String kind() {
        return "animal";
    }
}

class Dog extends Animal {
    @Override
    String kind() {
        return "dog";
    }

    String bark() {
        return "woof";
    }
}

class Puppy extends Dog {
    @Override
    String kind() {
        return "puppy";
    }
}

class Cat extends Animal {
    @Override
    String kind() {
        return "cat";
    }

    String meow() {
        return "meow";
    }
}

public class TestInstanceOfChain {

    static String chain(Object o) {
        if (o instanceof Puppy p) {
            return "puppy:" + p.kind();
        } else if (o instanceof Dog d) {
            return "dog:" + d.bark();
        } else if (o instanceof Cat c) {
            return "cat:" + c.meow();
        } else if (o instanceof Animal a) {
            return "animal:" + a.kind();
        } else if (o instanceof Marker m) {
            return "marker-only";
        } else if (o instanceof String s) {
            return "string:" + s.length();
        } else if (o instanceof Integer i) {
            return "int:" + (i + 1);
        }
        return "unknown";
    }

    static String oldStyle(Object o) {
        if (o instanceof Animal) {
            Animal a = (Animal) o;
            return "old-animal:" + a.kind();
        }
        return "old-unknown";
    }

    static int countInstances(List<Object> items, Class<?> ignored) {
        int dogs = 0;
        int cats = 0;
        for (Object o : items) {
            if (o instanceof Dog) {
                dogs++;
            } else if (o instanceof Cat) {
                cats++;
            }
        }
        return dogs * 100 + cats;
    }

    public static void main(String[] args) {
        Dog dog = new Dog();
        Puppy puppy = new Puppy();
        Cat cat = new Cat();
        Animal animal = new Animal();

        System.out.println(chain(puppy));
        System.out.println(chain(dog));
        System.out.println(chain(cat));
        System.out.println(chain(animal));
        System.out.println(chain("hello"));
        System.out.println(chain(Integer.valueOf(41)));
        System.out.println(chain(new Object()));

        // null 的 instanceof 恒为 false
        Object nothing = null;
        System.out.println("null instanceof Animal=" + (nothing instanceof Animal));
        System.out.println("null chain=" + chain(nothing));

        // 旧式写法对照
        System.out.println(oldStyle(dog));
        System.out.println(oldStyle("str"));

        // 转型后再 instanceof
        Animal upcast = puppy;
        System.out.println("upcast is Dog=" + (upcast instanceof Dog));
        System.out.println("upcast is Puppy=" + (upcast instanceof Puppy));

        // 错误的向下转型
        try {
            Cat wrong = (Cat) (Animal) dog;
            System.out.println("bad cast ok " + wrong);
        } catch (ClassCastException e) {
            System.out.println("bad cast -> " + e.getClass().getSimpleName());
        }

        // 在集合中统计类型
        List<Object> items = new ArrayList<>();
        items.add(dog);
        items.add(cat);
        items.add(new Dog());
        items.add(puppy);
        System.out.println("dogs/cats=" + countInstances(items, Dog.class));

        // 模式变量的作用域：在 if 之后不可见，但在 lambda/表达式内可用
        if (dog instanceof Dog dd && dd.bark().length() > 0) {
            System.out.println("scope ok " + dd.bark());
        }

        System.out.println("done");
    }
}
