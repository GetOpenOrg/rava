package com.example.redis;

import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisConnectionException;

/**
 * 知识点3：五种必会数据结构——String / Hash / List / Set / Sorted Set。
 * 每种结构对应一类业务模型：缓存计数、对象存储、消息队列、去重、排行榜。
 */
public class RedisDataStructuresDemo {

    public static void main(String[] args) {
        try (Jedis jedis = RedisClient.borrow()) {

            // ---------- 1. String：缓存、计数器 ----------
            jedis.set("demo:name", "redis大师");
            jedis.incr("demo:pv"); // 原子自增
            jedis.incr("demo:pv");
            System.out.println("[String] get=" + jedis.get("demo:name")
                    + ", pv=" + jedis.get("demo:pv"));

            // ---------- 2. Hash：存储对象（用户信息） ----------
            jedis.hset("demo:user:1001", Map.of(
                    "uid", "1001",
                    "username", "redis大师",
                    "age", "28"));
            // hgetAll 返回 HashMap（无序），用 TreeMap 排序后再打印
            System.out.println("[Hash] hgetall=" + new TreeMap<>(jedis.hgetAll("demo:user:1001")));

            // ---------- 3. List：最新列表 / 消息队列 ----------
            jedis.del("demo:news");
            jedis.lpush("demo:news", "新闻3", "新闻2", "新闻1");
            jedis.rpush("demo:news", "公告1");
            List<String> news = jedis.lrange("demo:news", 0, -1);
            System.out.println("[List] lrange=" + news);

            // ---------- 4. Set：标签、去重 ----------
            jedis.sadd("demo:tags:java", "jvm", "concurrency", "bytecode");
            jedis.sadd("demo:tags:kotlin", "coroutine");
            // smembers 返回 HashSet（无序），排序后打印保证输出固定
            System.out.println("[Set] smembers=" + new TreeSet<>(jedis.smembers("demo:tags:java"))
                    + ", scard=" + jedis.scard("demo:tags:java")
                    + ", isMember(jvm)=" + jedis.sismember("demo:tags:java", "jvm"));

            // ---------- 5. Sorted Set：排行榜 ----------
            jedis.zadd("demo:rank:score", 95, "alice");
            jedis.zadd("demo:rank:score", 88, "bob");
            jedis.zadd("demo:rank:score", 91, "carol");
            jedis.zincrby("demo:rank:score", 10, "bob"); // bob 加分反超
            List<String> top = jedis.zrevrange("demo:rank:score", 0, 2);
            System.out.println("[ZSet] 排行榜Top3=" + top);

            // 清理演示数据
            Set<String> keys = jedis.keys("demo:*");
            if (!keys.isEmpty()) {
                jedis.del(keys.toArray(new String[0]));
            }
        } catch (JedisConnectionException e) {
            System.out.println("本机 6379 未检测到 Redis，请先启动: "
                    + "docker run -d --name redis -p 6379:6379 redis:8");
        }
    }
}
