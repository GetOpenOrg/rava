package com.example.pekko;

import java.util.List;

import org.apache.pekko.actor.typed.ActorSystem;
import org.apache.pekko.actor.typed.Behavior;
import org.apache.pekko.actor.typed.javadsl.Behaviors;

/**
 * Apache Pekko（一）：typed Actor 消息处理（对应文章 Actor 模型主题，文章无代码、
 * demo 按 Pekko 1.x Java API 编写）。
 * 单 Actor 邮箱 FIFO：主线程顺序 tell 5 条消息，Actor 严格按序处理，
 * 处理完最后一条自动停止，等系统终止后收尾输出。
 *
 * 输出确定性说明：单消费者邮箱 FIFO + 顺序 tell，输出顺序确定。
 * 本包从 ajc 排除（ajc 对 Scala 泛型重载有歧义，见 pom 注释）。
 */
public class PekkoActorDemo {

    sealed interface Command permits Print, Stop {
    }

    record Print(String message) implements Command {
    }

    record Stop() implements Command {
    }

    static Behavior<Command> printer() {
        // Pekko javadsl 的 onMessage handler 是单参数 Function（无 context）
        return Behaviors.receive(Command.class)
                .onMessage(Print.class, msg -> {
                    System.out.println("[actor] 收到: " + msg.message());
                    return Behaviors.same();
                })
                .onMessage(Stop.class, msg -> {
                    System.out.println("[actor] 收到停止指令，即将停止");
                    return Behaviors.stopped();
                })
                .build();
    }

    public static void main(String[] args) throws Exception {
        ActorSystem<Command> system = ActorSystem.create(printer(), "printer-system");

        List<String> messages = List.of("hello", "pekko", "actor", "model");
        for (String msg : messages) {
            system.tell(new Print(msg));
        }
        system.tell(new Stop());

        system.getWhenTerminated().toCompletableFuture().get(); // 等 actor 停止、系统终止

        System.out.println("[main] 共处理 " + messages.size() + " 条消息后系统正常终止");
    }
}
