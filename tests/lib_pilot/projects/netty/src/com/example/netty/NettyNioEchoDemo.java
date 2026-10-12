package com.example.netty;

import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;
import java.util.List;

import io.netty.bootstrap.ServerBootstrap;
import io.netty.buffer.Unpooled;
import io.netty.channel.Channel;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.EventLoopGroup;
import io.netty.channel.SimpleChannelInboundHandler;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.SocketChannel;
import io.netty.channel.socket.nio.NioServerSocketChannel;
import io.netty.handler.codec.LineBasedFrameDecoder;
import io.netty.handler.codec.string.StringDecoder;
import io.netty.handler.codec.string.StringEncoder;

/**
 * Netty（二）：回环 echo（gateway-netty-iouring.md 的降级落地）。
 * 排查文的主角是 Linux 独有的 io_uring 传输在连接池复用下的随机超时（不可在本机
 * 复现）；健康对照组正是本例：常规 NioEventLoopGroup + 行解码，回环顺序收发。
 *
 * 输出确定性说明：单连接顺序请求/响应逐条收集，不打印端口/线程/耗时。
 */
public class NettyNioEchoDemo {

    public static void main(String[] args) throws Exception {
        EventLoopGroup boss = new NioEventLoopGroup(1);
        EventLoopGroup worker = new NioEventLoopGroup(1);
        try {
            Channel serverChannel = new ServerBootstrap()
                    .group(boss, worker)
                    .channel(NioServerSocketChannel.class)
                    .childHandler(new ChannelInitializer<SocketChannel>() {
                        @Override
                        protected void initChannel(SocketChannel ch) {
                            ch.pipeline().addLast(
                                    new LineBasedFrameDecoder(64),
                                    new StringDecoder(StandardCharsets.UTF_8),
                                    new StringEncoder(StandardCharsets.UTF_8),
                                    new SimpleChannelInboundHandler<String>() {
                                        @Override
                                        protected void channelRead0(ChannelHandlerContext ctx, String msg) {
                                            // 回显必须带行结束符，客户端的行解码器才能拆帧
                                            ctx.writeAndFlush("echo:" + msg + "\r\n");
                                        }
                                    });
                        }
                    })
                    .bind(0).sync().channel();
            int port = ((InetSocketAddress) serverChannel.localAddress()).getPort();

            // JDK HttpClient 走 HTTP 太重，这里直接用 Netty 客户端保持同栈
            var clientHandler = new CollectingHandler();
            io.netty.bootstrap.Bootstrap cb = new io.netty.bootstrap.Bootstrap()
                    .group(worker)
                    .channel(io.netty.channel.socket.nio.NioSocketChannel.class)
                    .handler(new ChannelInitializer<SocketChannel>() {
                        @Override
                        protected void initChannel(SocketChannel ch) {
                            ch.pipeline().addLast(
                                    new LineBasedFrameDecoder(64),
                                    new StringDecoder(StandardCharsets.UTF_8),
                                    new StringEncoder(StandardCharsets.UTF_8),
                                    clientHandler);
                        }
                    });
            Channel client = cb.connect("localhost", port).sync().channel();

            List<String> toSend = List.of("msg-1", "msg-2", "msg-3", "msg-4", "msg-5");
            for (String msg : toSend) {
                client.writeAndFlush(msg + "\r\n");
            }
            clientHandler.awaitCount(toSend.size()); // 逐条收集到齐

            System.out.println("[echo] 收到 " + clientHandler.received().size() + " 条: "
                    + clientHandler.received());
            boolean inOrder = true;
            for (int i = 0; i < toSend.size(); i++) {
                if (!("echo:" + toSend.get(i)).equals(clientHandler.received().get(i))) {
                    inOrder = false;
                    break;
                }
            }
            System.out.println("[echo] 严格有序回显 = " + inOrder);
            System.out.println("[结论] NIO 传输路径健康；排查文的 io_uring 随机超时仅出现在"
                    + " Linux 特定内核 + 连接池复用组合，见 docs/gateway-netty-iouring.md");

            client.close().sync();
            serverChannel.close().sync();
        } finally {
            boss.shutdownGracefully().sync();
            worker.shutdownGracefully().sync();
        }
    }

    /** 收集回显消息的客户端 handler（收到即挂起唤醒主线程）。 */
    static class CollectingHandler extends SimpleChannelInboundHandler<String> {
        private final java.util.List<String> received =
                java.util.Collections.synchronizedList(new java.util.ArrayList<>());

        @Override
        protected void channelRead0(ChannelHandlerContext ctx, String msg) {
            received.add(msg);
            synchronized (this) {
                notifyAll();
            }
        }

        List<String> received() {
            return received;
        }

        void awaitCount(int n) throws InterruptedException {
            synchronized (this) {
                while (received.size() < n) {
                    wait(1000);
                }
            }
        }
    }
}
