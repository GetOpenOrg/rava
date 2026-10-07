/*
 * VM 支持类（rava）：构建期嵌入的类路径资源（用户决策 (c)：原生二进制不设运行期应用类路径）。
 *
 * JVM 上应用类加载器经 URLClassPath 在运行期按类路径读取资源（目录 / jar 内文件）。原生二进制的类
 * 全集在构建期静态链接，类路径（用户编译输出与库归档）的全部文件在构建期按类路径顺序嵌入二进制；
 * 同名资源按类路径序保留多份（对应 getResources 的枚举序）。视图与系统类加载器所见一致：模块资源
 *（JDK 侧编译期嵌入表，含 <类名>.class 形态的 JDK 类文件）在前，类路径在后。本类以 Java 给出资源面：URL 与字节流，
 * 读表的两个 native 由 runtime/java_runtime/src/jdk/internal/loader/embedded_class_path_impl.rs 承载。
 * 入口：BuiltinClassLoader.findResourceOnClassPath / findResourcesOnClassPath（vm_intrinsics.toml
 * kind = class_path）与过渡期的 ClassLoader 资源族手写。
 * 方案：docs/plans/2026-10-01-c1d-closure-bloat.md §30.15
 *
 * URL 形如 rava-cp:/<encodePath(资源名)>，同名第 i（≥ 1）份带 #i；构造时显式给出本类的 Handler
 *（不经协议名查找处理器，toString 后按字符串重建 URL 不受支持，见计划 §30.15 取舍 T2）。
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

final class EmbeddedClassPath {
    /** URL 协议名 */
    static final String PROTOCOL = "rava-cp";

    private static final Handler HANDLER = new Handler();

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

    /** 首个同名资源的字节流；未命中 null */
    static InputStream stream(String name) {
        byte[] b = bytes(name, 0);
        return b == null ? null : new ByteArrayInputStream(b);
    }

    @SuppressWarnings("deprecation")
    private static URL url(String name, int index) {
        String file = "/" + ParseUtil.encodePath(name, false) + (index == 0 ? "" : "#" + index);
        try {
            return new URL(PROTOCOL, null, -1, file, HANDLER);
        } catch (MalformedURLException e) {
            throw new InternalError(e);
        }
    }

    /** 嵌入资源的 URL 处理器：连接读表字节 */
    static final class Handler extends URLStreamHandler {
        @Override
        protected URLConnection openConnection(URL u) {
            return new Connection(u);
        }
    }

    /** 嵌入资源的连接：字节取自嵌入表（名取 URL 路径去掉首个 /，份序取 ref） */
    static final class Connection extends URLConnection {
        private byte[] data;

        Connection(URL u) {
            super(u);
        }

        @Override
        public void connect() throws IOException {
            if (data == null) {
                String name = ParseUtil.decode(url.getPath().substring(1));
                String ref = url.getRef();
                byte[] b = bytes(name, ref == null ? 0 : Integer.parseInt(ref));
                if (b == null) {
                    throw new FileNotFoundException(url.toString());
                }
                data = b;
            }
            connected = true;
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
