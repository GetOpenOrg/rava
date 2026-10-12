package com.example.querydsl;

/**
 * querydsl-collections 顶层实体（顶层类：生成求值代码对嵌套类的类型引用有兼容问题）。
 */
public class QuerydslUser {

    private final String name;
    private final int age;
    private final String city;

    public QuerydslUser(String name, int age, String city) {
        this.name = name;
        this.age = age;
        this.city = city;
    }

    public String getName() {
        return name;
    }

    public int getAge() {
        return age;
    }

    public String getCity() {
        return city;
    }

    @Override
    public String toString() {
        return name + "(" + age + "," + city + ")";
    }
}
