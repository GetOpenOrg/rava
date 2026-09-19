# JDK 扫描报告

生成时间：2026-09-19

## 摘要

| 来源 | 类数 |
|------|-----:|
| 调用链 BFS | 20 |
| Field-only stub | 75 |
| 合计 | 95 |

## 调用链 BFS 发现的类

| 类名 | 方法数 | native 数 |
|------|-------:|----------:|
| `java/lang/ArithmeticException` | 2 | 0 |
| `java/lang/ArrayIndexOutOfBoundsException` | 3 | 0 |
| `java/lang/ClassCastException` | 2 | 0 |
| `java/lang/CloneNotSupportedException` | 2 | 0 |
| `java/lang/Error` | 5 | 0 |
| `java/lang/Exception` | 5 | 0 |
| `java/lang/IllegalMonitorStateException` | 2 | 0 |
| `java/lang/LinkageError` | 3 | 0 |
| `java/lang/NoClassDefFoundError` | 2 | 0 |
| `java/lang/Number` | 7 | 0 |
| `java/lang/RuntimeException` | 5 | 0 |
| `java/lang/StringIndexOutOfBoundsException` | 3 | 0 |
| `java/lang/VirtualMachineError` | 4 | 0 |
| `java/lang/reflect/AccessibleObject` | 29 | 0 |
| `java/lang/reflect/Array` | 24 | 21 |
| `java/util/AbstractCollection` | 16 | 0 |
| `java/util/AbstractMap` | 18 | 0 |
| `java/util/AbstractSet` | 4 | 0 |
| `java/util/Objects` | 22 | 0 |
| `sun/nio/cs/Unicode` | 2 | 0 |

## Field-only Stub 类

| 类名 | 方法数 | native 数 |
|------|-------:|----------:|
| `java/io/BufferedOutputStream` | 12 | 0 |
| `java/io/BufferedWriter` | 20 | 0 |
| `java/io/Closeable` | 1 | 0 |
| `java/io/FileDescriptor` | 17 | 5 |
| `java/io/FileDescriptor$1` | 11 | 0 |
| `java/io/FileOutputStream` | 17 | 4 |
| `java/io/FilterOutputStream` | 6 | 0 |
| `java/io/IOException` | 4 | 0 |
| `java/io/InputStream` | 16 | 0 |
| `java/io/InterruptedIOException` | 2 | 0 |
| `java/io/ObjectStreamField` | 16 | 0 |
| `java/io/OutputStream` | 7 | 0 |
| `java/io/OutputStreamWriter` | 16 | 0 |
| `java/io/PrintStream` | 71 | 0 |
| `java/io/PrintStream$1` | 2 | 0 |
| `java/io/Writer` | 19 | 0 |
| `java/lang/AbstractStringBuilder` | 87 | 0 |
| `java/lang/AssertionError` | 10 | 0 |
| `java/lang/BaseVirtualThread` | 4 | 0 |
| `java/lang/Character` | 105 | 0 |
| `java/lang/Class` | 171 | 35 |
| `java/lang/Double` | 35 | 2 |
| `java/lang/ExceptionInInitializerError` | 7 | 0 |
| `java/lang/Float` | 38 | 2 |
| `java/lang/IllegalArgumentException` | 4 | 0 |
| `java/lang/IllegalStateException` | 4 | 0 |
| `java/lang/IndexOutOfBoundsException` | 4 | 0 |
| `java/lang/Integer` | 65 | 0 |
| `java/lang/Math` | 103 | 0 |
| `java/lang/NegativeArraySizeException` | 2 | 0 |
| `java/lang/NullPointerException` | 5 | 1 |
| `java/lang/OutOfMemoryError` | 2 | 0 |
| `java/lang/SecurityManager` | 41 | 1 |
| `java/lang/StackTraceElement` | 24 | 2 |
| `java/lang/String` | 168 | 1 |
| `java/lang/String$CaseInsensitiveComparator` | 4 | 0 |
| `java/lang/StringBuilder` | 97 | 0 |
| `java/lang/StringLatin1` | 51 | 0 |
| `java/lang/StringUTF16` | 95 | 1 |
| `java/lang/System` | 48 | 9 |
| `java/lang/Thread` | 114 | 20 |
| `java/lang/Throwable` | 28 | 1 |
| `java/lang/UnsupportedOperationException` | 4 | 0 |
| `java/lang/Void` | 2 | 0 |
| `java/lang/reflect/Field` | 54 | 1 |
| `java/nio/charset/Charset` | 34 | 0 |
| `java/util/AbstractList` | 20 | 0 |
| `java/util/ArrayList` | 68 | 0 |
| `java/util/ArrayList$Itr` | 6 | 0 |
| `java/util/Arrays` | 249 | 0 |
| `java/util/Collections` | 88 | 0 |
| `java/util/Collections$EmptyIterator` | 6 | 0 |
| `java/util/Collections$EmptyList` | 19 | 0 |
| `java/util/Collections$EmptyMap` | 24 | 0 |
| `java/util/Collections$EmptySet` | 14 | 0 |
| `java/util/Comparator` | 25 | 0 |
| `java/util/ConcurrentModificationException` | 4 | 0 |
| `java/util/Iterator` | 4 | 0 |
| `java/util/List` | 49 | 0 |
| `java/util/Map` | 39 | 0 |
| `java/util/Map$Entry` | 15 | 0 |
| `java/util/NoSuchElementException` | 4 | 0 |
| `java/util/Set` | 29 | 0 |
| `java/util/function/BiFunction` | 3 | 0 |
| `jdk/internal/access/JavaIOFileDescriptorAccess` | 10 | 0 |
| `jdk/internal/access/JavaIOPrintStreamAccess` | 1 | 0 |
| `jdk/internal/access/SharedSecrets` | 72 | 0 |
| `jdk/internal/misc/Blocker` | 6 | 0 |
| `jdk/internal/misc/InternalLock` | 8 | 0 |
| `jdk/internal/misc/VM` | 35 | 8 |
| `jdk/internal/util/ArraysSupport` | 31 | 0 |
| `jdk/internal/util/Preconditions` | 17 | 0 |
| `sun/nio/ch/Interruptible` | 1 | 0 |
| `sun/nio/cs/StreamEncoder` | 32 | 0 |
| `sun/nio/cs/UTF_8` | 7 | 0 |
