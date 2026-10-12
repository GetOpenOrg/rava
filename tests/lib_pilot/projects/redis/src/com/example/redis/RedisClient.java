package com.example.redis;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.JedisPool;

/**
 * Jedis 连接工具：示例统一从这里拿连接。
 * 需要本机 6379 端口有 Redis（docker run -d --name redis -p 6379:6379 redis:8）。
 */
final class RedisClient {

    private static final JedisPool POOL = new JedisPool("localhost", 6379);

    private RedisClient() {
    }

    /** 拿到一个连接；Redis 不可用时抛出异常，由调用方给出友好提示。 */
    static Jedis borrow() {
        Jedis jedis = POOL.getResource();
        jedis.ping(); // 连通性检查，失败抛 JedisConnectionException
        return jedis;
    }

    static void close(Jedis jedis) {
        if (jedis != null) {
            jedis.close(); // 连接池模式下是归还连接
        }
    }
}
