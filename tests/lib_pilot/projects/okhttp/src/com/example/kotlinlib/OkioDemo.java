package com.example.kotlinlib;

import okio.Buffer;
import okio.ByteString;

/**
 * Java 调用 Kotlin 库：Okio（OkHttp 的底层 I/O 库，纯 Kotlin）。
 * ByteString 的编解码/摘要与 Buffer 的流式读写都是纯计算，天然输出确定。
 */
public class OkioDemo {

    public static void main(String[] args) throws Exception {
        // ---------- 1. ByteString：编码、摘要、十六进制 ----------
        ByteString bytes = ByteString.encodeUtf8("hello okio");
        System.out.println("[utf8编码]  " + bytes.utf8());
        System.out.println("[md5]       " + bytes.md5().hex());
        System.out.println("[sha256]    " + bytes.sha256().hex());
        System.out.println("[base64]    " + bytes.base64());

        // ---------- 2. Buffer：流式读写（读后消耗，size 变化可验证） ----------
        Buffer buffer = new Buffer()
                .writeUtf8("hello ")
                .writeUtf8("okio")
                .writeByte((byte) '!');
        System.out.println("[Buffer写入] 总字节数 = " + buffer.size());
        System.out.println("[Buffer读5]  " + buffer.readUtf8(5));   // 读走 hello
        System.out.println("[Buffer剩余] " + buffer.readUtf8() + "（剩余 " + buffer.size() + " 字节）");

        // ---------- 3. 十六进制解码：UTF-8 中文 ----------
        ByteString chinese = ByteString.decodeHex("e4bda0e5a5bd");
        System.out.println("[hex解码]   " + chinese.utf8());
    }
}
