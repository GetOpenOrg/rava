import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.BindException;
import java.net.ConnectException;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketException;

/**
 * 套接字 native 边界的错误与查询路径（TestSocketLoopbackPair 的补充）：connect 被拒 →
 * ConnectException、端口占用 → BindException（errno → 异常类映射）；对端 / 本端地址
 * （双栈套接字上 v4 映射地址还原为 127.0.0.1）；available（FIONREAD）；shutdownInput；
 * SO_LINGER / SO_KEEPALIVE 往返；关闭后读写异常。端口 0 自分配不打印。
 */
public class TestSocketErrorPaths {

    public static void main(String[] args) throws Exception {
        InetAddress lo = InetAddress.getLoopbackAddress();

        // connect 被拒：先占一个端口再关闭，连接该端口
        int deadPort;
        try (ServerSocket tmp = new ServerSocket(0, 1, lo)) {
            deadPort = tmp.getLocalPort();
        }
        try (Socket s = new Socket(lo, deadPort)) {
            System.out.println("connect-refused=none");
        } catch (ConnectException e) {
            System.out.println("connect-refused=" + e.getClass().getSimpleName() + ": " + e.getMessage());
        }

        try (ServerSocket server = new ServerSocket(0, 4, lo)) {
            int port = server.getLocalPort();

            // 端口占用：同地址同端口再绑定
            try (ServerSocket dup = new ServerSocket(port, 1, lo)) {
                System.out.println("bind-in-use=none");
            } catch (BindException e) {
                System.out.println("bind-in-use=" + e.getClass().getSimpleName() + ": " + e.getMessage());
            }

            try (Socket c = new Socket(lo, port); Socket a = server.accept()) {
                System.out.println("remote-addr=" + c.getInetAddress().getHostAddress());
                System.out.println("local-addr=" + c.getLocalAddress().getHostAddress());
                System.out.println("remote-port-match=" + (c.getPort() == port));
                System.out.println("accepted-peer=" + a.getInetAddress().getHostAddress());
                System.out.println("accepted-peer-port-match=" + (a.getPort() == c.getLocalPort()));
                InetSocketAddress ra = (InetSocketAddress) a.getRemoteSocketAddress();
                System.out.println("accepted-remote-sa=" + ra.getAddress().getHostAddress());

                // available：一次写出 3 字节，读走 1 字节后剩 2
                OutputStream out = c.getOutputStream();
                out.write(new byte[] {'a', 'b', 'c'});
                out.flush();
                InputStream in = a.getInputStream();
                System.out.println("first=" + (char) in.read());
                System.out.println("available=" + in.available());
                byte[] rest = in.readNBytes(2);
                System.out.println("rest=" + new String(rest, "US-ASCII"));

                // shutdownInput：之后读立即见 EOF
                a.shutdownInput();
                System.out.println("after-shutdown-input=" + in.read());
                System.out.println("input-shutdown=" + a.isInputShutdown());

                // 选项往返
                c.setSoLinger(true, 5);
                System.out.println("linger-on=" + c.getSoLinger());
                c.setSoLinger(false, 0);
                System.out.println("linger-off=" + c.getSoLinger());
                c.setKeepAlive(true);
                System.out.println("keepalive=" + c.getKeepAlive());
                c.setKeepAlive(false);
                System.out.println("keepalive-off=" + c.getKeepAlive());
                System.out.println("connected=" + c.isConnected() + " closed=" + c.isClosed());
            }
        }

        // 关闭后的读 / 写
        try (ServerSocket server = new ServerSocket(0, 1, lo)) {
            Socket c = new Socket(lo, server.getLocalPort());
            Socket a = server.accept();
            InputStream in = c.getInputStream();
            OutputStream out = c.getOutputStream();
            c.close();
            System.out.println("closed=" + c.isClosed());
            try {
                in.read();
            } catch (SocketException e) {
                System.out.println("read-after-close=" + e.getClass().getSimpleName() + ": " + e.getMessage());
            }
            try {
                out.write(1);
            } catch (SocketException e) {
                System.out.println("write-after-close=" + e.getClass().getSimpleName() + ": " + e.getMessage());
            }
            // 对端关闭后本端读见 EOF
            System.out.println("peer-closed-read=" + a.getInputStream().read());
            a.close();
        } catch (IOException e) {
            System.out.println("unexpected=" + e);
        }
    }
}
