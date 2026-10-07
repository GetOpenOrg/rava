package s0app;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

/**
 * S0 最小 Spring Boot 应用（docs/plans/2026-10-07-framework-driven-api-coverage.md 阶段 S0）。
 * 覆盖：SpringApplication.run 启动、@Component / @Bean 注入、@Value 读 application.yml、
 * CommandLineRunner 输出、默认 logback 日志。
 */
@SpringBootApplication
public class S0Application {
    public static void main(String[] args) {
        SpringApplication.run(S0Application.class, args);
    }
}
