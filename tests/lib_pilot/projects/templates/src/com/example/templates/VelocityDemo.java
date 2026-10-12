package com.example.templates;

import java.io.StringReader;
import java.io.StringWriter;
import java.util.List;

import org.apache.velocity.VelocityContext;
import org.apache.velocity.app.VelocityEngine;

/**
 * Apache Velocity 模板引擎（VTL）纯 Java 渲染。
 * 覆盖：$ 取值、$!{x} 静默引用（不存在不报错）、#set、#if/#else、#foreach、## 注释。
 * 场景呼应文章："动态生成 SQL、XML、配置文件、报告文本"——本示例生成一段 SQL。
 *
 * 输出确定性说明：字符串模板 + 固定 model；单线程。
 */
public class VelocityDemo {

    public static void main(String[] args) {
        String template = """
                ## 动态生成报表 SQL（VelocityDemo 生成）
                #set($table = "t_order")
                SELECT
                #foreach($col in $columns)#if($foreach.hasNext)$col,#else$col#end#end
                FROM $table
                WHERE 1 = 1
                #if($minAmount)  AND amount >= $minAmount#end
                #if($status)  AND status = '$status'#end
                ## $owner 未放入 context，静默引用不输出也不报错
                -- owner: $!{owner}
                """;

        VelocityEngine engine = new VelocityEngine();
        engine.init();

        VelocityContext context = new VelocityContext();
        context.put("columns", List.of("order_id", "user_id", "amount", "status"));
        context.put("minAmount", 100);
        context.put("status", "PAID");

        StringWriter out = new StringWriter();
        engine.evaluate(context, out, "sql-template", new StringReader(template));
        System.out.println(out.toString().stripTrailing());
    }
}
