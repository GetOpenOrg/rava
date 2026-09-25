package com.example;

public class PackageTest {
    public static void main(String[] args) {
        System.out.println("Package support works!");
        Helper helper = new Helper();
        System.out.println(helper.greet("JNC"));
    }
}

class Helper {
    public String greet(String name) {
        return "Hello, " + name + "!";
    }
}
