package com.example.frameworks;

import java.lang.reflect.Method;

import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import jakarta.ws.rs.Path;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 用反射读取三种框架注解声明的路由，验证“代码几乎一模一样”：
 * 三种写法最终都声明了同一个 GET /hello 端点。
 *
 * 输出确定性说明：反射遍历固定类列表，路由信息为编译期常量。
 */
public class RouteAnnotationDemo {

    public static void main(String[] args) {
        for (Class<?> clazz : HelloEndpoints.endpointClasses()) {
            System.out.println(describe(clazz));
        }
        System.out.println("结论: 三种框架写法殊途同归，都声明了 GET /hello");
    }

    private static String describe(Class<?> clazz) {
        Method hello = helloMethod(clazz);
        if (clazz.isAnnotationPresent(RestController.class)) {
            String path = hello.getAnnotation(GetMapping.class).value()[0];
            return "[Spring Boot] " + clazz.getSimpleName()
                    + "  @GetMapping(\"" + path + "\") -> GET " + path;
        }
        if (clazz.isAnnotationPresent(Path.class)) {
            String path = clazz.getAnnotation(Path.class).value();
            return "[Quarkus]     " + clazz.getSimpleName()
                    + "  @Path(\"" + path + "\") + @GET -> GET " + path;
        }
        if (clazz.isAnnotationPresent(Controller.class)) {
            String path = clazz.getAnnotation(Controller.class).value();
            return "[Micronaut]   " + clazz.getSimpleName()
                    + "  @Controller(\"" + path + "\") + @Get -> GET " + path;
        }
        return "[未知] " + clazz.getSimpleName();
    }

    private static Method helloMethod(Class<?> clazz) {
        for (Method m : clazz.getDeclaredMethods()) {
            if (m.getName().equals("hello")) {
                return m;
            }
        }
        throw new IllegalStateException(clazz + " 缺少 hello 方法");
    }
}
