import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketTimeoutException;
import java.nio.charset.StandardCharsets;

/**
 * 裸 Socket 回环对（K9 网络边界的最小可测面，此前误归纯结构性缺口）：阻塞读写、
 * shutdownOutput 半关闭、SO_TIMEOUT 超时边界、关闭后读写异常。端口 0 自分配不打印。
 */
public class TestSocketLoopbackPair {

    public static void main(String[] args) throws Exception {
        ServerSocket server = new ServerSocket(
                new InetSocketAddress(InetAddress.getLoopbackAddress(), 0).getPort(),
                1, InetAddress.getLoopbackAddress());
        int port = server.getLocalPort();

        // 服务线程：接受 → 读 → 回显 → 半关闭；输出收集到缓冲，join 后统一打印
        // （消除 worker/main 行序随线程竞速漂移——golden 双跑一致的先决条件）
        final StringBuilder workerOut = new StringBuilder();
        Thread worker = new Thread(() -> {
            try (Socket s = server.accept()) {
                InputStream in = s.getInputStream();
                OutputStream out = s.getOutputStream();
                byte[] buf = new byte[64];
                int n = in.read(buf);
                out.write(buf, 0, n);
                out.write('!');
                out.flush();
                s.shutdownOutput();               // 半关闭：写端结束，读端见 EOF
                int tail = in.read();
                workerOut.append("server-sees-eof=").append(tail == -1).append('\n');
            } catch (IOException e) {
                workerOut.append("server-ex=").append(e.getClass().getSimpleName()).append('\n');
            }
        });
        worker.start();

        try (Socket c = new Socket(InetAddress.getLoopbackAddress(), port)) {
            OutputStream out = c.getOutputStream();
            out.write("ping".getBytes(StandardCharsets.UTF_8));
            out.flush();

            InputStream in = c.getInputStream();
            byte[] echo = new byte[64];
            int n = in.read(echo);
            System.out.println("echo=" + new String(echo, 0, n, StandardCharsets.UTF_8));
            System.out.println("after-half-close=" + in.read());   // -1：对端 shutdownOutput
            c.shutdownOutput();
        }
        worker.join();
        System.out.print(workerOut);
        server.close();

        // 关闭后的 accept → SocketException
        try {
            server.accept();
        } catch (IOException e) {
            System.out.println("closed-accept-ex=" + e.getClass().getSimpleName());
        }

        // SO_TIMEOUT：读超时 → SocketTimeoutException（不中断连接，可继续用）
        ServerSocket server2 = new ServerSocket(
                new InetSocketAddress(InetAddress.getLoopbackAddress(), 0).getPort(),
                1, InetAddress.getLoopbackAddress());
        try (Socket idle = new Socket(InetAddress.getLoopbackAddress(),
                server2.getLocalPort())) {
            idle.setSoTimeout(120);
            System.out.println("timeout-set=" + idle.getSoTimeout());
            try {
                idle.getInputStream().read();
            } catch (SocketTimeoutException e) {
                System.out.println("read-timeout-ex=" + e.getClass().getSimpleName());
            }
            // 超时后连接仍可用：写一个字节不报错
            idle.getOutputStream().write(1);
            idle.getOutputStream().flush();
            System.out.println("alive-after-timeout=true");
        }
        server2.close();

        System.out.println("tcp-nodelay=" + c2n());
    }

    static boolean c2n() throws Exception {
        try (Socket s = new Socket()) {
            s.setTcpNoDelay(true);
            return s.getTcpNoDelay();
        }
    }
}
