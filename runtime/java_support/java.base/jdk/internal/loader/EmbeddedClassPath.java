/*
 * VM 支持类（rava）：构建期嵌入的类路径资源（用户决策 (c)：原生二进制不设运行期应用类路径）。
 *
 * JVM 上应用类加载器经 URLClassPath 在运行期按类路径读取资源（目录 / jar 内文件）。原生二进制的类
 * 全集在构建期静态链接，类路径（用户编译输出与库归档）的全部文件在构建期按类路径顺序嵌入二进制；
 * 同名资源按类路径序保留多份（对应 getResources 的枚举序）。本类只承载类路径：模块资源在本程序 jimage 里，
 * 经翻译的 BuiltinClassLoader / SystemModuleReader 读取（docs/plans/2026-10-05-boot-image-evaluator.md §5.7）。
 * 本类以 Java 给出资源面（URL），读表的两个 native 由
 * runtime/java_runtime/src/jdk/internal/loader/embedded_class_path_impl.rs 承载。
 * 入口：BuiltinClassLoader.findResourceOnClassPath / findResourcesOnClassPath（vm_intrinsics.toml
 * kind = class_path）。
 * 方案：docs/plans/2026-10-01-c1d-closure-bloat.md §30.15
 *
 * URL 形如 ravacp:/<encodePath(资源名)>，同名第 i（≥ 1）份带 #i，主机为空串（与 JVM 上类路径 file URL 同形，
 * 字符串重建后 equals 成立）。协议处理器是 VM 支持类 sun.net.www.protocol.ravacp.Handler：本类构造 URL 时显式
 * 给出它；由字符串重建的 URL（new URL(url.toString())、URI.create(s).toURL()）经 URL.getURLStreamHandler 的
 * 内建工厂按协议名 "sun.net.www.protocol." + 协议 + ".Handler" 反射构造同一个类，连接同样读嵌入表
 *（计划 §30.16，取舍 T2 (b)）。
 *
 * 编译：转译时以当前 JDK 的 javac --patch-module java.base 编入 jdk.internal.loader 包
 * （generator/crates/resolve/src/image.rs VM 支持类目录）。
 */
package jdk.internal.loader;

import java.io.ByteArrayInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.net.MalformedURLException;
import java.net.URL;
import java.net.URLConnection;
import java.net.URLStreamHandler;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Enumeration;
import java.util.List;

import sun.net.www.ParseUtil;

public final class EmbeddedClassPath {
    /** URL 协议名：协议处理器类为 sun.net.www.protocol.<协议名>.Handler（URL$DefaultFactory 的命名约定） */
    static final String PROTOCOL = "ravacp";

    private static final URLStreamHandler HANDLER = new sun.net.www.protocol.ravacp.Handler();

    private EmbeddedClassPath() {}

    /** 名为 name 的嵌入资源份数（未命中 0） */
    private static native int count(String name);

    /** 名为 name 的第 index 份嵌入资源的字节（越界 null） */
    private static native byte[] bytes(String name, int index);

    /** 首个同名资源的 URL；未命中 null */
    static URL findResource(String name) {
        return count(name) > 0 ? url(name, 0) : null;
    }

    /** 同名资源的 URL，类路径序 */
    static Enumeration<URL> findResources(String name) {
        int n = count(name);
        if (n == 0) {
            return Collections.emptyEnumeration();
        }
        List<URL> urls = new ArrayList<>(n);
        for (int i = 0; i < n; i++) {
            urls.add(url(name, i));
        }
        return Collections.enumeration(urls);
    }

    @SuppressWarnings("deprecation")
    private static URL url(String name, int index) {
        String file = "/" + ParseUtil.encodePath(name, false) + (index == 0 ? "" : "#" + index);
        try {
            return new URL(PROTOCOL, "", -1, file, HANDLER);
        } catch (MalformedURLException e) {
            throw new InternalError(e);
        }
    }

    /**
     * 嵌入资源 URL 的连接（协议处理器 sun.net.www.protocol.ravacp.Handler 的 openConnection）：
     * 与 {@link #url} 互逆——名取 URL 路径去掉首个 / 后解码，份序取 ref。连接时未命中 → FileNotFoundException
     */
    public static URLConnection openConnection(URL u) {
        return new Connection(u);
    }

    /** 嵌入资源的连接：字节取自嵌入表 */
    static final class Connection extends URLConnection {
        private byte[] data;

        Connection(URL u) {
            super(u);
        }

        @Override
        public void connect() throws IOException {
            if (data == null) {
                // 由字符串重建的 URL 形状任意：路径不以 / 开头、转义非法、ref 不是份序时同未命中
                String name = resourceName(url.getPath());
                String ref = url.getRef();
                int index = ref == null ? 0 : copyIndex(ref);
                byte[] b = name != null && index >= 0 ? bytes(name, index) : null;
                if (b == null) {
                    throw new FileNotFoundException(url.toString());
                }
                data = b;
            }
            connected = true;
        }

        /** URL 路径对应的资源名（去掉首个 / 后解码）；路径不以 / 开头或转义非法 → null */
        private static String resourceName(String path) {
            if (!path.startsWith("/")) {
                return null;
            }
            try {
                return ParseUtil.decode(path.substring(1));
            } catch (IllegalArgumentException e) {
                return null;
            }
        }

        /** ref 的份序（十进制非负整数）；否则 -1 */
        private static int copyIndex(String ref) {
            try {
                return Integer.parseInt(ref);
            } catch (NumberFormatException e) {
                return -1;
            }
        }

        @Override
        public InputStream getInputStream() throws IOException {
            connect();
            return new ByteArrayInputStream(data);
        }

        @Override
        public long getContentLengthLong() {
            try {
                connect();
            } catch (IOException e) {
                return -1;
            }
            return data.length;
        }
    }
}
