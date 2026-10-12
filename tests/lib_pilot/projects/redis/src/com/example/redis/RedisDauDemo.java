package com.example.redis;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisConnectionException;

/**
 * 知识点3 实操：用 Set 实现“日活跃用户（DAU）”统计。
 * sadd 记录活跃、scard 去重计数、sismember 成员判断。
 */
public class RedisDauDemo {

    public static void main(String[] args) {
        try (Jedis jedis = RedisClient.borrow()) {
            String dauKey = "dau:2026-10-04";

            // 用户登录：批量记录当日活跃 uid
            jedis.sadd(dauKey, "uid_101", "uid_205", "uid_309");

            // 重复登录不会重复计数（Set 天然去重）
            jedis.sadd(dauKey, "uid_101");

            System.out.println("今日活跃用户数: " + jedis.scard(dauKey));            // 3
            System.out.println("uid_101 是否活跃: " + jedis.sismember(dauKey, "uid_101")); // true
            System.out.println("uid_666 是否活跃: " + jedis.sismember(dauKey, "uid_666")); // false

            // 昨日/今日交集：连续活跃用户（留存分析）；sinter 返回无序 Set，排序后打印
            jedis.sadd("dau:2026-10-03", "uid_101", "uid_205", "uid_777");
            System.out.println("连续两日活跃: " + new java.util.TreeSet<>(jedis.sinter(dauKey, "dau:2026-10-03")));

            jedis.del(dauKey, "dau:2026-10-03");
        } catch (JedisConnectionException e) {
            System.out.println("本机 6379 未检测到 Redis，请先启动: "
                    + "docker run -d --name redis -p 6379:6379 redis:8");
        }
    }
}
