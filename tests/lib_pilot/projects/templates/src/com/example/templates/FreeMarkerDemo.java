package com.example.templates;

import java.io.StringReader;
import java.io.StringWriter;
import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;

import freemarker.template.Configuration;
import freemarker.template.Template;
import freemarker.template.TemplateExceptionHandler;

/**
 * FreeMarker 模板引擎（FTL）纯 Java 渲染（对应文章基础语法块）。
 * 覆盖：${} 插值、${missing!} 空值兜底、<#if>/<#else>、<#list>、<#--> 注释。
 *
 * 文章代码勘误：文章把 FTL 注释写成了 JSP 风格 <%-- --%>，正确语法是 <#-- -->。
 *
 * 输出确定性说明：显式固定 Locale 与数字格式（默认数字格式随 JVM 环境漂移，
 * 如千分位逗号），模板与 model 固定。
 */
public class FreeMarkerDemo {

    public static void main(String[] args) throws Exception {
        // 显式指定兼容版本（用 getVersion() 会被 FreeMarker 记 SEVERE 日志）
        Configuration cfg = new Configuration(Configuration.VERSION_2_3_34);
        cfg.setLocale(Locale.CHINA);          // 固定 Locale，防跨环境差异
        cfg.setNumberFormat("#");             // 禁用千分位等本地化数字格式
        cfg.setDefaultEncoding("UTF-8");
        cfg.setTemplateExceptionHandler(TemplateExceptionHandler.RETHROW_HANDLER);
        freemarker.cache.StringTemplateLoader loader =
                new freemarker.cache.StringTemplateLoader();

        String template = """
                <#-- 用户列表邮件模板（FreeMarkerDemo 生成） -->
                尊敬的 ${name}：
                会员等级：${level}
                积分：${points}
                <#if points gte 1000>
                您已达到金牌会员标准。
                <#else>
                再接再厉，距离金牌还差 ${1000 - points} 分。
                </#if>
                本次采购明细：
                <#list items as item>
                  ${item?index + 1}. ${item.name} x${item.qty}
                </#list>
                备注：${remark!"（无）"}
                """;
        loader.putTemplate("email", template);
        cfg.setTemplateLoader(loader);

        Map<String, Object> model = new HashMap<>();
        model.put("name", "张三");
        model.put("level", "白银");
        model.put("points", 620);
        model.put("items", List.of(
                Map.of("name", "机械键盘", "qty", 1),
                Map.of("name", "显示器", "qty", 2)));
        // remark 不放，演示 ${missing!} 空值兜底

        Template t = cfg.getTemplate("email");
        StringWriter out = new StringWriter();
        t.process(model, out);
        System.out.println(out.toString().stripTrailing());
    }
}
