package com.example.scalalib;

import java.time.Duration;
import java.util.List;
import java.util.Map;

import org.apache.kafka.clients.consumer.ConsumerRecord;
import org.apache.kafka.clients.consumer.MockConsumer;
import org.apache.kafka.clients.consumer.OffsetResetStrategy;
import org.apache.kafka.clients.producer.MockProducer;
import org.apache.kafka.clients.producer.ProducerRecord;
import org.apache.kafka.clients.producer.RecordMetadata;
import org.apache.kafka.common.TopicPartition;
import org.apache.kafka.common.serialization.StringSerializer;

/**
 * Java 调用"Scala 出身"的库：Apache Kafka。
 * broker 与初代客户端都是 Scala 写的，如今官方 Java 客户端（kafka-clients）
 * 是第一等公民——本示例全部是 Java 代码。
 *
 * 用 kafka-clients 内置的 MockProducer/MockConsumer 做内存级回环，
 * 无需 broker，offset/partition 确定性递增。
 */
public class KafkaClientsDemo {

    private static final String TOPIC = "demo-topic";
    private static final TopicPartition PARTITION = new TopicPartition(TOPIC, 0);

    public static void main(String[] args) throws Exception {
        try (MockConsumer<String, String> consumer = new MockConsumer<>(OffsetResetStrategy.EARLIEST);
             // kafka-clients 4.x 的三参构造器已移除，四参版 partitioner 传 null（分区默认 0）
             MockProducer<String, String> producer = new MockProducer<>(
                     true, null, new StringSerializer(), new StringSerializer())) {

            // ---------- 1. MockProducer：发送 3 条消息（自动确认模式） ----------
            String[] messages = {"k1=v1", "k2=v2", "k3=v3"};
            for (String msg : messages) {
                String[] kv = msg.split("=");
                RecordMetadata meta = producer
                        .send(new ProducerRecord<>(TOPIC, kv[0], kv[1])).get();
                System.out.println("[发送] " + kv[0] + " -> partition="
                        + meta.partition() + " offset=" + meta.offset());
            }
            List<ProducerRecord<String, String>> history = producer.history();
            System.out.println("[发送] history 记录数 = " + history.size());

            // ---------- 2. MockConsumer：回放同样的消息（offset 0,1,2） ----------
            consumer.assign(List.of(PARTITION));
            consumer.updateBeginningOffsets(Map.of(PARTITION, 0L));
            for (int i = 0; i < history.size(); i++) {
                ProducerRecord<String, String> record = history.get(i);
                consumer.addRecord(new ConsumerRecord<>(
                        TOPIC, 0, (long) i, record.key(), record.value()));
            }
            var records = consumer.poll(Duration.ofSeconds(1));
            System.out.println("[消费] 拉到 " + records.count() + " 条：");
            for (ConsumerRecord<String, String> r : records) {
                System.out.println("[消费] offset=" + r.offset()
                        + " key=" + r.key() + " value=" + r.value());
            }
        }
        System.out.println("[结论] Scala 出身的 Kafka，Java 客户端 API 完全一等公民");
    }
}
