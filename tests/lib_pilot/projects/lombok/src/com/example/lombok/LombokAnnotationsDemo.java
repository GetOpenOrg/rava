package com.example.lombok;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;

import lombok.Builder;
import lombok.Delegate;
import lombok.RequiredArgsConstructor;
import lombok.SneakyThrows;
import lombok.ToString;
import lombok.extern.slf4j.Slf4j;

/**
 * Lombok 6 个冷门注解示例（对应文章全部六节）。
 * 本包由 javac + Lombok 注解处理器编译（ajc 不支持 Lombok，已在 pom 排除）。
 *
 * 输出确定性说明：全部固定文案；@Slf4j 因绑定 NOP 日志实现，
 * 通过反射展示生成的 log 字段而非打印日志内容。
 */
public class LombokAnnotationsDemo {

    // ---------- ① @RequiredArgsConstructor：final 字段自动生成构造器 ----------
    static class UserRepository {
        public String name() {
            return "UserRepository";
        }
    }

    static class OrderRepository {
        public String name() {
            return "OrderRepository";
        }
    }

    static class EmailService {
        public String name() {
            return "EmailService";
        }
    }

    @RequiredArgsConstructor
    static class UserService {
        private final UserRepository userRepo;
        private final OrderRepository orderRepo;
        private final EmailService emailService;

        String deps() {
            return userRepo.name() + " / " + orderRepo.name() + " / " + emailService.name();
        }
    }

    // ---------- ② @SneakyThrows：受检异常不用 try-catch / throws ----------
    static class FileUtil {
        @SneakyThrows
        static String readStrict(String path) {
            return new String(java.nio.file.Files.readAllBytes(
                    java.nio.file.Path.of(path)));
        }
    }

    // ---------- ③ @Slf4j：一行声明日志对象 ----------
    @Slf4j
    static class BusinessService {
        void doSomething() {
            log.info("NOP 绑定下这行不会输出（见下方反射验证）");
        }
    }

    // ---------- ④ @ToString(callSuper = true)：父类字段不丢 ----------
    @ToString
    static class BaseEntity {
        private Long id = 1L;

        public Long getId() {
            return id;
        }
    }

    @ToString(callSuper = true)
    static class UserEntity extends BaseEntity {
        private String name = "张三";
        private int age = 25;
    }

    @ToString
    static class UserEntityNoSuper extends BaseEntity {
        private String name = "张三";
        private int age = 25;
    }

    // ---------- ⑤ @Builder.Default：Builder 默认值不丢 ----------
    @Builder
    static class ConfigWithDefault {
        @Builder.Default
        private int timeout = 3000;
        private String env = "prod";

        int timeout() {
            return timeout;
        }
    }

    @Builder
    static class ConfigWithoutDefault {
        private int timeout = 3000; // 不加 @Builder.Default，builder 构建后是 0

        int timeout() {
            return timeout;
        }
    }

    // ---------- ⑥ @Delegate：一行实现接口转发 ----------
    static class MyList {
        @Delegate
        private final List<String> list = new ArrayList<>();
    }

    public static void main(String[] args) {
        // ① 构造器由 Lombok 生成：直接 new 三个依赖（Spring 里即构造器注入）
        UserService service = new UserService(
                new UserRepository(), new OrderRepository(), new EmailService());
        System.out.println("[① RequiredArgsConstructor] 依赖注入: " + service.deps());

        // ② 方法没有 throws IOException，编译仍通过；异常在运行时原样传播
        try {
            FileUtil.readStrict("/definitely/not/exist.txt");
        } catch (Throwable t) {
            System.out.println("[② SneakyThrows] 传播的异常类型: " + t.getClass().getSimpleName());
        }

        // ③ log 字段由 Lombok 生成（NOP 绑定下日志静默，这里用反射验证字段存在）
        BusinessService biz = new BusinessService();
        biz.doSomething();
        try {
            Field logField = BusinessService.class.getDeclaredField("log");
            System.out.println("[③ Slf4j] 生成的字段: " + logField.getType().getName());
        } catch (NoSuchFieldException e) {
            System.out.println("[③ Slf4j] 字段未生成: " + e.getMessage());
        }

        // ④ callSuper 对比：不加会丢掉父类 id 字段
        System.out.println("[④ ToString] 带 callSuper  : " + new UserEntity());
        System.out.println("[④ ToString] 不带 callSuper: " + new UserEntityNoSuper() + " ← 丢了id");

        // ⑤ Builder.Default 对比
        System.out.println("[⑤ Builder.Default] 加了默认值  : timeout="
                + ConfigWithDefault.builder().build().timeout());
        System.out.println("[⑤ Builder.Default] 没加默认值  : timeout="
                + ConfigWithoutDefault.builder().build().timeout() + " ← 变成0");

        // ⑥ @Delegate：MyList 没写一行转发，却拥有 List 的全部方法
        MyList myList = new MyList();
        myList.add("hello");
        myList.add("lombok");
        System.out.println("[⑥ Delegate] add/size 直接可用: size=" + myList.size()
                + ", contains(hello)=" + myList.contains("hello"));
        // 反射证明 add 方法是 Lombok 生成的（源码里没有）
        try {
            Method generated = MyList.class.getMethod("add", Object.class);
            System.out.println("[⑥ Delegate] 生成的转发方法: " + generated.getName()
                    + " -> " + generated.getReturnType().getSimpleName());
        } catch (NoSuchMethodException e) {
            System.out.println("[⑥ Delegate] 方法未生成: " + e.getMessage());
        }
    }
}
