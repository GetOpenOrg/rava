package com.example.groovylib;

import java.util.HashMap;
import java.util.Map;

import groovy.text.SimpleTemplateEngine;
import groovy.text.Template;

/**
 * Java 调用 Groovy 库：模板引擎（文章场景："Groovy 模板引擎常被 Java Web
 * 应用拿来当页面/邮件模板渲染"）。
 *
 * 顺带呼应文章：Gradle 本身就是 Groovy 写的——所有 Java 项目每天都在调用它，
 * 但构建工具无法作为库内嵌演示，此处用同语言的模板引擎代表 Groovy 生态。
 *
 * 输出确定性说明：模板与绑定数据固定。
 */
public class GroovyTemplateDemo {

    public static void main(String[] args) throws Exception {
        // ---------- 1. 简单变量替换 ----------
        Template simple = new SimpleTemplateEngine()
                .createTemplate("你好，${name}！您的订单 ${orderId} 已发货。");
        Map<String, Object> binding = new HashMap<>();
        binding.put("name", "张三");
        binding.put("orderId", "SO-20261004-001");
        System.out.println("[变量替换] " + simple.make(binding).toString().trim());

        // ---------- 2. 带循环与条件的模板（邮件列表渲染） ----------
        Template email = new SimpleTemplateEngine().createTemplate("""
                订单号：${orderId}
                商品清单：
                <% items.eachWithIndex { item, i -> out << String.format('  %d. %s x%d%n', i + 1, item.name, item.qty) } %>
                <% if (vip) { out << '【VIP 用户已免运费】' } else { out << '运费：10 元' } %>""");
        Map<String, Object> data = new HashMap<>();
        data.put("orderId", "SO-20261004-002");
        data.put("items", java.util.List.of(
                Map.of("name", "《深入理解Java虚拟机》", "qty", 1),
                Map.of("name", "机械键盘", "qty", 2)));
        data.put("vip", true);
        System.out.println("[邮件模板]");
        System.out.println(email.make(data).toString().trim());

        System.out.println("[结论] Groovy 模板引擎从 Java 直接调用，零桥接成本");
    }
}
