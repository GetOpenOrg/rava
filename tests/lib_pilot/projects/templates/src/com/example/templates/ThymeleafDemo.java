package com.example.templates;

import java.util.List;

import org.thymeleaf.TemplateEngine;
import org.thymeleaf.context.Context;
import org.thymeleaf.templateresolver.StringTemplateResolver;
import org.thymeleaf.templatemode.TemplateMode;

/**
 * Thymeleaf 模板引擎脱离 Web 容器的纯 Java 渲染（不依赖 Spring）。
 * 覆盖文章的表达式表格：${} 变量、*{} 选择（th:object）、th:if、th:each；
 * 再演示 TEXT 模式（六种模板模式之一）渲染纯文本。
 *
 * 注：@{} URL 表达式依赖 Web 上下文、#{} i18n 需要消息解析器，本示例不涉及。
 * 输出确定性说明：字符串模板 + 固定 model。
 */
public class ThymeleafDemo {

    static TemplateEngine engine(TemplateMode mode) {
        StringTemplateResolver resolver = new StringTemplateResolver();
        resolver.setTemplateMode(mode);
        TemplateEngine engine = new TemplateEngine();
        engine.setTemplateResolver(resolver);
        return engine;
    }

    public static void main(String[] args) {
        // ---------- 1. HTML 模式：th:object + *{} + th:each + th:if ----------
        String html = """
                <html><body>
                <h1 th:text="'用户：' + ${user.name}">占位</h1>
                <div th:object="${user}">
                  <p>等级：<span th:text="*{level}">-</span></p>
                  <p>积分：<span th:text="*{points}">-</span></p>
                </div>
                <ul>
                  <li th:each="item : ${items}" th:text="${item.name} + ' x' + ${item.qty}">占位</li>
                </ul>
                <p th:if="${user.points >= 1000}">金牌会员</p>
                <p th:unless="${user.points >= 1000}">普通会员</p>
                </body></html>
                """;
        Context ctx = new Context();
        ctx.setVariable("user", new User("张三", "白银", 620));
        ctx.setVariable("items", List.of(
                new Item("机械键盘", 1), new Item("显示器", 2)));
        String rendered = engine(TemplateMode.HTML).process(html, ctx);
        System.out.println("== HTML 模式 ==");
        System.out.println(rendered.lines()
                .map(String::trim)
                .filter(l -> !l.isBlank())
                .reduce((a, b) -> a + "\n" + b).orElse(""));

        // ---------- 2. TEXT 模式：内联语法渲染纯文本 ----------
        String text = """
                订单摘要：
                [# th:each="item : ${items}"]
                - [# th:text="${item.name}"/] x[# th:text="${item.qty}"/]
                [/]
                合计用户：[# th:text="${user.name}"/]（[# th:text="${user.level}"/]）
                """;
        String textOut = engine(TemplateMode.TEXT).process(text, ctx);
        System.out.println("\n== TEXT 模式 ==");
        System.out.println(textOut.stripTrailing());
    }

    record User(String name, String level, int points) {
    }

    record Item(String name, int qty) {
    }
}
