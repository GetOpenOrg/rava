package com.example.cglib;

/** 与 User 字段同构的视图对象：供 BeanCopier 演示跨类型拷贝。 */
public class UserVO {

    private String name;
    private int age;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }

    @Override
    public String toString() {
        return "UserVO{name='" + name + "', age=" + age + "}";
    }
}
