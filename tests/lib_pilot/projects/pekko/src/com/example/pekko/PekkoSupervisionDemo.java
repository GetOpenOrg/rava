package com.example.pekko;

import org.apache.pekko.actor.typed.ActorRef;
import org.apache.pekko.actor.typed.ActorSystem;
import org.apache.pekko.actor.typed.Behavior;
import org.apache.pekko.actor.typed.SupervisorStrategy;
import org.apache.pekko.actor.typed.javadsl.Behaviors;

/**
 * Apache Pekko（二）：监督策略（对应文章"容错/故障隔离"主题）。
 * 子 Actor 遇到毒消息抛异常，父级 supervise(...).onFailure(restart) 自动重启，
 * 重启后 Actor 恢复初始状态继续处理后续消息，父级（守卫）全程存活。
 *
 * 输出确定性说明：固定消息序列、邮箱 FIFO；停止由显式 Stop 消息驱动，无时间等待。
 * 本包从 ajc 排除（ajc 对 Scala 泛型重载有歧义，见 pom 注释）。
 */
public class PekkoSupervisionDemo {

    sealed interface Command {
    }

    record Work(int n) implements Command {
    }

    record Boom() implements Command {
    }

    record Stop() implements Command {
    }

    static Behavior<Command> worker() {
        return Behaviors.receive(Command.class)
                .onMessage(Work.class, msg -> {
                    System.out.println("[worker] 处理 Work(" + msg.n() + ") 成功");
                    return Behaviors.same();
                })
                .onMessage(Boom.class, msg -> {
                    System.out.println("[worker] 收到 Boom，抛出异常模拟故障");
                    throw new IllegalStateException("毒消息");
                })
                .build();
    }

    public static void main(String[] args) throws Exception {
        // 守卫：spawn 被监督的子 actor 并转发业务消息；Stop 由守卫自己处理
        //（守卫停止 = 系统终止，子 actor 随之停止，否则 JVM 挂起不退出）
        ActorSystem<Command> system = ActorSystem.create(
                Behaviors.setup(ctx -> {
                    ActorRef<Command> child = ctx.spawn(
                            Behaviors.supervise(worker())
                                    .onFailure(SupervisorStrategy.restart()),
                            "worker");
                    return Behaviors.receiveMessage(msg -> {
                        if (msg instanceof Stop) {
                            return Behaviors.stopped(); // 守卫停止 → 系统终止
                        }
                        child.tell(msg);
                        return Behaviors.same();
                    });
                }),
                "supervision-system");

        system.tell(new Work(1));  // 正常
        system.tell(new Boom());   // 触发异常 → 框架自动重启 worker
        system.tell(new Work(2));  // 重启后继续处理（邮箱 FIFO 保证顺序）
        system.tell(new Stop());   // 守卫停止 → 系统终止

        system.getWhenTerminated().toCompletableFuture().get();
        System.out.println("[main] Boom 只重启了 worker，守卫存活，Work(2) 在重启后完成 = 故障隔离");
    }
}
