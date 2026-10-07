/*
 * VM 支持类（rava）：构建期嵌入资源 URL（协议 ravacp）的协议处理器。
 *
 * 原生二进制不设运行期应用类路径，类路径资源与 JDK 模块资源在构建期嵌入二进制，由
 * jdk.internal.loader.EmbeddedClassPath 给出 URL（ravacp:/<encodePath(资源名)>[#i]）。JVM 上这些 URL 是
 * file: / jar: / jrt: URL，程序可以 toString 后按字符串重建（new URL(s)、URI.create(s).toURL()）再读取。
 * 本类按 URL$DefaultFactory 的命名约定（"sun.net.www.protocol." + 协议 + ".Handler"）放在协议名对应的包里，
 * 重建时 URL.getURLStreamHandler 经内建工厂按协议名反射构造本类，连接读同一张嵌入表。
 * EmbeddedClassPath 构造 URL 时也显式给出本类实例，两条途径的 URL 处理器同类。
 * 方案：docs/plans/2026-10-01-c1d-closure-bloat.md §30.16
 *
 * 编译：转译时以当前 JDK 的 javac --patch-module java.base 编入（generator/crates/resolve/src/image.rs
 * VM 支持类目录；目录名即所属模块，本包由此归属 java.base）。
 */
package sun.net.www.protocol.ravacp;

import java.net.URL;
import java.net.URLConnection;
import java.net.URLStreamHandler;

import jdk.internal.loader.EmbeddedClassPath;

public class Handler extends URLStreamHandler {
    /** 内建工厂以无参构造器反射构造 */
    public Handler() {}

    @Override
    protected URLConnection openConnection(URL u) {
        return EmbeddedClassPath.openConnection(u);
    }
}
