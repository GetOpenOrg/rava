package com.example.netty;

import java.nio.charset.StandardCharsets;

import io.netty.buffer.Unpooled;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.SimpleChannelInboundHandler;
import io.netty.channel.embedded.EmbeddedChannel;
import io.netty.handler.codec.LineBasedFrameDecoder;
import io.netty.handler.codec.string.StringDecoder;
import io.netty.handler.codec.string.StringEncoder;

/**
 * Netty（一）：EmbeddedChannel 离线流水线测试（对应文章框架主题，文章无代码）。
 * 不开端口、不起线程：writeInbound 模拟入站数据，readOutbound 读出站结果，
 * 与 Netty 真实网络完全同一套 ChannelPipeline——这正是 Netty 官方推荐的
 * handler 单测方式，也天然满足输出确定性。
 *
 * 流水线：行解码器 → 字符串解码 → 业务 handler（转大写回写）→ 字符串编码。
 */
public class NettyEmbeddedChannelDemo {

    public static void main(String[] args) {
        EmbeddedChannel channel = new EmbeddedChannel(
                new LineBasedFrameDecoder(64),
                new StringDecoder(StandardCharsets.UTF_8),
                new StringEncoder(StandardCharsets.UTF_8),
                new SimpleChannelInboundHandler<String>() {
                    @Override
                    protected void channelRead0(ChannelHandlerContext ctx, String msg) {
                        System.out.println("[handler] 收到一行: " + msg);
                        ctx.writeAndFlush(msg.toUpperCase()); // 回写转大写
                    }
                });

        // 模拟 TCP 分段：半行 + 半行拼成一行——LineBasedFrameDecoder 负责拼帧
        channel.writeInbound(Unpooled.copiedBuffer("hel", StandardCharsets.UTF_8));
        channel.writeInbound(Unpooled.copiedBuffer("lo\r\n", StandardCharsets.UTF_8));
        channel.writeInbound(Unpooled.copiedBuffer("netty embedded\r\n", StandardCharsets.UTF_8));

        System.out.println("[outbound] " + channel.readOutbound());
        System.out.println("[outbound] " + channel.readOutbound());
        System.out.println("[outbound] 再读 = " + channel.readOutbound() + "（无更多数据）");

        channel.finishAndReleaseAll();
        System.out.println("[结论] 零端口零线程验证了 Netty 编解码流水线，同一套 handler 可直接上真实网络");
    }
}
