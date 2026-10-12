package com.example.redis;

import java.util.List;
import java.util.Map;
import java.util.UUID;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisConnectionException;
import redis.clients.jedis.params.SetParams;

/**
 * 知识点7：完整实战——用户会话缓存（Hash + 过期）+ 分布式锁（SET NX EX + Lua 原子释放）。
 * 对应文章中的 Python redis 示例，此处为 Java/Jedis 等价实现。
 */
public class RedisSessionLockDemo {

    /** 与文章一致的 Lua 脚本：值匹配才删除，保证“自己的锁自己释放”的原子性。 */
    private static final String UNLOCK_SCRIPT = """
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
            """;

    public static void main(String[] args) {
        try (Jedis jedis = RedisClient.borrow()) {

            // ---------- 1. 用户登录，缓存 session（Hash + 1小时过期） ----------
            // 使用固定的 sessionId 保证输出可预期（生产环境应为随机值）
            String sessionId = "session:user_10086";
            jedis.hset(sessionId, Map.of(
                    "uid", "10086",
                    "username", "redis大师",
                    // 演示用固定时间戳，保证输出可预期（生产环境应为当前时间）
                    "login_time", "1759548000"));
            jedis.expire(sessionId, 3600);
            System.out.println("[会话] 缓存写入: " + sessionId
                    + "，剩余TTL=" + jedis.ttl(sessionId) + "s");

            // ---------- 2. 分布式锁（防重复下单） ----------
            String lockKey = "lock:order:product_9527";
            String lockVal = UUID.randomUUID().toString(); // 唯一值标识锁持有者

            // SET key value NX EX 10：加锁与设置过期必须是一条原子命令
            String ok = jedis.set(lockKey, lockVal,
                    SetParams.setParams().nx().ex(10));

            if ("OK".equals(ok)) {
                try {
                    System.out.println("[分布式锁] 获取锁成功，执行下单...");
                    Thread.sleep(1000); // 模拟业务逻辑
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                } finally {
                    // Lua 脚本原子性释放锁，防止误删他人的锁
                    Object released = jedis.eval(UNLOCK_SCRIPT,
                            List.of(lockKey), List.of(lockVal));
                    System.out.println("[分布式锁] 释放结果: " + ("1".equals(released.toString()) ? "成功" : "锁不存在"));
                }
            } else {
                System.out.println("[分布式锁] 获取锁失败，请重试");
            }

            // ---------- 3. 查询 session（TreeMap 排序，保证字段打印顺序固定） ----------
            System.out.println("[会话] 读取: " + new java.util.TreeMap<>(jedis.hgetAll(sessionId)));

            jedis.del(sessionId);
        } catch (JedisConnectionException e) {
            System.out.println("本机 6379 未检测到 Redis，请先启动: "
                    + "docker run -d --name redis -p 6379:6379 redis:8");
        }
    }
}
