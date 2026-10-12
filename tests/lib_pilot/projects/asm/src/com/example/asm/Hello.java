package com.example.asm;

/**
 * ASM 插桩的目标类。
 * 原始 sayHello() 只打印一行；经 TimingClassAdapter 增强后，
 * 等价于在方法首尾插入了 System.nanoTime() 计时与耗时打印（见 AsmTransformDemo）。
 */
public class Hello {

    public void sayHello() {
        System.out.println("Hello World");
    }

    public int calculate() {
        int result = 0;
        for (int i = 1; i <= 10; i++) {
            result += i;
        }
        System.out.println("Result: " + result);
        return result;
    }
}
