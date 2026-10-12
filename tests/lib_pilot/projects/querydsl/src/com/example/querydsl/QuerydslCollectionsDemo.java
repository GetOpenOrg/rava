package com.example.querydsl;

import java.util.List;

import com.querydsl.collections.CollQueryFactory;
import com.querydsl.core.BooleanBuilder;
import com.querydsl.core.types.Predicate;
import com.querydsl.core.types.dsl.PathBuilder;

/**
 * Querydsl 内存集合查询（对应文章主题：类型安全查询。
 * 文章唯一代码与 Spring Data JPA 深度绑定，此处用 querydsl-collections 模块
 * 以 PathBuilder 方式演示同一套链式 API + BooleanBuilder 动态条件，
 * 无需 APT 生成 Q 类）。
 *
 * 输出确定性说明：固定数据 + 显式 orderBy；predicate.toString() 是稳定的表达式文本。
 */
public class QuerydslCollectionsDemo {

    static final java.util.List<QuerydslUser> USERS = java.util.List.of(
            new QuerydslUser("张三", 32, "北京"),
            new QuerydslUser("李四", 25, "上海"),
            new QuerydslUser("王五", 41, "北京"),
            new QuerydslUser("赵六", 28, "深圳"),
            new QuerydslUser("钱七", 36, "北京"));

    public static void main(String[] args) {
        PathBuilder<QuerydslUser> user = new PathBuilder<>(QuerydslUser.class, "user");

        // ---------- 1. 静态条件 + 排序 + 投影（对应文章 where/orderBy/select 链） ----------
        List<String> names = CollQueryFactory.from(user, USERS)
                .where(user.getNumber("age", Integer.class).gt(30))
                .orderBy(user.getString("name").asc())
                .select(user.getString("name"))
                .fetch();
        System.out.println("[静态查询] age > 30 且按 name 升序 = " + names);

        // ---------- 2. BooleanBuilder 动态条件（模拟可选筛选参数） ----------
        Integer minAge = 26;      // 传入
        Integer maxAge = 40;      // 传入
        String city = "北京";     // 传入
        BooleanBuilder builder = new BooleanBuilder();
        if (minAge != null) {
            builder.and(user.getNumber("age", Integer.class).goe(minAge)); // goe = >=
        }
        if (maxAge != null) {
            builder.and(user.getNumber("age", Integer.class).loe(maxAge));
        }
        if (city != null) {
            builder.and(user.getString("city").eq(city));
        }
        List<QuerydslUser> result = CollQueryFactory.from(user, USERS)
                .where(builder)
                .orderBy(user.getNumber("age", Integer.class).asc())
                .fetch();
        System.out.println("[动态条件] predicate = " + builder);
        System.out.println("[动态条件] 命中 " + result.size() + " 条 = " + result);

        // ---------- 3. 任意一个条件为 null 时自动跳过（对应"可选筛选"） ----------
        Predicate onlyCity = new BooleanBuilder()
                .and(user.getString("city").eq("深圳"));
        List<QuerydslUser> szUsers = CollQueryFactory.from(user, USERS).where(onlyCity).fetch();
        System.out.println("[单一条件] city=深圳 = " + szUsers);
        System.out.println("[结论] 字段名以类型安全路径表达，拼错编译期/运行期即刻暴露");
    }
}
