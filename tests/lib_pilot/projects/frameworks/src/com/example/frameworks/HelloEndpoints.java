package com.example.frameworks;

import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * 文章 §“三个框架现在的样子”：同一个 /hello 接口的三种写法。
 * 三段代码几乎一模一样，真正的差别在运行时姿态（启动/内存/构建期处理）。
 *
 * 说明：本类只做注解编译演示（依赖为 spring-web / jakarta.ws.rs-api / micronaut-http
 * 的 API 包），不启动真实服务器；路由信息由 RouteAnnotationDemo 反射读取。
 */
public final class HelloEndpoints {

    private HelloEndpoints() {
    }

    /** Spring Boot 写法。 */
    @RestController
    static class SpringHelloController {
        @GetMapping("/hello")
        String hello() {
            return "hello";
        }
    }

    /** Quarkus 写法（JAX-RS 注解）。 */
    @Path("/hello")
    static class QuarkusHelloResource {
        @GET
        public String hello() {
            return "hello";
        }
    }

    /** Micronaut 写法。 */
    @Controller("/hello")
    static class MicronautHelloController {
        @Get
        public String hello() {
            return "hello";
        }
    }

    /** 供反射演示的三个端点类。 */
    static Class<?>[] endpointClasses() {
        return new Class<?>[]{
                SpringHelloController.class,
                QuarkusHelloResource.class,
                MicronautHelloController.class};
    }
}
