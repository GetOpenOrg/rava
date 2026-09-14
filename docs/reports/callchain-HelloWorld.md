# 调用链分析报告：HelloWorld

生成时间：2026-09-14

## 摘要

| 策略 | 类数 | 方法数 |
|------|-----:|-------:|
| 类级 BFS | 3129 | — |
| 方法级 BFS | 880 | 5855 |
| 节省（方法级不需要） | 2249 | — |

## 类级 BFS 发现的类

| 类名 | 方法数 | native 数 |
|------|-------:|----------:|
| `com/sun/crypto/provider/SunJCE` | 7 | 0 |
| `com/sun/crypto/provider/SunJCE$1` | 3 | 0 |
| `java/io/BufferedInputStream` | 25 | 0 |
| `java/io/BufferedOutputStream` | 12 | 0 |
| `java/io/BufferedReader` | 24 | 0 |
| `java/io/BufferedReader$1` | 4 | 0 |
| `java/io/BufferedWriter` | 20 | 0 |
| `java/io/ByteArrayInputStream` | 14 | 0 |
| `java/io/ByteArrayOutputStream` | 15 | 0 |
| `java/io/ClassCache` | 4 | 0 |
| `java/io/ClassCache$1` | 3 | 0 |
| `java/io/ClassCache$CacheRef` | 4 | 0 |
| `java/io/Closeable` | 1 | 0 |
| `java/io/Console` | 23 | 2 |
| `java/io/DataInput` | 15 | 0 |
| `java/io/DataInputStream` | 20 | 0 |
| `java/io/DataOutput` | 14 | 0 |
| `java/io/DataOutputStream` | 19 | 0 |
| `java/io/DeleteOnExitHook` | 4 | 0 |
| `java/io/EOFException` | 2 | 0 |
| `java/io/Externalizable` | 2 | 0 |
| `java/io/File` | 64 | 0 |
| `java/io/File$TempDirectory` | 5 | 0 |
| `java/io/FileCleanable` | 6 | 1 |
| `java/io/FileDescriptor` | 17 | 5 |
| `java/io/FileFilter` | 1 | 0 |
| `java/io/FileInputStream` | 26 | 8 |
| `java/io/FileInputStream$1` | 2 | 0 |
| `java/io/FileNotFoundException` | 3 | 0 |
| `java/io/FileOutputStream` | 17 | 4 |
| `java/io/FileOutputStream$1` | 2 | 0 |
| `java/io/FilePermission` | 19 | 0 |
| `java/io/FilePermission$2` | 3 | 0 |
| `java/io/FilePermissionCollection` | 8 | 0 |
| `java/io/FileReader` | 5 | 0 |
| `java/io/FileSystem` | 30 | 0 |
| `java/io/FileWriter` | 9 | 0 |
| `java/io/FilenameFilter` | 1 | 0 |
| `java/io/FilterInputStream` | 10 | 0 |
| `java/io/FilterOutputStream` | 6 | 0 |
| `java/io/Flushable` | 1 | 0 |
| `java/io/IOException` | 4 | 0 |
| `java/io/InputStream` | 16 | 0 |
| `java/io/InputStream$1` | 12 | 0 |
| `java/io/InputStreamReader` | 11 | 0 |
| `java/io/InterruptedIOException` | 2 | 0 |
| `java/io/InvalidClassException` | 5 | 0 |
| `java/io/InvalidObjectException` | 2 | 0 |
| `java/io/NotActiveException` | 2 | 0 |
| `java/io/NotSerializableException` | 2 | 0 |
| `java/io/ObjectInput` | 7 | 0 |
| `java/io/ObjectInputFilter` | 5 | 0 |
| `java/io/ObjectInputFilter$Config` | 13 | 0 |
| `java/io/ObjectInputFilter$Config$Global` | 18 | 0 |
| `java/io/ObjectInputFilter$Config$MergeFilter` | 3 | 0 |
| `java/io/ObjectInputFilter$Config$PredicateFilter` | 3 | 0 |
| `java/io/ObjectInputFilter$Config$RejectUndecidedFilter` | 3 | 0 |
| `java/io/ObjectInputFilter$Config$RejectUndecidedFilter$SerialInfo` | 6 | 0 |
| `java/io/ObjectInputFilter$FilterInfo` | 5 | 0 |
| `java/io/ObjectInputFilter$Status` | 5 | 0 |
| `java/io/ObjectInputStream` | 66 | 0 |
| `java/io/ObjectInputStream$1` | 3 | 0 |
| `java/io/ObjectInputStream$BlockDataInputStream` | 43 | 0 |
| `java/io/ObjectInputStream$FieldValues` | 15 | 0 |
| `java/io/ObjectInputStream$FilterValues` | 6 | 0 |
| `java/io/ObjectInputStream$GetField` | 12 | 0 |
| `java/io/ObjectInputStream$HandleTable` | 11 | 0 |
| `java/io/ObjectInputStream$HandleTable$HandleList` | 4 | 0 |
| `java/io/ObjectInputStream$PeekInputStream` | 9 | 0 |
| `java/io/ObjectInputStream$ValidationList` | 4 | 0 |
| `java/io/ObjectInputStream$ValidationList$1` | 3 | 0 |
| `java/io/ObjectInputStream$ValidationList$Callback` | 1 | 0 |
| `java/io/ObjectInputValidation` | 1 | 0 |
| `java/io/ObjectOutput` | 6 | 0 |
| `java/io/ObjectOutputStream` | 56 | 0 |
| `java/io/ObjectOutputStream$1` | 3 | 0 |
| `java/io/ObjectOutputStream$BlockDataOutputStream` | 34 | 0 |
| `java/io/ObjectOutputStream$DebugTraceInfoStack` | 5 | 0 |
| `java/io/ObjectOutputStream$HandleTable` | 9 | 0 |
| `java/io/ObjectOutputStream$PutField` | 11 | 0 |
| `java/io/ObjectOutputStream$PutFieldImpl` | 13 | 0 |
| `java/io/ObjectOutputStream$ReplaceTable` | 6 | 0 |
| `java/io/ObjectStreamClass` | 80 | 2 |
| `java/io/ObjectStreamClass$1` | 3 | 0 |
| `java/io/ObjectStreamClass$2` | 3 | 0 |
| `java/io/ObjectStreamClass$3` | 3 | 0 |
| `java/io/ObjectStreamClass$4` | 3 | 0 |
| `java/io/ObjectStreamClass$5` | 3 | 0 |
| `java/io/ObjectStreamClass$ClassDataSlot` | 1 | 0 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache` | 4 | 0 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key` | 6 | 0 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key$Impl` | 4 | 0 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key$Lookup` | 4 | 0 |
| `java/io/ObjectStreamClass$ExceptionInfo` | 2 | 0 |
| `java/io/ObjectStreamClass$FieldReflector` | 9 | 0 |
| `java/io/ObjectStreamClass$FieldReflectorKey` | 3 | 0 |
| `java/io/ObjectStreamClass$MemberSignature` | 3 | 0 |
| `java/io/ObjectStreamClass$RecordSupport` | 5 | 0 |
| `java/io/ObjectStreamException` | 4 | 0 |
| `java/io/ObjectStreamField` | 16 | 0 |
| `java/io/OptionalDataException` | 2 | 0 |
| `java/io/OutputStream` | 7 | 0 |
| `java/io/OutputStream$1` | 5 | 0 |
| `java/io/OutputStreamWriter` | 16 | 0 |
| `java/io/PrintStream` | 71 | 0 |
| `java/io/PrintWriter` | 67 | 0 |
| `java/io/ProxyingConsole` | 11 | 0 |
| `java/io/ProxyingConsole$WrappingReader` | 3 | 0 |
| `java/io/ProxyingConsole$WrappingWriter` | 4 | 0 |
| `java/io/PushbackInputStream` | 16 | 0 |
| `java/io/RandomAccessFile` | 55 | 10 |
| `java/io/RandomAccessFile$1` | 2 | 0 |
| `java/io/Reader` | 16 | 0 |
| `java/io/Reader$1` | 9 | 0 |
| `java/io/SerialCallbackContext` | 6 | 0 |
| `java/io/Serializable` | 0 | 0 |
| `java/io/StreamCorruptedException` | 2 | 0 |
| `java/io/StreamTokenizer` | 20 | 0 |
| `java/io/StringReader` | 10 | 0 |
| `java/io/UTFDataFormatException` | 2 | 0 |
| `java/io/UncheckedIOException` | 5 | 0 |
| `java/io/UnsupportedEncodingException` | 2 | 0 |
| `java/io/WriteAbortedException` | 3 | 0 |
| `java/io/Writer` | 19 | 0 |
| `java/io/Writer$1` | 14 | 0 |
| `java/lang/AbstractMethodError` | 2 | 0 |
| `java/lang/AbstractStringBuilder` | 87 | 0 |
| `java/lang/Appendable` | 3 | 0 |
| `java/lang/ApplicationShutdownHooks` | 5 | 0 |
| `java/lang/ArithmeticException` | 2 | 0 |
| `java/lang/ArrayIndexOutOfBoundsException` | 3 | 0 |
| `java/lang/ArrayStoreException` | 2 | 0 |
| `java/lang/AssertionError` | 10 | 0 |
| `java/lang/BaseVirtualThread` | 4 | 0 |
| `java/lang/Boolean` | 20 | 0 |
| `java/lang/BootstrapMethodError` | 4 | 0 |
| `java/lang/Byte` | 27 | 0 |
| `java/lang/CharSequence` | 10 | 0 |
| `java/lang/CharSequence$1CharIterator` | 5 | 0 |
| `java/lang/CharSequence$1CodePointIterator` | 5 | 0 |
| `java/lang/Character` | 105 | 0 |
| `java/lang/Character$Subset` | 4 | 0 |
| `java/lang/Character$UnicodeBlock` | 7 | 0 |
| `java/lang/Character$UnicodeScript` | 7 | 0 |
| `java/lang/CharacterData` | 30 | 0 |
| `java/lang/CharacterDataLatin1` | 32 | 0 |
| `java/lang/CharacterName` | 9 | 0 |
| `java/lang/CharacterName$1` | 3 | 0 |
| `java/lang/Class` | 171 | 35 |
| `java/lang/Class$1` | 3 | 0 |
| `java/lang/Class$2` | 3 | 0 |
| `java/lang/Class$3` | 3 | 0 |
| `java/lang/Class$AnnotationData` | 1 | 0 |
| `java/lang/Class$Atomic` | 5 | 0 |
| `java/lang/Class$EnclosingMethodInfo` | 9 | 0 |
| `java/lang/Class$ReflectionData` | 2 | 0 |
| `java/lang/ClassCastException` | 2 | 0 |
| `java/lang/ClassFormatError` | 2 | 0 |
| `java/lang/ClassLoader` | 93 | 7 |
| `java/lang/ClassLoader$1` | 3 | 0 |
| `java/lang/ClassLoader$ParallelLoaders` | 4 | 0 |
| `java/lang/ClassNotFoundException` | 7 | 0 |
| `java/lang/ClassValue` | 16 | 0 |
| `java/lang/ClassValue$ClassValueMap` | 22 | 0 |
| `java/lang/ClassValue$Entry` | 10 | 0 |
| `java/lang/ClassValue$Identity` | 1 | 0 |
| `java/lang/ClassValue$Version` | 4 | 0 |
| `java/lang/CloneNotSupportedException` | 2 | 0 |
| `java/lang/Cloneable` | 0 | 0 |
| `java/lang/Comparable` | 1 | 0 |
| `java/lang/CompoundEnumeration` | 4 | 0 |
| `java/lang/ConditionalSpecialCasing` | 15 | 0 |
| `java/lang/ConditionalSpecialCasing$Entry` | 6 | 0 |
| `java/lang/Double` | 35 | 2 |
| `java/lang/Enum` | 16 | 0 |
| `java/lang/Enum$EnumDesc` | 5 | 0 |
| `java/lang/EnumConstantNotPresentException` | 3 | 0 |
| `java/lang/Error` | 5 | 0 |
| `java/lang/Exception` | 5 | 0 |
| `java/lang/ExceptionInInitializerError` | 7 | 0 |
| `java/lang/FdLibm` | 6 | 0 |
| `java/lang/FdLibm$Acos` | 2 | 0 |
| `java/lang/FdLibm$Asin` | 2 | 0 |
| `java/lang/FdLibm$Atan` | 3 | 0 |
| `java/lang/FdLibm$Atan2` | 2 | 0 |
| `java/lang/FdLibm$Cbrt` | 2 | 0 |
| `java/lang/FdLibm$Cos` | 3 | 0 |
| `java/lang/FdLibm$Cosh` | 2 | 0 |
| `java/lang/FdLibm$Exp` | 3 | 0 |
| `java/lang/FdLibm$Expm1` | 2 | 0 |
| `java/lang/FdLibm$Hypot` | 3 | 0 |
| `java/lang/FdLibm$IEEEremainder` | 5 | 0 |
| `java/lang/FdLibm$KernelRemPio2` | 3 | 0 |
| `java/lang/FdLibm$Log` | 2 | 0 |
| `java/lang/FdLibm$Log10` | 2 | 0 |
| `java/lang/FdLibm$Log1p` | 2 | 0 |
| `java/lang/FdLibm$Pow` | 2 | 0 |
| `java/lang/FdLibm$RemPio2` | 3 | 0 |
| `java/lang/FdLibm$Sin` | 3 | 0 |
| `java/lang/FdLibm$Sinh` | 2 | 0 |
| `java/lang/FdLibm$Sqrt` | 2 | 0 |
| `java/lang/FdLibm$Tan` | 4 | 0 |
| `java/lang/FdLibm$Tanh` | 2 | 0 |
| `java/lang/Float` | 38 | 2 |
| `java/lang/IllegalAccessError` | 2 | 0 |
| `java/lang/IllegalAccessException` | 2 | 0 |
| `java/lang/IllegalArgumentException` | 4 | 0 |
| `java/lang/IllegalCallerException` | 4 | 0 |
| `java/lang/IllegalMonitorStateException` | 2 | 0 |
| `java/lang/IllegalStateException` | 4 | 0 |
| `java/lang/IllegalThreadStateException` | 2 | 0 |
| `java/lang/IncompatibleClassChangeError` | 2 | 0 |
| `java/lang/IndexOutOfBoundsException` | 4 | 0 |
| `java/lang/InstantiationException` | 2 | 0 |
| `java/lang/Integer` | 65 | 0 |
| `java/lang/InternalError` | 4 | 0 |
| `java/lang/InterruptedException` | 2 | 0 |
| `java/lang/Iterable` | 3 | 0 |
| `java/lang/LayerInstantiationException` | 4 | 0 |
| `java/lang/LinkageError` | 3 | 0 |
| `java/lang/LiveStackFrame` | 8 | 0 |
| `java/lang/LiveStackFrame$PrimitiveSlot` | 4 | 0 |
| `java/lang/LiveStackFrameInfo` | 8 | 0 |
| `java/lang/LiveStackFrameInfo$PrimitiveSlot32` | 4 | 0 |
| `java/lang/LiveStackFrameInfo$PrimitiveSlot64` | 4 | 0 |
| `java/lang/Long` | 66 | 0 |
| `java/lang/MatchException` | 1 | 0 |
| `java/lang/Math` | 103 | 0 |
| `java/lang/Module` | 71 | 5 |
| `java/lang/Module$1` | 3 | 0 |
| `java/lang/Module$EnableNativeAccess` | 4 | 0 |
| `java/lang/ModuleLayer` | 28 | 0 |
| `java/lang/ModuleLayer$Controller` | 7 | 0 |
| `java/lang/NamedPackage` | 5 | 0 |
| `java/lang/NegativeArraySizeException` | 2 | 0 |
| `java/lang/NoClassDefFoundError` | 2 | 0 |
| `java/lang/NoSuchFieldError` | 2 | 0 |
| `java/lang/NoSuchFieldException` | 2 | 0 |
| `java/lang/NoSuchMethodError` | 2 | 0 |
| `java/lang/NoSuchMethodException` | 2 | 0 |
| `java/lang/NullPointerException` | 5 | 1 |
| `java/lang/Number` | 7 | 0 |
| `java/lang/NumberFormatException` | 4 | 0 |
| `java/lang/Object` | 13 | 6 |
| `java/lang/OutOfMemoryError` | 2 | 0 |
| `java/lang/Package` | 25 | 0 |
| `java/lang/Package$VersionInfo` | 3 | 0 |
| `java/lang/PinnedThreadPrinter` | 13 | 0 |
| `java/lang/PinnedThreadPrinter$Hashes` | 3 | 0 |
| `java/lang/Process` | 25 | 0 |
| `java/lang/Process$1` | 3 | 0 |
| `java/lang/Process$CharsetHolder` | 3 | 0 |
| `java/lang/Process$PipeInputStream` | 2 | 0 |
| `java/lang/ProcessBuilder` | 27 | 0 |
| `java/lang/ProcessBuilder$Redirect` | 10 | 0 |
| `java/lang/ProcessBuilder$Redirect$4` | 4 | 0 |
| `java/lang/ProcessBuilder$Redirect$5` | 5 | 0 |
| `java/lang/ProcessBuilder$Redirect$6` | 5 | 0 |
| `java/lang/ProcessBuilder$Redirect$Type` | 5 | 0 |
| `java/lang/ProcessBuilder$RedirectPipeImpl` | 4 | 0 |
| `java/lang/ProcessEnvironment` | 10 | 1 |
| `java/lang/ProcessEnvironment$ExternalData` | 5 | 0 |
| `java/lang/ProcessEnvironment$StringEntry` | 10 | 0 |
| `java/lang/ProcessEnvironment$StringEntrySet` | 10 | 0 |
| `java/lang/ProcessEnvironment$StringEntrySet$1` | 5 | 0 |
| `java/lang/ProcessEnvironment$StringEntrySet$2` | 7 | 0 |
| `java/lang/ProcessEnvironment$StringEnvironment` | 17 | 0 |
| `java/lang/ProcessEnvironment$StringKeySet` | 7 | 0 |
| `java/lang/ProcessEnvironment$StringKeySet$1` | 5 | 0 |
| `java/lang/ProcessEnvironment$StringValues` | 9 | 0 |
| `java/lang/ProcessEnvironment$StringValues$1` | 5 | 0 |
| `java/lang/ProcessEnvironment$Value` | 8 | 0 |
| `java/lang/ProcessEnvironment$Variable` | 8 | 0 |
| `java/lang/ProcessHandle` | 17 | 0 |
| `java/lang/ProcessHandleImpl` | 41 | 7 |
| `java/lang/ProcessHandleImpl$1` | 2 | 0 |
| `java/lang/ProcessHandleImpl$ExitCompletion` | 1 | 0 |
| `java/lang/ProcessHandleImpl$Info` | 12 | 2 |
| `java/lang/ProcessImpl` | 28 | 2 |
| `java/lang/ProcessImpl$DeferredCloseProcessPipeInputStream` | 11 | 0 |
| `java/lang/ProcessImpl$LaunchMechanism` | 5 | 0 |
| `java/lang/ProcessImpl$ProcessPipeInputStream` | 4 | 0 |
| `java/lang/ProcessImpl$ProcessPipeOutputStream` | 2 | 0 |
| `java/lang/PublicMethods` | 3 | 0 |
| `java/lang/PublicMethods$Key` | 5 | 0 |
| `java/lang/PublicMethods$MethodList` | 6 | 0 |
| `java/lang/Readable` | 1 | 0 |
| `java/lang/Record` | 4 | 0 |
| `java/lang/ReflectiveOperationException` | 4 | 0 |
| `java/lang/Runnable` | 1 | 0 |
| `java/lang/Runtime` | 24 | 5 |
| `java/lang/Runtime$Version` | 27 | 0 |
| `java/lang/RuntimeException` | 5 | 0 |
| `java/lang/RuntimePermission` | 2 | 0 |
| `java/lang/SecurityException` | 4 | 0 |
| `java/lang/SecurityManager` | 41 | 1 |
| `java/lang/SecurityManager$1` | 3 | 0 |
| `java/lang/SecurityManager$2` | 3 | 0 |
| `java/lang/Short` | 28 | 0 |
| `java/lang/Shutdown` | 10 | 2 |
| `java/lang/StackFrameInfo` | 16 | 0 |
| `java/lang/StackStreamFactory` | 9 | 1 |
| `java/lang/StackStreamFactory$AbstractStackWalker` | 24 | 3 |
| `java/lang/StackStreamFactory$CallerClassFinder` | 8 | 0 |
| `java/lang/StackStreamFactory$CallerClassFinder$ClassBuffer` | 7 | 0 |
| `java/lang/StackStreamFactory$FrameBuffer` | 17 | 0 |
| `java/lang/StackStreamFactory$LiveStackInfoTraverser` | 3 | 0 |
| `java/lang/StackStreamFactory$LiveStackInfoTraverser$LiveStackFrameBuffer` | 9 | 0 |
| `java/lang/StackStreamFactory$StackFrameTraverser` | 12 | 0 |
| `java/lang/StackStreamFactory$StackFrameTraverser$StackFrameBuffer` | 9 | 0 |
| `java/lang/StackStreamFactory$WalkerState` | 5 | 0 |
| `java/lang/StackTraceElement` | 24 | 2 |
| `java/lang/StackTraceElement$HashedModules` | 4 | 0 |
| `java/lang/StackWalker` | 29 | 0 |
| `java/lang/StackWalker$Option` | 5 | 0 |
| `java/lang/StackWalker$StackFrame` | 10 | 0 |
| `java/lang/StrictMath` | 102 | 0 |
| `java/lang/String` | 168 | 1 |
| `java/lang/StringBuffer` | 101 | 0 |
| `java/lang/StringBuilder` | 97 | 0 |
| `java/lang/StringCoding` | 5 | 0 |
| `java/lang/StringConcatHelper` | 36 | 0 |
| `java/lang/StringIndexOutOfBoundsException` | 3 | 0 |
| `java/lang/StringLatin1` | 51 | 0 |
| `java/lang/StringLatin1$CharsSpliterator` | 11 | 0 |
| `java/lang/StringLatin1$LinesSpliterator` | 10 | 0 |
| `java/lang/StringUTF16` | 95 | 1 |
| `java/lang/StringUTF16$CharsSpliterator` | 11 | 0 |
| `java/lang/StringUTF16$CodePointsSpliterator` | 12 | 0 |
| `java/lang/StringUTF16$LinesSpliterator` | 10 | 0 |
| `java/lang/System` | 48 | 9 |
| `java/lang/System$1` | 2 | 0 |
| `java/lang/System$2` | 87 | 0 |
| `java/lang/System$Logger` | 10 | 0 |
| `java/lang/System$Logger$Level` | 7 | 0 |
| `java/lang/System$LoggerFinder` | 9 | 0 |
| `java/lang/Terminator` | 4 | 0 |
| `java/lang/Terminator$1` | 2 | 0 |
| `java/lang/Thread` | 114 | 20 |
| `java/lang/Thread$1` | 3 | 0 |
| `java/lang/Thread$Builder$OfVirtual` | 8 | 0 |
| `java/lang/Thread$FieldHolder` | 1 | 0 |
| `java/lang/Thread$State` | 5 | 0 |
| `java/lang/Thread$ThreadIdentifiers` | 3 | 0 |
| `java/lang/Thread$ThreadNumbering` | 3 | 0 |
| `java/lang/Thread$UncaughtExceptionHandler` | 1 | 0 |
| `java/lang/ThreadBuilders` | 2 | 0 |
| `java/lang/ThreadBuilders$BaseThreadBuilder` | 10 | 0 |
| `java/lang/ThreadBuilders$BaseThreadFactory` | 5 | 0 |
| `java/lang/ThreadBuilders$BoundVirtualThread` | 7 | 0 |
| `java/lang/ThreadBuilders$PlatformThreadBuilder` | 17 | 0 |
| `java/lang/ThreadBuilders$PlatformThreadFactory` | 3 | 0 |
| `java/lang/ThreadBuilders$VirtualThreadBuilder` | 13 | 0 |
| `java/lang/ThreadBuilders$VirtualThreadFactory` | 2 | 0 |
| `java/lang/ThreadGroup` | 40 | 0 |
| `java/lang/ThreadLocal` | 25 | 0 |
| `java/lang/ThreadLocal$SuppliedThreadLocal` | 2 | 0 |
| `java/lang/ThreadLocal$ThreadLocalMap` | 17 | 0 |
| `java/lang/ThreadLocal$ThreadLocalMap$Entry` | 1 | 0 |
| `java/lang/Throwable` | 28 | 1 |
| `java/lang/Throwable$PrintStreamOrWriter` | 4 | 0 |
| `java/lang/Throwable$WrappedPrintStream` | 3 | 0 |
| `java/lang/Throwable$WrappedPrintWriter` | 3 | 0 |
| `java/lang/TypeNotPresentException` | 2 | 0 |
| `java/lang/UnsatisfiedLinkError` | 2 | 0 |
| `java/lang/UnsupportedClassVersionError` | 2 | 0 |
| `java/lang/UnsupportedOperationException` | 4 | 0 |
| `java/lang/VersionProps` | 11 | 0 |
| `java/lang/VirtualMachineError` | 4 | 0 |
| `java/lang/VirtualThread` | 64 | 6 |
| `java/lang/VirtualThread$VThreadContinuation` | 3 | 0 |
| `java/lang/VirtualThread$VThreadContinuation$1` | 2 | 0 |
| `java/lang/Void` | 2 | 0 |
| `java/lang/WeakPairMap` | 9 | 0 |
| `java/lang/WeakPairMap$Pair` | 6 | 0 |
| `java/lang/WeakPairMap$Pair$Lookup` | 5 | 0 |
| `java/lang/WeakPairMap$Pair$Weak` | 6 | 0 |
| `java/lang/WeakPairMap$Pair$Weak$1` | 2 | 0 |
| `java/lang/WeakPairMap$WeakRefPeer` | 2 | 0 |
| `java/lang/WrongThreadException` | 4 | 0 |
| `java/lang/annotation/Annotation` | 4 | 0 |
| `java/lang/annotation/AnnotationFormatError` | 3 | 0 |
| `java/lang/annotation/AnnotationTypeMismatchException` | 3 | 0 |
| `java/lang/annotation/IncompleteAnnotationException` | 3 | 0 |
| `java/lang/annotation/Repeatable` | 1 | 0 |
| `java/lang/annotation/Retention` | 1 | 0 |
| `java/lang/constant/AsTypeMethodHandleDesc` | 5 | 0 |
| `java/lang/constant/ClassDesc` | 20 | 0 |
| `java/lang/constant/Constable` | 1 | 0 |
| `java/lang/constant/ConstantDesc` | 1 | 0 |
| `java/lang/constant/ConstantUtils` | 16 | 0 |
| `java/lang/constant/DirectMethodHandleDesc` | 6 | 0 |
| `java/lang/constant/DirectMethodHandleDesc$Kind` | 10 | 0 |
| `java/lang/constant/DirectMethodHandleDescImpl` | 15 | 0 |
| `java/lang/constant/DynamicConstantDesc` | 21 | 0 |
| `java/lang/constant/DynamicConstantDesc$AnonymousDynamicConstantDesc` | 1 | 0 |
| `java/lang/constant/MethodHandleDesc` | 9 | 0 |
| `java/lang/constant/MethodTypeDesc` | 25 | 0 |
| `java/lang/constant/MethodTypeDescImpl` | 25 | 0 |
| `java/lang/constant/MethodTypeDescImpl$1` | 3 | 0 |
| `java/lang/constant/PrimitiveClassDescImpl` | 5 | 0 |
| `java/lang/constant/ReferenceClassDescImpl` | 8 | 0 |
| `java/lang/invoke/AbstractConstantGroup` | 6 | 0 |
| `java/lang/invoke/AbstractConstantGroup$BSCIWithCache` | 6 | 0 |
| `java/lang/invoke/AbstractConstantGroup$WithCache` | 9 | 0 |
| `java/lang/invoke/BootstrapMethodInvoker` | 12 | 0 |
| `java/lang/invoke/BootstrapMethodInvoker$VM_BSCI` | 6 | 0 |
| `java/lang/invoke/BoundMethodHandle` | 31 | 0 |
| `java/lang/invoke/BoundMethodHandle$Specializer` | 8 | 0 |
| `java/lang/invoke/BoundMethodHandle$Specializer$Factory` | 2 | 0 |
| `java/lang/invoke/BoundMethodHandle$SpeciesData` | 10 | 0 |
| `java/lang/invoke/BoundMethodHandle$Species_L` | 11 | 0 |
| `java/lang/invoke/CallSite` | 20 | 0 |
| `java/lang/invoke/ClassSpecializer` | 22 | 0 |
| `java/lang/invoke/ClassSpecializer$Factory` | 18 | 0 |
| `java/lang/invoke/ClassSpecializer$Factory$1Var` | 11 | 0 |
| `java/lang/invoke/ClassSpecializer$SpeciesData` | 24 | 0 |
| `java/lang/invoke/ConstantBootstraps` | 13 | 0 |
| `java/lang/invoke/ConstantCallSite` | 6 | 0 |
| `java/lang/invoke/DelegatingMethodHandle` | 19 | 0 |
| `java/lang/invoke/DirectMethodHandle` | 44 | 0 |
| `java/lang/invoke/DirectMethodHandle$1` | 3 | 0 |
| `java/lang/invoke/DirectMethodHandle$Accessor` | 6 | 0 |
| `java/lang/invoke/DirectMethodHandle$Constructor` | 5 | 0 |
| `java/lang/invoke/DirectMethodHandle$Interface` | 6 | 0 |
| `java/lang/invoke/DirectMethodHandle$Special` | 7 | 0 |
| `java/lang/invoke/DirectMethodHandle$StaticAccessor` | 6 | 0 |
| `java/lang/invoke/IndirectVarHandle` | 10 | 0 |
| `java/lang/invoke/InfoFromMemberName` | 11 | 0 |
| `java/lang/invoke/InfoFromMemberName$1` | 3 | 0 |
| `java/lang/invoke/InvokerBytecodeGenerator` | 83 | 0 |
| `java/lang/invoke/InvokerBytecodeGenerator$BytecodeGenerationException` | 1 | 0 |
| `java/lang/invoke/InvokerBytecodeGenerator$ClassData` | 3 | 0 |
| `java/lang/invoke/Invokers` | 40 | 0 |
| `java/lang/invoke/LambdaForm` | 83 | 0 |
| `java/lang/invoke/LambdaForm$BasicType` | 18 | 0 |
| `java/lang/invoke/LambdaForm$Kind` | 6 | 0 |
| `java/lang/invoke/LambdaForm$Name` | 38 | 0 |
| `java/lang/invoke/LambdaForm$NamedFunction` | 28 | 0 |
| `java/lang/invoke/LambdaFormBuffer` | 30 | 0 |
| `java/lang/invoke/LambdaFormEditor` | 33 | 0 |
| `java/lang/invoke/LambdaFormEditor$1` | 3 | 0 |
| `java/lang/invoke/LambdaFormEditor$Transform` | 7 | 0 |
| `java/lang/invoke/LambdaFormEditor$TransformKey` | 27 | 0 |
| `java/lang/invoke/MemberName` | 83 | 0 |
| `java/lang/invoke/MemberName$Factory` | 5 | 0 |
| `java/lang/invoke/MethodHandle` | 61 | 8 |
| `java/lang/invoke/MethodHandle$1` | 3 | 0 |
| `java/lang/invoke/MethodHandleImpl` | 62 | 0 |
| `java/lang/invoke/MethodHandleImpl$ArrayAccess` | 9 | 0 |
| `java/lang/invoke/MethodHandleImpl$ArrayAccessor` | 33 | 0 |
| `java/lang/invoke/MethodHandleImpl$AsVarargsCollector` | 11 | 0 |
| `java/lang/invoke/MethodHandleImpl$BindCaller` | 11 | 0 |
| `java/lang/invoke/MethodHandleImpl$BindCaller$InjectedInvokerHolder` | 3 | 0 |
| `java/lang/invoke/MethodHandleImpl$CasesHolder` | 1 | 0 |
| `java/lang/invoke/MethodHandleImpl$CountingWrapper` | 6 | 0 |
| `java/lang/invoke/MethodHandleImpl$CountingWrapper$1` | 3 | 0 |
| `java/lang/invoke/MethodHandleImpl$Intrinsic` | 5 | 0 |
| `java/lang/invoke/MethodHandleImpl$IntrinsicMethodHandle` | 9 | 0 |
| `java/lang/invoke/MethodHandleImpl$LoopClauses` | 3 | 0 |
| `java/lang/invoke/MethodHandleImpl$TableSwitchCacheKey` | 4 | 0 |
| `java/lang/invoke/MethodHandleImpl$WrappedMember` | 6 | 0 |
| `java/lang/invoke/MethodHandleInfo` | 9 | 0 |
| `java/lang/invoke/MethodHandleNatives` | 50 | 12 |
| `java/lang/invoke/MethodHandleStatics` | 17 | 0 |
| `java/lang/invoke/MethodHandles` | 113 | 0 |
| `java/lang/invoke/MethodHandles$Lookup` | 90 | 0 |
| `java/lang/invoke/MethodHandles$Lookup$ClassDefiner` | 8 | 0 |
| `java/lang/invoke/MethodHandles$Lookup$ClassFile` | 6 | 0 |
| `java/lang/invoke/MethodHandles$Lookup$ClassOption` | 6 | 0 |
| `java/lang/invoke/MethodType` | 80 | 0 |
| `java/lang/invoke/MethodTypeForm` | 16 | 0 |
| `java/lang/invoke/NativeMethodHandle` | 10 | 0 |
| `java/lang/invoke/SerializedLambda` | 14 | 0 |
| `java/lang/invoke/SerializedLambda$1` | 3 | 0 |
| `java/lang/invoke/SimpleMethodHandle` | 12 | 0 |
| `java/lang/invoke/VarForm` | 9 | 0 |
| `java/lang/invoke/VarHandle` | 59 | 31 |
| `java/lang/invoke/VarHandle$AccessDescriptor` | 1 | 0 |
| `java/lang/invoke/VarHandle$AccessMode` | 8 | 0 |
| `java/lang/invoke/VarHandle$AccessType` | 8 | 0 |
| `java/lang/invoke/VarHandle$VarHandleDesc` | 8 | 0 |
| `java/lang/invoke/VarHandle$VarHandleDesc$Kind` | 6 | 0 |
| `java/lang/invoke/VarHandleBooleans$Array` | 37 | 0 |
| `java/lang/invoke/VarHandleBooleans$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleBooleans$FieldInstanceReadWrite` | 33 | 0 |
| `java/lang/invoke/VarHandleBooleans$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleBooleans$FieldStaticReadWrite` | 33 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsChars` | 3 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsChars$ArrayHandle` | 18 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsChars$ByteArrayViewVarHandle` | 1 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsChars$ByteBufferHandle` | 20 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles` | 4 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles$ArrayHandle` | 29 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles$ByteArrayViewVarHandle` | 1 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles$ByteBufferHandle` | 31 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsFloats` | 4 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsFloats$ArrayHandle` | 29 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsFloats$ByteArrayViewVarHandle` | 1 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsFloats$ByteBufferHandle` | 31 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsInts` | 3 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsInts$ArrayHandle` | 45 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsInts$ByteArrayViewVarHandle` | 1 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsInts$ByteBufferHandle` | 47 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsLongs` | 3 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsLongs$ArrayHandle` | 45 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsLongs$ByteArrayViewVarHandle` | 1 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsLongs$ByteBufferHandle` | 47 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsShorts` | 3 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsShorts$ArrayHandle` | 18 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsShorts$ByteArrayViewVarHandle` | 1 | 0 |
| `java/lang/invoke/VarHandleByteArrayAsShorts$ByteBufferHandle` | 20 | 0 |
| `java/lang/invoke/VarHandleByteArrayBase` | 3 | 0 |
| `java/lang/invoke/VarHandleBytes$Array` | 40 | 0 |
| `java/lang/invoke/VarHandleBytes$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleBytes$FieldInstanceReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleBytes$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleBytes$FieldStaticReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleChars$Array` | 40 | 0 |
| `java/lang/invoke/VarHandleChars$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleChars$FieldInstanceReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleChars$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleChars$FieldStaticReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleDoubles$Array` | 31 | 0 |
| `java/lang/invoke/VarHandleDoubles$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleDoubles$FieldInstanceReadWrite` | 27 | 0 |
| `java/lang/invoke/VarHandleDoubles$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleDoubles$FieldStaticReadWrite` | 27 | 0 |
| `java/lang/invoke/VarHandleFloats$Array` | 31 | 0 |
| `java/lang/invoke/VarHandleFloats$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleFloats$FieldInstanceReadWrite` | 27 | 0 |
| `java/lang/invoke/VarHandleFloats$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleFloats$FieldStaticReadWrite` | 27 | 0 |
| `java/lang/invoke/VarHandleInts$Array` | 40 | 0 |
| `java/lang/invoke/VarHandleInts$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleInts$FieldInstanceReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleInts$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleInts$FieldStaticReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleLongs$Array` | 40 | 0 |
| `java/lang/invoke/VarHandleLongs$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleLongs$FieldInstanceReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleLongs$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleLongs$FieldStaticReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleReferences$Array` | 30 | 0 |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite` | 24 | 0 |
| `java/lang/invoke/VarHandleReferences$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleReferences$FieldStaticReadWrite` | 24 | 0 |
| `java/lang/invoke/VarHandleSegmentAsBytes` | 19 | 0 |
| `java/lang/invoke/VarHandleSegmentAsChars` | 19 | 0 |
| `java/lang/invoke/VarHandleSegmentAsDoubles` | 31 | 0 |
| `java/lang/invoke/VarHandleSegmentAsFloats` | 31 | 0 |
| `java/lang/invoke/VarHandleSegmentAsInts` | 46 | 0 |
| `java/lang/invoke/VarHandleSegmentAsLongs` | 46 | 0 |
| `java/lang/invoke/VarHandleSegmentAsShorts` | 19 | 0 |
| `java/lang/invoke/VarHandleSegmentViewBase` | 2 | 0 |
| `java/lang/invoke/VarHandleShorts$Array` | 40 | 0 |
| `java/lang/invoke/VarHandleShorts$FieldInstanceReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleShorts$FieldInstanceReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandleShorts$FieldStaticReadOnly` | 13 | 0 |
| `java/lang/invoke/VarHandleShorts$FieldStaticReadWrite` | 36 | 0 |
| `java/lang/invoke/VarHandles` | 30 | 0 |
| `java/lang/invoke/WrongMethodTypeException` | 4 | 0 |
| `java/lang/module/Configuration` | 19 | 0 |
| `java/lang/module/FindException` | 4 | 0 |
| `java/lang/module/ModuleDescriptor` | 38 | 0 |
| `java/lang/module/ModuleDescriptor$Builder` | 27 | 0 |
| `java/lang/module/ModuleDescriptor$Exports` | 12 | 0 |
| `java/lang/module/ModuleDescriptor$Exports$Modifier` | 6 | 0 |
| `java/lang/module/ModuleDescriptor$Modifier` | 6 | 0 |
| `java/lang/module/ModuleDescriptor$Opens` | 12 | 0 |
| `java/lang/module/ModuleDescriptor$Opens$Modifier` | 6 | 0 |
| `java/lang/module/ModuleDescriptor$Provides` | 9 | 0 |
| `java/lang/module/ModuleDescriptor$Requires` | 13 | 0 |
| `java/lang/module/ModuleDescriptor$Requires$Modifier` | 6 | 0 |
| `java/lang/module/ModuleDescriptor$Version` | 11 | 0 |
| `java/lang/module/ModuleFinder` | 5 | 0 |
| `java/lang/module/ModuleFinder$1` | 3 | 0 |
| `java/lang/module/ModuleFinder$2` | 7 | 0 |
| `java/lang/module/ModuleReference` | 4 | 0 |
| `java/lang/module/ResolutionException` | 4 | 0 |
| `java/lang/module/ResolvedModule` | 9 | 0 |
| `java/lang/module/Resolver` | 40 | 0 |
| `java/lang/ref/Cleaner` | 5 | 0 |
| `java/lang/ref/Cleaner$Cleanable` | 1 | 0 |
| `java/lang/ref/FinalReference` | 4 | 0 |
| `java/lang/ref/Reference` | 22 | 5 |
| `java/lang/ref/Reference$ReferenceHandler` | 2 | 0 |
| `java/lang/ref/ReferenceQueue` | 16 | 0 |
| `java/lang/ref/SoftReference` | 3 | 0 |
| `java/lang/ref/WeakReference` | 2 | 0 |
| `java/lang/reflect/AccessFlag` | 10 | 0 |
| `java/lang/reflect/AccessibleObject` | 29 | 0 |
| `java/lang/reflect/AccessibleObject$Cache` | 3 | 0 |
| `java/lang/reflect/AnnotatedArrayType` | 2 | 0 |
| `java/lang/reflect/AnnotatedElement` | 8 | 0 |
| `java/lang/reflect/AnnotatedParameterizedType` | 2 | 0 |
| `java/lang/reflect/AnnotatedType` | 5 | 0 |
| `java/lang/reflect/AnnotatedTypeVariable` | 2 | 0 |
| `java/lang/reflect/AnnotatedWildcardType` | 3 | 0 |
| `java/lang/reflect/Array` | 24 | 21 |
| `java/lang/reflect/Constructor` | 45 | 0 |
| `java/lang/reflect/Executable` | 49 | 2 |
| `java/lang/reflect/Executable$ParameterData` | 6 | 0 |
| `java/lang/reflect/Field` | 54 | 1 |
| `java/lang/reflect/GenericArrayType` | 1 | 0 |
| `java/lang/reflect/GenericDeclaration` | 1 | 0 |
| `java/lang/reflect/GenericSignatureFormatError` | 2 | 0 |
| `java/lang/reflect/InaccessibleObjectException` | 2 | 0 |
| `java/lang/reflect/InvocationHandler` | 2 | 0 |
| `java/lang/reflect/InvocationTargetException` | 5 | 0 |
| `java/lang/reflect/MalformedParameterizedTypeException` | 2 | 0 |
| `java/lang/reflect/MalformedParametersException` | 2 | 0 |
| `java/lang/reflect/Member` | 5 | 0 |
| `java/lang/reflect/Method` | 50 | 0 |
| `java/lang/reflect/Modifier` | 22 | 0 |
| `java/lang/reflect/Parameter` | 23 | 0 |
| `java/lang/reflect/ParameterizedType` | 3 | 0 |
| `java/lang/reflect/Proxy` | 20 | 0 |
| `java/lang/reflect/Proxy$2` | 3 | 0 |
| `java/lang/reflect/Proxy$InvocationException` | 3 | 0 |
| `java/lang/reflect/Proxy$ProxyBuilder` | 23 | 0 |
| `java/lang/reflect/Proxy$ProxyBuilder$1` | 3 | 0 |
| `java/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext` | 7 | 0 |
| `java/lang/reflect/ProxyGenerator` | 18 | 0 |
| `java/lang/reflect/ProxyGenerator$1` | 3 | 0 |
| `java/lang/reflect/ProxyGenerator$PrimitiveTypeInfo` | 6 | 0 |
| `java/lang/reflect/ProxyGenerator$ProxyMethod` | 9 | 0 |
| `java/lang/reflect/RecordComponent` | 15 | 0 |
| `java/lang/reflect/ReflectPermission` | 2 | 0 |
| `java/lang/reflect/Type` | 1 | 0 |
| `java/lang/reflect/TypeVariable` | 4 | 0 |
| `java/lang/reflect/UndeclaredThrowableException` | 6 | 0 |
| `java/lang/reflect/WildcardType` | 2 | 0 |
| `java/math/BigDecimal` | 171 | 0 |
| `java/math/BigDecimal$LongOverflow` | 3 | 0 |
| `java/math/BigDecimal$StringBuilderHelper` | 5 | 0 |
| `java/math/BigDecimal$UnsafeHolder` | 4 | 0 |
| `java/math/BigInteger` | 159 | 0 |
| `java/math/BigInteger$RecursiveOp` | 7 | 0 |
| `java/math/BigInteger$RecursiveOp$RecursiveMultiply` | 3 | 0 |
| `java/math/BigInteger$RecursiveOp$RecursiveSquare` | 3 | 0 |
| `java/math/BigInteger$UnsafeHolder` | 3 | 0 |
| `java/math/BitSieve` | 10 | 0 |
| `java/math/MathContext` | 10 | 0 |
| `java/math/MutableBigInteger` | 83 | 0 |
| `java/math/RoundingMode` | 6 | 0 |
| `java/math/SignedMutableBigInteger` | 8 | 0 |
| `java/net/Authenticator` | 18 | 0 |
| `java/net/BindException` | 2 | 0 |
| `java/net/ContentHandler` | 3 | 0 |
| `java/net/ContentHandlerFactory` | 1 | 0 |
| `java/net/DelegatingSocketImpl` | 27 | 0 |
| `java/net/FileNameMap` | 1 | 0 |
| `java/net/HostPortrange` | 13 | 0 |
| `java/net/HttpConnectSocketImpl` | 14 | 0 |
| `java/net/HttpConnectSocketImpl$2` | 3 | 0 |
| `java/net/HttpURLConnection` | 21 | 0 |
| `java/net/IDN` | 16 | 0 |
| `java/net/Inet4Address` | 22 | 1 |
| `java/net/Inet4AddressImpl` | 9 | 4 |
| `java/net/Inet6Address` | 37 | 1 |
| `java/net/Inet6Address$Inet6AddressHolder` | 19 | 0 |
| `java/net/InetAddress` | 54 | 3 |
| `java/net/InetAddress$Addresses` | 1 | 0 |
| `java/net/InetAddress$CachedLocalHost` | 1 | 0 |
| `java/net/InetAddress$CachedLookup` | 6 | 0 |
| `java/net/InetAddress$HostsFileResolver` | 8 | 0 |
| `java/net/InetAddress$InetAddressHolder` | 7 | 0 |
| `java/net/InetAddress$NameServiceAddresses` | 2 | 0 |
| `java/net/InetAddress$PlatformResolver` | 3 | 0 |
| `java/net/InetAddress$ValidCachedLookup` | 3 | 0 |
| `java/net/InetAddressImpl` | 6 | 0 |
| `java/net/InetSocketAddress` | 19 | 0 |
| `java/net/InetSocketAddress$InetSocketAddressHolder` | 9 | 0 |
| `java/net/InterfaceAddress` | 7 | 0 |
| `java/net/JarURLConnection` | 10 | 0 |
| `java/net/MalformedURLException` | 2 | 0 |
| `java/net/NetPermission` | 2 | 0 |
| `java/net/NetworkInterface` | 44 | 12 |
| `java/net/NetworkInterface$1` | 3 | 0 |
| `java/net/PasswordAuthentication` | 3 | 0 |
| `java/net/ProtocolException` | 2 | 0 |
| `java/net/ProtocolFamily` | 1 | 0 |
| `java/net/Proxy` | 8 | 0 |
| `java/net/Proxy$Type` | 5 | 0 |
| `java/net/ProxySelector` | 7 | 0 |
| `java/net/ProxySelector$StaticProxySelector` | 4 | 0 |
| `java/net/Socket` | 72 | 0 |
| `java/net/Socket$SocketInputStream` | 5 | 0 |
| `java/net/Socket$SocketOutputStream` | 4 | 0 |
| `java/net/SocketAddress` | 1 | 0 |
| `java/net/SocketException` | 4 | 0 |
| `java/net/SocketImpl` | 29 | 0 |
| `java/net/SocketImplFactory` | 1 | 0 |
| `java/net/SocketOption` | 2 | 0 |
| `java/net/SocketPermission` | 31 | 0 |
| `java/net/SocketPermission$1` | 3 | 0 |
| `java/net/SocketPermissionCollection` | 8 | 0 |
| `java/net/SocketTimeoutException` | 2 | 0 |
| `java/net/SocksSocketImpl` | 20 | 0 |
| `java/net/SocksSocketImpl$1` | 3 | 0 |
| `java/net/SocksSocketImpl$2` | 3 | 0 |
| `java/net/SocksSocketImpl$3` | 3 | 0 |
| `java/net/URI` | 76 | 0 |
| `java/net/URI$Parser` | 25 | 0 |
| `java/net/URISyntaxException` | 6 | 0 |
| `java/net/URL` | 51 | 0 |
| `java/net/URL$1` | 5 | 0 |
| `java/net/URL$2` | 3 | 0 |
| `java/net/URLConnection` | 65 | 0 |
| `java/net/URLConnection$1` | 2 | 0 |
| `java/net/URLConnection$2` | 3 | 0 |
| `java/net/URLPermission` | 13 | 0 |
| `java/net/URLPermission$Authority` | 6 | 0 |
| `java/net/URLStreamHandler` | 13 | 0 |
| `java/net/URLStreamHandlerFactory` | 1 | 0 |
| `java/net/UnixDomainSocketAddress` | 10 | 0 |
| `java/net/UnixDomainSocketAddress$Ser` | 2 | 0 |
| `java/net/UnknownHostException` | 2 | 0 |
| `java/net/UnknownServiceException` | 2 | 0 |
| `java/net/UrlDeserializedState` | 9 | 0 |
| `java/net/spi/InetAddressResolver` | 2 | 0 |
| `java/net/spi/InetAddressResolver$LookupPolicy` | 3 | 0 |
| `java/net/spi/InetAddressResolverProvider` | 6 | 0 |
| `java/net/spi/URLStreamHandlerProvider` | 3 | 0 |
| `java/nio/Bits` | 12 | 0 |
| `java/nio/Buffer` | 38 | 0 |
| `java/nio/BufferMismatch` | 9 | 0 |
| `java/nio/BufferOverflowException` | 1 | 0 |
| `java/nio/BufferUnderflowException` | 1 | 0 |
| `java/nio/ByteBuffer` | 94 | 0 |
| `java/nio/ByteBufferAsCharBufferB` | 26 | 0 |
| `java/nio/ByteBufferAsCharBufferL` | 26 | 0 |
| `java/nio/ByteBufferAsCharBufferRB` | 21 | 0 |
| `java/nio/ByteBufferAsCharBufferRL` | 21 | 0 |
| `java/nio/ByteBufferAsDoubleBufferB` | 21 | 0 |
| `java/nio/ByteBufferAsDoubleBufferL` | 21 | 0 |
| `java/nio/ByteBufferAsDoubleBufferRB` | 16 | 0 |
| `java/nio/ByteBufferAsDoubleBufferRL` | 16 | 0 |
| `java/nio/ByteBufferAsFloatBufferB` | 21 | 0 |
| `java/nio/ByteBufferAsFloatBufferL` | 21 | 0 |
| `java/nio/ByteBufferAsFloatBufferRB` | 16 | 0 |
| `java/nio/ByteBufferAsFloatBufferRL` | 16 | 0 |
| `java/nio/ByteBufferAsIntBufferB` | 21 | 0 |
| `java/nio/ByteBufferAsIntBufferL` | 21 | 0 |
| `java/nio/ByteBufferAsIntBufferRB` | 16 | 0 |
| `java/nio/ByteBufferAsIntBufferRL` | 16 | 0 |
| `java/nio/ByteBufferAsLongBufferB` | 21 | 0 |
| `java/nio/ByteBufferAsLongBufferL` | 21 | 0 |
| `java/nio/ByteBufferAsLongBufferRB` | 16 | 0 |
| `java/nio/ByteBufferAsLongBufferRL` | 16 | 0 |
| `java/nio/ByteBufferAsShortBufferB` | 21 | 0 |
| `java/nio/ByteBufferAsShortBufferL` | 21 | 0 |
| `java/nio/ByteBufferAsShortBufferRB` | 16 | 0 |
| `java/nio/ByteBufferAsShortBufferRL` | 16 | 0 |
| `java/nio/ByteOrder` | 4 | 0 |
| `java/nio/CharBuffer` | 82 | 0 |
| `java/nio/CharBufferSpliterator` | 12 | 0 |
| `java/nio/DirectByteBuffer` | 73 | 0 |
| `java/nio/DirectByteBuffer$Deallocator` | 9 | 0 |
| `java/nio/DirectByteBufferR` | 45 | 0 |
| `java/nio/DirectCharBufferRS` | 24 | 0 |
| `java/nio/DirectCharBufferRU` | 24 | 0 |
| `java/nio/DirectCharBufferS` | 32 | 0 |
| `java/nio/DirectCharBufferU` | 32 | 0 |
| `java/nio/DirectDoubleBufferRS` | 16 | 0 |
| `java/nio/DirectDoubleBufferRU` | 16 | 0 |
| `java/nio/DirectDoubleBufferS` | 22 | 0 |
| `java/nio/DirectDoubleBufferU` | 22 | 0 |
| `java/nio/DirectFloatBufferRS` | 16 | 0 |
| `java/nio/DirectFloatBufferRU` | 16 | 0 |
| `java/nio/DirectFloatBufferS` | 22 | 0 |
| `java/nio/DirectFloatBufferU` | 22 | 0 |
| `java/nio/DirectIntBufferRS` | 16 | 0 |
| `java/nio/DirectIntBufferRU` | 16 | 0 |
| `java/nio/DirectIntBufferS` | 22 | 0 |
| `java/nio/DirectIntBufferU` | 22 | 0 |
| `java/nio/DirectLongBufferRS` | 16 | 0 |
| `java/nio/DirectLongBufferRU` | 16 | 0 |
| `java/nio/DirectLongBufferS` | 22 | 0 |
| `java/nio/DirectLongBufferU` | 22 | 0 |
| `java/nio/DirectShortBufferRS` | 16 | 0 |
| `java/nio/DirectShortBufferRU` | 16 | 0 |
| `java/nio/DirectShortBufferS` | 22 | 0 |
| `java/nio/DirectShortBufferU` | 22 | 0 |
| `java/nio/DoubleBuffer` | 60 | 0 |
| `java/nio/FloatBuffer` | 60 | 0 |
| `java/nio/HeapByteBuffer` | 58 | 0 |
| `java/nio/HeapByteBufferR` | 38 | 0 |
| `java/nio/HeapCharBuffer` | 37 | 0 |
| `java/nio/HeapCharBufferR` | 29 | 0 |
| `java/nio/HeapDoubleBuffer` | 26 | 0 |
| `java/nio/HeapDoubleBufferR` | 19 | 0 |
| `java/nio/HeapFloatBuffer` | 26 | 0 |
| `java/nio/HeapFloatBufferR` | 19 | 0 |
| `java/nio/HeapIntBuffer` | 26 | 0 |
| `java/nio/HeapIntBufferR` | 19 | 0 |
| `java/nio/HeapLongBuffer` | 26 | 0 |
| `java/nio/HeapLongBufferR` | 19 | 0 |
| `java/nio/HeapShortBuffer` | 26 | 0 |
| `java/nio/HeapShortBufferR` | 19 | 0 |
| `java/nio/IntBuffer` | 60 | 0 |
| `java/nio/InvalidMarkException` | 1 | 0 |
| `java/nio/LongBuffer` | 60 | 0 |
| `java/nio/MappedByteBuffer` | 43 | 0 |
| `java/nio/MappedByteBuffer$1` | 5 | 0 |
| `java/nio/ReadOnlyBufferException` | 1 | 0 |
| `java/nio/ShortBuffer` | 60 | 0 |
| `java/nio/StringCharBuffer` | 26 | 0 |
| `java/nio/channels/AlreadyBoundException` | 1 | 0 |
| `java/nio/channels/AlreadyConnectedException` | 1 | 0 |
| `java/nio/channels/AsynchronousByteChannel` | 4 | 0 |
| `java/nio/channels/AsynchronousChannel` | 1 | 0 |
| `java/nio/channels/AsynchronousChannelGroup` | 10 | 0 |
| `java/nio/channels/AsynchronousCloseException` | 1 | 0 |
| `java/nio/channels/AsynchronousFileChannel` | 17 | 0 |
| `java/nio/channels/CancelledKeyException` | 1 | 0 |
| `java/nio/channels/Channel` | 2 | 0 |
| `java/nio/channels/Channels` | 13 | 0 |
| `java/nio/channels/Channels$1` | 4 | 0 |
| `java/nio/channels/Channels$2` | 4 | 0 |
| `java/nio/channels/Channels$ReadableByteChannelImpl` | 3 | 0 |
| `java/nio/channels/Channels$WritableByteChannelImpl` | 3 | 0 |
| `java/nio/channels/ClosedByInterruptException` | 1 | 0 |
| `java/nio/channels/ClosedChannelException` | 1 | 0 |
| `java/nio/channels/ClosedSelectorException` | 1 | 0 |
| `java/nio/channels/CompletionHandler` | 2 | 0 |
| `java/nio/channels/ConnectionPendingException` | 1 | 0 |
| `java/nio/channels/FileChannel` | 27 | 0 |
| `java/nio/channels/FileChannel$MapMode` | 3 | 0 |
| `java/nio/channels/FileLock` | 12 | 0 |
| `java/nio/channels/FileLockInterruptionException` | 1 | 0 |
| `java/nio/channels/IllegalBlockingModeException` | 1 | 0 |
| `java/nio/channels/IllegalSelectorException` | 1 | 0 |
| `java/nio/channels/NoConnectionPendingException` | 1 | 0 |
| `java/nio/channels/NonReadableChannelException` | 1 | 0 |
| `java/nio/channels/NonWritableChannelException` | 1 | 0 |
| `java/nio/channels/NotYetBoundException` | 1 | 0 |
| `java/nio/channels/NotYetConnectedException` | 1 | 0 |
| `java/nio/channels/OverlappingFileLockException` | 1 | 0 |
| `java/nio/channels/Pipe$SinkChannel` | 2 | 0 |
| `java/nio/channels/ReadableByteChannel` | 1 | 0 |
| `java/nio/channels/SeekableByteChannel` | 6 | 0 |
| `java/nio/channels/SelectableChannel` | 10 | 0 |
| `java/nio/channels/SelectionKey` | 17 | 0 |
| `java/nio/channels/Selector` | 17 | 0 |
| `java/nio/channels/ShutdownChannelGroupException` | 1 | 0 |
| `java/nio/channels/SocketChannel` | 25 | 0 |
| `java/nio/channels/UnresolvedAddressException` | 1 | 0 |
| `java/nio/channels/UnsupportedAddressTypeException` | 1 | 0 |
| `java/nio/channels/WritableByteChannel` | 1 | 0 |
| `java/nio/channels/spi/AbstractInterruptibleChannel` | 7 | 0 |
| `java/nio/channels/spi/AbstractInterruptibleChannel$1` | 2 | 0 |
| `java/nio/channels/spi/AbstractSelectableChannel` | 18 | 0 |
| `java/nio/channels/spi/AbstractSelectionKey` | 5 | 0 |
| `java/nio/channels/spi/AbstractSelector` | 12 | 0 |
| `java/nio/channels/spi/AbstractSelector$1` | 2 | 0 |
| `java/nio/channels/spi/AsynchronousChannelProvider` | 8 | 0 |
| `java/nio/channels/spi/SelectorProvider` | 13 | 0 |
| `java/nio/charset/CharacterCodingException` | 1 | 0 |
| `java/nio/charset/Charset` | 34 | 0 |
| `java/nio/charset/Charset$1` | 6 | 0 |
| `java/nio/charset/Charset$2` | 3 | 0 |
| `java/nio/charset/Charset$3` | 3 | 0 |
| `java/nio/charset/CharsetDecoder` | 26 | 0 |
| `java/nio/charset/CharsetEncoder` | 27 | 0 |
| `java/nio/charset/CoderMalfunctionError` | 1 | 0 |
| `java/nio/charset/CoderResult` | 14 | 0 |
| `java/nio/charset/IllegalCharsetNameException` | 2 | 0 |
| `java/nio/charset/MalformedInputException` | 3 | 0 |
| `java/nio/charset/UnmappableCharacterException` | 3 | 0 |
| `java/nio/charset/UnsupportedCharsetException` | 2 | 0 |
| `java/nio/charset/spi/CharsetProvider` | 5 | 0 |
| `java/nio/file/AccessDeniedException` | 2 | 0 |
| `java/nio/file/AccessMode` | 5 | 0 |
| `java/nio/file/AtomicMoveNotSupportedException` | 1 | 0 |
| `java/nio/file/ClosedDirectoryStreamException` | 1 | 0 |
| `java/nio/file/ClosedWatchServiceException` | 1 | 0 |
| `java/nio/file/CopyMoveHelper` | 5 | 0 |
| `java/nio/file/CopyMoveHelper$CopyOptions` | 2 | 0 |
| `java/nio/file/CopyOption` | 0 | 0 |
| `java/nio/file/DirectoryIteratorException` | 4 | 0 |
| `java/nio/file/DirectoryNotEmptyException` | 1 | 0 |
| `java/nio/file/DirectoryStream` | 1 | 0 |
| `java/nio/file/DirectoryStream$Filter` | 1 | 0 |
| `java/nio/file/FileAlreadyExistsException` | 2 | 0 |
| `java/nio/file/FileChannelLinesSpliterator` | 14 | 0 |
| `java/nio/file/FileChannelLinesSpliterator$1` | 4 | 0 |
| `java/nio/file/FileStore` | 12 | 0 |
| `java/nio/file/FileSystem` | 13 | 0 |
| `java/nio/file/FileSystemAlreadyExistsException` | 2 | 0 |
| `java/nio/file/FileSystemException` | 6 | 0 |
| `java/nio/file/FileSystemLoopException` | 1 | 0 |
| `java/nio/file/FileSystemNotFoundException` | 2 | 0 |
| `java/nio/file/FileSystems` | 9 | 0 |
| `java/nio/file/FileTreeIterator` | 7 | 0 |
| `java/nio/file/FileTreeWalker` | 11 | 0 |
| `java/nio/file/FileTreeWalker$DirectoryNode` | 7 | 0 |
| `java/nio/file/FileTreeWalker$Event` | 7 | 0 |
| `java/nio/file/FileTreeWalker$EventType` | 5 | 0 |
| `java/nio/file/FileVisitOption` | 5 | 0 |
| `java/nio/file/FileVisitor` | 4 | 0 |
| `java/nio/file/Files` | 85 | 0 |
| `java/nio/file/Files$1` | 3 | 0 |
| `java/nio/file/Files$2` | 4 | 0 |
| `java/nio/file/InvalidPathException` | 6 | 0 |
| `java/nio/file/LinkOption` | 5 | 0 |
| `java/nio/file/LinkPermission` | 3 | 0 |
| `java/nio/file/NoSuchFileException` | 2 | 0 |
| `java/nio/file/NotDirectoryException` | 1 | 0 |
| `java/nio/file/NotLinkException` | 2 | 0 |
| `java/nio/file/OpenOption` | 0 | 0 |
| `java/nio/file/Path` | 32 | 0 |
| `java/nio/file/Path$1` | 4 | 0 |
| `java/nio/file/PathMatcher` | 1 | 0 |
| `java/nio/file/Paths` | 3 | 0 |
| `java/nio/file/ProviderMismatchException` | 2 | 0 |
| `java/nio/file/ProviderNotFoundException` | 2 | 0 |
| `java/nio/file/StandardOpenOption` | 5 | 0 |
| `java/nio/file/TempFileHelper` | 6 | 0 |
| `java/nio/file/WatchEvent` | 3 | 0 |
| `java/nio/file/WatchEvent$Kind` | 2 | 0 |
| `java/nio/file/WatchEvent$Modifier` | 1 | 0 |
| `java/nio/file/WatchKey` | 5 | 0 |
| `java/nio/file/attribute/AclFileAttributeView` | 3 | 0 |
| `java/nio/file/attribute/BasicFileAttributeView` | 3 | 0 |
| `java/nio/file/attribute/BasicFileAttributes` | 9 | 0 |
| `java/nio/file/attribute/FileAttribute` | 2 | 0 |
| `java/nio/file/attribute/FileAttributeView` | 0 | 0 |
| `java/nio/file/attribute/FileOwnerAttributeView` | 3 | 0 |
| `java/nio/file/attribute/FileStoreAttributeView` | 0 | 0 |
| `java/nio/file/attribute/FileTime` | 16 | 0 |
| `java/nio/file/attribute/GroupPrincipal` | 0 | 0 |
| `java/nio/file/attribute/PosixFileAttributeView` | 5 | 0 |
| `java/nio/file/attribute/PosixFileAttributes` | 3 | 0 |
| `java/nio/file/attribute/PosixFilePermission` | 5 | 0 |
| `java/nio/file/attribute/UserPrincipal` | 0 | 0 |
| `java/nio/file/attribute/UserPrincipalNotFoundException` | 2 | 0 |
| `java/nio/file/spi/FileSystemProvider` | 36 | 0 |
| `java/nio/file/spi/FileSystemProvider$1` | 3 | 0 |
| `java/nio/file/spi/FileTypeDetector` | 4 | 0 |
| `java/security/AccessControlContext` | 27 | 0 |
| `java/security/AccessControlContext$1` | 3 | 0 |
| `java/security/AccessControlException` | 3 | 0 |
| `java/security/AccessController` | 26 | 4 |
| `java/security/AlgorithmParameters` | 13 | 0 |
| `java/security/AlgorithmParametersSpi` | 8 | 0 |
| `java/security/AllPermission` | 7 | 0 |
| `java/security/AllPermissionCollection` | 4 | 0 |
| `java/security/AllPermissionCollection$1` | 4 | 0 |
| `java/security/BasicPermission` | 10 | 0 |
| `java/security/BasicPermissionCollection` | 7 | 0 |
| `java/security/CodeSigner` | 7 | 0 |
| `java/security/CodeSource` | 15 | 0 |
| `java/security/DigestException` | 4 | 0 |
| `java/security/DomainCombiner` | 1 | 0 |
| `java/security/GeneralSecurityException` | 4 | 0 |
| `java/security/InvalidAlgorithmParameterException` | 4 | 0 |
| `java/security/InvalidKeyException` | 4 | 0 |
| `java/security/InvalidParameterException` | 4 | 0 |
| `java/security/Key` | 3 | 0 |
| `java/security/KeyException` | 4 | 0 |
| `java/security/KeyFactory` | 13 | 0 |
| `java/security/KeyFactorySpi` | 5 | 0 |
| `java/security/KeyStore` | 35 | 0 |
| `java/security/KeyStore$CallbackHandlerProtection` | 2 | 0 |
| `java/security/KeyStore$Entry` | 1 | 0 |
| `java/security/KeyStore$LoadStoreParameter` | 1 | 0 |
| `java/security/KeyStore$PasswordProtection` | 7 | 0 |
| `java/security/KeyStore$PrivateKeyEntry` | 7 | 0 |
| `java/security/KeyStore$SecretKeyEntry` | 5 | 0 |
| `java/security/KeyStore$TrustedCertificateEntry` | 5 | 0 |
| `java/security/KeyStoreException` | 4 | 0 |
| `java/security/KeyStoreSpi` | 25 | 0 |
| `java/security/MessageDigest` | 21 | 0 |
| `java/security/MessageDigest$Delegate` | 11 | 0 |
| `java/security/MessageDigest$Delegate$CloneableDelegate` | 1 | 0 |
| `java/security/MessageDigestSpi` | 9 | 0 |
| `java/security/NoSuchAlgorithmException` | 4 | 0 |
| `java/security/NoSuchProviderException` | 2 | 0 |
| `java/security/Permission` | 9 | 0 |
| `java/security/PermissionCollection` | 8 | 0 |
| `java/security/Permissions` | 10 | 0 |
| `java/security/PermissionsEnumerator` | 5 | 0 |
| `java/security/PermissionsHash` | 7 | 0 |
| `java/security/Policy` | 21 | 0 |
| `java/security/Policy$1` | 3 | 0 |
| `java/security/Policy$2` | 3 | 0 |
| `java/security/Policy$3` | 3 | 0 |
| `java/security/Policy$PolicyDelegate` | 8 | 0 |
| `java/security/Policy$PolicyInfo` | 1 | 0 |
| `java/security/PolicySpi` | 5 | 0 |
| `java/security/Principal` | 5 | 0 |
| `java/security/PrivateKey` | 0 | 0 |
| `java/security/PrivilegedAction` | 1 | 0 |
| `java/security/PrivilegedActionException` | 6 | 0 |
| `java/security/PrivilegedExceptionAction` | 1 | 0 |
| `java/security/ProtectionDomain` | 14 | 0 |
| `java/security/ProtectionDomain$Key` | 1 | 0 |
| `java/security/Provider` | 66 | 0 |
| `java/security/Provider$EngineDescription` | 1 | 0 |
| `java/security/Provider$OPType` | 5 | 0 |
| `java/security/Provider$Service` | 25 | 0 |
| `java/security/Provider$ServiceKey` | 5 | 0 |
| `java/security/Provider$UString` | 4 | 0 |
| `java/security/ProviderException` | 4 | 0 |
| `java/security/PublicKey` | 0 | 0 |
| `java/security/SecureRandom` | 30 | 0 |
| `java/security/SecureRandomParameters` | 0 | 0 |
| `java/security/SecureRandomSpi` | 9 | 0 |
| `java/security/Security` | 29 | 0 |
| `java/security/Security$Criteria` | 3 | 0 |
| `java/security/Security$ProviderProperty` | 1 | 0 |
| `java/security/SecurityPermission` | 2 | 0 |
| `java/security/Signature` | 33 | 0 |
| `java/security/Signature$Delegate` | 25 | 0 |
| `java/security/Signature$Delegate$CloneableDelegate` | 1 | 0 |
| `java/security/SignatureException` | 4 | 0 |
| `java/security/SignatureSpi` | 18 | 0 |
| `java/security/Timestamp` | 8 | 0 |
| `java/security/UnrecoverableEntryException` | 2 | 0 |
| `java/security/UnrecoverableKeyException` | 2 | 0 |
| `java/security/UnresolvedPermission` | 15 | 0 |
| `java/security/UnresolvedPermissionCollection` | 9 | 0 |
| `java/security/cert/CRL` | 4 | 0 |
| `java/security/cert/CRLException` | 4 | 0 |
| `java/security/cert/CertPath` | 10 | 0 |
| `java/security/cert/CertPath$CertPathRep` | 2 | 0 |
| `java/security/cert/CertPathValidatorException` | 10 | 0 |
| `java/security/cert/Certificate` | 11 | 0 |
| `java/security/cert/Certificate$CertificateRep` | 2 | 0 |
| `java/security/cert/CertificateEncodingException` | 4 | 0 |
| `java/security/cert/CertificateException` | 4 | 0 |
| `java/security/cert/CertificateExpiredException` | 2 | 0 |
| `java/security/cert/CertificateFactory` | 14 | 0 |
| `java/security/cert/CertificateFactorySpi` | 9 | 0 |
| `java/security/cert/CertificateNotYetValidException` | 2 | 0 |
| `java/security/cert/CertificateParsingException` | 4 | 0 |
| `java/security/cert/Extension` | 4 | 0 |
| `java/security/cert/PolicyQualifierInfo` | 5 | 0 |
| `java/security/cert/X509CRL` | 20 | 0 |
| `java/security/cert/X509CRLEntry` | 10 | 0 |
| `java/security/cert/X509Certificate` | 24 | 0 |
| `java/security/interfaces/DSAKey` | 1 | 0 |
| `java/security/interfaces/DSAParams` | 3 | 0 |
| `java/security/interfaces/DSAPublicKey` | 1 | 0 |
| `java/security/interfaces/ECKey` | 1 | 0 |
| `java/security/interfaces/ECPrivateKey` | 1 | 0 |
| `java/security/interfaces/ECPublicKey` | 1 | 0 |
| `java/security/interfaces/EdECKey` | 1 | 0 |
| `java/security/interfaces/EdECPrivateKey` | 1 | 0 |
| `java/security/interfaces/RSAKey` | 2 | 0 |
| `java/security/interfaces/XECKey` | 1 | 0 |
| `java/security/spec/AlgorithmParameterSpec` | 0 | 0 |
| `java/security/spec/ECField` | 1 | 0 |
| `java/security/spec/ECFieldF2m` | 9 | 0 |
| `java/security/spec/ECFieldFp` | 5 | 0 |
| `java/security/spec/ECGenParameterSpec` | 1 | 0 |
| `java/security/spec/ECParameterSpec` | 5 | 0 |
| `java/security/spec/ECPoint` | 7 | 0 |
| `java/security/spec/ECPrivateKeySpec` | 3 | 0 |
| `java/security/spec/ECPublicKeySpec` | 3 | 0 |
| `java/security/spec/EllipticCurve` | 9 | 0 |
| `java/security/spec/EncodedKeySpec` | 7 | 0 |
| `java/security/spec/InvalidKeySpecException` | 4 | 0 |
| `java/security/spec/InvalidParameterSpecException` | 2 | 0 |
| `java/security/spec/MGF1ParameterSpec` | 4 | 0 |
| `java/security/spec/NamedParameterSpec` | 3 | 0 |
| `java/security/spec/PKCS8EncodedKeySpec` | 4 | 0 |
| `java/security/spec/PSSParameterSpec` | 9 | 0 |
| `java/security/spec/X509EncodedKeySpec` | 4 | 0 |
| `java/text/Annotation` | 3 | 0 |
| `java/text/AttributeEntry` | 8 | 0 |
| `java/text/AttributedCharacterIterator` | 9 | 0 |
| `java/text/AttributedCharacterIterator$Attribute` | 7 | 0 |
| `java/text/AttributedString` | 26 | 0 |
| `java/text/AttributedString$AttributeMap` | 3 | 0 |
| `java/text/AttributedString$AttributedStringIterator` | 25 | 0 |
| `java/text/BreakIterator` | 27 | 0 |
| `java/text/BreakIterator$BreakIteratorCache` | 3 | 0 |
| `java/text/CalendarBuilder` | 10 | 0 |
| `java/text/CharacterIterator` | 10 | 0 |
| `java/text/CharacterIteratorFieldDelegate` | 4 | 0 |
| `java/text/ChoiceFormat` | 19 | 0 |
| `java/text/CollationElementIterator` | 22 | 0 |
| `java/text/CollationKey` | 5 | 0 |
| `java/text/Collator` | 16 | 0 |
| `java/text/CompactNumberFormat` | 69 | 0 |
| `java/text/CompactNumberFormat$Patterns` | 5 | 0 |
| `java/text/DateFormat` | 31 | 0 |
| `java/text/DateFormatSymbols` | 36 | 0 |
| `java/text/DecimalFormat` | 87 | 0 |
| `java/text/DecimalFormat$FastPathData` | 1 | 0 |
| `java/text/DecimalFormatSymbols` | 53 | 0 |
| `java/text/DigitList` | 28 | 0 |
| `java/text/EntryPair` | 2 | 0 |
| `java/text/FieldPosition` | 15 | 0 |
| `java/text/FieldPosition$Delegate` | 3 | 0 |
| `java/text/Format` | 11 | 0 |
| `java/text/Format$Field` | 1 | 0 |
| `java/text/Format$FieldDelegate` | 2 | 0 |
| `java/text/MergeCollation` | 12 | 0 |
| `java/text/MessageFormat` | 30 | 0 |
| `java/text/MessageFormat$Field` | 3 | 0 |
| `java/text/Normalizer` | 3 | 0 |
| `java/text/NumberFormat` | 48 | 0 |
| `java/text/ParseException` | 2 | 0 |
| `java/text/ParsePosition` | 8 | 0 |
| `java/text/PatternEntry` | 12 | 0 |
| `java/text/PatternEntry$Parser` | 2 | 0 |
| `java/text/RBCollationTables` | 14 | 0 |
| `java/text/RBCollationTables$BuildAPI` | 2 | 0 |
| `java/text/RBTableBuilder` | 16 | 0 |
| `java/text/RuleBasedCollationKey` | 6 | 0 |
| `java/text/RuleBasedCollator` | 12 | 0 |
| `java/text/SimpleDateFormat` | 44 | 0 |
| `java/text/StringCharacterIterator` | 16 | 0 |
| `java/text/spi/BreakIteratorProvider` | 5 | 0 |
| `java/text/spi/CollatorProvider` | 2 | 0 |
| `java/text/spi/DateFormatProvider` | 4 | 0 |
| `java/text/spi/DateFormatSymbolsProvider` | 2 | 0 |
| `java/text/spi/DecimalFormatSymbolsProvider` | 2 | 0 |
| `java/text/spi/NumberFormatProvider` | 6 | 0 |
| `java/time/Clock` | 18 | 0 |
| `java/time/Clock$FixedClock` | 8 | 0 |
| `java/time/Clock$OffsetClock` | 8 | 0 |
| `java/time/Clock$SystemClock` | 9 | 0 |
| `java/time/Clock$TickClock` | 8 | 0 |
| `java/time/DateTimeException` | 2 | 0 |
| `java/time/DayOfWeek` | 17 | 0 |
| `java/time/Duration` | 75 | 0 |
| `java/time/Instant` | 58 | 0 |
| `java/time/LocalDate` | 100 | 0 |
| `java/time/LocalDateTime` | 100 | 0 |
| `java/time/LocalTime` | 70 | 0 |
| `java/time/Month` | 22 | 0 |
| `java/time/MonthDay` | 36 | 0 |
| `java/time/OffsetDateTime` | 95 | 0 |
| `java/time/OffsetTime` | 69 | 0 |
| `java/time/Period` | 52 | 0 |
| `java/time/Ser` | 8 | 0 |
| `java/time/Year` | 52 | 0 |
| `java/time/YearMonth` | 59 | 0 |
| `java/time/ZoneId` | 22 | 0 |
| `java/time/ZoneId$1` | 4 | 0 |
| `java/time/ZoneOffset` | 33 | 0 |
| `java/time/ZoneRegion` | 11 | 0 |
| `java/time/ZonedDateTime` | 104 | 0 |
| `java/time/chrono/AbstractChronology` | 29 | 0 |
| `java/time/chrono/ChronoLocalDate` | 38 | 0 |
| `java/time/chrono/ChronoLocalDateImpl` | 28 | 0 |
| `java/time/chrono/ChronoLocalDateTime` | 35 | 0 |
| `java/time/chrono/ChronoLocalDateTimeImpl` | 34 | 0 |
| `java/time/chrono/ChronoPeriod` | 16 | 0 |
| `java/time/chrono/ChronoPeriodImpl` | 23 | 0 |
| `java/time/chrono/ChronoZonedDateTime` | 43 | 0 |
| `java/time/chrono/ChronoZonedDateTimeImpl` | 25 | 0 |
| `java/time/chrono/Chronology` | 34 | 0 |
| `java/time/chrono/Chronology$1` | 4 | 0 |
| `java/time/chrono/Era` | 8 | 0 |
| `java/time/chrono/HijrahChronology` | 71 | 0 |
| `java/time/chrono/HijrahDate` | 69 | 0 |
| `java/time/chrono/HijrahEra` | 9 | 0 |
| `java/time/chrono/IsoChronology` | 48 | 0 |
| `java/time/chrono/IsoEra` | 7 | 0 |
| `java/time/chrono/JapaneseChronology` | 41 | 0 |
| `java/time/chrono/JapaneseDate` | 68 | 0 |
| `java/time/chrono/JapaneseEra` | 20 | 0 |
| `java/time/chrono/MinguoChronology` | 36 | 0 |
| `java/time/chrono/MinguoDate` | 61 | 0 |
| `java/time/chrono/MinguoEra` | 8 | 0 |
| `java/time/chrono/Ser` | 8 | 0 |
| `java/time/chrono/ThaiBuddhistChronology` | 36 | 0 |
| `java/time/chrono/ThaiBuddhistDate` | 61 | 0 |
| `java/time/chrono/ThaiBuddhistEra` | 8 | 0 |
| `java/time/format/DateTimeFormatter` | 41 | 0 |
| `java/time/format/DateTimeFormatter$ClassicFormat` | 4 | 0 |
| `java/time/format/DateTimeFormatterBuilder` | 55 | 0 |
| `java/time/format/DateTimeFormatterBuilder$1` | 5 | 0 |
| `java/time/format/DateTimeFormatterBuilder$CharLiteralPrinterParser` | 4 | 0 |
| `java/time/format/DateTimeFormatterBuilder$ChronoPrinterParser` | 5 | 0 |
| `java/time/format/DateTimeFormatterBuilder$CompositePrinterParser` | 6 | 0 |
| `java/time/format/DateTimeFormatterBuilder$DateTimePrinterParser` | 2 | 0 |
| `java/time/format/DateTimeFormatterBuilder$DayPeriod` | 17 | 0 |
| `java/time/format/DateTimeFormatterBuilder$DayPeriodPrinterParser` | 9 | 0 |
| `java/time/format/DateTimeFormatterBuilder$DefaultValueParser` | 3 | 0 |
| `java/time/format/DateTimeFormatterBuilder$FractionPrinterParser` | 12 | 0 |
| `java/time/format/DateTimeFormatterBuilder$InstantPrinterParser` | 4 | 0 |
| `java/time/format/DateTimeFormatterBuilder$LocalizedOffsetIdPrinterParser` | 6 | 0 |
| `java/time/format/DateTimeFormatterBuilder$LocalizedPrinterParser` | 8 | 0 |
| `java/time/format/DateTimeFormatterBuilder$NanosPrinterParser` | 12 | 0 |
| `java/time/format/DateTimeFormatterBuilder$NumberPrinterParser` | 12 | 0 |
| `java/time/format/DateTimeFormatterBuilder$OffsetIdPrinterParser` | 15 | 0 |
| `java/time/format/DateTimeFormatterBuilder$PadPrinterParserDecorator` | 4 | 0 |
| `java/time/format/DateTimeFormatterBuilder$PrefixTree` | 13 | 0 |
| `java/time/format/DateTimeFormatterBuilder$PrefixTree$CI` | 5 | 0 |
| `java/time/format/DateTimeFormatterBuilder$ReducedPrinterParser` | 12 | 0 |
| `java/time/format/DateTimeFormatterBuilder$StringLiteralPrinterParser` | 4 | 0 |
| `java/time/format/DateTimeFormatterBuilder$TextPrinterParser` | 5 | 0 |
| `java/time/format/DateTimeFormatterBuilder$WeekBasedFieldPrinterParser` | 10 | 0 |
| `java/time/format/DateTimeFormatterBuilder$ZoneIdPrinterParser` | 6 | 0 |
| `java/time/format/DateTimeFormatterBuilder$ZoneTextPrinterParser` | 7 | 0 |
| `java/time/format/DateTimeParseContext` | 26 | 0 |
| `java/time/format/DateTimeParseException` | 4 | 0 |
| `java/time/format/DateTimePrintContext` | 10 | 0 |
| `java/time/format/DateTimePrintContext$1` | 6 | 0 |
| `java/time/format/DateTimeTextProvider` | 12 | 0 |
| `java/time/format/DateTimeTextProvider$LocaleStore` | 3 | 0 |
| `java/time/format/DecimalStyle` | 19 | 0 |
| `java/time/format/FormatStyle` | 5 | 0 |
| `java/time/format/Parsed` | 22 | 0 |
| `java/time/format/ResolverStyle` | 5 | 0 |
| `java/time/format/SignStyle` | 6 | 0 |
| `java/time/format/TextStyle` | 10 | 0 |
| `java/time/format/ZoneName` | 4 | 0 |
| `java/time/temporal/ChronoField` | 19 | 0 |
| `java/time/temporal/ChronoUnit` | 13 | 0 |
| `java/time/temporal/Temporal` | 8 | 0 |
| `java/time/temporal/TemporalAccessor` | 5 | 0 |
| `java/time/temporal/TemporalAdjuster` | 1 | 0 |
| `java/time/temporal/TemporalAdjusters` | 28 | 0 |
| `java/time/temporal/TemporalAmount` | 4 | 0 |
| `java/time/temporal/TemporalField` | 12 | 0 |
| `java/time/temporal/TemporalQueries` | 9 | 0 |
| `java/time/temporal/TemporalQuery` | 1 | 0 |
| `java/time/temporal/TemporalUnit` | 8 | 0 |
| `java/time/temporal/UnsupportedTemporalTypeException` | 2 | 0 |
| `java/time/temporal/ValueRange` | 19 | 0 |
| `java/time/temporal/WeekFields` | 16 | 0 |
| `java/time/temporal/WeekFields$ComputedDayOfField` | 34 | 0 |
| `java/time/zone/Ser` | 13 | 0 |
| `java/time/zone/ZoneOffsetTransition` | 25 | 0 |
| `java/time/zone/ZoneOffsetTransitionRule` | 20 | 0 |
| `java/time/zone/ZoneOffsetTransitionRule$TimeDefinition` | 6 | 0 |
| `java/time/zone/ZoneRules` | 30 | 0 |
| `java/time/zone/ZoneRulesException` | 2 | 0 |
| `java/time/zone/ZoneRulesProvider` | 14 | 0 |
| `java/util/AbstractCollection` | 16 | 0 |
| `java/util/AbstractList` | 20 | 0 |
| `java/util/AbstractList$Itr` | 5 | 0 |
| `java/util/AbstractList$ListItr` | 7 | 0 |
| `java/util/AbstractList$RandomAccessSpliterator` | 11 | 0 |
| `java/util/AbstractList$RandomAccessSubList` | 3 | 0 |
| `java/util/AbstractList$SubList` | 17 | 0 |
| `java/util/AbstractList$SubList$1` | 10 | 0 |
| `java/util/AbstractMap` | 18 | 0 |
| `java/util/AbstractMap$1` | 6 | 0 |
| `java/util/AbstractMap$1$1` | 4 | 0 |
| `java/util/AbstractMap$2` | 6 | 0 |
| `java/util/AbstractMap$2$1` | 4 | 0 |
| `java/util/AbstractMap$SimpleEntry` | 8 | 0 |
| `java/util/AbstractMap$SimpleImmutableEntry` | 8 | 0 |
| `java/util/AbstractMap$ViewCollection` | 23 | 0 |
| `java/util/AbstractQueue` | 6 | 0 |
| `java/util/AbstractSequentialList` | 8 | 0 |
| `java/util/AbstractSet` | 4 | 0 |
| `java/util/ArrayDeque` | 64 | 0 |
| `java/util/ArrayDeque$DeqIterator` | 6 | 0 |
| `java/util/ArrayDeque$DeqSpliterator` | 9 | 0 |
| `java/util/ArrayDeque$DescendingIterator` | 4 | 0 |
| `java/util/ArrayList` | 68 | 0 |
| `java/util/ArrayList$ArrayListSpliterator` | 8 | 0 |
| `java/util/ArrayList$Itr` | 6 | 0 |
| `java/util/ArrayList$ListItr` | 7 | 0 |
| `java/util/ArrayList$SubList` | 30 | 0 |
| `java/util/ArrayList$SubList$1` | 12 | 0 |
| `java/util/ArrayList$SubList$2` | 8 | 0 |
| `java/util/ArrayPrefixHelpers$CumulateTask` | 3 | 0 |
| `java/util/ArrayPrefixHelpers$DoubleCumulateTask` | 3 | 0 |
| `java/util/ArrayPrefixHelpers$IntCumulateTask` | 3 | 0 |
| `java/util/ArrayPrefixHelpers$LongCumulateTask` | 3 | 0 |
| `java/util/Arrays` | 249 | 0 |
| `java/util/Arrays$ArrayItr` | 3 | 0 |
| `java/util/Arrays$ArrayList` | 13 | 0 |
| `java/util/ArraysParallelSortHelpers$EmptyCompleter` | 2 | 0 |
| `java/util/ArraysParallelSortHelpers$FJObject$Merger` | 2 | 0 |
| `java/util/ArraysParallelSortHelpers$FJObject$Sorter` | 2 | 0 |
| `java/util/ArraysParallelSortHelpers$Relay` | 3 | 0 |
| `java/util/Base64` | 8 | 0 |
| `java/util/Base64$DecInputStream` | 8 | 0 |
| `java/util/Base64$Decoder` | 10 | 0 |
| `java/util/Base64$EncOutputStream` | 6 | 0 |
| `java/util/Base64$Encoder` | 11 | 0 |
| `java/util/BitSet` | 50 | 0 |
| `java/util/BitSet$1BitSetSpliterator` | 12 | 0 |
| `java/util/Calendar` | 91 | 0 |
| `java/util/Calendar$1` | 3 | 0 |
| `java/util/Calendar$Builder` | 20 | 0 |
| `java/util/CollSer` | 4 | 0 |
| `java/util/Collection` | 20 | 0 |
| `java/util/Collections` | 88 | 0 |
| `java/util/Collections$1` | 5 | 0 |
| `java/util/Collections$2` | 6 | 0 |
| `java/util/Collections$3` | 3 | 0 |
| `java/util/Collections$AsLIFOQueue` | 25 | 0 |
| `java/util/Collections$CheckedCollection` | 25 | 0 |
| `java/util/Collections$CheckedCollection$1` | 5 | 0 |
| `java/util/Collections$CheckedList` | 16 | 0 |
| `java/util/Collections$CheckedList$1` | 11 | 0 |
| `java/util/Collections$CheckedMap` | 33 | 0 |
| `java/util/Collections$CheckedMap$CheckedEntrySet` | 20 | 0 |
| `java/util/Collections$CheckedMap$CheckedEntrySet$1` | 7 | 0 |
| `java/util/Collections$CheckedMap$CheckedEntrySet$CheckedEntry` | 8 | 0 |
| `java/util/Collections$CheckedNavigableMap` | 30 | 0 |
| `java/util/Collections$CheckedNavigableSet` | 18 | 0 |
| `java/util/Collections$CheckedQueue` | 8 | 0 |
| `java/util/Collections$CheckedRandomAccessList` | 2 | 0 |
| `java/util/Collections$CheckedSet` | 3 | 0 |
| `java/util/Collections$CheckedSortedMap` | 7 | 0 |
| `java/util/Collections$CheckedSortedSet` | 7 | 0 |
| `java/util/Collections$CopiesList` | 19 | 0 |
| `java/util/Collections$ReverseComparator2` | 6 | 0 |
| `java/util/Collections$SequencedSetFromMap` | 11 | 0 |
| `java/util/Collections$SetFromMap` | 23 | 0 |
| `java/util/Collections$SingletonList` | 11 | 0 |
| `java/util/Collections$SingletonMap` | 21 | 0 |
| `java/util/Collections$SingletonSet` | 8 | 0 |
| `java/util/Collections$SynchronizedCollection` | 23 | 0 |
| `java/util/Collections$SynchronizedList` | 17 | 0 |
| `java/util/Collections$SynchronizedMap` | 29 | 0 |
| `java/util/Collections$SynchronizedNavigableMap` | 25 | 0 |
| `java/util/Collections$SynchronizedNavigableSet` | 19 | 0 |
| `java/util/Collections$SynchronizedRandomAccessList` | 4 | 0 |
| `java/util/Collections$SynchronizedSet` | 4 | 0 |
| `java/util/Collections$SynchronizedSortedMap` | 8 | 0 |
| `java/util/Collections$SynchronizedSortedSet` | 8 | 0 |
| `java/util/Collections$UnmodifiableCollection` | 21 | 0 |
| `java/util/Collections$UnmodifiableCollection$1` | 5 | 0 |
| `java/util/Collections$UnmodifiableList` | 16 | 0 |
| `java/util/Collections$UnmodifiableList$1` | 11 | 0 |
| `java/util/Collections$UnmodifiableMap` | 27 | 0 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet` | 13 | 0 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1` | 6 | 0 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry` | 7 | 0 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntrySetSpliterator` | 9 | 0 |
| `java/util/Collections$UnmodifiableNavigableMap` | 20 | 0 |
| `java/util/Collections$UnmodifiableNavigableSet` | 13 | 0 |
| `java/util/Collections$UnmodifiableRandomAccessList` | 3 | 0 |
| `java/util/Collections$UnmodifiableSequencedCollection` | 9 | 0 |
| `java/util/Collections$UnmodifiableSequencedMap` | 7 | 0 |
| `java/util/Collections$UnmodifiableSequencedSet` | 6 | 0 |
| `java/util/Collections$UnmodifiableSet` | 3 | 0 |
| `java/util/Collections$UnmodifiableSortedMap` | 7 | 0 |
| `java/util/Collections$UnmodifiableSortedSet` | 7 | 0 |
| `java/util/ComparableTimSort` | 16 | 0 |
| `java/util/Comparator` | 25 | 0 |
| `java/util/Comparators$NullComparator` | 4 | 0 |
| `java/util/ConcurrentModificationException` | 4 | 0 |
| `java/util/Currency` | 30 | 0 |
| `java/util/Currency$1` | 3 | 0 |
| `java/util/Currency$CurrencyProperty` | 8 | 0 |
| `java/util/Currency$OtherCurrencyEntry` | 2 | 0 |
| `java/util/Currency$SpecialCaseEntry` | 6 | 0 |
| `java/util/Date` | 49 | 0 |
| `java/util/Deque` | 30 | 0 |
| `java/util/Dictionary` | 8 | 0 |
| `java/util/DoubleSummaryStatistics` | 11 | 0 |
| `java/util/DualPivotQuicksort` | 49 | 0 |
| `java/util/DualPivotQuicksort$Merger` | 3 | 0 |
| `java/util/DualPivotQuicksort$RunMerger` | 4 | 0 |
| `java/util/DualPivotQuicksort$Sorter` | 4 | 0 |
| `java/util/DuplicateFormatFlagsException` | 3 | 0 |
| `java/util/EmptyStackException` | 1 | 0 |
| `java/util/EnumMap` | 31 | 0 |
| `java/util/EnumMap$EntryIterator` | 4 | 0 |
| `java/util/EnumMap$EntryIterator$Entry` | 9 | 0 |
| `java/util/EnumMap$EntrySet` | 9 | 0 |
| `java/util/EnumMap$EnumMapIterator` | 4 | 0 |
| `java/util/EnumMap$KeyIterator` | 3 | 0 |
| `java/util/EnumMap$KeySet` | 6 | 0 |
| `java/util/EnumMap$ValueIterator` | 2 | 0 |
| `java/util/EnumMap$Values` | 6 | 0 |
| `java/util/EnumSet` | 23 | 0 |
| `java/util/EnumSet$SerializationProxy` | 3 | 0 |
| `java/util/Enumeration` | 3 | 0 |
| `java/util/Enumeration$1` | 3 | 0 |
| `java/util/FormatFlagsConversionMismatchException` | 4 | 0 |
| `java/util/Formattable` | 1 | 0 |
| `java/util/Formatter` | 36 | 0 |
| `java/util/Formatter$Conversion` | 7 | 0 |
| `java/util/Formatter$DateTime` | 2 | 0 |
| `java/util/Formatter$FixedString` | 4 | 0 |
| `java/util/Formatter$Flags` | 8 | 0 |
| `java/util/Formatter$FormatSpecifier` | 54 | 0 |
| `java/util/Formatter$FormatSpecifier$BigDecimalLayout` | 6 | 0 |
| `java/util/Formatter$FormatString` | 3 | 0 |
| `java/util/FormatterClosedException` | 1 | 0 |
| `java/util/GregorianCalendar` | 62 | 0 |
| `java/util/HashMap` | 56 | 0 |
| `java/util/HashMap$EntryIterator` | 3 | 0 |
| `java/util/HashMap$EntrySet` | 8 | 0 |
| `java/util/HashMap$EntrySpliterator` | 6 | 0 |
| `java/util/HashMap$HashIterator` | 4 | 0 |
| `java/util/HashMap$HashMapSpliterator` | 3 | 0 |
| `java/util/HashMap$KeyIterator` | 2 | 0 |
| `java/util/HashMap$KeySet` | 10 | 0 |
| `java/util/HashMap$KeySpliterator` | 6 | 0 |
| `java/util/HashMap$Node` | 7 | 0 |
| `java/util/HashMap$TreeNode` | 17 | 0 |
| `java/util/HashMap$UnsafeHolder` | 3 | 0 |
| `java/util/HashMap$ValueIterator` | 2 | 0 |
| `java/util/HashMap$ValueSpliterator` | 6 | 0 |
| `java/util/HashMap$Values` | 9 | 0 |
| `java/util/HashSet` | 20 | 0 |
| `java/util/Hashtable` | 46 | 0 |
| `java/util/Hashtable$Entry` | 8 | 0 |
| `java/util/Hashtable$EntrySet` | 8 | 0 |
| `java/util/Hashtable$Enumerator` | 6 | 0 |
| `java/util/Hashtable$KeySet` | 6 | 0 |
| `java/util/Hashtable$UnsafeHolder` | 3 | 0 |
| `java/util/Hashtable$ValueCollection` | 5 | 0 |
| `java/util/HexFormat` | 45 | 0 |
| `java/util/IdentityHashMap` | 36 | 0 |
| `java/util/IdentityHashMap$EntryIterator` | 4 | 0 |
| `java/util/IdentityHashMap$EntryIterator$Entry` | 8 | 0 |
| `java/util/IdentityHashMap$EntrySet` | 10 | 0 |
| `java/util/IdentityHashMap$EntrySpliterator` | 6 | 0 |
| `java/util/IdentityHashMap$IdentityHashMapIterator` | 4 | 0 |
| `java/util/IdentityHashMap$IdentityHashMapSpliterator` | 3 | 0 |
| `java/util/IdentityHashMap$KeyIterator` | 2 | 0 |
| `java/util/IdentityHashMap$KeySet` | 11 | 0 |
| `java/util/IdentityHashMap$KeySpliterator` | 6 | 0 |
| `java/util/IdentityHashMap$ValueIterator` | 2 | 0 |
| `java/util/IdentityHashMap$ValueSpliterator` | 6 | 0 |
| `java/util/IdentityHashMap$Values` | 9 | 0 |
| `java/util/IllegalFormatArgumentIndexException` | 3 | 0 |
| `java/util/IllegalFormatCodePointException` | 3 | 0 |
| `java/util/IllegalFormatConversionException` | 4 | 0 |
| `java/util/IllegalFormatException` | 1 | 0 |
| `java/util/IllegalFormatFlagsException` | 3 | 0 |
| `java/util/IllegalFormatPrecisionException` | 3 | 0 |
| `java/util/IllegalFormatWidthException` | 3 | 0 |
| `java/util/IllformedLocaleException` | 4 | 0 |
| `java/util/ImmutableCollections` | 7 | 0 |
| `java/util/ImmutableCollections$AbstractImmutableCollection` | 8 | 0 |
| `java/util/ImmutableCollections$AbstractImmutableList` | 18 | 0 |
| `java/util/ImmutableCollections$AbstractImmutableMap` | 15 | 0 |
| `java/util/ImmutableCollections$AbstractImmutableSet` | 3 | 0 |
| `java/util/ImmutableCollections$List12` | 11 | 0 |
| `java/util/ImmutableCollections$ListItr` | 11 | 0 |
| `java/util/ImmutableCollections$ListN` | 10 | 0 |
| `java/util/ImmutableCollections$Map1` | 10 | 0 |
| `java/util/ImmutableCollections$MapN` | 11 | 0 |
| `java/util/ImmutableCollections$MapN$1` | 3 | 0 |
| `java/util/ImmutableCollections$MapN$MapNIterator` | 5 | 0 |
| `java/util/ImmutableCollections$Set12` | 11 | 0 |
| `java/util/ImmutableCollections$Set12$1` | 3 | 0 |
| `java/util/ImmutableCollections$SetN` | 11 | 0 |
| `java/util/ImmutableCollections$SetN$SetNIterator` | 3 | 0 |
| `java/util/ImmutableCollections$SubList` | 15 | 0 |
| `java/util/InputMismatchException` | 2 | 0 |
| `java/util/IntSummaryStatistics` | 10 | 0 |
| `java/util/Iterator` | 4 | 0 |
| `java/util/JapaneseImperialCalendar` | 40 | 0 |
| `java/util/JumboEnumSet` | 20 | 0 |
| `java/util/JumboEnumSet$EnumSetIterator` | 5 | 0 |
| `java/util/KeyValueHolder` | 7 | 0 |
| `java/util/LinkedHashMap` | 37 | 0 |
| `java/util/LinkedHashMap$Entry` | 1 | 0 |
| `java/util/LinkedHashMap$LinkedEntryIterator` | 3 | 0 |
| `java/util/LinkedHashMap$LinkedEntrySet` | 23 | 0 |
| `java/util/LinkedHashMap$LinkedHashIterator` | 4 | 0 |
| `java/util/LinkedHashMap$LinkedKeyIterator` | 2 | 0 |
| `java/util/LinkedHashMap$LinkedKeySet` | 18 | 0 |
| `java/util/LinkedHashMap$LinkedValueIterator` | 2 | 0 |
| `java/util/LinkedHashMap$LinkedValues` | 16 | 0 |
| `java/util/LinkedHashMap$ReversedLinkedHashMapView` | 33 | 0 |
| `java/util/LinkedHashSet` | 15 | 0 |
| `java/util/LinkedHashSet$1ReverseLinkedHashSetView` | 14 | 0 |
| `java/util/LinkedList` | 61 | 0 |
| `java/util/LinkedList$DescendingIterator` | 4 | 0 |
| `java/util/LinkedList$LLSpliterator` | 7 | 0 |
| `java/util/LinkedList$ListItr` | 12 | 0 |
| `java/util/LinkedList$Node` | 1 | 0 |
| `java/util/LinkedList$ReverseOrderLinkedListView` | 63 | 0 |
| `java/util/List` | 49 | 0 |
| `java/util/ListIterator` | 9 | 0 |
| `java/util/ListResourceBundle` | 6 | 0 |
| `java/util/Locale` | 80 | 0 |
| `java/util/Locale$Builder` | 14 | 0 |
| `java/util/Locale$Cache` | 4 | 0 |
| `java/util/Locale$Category` | 5 | 0 |
| `java/util/Locale$IsoCountryCode` | 7 | 0 |
| `java/util/Locale$LanguageRange` | 11 | 0 |
| `java/util/Locale$LocaleKey` | 3 | 0 |
| `java/util/LongSummaryStatistics` | 11 | 0 |
| `java/util/Map` | 39 | 0 |
| `java/util/Map$Entry` | 15 | 0 |
| `java/util/MissingFormatArgumentException` | 3 | 0 |
| `java/util/MissingFormatWidthException` | 3 | 0 |
| `java/util/MissingResourceException` | 4 | 0 |
| `java/util/NavigableMap` | 24 | 0 |
| `java/util/NavigableSet` | 21 | 0 |
| `java/util/NoSuchElementException` | 4 | 0 |
| `java/util/Objects` | 22 | 0 |
| `java/util/Optional` | 22 | 0 |
| `java/util/OptionalDouble` | 18 | 0 |
| `java/util/OptionalInt` | 18 | 0 |
| `java/util/OptionalLong` | 18 | 0 |
| `java/util/PrimitiveIterator$OfDouble` | 6 | 0 |
| `java/util/PrimitiveIterator$OfInt` | 6 | 0 |
| `java/util/PrimitiveIterator$OfLong` | 6 | 0 |
| `java/util/Properties` | 62 | 0 |
| `java/util/Properties$EntrySet` | 18 | 0 |
| `java/util/Properties$LineReader` | 3 | 0 |
| `java/util/PropertyPermission` | 13 | 0 |
| `java/util/PropertyPermissionCollection` | 8 | 0 |
| `java/util/PropertyResourceBundle` | 6 | 0 |
| `java/util/Queue` | 6 | 0 |
| `java/util/Random` | 32 | 0 |
| `java/util/Random$RandomWrapper` | 36 | 0 |
| `java/util/RandomAccess` | 0 | 0 |
| `java/util/RegularEnumSet` | 17 | 0 |
| `java/util/RegularEnumSet$EnumSetIterator` | 5 | 0 |
| `java/util/ResourceBundle` | 50 | 0 |
| `java/util/ResourceBundle$3` | 3 | 0 |
| `java/util/ResourceBundle$4` | 3 | 0 |
| `java/util/ResourceBundle$BundleReference` | 2 | 0 |
| `java/util/ResourceBundle$CacheKey` | 17 | 0 |
| `java/util/ResourceBundle$CacheKeyReference` | 1 | 0 |
| `java/util/ResourceBundle$Control` | 15 | 0 |
| `java/util/ResourceBundle$Control$1` | 3 | 0 |
| `java/util/ResourceBundle$Control$2` | 3 | 0 |
| `java/util/ResourceBundle$Control$CandidateListCache` | 4 | 0 |
| `java/util/ResourceBundle$KeyElementReference` | 2 | 0 |
| `java/util/ResourceBundle$ResourceBundleControlProviderHolder` | 5 | 0 |
| `java/util/ResourceBundle$ResourceBundleProviderHelper` | 9 | 0 |
| `java/util/ResourceBundle$SingleFormatControl` | 3 | 0 |
| `java/util/ReverseOrderDequeView` | 43 | 0 |
| `java/util/ReverseOrderListView` | 38 | 0 |
| `java/util/ReverseOrderListView$DescendingIterator` | 4 | 0 |
| `java/util/ReverseOrderListView$DescendingListIterator` | 10 | 0 |
| `java/util/ReverseOrderListView$Rand` | 1 | 0 |
| `java/util/ReverseOrderSortedMapView` | 33 | 0 |
| `java/util/ReverseOrderSortedMapView$1` | 6 | 0 |
| `java/util/ReverseOrderSortedMapView$2` | 6 | 0 |
| `java/util/ReverseOrderSortedMapView$3` | 6 | 0 |
| `java/util/ReverseOrderSortedMapView$4` | 4 | 0 |
| `java/util/ReverseOrderSortedMapView$5` | 4 | 0 |
| `java/util/ReverseOrderSortedMapView$6` | 5 | 0 |
| `java/util/ReverseOrderSortedMapView$Submap` | 15 | 0 |
| `java/util/ReverseOrderSortedMapView$Submap$1` | 5 | 0 |
| `java/util/ReverseOrderSortedMapView$Submap$2` | 3 | 0 |
| `java/util/ReverseOrderSortedMapView$ViewEntry` | 7 | 0 |
| `java/util/ReverseOrderSortedSetView` | 30 | 0 |
| `java/util/ReverseOrderSortedSetView$1` | 4 | 0 |
| `java/util/ReverseOrderSortedSetView$Subset` | 13 | 0 |
| `java/util/ReverseOrderSortedSetView$Subset$1` | 3 | 0 |
| `java/util/Scanner` | 111 | 0 |
| `java/util/Scanner$FindSpliterator` | 3 | 0 |
| `java/util/Scanner$PatternLRUCache` | 4 | 0 |
| `java/util/Scanner$TokenSpliterator` | 2 | 0 |
| `java/util/SequencedCollection` | 7 | 0 |
| `java/util/SequencedMap` | 10 | 0 |
| `java/util/SequencedMap$1SeqEntrySet` | 6 | 0 |
| `java/util/SequencedMap$1SeqKeySet` | 6 | 0 |
| `java/util/SequencedMap$1SeqValues` | 3 | 0 |
| `java/util/SequencedSet` | 2 | 0 |
| `java/util/ServiceConfigurationError` | 2 | 0 |
| `java/util/ServiceLoader` | 25 | 0 |
| `java/util/ServiceLoader$1` | 3 | 0 |
| `java/util/ServiceLoader$2` | 4 | 0 |
| `java/util/ServiceLoader$3` | 4 | 0 |
| `java/util/ServiceLoader$LayerLookupIterator` | 6 | 0 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator` | 10 | 0 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator$1` | 3 | 0 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator$2` | 3 | 0 |
| `java/util/ServiceLoader$ModuleServicesLookupIterator` | 8 | 0 |
| `java/util/ServiceLoader$Provider` | 2 | 0 |
| `java/util/ServiceLoader$ProviderImpl` | 8 | 0 |
| `java/util/ServiceLoader$ProviderImpl$1` | 2 | 0 |
| `java/util/ServiceLoader$ProviderImpl$2` | 2 | 0 |
| `java/util/ServiceLoader$ProviderSpliterator` | 5 | 0 |
| `java/util/Set` | 29 | 0 |
| `java/util/SimpleTimeZone` | 42 | 0 |
| `java/util/SimpleTimeZone$Cache` | 1 | 0 |
| `java/util/SortedMap` | 13 | 0 |
| `java/util/SortedSet` | 16 | 0 |
| `java/util/SortedSet$1` | 2 | 0 |
| `java/util/Spliterator` | 8 | 0 |
| `java/util/Spliterator$OfDouble` | 9 | 0 |
| `java/util/Spliterator$OfInt` | 9 | 0 |
| `java/util/Spliterator$OfLong` | 9 | 0 |
| `java/util/Spliterator$OfPrimitive` | 4 | 0 |
| `java/util/Spliterators` | 28 | 0 |
| `java/util/Spliterators$1Adapter` | 5 | 0 |
| `java/util/Spliterators$2Adapter` | 6 | 0 |
| `java/util/Spliterators$3Adapter` | 6 | 0 |
| `java/util/Spliterators$4Adapter` | 6 | 0 |
| `java/util/Spliterators$AbstractDoubleSpliterator` | 6 | 0 |
| `java/util/Spliterators$AbstractDoubleSpliterator$HoldingDoubleConsumer` | 2 | 0 |
| `java/util/Spliterators$AbstractIntSpliterator` | 6 | 0 |
| `java/util/Spliterators$AbstractIntSpliterator$HoldingIntConsumer` | 2 | 0 |
| `java/util/Spliterators$AbstractLongSpliterator` | 6 | 0 |
| `java/util/Spliterators$AbstractLongSpliterator$HoldingLongConsumer` | 2 | 0 |
| `java/util/Spliterators$AbstractSpliterator` | 4 | 0 |
| `java/util/Spliterators$AbstractSpliterator$HoldingConsumer` | 2 | 0 |
| `java/util/Spliterators$ArraySpliterator` | 9 | 0 |
| `java/util/Spliterators$DoubleArraySpliterator` | 13 | 0 |
| `java/util/Spliterators$DoubleIteratorSpliterator` | 12 | 0 |
| `java/util/Spliterators$IntArraySpliterator` | 13 | 0 |
| `java/util/Spliterators$IntIteratorSpliterator` | 12 | 0 |
| `java/util/Spliterators$IteratorSpliterator` | 9 | 0 |
| `java/util/Spliterators$LongArraySpliterator` | 13 | 0 |
| `java/util/Spliterators$LongIteratorSpliterator` | 12 | 0 |
| `java/util/Stack` | 6 | 0 |
| `java/util/StringJoiner` | 10 | 0 |
| `java/util/StringTokenizer` | 13 | 0 |
| `java/util/TimSort` | 16 | 0 |
| `java/util/TimeZone` | 34 | 2 |
| `java/util/TreeMap` | 96 | 0 |
| `java/util/TreeMap$AscendingSubMap` | 16 | 0 |
| `java/util/TreeMap$AscendingSubMap$AscendingEntrySetView` | 2 | 0 |
| `java/util/TreeMap$DescendingKeyIterator` | 3 | 0 |
| `java/util/TreeMap$DescendingKeySpliterator` | 6 | 0 |
| `java/util/TreeMap$DescendingSubMap` | 16 | 0 |
| `java/util/TreeMap$DescendingSubMap$DescendingEntrySetView` | 2 | 0 |
| `java/util/TreeMap$Entry` | 7 | 0 |
| `java/util/TreeMap$EntryIterator` | 3 | 0 |
| `java/util/TreeMap$EntrySet` | 7 | 0 |
| `java/util/TreeMap$EntrySpliterator` | 9 | 0 |
| `java/util/TreeMap$KeyIterator` | 2 | 0 |
| `java/util/TreeMap$KeySet` | 25 | 0 |
| `java/util/TreeMap$KeySpliterator` | 7 | 0 |
| `java/util/TreeMap$NavigableSubMap` | 54 | 0 |
| `java/util/TreeMap$NavigableSubMap$DescendingSubMapEntryIterator` | 4 | 0 |
| `java/util/TreeMap$NavigableSubMap$DescendingSubMapKeyIterator` | 8 | 0 |
| `java/util/TreeMap$NavigableSubMap$EntrySetView` | 5 | 0 |
| `java/util/TreeMap$NavigableSubMap$SubMapEntryIterator` | 4 | 0 |
| `java/util/TreeMap$NavigableSubMap$SubMapIterator` | 6 | 0 |
| `java/util/TreeMap$NavigableSubMap$SubMapKeyIterator` | 9 | 0 |
| `java/util/TreeMap$PrivateEntryIterator` | 5 | 0 |
| `java/util/TreeMap$TreeMapSpliterator` | 3 | 0 |
| `java/util/TreeMap$ValueIterator` | 2 | 0 |
| `java/util/TreeMap$ValueSpliterator` | 6 | 0 |
| `java/util/TreeMap$Values` | 7 | 0 |
| `java/util/TreeSet` | 37 | 0 |
| `java/util/Tripwire` | 4 | 0 |
| `java/util/UnknownFormatConversionException` | 3 | 0 |
| `java/util/UnknownFormatFlagsException` | 3 | 0 |
| `java/util/Vector` | 69 | 0 |
| `java/util/Vector$1` | 3 | 0 |
| `java/util/Vector$Itr` | 6 | 0 |
| `java/util/Vector$ListItr` | 7 | 0 |
| `java/util/Vector$VectorSpliterator` | 7 | 0 |
| `java/util/WeakHashMap` | 33 | 0 |
| `java/util/WeakHashMap$Entry` | 7 | 0 |
| `java/util/WeakHashMap$EntryIterator` | 3 | 0 |
| `java/util/WeakHashMap$EntrySet` | 10 | 0 |
| `java/util/WeakHashMap$EntrySpliterator` | 6 | 0 |
| `java/util/WeakHashMap$HashIterator` | 4 | 0 |
| `java/util/WeakHashMap$KeyIterator` | 2 | 0 |
| `java/util/WeakHashMap$KeySet` | 7 | 0 |
| `java/util/WeakHashMap$KeySpliterator` | 6 | 0 |
| `java/util/WeakHashMap$ValueIterator` | 2 | 0 |
| `java/util/WeakHashMap$ValueSpliterator` | 6 | 0 |
| `java/util/WeakHashMap$Values` | 6 | 0 |
| `java/util/WeakHashMap$WeakHashMapSpliterator` | 3 | 0 |
| `java/util/concurrent/AbstractExecutorService` | 14 | 0 |
| `java/util/concurrent/BlockingQueue` | 11 | 0 |
| `java/util/concurrent/Callable` | 1 | 0 |
| `java/util/concurrent/CancellationException` | 2 | 0 |
| `java/util/concurrent/CompletableFuture` | 178 | 0 |
| `java/util/concurrent/CompletableFuture$AltResult` | 1 | 0 |
| `java/util/concurrent/CompletableFuture$AnyOf` | 3 | 0 |
| `java/util/concurrent/CompletableFuture$AsyncRun` | 7 | 0 |
| `java/util/concurrent/CompletableFuture$AsyncSupply` | 7 | 0 |
| `java/util/concurrent/CompletableFuture$AsynchronousCompletionTask` | 0 | 0 |
| `java/util/concurrent/CompletableFuture$BiAccept` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$BiApply` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$BiCompletion` | 1 | 0 |
| `java/util/concurrent/CompletableFuture$BiRelay` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$BiRun` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$Canceller` | 3 | 0 |
| `java/util/concurrent/CompletableFuture$CoCompletion` | 3 | 0 |
| `java/util/concurrent/CompletableFuture$Completion` | 9 | 0 |
| `java/util/concurrent/CompletableFuture$DelayedCompleter` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$DelayedExecutor` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$Delayer` | 3 | 0 |
| `java/util/concurrent/CompletableFuture$MinimalStage` | 66 | 0 |
| `java/util/concurrent/CompletableFuture$OrAccept` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$OrApply` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$OrRun` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$Signaller` | 5 | 0 |
| `java/util/concurrent/CompletableFuture$TaskSubmitter` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$Timeout` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniAccept` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniApply` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniCompletion` | 3 | 0 |
| `java/util/concurrent/CompletableFuture$UniCompose` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniComposeExceptionally` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniExceptionally` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniHandle` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniRelay` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniRun` | 2 | 0 |
| `java/util/concurrent/CompletableFuture$UniWhenComplete` | 2 | 0 |
| `java/util/concurrent/CompletionException` | 4 | 0 |
| `java/util/concurrent/CompletionStage` | 52 | 0 |
| `java/util/concurrent/ConcurrentHashMap` | 95 | 0 |
| `java/util/concurrent/ConcurrentHashMap$BaseIterator` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$BulkTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$CollectionView` | 14 | 0 |
| `java/util/concurrent/ConcurrentHashMap$CounterCell` | 1 | 0 |
| `java/util/concurrent/ConcurrentHashMap$EntryIterator` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$EntrySetView` | 12 | 0 |
| `java/util/concurrent/ConcurrentHashMap$EntrySpliterator` | 7 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachEntryTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachKeyTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachMappingTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedEntryTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedKeyTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedMappingTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedValueTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForEachValueTask` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ForwardingNode` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$KeyIterator` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$KeySetView` | 13 | 0 |
| `java/util/concurrent/ConcurrentHashMap$KeySpliterator` | 7 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapEntry` | 7 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToDoubleTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToIntTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToLongTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysToDoubleTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysToIntTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysToLongTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToDoubleTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToIntTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToLongTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesToDoubleTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesToIntTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesToLongTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$Node` | 9 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ReduceEntriesTask` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ReduceKeysTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ReduceValuesTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ReservationNode` | 2 | 0 |
| `java/util/concurrent/ConcurrentHashMap$SearchEntriesTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$SearchKeysTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$SearchMappingsTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$SearchValuesTask` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$Segment` | 1 | 0 |
| `java/util/concurrent/ConcurrentHashMap$TableStack` | 1 | 0 |
| `java/util/concurrent/ConcurrentHashMap$Traverser` | 4 | 0 |
| `java/util/concurrent/ConcurrentHashMap$TreeBin` | 14 | 0 |
| `java/util/concurrent/ConcurrentHashMap$TreeNode` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ValueIterator` | 3 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ValueSpliterator` | 7 | 0 |
| `java/util/concurrent/ConcurrentHashMap$ValuesView` | 10 | 0 |
| `java/util/concurrent/ConcurrentLinkedQueue` | 35 | 0 |
| `java/util/concurrent/ConcurrentLinkedQueue$CLQSpliterator` | 8 | 0 |
| `java/util/concurrent/ConcurrentLinkedQueue$Itr` | 4 | 0 |
| `java/util/concurrent/ConcurrentLinkedQueue$Node` | 4 | 0 |
| `java/util/concurrent/ConcurrentMap` | 12 | 0 |
| `java/util/concurrent/CopyOnWriteArrayList` | 70 | 0 |
| `java/util/concurrent/CopyOnWriteArrayList$COWIterator` | 11 | 0 |
| `java/util/concurrent/CopyOnWriteArrayList$COWSubList` | 47 | 0 |
| `java/util/concurrent/CopyOnWriteArrayList$COWSubListIterator` | 11 | 0 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed` | 44 | 0 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed$DescendingIterator` | 4 | 0 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed$DescendingListIterator` | 10 | 0 |
| `java/util/concurrent/CountDownLatch` | 6 | 0 |
| `java/util/concurrent/CountDownLatch$Sync` | 4 | 0 |
| `java/util/concurrent/CountedCompleter` | 26 | 0 |
| `java/util/concurrent/Delayed` | 1 | 0 |
| `java/util/concurrent/ExecutionException` | 4 | 0 |
| `java/util/concurrent/Executor` | 1 | 0 |
| `java/util/concurrent/ExecutorCompletionService` | 9 | 0 |
| `java/util/concurrent/ExecutorCompletionService$QueueingFuture` | 2 | 0 |
| `java/util/concurrent/ExecutorService` | 13 | 0 |
| `java/util/concurrent/Executors` | 25 | 0 |
| `java/util/concurrent/Executors$1` | 2 | 0 |
| `java/util/concurrent/Executors$2` | 2 | 0 |
| `java/util/concurrent/Executors$AutoShutdownDelegatedExecutorService` | 5 | 0 |
| `java/util/concurrent/Executors$DefaultThreadFactory` | 3 | 0 |
| `java/util/concurrent/Executors$DelegatedExecutorService` | 14 | 0 |
| `java/util/concurrent/Executors$DelegatedScheduledExecutorService` | 5 | 0 |
| `java/util/concurrent/Executors$PrivilegedCallable` | 3 | 0 |
| `java/util/concurrent/Executors$PrivilegedCallable$1` | 2 | 0 |
| `java/util/concurrent/Executors$PrivilegedCallableUsingCurrentClassLoader` | 3 | 0 |
| `java/util/concurrent/Executors$PrivilegedCallableUsingCurrentClassLoader$1` | 2 | 0 |
| `java/util/concurrent/Executors$PrivilegedThreadFactory` | 2 | 0 |
| `java/util/concurrent/Executors$PrivilegedThreadFactory$1` | 2 | 0 |
| `java/util/concurrent/Executors$PrivilegedThreadFactory$1$1` | 3 | 0 |
| `java/util/concurrent/Executors$RunnableAdapter` | 3 | 0 |
| `java/util/concurrent/ForkJoinPool` | 93 | 0 |
| `java/util/concurrent/ForkJoinPool$ForkJoinWorkerThreadFactory` | 1 | 0 |
| `java/util/concurrent/ForkJoinPool$InvokeAnyRoot` | 5 | 0 |
| `java/util/concurrent/ForkJoinPool$InvokeAnyTask` | 5 | 0 |
| `java/util/concurrent/ForkJoinPool$ManagedBlocker` | 2 | 0 |
| `java/util/concurrent/ForkJoinPool$WorkQueue` | 22 | 0 |
| `java/util/concurrent/ForkJoinTask` | 68 | 0 |
| `java/util/concurrent/ForkJoinTask$AdaptedCallable` | 6 | 0 |
| `java/util/concurrent/ForkJoinTask$AdaptedInterruptibleCallable` | 7 | 0 |
| `java/util/concurrent/ForkJoinTask$AdaptedRunnable` | 6 | 0 |
| `java/util/concurrent/ForkJoinTask$AdaptedRunnableAction` | 8 | 0 |
| `java/util/concurrent/ForkJoinTask$Aux` | 3 | 0 |
| `java/util/concurrent/ForkJoinTask$RunnableExecuteAction` | 7 | 0 |
| `java/util/concurrent/ForkJoinWorkerThread` | 9 | 0 |
| `java/util/concurrent/Future` | 8 | 0 |
| `java/util/concurrent/Future$State` | 5 | 0 |
| `java/util/concurrent/FutureTask` | 22 | 0 |
| `java/util/concurrent/FutureTask$WaitNode` | 1 | 0 |
| `java/util/concurrent/Helpers` | 5 | 0 |
| `java/util/concurrent/LinkedBlockingDeque` | 68 | 0 |
| `java/util/concurrent/LinkedBlockingDeque$AbstractItr` | 8 | 0 |
| `java/util/concurrent/LinkedBlockingDeque$DescendingItr` | 3 | 0 |
| `java/util/concurrent/LinkedBlockingDeque$Itr` | 3 | 0 |
| `java/util/concurrent/LinkedBlockingDeque$LBDSpliterator` | 6 | 0 |
| `java/util/concurrent/LinkedBlockingDeque$Node` | 1 | 0 |
| `java/util/concurrent/LinkedBlockingQueue` | 41 | 0 |
| `java/util/concurrent/LinkedBlockingQueue$Itr` | 5 | 0 |
| `java/util/concurrent/LinkedBlockingQueue$LBQSpliterator` | 6 | 0 |
| `java/util/concurrent/LinkedBlockingQueue$Node` | 1 | 0 |
| `java/util/concurrent/LinkedTransferQueue` | 51 | 0 |
| `java/util/concurrent/LinkedTransferQueue$DualNode` | 10 | 0 |
| `java/util/concurrent/LinkedTransferQueue$Itr` | 6 | 0 |
| `java/util/concurrent/LinkedTransferQueue$LTQSpliterator` | 8 | 0 |
| `java/util/concurrent/RecursiveTask` | 5 | 0 |
| `java/util/concurrent/RejectedExecutionException` | 4 | 0 |
| `java/util/concurrent/RejectedExecutionHandler` | 1 | 0 |
| `java/util/concurrent/RunnableScheduledFuture` | 1 | 0 |
| `java/util/concurrent/ScheduledExecutorService` | 4 | 0 |
| `java/util/concurrent/ScheduledFuture` | 0 | 0 |
| `java/util/concurrent/ScheduledThreadPoolExecutor` | 31 | 0 |
| `java/util/concurrent/ScheduledThreadPoolExecutor$DelayedWorkQueue` | 34 | 0 |
| `java/util/concurrent/ScheduledThreadPoolExecutor$DelayedWorkQueue$Itr` | 5 | 0 |
| `java/util/concurrent/ScheduledThreadPoolExecutor$ScheduledFutureTask` | 10 | 0 |
| `java/util/concurrent/StructureViolationException` | 2 | 0 |
| `java/util/concurrent/SynchronousQueue` | 28 | 0 |
| `java/util/concurrent/SynchronousQueue$FifoWaitQueue` | 1 | 0 |
| `java/util/concurrent/SynchronousQueue$LifoWaitQueue` | 1 | 0 |
| `java/util/concurrent/SynchronousQueue$Transferer` | 3 | 0 |
| `java/util/concurrent/SynchronousQueue$WaitQueue` | 1 | 0 |
| `java/util/concurrent/ThreadFactory` | 1 | 0 |
| `java/util/concurrent/ThreadLocalRandom` | 40 | 0 |
| `java/util/concurrent/ThreadPerTaskExecutor` | 31 | 0 |
| `java/util/concurrent/ThreadPerTaskExecutor$AnyResultHolder` | 7 | 0 |
| `java/util/concurrent/ThreadPerTaskExecutor$TaskRunner` | 2 | 0 |
| `java/util/concurrent/ThreadPerTaskExecutor$ThreadBoundFuture` | 3 | 0 |
| `java/util/concurrent/ThreadPoolExecutor` | 64 | 0 |
| `java/util/concurrent/ThreadPoolExecutor$Worker` | 10 | 0 |
| `java/util/concurrent/TimeUnit` | 21 | 0 |
| `java/util/concurrent/TimeoutException` | 2 | 0 |
| `java/util/concurrent/atomic/AtomicBoolean` | 23 | 0 |
| `java/util/concurrent/atomic/AtomicInteger` | 37 | 0 |
| `java/util/concurrent/atomic/AtomicLong` | 38 | 1 |
| `java/util/concurrent/atomic/AtomicMarkableReference` | 10 | 0 |
| `java/util/concurrent/atomic/AtomicMarkableReference$Pair` | 2 | 0 |
| `java/util/concurrent/atomic/AtomicReference` | 27 | 0 |
| `java/util/concurrent/locks/AbstractOwnableSynchronizer` | 3 | 0 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer` | 44 | 0 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionNode` | 3 | 0 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionObject` | 17 | 0 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$ExclusiveNode` | 1 | 0 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$Node` | 8 | 0 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$SharedNode` | 1 | 0 |
| `java/util/concurrent/locks/Condition` | 7 | 0 |
| `java/util/concurrent/locks/Lock` | 6 | 0 |
| `java/util/concurrent/locks/LockSupport` | 13 | 0 |
| `java/util/concurrent/locks/ReadWriteLock` | 2 | 0 |
| `java/util/concurrent/locks/ReentrantLock` | 21 | 0 |
| `java/util/concurrent/locks/ReentrantLock$FairSync` | 3 | 0 |
| `java/util/concurrent/locks/ReentrantLock$NonfairSync` | 3 | 0 |
| `java/util/concurrent/locks/ReentrantLock$Sync` | 13 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock` | 23 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$FairSync` | 3 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$NonfairSync` | 3 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock` | 8 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$Sync` | 22 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$Sync$HoldCounter` | 1 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$Sync$ThreadLocalHoldCounter` | 3 | 0 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$WriteLock` | 10 | 0 |
| `java/util/function/BiConsumer` | 3 | 0 |
| `java/util/function/BiFunction` | 3 | 0 |
| `java/util/function/BiPredicate` | 7 | 0 |
| `java/util/function/BinaryOperator` | 4 | 0 |
| `java/util/function/BooleanSupplier` | 1 | 0 |
| `java/util/function/Consumer` | 3 | 0 |
| `java/util/function/DoubleBinaryOperator` | 1 | 0 |
| `java/util/function/DoubleConsumer` | 3 | 0 |
| `java/util/function/DoubleFunction` | 1 | 0 |
| `java/util/function/DoublePredicate` | 7 | 0 |
| `java/util/function/DoubleSupplier` | 1 | 0 |
| `java/util/function/DoubleToIntFunction` | 1 | 0 |
| `java/util/function/DoubleToLongFunction` | 1 | 0 |
| `java/util/function/DoubleUnaryOperator` | 7 | 0 |
| `java/util/function/Function` | 7 | 0 |
| `java/util/function/IntBinaryOperator` | 1 | 0 |
| `java/util/function/IntConsumer` | 3 | 0 |
| `java/util/function/IntFunction` | 1 | 0 |
| `java/util/function/IntPredicate` | 7 | 0 |
| `java/util/function/IntSupplier` | 1 | 0 |
| `java/util/function/IntToDoubleFunction` | 1 | 0 |
| `java/util/function/IntToLongFunction` | 1 | 0 |
| `java/util/function/IntUnaryOperator` | 7 | 0 |
| `java/util/function/LongBinaryOperator` | 1 | 0 |
| `java/util/function/LongConsumer` | 3 | 0 |
| `java/util/function/LongFunction` | 1 | 0 |
| `java/util/function/LongPredicate` | 7 | 0 |
| `java/util/function/LongSupplier` | 1 | 0 |
| `java/util/function/LongToDoubleFunction` | 1 | 0 |
| `java/util/function/LongToIntFunction` | 1 | 0 |
| `java/util/function/LongUnaryOperator` | 7 | 0 |
| `java/util/function/ObjDoubleConsumer` | 1 | 0 |
| `java/util/function/ObjIntConsumer` | 1 | 0 |
| `java/util/function/ObjLongConsumer` | 1 | 0 |
| `java/util/function/Predicate` | 10 | 0 |
| `java/util/function/Supplier` | 1 | 0 |
| `java/util/function/ToDoubleBiFunction` | 1 | 0 |
| `java/util/function/ToDoubleFunction` | 1 | 0 |
| `java/util/function/ToIntBiFunction` | 1 | 0 |
| `java/util/function/ToIntFunction` | 1 | 0 |
| `java/util/function/ToLongBiFunction` | 1 | 0 |
| `java/util/function/ToLongFunction` | 1 | 0 |
| `java/util/function/UnaryOperator` | 2 | 0 |
| `java/util/jar/Attributes` | 25 | 0 |
| `java/util/jar/Attributes$Name` | 8 | 0 |
| `java/util/jar/JarEntry` | 7 | 0 |
| `java/util/jar/JarException` | 2 | 0 |
| `java/util/jar/JarFile` | 35 | 0 |
| `java/util/jar/JarFile$JarFileEntry` | 9 | 0 |
| `java/util/jar/JarVerifier` | 12 | 0 |
| `java/util/jar/JarVerifier$VerifierStream` | 6 | 0 |
| `java/util/jar/Manifest` | 22 | 0 |
| `java/util/jar/Manifest$FastInputStream` | 11 | 0 |
| `java/util/random/RandomGenerator` | 35 | 0 |
| `java/util/random/RandomGeneratorFactory` | 31 | 0 |
| `java/util/regex/ASCII` | 21 | 0 |
| `java/util/regex/CharPredicates` | 60 | 0 |
| `java/util/regex/IntHashSet` | 5 | 0 |
| `java/util/regex/MatchResult` | 13 | 0 |
| `java/util/regex/Matcher` | 54 | 0 |
| `java/util/regex/Matcher$1MatchResultIterator` | 5 | 0 |
| `java/util/regex/Matcher$ImmutableMatchResult` | 12 | 0 |
| `java/util/regex/Pattern` | 119 | 0 |
| `java/util/regex/Pattern$1MatcherIterator` | 4 | 0 |
| `java/util/regex/Pattern$BackRef` | 3 | 0 |
| `java/util/regex/Pattern$Begin` | 2 | 0 |
| `java/util/regex/Pattern$Behind` | 2 | 0 |
| `java/util/regex/Pattern$BehindS` | 2 | 0 |
| `java/util/regex/Pattern$BitClass` | 4 | 0 |
| `java/util/regex/Pattern$BmpCharPredicate` | 3 | 0 |
| `java/util/regex/Pattern$BmpCharProperty` | 2 | 0 |
| `java/util/regex/Pattern$BmpCharPropertyGreedy` | 2 | 0 |
| `java/util/regex/Pattern$BnM` | 4 | 0 |
| `java/util/regex/Pattern$BnMS` | 2 | 0 |
| `java/util/regex/Pattern$Bound` | 4 | 0 |
| `java/util/regex/Pattern$Branch` | 4 | 0 |
| `java/util/regex/Pattern$BranchConn` | 3 | 0 |
| `java/util/regex/Pattern$CIBackRef` | 3 | 0 |
| `java/util/regex/Pattern$Caret` | 2 | 0 |
| `java/util/regex/Pattern$CharPredicate` | 5 | 0 |
| `java/util/regex/Pattern$CharProperty` | 3 | 0 |
| `java/util/regex/Pattern$CharPropertyGreedy` | 3 | 0 |
| `java/util/regex/Pattern$Curly` | 6 | 0 |
| `java/util/regex/Pattern$Dollar` | 3 | 0 |
| `java/util/regex/Pattern$End` | 2 | 0 |
| `java/util/regex/Pattern$First` | 3 | 0 |
| `java/util/regex/Pattern$GraphemeBound` | 2 | 0 |
| `java/util/regex/Pattern$GroupCurly` | 6 | 0 |
| `java/util/regex/Pattern$GroupHead` | 2 | 0 |
| `java/util/regex/Pattern$GroupTail` | 2 | 0 |
| `java/util/regex/Pattern$LastMatch` | 2 | 0 |
| `java/util/regex/Pattern$LazyLoop` | 4 | 0 |
| `java/util/regex/Pattern$LineEnding` | 3 | 0 |
| `java/util/regex/Pattern$Loop` | 4 | 0 |
| `java/util/regex/Pattern$NFCCharProperty` | 3 | 0 |
| `java/util/regex/Pattern$Neg` | 2 | 0 |
| `java/util/regex/Pattern$Node` | 3 | 0 |
| `java/util/regex/Pattern$NotBehind` | 2 | 0 |
| `java/util/regex/Pattern$NotBehindS` | 2 | 0 |
| `java/util/regex/Pattern$Pos` | 2 | 0 |
| `java/util/regex/Pattern$Prolog` | 3 | 0 |
| `java/util/regex/Pattern$Qtype` | 5 | 0 |
| `java/util/regex/Pattern$Ques` | 3 | 0 |
| `java/util/regex/Pattern$Slice` | 2 | 0 |
| `java/util/regex/Pattern$SliceI` | 2 | 0 |
| `java/util/regex/Pattern$SliceIS` | 3 | 0 |
| `java/util/regex/Pattern$SliceNode` | 2 | 0 |
| `java/util/regex/Pattern$SliceS` | 2 | 0 |
| `java/util/regex/Pattern$SliceU` | 2 | 0 |
| `java/util/regex/Pattern$SliceUS` | 2 | 0 |
| `java/util/regex/Pattern$Start` | 3 | 0 |
| `java/util/regex/Pattern$StartS` | 2 | 0 |
| `java/util/regex/Pattern$TreeInfo` | 2 | 0 |
| `java/util/regex/Pattern$UnixCaret` | 2 | 0 |
| `java/util/regex/Pattern$UnixDollar` | 3 | 0 |
| `java/util/regex/Pattern$XGrapheme` | 3 | 0 |
| `java/util/regex/PatternSyntaxException` | 5 | 0 |
| `java/util/spi/CalendarDataProvider` | 3 | 0 |
| `java/util/spi/CalendarNameProvider` | 3 | 0 |
| `java/util/spi/CurrencyNameProvider` | 3 | 0 |
| `java/util/spi/LocaleNameProvider` | 7 | 0 |
| `java/util/spi/LocaleServiceProvider` | 5 | 0 |
| `java/util/spi/ResourceBundleControlProvider` | 1 | 0 |
| `java/util/spi/ResourceBundleProvider` | 1 | 0 |
| `java/util/spi/TimeZoneNameProvider` | 3 | 0 |
| `java/util/stream/AbstractPipeline` | 39 | 0 |
| `java/util/stream/AbstractShortCircuitTask` | 11 | 0 |
| `java/util/stream/AbstractSpinedBuffer` | 6 | 0 |
| `java/util/stream/AbstractTask` | 18 | 0 |
| `java/util/stream/BaseStream` | 8 | 0 |
| `java/util/stream/Collector` | 7 | 0 |
| `java/util/stream/Collectors` | 132 | 0 |
| `java/util/stream/Collectors$1OptionalBox` | 2 | 0 |
| `java/util/stream/Collectors$1PairBox` | 4 | 0 |
| `java/util/stream/Collectors$CollectorImpl` | 10 | 0 |
| `java/util/stream/Collectors$Partition` | 7 | 0 |
| `java/util/stream/Collectors$Partition$1` | 3 | 0 |
| `java/util/stream/DistinctOps` | 2 | 0 |
| `java/util/stream/DistinctOps$1` | 6 | 0 |
| `java/util/stream/DistinctOps$1$1` | 4 | 0 |
| `java/util/stream/DistinctOps$1$2` | 4 | 0 |
| `java/util/stream/DoublePipeline` | 62 | 0 |
| `java/util/stream/DoublePipeline$1` | 2 | 0 |
| `java/util/stream/DoublePipeline$1$1` | 2 | 0 |
| `java/util/stream/DoublePipeline$2` | 2 | 0 |
| `java/util/stream/DoublePipeline$2$1` | 2 | 0 |
| `java/util/stream/DoublePipeline$3` | 2 | 0 |
| `java/util/stream/DoublePipeline$3$1` | 2 | 0 |
| `java/util/stream/DoublePipeline$4` | 2 | 0 |
| `java/util/stream/DoublePipeline$4$1` | 2 | 0 |
| `java/util/stream/DoublePipeline$5` | 2 | 0 |
| `java/util/stream/DoublePipeline$5$1` | 4 | 0 |
| `java/util/stream/DoublePipeline$6` | 2 | 0 |
| `java/util/stream/DoublePipeline$6$1` | 3 | 0 |
| `java/util/stream/DoublePipeline$7` | 2 | 0 |
| `java/util/stream/DoublePipeline$8` | 2 | 0 |
| `java/util/stream/DoublePipeline$8$1` | 3 | 0 |
| `java/util/stream/DoublePipeline$9` | 2 | 0 |
| `java/util/stream/DoublePipeline$9$1` | 2 | 0 |
| `java/util/stream/DoublePipeline$Head` | 12 | 0 |
| `java/util/stream/DoublePipeline$StatefulOp` | 10 | 0 |
| `java/util/stream/DoublePipeline$StatelessOp` | 9 | 0 |
| `java/util/stream/DoubleStream` | 49 | 0 |
| `java/util/stream/DoubleStream$1` | 3 | 0 |
| `java/util/stream/DoubleStream$2` | 5 | 0 |
| `java/util/stream/DoubleStream$DoubleMapMultiConsumer` | 1 | 0 |
| `java/util/stream/FindOps` | 5 | 0 |
| `java/util/stream/ForEachOps` | 5 | 0 |
| `java/util/stream/ForEachOps$ForEachOp` | 8 | 0 |
| `java/util/stream/ForEachOps$ForEachOp$OfDouble` | 6 | 0 |
| `java/util/stream/ForEachOps$ForEachOp$OfInt` | 6 | 0 |
| `java/util/stream/ForEachOps$ForEachOp$OfLong` | 6 | 0 |
| `java/util/stream/ForEachOps$ForEachOp$OfRef` | 5 | 0 |
| `java/util/stream/ForEachOps$ForEachOrderedTask` | 7 | 0 |
| `java/util/stream/ForEachOps$ForEachTask` | 3 | 0 |
| `java/util/stream/IntPipeline` | 61 | 0 |
| `java/util/stream/IntPipeline$1` | 2 | 0 |
| `java/util/stream/IntPipeline$1$1` | 2 | 0 |
| `java/util/stream/IntPipeline$10` | 2 | 0 |
| `java/util/stream/IntPipeline$10$1` | 3 | 0 |
| `java/util/stream/IntPipeline$11` | 2 | 0 |
| `java/util/stream/IntPipeline$11$1` | 2 | 0 |
| `java/util/stream/IntPipeline$2` | 2 | 0 |
| `java/util/stream/IntPipeline$2$1` | 2 | 0 |
| `java/util/stream/IntPipeline$3` | 2 | 0 |
| `java/util/stream/IntPipeline$3$1` | 2 | 0 |
| `java/util/stream/IntPipeline$4` | 2 | 0 |
| `java/util/stream/IntPipeline$4$1` | 2 | 0 |
| `java/util/stream/IntPipeline$5` | 2 | 0 |
| `java/util/stream/IntPipeline$5$1` | 2 | 0 |
| `java/util/stream/IntPipeline$6` | 2 | 0 |
| `java/util/stream/IntPipeline$6$1` | 2 | 0 |
| `java/util/stream/IntPipeline$7` | 2 | 0 |
| `java/util/stream/IntPipeline$7$1` | 4 | 0 |
| `java/util/stream/IntPipeline$8` | 2 | 0 |
| `java/util/stream/IntPipeline$8$1` | 3 | 0 |
| `java/util/stream/IntPipeline$9` | 2 | 0 |
| `java/util/stream/IntPipeline$Head` | 12 | 0 |
| `java/util/stream/IntPipeline$StatefulOp` | 10 | 0 |
| `java/util/stream/IntPipeline$StatelessOp` | 9 | 0 |
| `java/util/stream/IntStream` | 53 | 0 |
| `java/util/stream/IntStream$1` | 3 | 0 |
| `java/util/stream/IntStream$2` | 5 | 0 |
| `java/util/stream/IntStream$IntMapMultiConsumer` | 1 | 0 |
| `java/util/stream/LongPipeline` | 60 | 0 |
| `java/util/stream/LongPipeline$1` | 2 | 0 |
| `java/util/stream/LongPipeline$1$1` | 2 | 0 |
| `java/util/stream/LongPipeline$10` | 2 | 0 |
| `java/util/stream/LongPipeline$10$1` | 2 | 0 |
| `java/util/stream/LongPipeline$2` | 2 | 0 |
| `java/util/stream/LongPipeline$2$1` | 2 | 0 |
| `java/util/stream/LongPipeline$3` | 2 | 0 |
| `java/util/stream/LongPipeline$3$1` | 2 | 0 |
| `java/util/stream/LongPipeline$4` | 2 | 0 |
| `java/util/stream/LongPipeline$4$1` | 2 | 0 |
| `java/util/stream/LongPipeline$5` | 2 | 0 |
| `java/util/stream/LongPipeline$5$1` | 2 | 0 |
| `java/util/stream/LongPipeline$6` | 2 | 0 |
| `java/util/stream/LongPipeline$6$1` | 4 | 0 |
| `java/util/stream/LongPipeline$7` | 2 | 0 |
| `java/util/stream/LongPipeline$7$1` | 3 | 0 |
| `java/util/stream/LongPipeline$8` | 2 | 0 |
| `java/util/stream/LongPipeline$9` | 2 | 0 |
| `java/util/stream/LongPipeline$9$1` | 3 | 0 |
| `java/util/stream/LongPipeline$Head` | 12 | 0 |
| `java/util/stream/LongPipeline$StatefulOp` | 10 | 0 |
| `java/util/stream/LongPipeline$StatelessOp` | 9 | 0 |
| `java/util/stream/LongStream` | 52 | 0 |
| `java/util/stream/LongStream$1` | 3 | 0 |
| `java/util/stream/LongStream$2` | 5 | 0 |
| `java/util/stream/LongStream$LongMapMultiConsumer` | 1 | 0 |
| `java/util/stream/MatchOps` | 9 | 0 |
| `java/util/stream/MatchOps$1MatchSink` | 2 | 0 |
| `java/util/stream/MatchOps$2MatchSink` | 2 | 0 |
| `java/util/stream/MatchOps$3MatchSink` | 2 | 0 |
| `java/util/stream/MatchOps$4MatchSink` | 2 | 0 |
| `java/util/stream/MatchOps$BooleanTerminalSink` | 3 | 0 |
| `java/util/stream/MatchOps$MatchOp` | 7 | 0 |
| `java/util/stream/MatchOps$MatchTask` | 8 | 0 |
| `java/util/stream/Node` | 10 | 0 |
| `java/util/stream/Node$Builder` | 1 | 0 |
| `java/util/stream/Node$Builder$OfDouble` | 2 | 0 |
| `java/util/stream/Node$Builder$OfInt` | 2 | 0 |
| `java/util/stream/Node$Builder$OfLong` | 2 | 0 |
| `java/util/stream/Node$OfDouble` | 10 | 0 |
| `java/util/stream/Node$OfInt` | 10 | 0 |
| `java/util/stream/Node$OfLong` | 10 | 0 |
| `java/util/stream/Node$OfPrimitive` | 11 | 0 |
| `java/util/stream/Nodes` | 27 | 0 |
| `java/util/stream/Nodes$AbstractConcNode` | 4 | 0 |
| `java/util/stream/Nodes$ArrayNode` | 8 | 0 |
| `java/util/stream/Nodes$CollectionNode` | 7 | 0 |
| `java/util/stream/Nodes$CollectorTask` | 7 | 0 |
| `java/util/stream/Nodes$CollectorTask$OfDouble` | 3 | 0 |
| `java/util/stream/Nodes$CollectorTask$OfInt` | 3 | 0 |
| `java/util/stream/Nodes$CollectorTask$OfLong` | 3 | 0 |
| `java/util/stream/Nodes$CollectorTask$OfRef` | 4 | 0 |
| `java/util/stream/Nodes$ConcNode` | 7 | 0 |
| `java/util/stream/Nodes$ConcNode$OfDouble` | 4 | 0 |
| `java/util/stream/Nodes$ConcNode$OfInt` | 4 | 0 |
| `java/util/stream/Nodes$ConcNode$OfLong` | 4 | 0 |
| `java/util/stream/Nodes$ConcNode$OfPrimitive` | 6 | 0 |
| `java/util/stream/Nodes$DoubleArrayNode` | 13 | 0 |
| `java/util/stream/Nodes$DoubleFixedNodeBuilder` | 8 | 0 |
| `java/util/stream/Nodes$DoubleSpinedNodeBuilder` | 16 | 0 |
| `java/util/stream/Nodes$FixedNodeBuilder` | 7 | 0 |
| `java/util/stream/Nodes$IntArrayNode` | 13 | 0 |
| `java/util/stream/Nodes$IntFixedNodeBuilder` | 8 | 0 |
| `java/util/stream/Nodes$IntSpinedNodeBuilder` | 16 | 0 |
| `java/util/stream/Nodes$InternalNodeSpliterator` | 7 | 0 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfDouble` | 4 | 0 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfInt` | 4 | 0 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfLong` | 4 | 0 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfPrimitive` | 4 | 0 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfRef` | 3 | 0 |
| `java/util/stream/Nodes$LongArrayNode` | 13 | 0 |
| `java/util/stream/Nodes$LongFixedNodeBuilder` | 8 | 0 |
| `java/util/stream/Nodes$LongSpinedNodeBuilder` | 16 | 0 |
| `java/util/stream/Nodes$SizedCollectorTask` | 6 | 0 |
| `java/util/stream/Nodes$SizedCollectorTask$OfDouble` | 5 | 0 |
| `java/util/stream/Nodes$SizedCollectorTask$OfInt` | 5 | 0 |
| `java/util/stream/Nodes$SizedCollectorTask$OfLong` | 5 | 0 |
| `java/util/stream/Nodes$SizedCollectorTask$OfRef` | 5 | 0 |
| `java/util/stream/Nodes$SpinedNodeBuilder` | 10 | 0 |
| `java/util/stream/Nodes$ToArrayTask` | 5 | 0 |
| `java/util/stream/Nodes$ToArrayTask$OfDouble` | 1 | 0 |
| `java/util/stream/Nodes$ToArrayTask$OfInt` | 1 | 0 |
| `java/util/stream/Nodes$ToArrayTask$OfLong` | 1 | 0 |
| `java/util/stream/Nodes$ToArrayTask$OfPrimitive` | 5 | 0 |
| `java/util/stream/Nodes$ToArrayTask$OfRef` | 5 | 0 |
| `java/util/stream/PipelineHelper` | 11 | 0 |
| `java/util/stream/ReduceOps` | 18 | 0 |
| `java/util/stream/ReduceOps$1` | 3 | 0 |
| `java/util/stream/ReduceOps$10` | 3 | 0 |
| `java/util/stream/ReduceOps$10ReducingSink` | 5 | 0 |
| `java/util/stream/ReduceOps$11` | 3 | 0 |
| `java/util/stream/ReduceOps$11ReducingSink` | 7 | 0 |
| `java/util/stream/ReduceOps$12` | 3 | 0 |
| `java/util/stream/ReduceOps$12ReducingSink` | 7 | 0 |
| `java/util/stream/ReduceOps$13` | 8 | 0 |
| `java/util/stream/ReduceOps$13ReducingSink` | 5 | 0 |
| `java/util/stream/ReduceOps$14` | 3 | 0 |
| `java/util/stream/ReduceOps$15` | 3 | 0 |
| `java/util/stream/ReduceOps$16` | 3 | 0 |
| `java/util/stream/ReduceOps$17` | 8 | 0 |
| `java/util/stream/ReduceOps$1ReducingSink` | 5 | 0 |
| `java/util/stream/ReduceOps$2` | 3 | 0 |
| `java/util/stream/ReduceOps$2ReducingSink` | 7 | 0 |
| `java/util/stream/ReduceOps$3` | 4 | 0 |
| `java/util/stream/ReduceOps$3ReducingSink` | 5 | 0 |
| `java/util/stream/ReduceOps$4` | 3 | 0 |
| `java/util/stream/ReduceOps$4ReducingSink` | 5 | 0 |
| `java/util/stream/ReduceOps$5` | 8 | 0 |
| `java/util/stream/ReduceOps$5ReducingSink` | 7 | 0 |
| `java/util/stream/ReduceOps$6` | 3 | 0 |
| `java/util/stream/ReduceOps$6ReducingSink` | 7 | 0 |
| `java/util/stream/ReduceOps$7` | 3 | 0 |
| `java/util/stream/ReduceOps$7ReducingSink` | 5 | 0 |
| `java/util/stream/ReduceOps$8` | 3 | 0 |
| `java/util/stream/ReduceOps$8ReducingSink` | 7 | 0 |
| `java/util/stream/ReduceOps$9` | 8 | 0 |
| `java/util/stream/ReduceOps$9ReducingSink` | 7 | 0 |
| `java/util/stream/ReduceOps$AccumulatingSink` | 1 | 0 |
| `java/util/stream/ReduceOps$Box` | 2 | 0 |
| `java/util/stream/ReduceOps$CountingSink` | 6 | 0 |
| `java/util/stream/ReduceOps$CountingSink$OfDouble` | 4 | 0 |
| `java/util/stream/ReduceOps$CountingSink$OfInt` | 4 | 0 |
| `java/util/stream/ReduceOps$CountingSink$OfLong` | 4 | 0 |
| `java/util/stream/ReduceOps$CountingSink$OfRef` | 4 | 0 |
| `java/util/stream/ReduceOps$ReduceOp` | 5 | 0 |
| `java/util/stream/ReduceOps$ReduceTask` | 7 | 0 |
| `java/util/stream/ReferencePipeline` | 53 | 0 |
| `java/util/stream/ReferencePipeline$1` | 2 | 0 |
| `java/util/stream/ReferencePipeline$10` | 2 | 0 |
| `java/util/stream/ReferencePipeline$10$1` | 4 | 0 |
| `java/util/stream/ReferencePipeline$11` | 2 | 0 |
| `java/util/stream/ReferencePipeline$11$1` | 3 | 0 |
| `java/util/stream/ReferencePipeline$12` | 2 | 0 |
| `java/util/stream/ReferencePipeline$12$1` | 3 | 0 |
| `java/util/stream/ReferencePipeline$13` | 2 | 0 |
| `java/util/stream/ReferencePipeline$13$1` | 3 | 0 |
| `java/util/stream/ReferencePipeline$14` | 2 | 0 |
| `java/util/stream/ReferencePipeline$14$1` | 3 | 0 |
| `java/util/stream/ReferencePipeline$15` | 2 | 0 |
| `java/util/stream/ReferencePipeline$15$1` | 2 | 0 |
| `java/util/stream/ReferencePipeline$2` | 2 | 0 |
| `java/util/stream/ReferencePipeline$2$1` | 3 | 0 |
| `java/util/stream/ReferencePipeline$3` | 2 | 0 |
| `java/util/stream/ReferencePipeline$3$1` | 2 | 0 |
| `java/util/stream/ReferencePipeline$4` | 2 | 0 |
| `java/util/stream/ReferencePipeline$4$1` | 2 | 0 |
| `java/util/stream/ReferencePipeline$5` | 2 | 0 |
| `java/util/stream/ReferencePipeline$5$1` | 2 | 0 |
| `java/util/stream/ReferencePipeline$6` | 2 | 0 |
| `java/util/stream/ReferencePipeline$6$1` | 2 | 0 |
| `java/util/stream/ReferencePipeline$7` | 2 | 0 |
| `java/util/stream/ReferencePipeline$7$1` | 4 | 0 |
| `java/util/stream/ReferencePipeline$8` | 2 | 0 |
| `java/util/stream/ReferencePipeline$8$1` | 4 | 0 |
| `java/util/stream/ReferencePipeline$9` | 2 | 0 |
| `java/util/stream/ReferencePipeline$9$1` | 4 | 0 |
| `java/util/stream/ReferencePipeline$Head` | 7 | 0 |
| `java/util/stream/ReferencePipeline$StatefulOp` | 5 | 0 |
| `java/util/stream/ReferencePipeline$StatelessOp` | 4 | 0 |
| `java/util/stream/Sink` | 6 | 0 |
| `java/util/stream/Sink$ChainedDouble` | 4 | 0 |
| `java/util/stream/Sink$ChainedInt` | 4 | 0 |
| `java/util/stream/Sink$ChainedLong` | 4 | 0 |
| `java/util/stream/Sink$ChainedReference` | 4 | 0 |
| `java/util/stream/SliceOps` | 10 | 0 |
| `java/util/stream/SliceOps$1` | 6 | 0 |
| `java/util/stream/SliceOps$1$1` | 4 | 0 |
| `java/util/stream/SliceOps$2` | 7 | 0 |
| `java/util/stream/SliceOps$2$1` | 4 | 0 |
| `java/util/stream/SliceOps$3` | 7 | 0 |
| `java/util/stream/SliceOps$3$1` | 4 | 0 |
| `java/util/stream/SliceOps$4` | 7 | 0 |
| `java/util/stream/SliceOps$4$1` | 4 | 0 |
| `java/util/stream/SliceOps$SliceTask` | 13 | 0 |
| `java/util/stream/SortedOps` | 6 | 0 |
| `java/util/stream/SortedOps$AbstractDoubleSortingSink` | 2 | 0 |
| `java/util/stream/SortedOps$AbstractIntSortingSink` | 2 | 0 |
| `java/util/stream/SortedOps$AbstractLongSortingSink` | 2 | 0 |
| `java/util/stream/SortedOps$AbstractRefSortingSink` | 2 | 0 |
| `java/util/stream/SortedOps$DoubleSortingSink` | 4 | 0 |
| `java/util/stream/SortedOps$IntSortingSink` | 4 | 0 |
| `java/util/stream/SortedOps$LongSortingSink` | 4 | 0 |
| `java/util/stream/SortedOps$OfDouble` | 3 | 0 |
| `java/util/stream/SortedOps$OfInt` | 3 | 0 |
| `java/util/stream/SortedOps$OfLong` | 3 | 0 |
| `java/util/stream/SortedOps$OfRef` | 4 | 0 |
| `java/util/stream/SortedOps$RefSortingSink` | 4 | 0 |
| `java/util/stream/SortedOps$SizedDoubleSortingSink` | 4 | 0 |
| `java/util/stream/SortedOps$SizedIntSortingSink` | 4 | 0 |
| `java/util/stream/SortedOps$SizedLongSortingSink` | 4 | 0 |
| `java/util/stream/SortedOps$SizedRefSortingSink` | 4 | 0 |
| `java/util/stream/SpinedBuffer` | 15 | 0 |
| `java/util/stream/SpinedBuffer$1Splitr` | 7 | 0 |
| `java/util/stream/SpinedBuffer$OfDouble` | 18 | 0 |
| `java/util/stream/SpinedBuffer$OfDouble$1Splitr` | 10 | 0 |
| `java/util/stream/SpinedBuffer$OfInt` | 18 | 0 |
| `java/util/stream/SpinedBuffer$OfInt$1Splitr` | 10 | 0 |
| `java/util/stream/SpinedBuffer$OfLong` | 18 | 0 |
| `java/util/stream/SpinedBuffer$OfLong$1Splitr` | 10 | 0 |
| `java/util/stream/SpinedBuffer$OfPrimitive` | 18 | 0 |
| `java/util/stream/SpinedBuffer$OfPrimitive$BaseSpliterator` | 11 | 0 |
| `java/util/stream/Stream` | 52 | 0 |
| `java/util/stream/Stream$1` | 2 | 0 |
| `java/util/stream/Stream$2` | 3 | 0 |
| `java/util/stream/StreamOpFlag` | 21 | 0 |
| `java/util/stream/StreamOpFlag$MaskBuilder` | 6 | 0 |
| `java/util/stream/StreamOpFlag$Type` | 5 | 0 |
| `java/util/stream/StreamShape` | 5 | 0 |
| `java/util/stream/StreamSpliterators$AbstractWrappingSpliterator` | 13 | 0 |
| `java/util/stream/StreamSpliterators$ArrayBuffer` | 2 | 0 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfDouble` | 4 | 0 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfInt` | 4 | 0 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfLong` | 4 | 0 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfPrimitive` | 3 | 0 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfRef` | 3 | 0 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator` | 10 | 0 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfDouble` | 4 | 0 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfInt` | 4 | 0 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfLong` | 4 | 0 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfPrimitive` | 4 | 0 |
| `java/util/stream/StreamSpliterators$DistinctSpliterator` | 12 | 0 |
| `java/util/stream/StreamSpliterators$DoubleWrappingSpliterator` | 12 | 0 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator` | 3 | 0 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfDouble` | 6 | 0 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfInt` | 6 | 0 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfLong` | 6 | 0 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef` | 3 | 0 |
| `java/util/stream/StreamSpliterators$IntWrappingSpliterator` | 12 | 0 |
| `java/util/stream/StreamSpliterators$LongWrappingSpliterator` | 12 | 0 |
| `java/util/stream/StreamSpliterators$SliceSpliterator` | 6 | 0 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfDouble` | 10 | 0 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfInt` | 10 | 0 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfLong` | 10 | 0 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfPrimitive` | 6 | 0 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfRef` | 7 | 0 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator` | 9 | 0 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfDouble` | 12 | 0 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfInt` | 12 | 0 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfLong` | 12 | 0 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfPrimitive` | 7 | 0 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfRef` | 6 | 0 |
| `java/util/stream/StreamSpliterators$WrappingSpliterator` | 8 | 0 |
| `java/util/stream/StreamSupport` | 9 | 0 |
| `java/util/stream/Streams` | 3 | 0 |
| `java/util/stream/Streams$1` | 2 | 0 |
| `java/util/stream/Streams$2` | 2 | 0 |
| `java/util/stream/Streams$AbstractStreamBuilderImpl` | 4 | 0 |
| `java/util/stream/Streams$ConcatSpliterator` | 7 | 0 |
| `java/util/stream/Streams$ConcatSpliterator$OfDouble` | 4 | 0 |
| `java/util/stream/Streams$ConcatSpliterator$OfInt` | 4 | 0 |
| `java/util/stream/Streams$ConcatSpliterator$OfLong` | 4 | 0 |
| `java/util/stream/Streams$ConcatSpliterator$OfPrimitive` | 4 | 0 |
| `java/util/stream/Streams$ConcatSpliterator$OfRef` | 1 | 0 |
| `java/util/stream/Streams$DoubleStreamBuilderImpl` | 10 | 0 |
| `java/util/stream/Streams$IntStreamBuilderImpl` | 10 | 0 |
| `java/util/stream/Streams$LongStreamBuilderImpl` | 10 | 0 |
| `java/util/stream/Streams$RangeIntSpliterator` | 13 | 0 |
| `java/util/stream/Streams$RangeLongSpliterator` | 14 | 0 |
| `java/util/stream/Streams$StreamBuilderImpl` | 7 | 0 |
| `java/util/stream/TerminalOp` | 4 | 0 |
| `java/util/stream/Tripwire` | 4 | 0 |
| `java/util/stream/WhileOps` | 10 | 0 |
| `java/util/stream/WhileOps$1` | 4 | 0 |
| `java/util/stream/WhileOps$1$1` | 4 | 0 |
| `java/util/stream/WhileOps$1Op` | 5 | 0 |
| `java/util/stream/WhileOps$1Op$1OpSink` | 3 | 0 |
| `java/util/stream/WhileOps$2` | 5 | 0 |
| `java/util/stream/WhileOps$2$1` | 4 | 0 |
| `java/util/stream/WhileOps$2Op` | 6 | 0 |
| `java/util/stream/WhileOps$2Op$1OpSink` | 3 | 0 |
| `java/util/stream/WhileOps$3` | 5 | 0 |
| `java/util/stream/WhileOps$3$1` | 4 | 0 |
| `java/util/stream/WhileOps$3Op` | 6 | 0 |
| `java/util/stream/WhileOps$3Op$1OpSink` | 3 | 0 |
| `java/util/stream/WhileOps$4` | 5 | 0 |
| `java/util/stream/WhileOps$4$1` | 4 | 0 |
| `java/util/stream/WhileOps$4Op` | 6 | 0 |
| `java/util/stream/WhileOps$4Op$1OpSink` | 3 | 0 |
| `java/util/stream/WhileOps$DropWhileOp` | 1 | 0 |
| `java/util/stream/WhileOps$DropWhileSink` | 1 | 0 |
| `java/util/stream/WhileOps$DropWhileTask` | 10 | 0 |
| `java/util/stream/WhileOps$TakeWhileTask` | 11 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator` | 9 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble` | 5 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble$Dropping` | 8 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble$Taking` | 9 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt` | 5 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Dropping` | 8 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Taking` | 9 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong` | 5 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong$Dropping` | 8 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong$Taking` | 9 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef` | 3 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Dropping` | 4 | 0 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking` | 5 | 0 |
| `java/util/zip/CRC32` | 14 | 3 |
| `java/util/zip/DataFormatException` | 2 | 0 |
| `java/util/zip/Inflater` | 36 | 11 |
| `java/util/zip/Inflater$InflaterZStreamRef` | 4 | 0 |
| `java/util/zip/InflaterInputStream` | 13 | 0 |
| `java/util/zip/ZipCoder` | 16 | 0 |
| `java/util/zip/ZipCoder$Comparison` | 5 | 0 |
| `java/util/zip/ZipCoder$UTF8ZipCoder` | 7 | 0 |
| `java/util/zip/ZipEntry` | 30 | 0 |
| `java/util/zip/ZipException` | 2 | 0 |
| `java/util/zip/ZipFile` | 29 | 0 |
| `java/util/zip/ZipFile$CleanableResource` | 5 | 0 |
| `java/util/zip/ZipFile$EntrySpliterator` | 2 | 0 |
| `java/util/zip/ZipFile$InflaterCleanupAction` | 2 | 0 |
| `java/util/zip/ZipFile$Source` | 26 | 0 |
| `java/util/zip/ZipFile$Source$End` | 1 | 0 |
| `java/util/zip/ZipFile$Source$Key` | 3 | 0 |
| `java/util/zip/ZipFile$ZipEntryIterator` | 8 | 0 |
| `java/util/zip/ZipFile$ZipFileInflaterInputStream` | 5 | 0 |
| `java/util/zip/ZipFile$ZipFileInputStream` | 9 | 0 |
| `java/util/zip/ZipInputStream` | 19 | 0 |
| `java/util/zip/ZipUtils` | 66 | 0 |
| `javax/crypto/Cipher` | 58 | 0 |
| `javax/crypto/Cipher$Transform` | 8 | 0 |
| `javax/crypto/CipherSpi` | 23 | 0 |
| `javax/crypto/CryptoPermission` | 20 | 0 |
| `javax/crypto/CryptoPermissionCollection` | 4 | 0 |
| `javax/crypto/CryptoPermissions` | 14 | 0 |
| `javax/crypto/CryptoPolicyParser` | 11 | 0 |
| `javax/crypto/CryptoPolicyParser$CryptoPermissionEntry` | 3 | 0 |
| `javax/crypto/CryptoPolicyParser$GrantEntry` | 2 | 0 |
| `javax/crypto/CryptoPolicyParser$ParsingException` | 3 | 0 |
| `javax/crypto/ExemptionMechanism` | 14 | 0 |
| `javax/crypto/ExemptionMechanismException` | 2 | 0 |
| `javax/crypto/ExemptionMechanismSpi` | 7 | 0 |
| `javax/crypto/JceSecurity` | 15 | 0 |
| `javax/crypto/JceSecurity$2` | 3 | 0 |
| `javax/crypto/JceSecurity$3` | 3 | 0 |
| `javax/crypto/JceSecurity$WeakIdentityWrapper` | 3 | 0 |
| `javax/crypto/JceSecurityManager` | 11 | 0 |
| `javax/crypto/NoSuchPaddingException` | 2 | 0 |
| `javax/crypto/NullCipher` | 1 | 0 |
| `javax/crypto/NullCipherSpi` | 15 | 0 |
| `javax/crypto/PermissionsEnumerator` | 5 | 0 |
| `javax/crypto/ProviderVerifier` | 7 | 0 |
| `javax/crypto/SecretKey` | 0 | 0 |
| `javax/crypto/ShortBufferException` | 2 | 0 |
| `javax/crypto/interfaces/DHKey` | 1 | 0 |
| `javax/crypto/interfaces/DHPublicKey` | 1 | 0 |
| `javax/crypto/spec/DHParameterSpec` | 5 | 0 |
| `javax/crypto/spec/DHPublicKeySpec` | 4 | 0 |
| `javax/crypto/spec/PBEParameterSpec` | 5 | 0 |
| `javax/crypto/spec/RC2ParameterSpec` | 7 | 0 |
| `javax/crypto/spec/RC5ParameterSpec` | 9 | 0 |
| `javax/net/ssl/SNIHostName` | 8 | 0 |
| `javax/net/ssl/SNIHostName$SNIHostNameMatcher` | 2 | 0 |
| `javax/net/ssl/SNIMatcher` | 3 | 0 |
| `javax/net/ssl/SNIServerName` | 7 | 0 |
| `javax/net/ssl/SSLSession` | 21 | 0 |
| `javax/security/auth/AuthPermission` | 2 | 0 |
| `javax/security/auth/PrivateCredentialPermission` | 14 | 0 |
| `javax/security/auth/PrivateCredentialPermission$CredOwner` | 3 | 0 |
| `javax/security/auth/Subject` | 28 | 0 |
| `javax/security/auth/Subject$1` | 3 | 0 |
| `javax/security/auth/Subject$2` | 3 | 0 |
| `javax/security/auth/Subject$ClassSet` | 5 | 0 |
| `javax/security/auth/Subject$ClassSet$1` | 2 | 0 |
| `javax/security/auth/Subject$SecureSet` | 20 | 0 |
| `javax/security/auth/Subject$SecureSet$1` | 4 | 0 |
| `javax/security/auth/Subject$SecureSet$2` | 2 | 0 |
| `javax/security/auth/Subject$SecureSet$3` | 2 | 0 |
| `javax/security/auth/Subject$SecureSet$4` | 2 | 0 |
| `javax/security/auth/Subject$SecureSet$5` | 2 | 0 |
| `javax/security/auth/Subject$SecureSet$6` | 2 | 0 |
| `javax/security/auth/SubjectDomainCombiner` | 7 | 0 |
| `javax/security/auth/SubjectDomainCombiner$1` | 3 | 0 |
| `javax/security/auth/SubjectDomainCombiner$2` | 3 | 0 |
| `javax/security/auth/SubjectDomainCombiner$WeakKeyValueMap` | 3 | 0 |
| `javax/security/auth/callback/Callback` | 0 | 0 |
| `javax/security/auth/callback/CallbackHandler` | 1 | 0 |
| `javax/security/auth/callback/PasswordCallback` | 9 | 0 |
| `javax/security/auth/x500/X500Principal` | 14 | 0 |
| `sun/invoke/util/BytecodeDescriptor` | 10 | 0 |
| `sun/invoke/util/BytecodeName` | 22 | 0 |
| `sun/invoke/util/ValueConversions` | 101 | 0 |
| `sun/invoke/util/ValueConversions$WrapperCache` | 3 | 0 |
| `sun/invoke/util/VerifyAccess` | 16 | 0 |
| `sun/invoke/util/VerifyAccess$1` | 3 | 0 |
| `sun/invoke/util/VerifyType` | 4 | 0 |
| `sun/invoke/util/Wrapper` | 56 | 0 |
| `sun/net/ApplicationProxy` | 2 | 0 |
| `sun/net/InetAddressCachePolicy` | 9 | 0 |
| `sun/net/InetAddressCachePolicy$1` | 3 | 0 |
| `sun/net/NetHooks` | 4 | 0 |
| `sun/net/NetHooks$Provider` | 3 | 0 |
| `sun/net/NetProperties` | 6 | 0 |
| `sun/net/PlatformSocketImpl` | 0 | 0 |
| `sun/net/PortConfig` | 6 | 2 |
| `sun/net/ResolverProviderConfiguration` | 3 | 0 |
| `sun/net/ResourceManager` | 4 | 0 |
| `sun/net/SocksProxy` | 3 | 0 |
| `sun/net/ext/ExtendedSocketOptions` | 16 | 0 |
| `sun/net/ext/ExtendedSocketOptions$NoExtendedSocketOptions` | 3 | 0 |
| `sun/net/spi/DefaultProxySelector` | 11 | 2 |
| `sun/net/spi/DefaultProxySelector$2` | 3 | 0 |
| `sun/net/spi/DefaultProxySelector$3` | 3 | 0 |
| `sun/net/util/IPAddressUtil` | 36 | 0 |
| `sun/net/util/SocketExceptions` | 6 | 0 |
| `sun/net/util/SocketExceptions$1` | 3 | 0 |
| `sun/net/util/URLUtil` | 4 | 0 |
| `sun/net/www/MessageHeader` | 31 | 0 |
| `sun/net/www/MessageHeader$HeaderIterator` | 5 | 0 |
| `sun/net/www/MimeEntry` | 23 | 0 |
| `sun/net/www/MimeTable` | 24 | 0 |
| `sun/net/www/ParseUtil` | 22 | 0 |
| `sun/net/www/URLConnection` | 20 | 0 |
| `sun/net/www/protocol/file/FileURLConnection` | 15 | 0 |
| `sun/net/www/protocol/jar/Handler` | 12 | 0 |
| `sun/net/www/protocol/jar/JarFileFactory` | 13 | 0 |
| `sun/net/www/protocol/jar/JarURLConnection` | 24 | 0 |
| `sun/net/www/protocol/jar/JarURLConnection$JarURLInputStream` | 2 | 0 |
| `sun/net/www/protocol/jar/URLJarFile` | 14 | 0 |
| `sun/net/www/protocol/jar/URLJarFile$1` | 3 | 0 |
| `sun/net/www/protocol/jar/URLJarFile$URLJarFileCloseController` | 1 | 0 |
| `sun/net/www/protocol/jar/URLJarFile$URLJarFileEntry` | 4 | 0 |
| `sun/net/www/protocol/jar/URLJarFileCallBack` | 1 | 0 |
| `sun/nio/ch/AllocatedNativeObject` | 2 | 0 |
| `sun/nio/ch/AsynchronousChannelGroupImpl` | 28 | 0 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$1` | 2 | 0 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$2` | 3 | 0 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$3` | 3 | 0 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$4` | 2 | 0 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$4$1` | 3 | 0 |
| `sun/nio/ch/AsynchronousFileChannelImpl` | 21 | 0 |
| `sun/nio/ch/Cancellable` | 1 | 0 |
| `sun/nio/ch/ChannelInputStream` | 12 | 0 |
| `sun/nio/ch/ChannelOutputStream` | 6 | 0 |
| `sun/nio/ch/CompletedFuture` | 9 | 0 |
| `sun/nio/ch/DefaultPollerProvider` | 3 | 0 |
| `sun/nio/ch/DirectBuffer` | 3 | 0 |
| `sun/nio/ch/DummySocketImpl` | 23 | 0 |
| `sun/nio/ch/FileChannelImpl` | 46 | 0 |
| `sun/nio/ch/FileChannelImpl$1` | 2 | 0 |
| `sun/nio/ch/FileChannelImpl$2` | 5 | 0 |
| `sun/nio/ch/FileChannelImpl$3` | 5 | 0 |
| `sun/nio/ch/FileChannelImpl$Closer` | 2 | 0 |
| `sun/nio/ch/FileChannelImpl$DefaultUnmapper` | 4 | 0 |
| `sun/nio/ch/FileChannelImpl$SyncUnmapper` | 4 | 0 |
| `sun/nio/ch/FileChannelImpl$Unmapper` | 9 | 0 |
| `sun/nio/ch/FileDispatcher` | 18 | 0 |
| `sun/nio/ch/FileKey` | 7 | 2 |
| `sun/nio/ch/FileLockImpl` | 6 | 0 |
| `sun/nio/ch/FileLockTable` | 9 | 0 |
| `sun/nio/ch/FileLockTable$FileLockReference` | 2 | 0 |
| `sun/nio/ch/Groupable` | 1 | 0 |
| `sun/nio/ch/IOStatus` | 7 | 0 |
| `sun/nio/ch/IOUtil` | 41 | 12 |
| `sun/nio/ch/IOUtil$LinkedRunnable` | 8 | 0 |
| `sun/nio/ch/IOUtil$Releaser` | 8 | 0 |
| `sun/nio/ch/IOVecWrapper` | 12 | 0 |
| `sun/nio/ch/IOVecWrapper$Deallocator` | 2 | 0 |
| `sun/nio/ch/Interruptible` | 1 | 0 |
| `sun/nio/ch/Invoker` | 15 | 0 |
| `sun/nio/ch/Invoker$2` | 2 | 0 |
| `sun/nio/ch/Invoker$3` | 2 | 0 |
| `sun/nio/ch/Invoker$GroupAndInvokeCount` | 6 | 0 |
| `sun/nio/ch/KQueue` | 15 | 7 |
| `sun/nio/ch/KQueuePoller` | 5 | 0 |
| `sun/nio/ch/NativeDispatcher` | 13 | 0 |
| `sun/nio/ch/NativeObject` | 26 | 0 |
| `sun/nio/ch/NativeThread` | 12 | 4 |
| `sun/nio/ch/NativeThreadSet` | 5 | 0 |
| `sun/nio/ch/Net` | 95 | 40 |
| `sun/nio/ch/Net$2` | 3 | 0 |
| `sun/nio/ch/NioSocketImpl` | 56 | 0 |
| `sun/nio/ch/NioSocketImpl$1` | 5 | 0 |
| `sun/nio/ch/NioSocketImpl$2` | 4 | 0 |
| `sun/nio/ch/OptionKey` | 3 | 0 |
| `sun/nio/ch/PendingFuture` | 21 | 0 |
| `sun/nio/ch/Poller` | 33 | 0 |
| `sun/nio/ch/Poller$Request` | 4 | 0 |
| `sun/nio/ch/PollerProvider` | 5 | 0 |
| `sun/nio/ch/SelChImpl` | 8 | 0 |
| `sun/nio/ch/SelectionKeyImpl` | 26 | 0 |
| `sun/nio/ch/SelectorImpl` | 24 | 0 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl` | 12 | 0 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl$1` | 2 | 0 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl$2` | 2 | 0 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl$3` | 2 | 0 |
| `sun/nio/ch/SinkChannelImpl` | 22 | 0 |
| `sun/nio/ch/SocketAdaptor` | 53 | 0 |
| `sun/nio/ch/SocketChannelImpl` | 69 | 0 |
| `sun/nio/ch/SocketInputStream` | 7 | 0 |
| `sun/nio/ch/SocketOptionRegistry` | 2 | 0 |
| `sun/nio/ch/SocketOptionRegistry$RegistryKey` | 3 | 0 |
| `sun/nio/ch/SocketOutputStream` | 5 | 0 |
| `sun/nio/ch/Streams` | 3 | 0 |
| `sun/nio/ch/ThreadPool` | 14 | 0 |
| `sun/nio/ch/UnixDomainSockets` | 23 | 6 |
| `sun/nio/ch/UnixDomainSocketsUtil` | 4 | 0 |
| `sun/nio/ch/Util` | 25 | 0 |
| `sun/nio/ch/Util$2` | 17 | 0 |
| `sun/nio/ch/Util$3` | 3 | 0 |
| `sun/nio/ch/Util$4` | 3 | 0 |
| `sun/nio/ch/Util$BufferCache` | 8 | 0 |
| `sun/nio/cs/ArrayDecoder` | 4 | 0 |
| `sun/nio/cs/ArrayEncoder` | 4 | 0 |
| `sun/nio/cs/CESU_8` | 6 | 0 |
| `sun/nio/cs/CESU_8$Decoder` | 17 | 0 |
| `sun/nio/cs/CESU_8$Encoder` | 11 | 0 |
| `sun/nio/cs/HistoricallyNamedCharset` | 1 | 0 |
| `sun/nio/cs/ISO_8859_1` | 6 | 0 |
| `sun/nio/cs/ISO_8859_1$Decoder` | 5 | 0 |
| `sun/nio/cs/ISO_8859_1$Encoder` | 10 | 0 |
| `sun/nio/cs/ISO_8859_15` | 5 | 0 |
| `sun/nio/cs/ISO_8859_16` | 5 | 0 |
| `sun/nio/cs/MS1252` | 5 | 0 |
| `sun/nio/cs/SingleByte` | 4 | 0 |
| `sun/nio/cs/SingleByte$Decoder` | 12 | 0 |
| `sun/nio/cs/SingleByte$Encoder` | 12 | 0 |
| `sun/nio/cs/StandardCharsets` | 52 | 0 |
| `sun/nio/cs/StandardCharsets$1` | 5 | 0 |
| `sun/nio/cs/StandardCharsets$Aliases` | 2 | 0 |
| `sun/nio/cs/StandardCharsets$Cache` | 2 | 0 |
| `sun/nio/cs/StandardCharsets$Classes` | 2 | 0 |
| `sun/nio/cs/StreamDecoder` | 28 | 0 |
| `sun/nio/cs/StreamEncoder` | 32 | 0 |
| `sun/nio/cs/StringUTF16` | 3 | 0 |
| `sun/nio/cs/Surrogate$Parser` | 9 | 0 |
| `sun/nio/cs/ThreadLocalCoders` | 4 | 0 |
| `sun/nio/cs/ThreadLocalCoders$Cache` | 5 | 0 |
| `sun/nio/cs/US_ASCII` | 6 | 0 |
| `sun/nio/cs/US_ASCII$Decoder` | 4 | 0 |
| `sun/nio/cs/US_ASCII$Encoder` | 7 | 0 |
| `sun/nio/cs/UTF_16` | 5 | 0 |
| `sun/nio/cs/UTF_16$Decoder` | 1 | 0 |
| `sun/nio/cs/UTF_16$Encoder` | 1 | 0 |
| `sun/nio/cs/UTF_16BE` | 5 | 0 |
| `sun/nio/cs/UTF_16BE$Decoder` | 1 | 0 |
| `sun/nio/cs/UTF_16BE$Encoder` | 1 | 0 |
| `sun/nio/cs/UTF_16LE` | 5 | 0 |
| `sun/nio/cs/UTF_16LE$Decoder` | 1 | 0 |
| `sun/nio/cs/UTF_16LE$Encoder` | 1 | 0 |
| `sun/nio/cs/UTF_16LE_BOM` | 4 | 0 |
| `sun/nio/cs/UTF_16LE_BOM$Decoder` | 1 | 0 |
| `sun/nio/cs/UTF_16LE_BOM$Encoder` | 1 | 0 |
| `sun/nio/cs/UTF_32` | 5 | 0 |
| `sun/nio/cs/UTF_32BE` | 5 | 0 |
| `sun/nio/cs/UTF_32BE_BOM` | 5 | 0 |
| `sun/nio/cs/UTF_32Coder$Decoder` | 4 | 0 |
| `sun/nio/cs/UTF_32Coder$Encoder` | 4 | 0 |
| `sun/nio/cs/UTF_32LE` | 5 | 0 |
| `sun/nio/cs/UTF_32LE_BOM` | 5 | 0 |
| `sun/nio/cs/UTF_8` | 7 | 0 |
| `sun/nio/cs/UTF_8$Decoder` | 18 | 0 |
| `sun/nio/cs/UTF_8$Encoder` | 9 | 0 |
| `sun/nio/cs/Unicode` | 2 | 0 |
| `sun/nio/cs/UnicodeDecoder` | 5 | 0 |
| `sun/nio/cs/UnicodeEncoder` | 5 | 0 |
| `sun/nio/fs/AbstractBasicFileAttributeView` | 6 | 0 |
| `sun/nio/fs/AbstractBasicFileAttributeView$AttributesBuilder` | 5 | 0 |
| `sun/nio/fs/AbstractFileSystemProvider` | 12 | 0 |
| `sun/nio/fs/AbstractFileTypeDetector` | 7 | 0 |
| `sun/nio/fs/AbstractUserDefinedFileAttributeView` | 6 | 0 |
| `sun/nio/fs/AbstractWatchKey` | 9 | 0 |
| `sun/nio/fs/AbstractWatchKey$Event` | 5 | 0 |
| `sun/nio/fs/AbstractWatchService` | 12 | 0 |
| `sun/nio/fs/AbstractWatchService$1` | 3 | 0 |
| `sun/nio/fs/BasicFileAttributesHolder` | 2 | 0 |
| `sun/nio/fs/BsdFileAttributeViews` | 6 | 0 |
| `sun/nio/fs/BsdFileAttributeViews$Basic` | 2 | 0 |
| `sun/nio/fs/BsdFileAttributeViews$Posix` | 2 | 0 |
| `sun/nio/fs/BsdFileAttributeViews$Unix` | 2 | 0 |
| `sun/nio/fs/BsdFileStore` | 5 | 0 |
| `sun/nio/fs/BsdFileSystem` | 10 | 1 |
| `sun/nio/fs/BsdFileSystemProvider` | 7 | 0 |
| `sun/nio/fs/BsdNativeDispatcher` | 14 | 8 |
| `sun/nio/fs/BsdUserDefinedFileAttributeView` | 2 | 0 |
| `sun/nio/fs/Cancellable` | 9 | 0 |
| `sun/nio/fs/DefaultFileSystemProvider` | 4 | 0 |
| `sun/nio/fs/DynamicFileAttributeView` | 2 | 0 |
| `sun/nio/fs/ExtendedOptions$InternalOption` | 8 | 0 |
| `sun/nio/fs/ExtendedOptions$Wrapper` | 2 | 0 |
| `sun/nio/fs/FileOwnerAttributeViewImpl` | 7 | 0 |
| `sun/nio/fs/Globs` | 8 | 0 |
| `sun/nio/fs/MacOSXFileSystem` | 6 | 0 |
| `sun/nio/fs/MacOSXFileSystemProvider` | 5 | 0 |
| `sun/nio/fs/MacOSXNativeDispatcher` | 2 | 1 |
| `sun/nio/fs/MimeTypesFileTypeDetector` | 5 | 0 |
| `sun/nio/fs/MimeTypesFileTypeDetector$1` | 3 | 0 |
| `sun/nio/fs/NativeBuffer` | 9 | 0 |
| `sun/nio/fs/NativeBuffer$Deallocator` | 2 | 0 |
| `sun/nio/fs/NativeBuffers` | 8 | 0 |
| `sun/nio/fs/PollingWatchService` | 4 | 0 |
| `sun/nio/fs/PollingWatchService$1` | 2 | 0 |
| `sun/nio/fs/PollingWatchService$2` | 3 | 0 |
| `sun/nio/fs/PollingWatchService$3` | 3 | 0 |
| `sun/nio/fs/PollingWatchService$CacheEntry` | 4 | 0 |
| `sun/nio/fs/PollingWatchService$PollingWatchKey` | 8 | 0 |
| `sun/nio/fs/PollingWatchService$PollingWatchKey$1` | 2 | 0 |
| `sun/nio/fs/UTIFileTypeDetector` | 4 | 1 |
| `sun/nio/fs/UnixChannelFactory` | 6 | 0 |
| `sun/nio/fs/UnixChannelFactory$Flags` | 2 | 0 |
| `sun/nio/fs/UnixDirectoryStream` | 9 | 0 |
| `sun/nio/fs/UnixDirectoryStream$UnixDirectoryIterator` | 8 | 0 |
| `sun/nio/fs/UnixException` | 11 | 0 |
| `sun/nio/fs/UnixFileAttributeViews` | 5 | 0 |
| `sun/nio/fs/UnixFileAttributeViews$Basic` | 4 | 0 |
| `sun/nio/fs/UnixFileAttributeViews$Posix` | 17 | 0 |
| `sun/nio/fs/UnixFileAttributeViews$Unix` | 5 | 0 |
| `sun/nio/fs/UnixFileAttributes` | 30 | 0 |
| `sun/nio/fs/UnixFileAttributes$UnixAsBasicFileAttributes` | 11 | 0 |
| `sun/nio/fs/UnixFileKey` | 4 | 0 |
| `sun/nio/fs/UnixFileModeAttribute` | 3 | 0 |
| `sun/nio/fs/UnixFileStore` | 27 | 0 |
| `sun/nio/fs/UnixFileStore$1` | 3 | 0 |
| `sun/nio/fs/UnixFileStoreAttributes` | 6 | 0 |
| `sun/nio/fs/UnixFileSystem` | 40 | 1 |
| `sun/nio/fs/UnixFileSystem$1` | 2 | 0 |
| `sun/nio/fs/UnixFileSystem$2` | 2 | 0 |
| `sun/nio/fs/UnixFileSystem$3` | 2 | 0 |
| `sun/nio/fs/UnixFileSystem$4` | 2 | 0 |
| `sun/nio/fs/UnixFileSystem$FileStoreIterator` | 7 | 0 |
| `sun/nio/fs/UnixFileSystem$Flags` | 3 | 0 |
| `sun/nio/fs/UnixFileSystemProvider` | 39 | 0 |
| `sun/nio/fs/UnixFileSystemProvider$1` | 2 | 0 |
| `sun/nio/fs/UnixFileSystemProvider$2` | 2 | 0 |
| `sun/nio/fs/UnixMountEntry` | 8 | 0 |
| `sun/nio/fs/UnixNativeDispatcher` | 98 | 49 |
| `sun/nio/fs/UnixPath` | 55 | 0 |
| `sun/nio/fs/UnixSecureDirectoryStream` | 22 | 0 |
| `sun/nio/fs/UnixSecureDirectoryStream$BasicFileAttributeViewImpl` | 7 | 0 |
| `sun/nio/fs/UnixSecureDirectoryStream$PosixFileAttributeViewImpl` | 12 | 0 |
| `sun/nio/fs/UnixUriUtils` | 10 | 0 |
| `sun/nio/fs/UnixUserDefinedFileAttributeView` | 22 | 0 |
| `sun/nio/fs/UnixUserPrincipals` | 8 | 0 |
| `sun/nio/fs/UnixUserPrincipals$Group` | 1 | 0 |
| `sun/nio/fs/UnixUserPrincipals$User` | 8 | 0 |
| `sun/nio/fs/Util` | 9 | 0 |
| `sun/reflect/annotation/AnnotatedTypeFactory` | 5 | 0 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedArrayTypeImpl` | 7 | 0 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedParameterizedTypeImpl` | 7 | 0 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedTypeBaseImpl` | 17 | 0 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedTypeVariableImpl` | 5 | 0 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedWildcardTypeImpl` | 10 | 0 |
| `sun/reflect/annotation/AnnotationInvocationHandler` | 32 | 0 |
| `sun/reflect/annotation/AnnotationInvocationHandler$1` | 3 | 0 |
| `sun/reflect/annotation/AnnotationInvocationHandler$UnsafeAccessor` | 4 | 0 |
| `sun/reflect/annotation/AnnotationParser` | 42 | 0 |
| `sun/reflect/annotation/AnnotationParser$1` | 3 | 0 |
| `sun/reflect/annotation/AnnotationSupport` | 10 | 0 |
| `sun/reflect/annotation/AnnotationSupport$1` | 3 | 0 |
| `sun/reflect/annotation/AnnotationType` | 10 | 0 |
| `sun/reflect/annotation/AnnotationType$1` | 3 | 0 |
| `sun/reflect/annotation/AnnotationTypeMismatchExceptionProxy` | 4 | 0 |
| `sun/reflect/annotation/EnumConstantNotPresentExceptionProxy` | 3 | 0 |
| `sun/reflect/annotation/ExceptionProxy` | 2 | 0 |
| `sun/reflect/annotation/TypeAnnotation` | 7 | 0 |
| `sun/reflect/annotation/TypeAnnotation$LocationInfo` | 12 | 0 |
| `sun/reflect/annotation/TypeAnnotation$LocationInfo$Location` | 2 | 0 |
| `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTargetInfo` | 7 | 0 |
| `sun/reflect/annotation/TypeAnnotationParser` | 17 | 0 |
| `sun/reflect/annotation/TypeNotPresentExceptionProxy` | 5 | 0 |
| `sun/reflect/generics/factory/CoreReflectionFactory` | 21 | 0 |
| `sun/reflect/generics/factory/GenericsFactory` | 15 | 0 |
| `sun/reflect/generics/parser/SignatureParser` | 40 | 0 |
| `sun/reflect/generics/reflectiveObjects/GenericArrayTypeImpl` | 6 | 0 |
| `sun/reflect/generics/reflectiveObjects/LazyReflectiveObjectGenerator` | 4 | 0 |
| `sun/reflect/generics/reflectiveObjects/ParameterizedTypeImpl` | 10 | 0 |
| `sun/reflect/generics/reflectiveObjects/TypeVariableImpl` | 18 | 0 |
| `sun/reflect/generics/reflectiveObjects/WildcardTypeImpl` | 8 | 0 |
| `sun/reflect/generics/repository/AbstractRepository` | 5 | 0 |
| `sun/reflect/generics/repository/ClassRepository` | 9 | 0 |
| `sun/reflect/generics/repository/ConstructorRepository` | 8 | 0 |
| `sun/reflect/generics/repository/FieldRepository` | 6 | 0 |
| `sun/reflect/generics/repository/GenericDeclRepository` | 4 | 0 |
| `sun/reflect/generics/repository/MethodRepository` | 4 | 0 |
| `sun/reflect/generics/scope/AbstractScope` | 5 | 0 |
| `sun/reflect/generics/scope/ClassScope` | 3 | 0 |
| `sun/reflect/generics/scope/ConstructorScope` | 4 | 0 |
| `sun/reflect/generics/scope/DummyScope` | 4 | 0 |
| `sun/reflect/generics/scope/MethodScope` | 4 | 0 |
| `sun/reflect/generics/scope/Scope` | 1 | 0 |
| `sun/reflect/generics/tree/ArrayTypeSignature` | 4 | 0 |
| `sun/reflect/generics/tree/BooleanSignature` | 4 | 0 |
| `sun/reflect/generics/tree/BottomSignature` | 4 | 0 |
| `sun/reflect/generics/tree/ByteSignature` | 4 | 0 |
| `sun/reflect/generics/tree/CharSignature` | 4 | 0 |
| `sun/reflect/generics/tree/ClassSignature` | 6 | 0 |
| `sun/reflect/generics/tree/ClassTypeSignature` | 4 | 0 |
| `sun/reflect/generics/tree/DoubleSignature` | 4 | 0 |
| `sun/reflect/generics/tree/FieldTypeSignature` | 0 | 0 |
| `sun/reflect/generics/tree/FloatSignature` | 4 | 0 |
| `sun/reflect/generics/tree/FormalTypeParameter` | 5 | 0 |
| `sun/reflect/generics/tree/IntSignature` | 4 | 0 |
| `sun/reflect/generics/tree/LongSignature` | 4 | 0 |
| `sun/reflect/generics/tree/MethodTypeSignature` | 7 | 0 |
| `sun/reflect/generics/tree/ReturnType` | 0 | 0 |
| `sun/reflect/generics/tree/ShortSignature` | 4 | 0 |
| `sun/reflect/generics/tree/Signature` | 1 | 0 |
| `sun/reflect/generics/tree/SimpleClassTypeSignature` | 6 | 0 |
| `sun/reflect/generics/tree/TypeArgument` | 0 | 0 |
| `sun/reflect/generics/tree/TypeSignature` | 0 | 0 |
| `sun/reflect/generics/tree/TypeTree` | 1 | 0 |
| `sun/reflect/generics/tree/TypeVariableSignature` | 4 | 0 |
| `sun/reflect/generics/tree/VoidDescriptor` | 4 | 0 |
| `sun/reflect/generics/tree/Wildcard` | 6 | 0 |
| `sun/reflect/generics/visitor/Reifier` | 23 | 0 |
| `sun/reflect/generics/visitor/TypeTreeVisitor` | 17 | 0 |
| `sun/reflect/generics/visitor/Visitor` | 2 | 0 |
| `sun/reflect/misc/ReflectUtil` | 16 | 0 |
| `sun/security/action/GetIntegerAction` | 6 | 0 |
| `sun/security/action/GetPropertyAction` | 9 | 0 |
| `sun/security/action/GetPropertyAction$1` | 3 | 0 |
| `sun/security/jca/GetInstance` | 16 | 0 |
| `sun/security/jca/GetInstance$Instance` | 2 | 0 |
| `sun/security/jca/JCAUtil` | 7 | 0 |
| `sun/security/jca/ProviderConfig` | 15 | 0 |
| `sun/security/jca/ProviderConfig$1` | 3 | 0 |
| `sun/security/jca/ProviderConfig$2` | 3 | 0 |
| `sun/security/jca/ProviderConfig$3` | 3 | 0 |
| `sun/security/jca/ProviderConfig$4` | 3 | 0 |
| `sun/security/jca/ProviderConfig$ProviderLoader` | 4 | 0 |
| `sun/security/jca/ProviderConfig$ProviderLoader$1` | 3 | 0 |
| `sun/security/jca/ProviderList` | 23 | 0 |
| `sun/security/jca/ProviderList$2` | 3 | 0 |
| `sun/security/jca/ProviderList$3` | 4 | 0 |
| `sun/security/jca/ProviderList$PreferredEntry` | 4 | 0 |
| `sun/security/jca/ProviderList$PreferredList` | 8 | 0 |
| `sun/security/jca/ProviderList$ServiceList` | 9 | 0 |
| `sun/security/jca/ProviderList$ServiceList$1` | 5 | 0 |
| `sun/security/jca/Providers` | 15 | 0 |
| `sun/security/jca/ServiceId` | 1 | 0 |
| `sun/security/pkcs/ContentInfo` | 11 | 0 |
| `sun/security/pkcs/PKCS7` | 29 | 0 |
| `sun/security/pkcs/PKCS9Attribute` | 16 | 0 |
| `sun/security/pkcs/PKCS9Attributes` | 14 | 0 |
| `sun/security/pkcs/ParsingException` | 2 | 0 |
| `sun/security/pkcs/SignerInfo` | 25 | 0 |
| `sun/security/pkcs/SignerInfo$AlgorithmInfo` | 6 | 0 |
| `sun/security/pkcs/SigningCertificateInfo` | 4 | 0 |
| `sun/security/pkcs/SigningCertificateInfo$ESSCertId` | 2 | 0 |
| `sun/security/provider/ByteArrayAccess` | 12 | 0 |
| `sun/security/provider/DigestBase` | 15 | 0 |
| `sun/security/provider/FileInputStreamPool` | 3 | 0 |
| `sun/security/provider/FileInputStreamPool$StreamRef` | 1 | 0 |
| `sun/security/provider/FileInputStreamPool$UnclosableInputStream` | 3 | 0 |
| `sun/security/provider/NativePRNG` | 8 | 0 |
| `sun/security/provider/NativePRNG$1` | 3 | 0 |
| `sun/security/provider/NativePRNG$Blocking` | 6 | 0 |
| `sun/security/provider/NativePRNG$NonBlocking` | 6 | 0 |
| `sun/security/provider/NativePRNG$RandomIO` | 7 | 0 |
| `sun/security/provider/NativePRNG$RandomIO$1` | 3 | 0 |
| `sun/security/provider/NativePRNG$Variant` | 5 | 0 |
| `sun/security/provider/PolicyFile` | 36 | 0 |
| `sun/security/provider/PolicyFile$1` | 3 | 0 |
| `sun/security/provider/PolicyFile$2` | 3 | 0 |
| `sun/security/provider/PolicyFile$3` | 3 | 0 |
| `sun/security/provider/PolicyFile$4` | 3 | 0 |
| `sun/security/provider/PolicyFile$5` | 3 | 0 |
| `sun/security/provider/PolicyFile$6` | 3 | 0 |
| `sun/security/provider/PolicyFile$7` | 3 | 0 |
| `sun/security/provider/PolicyFile$8` | 3 | 0 |
| `sun/security/provider/PolicyFile$PolicyEntry` | 6 | 0 |
| `sun/security/provider/PolicyFile$PolicyInfo` | 2 | 0 |
| `sun/security/provider/PolicyFile$SelfPermission` | 11 | 0 |
| `sun/security/provider/PolicyParser` | 33 | 0 |
| `sun/security/provider/PolicyParser$DomainEntry` | 6 | 0 |
| `sun/security/provider/PolicyParser$GrantEntry` | 10 | 0 |
| `sun/security/provider/PolicyParser$KeyStoreEntry` | 4 | 0 |
| `sun/security/provider/PolicyParser$ParsingException` | 5 | 0 |
| `sun/security/provider/PolicyParser$PermissionEntry` | 5 | 0 |
| `sun/security/provider/PolicyParser$PrincipalEntry` | 14 | 0 |
| `sun/security/provider/SHA3` | 12 | 0 |
| `sun/security/provider/SHAKE256` | 6 | 0 |
| `sun/security/provider/SecureRandom` | 8 | 0 |
| `sun/security/provider/SeedGenerator` | 7 | 0 |
| `sun/security/provider/SeedGenerator$1` | 3 | 0 |
| `sun/security/provider/Sun` | 2 | 0 |
| `sun/security/provider/Sun$1` | 3 | 0 |
| `sun/security/provider/SunEntries` | 8 | 0 |
| `sun/security/provider/VerificationProvider` | 3 | 0 |
| `sun/security/provider/VerificationProvider$1` | 3 | 0 |
| `sun/security/provider/X509Factory` | 21 | 0 |
| `sun/security/provider/certpath/X509CertPath` | 14 | 0 |
| `sun/security/provider/certpath/X509CertificatePair` | 15 | 0 |
| `sun/security/rsa/RSAUtil` | 9 | 0 |
| `sun/security/rsa/RSAUtil$KeyType` | 6 | 0 |
| `sun/security/rsa/SunRsaSign` | 2 | 0 |
| `sun/security/rsa/SunRsaSign$1` | 3 | 0 |
| `sun/security/rsa/SunRsaSignEntries` | 4 | 0 |
| `sun/security/ssl/SSLLogger` | 14 | 0 |
| `sun/security/ssl/SSLLogger$SSLSimpleFormatter` | 14 | 0 |
| `sun/security/ssl/SSLScope` | 6 | 0 |
| `sun/security/ssl/SunJSSE` | 5 | 0 |
| `sun/security/ssl/Utilities` | 14 | 0 |
| `sun/security/timestamp/HttpTimestamper` | 4 | 0 |
| `sun/security/timestamp/TSRequest` | 8 | 0 |
| `sun/security/timestamp/TSResponse` | 12 | 0 |
| `sun/security/timestamp/TSResponse$TimestampException` | 1 | 0 |
| `sun/security/timestamp/TimestampToken` | 8 | 0 |
| `sun/security/timestamp/Timestamper` | 1 | 0 |
| `sun/security/util/AbstractAlgorithmConstraints` | 4 | 0 |
| `sun/security/util/AbstractAlgorithmConstraints$1` | 3 | 0 |
| `sun/security/util/AlgorithmDecomposer` | 7 | 0 |
| `sun/security/util/AnchorCertificates` | 4 | 0 |
| `sun/security/util/ArrayUtil` | 5 | 0 |
| `sun/security/util/BitArray` | 18 | 0 |
| `sun/security/util/Cache` | 17 | 0 |
| `sun/security/util/Cache$CacheVisitor` | 1 | 0 |
| `sun/security/util/Cache$EqualByteArray` | 3 | 0 |
| `sun/security/util/ConstraintsParameters` | 5 | 0 |
| `sun/security/util/CryptoAlgorithmConstraints` | 9 | 0 |
| `sun/security/util/CurveDB` | 7 | 0 |
| `sun/security/util/Debug` | 22 | 0 |
| `sun/security/util/DerEncoder` | 1 | 0 |
| `sun/security/util/DerIndefLenConverter` | 15 | 0 |
| `sun/security/util/DerInputStream` | 40 | 0 |
| `sun/security/util/DerOutputStream` | 39 | 0 |
| `sun/security/util/DerValue` | 67 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints` | 20 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$Constraint` | 7 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$Constraint$Operator` | 6 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$Constraints` | 5 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$DenyAfterConstraint` | 3 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$DisabledConstraint` | 3 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$KeySizeConstraint` | 5 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$UsageConstraint` | 3 | 0 |
| `sun/security/util/DisabledAlgorithmConstraints$jdkCAConstraint` | 2 | 0 |
| `sun/security/util/DomainName` | 4 | 0 |
| `sun/security/util/DomainName$CommonMatch` | 3 | 0 |
| `sun/security/util/DomainName$Match` | 2 | 0 |
| `sun/security/util/DomainName$OtherMatch` | 4 | 0 |
| `sun/security/util/DomainName$OtherRule` | 1 | 0 |
| `sun/security/util/DomainName$RegisteredDomainImpl` | 4 | 0 |
| `sun/security/util/DomainName$Rule` | 1 | 0 |
| `sun/security/util/DomainName$Rule$Type` | 5 | 0 |
| `sun/security/util/DomainName$Rules` | 9 | 0 |
| `sun/security/util/DomainName$Rules$1` | 3 | 0 |
| `sun/security/util/DomainName$Rules$RuleSet` | 11 | 0 |
| `sun/security/util/ECKeySizeParameterSpec` | 2 | 0 |
| `sun/security/util/ECUtil` | 22 | 0 |
| `sun/security/util/FilePermCompat` | 4 | 0 |
| `sun/security/util/HexDumpEncoder` | 17 | 0 |
| `sun/security/util/IOUtils` | 2 | 0 |
| `sun/security/util/JarConstraintsParameters` | 11 | 0 |
| `sun/security/util/KeyUtil` | 12 | 0 |
| `sun/security/util/KnownOIDs` | 12 | 0 |
| `sun/security/util/Length` | 1 | 0 |
| `sun/security/util/LocalizedMessage` | 6 | 0 |
| `sun/security/util/ManifestDigester` | 9 | 0 |
| `sun/security/util/ManifestDigester$Entry` | 6 | 0 |
| `sun/security/util/ManifestDigester$Position` | 1 | 0 |
| `sun/security/util/ManifestDigester$Section` | 3 | 0 |
| `sun/security/util/ManifestEntryVerifier` | 9 | 0 |
| `sun/security/util/MemoryCache` | 20 | 0 |
| `sun/security/util/MemoryCache$CacheEntry` | 5 | 0 |
| `sun/security/util/MemoryCache$HardCacheEntry` | 6 | 0 |
| `sun/security/util/MemoryCache$QueueCacheEntry` | 13 | 0 |
| `sun/security/util/MemoryCache$SoftCacheEntry` | 7 | 0 |
| `sun/security/util/MessageDigestSpi2` | 1 | 0 |
| `sun/security/util/NamedCurve` | 5 | 0 |
| `sun/security/util/ObjectIdentifier` | 28 | 0 |
| `sun/security/util/Password` | 3 | 0 |
| `sun/security/util/Password$ConsoleHolder` | 5 | 0 |
| `sun/security/util/Pem` | 2 | 0 |
| `sun/security/util/PolicyUtil` | 3 | 0 |
| `sun/security/util/PropertyExpander` | 3 | 0 |
| `sun/security/util/PropertyExpander$ExpandException` | 1 | 0 |
| `sun/security/util/RegisteredDomain` | 4 | 0 |
| `sun/security/util/Resources` | 3 | 0 |
| `sun/security/util/ResourcesMgr` | 5 | 0 |
| `sun/security/util/SafeDHParameterSpec` | 2 | 0 |
| `sun/security/util/SecurityProperties` | 6 | 0 |
| `sun/security/util/SecurityProviderConstants` | 7 | 0 |
| `sun/security/util/SignatureFileVerifier` | 23 | 0 |
| `sun/security/util/SignatureUtil` | 20 | 0 |
| `sun/security/x509/AVA` | 33 | 0 |
| `sun/security/x509/AVAComparator` | 5 | 0 |
| `sun/security/x509/AVAKeyword` | 7 | 0 |
| `sun/security/x509/AccessDescription` | 9 | 0 |
| `sun/security/x509/AlgorithmId` | 27 | 0 |
| `sun/security/x509/AuthorityInfoAccessExtension` | 7 | 0 |
| `sun/security/x509/AuthorityKeyIdentifierExtension` | 10 | 0 |
| `sun/security/x509/BasicConstraintsExtension` | 9 | 0 |
| `sun/security/x509/CRLDistributionPointsExtension` | 11 | 0 |
| `sun/security/x509/CRLExtensions` | 14 | 0 |
| `sun/security/x509/CRLNumberExtension` | 11 | 0 |
| `sun/security/x509/CRLReasonCodeExtension` | 10 | 0 |
| `sun/security/x509/CertificateAlgorithmId` | 6 | 0 |
| `sun/security/x509/CertificateExtensions` | 17 | 0 |
| `sun/security/x509/CertificateIssuerExtension` | 7 | 0 |
| `sun/security/x509/CertificatePoliciesExtension` | 8 | 0 |
| `sun/security/x509/CertificatePolicyId` | 7 | 0 |
| `sun/security/x509/CertificatePolicyMap` | 6 | 0 |
| `sun/security/x509/CertificateSerialNumber` | 9 | 0 |
| `sun/security/x509/CertificateValidity` | 8 | 0 |
| `sun/security/x509/CertificateVersion` | 10 | 0 |
| `sun/security/x509/CertificateX509Key` | 6 | 0 |
| `sun/security/x509/DNSName` | 11 | 0 |
| `sun/security/x509/DeltaCRLIndicatorExtension` | 4 | 0 |
| `sun/security/x509/DistributionPoint` | 13 | 0 |
| `sun/security/x509/DistributionPointName` | 9 | 0 |
| `sun/security/x509/EDIPartyName` | 12 | 0 |
| `sun/security/x509/ExtendedKeyUsageExtension` | 9 | 0 |
| `sun/security/x509/Extension` | 16 | 0 |
| `sun/security/x509/GeneralName` | 9 | 0 |
| `sun/security/x509/GeneralNameInterface` | 3 | 0 |
| `sun/security/x509/GeneralNames` | 12 | 0 |
| `sun/security/x509/GeneralSubtree` | 9 | 0 |
| `sun/security/x509/GeneralSubtrees` | 22 | 0 |
| `sun/security/x509/IPAddressName` | 14 | 0 |
| `sun/security/x509/IssuerAlternativeNameExtension` | 8 | 0 |
| `sun/security/x509/IssuingDistributionPointExtension` | 13 | 0 |
| `sun/security/x509/KeyIdentifier` | 8 | 0 |
| `sun/security/x509/KeyUsageExtension` | 14 | 0 |
| `sun/security/x509/NameConstraintsExtension` | 14 | 0 |
| `sun/security/x509/OIDMap` | 8 | 0 |
| `sun/security/x509/OIDMap$OIDInfo` | 3 | 0 |
| `sun/security/x509/OIDName` | 11 | 0 |
| `sun/security/x509/OtherName` | 12 | 0 |
| `sun/security/x509/PolicyConstraintsExtension` | 9 | 0 |
| `sun/security/x509/PolicyInformation` | 8 | 0 |
| `sun/security/x509/PolicyMappingsExtension` | 7 | 0 |
| `sun/security/x509/PrivateKeyUsageExtension` | 10 | 0 |
| `sun/security/x509/RDN` | 21 | 0 |
| `sun/security/x509/RFC822Name` | 11 | 0 |
| `sun/security/x509/ReasonFlags` | 14 | 0 |
| `sun/security/x509/SerialNumber` | 9 | 0 |
| `sun/security/x509/SubjectAlternativeNameExtension` | 8 | 0 |
| `sun/security/x509/SubjectKeyIdentifierExtension` | 7 | 0 |
| `sun/security/x509/URIName` | 16 | 0 |
| `sun/security/x509/UniqueIdentity` | 7 | 0 |
| `sun/security/x509/UnparseableExtension` | 3 | 0 |
| `sun/security/x509/X400Address` | 6 | 0 |
| `sun/security/x509/X500Name` | 60 | 0 |
| `sun/security/x509/X509CRLEntryImpl` | 27 | 0 |
| `sun/security/x509/X509CRLImpl` | 47 | 0 |
| `sun/security/x509/X509CRLImpl$TBSCertList` | 7 | 0 |
| `sun/security/x509/X509CRLImpl$X509IssuerSerial` | 8 | 0 |
| `sun/security/x509/X509CertImpl` | 81 | 0 |
| `sun/security/x509/X509CertInfo` | 33 | 0 |
| `sun/security/x509/X509Key` | 22 | 0 |
| `sun/text/BreakDictionary` | 7 | 0 |
| `sun/text/CollatorUtilities` | 4 | 0 |
| `sun/text/CompactByteArray` | 15 | 0 |
| `sun/text/ComposedCharIter` | 4 | 0 |
| `sun/text/DictionaryBasedBreakIterator` | 11 | 0 |
| `sun/text/IntHashtable` | 18 | 0 |
| `sun/text/Normalizer` | 4 | 0 |
| `sun/text/RuleBasedBreakIterator` | 32 | 0 |
| `sun/text/RuleBasedBreakIterator$SafeCharIterator` | 11 | 0 |
| `sun/text/SupplementaryCharacterData` | 4 | 0 |
| `sun/text/UCompactIntArray` | 8 | 0 |
| `sun/text/spi/JavaTimeDateTimePatternProvider` | 3 | 0 |
| `sun/util/BuddhistCalendar` | 16 | 0 |
| `sun/util/PreHashedMap` | 8 | 0 |
| `sun/util/PreHashedMap$1` | 3 | 0 |
| `sun/util/PreHashedMap$1$1` | 6 | 0 |
| `sun/util/PreHashedMap$2` | 3 | 0 |
| `sun/util/PreHashedMap$2$1` | 5 | 0 |
| `sun/util/PreHashedMap$2$1$1` | 7 | 0 |
| `sun/util/PropertyResourceBundleCharset` | 5 | 0 |
| `sun/util/PropertyResourceBundleCharset$PropertiesFileDecoder` | 3 | 0 |
| `sun/util/ResourceBundleEnumeration` | 4 | 0 |
| `sun/util/calendar/AbstractCalendar` | 21 | 0 |
| `sun/util/calendar/BaseCalendar` | 19 | 0 |
| `sun/util/calendar/BaseCalendar$Date` | 10 | 0 |
| `sun/util/calendar/CalendarDate` | 45 | 0 |
| `sun/util/calendar/CalendarSystem` | 21 | 0 |
| `sun/util/calendar/CalendarUtils` | 12 | 0 |
| `sun/util/calendar/Era` | 10 | 0 |
| `sun/util/calendar/Gregorian` | 14 | 0 |
| `sun/util/calendar/Gregorian$Date` | 4 | 0 |
| `sun/util/calendar/ImmutableGregorianDate` | 47 | 0 |
| `sun/util/calendar/JulianCalendar` | 20 | 0 |
| `sun/util/calendar/JulianCalendar$Date` | 8 | 0 |
| `sun/util/calendar/LocalGregorianCalendar` | 28 | 0 |
| `sun/util/calendar/LocalGregorianCalendar$Date` | 13 | 0 |
| `sun/util/calendar/ZoneInfo` | 30 | 0 |
| `sun/util/calendar/ZoneInfoFile` | 22 | 0 |
| `sun/util/calendar/ZoneInfoFile$1` | 3 | 0 |
| `sun/util/calendar/ZoneInfoFile$Checksum` | 3 | 0 |
| `sun/util/calendar/ZoneInfoFile$ZoneOffsetTransitionRule` | 8 | 0 |
| `sun/util/cldr/CLDRBaseLocaleDataMetaInfo` | 7 | 0 |
| `sun/util/cldr/CLDRCalendarDataProviderImpl` | 6 | 0 |
| `sun/util/cldr/CLDRCalendarNameProviderImpl` | 2 | 0 |
| `sun/util/cldr/CLDRLocaleProviderAdapter` | 23 | 0 |
| `sun/util/cldr/CLDRTimeZoneNameProviderImpl` | 10 | 0 |
| `sun/util/locale/BaseLocale` | 12 | 0 |
| `sun/util/locale/BaseLocale$Cache` | 6 | 0 |
| `sun/util/locale/BaseLocale$Key` | 7 | 0 |
| `sun/util/locale/Extension` | 7 | 0 |
| `sun/util/locale/InternalLocaleBuilder` | 21 | 0 |
| `sun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar` | 5 | 0 |
| `sun/util/locale/InternalLocaleBuilder$CaseInsensitiveString` | 4 | 0 |
| `sun/util/locale/LanguageTag` | 41 | 0 |
| `sun/util/locale/LocaleExtensions` | 17 | 0 |
| `sun/util/locale/LocaleMatcher` | 21 | 0 |
| `sun/util/locale/LocaleObjectCache` | 7 | 0 |
| `sun/util/locale/LocaleObjectCache$CacheEntry` | 2 | 0 |
| `sun/util/locale/LocaleSyntaxException` | 3 | 0 |
| `sun/util/locale/LocaleUtils` | 20 | 0 |
| `sun/util/locale/ParseStatus` | 6 | 0 |
| `sun/util/locale/StringTokenIterator` | 11 | 0 |
| `sun/util/locale/UnicodeLocaleExtension` | 14 | 0 |
| `sun/util/locale/provider/BaseLocaleDataMetaInfo` | 5 | 0 |
| `sun/util/locale/provider/BreakIteratorProviderImpl` | 9 | 0 |
| `sun/util/locale/provider/BreakIteratorProviderImpl$CharacterIteratorCharSequence` | 4 | 0 |
| `sun/util/locale/provider/BreakIteratorProviderImpl$GraphemeBreakIterator` | 13 | 0 |
| `sun/util/locale/provider/CalendarDataProviderImpl` | 6 | 0 |
| `sun/util/locale/provider/CalendarDataUtility` | 10 | 0 |
| `sun/util/locale/provider/CalendarNameProviderImpl` | 16 | 0 |
| `sun/util/locale/provider/CalendarProviderImpl` | 5 | 0 |
| `sun/util/locale/provider/CollatorProviderImpl` | 5 | 0 |
| `sun/util/locale/provider/CurrencyNameProviderImpl` | 6 | 0 |
| `sun/util/locale/provider/DateFormatProviderImpl` | 8 | 0 |
| `sun/util/locale/provider/DateFormatSymbolsProviderImpl` | 5 | 0 |
| `sun/util/locale/provider/DecimalFormatSymbolsProviderImpl` | 5 | 0 |
| `sun/util/locale/provider/JRELocaleProviderAdapter` | 41 | 0 |
| `sun/util/locale/provider/JavaTimeDateTimePatternImpl` | 9 | 0 |
| `sun/util/locale/provider/LocaleDataMetaInfo` | 4 | 0 |
| `sun/util/locale/provider/LocaleNameProviderImpl` | 11 | 0 |
| `sun/util/locale/provider/LocaleProviderAdapter` | 29 | 0 |
| `sun/util/locale/provider/LocaleProviderAdapter$Type` | 9 | 0 |
| `sun/util/locale/provider/LocaleResources` | 42 | 0 |
| `sun/util/locale/provider/LocaleResources$ResourceReference` | 2 | 0 |
| `sun/util/locale/provider/LocaleServiceProviderPool` | 14 | 0 |
| `sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter` | 1 | 0 |
| `sun/util/locale/provider/NumberFormatProviderImpl` | 11 | 0 |
| `sun/util/locale/provider/ResourceBundleBasedAdapter` | 2 | 0 |
| `sun/util/locale/provider/TimeZoneNameProviderImpl` | 8 | 0 |
| `sun/util/locale/provider/TimeZoneNameUtility` | 10 | 0 |
| `sun/util/logging/PlatformLogger` | 30 | 0 |
| `sun/util/logging/PlatformLogger$Bridge` | 18 | 0 |
| `sun/util/logging/PlatformLogger$ConfigurableBridge` | 2 | 0 |
| `sun/util/logging/PlatformLogger$ConfigurableBridge$LoggerConfiguration` | 3 | 0 |
| `sun/util/resources/Bundles` | 12 | 0 |
| `sun/util/resources/Bundles$2` | 3 | 0 |
| `sun/util/resources/Bundles$BundleReference` | 2 | 0 |
| `sun/util/resources/Bundles$CacheKey` | 14 | 0 |
| `sun/util/resources/Bundles$CacheKeyReference` | 1 | 0 |
| `sun/util/resources/Bundles$Strategy` | 3 | 0 |
| `sun/util/resources/LocaleData` | 15 | 0 |
| `sun/util/resources/LocaleData$1` | 3 | 0 |
| `sun/util/resources/LocaleData$2` | 3 | 0 |
| `sun/util/resources/OpenListResourceBundle` | 10 | 0 |
| `sun/util/resources/ParallelListResourceBundle` | 12 | 0 |
| `sun/util/resources/ParallelListResourceBundle$KeySet` | 4 | 0 |
| `sun/util/resources/ParallelListResourceBundle$KeySet$1` | 5 | 0 |
| `sun/util/resources/TimeZoneNamesBundle` | 5 | 0 |
| `sun/util/spi/CalendarProvider` | 2 | 0 |

## 方法级 BFS 可达方法

### `com/sun/crypto/provider/SunJCE`

- `<init>()V`
- `ps(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `ps(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/List;Ljava/util/HashMap;)V`
- `psA(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/HashMap;)V`
- `putEntries()V`
- `putService(Ljava/security/Provider$Service;)V`

### `com/sun/crypto/provider/SunJCE$1`

- `<init>()V`
- `<init>(Lcom/sun/crypto/provider/SunJCE;)V`

### `java/io/BufferedInputStream`

- `<init>()V`
- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/io/InputStream;I)V`
- `ensureOpen()V`
- `implMark(I)V`
- `implReset()V`
- `mark(I)V`
- `reset()V`

### `java/io/BufferedReader`

- `<init>()V`
- `<init>(Ljava/io/Reader;)V`
- `<init>(Ljava/io/Reader;I)V`
- `ensureOpen()V`
- `fill()V`
- `implReadLine(Z[Z)Ljava/lang/String;`
- `readLine()Ljava/lang/String;`
- `readLine(Z[Z)Ljava/lang/String;`

### `java/io/BufferedWriter`

- `<init>()V`
- `<init>(Ljava/io/Writer;)V`
- `<init>(Ljava/io/Writer;II)V`
- `ensureOpen()V`
- `flushBuffer()V`
- `growIfNeeded(I)V`
- `implFlushBuffer()V`
- `implWrite(I)V`
- `implWrite(Ljava/lang/String;II)V`
- `implWrite([CII)V`
- `initialBufferSize()I`
- `min(II)I`
- `newLine()V`
- `write(Ljava/lang/String;)V`
- `write([C)V`

### `java/io/ByteArrayInputStream`

- `<init>()V`
- `<init>([B)V`
- `<init>([BII)V`
- `available()I`
- `read()I`

### `java/io/ByteArrayOutputStream`

- `<init>()V`
- `<init>(I)V`
- `ensureCapacity(I)V`
- `toByteArray()[B`
- `toString()Ljava/lang/String;`
- `write(I)V`
- `write([B)V`

### `java/io/ClassCache`

- `get(Ljava/lang/Class;)Ljava/lang/Object;`
- `processQueue()V`

### `java/io/ClassCache$CacheRef`

- `<init>()V`
- `clearStrong()V`
- `get()Ljava/lang/Object;`
- `getStrong()Ljava/lang/Object;`
- `getType()Ljava/lang/Class;`

### `java/io/Console`

- `newUnsupportedOperationException()Ljava/lang/UnsupportedOperationException;`
- `readPassword()[C`

### `java/io/DataInput`

- `readByte()B`
- `readInt()I`
- `readLong()J`

### `java/io/DataInputStream`

- `<init>()V`
- `<init>(Ljava/io/InputStream;)V`
- `close()V`
- `mark(I)V`
- `readBoolean()Z`
- `readChar()C`
- `readDouble()D`
- `readFloat()F`
- `readFully([B)V`
- `readFully([BII)V`
- `readInt()I`
- `readLong()J`
- `readShort()S`
- `readUnsignedByte()I`
- `readUnsignedShort()I`
- `reset()V`

### `java/io/EOFException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/io/Externalizable`

- `<init>()V`
- `readExternal(Ljava/io/ObjectInput;)V`

### `java/io/File`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `getAbsolutePath()Ljava/lang/String;`
- `getCanonicalPath()Ljava/lang/String;`
- `getPath()Ljava/lang/String;`
- `isDirectory()Z`
- `isFile()Z`
- `isInvalid()Z`

### `java/io/FileCleanable`

- `<init>()V`
- `<init>(Ljava/io/FileDescriptor;Ljava/lang/ref/Cleaner;IJ)V`
- `register(Ljava/io/FileDescriptor;)V`

### `java/io/FileDescriptor`

- `<init>()V`
- `attach(Ljava/io/Closeable;)V`
- `getAppend(I)Z`
- `getHandle(I)J`
- `registerCleanup(Ljdk/internal/ref/PhantomCleanable;)V`
- `valid()Z`

### `java/io/FileInputStream`

- `<init>()V`
- `<init>(Ljava/io/File;)V`
- `<init>(Ljava/lang/String;)V`
- `open(Ljava/lang/String;)V`
- `open0(Ljava/lang/String;)V`

### `java/io/FileNotFoundException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/io/FileOutputStream`

- `<init>()V`
- `<init>(Ljava/io/File;)V`
- `<init>(Ljava/io/File;Z)V`
- `<init>(Ljava/lang/String;)V`
- `open(Ljava/lang/String;Z)V`
- `open0(Ljava/lang/String;Z)V`

### `java/io/FilePermission`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `getMask(Ljava/lang/String;)I`
- `getName()Ljava/lang/String;`
- `init(I)V`

### `java/io/FilePermission$2`

- `<init>()V`
- `<init>(Ljava/io/FilePermission;)V`

### `java/io/FileSystem`

- `canonicalize(Ljava/lang/String;)Ljava/lang/String;`
- `fromURIPath(Ljava/lang/String;)Ljava/lang/String;`
- `getBooleanAttributes(Ljava/io/File;)I`
- `getDefaultParent()Ljava/lang/String;`
- `hasBooleanAttributes(Ljava/io/File;I)Z`
- `isInvalid(Ljava/io/File;)Z`
- `normalize(Ljava/lang/String;)Ljava/lang/String;`
- `prefixLength(Ljava/lang/String;)I`
- `resolve(Ljava/io/File;)Ljava/lang/String;`
- `resolve(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`

### `java/io/FilterInputStream`

- `<init>(Ljava/io/InputStream;)V`

### `java/io/FilterOutputStream`

- `<init>(Ljava/io/OutputStream;)V`

### `java/io/IOException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`
- `getMessage()Ljava/lang/String;`

### `java/io/InputStream`

- `<init>()V`
- `available()I`
- `close()V`
- `mark(I)V`
- `markSupported()Z`
- `read()I`
- `read([BII)I`
- `readNBytes(I)[B`
- `readNBytes([BII)I`
- `reset()V`

### `java/io/InputStreamReader`

- `<init>()V`
- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/io/InputStream;Ljava/nio/charset/Charset;)V`
- `close()V`
- `lockFor(Ljava/io/InputStreamReader;)Ljava/lang/Object;`

### `java/io/InvalidClassException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/io/InvalidObjectException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/io/NotActiveException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/io/ObjectInputFilter`

- `checkInput(Ljava/io/ObjectInputFilter$FilterInfo;)Ljava/io/ObjectInputFilter$Status;`

### `java/io/ObjectInputFilter$Status`

- `name()Ljava/lang/String;`

### `java/io/ObjectInputStream`

- `checkResolve(Ljava/lang/Object;)Ljava/lang/Object;`
- `clear()V`
- `cloneArray(Ljava/lang/Object;)Ljava/lang/Object;`
- `filterCheck(Ljava/lang/Class;I)V`
- `freeze()V`
- `handleReset()V`
- `isCustomSubclass()Z`
- `latestUserDefinedLoader()Ljava/lang/ClassLoader;`
- `readArray(Z)Ljava/lang/Object;`
- `readByte()B`
- `readClass(Z)Ljava/lang/Class;`
- `readClassDesc(Z)Ljava/io/ObjectStreamClass;`
- `readClassDescriptor()Ljava/io/ObjectStreamClass;`
- `readEnum(Z)Ljava/lang/Enum;`
- `readExternalData(Ljava/io/Externalizable;Ljava/io/ObjectStreamClass;)V`
- `readFatalException()Ljava/io/IOException;`
- `readHandle(Z)Ljava/lang/Object;`
- `readLong()J`
- `readNonProxyDesc(Z)Ljava/io/ObjectStreamClass;`
- `readNull()Ljava/lang/Object;`
- `readObject()Ljava/lang/Object;`
- `readObject(Ljava/lang/Class;)Ljava/lang/Object;`
- `readObject0(Ljava/lang/Class;Z)Ljava/lang/Object;`
- `readObjectOverride()Ljava/lang/Object;`
- `readOrdinaryObject(Z)Ljava/lang/Object;`
- `readProxyDesc(Z)Ljava/io/ObjectStreamClass;`
- `readRecord(Ljava/io/ObjectStreamClass;)Ljava/lang/Object;`
- `readSerialData(Ljava/lang/Object;Ljava/io/ObjectStreamClass;)V`
- `readShort()S`
- `readString(Z)Ljava/lang/String;`
- `readTypeString()Ljava/lang/String;`
- `readUTF()Ljava/lang/String;`
- `resolveClass(Ljava/io/ObjectStreamClass;)Ljava/lang/Class;`
- `resolveObject(Ljava/lang/Object;)Ljava/lang/Object;`
- `resolveProxyClass([Ljava/lang/String;)Ljava/lang/Class;`
- `skipCustomData()V`

### `java/io/ObjectInputStream$BlockDataInputStream`

- `currentBlockRemaining()I`
- `getBlockDataMode()Z`
- `getBytesRead()J`
- `peek()I`
- `peekByte()B`
- `read()I`
- `read([BIIZ)I`
- `readBlockHeader(Z)I`
- `readBooleans([ZII)V`
- `readByte()B`
- `readChars([CII)V`
- `readDoubles([DII)V`
- `readFloats([FII)V`
- `readFully([BIIZ)V`
- `readInt()I`
- `readInts([III)V`
- `readLong()J`
- `readLongUTF()Ljava/lang/String;`
- `readLongs([JII)V`
- `readShort()S`
- `readShorts([SII)V`
- `readUTF()Ljava/lang/String;`
- `readUTFBody(J)Ljava/lang/String;`
- `readUTFChar(Ljava/lang/StringBuilder;J)I`
- `readUTFSpan(Ljava/lang/StringBuilder;J)J`
- `readUnsignedShort()I`
- `refill()V`
- `setBlockDataMode(Z)Z`
- `skipBlockData()V`

### `java/io/ObjectInputStream$FieldValues`

- `<init>()V`
- `<init>(Ljava/io/ObjectInputStream;Ljava/io/ObjectStreamClass;Z)V`
- `defaultCheckFieldValues(Ljava/lang/Object;)V`
- `defaultSetFieldValues(Ljava/lang/Object;)V`

### `java/io/ObjectInputStream$FilterValues`

- `<init>()V`
- `<init>(Ljava/lang/Class;JJJJ)V`

### `java/io/ObjectInputStream$GetField`

- `<init>()V`

### `java/io/ObjectInputStream$HandleTable`

- `assign(Ljava/lang/Object;)I`
- `clear()V`
- `finish(I)V`
- `grow()V`
- `lookupException(I)Ljava/lang/ClassNotFoundException;`
- `lookupObject(I)Ljava/lang/Object;`
- `markDependency(II)V`
- `markException(ILjava/lang/ClassNotFoundException;)V`
- `setObject(ILjava/lang/Object;)V`
- `size()I`

### `java/io/ObjectInputStream$HandleTable$HandleList`

- `<init>()V`
- `add(I)V`
- `get(I)I`
- `size()I`

### `java/io/ObjectInputStream$PeekInputStream`

- `available()I`
- `getBytesRead()J`
- `peek()I`
- `read()I`
- `read([BII)I`
- `readFully([BII)V`

### `java/io/ObjectInputStream$ValidationList`

- `clear()V`
- `doCallbacks()V`

### `java/io/ObjectInputStream$ValidationList$1`

- `<init>()V`
- `<init>(Ljava/io/ObjectInputStream$ValidationList;)V`

### `java/io/ObjectStreamClass`

- `<init>()V`
- `checkDeserialize()V`
- `checkInitialized()V`
- `checkObjFieldValueTypes(Ljava/lang/Object;[Ljava/lang/Object;)V`
- `classNamesEqual(Ljava/lang/String;Ljava/lang/String;)Z`
- `computeFieldOffsets()V`
- `forClass()Ljava/lang/Class;`
- `getClassDataLayout()[Ljava/io/ObjectStreamClass$ClassDataSlot;`
- `getClassDataLayout0()[Ljava/io/ObjectStreamClass$ClassDataSlot;`
- `getFields()[Ljava/io/ObjectStreamField;`
- `getFields(Z)[Ljava/io/ObjectStreamField;`
- `getLocalDesc()Ljava/io/ObjectStreamClass;`
- `getName()Ljava/lang/String;`
- `getNumObjFields()I`
- `getPrimDataSize()I`
- `getRecordConstructor()Ljava/lang/invoke/MethodHandle;`
- `getReflector([Ljava/io/ObjectStreamField;Ljava/io/ObjectStreamClass;)Ljava/io/ObjectStreamClass$FieldReflector;`
- `getResolveException()Ljava/lang/ClassNotFoundException;`
- `getSerialVersionUID()J`
- `getSuperDesc()Ljava/io/ObjectStreamClass;`
- `getVariantFor(Ljava/lang/Class;)Ljava/io/ObjectStreamClass;`
- `hasBlockExternalData()Z`
- `hasReadObjectMethod()Z`
- `hasReadObjectNoDataMethod()Z`
- `hasReadResolveMethod()Z`
- `hasWriteObjectData()Z`
- `initNonProxy(Ljava/io/ObjectStreamClass;Ljava/lang/Class;Ljava/lang/ClassNotFoundException;Ljava/io/ObjectStreamClass;)V`
- `initProxy(Ljava/lang/Class;Ljava/lang/ClassNotFoundException;Ljava/io/ObjectStreamClass;)V`
- `invokeReadObject(Ljava/lang/Object;Ljava/io/ObjectInputStream;)V`
- `invokeReadObjectNoData(Ljava/lang/Object;)V`
- `invokeReadResolve(Ljava/lang/Object;)Ljava/lang/Object;`
- `isEnum()Z`
- `isExternalizable()Z`
- `isInstantiable()Z`
- `isRecord()Z`
- `lookup(Ljava/lang/Class;Z)Ljava/io/ObjectStreamClass;`
- `matchFields([Ljava/io/ObjectStreamField;Ljava/io/ObjectStreamClass;)[Ljava/io/ObjectStreamField;`
- `newInstance()Ljava/lang/Object;`
- `readNonProxy(Ljava/io/ObjectInputStream;)V`
- `requireInitialized()V`
- `setObjFieldValues(Ljava/lang/Object;[Ljava/lang/Object;)V`
- `setPrimFieldValues(Ljava/lang/Object;[B)V`
- `throwMiscException(Ljava/lang/Throwable;)V`

### `java/io/ObjectStreamClass$1`

- `<init>()V`
- `<init>(Ljava/io/ObjectStreamClass;)V`

### `java/io/ObjectStreamClass$2`

- `<init>()V`
- `<init>(Ljava/io/ObjectStreamClass;Ljava/lang/Class;)V`

### `java/io/ObjectStreamClass$ClassDataSlot`

- `<init>()V`
- `<init>(Ljava/io/ObjectStreamClass;Z)V`

### `java/io/ObjectStreamClass$DeserializationConstructorsCache`

- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `get([Ljava/io/ObjectStreamField;)Ljava/lang/invoke/MethodHandle;`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putIfAbsentAndGet([Ljava/io/ObjectStreamField;Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `remove(Ljava/lang/Object;)Ljava/lang/Object;`
- `size()I`

### `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key`

- `<init>()V`

### `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key$Impl`

- `<init>()V`
- `<init>([Ljava/io/ObjectStreamField;)V`

### `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key$Lookup`

- `<init>()V`
- `<init>([Ljava/io/ObjectStreamField;)V`

### `java/io/ObjectStreamClass$ExceptionInfo`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `newInvalidClassException()Ljava/io/InvalidClassException;`

### `java/io/ObjectStreamClass$FieldReflector`

- `<init>()V`
- `<init>([Ljava/io/ObjectStreamField;)V`
- `checkObjectFieldValueTypes(Ljava/lang/Object;[Ljava/lang/Object;)V`
- `getFields()[Ljava/io/ObjectStreamField;`
- `setObjFieldValues(Ljava/lang/Object;[Ljava/lang/Object;)V`
- `setObjFieldValues(Ljava/lang/Object;[Ljava/lang/Object;Z)V`
- `setPrimFieldValues(Ljava/lang/Object;[B)V`

### `java/io/ObjectStreamClass$FieldReflectorKey`

- `<init>()V`
- `<init>([Ljava/io/ObjectStreamField;)V`

### `java/io/ObjectStreamClass$RecordSupport`

- `deserializationCtr(Ljava/io/ObjectStreamClass;)Ljava/lang/invoke/MethodHandle;`
- `numberPrimValues(Ljava/io/ObjectStreamClass;)I`
- `streamFieldExtractor(Ljava/lang/String;Ljava/lang/Class;Ljava/io/ObjectStreamClass;)Ljava/lang/invoke/MethodHandle;`

### `java/io/ObjectStreamException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/io/ObjectStreamField`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/Class;Z)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Z)V`
- `<init>(Ljava/lang/reflect/Field;ZZ)V`
- `getField()Ljava/lang/reflect/Field;`
- `getName()Ljava/lang/String;`
- `getOffset()I`
- `getSignature()Ljava/lang/String;`
- `getType()Ljava/lang/Class;`
- `getTypeCode()C`
- `isPrimitive()Z`
- `isUnshared()Z`
- `setOffset(I)V`

### `java/io/OptionalDataException`

- `<init>()V`
- `<init>(I)V`
- `<init>(Z)V`

### `java/io/OutputStream`

- `<init>()V`
- `flush()V`
- `write(I)V`
- `write([BII)V`

### `java/io/OutputStreamWriter`

- `<init>()V`
- `<init>(Ljava/io/OutputStream;)V`
- `<init>(Ljava/io/OutputStream;Ljava/lang/String;)V`
- `<init>(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V`
- `flushBuffer()V`
- `lockFor(Ljava/io/OutputStreamWriter;)Ljava/lang/Object;`

### `java/io/PrintStream`

- `<init>()V`
- `<init>(Ljava/io/OutputStream;)V`
- `<init>(Ljava/io/OutputStream;Z)V`
- `<init>(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V`
- `<init>(ZLjava/io/OutputStream;)V`
- `<init>(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V`
- `charset()Ljava/nio/charset/Charset;`
- `ensureOpen()V`
- `implNewLine()V`
- `implWrite(I)V`
- `implWrite(Ljava/lang/String;)V`
- `implWrite([BII)V`
- `implWrite([C)V`
- `implWriteln(Ljava/lang/String;)V`
- `implWriteln([C)V`
- `newLine()V`
- `print(C)V`
- `print(D)V`
- `print(F)V`
- `print(I)V`
- `print(J)V`
- `print(Ljava/lang/String;)V`
- `print(Z)V`
- `print([C)V`
- `println()V`
- `println(I)V`
- `println(Ljava/lang/String;)V`
- `requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;`
- `toCharset(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `write(I)V`
- `write(Ljava/lang/String;)V`
- `write([BII)V`
- `write([C)V`
- `writeln(Ljava/lang/String;)V`
- `writeln([C)V`

### `java/io/PushbackInputStream`

- `<init>()V`
- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/io/InputStream;I)V`
- `ensureOpen()V`
- `unread(I)V`
- `unread([BII)V`

### `java/io/Reader`

- `<init>()V`
- `<init>(Ljava/io/Reader;)V`
- `<init>(Ljava/lang/Object;)V`
- `mark(I)V`
- `markSupported()Z`
- `read()I`
- `read([CII)I`
- `reset()V`

### `java/io/SerialCallbackContext`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/io/ObjectStreamClass;)V`
- `check()V`
- `setUsed()V`

### `java/io/Serializable`

- `<init>()V`

### `java/io/StreamCorruptedException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/io/StreamTokenizer`

- `<init>()V`
- `<init>(Ljava/io/Reader;)V`
- `commentChar(I)V`
- `lineno()I`
- `lowerCaseMode(Z)V`
- `nextToken()I`
- `ordinaryChar(I)V`
- `parseNumbers()V`
- `quoteChar(I)V`
- `read()I`
- `resetSyntax()V`
- `slashSlashComments(Z)V`
- `slashStarComments(Z)V`
- `whitespaceChars(II)V`
- `wordChars(II)V`

### `java/io/StringReader`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/io/UTFDataFormatException`

- `<init>()V`

### `java/io/UncheckedIOException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/io/IOException;)V`

### `java/io/UnsupportedEncodingException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/io/WriteAbortedException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/Exception;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/io/Writer`

- `<init>()V`
- `<init>(Ljava/io/Writer;)V`
- `<init>(Ljava/lang/Object;)V`
- `implWrite(I)V`
- `implWrite(Ljava/lang/String;II)V`
- `write(Ljava/lang/String;II)V`
- `write([CII)V`

### `java/lang/AbstractMethodError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/AbstractStringBuilder`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/lang/CharSequence;)V`
- `<init>(Ljava/lang/String;)V`
- `append(C)Ljava/lang/AbstractStringBuilder;`
- `append(D)Ljava/lang/AbstractStringBuilder;`
- `append(F)Ljava/lang/AbstractStringBuilder;`
- `append(I)Ljava/lang/AbstractStringBuilder;`
- `append(J)Ljava/lang/AbstractStringBuilder;`
- `append(Ljava/lang/AbstractStringBuilder;)Ljava/lang/AbstractStringBuilder;`
- `append(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;`
- `append(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;`
- `append(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;`
- `append(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;`
- `append(Z)Ljava/lang/AbstractStringBuilder;`
- `append([C)Ljava/lang/AbstractStringBuilder;`
- `append([CII)Ljava/lang/AbstractStringBuilder;`
- `appendChars(Ljava/lang/CharSequence;II)V`
- `appendChars(Ljava/lang/String;II)V`
- `appendChars([CII)V`
- `appendCodePoint(I)Ljava/lang/AbstractStringBuilder;`
- `appendNull()Ljava/lang/AbstractStringBuilder;`
- `charAt(I)C`
- `delete(II)Ljava/lang/AbstractStringBuilder;`
- `ensureCapacityInternal(I)V`
- `getBytes([BIB)V`
- `getCoder()B`
- `getValue()[B`
- `inflate()V`
- `inflateIfNeededFor(Ljava/lang/AbstractStringBuilder;)V`
- `inflateIfNeededFor(Ljava/lang/String;)V`
- `insert(IC)Ljava/lang/AbstractStringBuilder;`
- `insert(ID)Ljava/lang/AbstractStringBuilder;`
- `insert(IF)Ljava/lang/AbstractStringBuilder;`
- `insert(II)Ljava/lang/AbstractStringBuilder;`
- `insert(IJ)Ljava/lang/AbstractStringBuilder;`
- `insert(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;`
- `insert(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;`
- `insert(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;`
- `insert(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;`
- `insert(IZ)Ljava/lang/AbstractStringBuilder;`
- `insert(I[C)Ljava/lang/AbstractStringBuilder;`
- `insert(I[CII)Ljava/lang/AbstractStringBuilder;`
- `isLatin1()Z`
- `length()I`
- `newCapacity(I)I`
- `putCharsAt(ILjava/lang/CharSequence;II)V`
- `putCharsAt(I[CII)V`
- `putStringAt(ILjava/lang/String;)V`
- `setLength(I)V`
- `shift(II)V`

### `java/lang/Appendable`

- `<init>()V`
- `append(C)Ljava/lang/Appendable;`
- `append(Ljava/lang/CharSequence;)Ljava/lang/Appendable;`
- `append(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;`
- `toString()Ljava/lang/String;`

### `java/lang/ArithmeticException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/ArrayIndexOutOfBoundsException`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/AssertionError`

- `<init>()V`
- `<init>(I)V`
- `<init>(J)V`
- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/BaseVirtualThread`

- `<init>()V`
- `<init>(Ljava/lang/String;IZ)V`

### `java/lang/Boolean`

- `<init>()V`
- `<init>(Z)V`
- `booleanValue()Z`
- `parseBoolean(Ljava/lang/String;)Z`
- `valueOf(Z)Ljava/lang/Boolean;`

### `java/lang/Byte`

- `<init>()V`
- `byteValue()B`
- `parseByte(Ljava/lang/String;I)B`
- `valueOf(B)Ljava/lang/Byte;`
- `valueOf(Ljava/lang/String;I)Ljava/lang/Byte;`

### `java/lang/CharSequence`

- `charAt(I)C`
- `length()I`
- `subSequence(II)Ljava/lang/CharSequence;`
- `toString()Ljava/lang/String;`

### `java/lang/Character`

- `<init>()V`
- `<init>(C)V`
- `charCount(I)I`
- `charValue()C`
- `codePointAt(Ljava/lang/CharSequence;I)I`
- `codePointAtImpl([CII)I`
- `codePointOf(Ljava/lang/String;)I`
- `digit(CI)I`
- `digit(II)I`
- `forDigit(II)C`
- `getName(I)Ljava/lang/String;`
- `getType(I)I`
- `highSurrogate(I)C`
- `isBmpCodePoint(I)Z`
- `isDigit(C)Z`
- `isDigit(I)Z`
- `isHighSurrogate(C)Z`
- `isISOControl(C)Z`
- `isISOControl(I)Z`
- `isLetter(C)Z`
- `isLetter(I)Z`
- `isLetterOrDigit(C)Z`
- `isLetterOrDigit(I)Z`
- `isLowSurrogate(C)Z`
- `isSpaceChar(C)Z`
- `isSpaceChar(I)Z`
- `isSupplementaryCodePoint(I)Z`
- `isSurrogate(C)Z`
- `isUpperCase(C)Z`
- `isUpperCase(I)Z`
- `isValidCodePoint(I)Z`
- `isWhitespace(C)Z`
- `isWhitespace(I)Z`
- `lowSurrogate(I)C`
- `toChars(I)[C`
- `toCodePoint(CC)I`
- `toLowerCase(C)C`
- `toLowerCase(I)I`
- `toString(C)Ljava/lang/String;`
- `toSurrogates(I[CI)V`
- `toUpperCase(C)C`
- `toUpperCase(I)I`
- `toUpperCaseCharArray(I)[C`
- `toUpperCaseEx(I)I`
- `valueOf(C)Ljava/lang/Character;`

### `java/lang/Character$Subset`

- `<init>(Ljava/lang/String;)V`

### `java/lang/Character$UnicodeBlock`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `forName(Ljava/lang/String;)Ljava/lang/Character$UnicodeBlock;`
- `of(I)Ljava/lang/Character$UnicodeBlock;`
- `toString()Ljava/lang/String;`

### `java/lang/Character$UnicodeScript`

- `<init>()V`
- `forName(Ljava/lang/String;)Ljava/lang/Character$UnicodeScript;`
- `valueOf(Ljava/lang/String;)Ljava/lang/Character$UnicodeScript;`

### `java/lang/CharacterData`

- `digit(II)I`
- `getType(I)I`
- `isDigit(I)Z`
- `isUpperCase(I)Z`
- `isWhitespace(I)Z`
- `of(I)Ljava/lang/CharacterData;`
- `toLowerCase(I)I`
- `toUpperCase(I)I`
- `toUpperCaseCharArray(I)[C`
- `toUpperCaseEx(I)I`

### `java/lang/CharacterDataLatin1`

- `equalsIgnoreCase(BB)Z`
- `getProperties(I)I`
- `toLowerCase(I)I`
- `toUpperCase(I)I`
- `toUpperCaseCharArray(I)[C`
- `toUpperCaseEx(I)I`

### `java/lang/CharacterName`

- `<init>()V`
- `addCp(IIII)I`
- `getCodePoint(Ljava/lang/String;)I`
- `getCp(I)I`
- `getCpHash(I)I`
- `getCpNext(I)I`
- `getInstance()Ljava/lang/CharacterName;`
- `getName(I)Ljava/lang/String;`
- `hashN([BII)I`

### `java/lang/CharacterName$1`

- `<init>()V`
- `<init>(Ljava/lang/CharacterName;)V`

### `java/lang/Class`

- `<init>()V`
- `arrayContentsEq([Ljava/lang/Object;[Ljava/lang/Object;)Z`
- `cannotCastMsg(Ljava/lang/Object;)Ljava/lang/String;`
- `cast(Ljava/lang/Object;)Ljava/lang/Object;`
- `checkMemberAccess(Ljava/lang/SecurityManager;ILjava/lang/Class;Z)V`
- `checkPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V`
- `componentType()Ljava/lang/Class;`
- `descriptorString()Ljava/lang/String;`
- `desiredAssertionStatus()Z`
- `desiredAssertionStatus0(Ljava/lang/Class;)Z`
- `elementType()Ljava/lang/Class;`
- `enumConstantDirectory()Ljava/util/Map;`
- `forName(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;`
- `forName(Ljava/lang/String;)Ljava/lang/Class;`
- `forName(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;`
- `forName(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;`
- `forName(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;`
- `forName0(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;`
- `getCanonicalName()Ljava/lang/String;`
- `getCanonicalName0()Ljava/lang/String;`
- `getClassLoader()Ljava/lang/ClassLoader;`
- `getClassLoader0()Ljava/lang/ClassLoader;`
- `getComponentType()Ljava/lang/Class;`
- `getConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;`
- `getConstructor0([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;`
- `getDeclaredConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;`
- `getDeclaredConstructors0(Z)[Ljava/lang/reflect/Constructor;`
- `getDeclaredField(Ljava/lang/String;)Ljava/lang/reflect/Field;`
- `getDeclaredFields0(Z)[Ljava/lang/reflect/Field;`
- `getDeclaredMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;`
- `getDeclaredMethods0(Z)[Ljava/lang/reflect/Method;`
- `getDeclaringClass()Ljava/lang/Class;`
- `getDeclaringClass0()Ljava/lang/Class;`
- `getEnclosingClass()Ljava/lang/Class;`
- `getEnclosingMethod0()[Ljava/lang/Object;`
- `getEnclosingMethodInfo()Ljava/lang/Class$EnclosingMethodInfo;`
- `getEnumConstantsShared()[Ljava/lang/Object;`
- `getFactory()Lsun/reflect/generics/factory/GenericsFactory;`
- `getGenericInfo()Lsun/reflect/generics/repository/ClassRepository;`
- `getGenericInterfaces()[Ljava/lang/reflect/Type;`
- `getGenericSignature0()Ljava/lang/String;`
- `getInterfaces()[Ljava/lang/Class;`
- `getInterfaces(Z)[Ljava/lang/Class;`
- `getInterfaces0()[Ljava/lang/Class;`
- `getMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;`
- `getMethod0(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;`
- `getMethodsRecursive(Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;`
- `getModifiers()I`
- `getModule()Ljava/lang/Module;`
- `getName()Ljava/lang/String;`
- `getNestHost()Ljava/lang/Class;`
- `getNestHost0()Ljava/lang/Class;`
- `getPackageName()Ljava/lang/String;`
- `getReflectionFactory()Ljdk/internal/reflect/ReflectionFactory;`
- `getSigners()[Ljava/lang/Object;`
- `getSimpleBinaryName()Ljava/lang/String;`
- `getSimpleBinaryName0()Ljava/lang/String;`
- `getSimpleName()Ljava/lang/String;`
- `getSimpleName0()Ljava/lang/String;`
- `getSuperclass()Ljava/lang/Class;`
- `hasEnclosingMethodInfo()Z`
- `initClassName()Ljava/lang/String;`
- `isAnonymousClass()Z`
- `isArray()Z`
- `isAssignableFrom(Ljava/lang/Class;)Z`
- `isEnum()Z`
- `isHidden()Z`
- `isInstance(Ljava/lang/Object;)Z`
- `isInterface()Z`
- `isLocalClass()Z`
- `isLocalOrAnonymousClass()Z`
- `isPrimitive()Z`
- `isRecord()Z`
- `isRecord0()Z`
- `isSynthetic()Z`
- `isTopLevelClass()Z`
- `isUnnamedClass()Z`
- `methodToString(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/String;`
- `newInstance()Ljava/lang/Object;`
- `newReflectionData(Ljava/lang/ref/SoftReference;I)Ljava/lang/Class$ReflectionData;`
- `privateGetDeclaredConstructors(Z)[Ljava/lang/reflect/Constructor;`
- `privateGetDeclaredFields(Z)[Ljava/lang/reflect/Field;`
- `privateGetDeclaredMethods(Z)[Ljava/lang/reflect/Method;`
- `reflectionData()Ljava/lang/Class$ReflectionData;`
- `searchFields([Ljava/lang/reflect/Field;Ljava/lang/String;)Ljava/lang/reflect/Field;`
- `searchMethods([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;`

### `java/lang/Class$1`

- `<init>()V`
- `<init>(Ljava/lang/Class;Ljava/lang/reflect/Constructor;)V`

### `java/lang/Class$3`

- `<init>()V`
- `<init>(Ljava/lang/Class;Ljava/lang/reflect/Method;)V`

### `java/lang/Class$Atomic`

- `casReflectionData(Ljava/lang/Class;Ljava/lang/ref/SoftReference;Ljava/lang/ref/SoftReference;)Z`

### `java/lang/Class$EnclosingMethodInfo`

- `<init>()V`
- `<init>([Ljava/lang/Object;)V`
- `getEnclosingClass()Ljava/lang/Class;`
- `validate([Ljava/lang/Object;)V`

### `java/lang/Class$ReflectionData`

- `<init>()V`
- `<init>(I)V`

### `java/lang/ClassCastException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/ClassFormatError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/ClassLoader`

- `<init>()V`
- `<init>(Ljava/lang/Void;Ljava/lang/String;Ljava/lang/ClassLoader;)V`
- `checkClassLoaderPermission(Ljava/lang/ClassLoader;Ljava/lang/Class;)V`
- `checkCreateClassLoader()Ljava/lang/Void;`
- `checkCreateClassLoader(Ljava/lang/String;)Ljava/lang/Void;`
- `checkName(Ljava/lang/String;)Z`
- `desiredAssertionStatus(Ljava/lang/String;)Z`
- `findBootstrapClass(Ljava/lang/String;)Ljava/lang/Class;`
- `findBootstrapClassOrNull(Ljava/lang/String;)Ljava/lang/Class;`
- `findClass(Ljava/lang/String;)Ljava/lang/Class;`
- `findClass(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Class;`
- `findLoadedClass(Ljava/lang/String;)Ljava/lang/Class;`
- `findLoadedClass0(Ljava/lang/String;)Ljava/lang/Class;`
- `getBuiltinAppClassLoader()Ljava/lang/ClassLoader;`
- `getClassLoader(Ljava/lang/Class;)Ljava/lang/ClassLoader;`
- `getClassLoadingLock(Ljava/lang/String;)Ljava/lang/Object;`
- `getName()Ljava/lang/String;`
- `getParent()Ljava/lang/ClassLoader;`
- `getSystemClassLoader()Ljava/lang/ClassLoader;`
- `isAncestor(Ljava/lang/ClassLoader;)Z`
- `loadClass(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;`
- `loadClass(Ljava/lang/String;)Ljava/lang/Class;`
- `loadClass(Ljava/lang/String;Z)Ljava/lang/Class;`
- `nameAndId(Ljava/lang/ClassLoader;)Ljava/lang/String;`
- `needsClassLoaderPermissionCheck(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`
- `resolveClass(Ljava/lang/Class;)V`

### `java/lang/ClassLoader$ParallelLoaders`

- `isRegistered(Ljava/lang/Class;)Z`

### `java/lang/ClassNotFoundException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/lang/ClassValue`

- `bumpVersion()V`
- `castEntry(Ljava/lang/ClassValue$Entry;)Ljava/lang/ClassValue$Entry;`
- `computeValue(Ljava/lang/Class;)Ljava/lang/Object;`
- `get(Ljava/lang/Class;)Ljava/lang/Object;`
- `getCacheCarefully(Ljava/lang/Class;)[Ljava/lang/ClassValue$Entry;`
- `getFromBackup([Ljava/lang/ClassValue$Entry;Ljava/lang/Class;)Ljava/lang/Object;`
- `getFromHashMap(Ljava/lang/Class;)Ljava/lang/Object;`
- `getMap(Ljava/lang/Class;)Ljava/lang/ClassValue$ClassValueMap;`
- `initializeMap(Ljava/lang/Class;)Ljava/lang/ClassValue$ClassValueMap;`
- `makeEntry(Ljava/lang/ClassValue$Version;Ljava/lang/Object;)Ljava/lang/ClassValue$Entry;`
- `match(Ljava/lang/ClassValue$Entry;)Z`
- `remove(Ljava/lang/Class;)V`
- `version()Ljava/lang/ClassValue$Version;`

### `java/lang/ClassValue$ClassValueMap`

- `<init>()V`
- `addToCache(Ljava/lang/ClassValue$Entry;)V`
- `addToCache(Ljava/lang/ClassValue;Ljava/lang/ClassValue$Entry;)V`
- `checkCacheLoad()V`
- `entryDislocation([Ljava/lang/ClassValue$Entry;ILjava/lang/ClassValue$Entry;)I`
- `findReplacement([Ljava/lang/ClassValue$Entry;I)Ljava/lang/ClassValue$Entry;`
- `finishEntry(Ljava/lang/ClassValue;Ljava/lang/ClassValue$Entry;)Ljava/lang/ClassValue$Entry;`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getCache()[Ljava/lang/ClassValue$Entry;`
- `loadFromCache([Ljava/lang/ClassValue$Entry;I)Ljava/lang/ClassValue$Entry;`
- `overwrittenEntry(Ljava/lang/ClassValue$Entry;)Ljava/lang/ClassValue$Entry;`
- `placeInCache([Ljava/lang/ClassValue$Entry;ILjava/lang/ClassValue$Entry;Z)Ljava/lang/ClassValue$Entry;`
- `probeBackupLocations([Ljava/lang/ClassValue$Entry;Ljava/lang/ClassValue;)Ljava/lang/ClassValue$Entry;`
- `probeHomeLocation([Ljava/lang/ClassValue$Entry;Ljava/lang/ClassValue;)Ljava/lang/ClassValue$Entry;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `reduceCacheLoad()V`
- `remove(Ljava/lang/Object;)Ljava/lang/Object;`
- `removeEntry(Ljava/lang/ClassValue;)V`
- `removeStaleEntries()V`
- `removeStaleEntries(Ljava/lang/ClassValue;)V`
- `removeStaleEntries([Ljava/lang/ClassValue$Entry;II)V`
- `sizeCache(I)V`
- `startEntry(Ljava/lang/ClassValue;)Ljava/lang/ClassValue$Entry;`

### `java/lang/ClassValue$Entry`

- `<init>()V`
- `<init>(Ljava/lang/ClassValue$Version;)V`
- `<init>(Ljava/lang/ClassValue$Version;Ljava/lang/Object;)V`
- `assertNotPromise()V`
- `classValueOrNull()Ljava/lang/ClassValue;`
- `clear()V`
- `get()Ljava/lang/Object;`
- `isLive()Z`
- `isPromise()Z`
- `refreshVersion(Ljava/lang/ClassValue$Version;)Ljava/lang/ClassValue$Entry;`
- `value()Ljava/lang/Object;`
- `version()Ljava/lang/ClassValue$Version;`

### `java/lang/ClassValue$Version`

- `<init>()V`
- `<init>(Ljava/lang/ClassValue;)V`
- `classValue()Ljava/lang/ClassValue;`
- `isLive()Z`
- `promise()Ljava/lang/ClassValue$Entry;`

### `java/lang/Comparable`

- `<init>()V`
- `compareTo(Ljava/lang/Object;)I`

### `java/lang/ConditionalSpecialCasing`

- `isAfterI(Ljava/lang/String;I)Z`
- `isAfterSoftDotted(Ljava/lang/String;I)Z`
- `isBeforeDot(Ljava/lang/String;I)Z`
- `isCased(I)Z`
- `isConditionMet(Ljava/lang/String;ILjava/util/Locale;I)Z`
- `isFinalCased(Ljava/lang/String;ILjava/util/Locale;)Z`
- `isMoreAbove(Ljava/lang/String;I)Z`
- `isSoftDotted(I)Z`
- `lookUpTable(Ljava/lang/String;ILjava/util/Locale;Z)[C`
- `toLowerCaseCharArray(Ljava/lang/String;ILjava/util/Locale;)[C`
- `toLowerCaseEx(Ljava/lang/String;ILjava/util/Locale;)I`
- `toUpperCaseCharArray(Ljava/lang/String;ILjava/util/Locale;)[C`
- `toUpperCaseEx(Ljava/lang/String;ILjava/util/Locale;)I`

### `java/lang/ConditionalSpecialCasing$Entry`

- `<init>()V`
- `getCondition()I`
- `getLanguage()Ljava/lang/String;`
- `getLowerCase()[C`
- `getUpperCase()[C`

### `java/lang/Double`

- `<init>()V`
- `<init>(D)V`
- `compare(DD)I`
- `doubleToLongBits(D)J`
- `doubleToRawLongBits(D)J`
- `doubleValue()D`
- `isInfinite(D)Z`
- `isNaN(D)Z`
- `longBitsToDouble(J)D`
- `parseDouble(Ljava/lang/String;)D`
- `toString(D)Ljava/lang/String;`
- `valueOf(D)Ljava/lang/Double;`

### `java/lang/Enum`

- `<init>()V`
- `<init>(Ljava/lang/String;I)V`
- `getDeclaringClass()Ljava/lang/Class;`
- `name()Ljava/lang/String;`
- `valueOf(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;`

### `java/lang/Error`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/Exception`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V`
- `<init>(Ljava/lang/Throwable;)V`
- `getCause()Ljava/lang/Throwable;`
- `getMessage()Ljava/lang/String;`
- `printStackTrace()V`
- `toString()Ljava/lang/String;`

### `java/lang/Float`

- `<init>()V`
- `<init>(F)V`
- `compare(FF)I`
- `floatToIntBits(F)I`
- `floatToRawIntBits(F)I`
- `floatValue()F`
- `intBitsToFloat(I)F`
- `isNaN(F)Z`
- `parseFloat(Ljava/lang/String;)F`
- `toString(F)Ljava/lang/String;`
- `valueOf(F)Ljava/lang/Float;`

### `java/lang/IllegalAccessError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `getMessage()Ljava/lang/String;`

### `java/lang/IllegalAccessException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/IllegalArgumentException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/IllegalCallerException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/IllegalStateException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/IncompatibleClassChangeError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/IndexOutOfBoundsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/InstantiationException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/Integer`

- `<init>()V`
- `<init>(I)V`
- `bitCount(I)I`
- `compare(II)I`
- `formatUnsignedInt(II[BI)V`
- `formatUnsignedIntUTF16(II[BI)V`
- `getChars(II[B)I`
- `highestOneBit(I)I`
- `intValue()I`
- `numberOfLeadingZeros(I)I`
- `numberOfTrailingZeros(I)I`
- `parseInt(Ljava/lang/CharSequence;III)I`
- `parseInt(Ljava/lang/String;)I`
- `parseInt(Ljava/lang/String;I)I`
- `parseUnsignedInt(Ljava/lang/String;I)I`
- `stringSize(I)I`
- `toHexString(I)Ljava/lang/String;`
- `toString()Ljava/lang/String;`
- `toString(I)Ljava/lang/String;`
- `toString(II)Ljava/lang/String;`
- `toStringUTF16(II)Ljava/lang/String;`
- `toUnsignedString0(II)Ljava/lang/String;`
- `valueOf(I)Ljava/lang/Integer;`

### `java/lang/InternalError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/LinkageError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/Long`

- `<init>()V`
- `<init>(J)V`
- `compare(JJ)I`
- `formatUnsignedLong0(JI[BII)V`
- `formatUnsignedLong0UTF16(JI[BII)V`
- `getChars(JI[B)I`
- `longValue()J`
- `numberOfLeadingZeros(J)I`
- `parseLong(Ljava/lang/CharSequence;III)J`
- `parseLong(Ljava/lang/String;)J`
- `parseLong(Ljava/lang/String;I)J`
- `signum(J)I`
- `stringSize(J)I`
- `toHexString(J)Ljava/lang/String;`
- `toString(J)Ljava/lang/String;`
- `toStringUTF16(JI)Ljava/lang/String;`
- `toUnsignedString0(JI)Ljava/lang/String;`
- `valueOf(J)Ljava/lang/Long;`

### `java/lang/Math`

- `abs(D)D`
- `abs(I)I`
- `abs(J)J`
- `addExact(II)I`
- `addExact(JJ)J`
- `ceil(D)D`
- `clamp(JII)I`
- `floorDiv(JI)J`
- `floorDiv(JJ)J`
- `floorMod(II)I`
- `floorMod(JI)I`
- `floorMod(JJ)J`
- `getExponent(D)I`
- `max(DD)D`
- `max(FF)F`
- `max(II)I`
- `max(JJ)J`
- `min(DD)D`
- `min(FF)F`
- `min(II)I`
- `min(JJ)J`
- `multiplyExact(II)I`
- `multiplyExact(JI)J`
- `multiplyExact(JJ)J`

### `java/lang/Module`

- `<init>()V`
- `<init>(Ljava/lang/ClassLoader;)V`
- `<init>(Ljava/lang/ModuleLayer;Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;Ljava/net/URI;)V`
- `addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V`
- `addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V`
- `addReads0(Ljava/lang/Module;Ljava/lang/Module;)V`
- `allows(Ljava/util/Set;Ljava/lang/Module;)Z`
- `canRead(Ljava/lang/Module;)Z`
- `canUse(Ljava/lang/Class;)Z`
- `defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V`
- `defineModules(Ljava/lang/module/Configuration;Ljava/util/function/Function;Ljava/lang/ModuleLayer;)Ljava/util/Map;`
- `findModule(Ljava/lang/ModuleLayer;Ljava/lang/module/ResolvedModule;)Ljava/lang/Module;`
- `findModule(Ljava/lang/String;Ljava/util/Map;Ljava/util/Map;Ljava/util/List;)Ljava/lang/Module;`
- `getClassLoader()Ljava/lang/ClassLoader;`
- `getDescriptor()Ljava/lang/module/ModuleDescriptor;`
- `getLayer()Ljava/lang/ModuleLayer;`
- `getName()Ljava/lang/String;`
- `implAddEnableNativeAccess()Ljava/lang/Module;`
- `implAddReads(Ljava/lang/Module;Z)V`
- `implIsExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z`
- `initExports(Ljava/lang/Module;Ljava/util/Map;)V`
- `initExportsAndOpens(Ljava/lang/Module;Ljava/util/Map;Ljava/util/Map;Ljava/util/List;)V`
- `isExported(Ljava/lang/String;)Z`
- `isExported(Ljava/lang/String;Ljava/lang/Module;)Z`
- `isNamed()Z`
- `isReflectivelyExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z`
- `isStaticallyExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z`

### `java/lang/Module$EnableNativeAccess`

- `trySetEnableNativeAccess(Ljava/lang/Module;)Z`

### `java/lang/ModuleLayer`

- `<init>()V`
- `bindToLoader(Ljava/lang/ClassLoader;)V`
- `boot()Ljava/lang/ModuleLayer;`
- `findModule(Ljava/lang/String;)Ljava/util/Optional;`
- `layers()Ljava/util/stream/Stream;`
- `parents()Ljava/util/List;`

### `java/lang/NegativeArraySizeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/NoSuchFieldError`

- `<init>()V`

### `java/lang/NoSuchFieldException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/NoSuchMethodError`

- `<init>()V`

### `java/lang/NoSuchMethodException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `addSuppressed(Ljava/lang/Throwable;)V`
- `printStackTrace()V`

### `java/lang/NullPointerException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/Number`

- `<init>()V`
- `byteValue()B`
- `doubleValue()D`
- `floatValue()F`
- `intValue()I`
- `longValue()J`

### `java/lang/NumberFormatException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `forCharSequence(Ljava/lang/CharSequence;III)Ljava/lang/NumberFormatException;`
- `forInputString(Ljava/lang/String;I)Ljava/lang/NumberFormatException;`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/Object`

- `<init>()V`
- `clone()Ljava/lang/Object;`
- `equals(Ljava/lang/Object;)Z`
- `getClass()Ljava/lang/Class;`
- `hashCode()I`
- `toString()Ljava/lang/String;`

### `java/lang/OutOfMemoryError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/PublicMethods$Key`

- `matches(Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Z`

### `java/lang/PublicMethods$MethodList`

- `<init>()V`
- `<init>(Ljava/lang/reflect/Method;)V`
- `filter([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;`
- `getMostSpecific()Ljava/lang/reflect/Method;`
- `merge(Ljava/lang/PublicMethods$MethodList;Ljava/lang/PublicMethods$MethodList;)Ljava/lang/PublicMethods$MethodList;`
- `merge(Ljava/lang/PublicMethods$MethodList;Ljava/lang/reflect/Method;)Ljava/lang/PublicMethods$MethodList;`

### `java/lang/Record`

- `<init>()V`

### `java/lang/ReflectiveOperationException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/RuntimeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`
- `getMessage()Ljava/lang/String;`

### `java/lang/RuntimePermission`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`

### `java/lang/SecurityException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/SecurityManager`

- `checkAccess(Ljava/lang/Thread;)V`
- `checkAccess(Ljava/lang/ThreadGroup;)V`
- `checkConnect(Ljava/lang/String;I)V`
- `checkCreateClassLoader()V`
- `checkPackageAccess(Ljava/lang/String;)V`
- `checkPermission(Ljava/security/Permission;)V`
- `checkPermission(Ljava/security/Permission;Ljava/lang/Object;)V`
- `checkPropertiesAccess()V`
- `checkPropertyAccess(Ljava/lang/String;)V`
- `checkRead(Ljava/io/FileDescriptor;)V`
- `checkRead(Ljava/lang/String;)V`
- `checkWrite(Ljava/io/FileDescriptor;)V`
- `checkWrite(Ljava/lang/String;)V`
- `getPackages(Ljava/lang/String;)[Ljava/lang/String;`
- `getThreadGroup()Ljava/lang/ThreadGroup;`

### `java/lang/SecurityManager$1`

- `<init>()V`
- `<init>(Ljava/lang/SecurityManager;)V`

### `java/lang/Short`

- `<init>()V`
- `<init>(S)V`
- `parseShort(Ljava/lang/String;I)S`
- `shortValue()S`
- `valueOf(Ljava/lang/String;I)Ljava/lang/Short;`
- `valueOf(S)Ljava/lang/Short;`

### `java/lang/StackStreamFactory`

- `makeStackTraverser(Ljava/lang/StackWalker;Ljava/util/function/Function;)Ljava/lang/StackStreamFactory$StackFrameTraverser;`

### `java/lang/StackStreamFactory$AbstractStackWalker`

- `<init>(Ljava/lang/StackWalker;I)V`
- `<init>(Ljava/lang/StackWalker;II)V`
- `toStackWalkMode(Ljava/lang/StackWalker;I)I`

### `java/lang/StackStreamFactory$LiveStackInfoTraverser`

- `<init>()V`
- `<init>(Ljava/lang/StackWalker;Ljava/util/function/Function;)V`

### `java/lang/StackStreamFactory$StackFrameTraverser`

- `<init>()V`
- `<init>(Ljava/lang/StackWalker;Ljava/util/function/Function;)V`
- `<init>(Ljava/lang/StackWalker;Ljava/util/function/Function;I)V`
- `walk()Ljava/lang/Object;`

### `java/lang/StackTraceElement`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V`
- `computeFormat()V`
- `equals(Ljava/lang/Object;)Z`
- `initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V`
- `initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V`
- `isHashedInJavaBase(Ljava/lang/Module;)Z`
- `of(Ljava/lang/Object;I)[Ljava/lang/StackTraceElement;`
- `of([Ljava/lang/StackTraceElement;)[Ljava/lang/StackTraceElement;`

### `java/lang/StackTraceElement$HashedModules`

- `contains(Ljava/lang/Module;)Z`

### `java/lang/StackWalker`

- `<init>()V`
- `<init>(Ljava/util/EnumSet;I)V`
- `<init>(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V`
- `<init>(Ljava/util/EnumSet;Ljdk/internal/vm/ContinuationScope;)V`
- `checkPermission(Ljava/util/Set;)V`
- `getContScope()Ljdk/internal/vm/ContinuationScope;`
- `getContinuation()Ljdk/internal/vm/Continuation;`
- `getInstance()Ljava/lang/StackWalker;`
- `getInstance(Ljava/util/Set;)Ljava/lang/StackWalker;`
- `getInstance(Ljava/util/Set;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;`
- `hasLocalsOperandsOption()Z`
- `hasOption(Ljava/lang/StackWalker$Option;)Z`
- `toEnumSet(Ljava/util/Set;)Ljava/util/EnumSet;`
- `walk(Ljava/util/function/Function;)Ljava/lang/Object;`

### `java/lang/StackWalker$Option`

- `<init>()V`

### `java/lang/StrictMath`

- `ceil(D)D`
- `floorOrCeil(DDDD)D`

### `java/lang/String`

- `<init>()V`
- `<init>(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/StringBuilder;)V`
- `<init>(Ljava/nio/charset/Charset;[BII)V`
- `<init>([BB)V`
- `<init>([BII)V`
- `<init>([BIII)V`
- `<init>([BIILjava/lang/String;)V`
- `<init>([BIILjava/nio/charset/Charset;)V`
- `<init>([BLjava/nio/charset/Charset;)V`
- `<init>([C)V`
- `<init>([CII)V`
- `<init>([CIILjava/lang/Void;)V`
- `<init>([III)V`
- `charAt(I)C`
- `checkBoundsBeginEnd(III)V`
- `checkBoundsOffCount(III)I`
- `checkIndex(II)V`
- `checkOffset(II)V`
- `codePointAt(I)I`
- `codePointBefore(I)I`
- `codePointCount(II)I`
- `coder()B`
- `compareTo(Ljava/lang/String;)I`
- `compareToIgnoreCase(Ljava/lang/String;)I`
- `concat(Ljava/lang/String;)Ljava/lang/String;`
- `contains(Ljava/lang/CharSequence;)Z`
- `copyValueOf([CII)Ljava/lang/String;`
- `decode2(II)C`
- `decode3(III)C`
- `decode4(IIII)I`
- `decodeUTF8_UTF16([BII[BIZ)I`
- `decodeWithDecoder(Ljava/nio/charset/CharsetDecoder;[C[BII)I`
- `encode(Ljava/nio/charset/Charset;B[B)[B`
- `encode8859_1(B[B)[B`
- `encode8859_1(B[BZ)[B`
- `encodeASCII(B[B)[B`
- `encodeUTF8(B[BZ)[B`
- `encodeUTF8_UTF16([BZ)[B`
- `encodeWithEncoder(Ljava/nio/charset/Charset;B[BZ)[B`
- `endsWith(Ljava/lang/String;)Z`
- `equals(Ljava/lang/Object;)Z`
- `equalsIgnoreCase(Ljava/lang/String;)Z`
- `format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;`
- `getBytes(Ljava/nio/charset/Charset;)[B`
- `getBytes([BIB)V`
- `getChars(II[CI)V`
- `hashCode()I`
- `indexOf(I)I`
- `indexOf(II)I`
- `indexOf(III)I`
- `indexOf(Ljava/lang/String;)I`
- `indexOf(Ljava/lang/String;I)I`
- `indexOf([BBILjava/lang/String;I)I`
- `intern()Ljava/lang/String;`
- `isEmpty()Z`
- `isLatin1()Z`
- `isMalformed3(III)Z`
- `isMalformed3_2(II)Z`
- `isMalformed4(III)Z`
- `isMalformed4_2(II)Z`
- `isMalformed4_3(I)Z`
- `isNotContinuation(I)Z`
- `lastIndexOf(I)I`
- `lastIndexOf(II)I`
- `lastIndexOf(Ljava/lang/String;)I`
- `lastIndexOf(Ljava/lang/String;I)I`
- `lastIndexOf([BBILjava/lang/String;I)I`
- `length()I`
- `lookupCharset(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `malformed3([BI)I`
- `malformed4([BI)I`
- `rangeCheck([CII)Ljava/lang/Void;`
- `regionMatches(ILjava/lang/String;II)Z`
- `regionMatches(ZILjava/lang/String;II)Z`
- `repeat(I)Ljava/lang/String;`
- `repeatCopyRest([BIII)V`
- `replace(CC)Ljava/lang/String;`
- `replace(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;`
- `replaceAll(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`
- `replaceFirst(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`
- `replaceNegatives([BI)V`
- `safeTrim([BIZ)[B`
- `scale(IF)I`
- `split(CIZ)[Ljava/lang/String;`
- `split(Ljava/lang/String;)[Ljava/lang/String;`
- `split(Ljava/lang/String;IZ)[Ljava/lang/String;`
- `startsWith(Ljava/lang/String;)Z`
- `startsWith(Ljava/lang/String;I)Z`
- `substring(I)Ljava/lang/String;`
- `substring(II)Ljava/lang/String;`
- `throwMalformed(II)V`
- `throwUnmappable(I)V`
- `toCharArray()[C`
- `toLowerCase()Ljava/lang/String;`
- `toLowerCase(Ljava/util/Locale;)Ljava/lang/String;`
- `toUpperCase()Ljava/lang/String;`
- `toUpperCase(Ljava/util/Locale;)Ljava/lang/String;`
- `trim()Ljava/lang/String;`
- `value()[B`
- `valueOf(C)Ljava/lang/String;`
- `valueOf(D)Ljava/lang/String;`
- `valueOf(F)Ljava/lang/String;`
- `valueOf(I)Ljava/lang/String;`
- `valueOf(J)Ljava/lang/String;`
- `valueOf(Ljava/lang/Object;)Ljava/lang/String;`
- `valueOf(Z)Ljava/lang/String;`
- `valueOf([C)Ljava/lang/String;`
- `valueOfCodePoint(I)Ljava/lang/String;`

### `java/lang/StringBuffer`

- `append(C)Ljava/lang/StringBuffer;`
- `append(D)Ljava/lang/StringBuffer;`
- `append(F)Ljava/lang/StringBuffer;`
- `append(I)Ljava/lang/StringBuffer;`
- `append(J)Ljava/lang/StringBuffer;`
- `append(Ljava/lang/AbstractStringBuilder;)Ljava/lang/StringBuffer;`
- `append(Ljava/lang/CharSequence;)Ljava/lang/StringBuffer;`
- `append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuffer;`
- `append(Ljava/lang/Object;)Ljava/lang/StringBuffer;`
- `append(Ljava/lang/String;)Ljava/lang/StringBuffer;`
- `append(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;`
- `append(Z)Ljava/lang/StringBuffer;`
- `append([C)Ljava/lang/StringBuffer;`
- `append([CII)Ljava/lang/StringBuffer;`
- `length()I`
- `setLength(I)V`
- `toString()Ljava/lang/String;`

### `java/lang/StringBuilder`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/lang/String;)V`
- `append(C)Ljava/lang/StringBuilder;`
- `append(D)Ljava/lang/StringBuilder;`
- `append(F)Ljava/lang/StringBuilder;`
- `append(I)Ljava/lang/StringBuilder;`
- `append(J)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/Object;)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/String;)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;`
- `append(Z)Ljava/lang/StringBuilder;`
- `append([C)Ljava/lang/StringBuilder;`
- `append([CII)Ljava/lang/StringBuilder;`
- `appendCodePoint(I)Ljava/lang/StringBuilder;`
- `charAt(I)C`
- `delete(II)Ljava/lang/StringBuilder;`
- `insert(IC)Ljava/lang/StringBuilder;`
- `insert(ID)Ljava/lang/StringBuilder;`
- `insert(IF)Ljava/lang/StringBuilder;`
- `insert(II)Ljava/lang/StringBuilder;`
- `insert(IJ)Ljava/lang/StringBuilder;`
- `insert(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;`
- `insert(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;`
- `insert(ILjava/lang/Object;)Ljava/lang/StringBuilder;`
- `insert(ILjava/lang/String;)Ljava/lang/StringBuilder;`
- `insert(IZ)Ljava/lang/StringBuilder;`
- `insert(I[C)Ljava/lang/StringBuilder;`
- `insert(I[CII)Ljava/lang/StringBuilder;`
- `length()I`
- `setLength(I)V`
- `toString()Ljava/lang/String;`

### `java/lang/StringCoding`

- `countPositives([BII)I`
- `hasNegatives([BII)Z`
- `implEncodeISOArray([BI[BII)I`

### `java/lang/StringConcatHelper`

- `checkOverflow(J)J`
- `coder(C)J`
- `initialCoder()J`
- `mix(JLjava/lang/String;)J`
- `newArray(J)[B`
- `newString([BJ)Ljava/lang/String;`
- `prepend(J[BC)J`
- `prepend(J[BI)J`
- `prepend(J[BJ)J`
- `prepend(J[BLjava/lang/String;)J`
- `prepend(J[BLjdk/internal/util/FormatConcatItem;)J`
- `prepend(J[BZ)J`
- `simpleConcat(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/String;`
- `stringOf(Ljava/lang/Object;)Ljava/lang/String;`

### `java/lang/StringIndexOutOfBoundsException`

- `<init>()V`
- `<init>(I)V`

### `java/lang/StringLatin1`

- `canEncode(C)Z`
- `canEncode(I)Z`
- `charAt([BI)C`
- `compareTo([B[B)I`
- `compareTo([B[BII)I`
- `compareToUTF16([B[B)I`
- `compareToUTF16([B[BII)I`
- `compareToUTF16Values([B[BII)I`
- `equals([B[B)Z`
- `fillNull([BII)V`
- `getBytes([BII[BI)V`
- `getChar([BI)C`
- `getChars([BII[CI)V`
- `hashCode([B)I`
- `indexOf([BIII)I`
- `indexOf([BI[BII)I`
- `indexOf([B[B)I`
- `indexOfChar([BIII)I`
- `inflate([BII)[B`
- `inflate([BI[BII)V`
- `inflate([BI[CII)V`
- `lastIndexOf([BII)I`
- `lastIndexOf([BI[BII)I`
- `length([B)I`
- `newString([BII)Ljava/lang/String;`
- `regionMatchesCI([BI[BII)Z`
- `regionMatchesCI_UTF16([BI[BII)Z`
- `replace([BCC)Ljava/lang/String;`
- `replace([BI[BI[BI)Ljava/lang/String;`
- `toBytes(C)[B`
- `toChars([B)[C`
- `toLowerCase(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;`
- `toLowerCaseEx(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;`
- `toUpperCase(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;`
- `toUpperCaseEx(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;`
- `trim([B)Ljava/lang/String;`

### `java/lang/StringUTF16`

- `charAt([BI)C`
- `checkBoundsBeginEnd(II[B)V`
- `checkBoundsOffCount(II[B)V`
- `checkIndex(I[B)V`
- `checkOffset(I[B)V`
- `codePointAt([BII)I`
- `codePointAt([BIIZ)I`
- `codePointBefore([BI)I`
- `codePointBefore([BIZ)I`
- `codePointCount([BII)I`
- `codePointCount([BIIZ)I`
- `codePointIncluding([BIIII)I`
- `coderFromArrayLen([BI)B`
- `compareCodePointCI(II)I`
- `compareTo([B[B)I`
- `compareToCIImpl([BII[BII)I`
- `compareToLatin1([B[B)I`
- `compareValues([B[BII)I`
- `compress([BII)[B`
- `compress([BI[BII)I`
- `compress([CII)[B`
- `compress([CI[BII)I`
- `compress([III)[B`
- `computeCodePointSize([III)I`
- `extractCodepoints([III[BI)[B`
- `fillNull([BII)V`
- `getBytes([BII[BI)V`
- `getChar([BI)C`
- `getChars(III[B)I`
- `getChars(II[B)I`
- `getChars(JII[B)I`
- `getChars(JI[B)I`
- `getChars([BII[CI)V`
- `hashCode([B)I`
- `indexOf([BIII)I`
- `indexOf([BI[BII)I`
- `indexOf([B[B)I`
- `indexOfChar([BIII)I`
- `indexOfCharUnsafe([BIII)I`
- `indexOfLatin1([BI[BII)I`
- `indexOfLatin1([B[B)I`
- `indexOfLatin1Unsafe([BI[BII)I`
- `indexOfSupplementary([BIII)I`
- `indexOfUnsafe([BI[BII)I`
- `inflate([BI[BII)V`
- `lastIndexOf([BII)I`
- `lastIndexOf([BI[BII)I`
- `lastIndexOfLatin1([BI[BII)I`
- `lastIndexOfSupplementary([BII)I`
- `length([B)I`
- `newBytesFor(I)[B`
- `newBytesLength(I)I`
- `newString([BII)Ljava/lang/String;`
- `putChar([BII)V`
- `putCharSB([BII)V`
- `putChars([BI[CII)V`
- `putCharsAt([BICCCC)I`
- `putCharsAt([BICCCCC)I`
- `putCharsSB([BILjava/lang/CharSequence;II)V`
- `putCharsSB([BI[CII)V`
- `regionMatchesCI([BI[BII)Z`
- `regionMatchesCI_Latin1([BI[BII)Z`
- `replace([BCC)Ljava/lang/String;`
- `replace([BIZ[BIZ[BIZ)Ljava/lang/String;`
- `toBytes(C)[B`
- `toBytes([CII)[B`
- `toBytes([III)[B`
- `toBytesSupplementary(I)[B`
- `toChars([B)[C`
- `toLowerCase(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;`
- `toLowerCaseEx(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;`
- `toUpperCase(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;`
- `toUpperCaseEx(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;`
- `trim([B)Ljava/lang/String;`

### `java/lang/System`

- `allowSecurityManager()Z`
- `arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V`
- `checkKey(Ljava/lang/String;)V`
- `currentTimeMillis()J`
- `getLogger(Ljava/lang/String;)Ljava/lang/System$Logger;`
- `getProperties()Ljava/util/Properties;`
- `getProperty(Ljava/lang/String;)Ljava/lang/String;`
- `getProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`
- `getSecurityManager()Ljava/lang/SecurityManager;`
- `identityHashCode(Ljava/lang/Object;)I`
- `lineSeparator()Ljava/lang/String;`
- `nanoTime()J`

### `java/lang/System$Logger`

- `<init>()V`
- `isLoggable(Ljava/lang/System$Logger$Level;)Z`
- `log(Ljava/lang/System$Logger$Level;Ljava/lang/String;)V`
- `log(Ljava/lang/System$Logger$Level;Ljava/lang/String;[Ljava/lang/Object;)V`
- `log(Ljava/lang/System$Logger$Level;Ljava/util/ResourceBundle;Ljava/lang/String;Ljava/lang/Throwable;)V`
- `log(Ljava/lang/System$Logger$Level;Ljava/util/ResourceBundle;Ljava/lang/String;[Ljava/lang/Object;)V`

### `java/lang/System$Logger$Level`

- `<init>()V`

### `java/lang/System$LoggerFinder`

- `<init>()V`
- `<init>(Ljava/lang/Void;)V`
- `accessProvider()Ljava/lang/System$LoggerFinder;`
- `checkPermission()Ljava/lang/Void;`
- `getLocalizedLogger(Ljava/lang/String;Ljava/util/ResourceBundle;Ljava/lang/Module;)Ljava/lang/System$Logger;`
- `getLogger(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/System$Logger;`

### `java/lang/Thread`

- `<init>(Ljava/lang/String;IZ)V`
- `<init>(Ljava/lang/ThreadGroup;Ljava/lang/String;ILjava/lang/Runnable;JLjava/security/AccessControlContext;)V`
- `checkAccess()V`
- `checkName(Ljava/lang/String;)Ljava/lang/String;`
- `contextClassLoader(Ljava/lang/Thread;)Ljava/lang/ClassLoader;`
- `currentCarrierThread()Ljava/lang/Thread;`
- `currentThread()Ljava/lang/Thread;`
- `dumpStack()V`
- `genThreadName()Ljava/lang/String;`
- `getContextClassLoader()Ljava/lang/ClassLoader;`
- `getName()Ljava/lang/String;`
- `getPriority()I`
- `getThreadGroup()Ljava/lang/ThreadGroup;`
- `holdsLock(Ljava/lang/Object;)Z`
- `interrupt()V`
- `interrupt0()V`
- `isCCLOverridden(Ljava/lang/Class;)Z`
- `isDaemon()Z`
- `isTerminated()Z`
- `isVirtual()Z`
- `setCurrentThread(Ljava/lang/Thread;)V`
- `threadId()J`
- `threadState()Ljava/lang/Thread$State;`
- `virtualThreadGroup()Ljava/lang/ThreadGroup;`
- `yield()V`
- `yield0()V`

### `java/lang/Thread$FieldHolder`

- `<init>()V`
- `<init>(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;JIZ)V`

### `java/lang/Thread$ThreadIdentifiers`

- `next()J`

### `java/lang/Thread$ThreadNumbering`

- `next()I`

### `java/lang/ThreadGroup`

- `getMaxPriority()I`

### `java/lang/ThreadLocal`

- `<init>()V`
- `childValue(Ljava/lang/Object;)Ljava/lang/Object;`
- `createInheritedMap(Ljava/lang/ThreadLocal$ThreadLocalMap;)Ljava/lang/ThreadLocal$ThreadLocalMap;`
- `createMap(Ljava/lang/Thread;Ljava/lang/Object;)V`
- `dumpStackIfVirtualThread()V`
- `get()Ljava/lang/Object;`
- `get(Ljava/lang/Thread;)Ljava/lang/Object;`
- `getMap(Ljava/lang/Thread;)Ljava/lang/ThreadLocal$ThreadLocalMap;`
- `initialValue()Ljava/lang/Object;`
- `nextHashCode()I`
- `set(Ljava/lang/Object;)V`
- `set(Ljava/lang/Thread;Ljava/lang/Object;)V`
- `setInitialValue(Ljava/lang/Thread;)Ljava/lang/Object;`

### `java/lang/ThreadLocal$ThreadLocalMap`

- `<init>()V`
- `<init>(Ljava/lang/ThreadLocal$ThreadLocalMap;)V`
- `<init>(Ljava/lang/ThreadLocal;Ljava/lang/Object;)V`
- `cleanSomeSlots(II)Z`
- `expungeStaleEntries()V`
- `expungeStaleEntry(I)I`
- `getEntry(Ljava/lang/ThreadLocal;)Ljava/lang/ThreadLocal$ThreadLocalMap$Entry;`
- `getEntryAfterMiss(Ljava/lang/ThreadLocal;ILjava/lang/ThreadLocal$ThreadLocalMap$Entry;)Ljava/lang/ThreadLocal$ThreadLocalMap$Entry;`
- `nextIndex(II)I`
- `prevIndex(II)I`
- `rehash()V`
- `replaceStaleEntry(Ljava/lang/ThreadLocal;Ljava/lang/Object;I)V`
- `resize()V`
- `set(Ljava/lang/ThreadLocal;Ljava/lang/Object;)V`
- `setThreshold(I)V`
- `size()I`

### `java/lang/ThreadLocal$ThreadLocalMap$Entry`

- `<init>()V`
- `<init>(Ljava/lang/ThreadLocal;Ljava/lang/Object;)V`
- `get()Ljava/lang/Object;`
- `refersTo(Ljava/lang/Object;)Z`

### `java/lang/Throwable`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V`
- `<init>(Ljava/lang/Throwable;)V`
- `addSuppressed(Ljava/lang/Throwable;)V`
- `fillInStackTrace()Ljava/lang/Throwable;`
- `fillInStackTrace(I)Ljava/lang/Throwable;`
- `getCause()Ljava/lang/Throwable;`
- `getLocalizedMessage()Ljava/lang/String;`
- `getMessage()Ljava/lang/String;`
- `getOurStackTrace()[Ljava/lang/StackTraceElement;`
- `getSuppressed()[Ljava/lang/Throwable;`
- `lockedPrintStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V`
- `printEnclosedStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V`
- `printStackTrace()V`
- `printStackTrace(Ljava/io/PrintStream;)V`
- `printStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V`
- `toString()Ljava/lang/String;`

### `java/lang/Throwable$PrintStreamOrWriter`

- `<init>()V`
- `isLockedByCurrentThread()Z`
- `lock()Ljava/lang/Object;`
- `println(Ljava/lang/Object;)V`

### `java/lang/Throwable$WrappedPrintStream`

- `<init>()V`
- `<init>(Ljava/io/PrintStream;)V`

### `java/lang/Throwable$WrappedPrintWriter`

- `<init>()V`
- `<init>(Ljava/io/PrintWriter;)V`

### `java/lang/TypeNotPresentException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/lang/UnsupportedClassVersionError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/UnsupportedOperationException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/VirtualMachineError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/VirtualThread`

- `<init>()V`
- `continuationScope()Ljdk/internal/vm/ContinuationScope;`
- `executeOnCarrierThread(Ljava/util/concurrent/Callable;)Ljava/lang/Object;`
- `notifyJvmtiHideFrames(Z)V`
- `notifyJvmtiMount(Z)V`
- `notifyJvmtiUnmount(Z)V`
- `setState(I)V`
- `state()I`
- `switchToCarrierThread()V`
- `switchToVirtualThread(Ljava/lang/VirtualThread;)V`
- `tryYield()V`
- `yieldContinuation()Z`

### `java/lang/VirtualThread$VThreadContinuation`

- `<init>()V`
- `<init>(Ljava/lang/VirtualThread;Ljava/lang/Runnable;)V`
- `wrap(Ljava/lang/VirtualThread;Ljava/lang/Runnable;)Ljava/lang/Runnable;`

### `java/lang/VirtualThread$VThreadContinuation$1`

- `<init>()V`
- `<init>(Ljava/lang/VirtualThread;Ljava/lang/Runnable;)V`

### `java/lang/Void`

- `<init>()V`

### `java/lang/WeakPairMap`

- `containsKeyPair(Ljava/lang/Object;Ljava/lang/Object;)Z`
- `expungeStaleAssociations()V`
- `get(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/lang/WeakPairMap$Pair`

- `hashCode(Ljava/lang/Object;Ljava/lang/Object;)I`
- `lookup(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/WeakPairMap$Pair;`
- `weak(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)Ljava/lang/WeakPairMap$Pair;`

### `java/lang/WeakPairMap$Pair$Lookup`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/lang/WeakPairMap$Pair$Weak`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`

### `java/lang/WeakPairMap$Pair$Weak$1`

- `<init>()V`
- `<init>(Ljava/lang/WeakPairMap$Pair$Weak;Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`

### `java/lang/WeakPairMap$WeakRefPeer`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`
- `weakPair()Ljava/lang/WeakPairMap$Pair$Weak;`

### `java/lang/constant/Constable`

- `<init>()V`

### `java/lang/invoke/BoundMethodHandle`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)V`
- `bindArgumentD(ID)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentF(IF)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentI(II)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentJ(IJ)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentL(ILjava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `bindSingle(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `copyWith(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/BoundMethodHandle;`
- `copyWithExtendD(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;D)Ljava/lang/invoke/BoundMethodHandle;`
- `copyWithExtendF(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;F)Ljava/lang/invoke/BoundMethodHandle;`
- `copyWithExtendI(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;I)Ljava/lang/invoke/BoundMethodHandle;`
- `copyWithExtendJ(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;J)Ljava/lang/invoke/BoundMethodHandle;`
- `copyWithExtendL(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `editor()Ljava/lang/invoke/LambdaFormEditor;`
- `fieldCount()I`
- `makeReinvoker(Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/BoundMethodHandle;`
- `rebind()Ljava/lang/invoke/BoundMethodHandle;`
- `speciesData()Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `speciesDataFor(Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `speciesData_L()Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `tooComplex()Z`
- `type()Ljava/lang/invoke/MethodType;`

### `java/lang/invoke/BoundMethodHandle$Specializer`

- `findSpecies(Ljava/lang/Object;)Ljava/lang/invoke/ClassSpecializer$SpeciesData;`
- `topSpecies()Ljava/lang/invoke/ClassSpecializer$SpeciesData;`

### `java/lang/invoke/BoundMethodHandle$SpeciesData`

- `<init>()V`
- `extendWith(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `factory()Ljava/lang/invoke/MethodHandle;`
- `fieldCount()I`
- `getterFunction(I)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `getterFunctions()Ljava/util/List;`
- `key()Ljava/lang/Object;`

### `java/lang/invoke/BoundMethodHandle$Species_L`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/Object;)V`
- `make(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`

### `java/lang/invoke/ClassSpecializer$SpeciesData`

- `<init>()V`
- `<init>(Ljava/lang/invoke/ClassSpecializer;Ljava/lang/Object;)V`
- `deriveFieldTypes(Ljava/lang/Object;)Ljava/util/List;`

### `java/lang/invoke/DelegatingMethodHandle`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;)V`
- `chooseDelegatingForm(Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/LambdaForm;`
- `makeReinvokerForm(Ljava/lang/invoke/MethodHandle;ILjava/lang/Object;Ljava/lang/invoke/LambdaForm$NamedFunction;)Ljava/lang/invoke/LambdaForm;`
- `makeReinvokerForm(Ljava/lang/invoke/MethodHandle;ILjava/lang/Object;ZLjava/lang/invoke/LambdaForm$NamedFunction;Ljava/lang/invoke/LambdaForm$NamedFunction;)Ljava/lang/invoke/LambdaForm;`
- `whichKind(I)Ljava/lang/invoke/LambdaForm$Kind;`

### `java/lang/invoke/DirectMethodHandle`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;Z)V`
- `afIndex(BZI)I`
- `copyWith(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/MethodHandle;`
- `createFunction(B)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `findDirectMethodHandle(Ljava/lang/invoke/LambdaForm$Name;)Ljava/lang/Object;`
- `ftypeKind(Ljava/lang/Class;)I`
- `getFieldKind(ZZLsun/invoke/util/Wrapper;)Ljava/lang/invoke/LambdaForm$Kind;`
- `getFunction(B)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `getNamedFunction(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `isVarargsCollector()Z`
- `make(BLjava/lang/Class;Ljava/lang/invoke/MemberName;Ljava/lang/Class;)Ljava/lang/invoke/DirectMethodHandle;`
- `make(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)Ljava/lang/invoke/DirectMethodHandle;`
- `make(Ljava/lang/invoke/MemberName;)Ljava/lang/invoke/DirectMethodHandle;`
- `makeAllocator(Ljava/lang/invoke/MemberName;)Ljava/lang/invoke/DirectMethodHandle;`
- `makePreparedFieldLambdaForm(BZI)Ljava/lang/invoke/LambdaForm;`
- `makePreparedLambdaForm(Ljava/lang/invoke/MethodType;I)Ljava/lang/invoke/LambdaForm;`
- `maybeCompile(Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;)V`
- `preparedFieldLambdaForm(BZLjava/lang/Class;)Ljava/lang/invoke/LambdaForm;`
- `preparedFieldLambdaForm(Ljava/lang/invoke/MemberName;)Ljava/lang/invoke/LambdaForm;`
- `preparedLambdaForm(Ljava/lang/invoke/MemberName;)Ljava/lang/invoke/LambdaForm;`
- `preparedLambdaForm(Ljava/lang/invoke/MemberName;Z)Ljava/lang/invoke/LambdaForm;`
- `preparedLambdaForm(Ljava/lang/invoke/MethodType;I)Ljava/lang/invoke/LambdaForm;`
- `shouldBeInitialized(Ljava/lang/invoke/MemberName;)Z`
- `type()Ljava/lang/invoke/MethodType;`
- `viewAsTypeChecks(Ljava/lang/invoke/MethodType;Z)Z`

### `java/lang/invoke/DirectMethodHandle$Accessor`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZI)V`

### `java/lang/invoke/DirectMethodHandle$Constructor`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/invoke/MemberName;Ljava/lang/Class;)V`

### `java/lang/invoke/DirectMethodHandle$Interface`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/Class;)V`

### `java/lang/invoke/DirectMethodHandle$Special`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/Class;)V`

### `java/lang/invoke/DirectMethodHandle$StaticAccessor`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/Object;J)V`

### `java/lang/invoke/InvokerBytecodeGenerator`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MethodType;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/invoke/MethodType;)V`
- `<init>(Ljava/lang/String;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MethodType;)V`
- `<init>(Ljava/lang/invoke/LambdaForm;ILjava/lang/String;Ljava/lang/String;Ljava/lang/invoke/MethodType;)V`
- `addMethod()V`
- `arrayInsnOpcode(BI)I`
- `arrayTypeCode(Lsun/invoke/util/Wrapper;)B`
- `assertStaticType(Ljava/lang/Class;Ljava/lang/invoke/LambdaForm$Name;)Z`
- `bogusMethod(Ljava/lang/Object;)V`
- `checkActualReceiver()Z`
- `classData(Ljava/lang/Object;)Ljava/lang/String;`
- `classDataValues()Ljava/lang/Object;`
- `classFilePrologue()Ljdk/internal/org/objectweb/asm/ClassWriter;`
- `clinit(Ljdk/internal/org/objectweb/asm/ClassWriter;Ljava/lang/String;Ljava/util/List;)V`
- `emitAloadInsn(I)V`
- `emitArrayLength(Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitArrayLoad(Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitArrayOp(Ljava/lang/invoke/LambdaForm$Name;I)V`
- `emitArrayStore(Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitAstoreInsn(I)V`
- `emitBoxing(Lsun/invoke/util/Wrapper;)V`
- `emitConst(Ljava/lang/Object;)V`
- `emitGuardWithCatch(I)Ljava/lang/invoke/LambdaForm$Name;`
- `emitI2X(Lsun/invoke/util/Wrapper;)V`
- `emitIconstInsn(I)V`
- `emitIconstInsn(Ljdk/internal/org/objectweb/asm/MethodVisitor;I)V`
- `emitImplicitConversion(Ljava/lang/invoke/LambdaForm$BasicType;Ljava/lang/Class;Ljava/lang/Object;)V`
- `emitInvoke(Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitLoadInsn(Ljava/lang/invoke/LambdaForm$BasicType;I)V`
- `emitLoop(I)Ljava/lang/invoke/LambdaForm$Name;`
- `emitLoopHandleInvoke(Ljava/lang/invoke/LambdaForm$Name;IILjava/lang/invoke/LambdaForm$Name;ZLjava/lang/invoke/MethodType;[Ljava/lang/Class;II)V`
- `emitPopInsn(Ljava/lang/invoke/LambdaForm$BasicType;)V`
- `emitPrimCast(Lsun/invoke/util/Wrapper;Lsun/invoke/util/Wrapper;)V`
- `emitPushArgument(Ljava/lang/Class;Ljava/lang/Object;)V`
- `emitPushArgument(Ljava/lang/invoke/LambdaForm$Name;I)V`
- `emitPushArguments(Ljava/lang/invoke/LambdaForm$Name;I)V`
- `emitPushClauseArray(II)V`
- `emitReferenceCast(Ljava/lang/Class;Ljava/lang/Object;)V`
- `emitReturn(Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitReturnInsn(Ljava/lang/invoke/LambdaForm$BasicType;)V`
- `emitSelectAlternative(Ljava/lang/invoke/LambdaForm$Name;Ljava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaForm$Name;`
- `emitStaticInvoke(Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitStaticInvoke(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitStoreInsn(Ljava/lang/invoke/LambdaForm$BasicType;I)V`
- `emitStoreResult(Ljava/lang/invoke/LambdaForm$Name;)V`
- `emitTableSwitch(II)Ljava/lang/invoke/LambdaForm$Name;`
- `emitTryFinally(I)Ljava/lang/invoke/LambdaForm$Name;`
- `emitUnboxing(Lsun/invoke/util/Wrapper;)V`
- `emitX2I(Lsun/invoke/util/Wrapper;)V`
- `emitZero(Ljava/lang/invoke/LambdaForm$BasicType;)V`
- `extendLocalsMap([Ljava/lang/Class;)I`
- `generateCustomizedCode(Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `generateCustomizedCodeBytes()[B`
- `generateLambdaFormInterpreterEntryPoint(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `generateLambdaFormInterpreterEntryPointBytes()[B`
- `getInternalName(Ljava/lang/Class;)Ljava/lang/String;`
- `isStaticallyInvocable(Ljava/lang/invoke/LambdaForm$Name;)Z`
- `isStaticallyInvocable(Ljava/lang/invoke/MemberName;)Z`
- `isStaticallyInvocable([Ljava/lang/invoke/LambdaForm$NamedFunction;)Z`
- `isStaticallyInvocableType(Ljava/lang/invoke/MethodType;)Z`
- `isStaticallyNameable(Ljava/lang/Class;)Z`
- `loadInsnOpcode(Ljava/lang/invoke/LambdaForm$BasicType;)I`
- `loadMethod([B)Ljava/lang/invoke/MemberName;`
- `lookupPregenerated(Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `makeDumpableClassName(Ljava/lang/String;)Ljava/lang/String;`
- `methodEpilogue()V`
- `methodPrologue()V`
- `popInsnOpcode(Ljava/lang/invoke/LambdaForm$BasicType;)I`
- `refKindOpcode(B)I`
- `resolveFrom(Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)Ljava/lang/invoke/MemberName;`
- `resolveInvokerMember(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `setClassWriter(Ljdk/internal/org/objectweb/asm/ClassWriter;)V`
- `storeInsnOpcode(Ljava/lang/invoke/LambdaForm$BasicType;)I`
- `toByteArray()[B`

### `java/lang/invoke/InvokerBytecodeGenerator$BytecodeGenerationException`

- `<init>()V`
- `<init>(Ljava/lang/Exception;)V`
- `printStackTrace(Ljava/io/PrintStream;)V`

### `java/lang/invoke/InvokerBytecodeGenerator$ClassData`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)V`

### `java/lang/invoke/Invokers`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;)V`
- `basicInvoker()Ljava/lang/invoke/MethodHandle;`
- `cachedInvoker(I)Ljava/lang/invoke/MethodHandle;`
- `cachedVHInvoker(ZLjava/lang/invoke/VarHandle$AccessMode;)Ljava/lang/invoke/MethodHandle;`
- `checkInvoker(Ljava/lang/invoke/MethodHandle;)Z`
- `checkVarHandleInvoker(Ljava/lang/invoke/MethodHandle;)Z`
- `createFunction(B)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `exactInvoker()Ljava/lang/invoke/MethodHandle;`
- `genericInvoker()Ljava/lang/invoke/MethodHandle;`
- `getFunction(B)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `getNamedFunction(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `invokeBasicMethod(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `invokeHandleForm(Ljava/lang/invoke/MethodType;ZI)Ljava/lang/invoke/LambdaForm;`
- `makeExactOrGeneralInvoker(Z)Ljava/lang/invoke/MethodHandle;`
- `makeVarHandleMethodInvoker(Ljava/lang/invoke/VarHandle$AccessMode;Z)Ljava/lang/invoke/MethodHandle;`
- `maybeCompileToBytecode(Ljava/lang/invoke/MethodHandle;)V`
- `setCachedInvoker(ILjava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `setCachedVHInvoker(ZLjava/lang/invoke/VarHandle$AccessMode;Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `varHandleMethodInvoker(Ljava/lang/invoke/VarHandle$AccessMode;)Ljava/lang/invoke/MethodHandle;`
- `varHandleMethodInvokerHandleForm(Ljava/lang/invoke/MethodType;Z)Ljava/lang/invoke/LambdaForm;`

### `java/lang/invoke/LambdaForm`

- `<init>()V`
- `<init>(IIZLjava/lang/invoke/MethodHandle;[Ljava/lang/invoke/LambdaForm$Name;Ljava/lang/invoke/LambdaForm$Kind;)V`
- `argument(ILjava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/LambdaForm$Name;`
- `arguments(ILjava/lang/invoke/MethodType;)[Ljava/lang/invoke/LambdaForm$Name;`
- `arity()I`
- `associateWithDebugName(Ljava/lang/invoke/LambdaForm;Ljava/lang/String;)V`
- `basicTypeSignature()Ljava/lang/String;`
- `basicTypeSignature(Ljava/lang/invoke/MethodType;)Ljava/lang/String;`
- `buildEmptyNames(ILjava/lang/invoke/MethodType;Z)[Ljava/lang/invoke/LambdaForm$Name;`
- `compileToBytecode()V`
- `constantZero(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `create(I[Ljava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaForm;`
- `create(I[Ljava/lang/invoke/LambdaForm$Name;I)Ljava/lang/invoke/LambdaForm;`
- `create(I[Ljava/lang/invoke/LambdaForm$Name;ILjava/lang/invoke/LambdaForm$Kind;)Ljava/lang/invoke/LambdaForm;`
- `create(I[Ljava/lang/invoke/LambdaForm$Name;IZLjava/lang/invoke/MethodHandle;Ljava/lang/invoke/LambdaForm$Kind;)Ljava/lang/invoke/LambdaForm;`
- `create(I[Ljava/lang/invoke/LambdaForm$Name;Ljava/lang/invoke/LambdaForm$Kind;)Ljava/lang/invoke/LambdaForm;`
- `create(I[Ljava/lang/invoke/LambdaForm$Name;ZLjava/lang/invoke/LambdaForm$Kind;)Ljava/lang/invoke/LambdaForm;`
- `createBlankForType(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/LambdaForm;`
- `createFormsFor(Ljava/lang/invoke/LambdaForm$BasicType;)V`
- `debugNames()Z`
- `debugString(I)Ljava/lang/String;`
- `editor()Ljava/lang/invoke/LambdaFormEditor;`
- `expressionCount()I`
- `failedCompilationCounter()Ljdk/internal/perf/PerfCounter;`
- `fixResult(I[Ljava/lang/invoke/LambdaForm$Name;)I`
- `forceInterpretation()Z`
- `generateDebugName()Ljava/lang/String;`
- `identity(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `identityForm(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/LambdaForm;`
- `internArgument(Ljava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaForm$Name;`
- `isEmpty()Z`
- `isGuardWithCatch(I)Z`
- `isLoop(I)Z`
- `isMatchingIdiom(ILjava/lang/String;I)Z`
- `isSelectAlternative(I)Z`
- `isTableSwitch(I)Z`
- `isTryFinally(I)Z`
- `isValidSignature(Ljava/lang/String;)Z`
- `lambdaName()Ljava/lang/String;`
- `lastUseIndex(Ljava/lang/invoke/LambdaForm$Name;)I`
- `methodType()Ljava/lang/invoke/MethodType;`
- `nameRefsAreLegal()Z`
- `namesOK(I[Ljava/lang/invoke/LambdaForm$Name;)Z`
- `normalizeNames(I[Ljava/lang/invoke/LambdaForm$Name;)Z`
- `parameter(I)Ljava/lang/invoke/LambdaForm$Name;`
- `parameterConstraint(I)Ljava/lang/Object;`
- `parameterType(I)Ljava/lang/invoke/LambdaForm$BasicType;`
- `prepare()V`
- `returnType()Ljava/lang/invoke/LambdaForm$BasicType;`
- `shortenSignature(Ljava/lang/String;)Ljava/lang/String;`
- `toString()Ljava/lang/String;`
- `traceInterpreter(Ljava/lang/String;Ljava/lang/Object;)V`
- `traceInterpreter(Ljava/lang/String;Ljava/lang/Object;[Ljava/lang/Object;)V`
- `uncustomize()Ljava/lang/invoke/LambdaForm;`
- `useCount(Ljava/lang/invoke/LambdaForm$Name;)I`
- `zeroForm(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/LambdaForm;`

### `java/lang/invoke/LambdaForm$BasicType`

- `basicType(C)Ljava/lang/invoke/LambdaForm$BasicType;`
- `basicType(Ljava/lang/Class;)Ljava/lang/invoke/LambdaForm$BasicType;`
- `basicTypeChar()C`
- `basicTypeChar(Ljava/lang/Class;)C`
- `basicTypeClass()Ljava/lang/Class;`
- `basicTypeSlots()I`
- `basicTypeWrapper()Lsun/invoke/util/Wrapper;`
- `basicTypesOrd([Ljava/lang/Class;)[I`
- `isArgBasicTypeChar(C)Z`
- `ordinal()I`

### `java/lang/invoke/LambdaForm$Kind`

- `ordinal()I`

### `java/lang/invoke/LambdaForm$Name`

- `<init>()V`
- `<init>(ILjava/lang/invoke/LambdaForm$BasicType;)V`
- `<init>(ILjava/lang/invoke/LambdaForm$BasicType;Ljava/lang/invoke/LambdaForm$NamedFunction;[Ljava/lang/Object;)V`
- `<init>(Ljava/lang/invoke/LambdaForm$BasicType;)V`
- `<init>(Ljava/lang/invoke/LambdaForm$Name;Ljava/lang/Object;)V`
- `<init>(Ljava/lang/invoke/LambdaForm$NamedFunction;)V`
- `<init>(Ljava/lang/invoke/LambdaForm$NamedFunction;Ljava/lang/Object;)V`
- `<init>(Ljava/lang/invoke/LambdaForm$NamedFunction;Ljava/lang/Object;Ljava/lang/Object;)V`
- `<init>(Ljava/lang/invoke/LambdaForm$NamedFunction;[Ljava/lang/Object;)V`
- `<init>(Ljava/lang/invoke/MemberName;[Ljava/lang/Object;)V`
- `<init>(Ljava/lang/invoke/MethodHandle;[Ljava/lang/Object;)V`
- `<init>(Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)V`
- `cloneWithIndex(I)Ljava/lang/invoke/LambdaForm$Name;`
- `debugString()Ljava/lang/String;`
- `exprString()Ljava/lang/String;`
- `index()I`
- `initIndex(I)Z`
- `internArguments()V`
- `isConstantZero()Z`
- `isInvokeBasic()Z`
- `isLinkerMethodInvoke()Z`
- `isParam()Z`
- `lastUseIndex(Ljava/lang/invoke/LambdaForm$Name;)I`
- `newIndex(I)Ljava/lang/invoke/LambdaForm$Name;`
- `paramString()Ljava/lang/String;`
- `refersTo(Ljava/lang/Class;Ljava/lang/String;)Z`
- `replaceNames([Ljava/lang/invoke/LambdaForm$Name;[Ljava/lang/invoke/LambdaForm$Name;II)Ljava/lang/invoke/LambdaForm$Name;`
- `toString()Ljava/lang/String;`
- `type()Ljava/lang/invoke/LambdaForm$BasicType;`
- `typeChar()C`
- `typesMatch(Ljava/lang/invoke/LambdaForm$BasicType;Ljava/lang/Object;)Z`
- `typesMatch(Ljava/lang/invoke/LambdaForm$NamedFunction;[Ljava/lang/Object;)Z`
- `useCount(Ljava/lang/invoke/LambdaForm$Name;)I`
- `withConstraint(Ljava/lang/Object;)Ljava/lang/invoke/LambdaForm$Name;`

### `java/lang/invoke/LambdaForm$NamedFunction`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MemberName;)V`
- `<init>(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodHandle;)V`
- `<init>(Ljava/lang/invoke/MethodHandle;)V`
- `<init>(Ljava/lang/invoke/MethodType;)V`
- `<init>(Ljava/lang/reflect/Method;)V`
- `arity()I`
- `assertMemberIsConsistent()Z`
- `calculateMethodType(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodType;`
- `equals(Ljava/lang/Object;)Z`
- `intrinsicData()Ljava/lang/Object;`
- `intrinsicName()Ljava/lang/invoke/MethodHandleImpl$Intrinsic;`
- `isConstantZero()Z`
- `isIdentity()Z`
- `isInvokeBasic(Ljava/lang/invoke/MemberName;)Z`
- `member()Ljava/lang/invoke/MemberName;`
- `methodType()Ljava/lang/invoke/MethodType;`
- `parameterType(I)Ljava/lang/invoke/LambdaForm$BasicType;`
- `resolve()V`
- `resolvedHandle()Ljava/lang/invoke/MethodHandle;`
- `returnType()Ljava/lang/invoke/LambdaForm$BasicType;`
- `toString()Ljava/lang/String;`

### `java/lang/invoke/LambdaFormBuffer`

- `<init>()V`
- `<init>(Ljava/lang/invoke/LambdaForm;)V`
- `changeName(ILjava/lang/invoke/LambdaForm$Name;)V`
- `clearDuplicatesAndNulls()V`
- `copyNamesInto([Ljava/lang/invoke/LambdaForm$Name;)[Ljava/lang/invoke/LambdaForm$Name;`
- `endEdit()Ljava/lang/invoke/LambdaForm;`
- `growNames(II)V`
- `inTrans()Z`
- `indexOf(Ljava/lang/invoke/LambdaForm$Name;[Ljava/lang/invoke/LambdaForm$Name;)I`
- `indexOf(Ljava/lang/invoke/LambdaForm$NamedFunction;Ljava/util/List;)I`
- `insertExpression(ILjava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaFormBuffer;`
- `insertName(ILjava/lang/invoke/LambdaForm$Name;Z)V`
- `insertParameter(ILjava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaFormBuffer;`
- `lambdaForm()Ljava/lang/invoke/LambdaForm;`
- `lastIndexOf(Ljava/lang/invoke/LambdaForm$Name;)I`
- `nameArray()[Ljava/lang/invoke/LambdaForm$Name;`
- `noteDuplicate(II)V`
- `ownedCount()I`
- `renameParameter(ILjava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaFormBuffer;`
- `replaceFunctions(Ljava/util/List;Ljava/util/List;[Ljava/lang/Object;)Ljava/lang/invoke/LambdaFormBuffer;`
- `replaceName(ILjava/lang/invoke/LambdaForm$Name;)V`
- `replaceParameterByCopy(II)Ljava/lang/invoke/LambdaFormBuffer;`
- `replaceParameterByNewExpression(ILjava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaFormBuffer;`
- `resultIndex()I`
- `setNames([Ljava/lang/invoke/LambdaForm$Name;)V`
- `setResult(Ljava/lang/invoke/LambdaForm$Name;)V`
- `startEdit()V`
- `verifyArity()Z`
- `verifyFirstChange()Z`

### `java/lang/invoke/LambdaFormEditor`

- `<init>()V`
- `<init>(Ljava/lang/invoke/LambdaForm;)V`
- `addArgumentForm(ILjava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/LambdaForm;`
- `bindArgumentD(Ljava/lang/invoke/BoundMethodHandle;ID)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentF(Ljava/lang/invoke/BoundMethodHandle;IF)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentForm(I)Ljava/lang/invoke/LambdaForm;`
- `bindArgumentI(Ljava/lang/invoke/BoundMethodHandle;II)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentJ(Ljava/lang/invoke/BoundMethodHandle;IJ)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentL(Ljava/lang/invoke/BoundMethodHandle;ILjava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `bindArgumentType(Ljava/lang/invoke/BoundMethodHandle;ILjava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/MethodType;`
- `buffer()Ljava/lang/invoke/LambdaFormBuffer;`
- `collectArgumentsForm(ILjava/lang/invoke/MethodType;)Ljava/lang/invoke/LambdaForm;`
- `filterArgumentForm(ILjava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/LambdaForm;`
- `filterRepeatedArgumentForm(Ljava/lang/invoke/LambdaForm$BasicType;[I)Ljava/lang/invoke/LambdaForm;`
- `filterReturnForm(Ljava/lang/invoke/LambdaForm$BasicType;Z)Ljava/lang/invoke/LambdaForm;`
- `foldArgumentsForm(IZLjava/lang/invoke/MethodType;)Ljava/lang/invoke/LambdaForm;`
- `formParametersMatch(Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/LambdaForm$BasicType;[I)Z`
- `getInCache(Ljava/lang/invoke/LambdaFormEditor$TransformKey;)Ljava/lang/invoke/LambdaForm;`
- `lambdaFormEditor(Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/LambdaFormEditor;`
- `makeArgumentCombinationForm(ILjava/lang/invoke/MethodType;ZZ)Ljava/lang/invoke/LambdaForm;`
- `makeArgumentCombinationForm(ILjava/lang/invoke/MethodType;[IZZ)Ljava/lang/invoke/LambdaForm;`
- `makeRepeatedFilterForm(Ljava/lang/invoke/MethodType;[I)Ljava/lang/invoke/LambdaForm;`
- `newSpeciesData(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `oldSpeciesData()Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `putInCache(Ljava/lang/invoke/LambdaFormEditor$TransformKey;Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/LambdaForm;`
- `spreadArgumentsForm(ILjava/lang/Class;I)Ljava/lang/invoke/LambdaForm;`

### `java/lang/invoke/LambdaFormEditor$1`

- `<init>()V`
- `<init>(Ljava/lang/invoke/LambdaFormEditor;)V`

### `java/lang/invoke/LambdaFormEditor$Transform`

- `<init>()V`
- `<init>(J[BLjava/lang/invoke/LambdaForm;)V`
- `equals(Ljava/lang/invoke/LambdaFormEditor$Transform;)Z`
- `equals(Ljava/lang/invoke/LambdaFormEditor$TransformKey;)Z`
- `get()Ljava/lang/Object;`

### `java/lang/invoke/LambdaFormEditor$TransformKey`

- `<init>()V`
- `<init>(J)V`
- `<init>([B)V`
- `bval(I)B`
- `equals(Ljava/lang/invoke/LambdaFormEditor$Transform;)Z`
- `equals(Ljava/lang/invoke/LambdaFormEditor$TransformKey;)Z`
- `fullBytes([I)[B`
- `inRange(I)Z`
- `ival(I)I`
- `of(BI)Ljava/lang/invoke/LambdaFormEditor$TransformKey;`
- `of(BII)Ljava/lang/invoke/LambdaFormEditor$TransformKey;`
- `of(BIII)Ljava/lang/invoke/LambdaFormEditor$TransformKey;`
- `of(BII[I)Ljava/lang/invoke/LambdaFormEditor$TransformKey;`
- `of(BI[I)Ljava/lang/invoke/LambdaFormEditor$TransformKey;`
- `packedBytes(BII[I)J`
- `packedBytes(BI[I)J`
- `packedBytes(B[I)J`
- `packedBytes(II)J`
- `packedBytes(III)J`
- `packedBytes(IIII)J`
- `packedBytes([B)J`
- `withResult(Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/LambdaFormEditor$Transform;`

### `java/lang/invoke/MemberName`

- `<init>()V`
- `<init>(Ljava/lang/Class;)V`
- `<init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;B)V`
- `<init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;B)V`
- `<init>(Ljava/lang/reflect/Field;Z)V`
- `<init>(Ljava/lang/reflect/Method;)V`
- `<init>(Ljava/lang/reflect/Method;Z)V`
- `allFlagsSet(I)Z`
- `anyFlagSet(I)Z`
- `asConstructor()Ljava/lang/invoke/MemberName;`
- `asSpecial()Ljava/lang/invoke/MemberName;`
- `canBeStaticallyBound()Z`
- `changeReferenceKind(BB)Ljava/lang/invoke/MemberName;`
- `checkForTypeAlias(Ljava/lang/Class;)V`
- `clone()Ljava/lang/invoke/MemberName;`
- `equals(Ljava/lang/invoke/MemberName;)Z`
- `expandFromVM()V`
- `flagsMods(IIB)I`
- `getClassLoader()Ljava/lang/ClassLoader;`
- `getDeclaringClass()Ljava/lang/Class;`
- `getFactory()Ljava/lang/invoke/MemberName$Factory;`
- `getFieldType()Ljava/lang/Class;`
- `getInvocationType()Ljava/lang/invoke/MethodType;`
- `getMethodOrFieldType()Ljava/lang/invoke/MethodType;`
- `getMethodType()Ljava/lang/invoke/MethodType;`
- `getModifiers()I`
- `getName()Ljava/lang/String;`
- `getName(Ljava/lang/Object;)Ljava/lang/String;`
- `getReferenceKind()B`
- `getType()Ljava/lang/Object;`
- `init(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Object;I)V`
- `initResolved(Z)V`
- `isAbstract()Z`
- `isCallerSensitive()Z`
- `isConstructor()Z`
- `isField()Z`
- `isGetter()Z`
- `isInvocable()Z`
- `isMethod()Z`
- `isMethodHandleInvoke()Z`
- `isMethodHandleInvokeName(Ljava/lang/String;)Z`
- `isObjectPublicMethod()Z`
- `isPrivate()Z`
- `isProtected()Z`
- `isPublic()Z`
- `isResolved()Z`
- `isSetter()Z`
- `isStatic()Z`
- `isType()Z`
- `isVarHandleMethodInvoke()Z`
- `isVarHandleMethodInvokeName(Ljava/lang/String;)Z`
- `isVarargs()Z`
- `isVolatile()Z`
- `makeAccessException()Ljava/lang/ReflectiveOperationException;`
- `makeAccessException(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/IllegalAccessException;`
- `makeMethodHandleInvoke(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `makeMethodHandleInvoke(Ljava/lang/String;Ljava/lang/invoke/MethodType;I)Ljava/lang/invoke/MemberName;`
- `makeVarHandleMethodInvoke(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `makeVarHandleMethodInvoke(Ljava/lang/String;Ljava/lang/invoke/MethodType;I)Ljava/lang/invoke/MemberName;`
- `matchingFlagsSet(II)Z`
- `message()Ljava/lang/String;`
- `referenceKindIsConsistent()Z`
- `referenceKindIsConsistentWith(I)Z`
- `refersTo(Ljava/lang/Class;Ljava/lang/String;)Z`
- `staticIsConsistent()Z`
- `toString()Ljava/lang/String;`
- `vminfoIsConsistent()Z`

### `java/lang/invoke/MemberName$Factory`

- `resolve(BLjava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;`
- `resolveOrFail(BLjava/lang/invoke/MemberName;Ljava/lang/Class;ILjava/lang/Class;)Ljava/lang/invoke/MemberName;`
- `resolveOrNull(BLjava/lang/invoke/MemberName;Ljava/lang/Class;I)Ljava/lang/invoke/MemberName;`

### `java/lang/invoke/MethodHandle`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)V`
- `asCollector(ILjava/lang/Class;I)Ljava/lang/invoke/MethodHandle;`
- `asCollector(Ljava/lang/Class;I)Ljava/lang/invoke/MethodHandle;`
- `asCollectorChecks(Ljava/lang/Class;II)Z`
- `asFixedArity()Ljava/lang/invoke/MethodHandle;`
- `asSpreader(ILjava/lang/Class;I)Ljava/lang/invoke/MethodHandle;`
- `asSpreader(Ljava/lang/Class;I)Ljava/lang/invoke/MethodHandle;`
- `asSpreaderChecks(Ljava/lang/Class;II)Ljava/lang/invoke/MethodType;`
- `asType(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `asTypeCached(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `asTypeUncached(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `asVarargsCollector(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `bindArgumentL(ILjava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `bindTo(Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;`
- `copyWith(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/MethodHandle;`
- `debugPrefix(I)Ljava/lang/String;`
- `getApproximateCommonClassLoader(Ljava/lang/invoke/MethodType;)Ljava/lang/ClassLoader;`
- `internalForm()Ljava/lang/invoke/LambdaForm;`
- `internalMemberName()Ljava/lang/invoke/MemberName;`
- `intrinsicData()Ljava/lang/Object;`
- `intrinsicName()Ljava/lang/invoke/MethodHandleImpl$Intrinsic;`
- `invokeBasic(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `invokeExact([B[Ljava/lang/Object;)Ljava/lang/Object;`
- `isAncestorLoaderOf(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`
- `isBuiltinLoader(Ljava/lang/ClassLoader;)Z`
- `isInvokeSpecial()Z`
- `isSafeToCache(Ljava/lang/invoke/MethodType;)Z`
- `isVarargsCollector()Z`
- `keepsAlive(Ljava/lang/Class;Ljava/lang/ClassLoader;)Z`
- `keepsAlive(Ljava/lang/invoke/MethodType;Ljava/lang/ClassLoader;)Z`
- `rebind()Ljava/lang/invoke/BoundMethodHandle;`
- `setAsTypeCache(Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `setVarargs(Ljava/lang/invoke/MemberName;)Ljava/lang/invoke/MethodHandle;`
- `spreadArrayChecks(Ljava/lang/Class;I)V`
- `type()Ljava/lang/invoke/MethodType;`
- `viewAsType(Ljava/lang/invoke/MethodType;Z)Ljava/lang/invoke/MethodHandle;`
- `viewAsTypeChecks(Ljava/lang/invoke/MethodType;Z)Z`
- `withInternalMemberName(Ljava/lang/invoke/MemberName;Z)Ljava/lang/invoke/MethodHandle;`
- `withVarargs(Z)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandleImpl`

- `assertCorrectArity(Ljava/lang/invoke/MethodHandle;I)Z`
- `bindCaller(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `computeValueConversions(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;ZZ)[Ljava/lang/Object;`
- `countNonNull([Ljava/lang/Object;)I`
- `createFunction(B)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `getConstantHandle(I)Ljava/lang/invoke/MethodHandle;`
- `getFunction(B)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `makeArrayElementAccessor(Ljava/lang/Class;Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/invoke/MethodHandle;`
- `makeCollector(Ljava/lang/Class;I)Ljava/lang/invoke/MethodHandle;`
- `makeCollectorForm(Ljava/lang/invoke/MethodType;Ljava/lang/Class;)Ljava/lang/invoke/LambdaForm;`
- `makeConstantHandle(I)Ljava/lang/invoke/MethodHandle;`
- `makeIntrinsic(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandleImpl$Intrinsic;)Ljava/lang/invoke/MethodHandle;`
- `makeIntrinsic(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandleImpl$Intrinsic;Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;`
- `makeIntrinsic(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MethodHandleImpl$Intrinsic;)Ljava/lang/invoke/MethodHandle;`
- `makePairwiseConvert(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;Z)Ljava/lang/invoke/MethodHandle;`
- `makePairwiseConvert(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;ZZ)Ljava/lang/invoke/MethodHandle;`
- `makePairwiseConvertByEditor(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;ZZ)Ljava/lang/invoke/MethodHandle;`
- `makeVarargsCollector(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `makeWrappedMember(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MemberName;Z)Ljava/lang/invoke/MethodHandle;`
- `setCachedHandle(ILjava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `unmatchedArrayAccess(Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/InternalError;`
- `valueConversion(Ljava/lang/Class;Ljava/lang/Class;ZZ)Ljava/lang/Object;`
- `varargsArray(I)Ljava/lang/invoke/MethodHandle;`
- `varargsArray(Ljava/lang/Class;I)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandleImpl$ArrayAccess`

- `cacheIndex(Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)I`
- `intrinsic(Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/invoke/MethodHandleImpl$Intrinsic;`
- `objectAccessor(Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/invoke/MethodHandle;`
- `opName(Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/String;`
- `ordinal()I`

### `java/lang/invoke/MethodHandleImpl$ArrayAccessor`

- `correctType(Ljava/lang/Class;Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/invoke/MethodType;`
- `getAccessor(Ljava/lang/Class;Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/invoke/MethodHandle;`
- `name(Ljava/lang/Class;Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/String;`
- `type(Ljava/lang/Class;Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/invoke/MethodType;`

### `java/lang/invoke/MethodHandleImpl$AsVarargsCollector`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)V`

### `java/lang/invoke/MethodHandleImpl$BindCaller`

- `bindCaller(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `bindCallerWithInjectedInvoker(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `prepareForInvoker(Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `restoreToType(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandleImpl$BindCaller$InjectedInvokerHolder`

- `<init>()V`
- `invoker()Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandleImpl$Intrinsic`

- `ordinal()I`

### `java/lang/invoke/MethodHandleImpl$IntrinsicMethodHandle`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandleImpl$Intrinsic;)V`
- `<init>(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandleImpl$Intrinsic;Ljava/lang/Object;)V`

### `java/lang/invoke/MethodHandleImpl$WrappedMember`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;ZLjava/lang/Class;)V`

### `java/lang/invoke/MethodHandleNatives`

- `canBeCalledVirtual(Ljava/lang/invoke/MemberName;)Z`
- `canBeCalledVirtual(Ljava/lang/invoke/MemberName;Ljava/lang/Class;)Z`
- `expand(Ljava/lang/invoke/MemberName;)V`
- `getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;`
- `init(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V`
- `isCallerSensitive(Ljava/lang/invoke/MemberName;)Z`
- `objectFieldOffset(Ljava/lang/invoke/MemberName;)J`
- `refKindDoesDispatch(B)Z`
- `refKindHasReceiver(B)Z`
- `refKindIsField(B)Z`
- `refKindIsGetter(B)Z`
- `refKindIsMethod(B)Z`
- `refKindIsSetter(B)Z`
- `refKindIsStatic(B)Z`
- `refKindIsValid(I)Z`
- `refKindName(B)Ljava/lang/String;`
- `resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;`
- `staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;`
- `staticFieldOffset(Ljava/lang/invoke/MemberName;)J`

### `java/lang/invoke/MethodHandleStatics`

- `dumper()Ljdk/internal/util/ClassFileDumper;`
- `message(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/String;`
- `message(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/String;`
- `newIllegalArgumentException(Ljava/lang/String;)Ljava/lang/RuntimeException;`
- `newIllegalArgumentException(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/RuntimeException;`
- `newIllegalArgumentException(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/RuntimeException;`
- `newInternalError(Ljava/lang/Exception;)Ljava/lang/InternalError;`
- `newInternalError(Ljava/lang/String;)Ljava/lang/InternalError;`
- `newInternalError(Ljava/lang/String;Ljava/lang/Exception;)Ljava/lang/InternalError;`
- `traceLambdaForm(Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `uncaughtException(Ljava/lang/Throwable;)Ljava/lang/Error;`

### `java/lang/invoke/MethodHandles`

- `arrayConstructor(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `arrayElementGetter(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `basicInvoker(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `dropArgumentChecks(Ljava/lang/invoke/MethodType;I[Ljava/lang/Class;)I`
- `dropArguments(Ljava/lang/invoke/MethodHandle;I[Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `dropArgumentsTrusted(Ljava/lang/invoke/MethodHandle;I[Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `empty(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `exactInvoker(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `foldArgumentChecks(ILjava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;`
- `foldArguments(Ljava/lang/invoke/MethodHandle;ILjava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `identity(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `insertArgumentPrimitive(Ljava/lang/invoke/BoundMethodHandle;ILjava/lang/Class;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `insertArguments(Ljava/lang/invoke/MethodHandle;I[Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;`
- `insertArgumentsChecks(Ljava/lang/invoke/MethodHandle;II)[Ljava/lang/Class;`
- `invoker(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `makeIdentity(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `makeZero(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `misMatchedTypes(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/RuntimeException;`
- `publicLookup()Ljava/lang/invoke/MethodHandles$Lookup;`
- `setCachedMethodHandle([Ljava/lang/invoke/MethodHandle;ILjava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `varHandleInvoker(Ljava/lang/invoke/VarHandle$AccessMode;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `zero(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `zero(Lsun/invoke/util/Wrapper;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandles$Lookup`

- `<init>()V`
- `<init>(Ljava/lang/Class;Ljava/lang/Class;I)V`
- `accessFailedMessage(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)Ljava/lang/String;`
- `checkAccess(BLjava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `checkMethod(BLjava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `checkMethodName(BLjava/lang/String;)V`
- `checkSecurityManager(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `checkSymbolicClass(Ljava/lang/Class;)V`
- `defaultDumper()Ljdk/internal/util/ClassFileDumper;`
- `findBoundCallerLookup(Ljava/lang/invoke/MemberName;)Ljava/lang/invoke/MethodHandles$Lookup;`
- `findStatic(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `findVirtual(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `findVirtualForMH(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `findVirtualForVH(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `fixmods(I)I`
- `getDirectMethod(BLjava/lang/Class;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/invoke/MethodHandle;`
- `getDirectMethodCommon(BLjava/lang/Class;Ljava/lang/invoke/MemberName;ZZLjava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/invoke/MethodHandle;`
- `hasFullPrivilegeAccess()Z`
- `isArrayClone(BLjava/lang/Class;Ljava/lang/invoke/MemberName;)Z`
- `isClassAccessible(Ljava/lang/Class;)Z`
- `lookupClass()Ljava/lang/Class;`
- `lookupClassOrNull()Ljava/lang/Class;`
- `lookupClassProtectionDomain()Ljava/security/ProtectionDomain;`
- `lookupModes()I`
- `makeHiddenClassDefiner(Ljava/lang/String;[BLjava/util/Set;Ljdk/internal/util/ClassFileDumper;)Ljava/lang/invoke/MethodHandles$Lookup$ClassDefiner;`
- `makeHiddenClassDefiner(Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;Ljava/util/Set;ZLjdk/internal/util/ClassFileDumper;)Ljava/lang/invoke/MethodHandles$Lookup$ClassDefiner;`
- `maybeBindCaller(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/invoke/MethodHandle;`
- `previousLookupClass()Ljava/lang/Class;`
- `resolveOrFail(BLjava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `resolveOrNull(BLjava/lang/invoke/MemberName;)Ljava/lang/invoke/MemberName;`
- `restrictProtectedReceiver(Ljava/lang/invoke/MemberName;)Z`
- `restrictReceiver(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/DirectMethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandles$Lookup$ClassDefiner`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;ILjdk/internal/util/ClassFileDumper;)V`
- `defineClass(ZLjava/lang/Object;)Ljava/lang/Class;`
- `internalName()Ljava/lang/String;`
- `isNestmate()Z`

### `java/lang/invoke/MethodHandles$Lookup$ClassFile`

- `<init>()V`
- `<init>(Ljava/lang/String;I[B)V`
- `newInstance([BLjava/lang/String;)Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;`
- `newInstanceNoCheck(Ljava/lang/String;[B)Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;`
- `readClassFile([B)Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;`
- `readInt([BI)I`
- `readUnsignedShort([BI)I`

### `java/lang/invoke/MethodHandles$Lookup$ClassOption`

- `<init>()V`
- `optionsToFlag(Ljava/util/Set;)I`

### `java/lang/invoke/MethodType`

- `<init>()V`
- `<init>(Ljava/lang/Class;[Ljava/lang/Class;)V`
- `appendParameterTypes([Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `asCollectorType(Ljava/lang/Class;II)Ljava/lang/invoke/MethodType;`
- `asSpreaderType(Ljava/lang/Class;II)Ljava/lang/invoke/MethodType;`
- `basicType()Ljava/lang/invoke/MethodType;`
- `canConvert(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `canConvertParameters([Ljava/lang/Class;[Ljava/lang/Class;)Z`
- `changeParameterType(ILjava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `changeReturnType(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `checkPtypes([Ljava/lang/Class;)I`
- `checkSlotCount(I)V`
- `dropParameterTypes(II)Ljava/lang/invoke/MethodType;`
- `equals(Ljava/lang/Object;)Z`
- `equals(Ljava/lang/invoke/MethodType;)Z`
- `erase()Ljava/lang/invoke/MethodType;`
- `form()Ljava/lang/invoke/MethodTypeForm;`
- `fromDescriptor(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/invoke/MethodType;`
- `generic()Ljava/lang/invoke/MethodType;`
- `genericMethodType(I)Ljava/lang/invoke/MethodType;`
- `genericMethodType(IZ)Ljava/lang/invoke/MethodType;`
- `hasPrimitives()Z`
- `insertParameterTypes(ILjava/util/List;)Ljava/lang/invoke/MethodType;`
- `insertParameterTypes(I[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `invokerType()Ljava/lang/invoke/MethodType;`
- `invokers()Ljava/lang/invoke/Invokers;`
- `isAllObject([Ljava/lang/Class;I)Z`
- `isConvertibleTo(Ljava/lang/invoke/MethodType;)Z`
- `isGeneric()Z`
- `isViewableAs(Ljava/lang/invoke/MethodType;Z)Z`
- `lastParameterType()Ljava/lang/Class;`
- `leadingReferenceParameter()Ljava/lang/Class;`
- `listToArray(Ljava/util/List;)[Ljava/lang/Class;`
- `makeImpl(Ljava/lang/Class;[Ljava/lang/Class;Z)Ljava/lang/invoke/MethodType;`
- `methodType(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `methodType(Ljava/lang/Class;Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `methodType(Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `methodType(Ljava/lang/Class;Ljava/util/List;)Ljava/lang/invoke/MethodType;`
- `methodType(Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `methodType(Ljava/lang/Class;[Ljava/lang/Class;Z)Ljava/lang/invoke/MethodType;`
- `newIndexOutOfBoundsException(Ljava/lang/Object;)Ljava/lang/IndexOutOfBoundsException;`
- `parameterCount()I`
- `parameterSlotCount()I`
- `parameterType(I)Ljava/lang/Class;`
- `ptypes()[Ljava/lang/Class;`
- `replaceParameterTypes(II[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;`
- `returnType()Ljava/lang/Class;`
- `rtype()Ljava/lang/Class;`
- `toFieldDescriptorString(Ljava/lang/Class;)Ljava/lang/String;`
- `toMethodDescriptorString()Ljava/lang/String;`

### `java/lang/invoke/MethodTypeForm`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;)V`
- `basicType()Ljava/lang/invoke/MethodType;`
- `cachedLambdaForm(I)Ljava/lang/invoke/LambdaForm;`
- `cachedMethodHandle(I)Ljava/lang/invoke/MethodHandle;`
- `canonicalize(Ljava/lang/Class;I)Ljava/lang/Class;`
- `canonicalize(Ljava/lang/invoke/MethodType;I)Ljava/lang/invoke/MethodType;`
- `canonicalizeAll([Ljava/lang/Class;I)[Ljava/lang/Class;`
- `erasedType()Ljava/lang/invoke/MethodType;`
- `findForm(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodTypeForm;`
- `hasPrimitives()Z`
- `parameterSlotCount()I`
- `setCachedLambdaForm(ILjava/lang/invoke/LambdaForm;)Ljava/lang/invoke/LambdaForm;`
- `setCachedMethodHandle(ILjava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/SimpleMethodHandle`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)V`
- `make(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/BoundMethodHandle;`

### `java/lang/invoke/VarHandle$AccessDescriptor`

- `<init>()V`
- `<init>(Ljava/lang/invoke/MethodType;II)V`

### `java/lang/invoke/VarHandle$AccessMode`

- `methodName()Ljava/lang/String;`
- `ordinal()I`
- `valueFromMethodName(Ljava/lang/String;)Ljava/lang/invoke/VarHandle$AccessMode;`

### `java/lang/invoke/VarHandle$AccessType`

- `ordinal()I`

### `java/lang/invoke/WrongMethodTypeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/module/Configuration`

- `<init>()V`
- `configurations()Ljava/util/stream/Stream;`
- `empty()Ljava/lang/module/Configuration;`
- `findModule(Ljava/lang/String;)Ljava/util/Optional;`
- `modules()Ljava/util/Set;`
- `reads(Ljava/lang/module/ResolvedModule;)Ljava/util/Set;`

### `java/lang/module/FindException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/module/ModuleDescriptor`

- `<init>()V`
- `exports()Ljava/util/Set;`
- `isAutomatic()Z`
- `isOpen()Z`
- `name()Ljava/lang/String;`
- `opens()Ljava/util/Set;`
- `packages()Ljava/util/Set;`
- `provides()Ljava/util/Set;`
- `requires()Ljava/util/Set;`
- `uses()Ljava/util/Set;`
- `version()Ljava/util/Optional;`

### `java/lang/module/ModuleDescriptor$Exports`

- `<init>()V`
- `isQualified()Z`
- `source()Ljava/lang/String;`
- `targets()Ljava/util/Set;`

### `java/lang/module/ModuleDescriptor$Opens`

- `<init>()V`
- `isQualified()Z`
- `source()Ljava/lang/String;`
- `targets()Ljava/util/Set;`

### `java/lang/module/ModuleDescriptor$Provides`

- `<init>()V`
- `service()Ljava/lang/String;`

### `java/lang/module/ModuleDescriptor$Requires`

- `<init>()V`
- `modifiers()Ljava/util/Set;`
- `name()Ljava/lang/String;`

### `java/lang/module/ModuleDescriptor$Version`

- `<init>()V`
- `takeNumber(Ljava/lang/String;ILjava/util/List;)I`
- `takeString(Ljava/lang/String;ILjava/util/List;)I`

### `java/lang/module/ModuleFinder`

- `find(Ljava/lang/String;)Ljava/util/Optional;`

### `java/lang/module/ModuleReference`

- `<init>()V`
- `descriptor()Ljava/lang/module/ModuleDescriptor;`
- `location()Ljava/util/Optional;`

### `java/lang/module/ResolutionException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/module/ResolvedModule`

- `<init>()V`
- `<init>(Ljava/lang/module/Configuration;Ljava/lang/module/ModuleReference;)V`
- `configuration()Ljava/lang/module/Configuration;`
- `descriptor()Ljava/lang/module/ModuleDescriptor;`
- `name()Ljava/lang/String;`
- `reads()Ljava/util/Set;`
- `reference()Ljava/lang/module/ModuleReference;`

### `java/lang/module/Resolver`

- `checkExportSuppliers(Ljava/util/Map;)V`
- `checkHashes()V`
- `computeIfAbsent(Ljava/util/Map;Ljava/lang/String;Ljava/lang/module/Configuration;Ljava/lang/module/ModuleReference;)Ljava/lang/module/ResolvedModule;`
- `cycleAsString(Ljava/lang/module/ModuleDescriptor;)Ljava/lang/String;`
- `detectCycles()V`
- `failTwoSuppliers(Ljava/lang/module/ModuleDescriptor;Ljava/lang/String;Ljava/lang/module/ModuleDescriptor;Ljava/lang/module/ModuleDescriptor;)V`
- `findFail(Ljava/lang/String;[Ljava/lang/Object;)V`
- `findInParent(Ljava/lang/String;)Ljava/lang/module/ResolvedModule;`
- `finish(Ljava/lang/module/Configuration;)Ljava/util/Map;`
- `makeGraph(Ljava/lang/module/Configuration;)Ljava/util/Map;`
- `packageName(Ljava/lang/String;)Ljava/lang/String;`
- `resolveFail(Ljava/lang/String;[Ljava/lang/Object;)V`
- `targetPlatform()Ljava/lang/String;`
- `visit(Ljava/lang/module/ModuleDescriptor;)V`

### `java/lang/ref/Cleaner`

- `register(Ljava/lang/Object;Ljava/lang/Runnable;)Ljava/lang/ref/Cleaner$Cleanable;`

### `java/lang/ref/Cleaner$Cleanable`

- `clean()V`

### `java/lang/ref/FinalReference`

- `<init>()V`

### `java/lang/ref/Reference`

- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`
- `get()Ljava/lang/Object;`
- `reachabilityFence(Ljava/lang/Object;)V`

### `java/lang/ref/ReferenceQueue`

- `<init>()V`
- `headIsNull()Z`
- `poll()Ljava/lang/ref/Reference;`
- `poll0()Ljava/lang/ref/Reference;`

### `java/lang/ref/SoftReference`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`
- `get()Ljava/lang/Object;`

### `java/lang/ref/WeakReference`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`
- `get()Ljava/lang/Object;`

### `java/lang/reflect/AccessibleObject`

- `<init>()V`

### `java/lang/reflect/Array`

- `getLength(Ljava/lang/Object;)I`
- `multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;`
- `newArray(Ljava/lang/Class;I)Ljava/lang/Object;`
- `newInstance(Ljava/lang/Class;I)Ljava/lang/Object;`

### `java/lang/reflect/Constructor`

- `<init>()V`
- `acquireConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V`
- `getConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;`
- `getDeclaringClass()Ljava/lang/Class;`
- `getParameterTypes()[Ljava/lang/Class;`
- `newInstance([Ljava/lang/Object;)Ljava/lang/Object;`
- `newInstanceWithCaller([Ljava/lang/Object;ZLjava/lang/Class;)Ljava/lang/Object;`
- `setConstructorAccessor(Ljdk/internal/reflect/ConstructorAccessor;)V`

### `java/lang/reflect/Executable`

- `<init>()V`

### `java/lang/reflect/Field`

- `acquireFieldAccessor()Ljdk/internal/reflect/FieldAccessor;`
- `acquireOverrideFieldAccessor()Ljdk/internal/reflect/FieldAccessor;`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Object;)V`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getDeclaringClass()Ljava/lang/Class;`
- `getFieldAccessor()Ljdk/internal/reflect/FieldAccessor;`
- `getName()Ljava/lang/String;`
- `getOverrideFieldAccessor()Ljdk/internal/reflect/FieldAccessor;`
- `getType()Ljava/lang/Class;`
- `setFieldAccessor(Ljdk/internal/reflect/FieldAccessor;)V`
- `setOverrideFieldAccessor(Ljdk/internal/reflect/FieldAccessor;)V`

### `java/lang/reflect/InvocationTargetException`

- `<init>()V`
- `getCause()Ljava/lang/Throwable;`
- `getTargetException()Ljava/lang/Throwable;`

### `java/lang/reflect/Method`

- `acquireMethodAccessor()Ljdk/internal/reflect/MethodAccessor;`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V`
- `getDeclaringClass()Ljava/lang/Class;`
- `getMethodAccessor()Ljdk/internal/reflect/MethodAccessor;`
- `getModifiers()I`
- `getName()Ljava/lang/String;`
- `getParameterTypes()[Ljava/lang/Class;`
- `getReturnType()Ljava/lang/Class;`
- `invoke(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;`
- `isCallerSensitive()Z`
- `isDefault()Z`
- `setMethodAccessor(Ljdk/internal/reflect/MethodAccessor;)V`
- `sharedToString(IZ[Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `java/lang/reflect/Modifier`

- `isAbstract(I)Z`
- `isFinal(I)Z`
- `isPrivate(I)Z`
- `isProtected(I)Z`
- `isPublic(I)Z`
- `isStatic(I)Z`
- `isVolatile(I)Z`
- `methodModifiers()I`
- `toString(I)Ljava/lang/String;`

### `java/lang/reflect/ParameterizedType`

- `<init>()V`
- `getActualTypeArguments()[Ljava/lang/reflect/Type;`
- `getRawType()Ljava/lang/reflect/Type;`

### `java/lang/reflect/Proxy`

- `checkProxyAccess(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V`
- `getProxyClass(Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/Class;`
- `getProxyConstructor(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/reflect/Constructor;`
- `isProxyClass(Ljava/lang/Class;)Z`

### `java/lang/reflect/Proxy$ProxyBuilder`

- `isProxyClass(Ljava/lang/Class;)Z`

### `java/lang/reflect/RecordComponent`

- `getName()Ljava/lang/String;`
- `getType()Ljava/lang/Class;`

### `java/lang/reflect/Type`

- `<init>()V`

### `java/lang/reflect/UndeclaredThrowableException`

- `getCause()Ljava/lang/Throwable;`

### `java/math/BigDecimal`

- `<init>()V`
- `<init>(DLjava/math/MathContext;)V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/math/BigInteger;I)V`
- `<init>(Ljava/math/BigInteger;ILjava/math/MathContext;)V`
- `<init>(Ljava/math/BigInteger;JII)V`
- `<init>([CII)V`
- `<init>([CIILjava/math/MathContext;)V`
- `add(JIJI)Ljava/math/BigDecimal;`
- `add(JILjava/math/BigInteger;I)Ljava/math/BigDecimal;`
- `add(JJ)J`
- `add(JJI)Ljava/math/BigDecimal;`
- `add(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;`
- `add(Ljava/math/BigInteger;ILjava/math/BigInteger;I)Ljava/math/BigDecimal;`
- `bigDigitLength(Ljava/math/BigInteger;)I`
- `bigMultiplyPowerTen(I)Ljava/math/BigInteger;`
- `bigMultiplyPowerTen(JI)Ljava/math/BigInteger;`
- `bigMultiplyPowerTen(Ljava/math/BigInteger;I)Ljava/math/BigInteger;`
- `bigTenToThe(I)Ljava/math/BigInteger;`
- `checkScale(J)I`
- `checkScale(JJ)I`
- `checkScale(Ljava/math/BigInteger;J)I`
- `checkScaleNonZero(J)I`
- `commonNeedIncrement(IIIZ)Z`
- `compactValFor(Ljava/math/BigInteger;)J`
- `createAndStripZerosToMatchScale(JIJ)Ljava/math/BigDecimal;`
- `createAndStripZerosToMatchScale(Ljava/math/BigInteger;IJ)Ljava/math/BigDecimal;`
- `divRemNegativeLong(JJ)[J`
- `divideAndRound(JJI)J`
- `divideAndRound(JJIII)Ljava/math/BigDecimal;`
- `divideAndRound(Ljava/math/BigInteger;JI)Ljava/math/BigInteger;`
- `divideAndRound(Ljava/math/BigInteger;JIII)Ljava/math/BigDecimal;`
- `divideAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigInteger;`
- `divideAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;III)Ljava/math/BigDecimal;`
- `divideAndRound128(JJJIIII)Ljava/math/BigDecimal;`
- `divideAndRoundByTenPow(Ljava/math/BigInteger;II)Ljava/math/BigInteger;`
- `doRound(JILjava/math/MathContext;)Ljava/math/BigDecimal;`
- `doRound(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;`
- `doRound(Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;`
- `doRound128(JJIILjava/math/MathContext;)Ljava/math/BigDecimal;`
- `expandBigIntegerTenPowers(I)Ljava/math/BigInteger;`
- `inflated()Ljava/math/BigInteger;`
- `longCompareMagnitude(JJ)I`
- `longDigitLength(J)I`
- `longLongCompareMagnitude(JJJJ)Z`
- `longMultiplyPowerTen(JI)J`
- `make64(JJ)J`
- `matchScale([Ljava/math/BigDecimal;)V`
- `movePointRight(I)Ljava/math/BigDecimal;`
- `mulsub(JJJJJ)J`
- `multiply(JJ)J`
- `multiply(JJI)Ljava/math/BigDecimal;`
- `multiply(JLjava/math/BigInteger;I)Ljava/math/BigDecimal;`
- `multiply(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;`
- `multiply(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigDecimal;`
- `multiplyAndRound(JJILjava/math/MathContext;)Ljava/math/BigDecimal;`
- `multiplyAndRound(JLjava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;`
- `multiplyAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;`
- `needIncrement(JIIJJ)Z`
- `needIncrement(JIILjava/math/MutableBigInteger;J)Z`
- `needIncrement(Ljava/math/MutableBigInteger;IILjava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Z`
- `parseExp([CII)J`
- `preAlign(Ljava/math/BigDecimal;Ljava/math/BigDecimal;JLjava/math/MathContext;)[Ljava/math/BigDecimal;`
- `precision()I`
- `precision(JJ)I`
- `scale()I`
- `setScale(I)Ljava/math/BigDecimal;`
- `setScale(II)Ljava/math/BigDecimal;`
- `signum()I`
- `stripZerosToMatchScale(Ljava/math/BigInteger;JII)Ljava/math/BigDecimal;`
- `toBigIntegerExact()Ljava/math/BigInteger;`
- `toStrictBigInteger(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `unsignedLongCompare(JJ)Z`
- `unsignedLongCompareEq(JJ)Z`
- `valueOf(J)Ljava/math/BigDecimal;`
- `valueOf(JI)Ljava/math/BigDecimal;`
- `valueOf(JII)Ljava/math/BigDecimal;`
- `valueOf(Ljava/math/BigInteger;II)Ljava/math/BigDecimal;`
- `zeroValueOf(I)Ljava/math/BigDecimal;`

### `java/math/BigInteger`

- `<init>()V`
- `<init>(ILjava/util/Random;)V`
- `<init>(I[B)V`
- `<init>(I[BII)V`
- `<init>(I[I)V`
- `<init>(J)V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;I)V`
- `<init>([B)V`
- `<init>([BII)V`
- `<init>([CII)V`
- `<init>([I)V`
- `<init>([II)V`
- `abs()Ljava/math/BigInteger;`
- `add(J)Ljava/math/BigInteger;`
- `add(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `add([IJ)[I`
- `add([I[I)[I`
- `addOne([IIII)I`
- `bitLength()I`
- `bitLength([II)I`
- `bitLengthForInt(I)I`
- `checkRange()V`
- `compareMagnitude(J)I`
- `compareMagnitude(Ljava/math/BigInteger;)I`
- `compareTo(Ljava/math/BigInteger;)I`
- `destructiveMulAdd([III)V`
- `divideAndRemainder(Ljava/math/BigInteger;)[Ljava/math/BigInteger;`
- `divideAndRemainderBurnikelZiegler(Ljava/math/BigInteger;)[Ljava/math/BigInteger;`
- `divideAndRemainderKnuth(Ljava/math/BigInteger;)[Ljava/math/BigInteger;`
- `equals(Ljava/lang/Object;)Z`
- `exactDivideBy3()Ljava/math/BigInteger;`
- `firstNonzeroIntNum()I`
- `getInt(I)I`
- `getLower(I)Ljava/math/BigInteger;`
- `getLowestSetBit()I`
- `getPrimeSearchLen(I)I`
- `getToomSlice(IIII)Ljava/math/BigInteger;`
- `getUpper(I)Ljava/math/BigInteger;`
- `implMontgomeryMultiply([I[I[IIJ[I)[I`
- `implMontgomeryMultiplyChecks([I[I[II[I)V`
- `implMontgomerySquare([I[IIJ[I)[I`
- `implMulAdd([I[IIII)I`
- `implMulAddCheck([I[IIII)V`
- `implMultiplyToLen([II[II[I)[I`
- `implSquareToLen([II[II)[I`
- `implSquareToLenChecks([II[II)V`
- `intArrayCmpToLen([I[II)I`
- `intLength()I`
- `intValue()I`
- `jacobiSymbol(ILjava/math/BigInteger;)I`
- `javaIncrement([I)[I`
- `largePrime(IILjava/util/Random;)Ljava/math/BigInteger;`
- `leftShift([III)[I`
- `longValue()J`
- `lucasLehmerSequence(ILjava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `makePositive(I[BII)[I`
- `makePositive([I)[I`
- `materialize([II)[I`
- `mod(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `mod2(I)Ljava/math/BigInteger;`
- `modInverse(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `modPow(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `modPow2(Ljava/math/BigInteger;I)Ljava/math/BigInteger;`
- `montReduce([I[III)[I`
- `montgomeryMultiply([I[I[IIJ[I)[I`
- `montgomerySquare([I[IIJ[I)[I`
- `mulAdd([I[IIII)I`
- `multiply(J)Ljava/math/BigInteger;`
- `multiply(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `multiply(Ljava/math/BigInteger;ZZI)Ljava/math/BigInteger;`
- `multiplyByInt([III)Ljava/math/BigInteger;`
- `multiplyKaratsuba(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `multiplyToLen([II[II[I)[I`
- `multiplyToLenCheck([II)V`
- `multiplyToomCook3(Ljava/math/BigInteger;Ljava/math/BigInteger;ZI)Ljava/math/BigInteger;`
- `negate()Ljava/math/BigInteger;`
- `oddModPow(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `parseInt([CII)I`
- `passesLucasLehmer()Z`
- `passesMillerRabin(ILjava/util/Random;)Z`
- `pow(I)Ljava/math/BigInteger;`
- `primeToCertainty(ILjava/util/Random;)Z`
- `primitiveLeftShift([III)V`
- `primitiveRightShift([III)V`
- `randomBits(ILjava/util/Random;)[B`
- `remainder(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `remainderBurnikelZiegler(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `remainderKnuth(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `reportOverflow()V`
- `setBit(I)Ljava/math/BigInteger;`
- `shiftLeft(I)Ljava/math/BigInteger;`
- `shiftLeft([II)[I`
- `shiftLeftImplWorker([I[IIII)V`
- `shiftRight(I)Ljava/math/BigInteger;`
- `shiftRightImpl(I)Ljava/math/BigInteger;`
- `shiftRightImplWorker([I[IIII)V`
- `signInt()I`
- `signum()I`
- `smallPrime(IILjava/util/Random;)Ljava/math/BigInteger;`
- `square()Ljava/math/BigInteger;`
- `square(ZZI)Ljava/math/BigInteger;`
- `squareKaratsuba()Ljava/math/BigInteger;`
- `squareToLen([II[I)[I`
- `squareToomCook3(ZI)Ljava/math/BigInteger;`
- `stripLeadingZeroBytes(I[BII)[I`
- `stripLeadingZeroBytes([BII)[I`
- `stripLeadingZeroInts([I)[I`
- `subN([I[II)I`
- `subtract(J[I)[I`
- `subtract(Ljava/math/BigInteger;)Ljava/math/BigInteger;`
- `subtract([IJ)[I`
- `subtract([I[I)[I`
- `testBit(I)Z`
- `toByteArray()[B`
- `trustedStripLeadingZeroInts([I)[I`
- `valueOf(J)Ljava/math/BigInteger;`
- `valueOf([I)Ljava/math/BigInteger;`

### `java/math/BigInteger$RecursiveOp`

- `<init>(ZI)V`
- `multiply(Ljava/math/BigInteger;Ljava/math/BigInteger;ZI)Ljava/util/concurrent/RecursiveTask;`
- `square(Ljava/math/BigInteger;ZI)Ljava/util/concurrent/RecursiveTask;`

### `java/math/BigInteger$RecursiveOp$RecursiveMultiply`

- `<init>()V`
- `<init>(Ljava/math/BigInteger;Ljava/math/BigInteger;ZI)V`
- `forkOrInvoke()Ljava/util/concurrent/RecursiveTask;`

### `java/math/BigInteger$RecursiveOp$RecursiveSquare`

- `<init>()V`
- `<init>(Ljava/math/BigInteger;ZI)V`
- `forkOrInvoke()Ljava/util/concurrent/RecursiveTask;`

### `java/math/BitSieve`

- `<init>()V`
- `<init>(Ljava/math/BigInteger;I)V`
- `bit(I)J`
- `get(I)Z`
- `retrieve(Ljava/math/BigInteger;ILjava/util/Random;)Ljava/math/BigInteger;`
- `set(I)V`
- `sieveSearch(II)I`
- `sieveSingle(III)V`
- `unitIndex(I)I`

### `java/math/MutableBigInteger`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/math/BigInteger;)V`
- `<init>(Ljava/math/MutableBigInteger;)V`
- `<init>([I)V`
- `add(Ljava/math/MutableBigInteger;)V`
- `addDisjoint(Ljava/math/MutableBigInteger;I)V`
- `addLower(Ljava/math/MutableBigInteger;I)V`
- `addShifted(Ljava/math/MutableBigInteger;I)V`
- `bitLength()J`
- `clear()V`
- `compare(Ljava/math/MutableBigInteger;)I`
- `compareHalf(Ljava/math/MutableBigInteger;)I`
- `compareShifted(Ljava/math/MutableBigInteger;I)I`
- `copyAndShift([III[III)V`
- `copyValue(Ljava/math/MutableBigInteger;)V`
- `divWord(JI)J`
- `divadd([I[II)I`
- `divaddLong(II[II)I`
- `divide(JLjava/math/MutableBigInteger;)J`
- `divide(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `divide(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;`
- `divide2n1n(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `divide3n2n(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `divideAndRemainderBurnikelZiegler(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `divideKnuth(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `divideKnuth(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;`
- `divideLongMagnitude(JLjava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `divideMagnitude(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;`
- `divideOneWord(ILjava/math/MutableBigInteger;)I`
- `euclidModInverse(I)Ljava/math/MutableBigInteger;`
- `fixup(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;I)Ljava/math/MutableBigInteger;`
- `getBlock(III)Ljava/math/MutableBigInteger;`
- `getLower(I)Ljava/math/BigInteger;`
- `getLowestSetBit()I`
- `getMagnitudeArray()[I`
- `inverseMod32(I)I`
- `inverseMod64(J)J`
- `isEven()Z`
- `isOdd()Z`
- `isOne()Z`
- `isZero()Z`
- `keepLower(I)V`
- `leftShift(I)V`
- `modInverse(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `modInverseBP2(Ljava/math/MutableBigInteger;I)Ljava/math/MutableBigInteger;`
- `modInverseMP2(I)Ljava/math/MutableBigInteger;`
- `mul(ILjava/math/MutableBigInteger;)V`
- `mulsub([I[IIII)I`
- `mulsubBorrow([I[IIII)I`
- `mulsubLong([IIIII)I`
- `multiply(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)V`
- `mutableModInverse(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `normalize()V`
- `ones(I)V`
- `primitiveLeftShift(I)V`
- `primitiveRightShift(I)V`
- `reset()V`
- `rightShift(I)V`
- `safeLeftShift(I)V`
- `safeRightShift(I)V`
- `setValue([II)V`
- `subtract(Ljava/math/MutableBigInteger;)I`
- `toBigDecimal(II)Ljava/math/BigDecimal;`
- `toBigInteger()Ljava/math/BigInteger;`
- `toBigInteger(I)Ljava/math/BigInteger;`
- `toCompactValue(I)J`
- `toIntArray()[I`
- `toLong()J`
- `unsignedLongCompare(JJ)Z`

### `java/math/RoundingMode`

- `<init>()V`
- `valueOf(I)Ljava/math/RoundingMode;`

### `java/math/SignedMutableBigInteger`

- `<init>()V`
- `<init>(I)V`
- `add(Ljava/math/MutableBigInteger;)V`
- `compare(Ljava/math/MutableBigInteger;)I`
- `copyValue(Ljava/math/MutableBigInteger;)V`
- `divide(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;`
- `leftShift(I)V`
- `signedAdd(Ljava/math/MutableBigInteger;)V`
- `signedAdd(Ljava/math/SignedMutableBigInteger;)V`
- `signedSubtract(Ljava/math/SignedMutableBigInteger;)V`
- `subtract(Ljava/math/MutableBigInteger;)I`

### `java/net/Inet4Address`

- `<init>()V`
- `<init>(Ljava/lang/String;[B)V`
- `holder()Ljava/net/InetAddress$InetAddressHolder;`

### `java/net/Inet6Address`

- `<init>()V`
- `<init>(Ljava/lang/String;[B)V`
- `<init>(Ljava/lang/String;[BI)V`
- `<init>(Ljava/lang/String;[BLjava/lang/String;)V`
- `deriveNumericScope([BLjava/net/NetworkInterface;)I`
- `getAddress()[B`
- `getScopeId()I`
- `initif(Ljava/lang/String;[BLjava/net/NetworkInterface;)V`
- `initstr(Ljava/lang/String;[BLjava/lang/String;)V`
- `isDifferentLocalAddressType([B[B)Z`
- `isLinkLocalAddress([B)Z`
- `isSiteLocalAddress([B)Z`

### `java/net/Inet6Address$Inet6AddressHolder`

- `<init>()V`
- `init([BI)V`
- `init([BLjava/net/NetworkInterface;)V`
- `isLinkLocalAddress()Z`
- `isSiteLocalAddress()Z`
- `setAddr([B)V`

### `java/net/InetAddress`

- `<init>()V`
- `anyLocalAddress()Ljava/net/InetAddress;`
- `checkNumericZone(Ljava/lang/String;)I`
- `equals(Ljava/lang/Object;)Z`
- `getAddress()[B`
- `getAllByName(Ljava/lang/String;)[Ljava/net/InetAddress;`
- `getAllByName0(Ljava/lang/String;Z)[Ljava/net/InetAddress;`
- `getAllByName0(Ljava/lang/String;ZZ)[Ljava/net/InetAddress;`
- `getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;`
- `getByAddress([B)Ljava/net/InetAddress;`
- `getByName(Ljava/lang/String;)Ljava/net/InetAddress;`
- `getHostAddress()Ljava/lang/String;`
- `getHostFromNameService(Ljava/net/InetAddress;Z)Ljava/lang/String;`
- `getHostName()Ljava/lang/String;`
- `getHostName(Z)Ljava/lang/String;`
- `holder()Ljava/net/InetAddress$InetAddressHolder;`
- `invalidIPv6LiteralException(Ljava/lang/String;Z)Ljava/net/UnknownHostException;`
- `loadResolver()Ljava/net/spi/InetAddressResolver;`
- `resolver()Ljava/net/spi/InetAddressResolver;`
- `validate(Ljava/lang/String;)V`

### `java/net/InetAddress$Addresses`

- `<init>()V`
- `get()[Ljava/net/InetAddress;`

### `java/net/InetAddress$CachedLookup`

- `<init>()V`
- `tryRemoveExpiredAddress(J)Z`

### `java/net/InetAddress$InetAddressHolder`

- `<init>()V`
- `getHostName()Ljava/lang/String;`
- `init(Ljava/lang/String;I)V`

### `java/net/InetAddress$NameServiceAddresses`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/net/InetAddressImpl`

- `anyLocalAddress()Ljava/net/InetAddress;`
- `loopbackAddress()Ljava/net/InetAddress;`

### `java/net/InetSocketAddress`

- `<init>()V`
- `<init>(Ljava/net/InetAddress;I)V`
- `checkHost(Ljava/lang/String;)Ljava/lang/String;`
- `checkPort(I)I`
- `getAddress()Ljava/net/InetAddress;`
- `getHostName()Ljava/lang/String;`
- `getPort()I`
- `isUnresolved()Z`

### `java/net/InetSocketAddress$InetSocketAddressHolder`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/net/InetAddress;I)V`
- `getAddress()Ljava/net/InetAddress;`
- `getHostName()Ljava/lang/String;`
- `getPort()I`
- `isUnresolved()Z`

### `java/net/MalformedURLException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/net/NetPermission`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`

### `java/net/NetworkInterface`

- `<init>()V`
- `enumerationFromArray([Ljava/lang/Object;)Ljava/util/Enumeration;`
- `getAll()[Ljava/net/NetworkInterface;`
- `getByName(Ljava/lang/String;)Ljava/net/NetworkInterface;`
- `getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;`
- `getCheckedInetAddresses()[Ljava/net/InetAddress;`
- `getInetAddresses()Ljava/util/Enumeration;`
- `getName()Ljava/lang/String;`
- `getNetworkInterfaces()Ljava/util/Enumeration;`

### `java/net/NetworkInterface$1`

- `<init>()V`
- `<init>([Ljava/lang/Object;)V`

### `java/net/Proxy`

- `<init>(Ljava/net/Proxy$Type;Ljava/net/SocketAddress;)V`
- `address()Ljava/net/SocketAddress;`
- `type()Ljava/net/Proxy$Type;`

### `java/net/SocketAddress`

- `<init>()V`

### `java/net/SocketException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/net/SocketPermission`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `getHost(Ljava/lang/String;)Ljava/lang/String;`
- `getMask(Ljava/lang/String;)I`
- `getName()Ljava/lang/String;`
- `init(Ljava/lang/String;I)V`
- `parsePort(Ljava/lang/String;)[I`

### `java/net/URI`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `appendAuthority(Ljava/lang/StringBuilder;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V`
- `appendEncoded(Ljava/nio/charset/CharsetEncoder;Ljava/lang/StringBuilder;C)V`
- `appendEscape(Ljava/lang/StringBuilder;B)V`
- `appendFragment(Ljava/lang/StringBuilder;Ljava/lang/String;)V`
- `appendSchemeSpecificPart(Ljava/lang/StringBuilder;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V`
- `checkPath(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `decode(C)I`
- `decode(CC)B`
- `decode(Ljava/lang/String;)Ljava/lang/String;`
- `decode(Ljava/lang/String;Z)Ljava/lang/String;`
- `defineString()Ljava/lang/String;`
- `getAuthority()Ljava/lang/String;`
- `getHost()Ljava/lang/String;`
- `getPath()Ljava/lang/String;`
- `getQuery()Ljava/lang/String;`
- `getRawAuthority()Ljava/lang/String;`
- `getRawFragment()Ljava/lang/String;`
- `getRawQuery()Ljava/lang/String;`
- `getRawSchemeSpecificPart()Ljava/lang/String;`
- `getScheme()Ljava/lang/String;`
- `getSchemeSpecificPart()Ljava/lang/String;`
- `getUserInfo()Ljava/lang/String;`
- `isAbsolute()Z`
- `isOpaque()Z`
- `match(CJJ)Z`
- `quote(Ljava/lang/String;JJ)Ljava/lang/String;`
- `toString(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`
- `validSchemeAndPath(Ljava/lang/String;Ljava/lang/String;)Z`

### `java/net/URI$Parser`

- `<init>()V`
- `<init>(Ljava/net/URI;Ljava/lang/String;)V`
- `at(IIC)Z`
- `at(IILjava/lang/String;)Z`
- `checkChar(IJJLjava/lang/String;)V`
- `checkChars(IIJJLjava/lang/String;)V`
- `fail(Ljava/lang/String;)V`
- `fail(Ljava/lang/String;I)V`
- `failExpecting(Ljava/lang/String;I)V`
- `parse(Z)V`
- `parseAuthority(II)I`
- `parseHierarchical(II)I`
- `parseHostname(IIZ)I`
- `parseIPv4Address(II)I`
- `parseIPv6Reference(II)I`
- `parseServer(IIZ)I`
- `scan(IIC)I`
- `scan(IIJJ)I`
- `scan(IILjava/lang/String;)I`
- `scan(IILjava/lang/String;Ljava/lang/String;)I`
- `scanByte(II)I`
- `scanEscape(IIC)I`
- `scanHexPost(II)I`
- `scanHexSeq(II)I`
- `scanIPv4Address(IIZ)I`
- `takeIPv4Address(IILjava/lang/String;)I`

### `java/net/URISyntaxException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;I)V`

### `java/net/URL`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `<init>(Ljava/net/URL;Ljava/lang/String;)V`
- `<init>(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V`
- `checkSpecifyHandler(Ljava/lang/SecurityManager;)V`
- `endLookup(Ljava/lang/Object;)V`
- `getAuthority()Ljava/lang/String;`
- `getDefaultPort()I`
- `getFile()Ljava/lang/String;`
- `getHost()Ljava/lang/String;`
- `getPath()Ljava/lang/String;`
- `getPort()I`
- `getProtocol()Ljava/lang/String;`
- `getQuery()Ljava/lang/String;`
- `getRef()Ljava/lang/String;`
- `getURLStreamHandler(Ljava/lang/String;)Ljava/net/URLStreamHandler;`
- `getUserInfo()Ljava/lang/String;`
- `isBuiltinStreamHandler(Ljava/lang/String;)Z`
- `isBuiltinStreamHandler(Ljava/net/URLStreamHandler;)Z`
- `isOverrideable(Ljava/lang/String;)Z`
- `isValidProtocol(Ljava/lang/String;)Z`
- `lookupViaProperty(Ljava/lang/String;)Ljava/net/URLStreamHandler;`
- `lookupViaProviders(Ljava/lang/String;)Ljava/net/URLStreamHandler;`
- `lowerCaseProtocol(Ljava/lang/String;)Ljava/lang/String;`
- `openConnection()Ljava/net/URLConnection;`
- `openStream()Ljava/io/InputStream;`
- `set(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `toExternalForm()Ljava/lang/String;`
- `toString()Ljava/lang/String;`
- `tryBeginLookup()Ljava/lang/Object;`

### `java/net/URL$2`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/net/URLConnection`

- `getInputStream()Ljava/io/InputStream;`

### `java/net/URLStreamHandler`

- `<init>()V`
- `getDefaultPort()I`
- `openConnection(Ljava/net/URL;)Ljava/net/URLConnection;`
- `openConnection(Ljava/net/URL;Ljava/net/Proxy;)Ljava/net/URLConnection;`
- `parseURL(Ljava/net/URL;Ljava/lang/String;II)V`
- `setURL(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `toExternalForm(Ljava/net/URL;)Ljava/lang/String;`

### `java/net/URLStreamHandlerFactory`

- `createURLStreamHandler(Ljava/lang/String;)Ljava/net/URLStreamHandler;`

### `java/net/UnknownHostException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/net/UnknownServiceException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/net/spi/InetAddressResolver`

- `<init>()V`
- `lookupByAddress([B)Ljava/lang/String;`

### `java/nio/Buffer`

- `<init>(IIIILjava/lang/foreign/MemorySegment;)V`
- `<init>(JILjava/lang/foreign/MemorySegment;)V`
- `clear()Ljava/nio/Buffer;`
- `createCapacityException(I)Ljava/lang/IllegalArgumentException;`
- `createLimitException(I)Ljava/lang/IllegalArgumentException;`
- `createPositionException(I)Ljava/lang/IllegalArgumentException;`
- `flip()Ljava/nio/Buffer;`
- `limit(I)Ljava/nio/Buffer;`
- `position(I)Ljava/nio/Buffer;`

### `java/nio/BufferOverflowException`

- `<init>()V`

### `java/nio/BufferUnderflowException`

- `<init>()V`

### `java/nio/ByteBuffer`

- `<init>(IIII[BILjava/lang/foreign/MemorySegment;)V`
- `allocate(I)Ljava/nio/ByteBuffer;`
- `array()[B`
- `arrayOffset()I`
- `base()Ljava/lang/Object;`
- `capacity()I`
- `clear()Ljava/nio/ByteBuffer;`
- `createCapacityException(I)Ljava/lang/IllegalArgumentException;`
- `createSameBufferException()Ljava/lang/IllegalArgumentException;`
- `flip()Ljava/nio/ByteBuffer;`
- `get()B`
- `get(I)B`
- `get(I[BII)Ljava/nio/ByteBuffer;`
- `get([B)Ljava/nio/ByteBuffer;`
- `get([BII)Ljava/nio/ByteBuffer;`
- `getArray(I[BII)Ljava/nio/ByteBuffer;`
- `hasArray()Z`
- `hasRemaining()Z`
- `isDirect()Z`
- `isReadOnly()Z`
- `limit()I`
- `limit(I)Ljava/nio/ByteBuffer;`
- `position()I`
- `position(I)Ljava/nio/ByteBuffer;`
- `put(B)Ljava/nio/ByteBuffer;`
- `put(IB)Ljava/nio/ByteBuffer;`
- `put(I[BII)Ljava/nio/ByteBuffer;`
- `put(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;`
- `put([B)Ljava/nio/ByteBuffer;`
- `put([BII)Ljava/nio/ByteBuffer;`
- `putArray(I[BII)Ljava/nio/ByteBuffer;`
- `putBuffer(ILjava/nio/ByteBuffer;II)V`
- `remaining()I`
- `session()Ljdk/internal/foreign/MemorySessionImpl;`
- `wrap([B)Ljava/nio/ByteBuffer;`
- `wrap([BII)Ljava/nio/ByteBuffer;`

### `java/nio/ByteOrder`

- `nativeOrder()Ljava/nio/ByteOrder;`

### `java/nio/CharBuffer`

- `<init>(IIIILjava/lang/foreign/MemorySegment;)V`
- `<init>(IIII[CILjava/lang/foreign/MemorySegment;)V`
- `allocate(I)Ljava/nio/CharBuffer;`
- `array()[C`
- `arrayOffset()I`
- `base()Ljava/lang/Object;`
- `clear()Ljava/nio/CharBuffer;`
- `createCapacityException(I)Ljava/lang/IllegalArgumentException;`
- `createSameBufferException()Ljava/lang/IllegalArgumentException;`
- `flip()Ljava/nio/CharBuffer;`
- `get()C`
- `get(I)C`
- `get(I[CII)Ljava/nio/CharBuffer;`
- `get([CII)Ljava/nio/CharBuffer;`
- `getArray(I[CII)Ljava/nio/CharBuffer;`
- `hasArray()Z`
- `hasRemaining()Z`
- `isAddressable()Z`
- `isDirect()Z`
- `isReadOnly()Z`
- `limit()I`
- `limit(I)Ljava/nio/CharBuffer;`
- `order()Ljava/nio/ByteOrder;`
- `position()I`
- `position(I)Ljava/nio/CharBuffer;`
- `put(C)Ljava/nio/CharBuffer;`
- `put(IC)Ljava/nio/CharBuffer;`
- `put(I[CII)Ljava/nio/CharBuffer;`
- `put(Ljava/lang/String;)Ljava/nio/CharBuffer;`
- `put(Ljava/lang/String;II)Ljava/nio/CharBuffer;`
- `put(Ljava/nio/CharBuffer;)Ljava/nio/CharBuffer;`
- `put([CII)Ljava/nio/CharBuffer;`
- `putArray(I[CII)Ljava/nio/CharBuffer;`
- `putBuffer(ILjava/nio/CharBuffer;II)V`
- `remaining()I`
- `session()Ljdk/internal/foreign/MemorySessionImpl;`
- `toString()Ljava/lang/String;`
- `toString(II)Ljava/lang/String;`
- `wrap(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;`
- `wrap(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;`
- `wrap([C)Ljava/nio/CharBuffer;`
- `wrap([CII)Ljava/nio/CharBuffer;`

### `java/nio/HeapByteBuffer`

- `<init>()V`
- `<init>(IILjava/lang/foreign/MemorySegment;)V`
- `<init>([BIILjava/lang/foreign/MemorySegment;)V`

### `java/nio/HeapCharBuffer`

- `<init>()V`
- `<init>(IILjava/lang/foreign/MemorySegment;)V`
- `<init>([CIILjava/lang/foreign/MemorySegment;)V`

### `java/nio/ReadOnlyBufferException`

- `<init>()V`

### `java/nio/StringCharBuffer`

- `<init>()V`
- `<init>(Ljava/lang/CharSequence;II)V`

### `java/nio/channels/ReadableByteChannel`

- `close()V`

### `java/nio/channels/WritableByteChannel`

- `write(Ljava/nio/ByteBuffer;)I`

### `java/nio/charset/CharacterCodingException`

- `<init>()V`

### `java/nio/charset/Charset`

- `<init>()V`
- `<init>(Ljava/lang/String;[Ljava/lang/String;)V`
- `cache(Ljava/lang/String;Ljava/nio/charset/Charset;)V`
- `checkName(Ljava/lang/String;)V`
- `defaultCharset()Ljava/nio/charset/Charset;`
- `endLookup(Ljava/lang/Object;)V`
- `forName(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `lookup(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `lookup2(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `lookupExtendedCharset(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `lookupViaProviders(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `newDecoder()Ljava/nio/charset/CharsetDecoder;`
- `newEncoder()Ljava/nio/charset/CharsetEncoder;`
- `tryBeginLookup()Ljava/lang/Object;`

### `java/nio/charset/Charset$2`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/nio/charset/CharsetDecoder`

- `<init>()V`
- `<init>(Ljava/nio/charset/Charset;FF)V`
- `<init>(Ljava/nio/charset/Charset;FFLjava/lang/String;)V`
- `averageCharsPerByte()F`
- `charset()Ljava/nio/charset/Charset;`
- `decode(Ljava/nio/ByteBuffer;Ljava/nio/CharBuffer;Z)Ljava/nio/charset/CoderResult;`
- `decodeLoop(Ljava/nio/ByteBuffer;Ljava/nio/CharBuffer;)Ljava/nio/charset/CoderResult;`
- `flush(Ljava/nio/CharBuffer;)Ljava/nio/charset/CoderResult;`
- `implFlush(Ljava/nio/CharBuffer;)Ljava/nio/charset/CoderResult;`
- `implOnMalformedInput(Ljava/nio/charset/CodingErrorAction;)V`
- `implOnUnmappableCharacter(Ljava/nio/charset/CodingErrorAction;)V`
- `implReplaceWith(Ljava/lang/String;)V`
- `implReset()V`
- `maxCharsPerByte()F`
- `onMalformedInput(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;`
- `onUnmappableCharacter(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;`
- `replaceWith(Ljava/lang/String;)Ljava/nio/charset/CharsetDecoder;`
- `reset()Ljava/nio/charset/CharsetDecoder;`
- `throwIllegalStateException(II)V`

### `java/nio/charset/CharsetEncoder`

- `<init>(Ljava/nio/charset/Charset;FF)V`
- `<init>(Ljava/nio/charset/Charset;FF[B)V`
- `averageBytesPerChar()F`
- `charset()Ljava/nio/charset/Charset;`
- `encode(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;`
- `encode(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;`
- `encodeLoop(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;`
- `flush(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;`
- `implFlush(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;`
- `implOnMalformedInput(Ljava/nio/charset/CodingErrorAction;)V`
- `implOnUnmappableCharacter(Ljava/nio/charset/CodingErrorAction;)V`
- `implReplaceWith([B)V`
- `implReset()V`
- `isLegalReplacement([B)Z`
- `maxBytesPerChar()F`
- `onMalformedInput(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;`
- `onUnmappableCharacter(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;`
- `replaceWith([B)Ljava/nio/charset/CharsetEncoder;`
- `reset()Ljava/nio/charset/CharsetEncoder;`
- `throwIllegalStateException(II)V`

### `java/nio/charset/CoderMalfunctionError`

- `<init>()V`
- `<init>(Ljava/lang/Exception;)V`

### `java/nio/charset/CoderResult`

- `<init>()V`
- `isError()Z`
- `isMalformed()Z`
- `isOverflow()Z`
- `isUnderflow()Z`
- `isUnmappable()Z`
- `length()I`
- `malformedForLength(I)Ljava/nio/charset/CoderResult;`
- `throwException()V`
- `toString()Ljava/lang/String;`

### `java/nio/charset/IllegalCharsetNameException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/nio/charset/MalformedInputException`

- `<init>()V`
- `<init>(I)V`

### `java/nio/charset/UnmappableCharacterException`

- `<init>()V`
- `<init>(I)V`

### `java/nio/charset/UnsupportedCharsetException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/nio/charset/spi/CharsetProvider`

- `charsetForName(Ljava/lang/String;)Ljava/nio/charset/Charset;`

### `java/nio/file/FileSystem`

- `getPath(Ljava/lang/String;[Ljava/lang/String;)Ljava/nio/file/Path;`

### `java/nio/file/Path`

- `equals(Ljava/lang/Object;)Z`
- `getFileName()Ljava/nio/file/Path;`
- `getParent()Ljava/nio/file/Path;`
- `normalize()Ljava/nio/file/Path;`

### `java/security/AccessControlContext`

- `<init>()V`
- `<init>(Ljava/security/AccessControlContext;Ljava/security/DomainCombiner;Z)V`
- `<init>(Ljava/security/ProtectionDomain;Ljava/security/DomainCombiner;Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)V`
- `<init>([Ljava/security/ProtectionDomain;)V`
- `<init>([Ljava/security/ProtectionDomain;Z)V`
- `calculateFields(Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)V`
- `checkPermission(Ljava/security/Permission;)V`
- `checkPermission2(Ljava/security/Permission;)V`
- `combine([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)[Ljava/security/ProtectionDomain;`
- `containsAllPDs([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)Z`
- `getCombiner()Ljava/security/DomainCombiner;`
- `getDebug()Lsun/security/util/Debug;`
- `isAuthorized()Z`
- `isPrivileged()Z`
- `optimize()Ljava/security/AccessControlContext;`

### `java/security/AccessControlContext$1`

- `<init>()V`
- `<init>(Ljava/security/AccessControlContext;Lsun/security/util/Debug;Ljava/security/ProtectionDomain;)V`

### `java/security/AccessControlException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/security/Permission;)V`

### `java/security/AccessController`

- `checkContext(Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/security/AccessControlContext;`
- `checkPermission(Ljava/security/Permission;)V`
- `createWrapper(Ljava/security/DomainCombiner;Ljava/lang/Class;Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/security/AccessControlContext;`
- `doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;`
- `doPrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;`
- `doPrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;`
- `doPrivileged(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;`
- `doPrivileged(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;`
- `ensureMaterializedForStackWalk(Ljava/lang/Object;)V`
- `executePrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;`
- `executePrivileged(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;`
- `getContext()Ljava/security/AccessControlContext;`
- `getInheritedAccessControlContext()Ljava/security/AccessControlContext;`
- `getInnocuousAcc()Ljava/security/AccessControlContext;`
- `getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;`
- `getStackAccessControlContext()Ljava/security/AccessControlContext;`
- `isPrivileged()Z`
- `wrapException(Ljava/lang/Exception;)Ljava/security/PrivilegedActionException;`

### `java/security/AlgorithmParameters`

- `<init>()V`
- `<init>(Ljava/security/AlgorithmParametersSpi;Ljava/security/Provider;Ljava/lang/String;)V`
- `getEncoded()[B`
- `getInstance(Ljava/lang/String;)Ljava/security/AlgorithmParameters;`
- `init([B)V`
- `toString()Ljava/lang/String;`

### `java/security/AlgorithmParametersSpi`

- `<init>()V`
- `engineGetEncoded()[B`
- `engineGetEncoded(Ljava/lang/String;)[B`
- `engineInit(Ljava/security/spec/AlgorithmParameterSpec;)V`
- `engineInit([B)V`
- `engineInit([BLjava/lang/String;)V`
- `engineToString()Ljava/lang/String;`

### `java/security/AllPermission`

- `<init>()V`

### `java/security/BasicPermission`

- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `init(Ljava/lang/String;)V`

### `java/security/CodeSigner`

- `getSignerCertPath()Ljava/security/cert/CertPath;`

### `java/security/CodeSource`

- `<init>()V`
- `<init>(Ljava/net/URL;[Ljava/security/cert/Certificate;)V`
- `getCertificates()[Ljava/security/cert/Certificate;`
- `getLocation()Ljava/net/URL;`

### `java/security/DomainCombiner`

- `combine([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)[Ljava/security/ProtectionDomain;`

### `java/security/GeneralSecurityException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/security/InvalidKeyException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `getMessage()Ljava/lang/String;`

### `java/security/InvalidParameterException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/security/KeyException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/security/KeyFactory`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/security/KeyFactorySpi;Ljava/security/Provider;Ljava/lang/String;)V`
- `generatePublic(Ljava/security/spec/KeySpec;)Ljava/security/PublicKey;`
- `getInstance(Ljava/lang/String;)Ljava/security/KeyFactory;`
- `nextSpi(Ljava/security/KeyFactorySpi;)Ljava/security/KeyFactorySpi;`

### `java/security/KeyFactorySpi`

- `<init>()V`
- `engineGeneratePublic(Ljava/security/spec/KeySpec;)Ljava/security/PublicKey;`

### `java/security/KeyStore`

- `<init>()V`
- `<init>(Ljava/security/KeyStoreSpi;Ljava/security/Provider;Ljava/lang/String;)V`
- `getCertificate(Ljava/lang/String;)Ljava/security/cert/Certificate;`
- `getDefaultType()Ljava/lang/String;`
- `getInstance(Ljava/io/File;[CLjava/security/KeyStore$LoadStoreParameter;Z)Ljava/security/KeyStore;`
- `getInstance(Ljava/lang/String;)Ljava/security/KeyStore;`
- `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyStore;`
- `getProviderName()Ljava/lang/String;`
- `load(Ljava/io/InputStream;[C)V`

### `java/security/KeyStore$CallbackHandlerProtection`

- `<init>()V`
- `getCallbackHandler()Ljavax/security/auth/callback/CallbackHandler;`

### `java/security/KeyStore$LoadStoreParameter`

- `getProtectionParameter()Ljava/security/KeyStore$ProtectionParameter;`

### `java/security/KeyStore$PasswordProtection`

- `<init>()V`
- `getPassword()[C`

### `java/security/KeyStoreException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/security/KeyStoreSpi`

- `<init>()V`
- `engineGetCertificate(Ljava/lang/String;)Ljava/security/cert/Certificate;`
- `engineLoad(Ljava/io/InputStream;Ljava/security/KeyStore$LoadStoreParameter;)V`
- `engineLoad(Ljava/io/InputStream;[C)V`
- `engineLoad(Ljava/security/KeyStore$LoadStoreParameter;)V`
- `engineProbe(Ljava/io/InputStream;)Z`

### `java/security/NoSuchAlgorithmException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/security/NoSuchProviderException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/security/Permission`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `implies(Ljava/security/Permission;)Z`
- `newPermissionCollection()Ljava/security/PermissionCollection;`

### `java/security/PermissionCollection`

- `<init>()V`
- `add(Ljava/security/Permission;)V`
- `elements()Ljava/util/Enumeration;`
- `implies(Ljava/security/Permission;)Z`
- `setReadOnly()V`

### `java/security/Permissions`

- `<init>()V`
- `add(Ljava/security/Permission;)V`
- `createPermissionCollection(Ljava/security/Permission;Z)Ljava/security/PermissionCollection;`
- `getPermissionCollection(Ljava/security/Permission;Z)Ljava/security/PermissionCollection;`
- `getUnresolvedPermissions(Ljava/security/Permission;)Ljava/security/PermissionCollection;`
- `isReadOnly()Z`

### `java/security/PermissionsHash`

- `<init>()V`

### `java/security/Policy`

- `<init>()V`
- `addStaticPerms(Ljava/security/PermissionCollection;Ljava/security/PermissionCollection;)V`
- `getPermissions(Ljava/security/CodeSource;)Ljava/security/PermissionCollection;`
- `getPermissions(Ljava/security/ProtectionDomain;)Ljava/security/PermissionCollection;`
- `getPolicyNoCheck()Ljava/security/Policy;`
- `implies(Ljava/security/ProtectionDomain;Ljava/security/Permission;)Z`
- `initPolicy(Ljava/security/Policy;)V`
- `isSet()Z`
- `loadPolicyProvider()Ljava/security/Policy;`

### `java/security/Policy$1`

- `<init>()V`

### `java/security/Policy$2`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/security/Policy$3`

- `<init>()V`
- `<init>(Ljava/security/Policy;)V`

### `java/security/Policy$PolicyInfo`

- `<init>()V`
- `<init>(Ljava/security/Policy;Z)V`

### `java/security/Principal`

- `<init>()V`

### `java/security/PrivilegedAction`

- `run()Ljava/lang/Object;`

### `java/security/PrivilegedActionException`

- `<init>()V`
- `<init>(Ljava/lang/Exception;)V`
- `getCause()Ljava/lang/Throwable;`
- `getException()Ljava/lang/Exception;`

### `java/security/PrivilegedExceptionAction`

- `run()Ljava/lang/Object;`

### `java/security/ProtectionDomain`

- `<init>()V`
- `<init>(Ljava/security/CodeSource;Ljava/security/PermissionCollection;Ljava/lang/ClassLoader;[Ljava/security/Principal;)V`
- `getCodeSource()Ljava/security/CodeSource;`
- `getPermissions()Ljava/security/PermissionCollection;`
- `implies(Ljava/security/Permission;)Z`
- `impliesWithAltFilePerm(Ljava/security/Permission;)Z`

### `java/security/ProtectionDomain$Key`

- `<init>()V`

### `java/security/Provider`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `checkInitialized()V`
- `getEngineName(Ljava/lang/String;)Ljava/lang/String;`
- `getName()Ljava/lang/String;`
- `getProperty(Ljava/lang/String;)Ljava/lang/String;`
- `getService(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service;`
- `getServices()Ljava/util/Set;`
- `keySet()Ljava/util/Set;`
- `parseVersionStr(Ljava/lang/String;)D`
- `putId()V`

### `java/security/Provider$EngineDescription`

- `<init>()V`

### `java/security/Provider$Service`

- `<init>()V`
- `<init>(Ljava/security/Provider;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/List;Ljava/util/Map;)V`
- `getAlgorithm()Ljava/lang/String;`
- `getClassName()Ljava/lang/String;`
- `getDefaultConstructor()Ljava/lang/reflect/Constructor;`
- `getImplClass()Ljava/lang/Class;`
- `getProvider()Ljava/security/Provider;`
- `getType()Ljava/lang/String;`
- `isValid()Z`
- `newInstance(Ljava/lang/Object;)Ljava/lang/Object;`
- `newInstanceOf()Ljava/lang/Object;`
- `newInstanceUtil(Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/security/Provider$ServiceKey`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Z)V`
- `matches(Ljava/lang/String;Ljava/lang/String;)Z`

### `java/security/Provider$UString`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/security/ProviderException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/security/Security`

- `getImpl(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/Object;`
- `getImpl(Ljava/lang/String;Ljava/lang/String;Ljava/security/Provider;)[Ljava/lang/Object;`
- `getProperty(Ljava/lang/String;)Ljava/lang/String;`
- `getProvider(Ljava/lang/String;)Ljava/security/Provider;`
- `getProviders()[Ljava/security/Provider;`
- `getProviders(Ljava/util/Map;)[Ljava/security/Provider;`
- `getSpiClass(Ljava/lang/String;)Ljava/lang/Class;`

### `java/security/Security$Criteria`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `isCompositeValue()Z`

### `java/security/SecurityPermission`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`

### `java/security/UnresolvedPermission`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/security/cert/Certificate;)V`
- `resolve(Ljava/security/Permission;[Ljava/security/cert/Certificate;)Ljava/security/Permission;`

### `java/security/UnresolvedPermissionCollection`

- `<init>()V`
- `getUnresolvedPermissions(Ljava/security/Permission;)Ljava/util/List;`

### `java/security/cert/CertPath`

- `getCertificates()Ljava/util/List;`

### `java/security/cert/Certificate`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `equals(Ljava/lang/Object;)Z`
- `getEncoded()[B`

### `java/security/cert/CertificateException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/security/cert/CertificateParsingException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/security/cert/X509Certificate`

- `<init>()V`
- `getEncoded()[B`
- `getIssuerX500Principal()Ljavax/security/auth/x500/X500Principal;`
- `getSubjectX500Principal()Ljavax/security/auth/x500/X500Principal;`

### `java/security/spec/EncodedKeySpec`

- `<init>([B)V`
- `<init>([BLjava/lang/String;)V`

### `java/security/spec/InvalidKeySpecException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `getMessage()Ljava/lang/String;`

### `java/security/spec/InvalidParameterSpecException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/security/spec/X509EncodedKeySpec`

- `<init>()V`
- `<init>([B)V`

### `java/text/BreakIterator`

- `<init>()V`
- `clone()Ljava/lang/Object;`
- `createBreakInstance(Ljava/util/Locale;I)Ljava/text/BreakIterator;`
- `createBreakInstance(Lsun/util/locale/provider/LocaleProviderAdapter;Ljava/util/Locale;I)Ljava/text/BreakIterator;`
- `following(I)I`
- `getBreakInstance(Ljava/util/Locale;I)Ljava/text/BreakIterator;`
- `getWordInstance(Ljava/util/Locale;)Ljava/text/BreakIterator;`
- `isBoundary(I)Z`
- `setText(Ljava/lang/String;)V`
- `setText(Ljava/text/CharacterIterator;)V`

### `java/text/BreakIterator$BreakIteratorCache`

- `<init>()V`
- `<init>(Ljava/util/Locale;Ljava/text/BreakIterator;)V`
- `createBreakInstance()Ljava/text/BreakIterator;`
- `getLocale()Ljava/util/Locale;`

### `java/text/Normalizer`

- `normalize(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;)Ljava/lang/String;`

### `java/text/StringCharacterIterator`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;I)V`
- `<init>(Ljava/lang/String;III)V`

### `java/text/spi/BreakIteratorProvider`

- `getCharacterInstance(Ljava/util/Locale;)Ljava/text/BreakIterator;`
- `getLineInstance(Ljava/util/Locale;)Ljava/text/BreakIterator;`
- `getSentenceInstance(Ljava/util/Locale;)Ljava/text/BreakIterator;`
- `getWordInstance(Ljava/util/Locale;)Ljava/text/BreakIterator;`

### `java/time/Clock`

- `currentInstant()Ljava/time/Instant;`
- `instant()Ljava/time/Instant;`

### `java/time/DateTimeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/time/DayOfWeek`

- `getValue()I`
- `of(I)Ljava/time/DayOfWeek;`
- `ordinal()I`

### `java/time/Duration`

- `<init>()V`
- `<init>(JI)V`
- `create(JI)Ljava/time/Duration;`
- `create(Ljava/math/BigDecimal;)Ljava/time/Duration;`
- `getSeconds()J`
- `multipliedBy(J)Ljava/time/Duration;`
- `negated()Ljava/time/Duration;`
- `ofSeconds(J)Ljava/time/Duration;`
- `ofSeconds(JJ)Ljava/time/Duration;`
- `toBigDecimalSeconds()Ljava/math/BigDecimal;`

### `java/time/Instant`

- `<init>()V`
- `<init>(JI)V`
- `create(JI)Ljava/time/Instant;`
- `from(Ljava/time/temporal/TemporalAccessor;)Ljava/time/Instant;`
- `getEpochSecond()J`
- `getNano()I`
- `now()Ljava/time/Instant;`
- `ofEpochSecond(JJ)Ljava/time/Instant;`

### `java/time/LocalDate`

- `<init>()V`
- `<init>(III)V`
- `compareTo0(Ljava/time/LocalDate;)I`
- `create(III)Ljava/time/LocalDate;`
- `equals(Ljava/lang/Object;)Z`
- `from(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDate;`
- `get0(Ljava/time/temporal/TemporalField;)I`
- `getDayOfMonth()I`
- `getDayOfWeek()Ljava/time/DayOfWeek;`
- `getDayOfYear()I`
- `getLong(Ljava/time/temporal/TemporalField;)J`
- `getMonth()Ljava/time/Month;`
- `getMonthValue()I`
- `getProlepticMonth()J`
- `getYear()I`
- `isLeapYear()Z`
- `lengthOfMonth()I`
- `of(III)Ljava/time/LocalDate;`
- `of(ILjava/time/Month;I)Ljava/time/LocalDate;`
- `ofEpochDay(J)Ljava/time/LocalDate;`
- `ofYearDay(II)Ljava/time/LocalDate;`
- `plusDays(J)Ljava/time/LocalDate;`
- `plusMonths(J)Ljava/time/LocalDate;`
- `plusWeeks(J)Ljava/time/LocalDate;`
- `resolvePreviousValid(III)Ljava/time/LocalDate;`
- `toEpochDay()J`
- `with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalDate;`
- `with(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalDate;`
- `withDayOfMonth(I)Ljava/time/LocalDate;`
- `withDayOfYear(I)Ljava/time/LocalDate;`
- `withMonth(I)Ljava/time/LocalDate;`
- `withYear(I)Ljava/time/LocalDate;`

### `java/time/LocalDateTime`

- `<init>()V`
- `<init>(Ljava/time/LocalDate;Ljava/time/LocalTime;)V`
- `compareTo0(Ljava/time/LocalDateTime;)I`
- `equals(Ljava/lang/Object;)Z`
- `from(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDateTime;`
- `getDayOfMonth()I`
- `getMonthValue()I`
- `getNano()I`
- `getYear()I`
- `isAfter(Ljava/time/chrono/ChronoLocalDateTime;)Z`
- `isBefore(Ljava/time/chrono/ChronoLocalDateTime;)Z`
- `of(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;`
- `ofEpochSecond(JILjava/time/ZoneOffset;)Ljava/time/LocalDateTime;`
- `plusSeconds(J)Ljava/time/LocalDateTime;`
- `plusWithOverflow(Ljava/time/LocalDate;JJJJI)Ljava/time/LocalDateTime;`
- `toEpochSecond(Ljava/time/ZoneOffset;)J`
- `toLocalDate()Ljava/time/LocalDate;`
- `toLocalTime()Ljava/time/LocalTime;`
- `with(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;`
- `with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalDateTime;`
- `with(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalDateTime;`

### `java/time/LocalTime`

- `<init>()V`
- `<init>(IIII)V`
- `compareTo(Ljava/time/LocalTime;)I`
- `create(IIII)Ljava/time/LocalTime;`
- `equals(Ljava/lang/Object;)Z`
- `from(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalTime;`
- `getNano()I`
- `of(II)Ljava/time/LocalTime;`
- `of(III)Ljava/time/LocalTime;`
- `of(IIII)Ljava/time/LocalTime;`
- `ofNanoOfDay(J)Ljava/time/LocalTime;`
- `plusHours(J)Ljava/time/LocalTime;`
- `plusMinutes(J)Ljava/time/LocalTime;`
- `plusSeconds(J)Ljava/time/LocalTime;`
- `toNanoOfDay()J`
- `toSecondOfDay()I`
- `with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalTime;`
- `with(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalTime;`
- `withHour(I)Ljava/time/LocalTime;`
- `withMinute(I)Ljava/time/LocalTime;`
- `withNano(I)Ljava/time/LocalTime;`
- `withSecond(I)Ljava/time/LocalTime;`

### `java/time/Month`

- `firstDayOfYear(Z)I`
- `getValue()I`
- `length(Z)I`
- `name()Ljava/lang/String;`
- `of(I)Ljava/time/Month;`
- `ordinal()I`
- `plus(J)Ljava/time/Month;`

### `java/time/OffsetDateTime`

- `<init>()V`
- `toLocalDateTime()Ljava/time/LocalDateTime;`

### `java/time/ZoneId`

- `<init>()V`
- `from(Ljava/time/temporal/TemporalAccessor;)Ljava/time/ZoneId;`
- `getId()Ljava/lang/String;`
- `getRules()Ljava/time/zone/ZoneRules;`
- `normalized()Ljava/time/ZoneId;`

### `java/time/ZoneOffset`

- `<init>()V`
- `<init>(I)V`
- `buildId(I)Ljava/lang/String;`
- `equals(Ljava/lang/Object;)Z`
- `getId()Ljava/lang/String;`
- `getTotalSeconds()I`
- `ofTotalSeconds(I)Ljava/time/ZoneOffset;`

### `java/time/ZonedDateTime`

- `<init>()V`
- `toLocalDateTime()Ljava/time/LocalDateTime;`

### `java/time/chrono/ChronoLocalDate`

- `<init>()V`
- `atTime(Ljava/time/LocalTime;)Ljava/time/chrono/ChronoLocalDateTime;`
- `getChronology()Ljava/time/chrono/Chronology;`
- `getLong(Ljava/time/temporal/TemporalField;)J`
- `plus(JLjava/time/temporal/TemporalUnit;)Ljava/time/chrono/ChronoLocalDate;`
- `plus(Ljava/time/temporal/TemporalAmount;)Ljava/time/chrono/ChronoLocalDate;`
- `toEpochDay()J`
- `with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/chrono/ChronoLocalDate;`
- `with(Ljava/time/temporal/TemporalField;J)Ljava/time/chrono/ChronoLocalDate;`

### `java/time/chrono/ChronoLocalDateImpl`

- `ensureValid(Ljava/time/chrono/Chronology;Ljava/time/temporal/Temporal;)Ljava/time/chrono/ChronoLocalDate;`

### `java/time/chrono/ChronoLocalDateTime`

- `isAfter(Ljava/time/chrono/ChronoLocalDateTime;)Z`
- `isBefore(Ljava/time/chrono/ChronoLocalDateTime;)Z`
- `toLocalDate()Ljava/time/chrono/ChronoLocalDate;`
- `toLocalTime()Ljava/time/LocalTime;`

### `java/time/chrono/ChronoLocalDateTimeImpl`

- `<init>()V`
- `<init>(Ljava/time/chrono/ChronoLocalDate;Ljava/time/LocalTime;)V`
- `ensureValid(Ljava/time/chrono/Chronology;Ljava/time/temporal/Temporal;)Ljava/time/chrono/ChronoLocalDateTimeImpl;`
- `getChronology()Ljava/time/chrono/Chronology;`
- `of(Ljava/time/chrono/ChronoLocalDate;Ljava/time/LocalTime;)Ljava/time/chrono/ChronoLocalDateTimeImpl;`
- `plusSeconds(J)Ljava/time/chrono/ChronoLocalDateTimeImpl;`
- `plusWithOverflow(Ljava/time/chrono/ChronoLocalDate;JJJJ)Ljava/time/chrono/ChronoLocalDateTimeImpl;`
- `with(Ljava/time/temporal/Temporal;Ljava/time/LocalTime;)Ljava/time/chrono/ChronoLocalDateTimeImpl;`
- `with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/chrono/ChronoLocalDateTimeImpl;`
- `with(Ljava/time/temporal/TemporalField;J)Ljava/time/chrono/ChronoLocalDateTimeImpl;`

### `java/time/chrono/ChronoZonedDateTimeImpl`

- `<init>()V`
- `<init>(Ljava/time/chrono/ChronoLocalDateTimeImpl;Ljava/time/ZoneOffset;Ljava/time/ZoneId;)V`
- `ofBest(Ljava/time/chrono/ChronoLocalDateTimeImpl;Ljava/time/ZoneId;Ljava/time/ZoneOffset;)Ljava/time/chrono/ChronoZonedDateTime;`
- `ofInstant(Ljava/time/chrono/Chronology;Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/chrono/ChronoZonedDateTimeImpl;`

### `java/time/chrono/Chronology`

- `<init>()V`
- `date(III)Ljava/time/chrono/ChronoLocalDate;`
- `date(Ljava/time/temporal/TemporalAccessor;)Ljava/time/chrono/ChronoLocalDate;`
- `equals(Ljava/lang/Object;)Z`
- `getId()Ljava/lang/String;`
- `localDateTime(Ljava/time/temporal/TemporalAccessor;)Ljava/time/chrono/ChronoLocalDateTime;`
- `prolepticYear(Ljava/time/chrono/Era;I)I`
- `zonedDateTime(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/chrono/ChronoZonedDateTime;`

### `java/time/chrono/IsoChronology`

- `isLeapYear(J)Z`

### `java/time/format/DateTimeFormatter`

- `format(Ljava/time/temporal/TemporalAccessor;)Ljava/lang/String;`
- `formatTo(Ljava/time/temporal/TemporalAccessor;Ljava/lang/Appendable;)V`
- `getChronology()Ljava/time/chrono/Chronology;`
- `getZone()Ljava/time/ZoneId;`

### `java/time/format/DateTimeFormatterBuilder$CompositePrinterParser`

- `format(Ljava/time/format/DateTimePrintContext;Ljava/lang/StringBuilder;)Z`

### `java/time/format/DateTimeFormatterBuilder$DateTimePrinterParser`

- `format(Ljava/time/format/DateTimePrintContext;Ljava/lang/StringBuilder;)Z`

### `java/time/format/DateTimePrintContext`

- `<init>()V`
- `<init>(Ljava/time/temporal/TemporalAccessor;Ljava/time/format/DateTimeFormatter;)V`
- `adjust(Ljava/time/temporal/TemporalAccessor;Ljava/time/format/DateTimeFormatter;)Ljava/time/temporal/TemporalAccessor;`
- `endOptional()V`
- `startOptional()V`

### `java/time/format/DateTimePrintContext$1`

- `<init>()V`
- `<init>(Ljava/time/chrono/ChronoLocalDate;Ljava/time/temporal/TemporalAccessor;Ljava/time/chrono/Chronology;Ljava/time/ZoneId;)V`

### `java/time/temporal/ChronoField`

- `<init>()V`
- `checkValidIntValue(J)I`
- `checkValidValue(J)J`
- `isDateBased()Z`
- `isTimeBased()Z`
- `ordinal()I`
- `range()Ljava/time/temporal/ValueRange;`
- `values()[Ljava/time/temporal/ChronoField;`

### `java/time/temporal/ChronoUnit`

- `<init>()V`

### `java/time/temporal/Temporal`

- `plus(Ljava/time/temporal/TemporalAmount;)Ljava/time/temporal/Temporal;`
- `with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/temporal/Temporal;`

### `java/time/temporal/TemporalAccessor`

- `get(Ljava/time/temporal/TemporalField;)I`
- `getClass()Ljava/lang/Class;`
- `getLong(Ljava/time/temporal/TemporalField;)J`
- `isSupported(Ljava/time/temporal/TemporalField;)Z`
- `query(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;`
- `range(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;`

### `java/time/temporal/TemporalAdjuster`

- `adjustInto(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;`

### `java/time/temporal/TemporalAdjusters`

- `nextOrSame(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;`
- `previousOrSame(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;`

### `java/time/temporal/TemporalAmount`

- `addTo(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;`

### `java/time/temporal/TemporalField`

- `adjustInto(Ljava/time/temporal/Temporal;J)Ljava/time/temporal/Temporal;`
- `getFrom(Ljava/time/temporal/TemporalAccessor;)J`
- `range()Ljava/time/temporal/ValueRange;`
- `rangeRefinedBy(Ljava/time/temporal/TemporalAccessor;)Ljava/time/temporal/ValueRange;`

### `java/time/temporal/TemporalQueries`

- `chronology()Ljava/time/temporal/TemporalQuery;`
- `localDate()Ljava/time/temporal/TemporalQuery;`
- `localTime()Ljava/time/temporal/TemporalQuery;`
- `precision()Ljava/time/temporal/TemporalQuery;`
- `zone()Ljava/time/temporal/TemporalQuery;`
- `zoneId()Ljava/time/temporal/TemporalQuery;`

### `java/time/temporal/TemporalQuery`

- `queryFrom(Ljava/time/temporal/TemporalAccessor;)Ljava/lang/Object;`

### `java/time/temporal/TemporalUnit`

- `addTo(Ljava/time/temporal/Temporal;J)Ljava/time/temporal/Temporal;`

### `java/time/temporal/UnsupportedTemporalTypeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/time/temporal/ValueRange`

- `checkValidIntValue(JLjava/time/temporal/TemporalField;)I`
- `checkValidValue(JLjava/time/temporal/TemporalField;)J`
- `genInvalidFieldMessage(Ljava/time/temporal/TemporalField;J)Ljava/lang/String;`
- `getMaximum()J`
- `getMinimum()J`
- `isIntValue()Z`
- `isValidIntValue(J)Z`
- `isValidValue(J)Z`

### `java/time/zone/ZoneOffsetTransition`

- `<init>()V`
- `<init>(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)V`
- `getDateTimeAfter()Ljava/time/LocalDateTime;`
- `getDateTimeBefore()Ljava/time/LocalDateTime;`
- `getDuration()Ljava/time/Duration;`
- `getDurationSeconds()I`
- `getOffsetAfter()Ljava/time/ZoneOffset;`
- `getOffsetBefore()Ljava/time/ZoneOffset;`
- `getValidOffsets()Ljava/util/List;`
- `isGap()Z`
- `toEpochSecond()J`

### `java/time/zone/ZoneOffsetTransitionRule`

- `createTransition(I)Ljava/time/zone/ZoneOffsetTransition;`

### `java/time/zone/ZoneOffsetTransitionRule$TimeDefinition`

- `createDateTime(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)Ljava/time/LocalDateTime;`
- `ordinal()I`

### `java/time/zone/ZoneRules`

- `findOffsetInfo(Ljava/time/LocalDateTime;Ljava/time/zone/ZoneOffsetTransition;)Ljava/lang/Object;`
- `findTransitionArray(I)[Ljava/time/zone/ZoneOffsetTransition;`
- `findYear(JLjava/time/ZoneOffset;)I`
- `getOffset(Ljava/time/Instant;)Ljava/time/ZoneOffset;`
- `getOffsetInfo(Ljava/time/LocalDateTime;)Ljava/lang/Object;`
- `getTransition(Ljava/time/LocalDateTime;)Ljava/time/zone/ZoneOffsetTransition;`
- `getValidOffsets(Ljava/time/LocalDateTime;)Ljava/util/List;`
- `isFixedOffset()Z`

### `java/util/AbstractCollection`

- `<init>()V`

### `java/util/AbstractList`

- `<init>()V`

### `java/util/AbstractMap`

- `<init>()V`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putAll(Ljava/util/Map;)V`

### `java/util/AbstractSequentialList`

- `<init>()V`

### `java/util/AbstractSet`

- `<init>()V`
- `addAll(Ljava/util/Collection;)Z`

### `java/util/ArrayDeque`

- `<init>()V`
- `<init>(I)V`
- `copyElements(Ljava/util/Collection;)V`

### `java/util/ArrayList`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/util/Collection;)V`
- `add(Ljava/lang/Object;)Z`
- `add(Ljava/lang/Object;[Ljava/lang/Object;I)V`
- `addAll(Ljava/util/Collection;)Z`
- `clear()V`
- `elementData(I)Ljava/lang/Object;`
- `get(I)Ljava/lang/Object;`
- `grow()[Ljava/lang/Object;`
- `grow(I)[Ljava/lang/Object;`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `outOfBoundsMsg(I)Ljava/lang/String;`
- `rangeCheckForAdd(I)V`
- `set(ILjava/lang/Object;)Ljava/lang/Object;`
- `size()I`
- `subList(II)Ljava/util/List;`
- `subListRangeCheck(III)V`
- `toArray([Ljava/lang/Object;)[Ljava/lang/Object;`

### `java/util/ArrayList$Itr`

- `<init>()V`
- `<init>(Ljava/util/ArrayList;)V`

### `java/util/ArrayList$SubList`

- `<init>()V`
- `<init>(Ljava/util/ArrayList;II)V`

### `java/util/Arrays`

- `asList([Ljava/lang/Object;)Ljava/util/List;`
- `binarySearch([CC)I`
- `binarySearch([JJ)I`
- `binarySearch([Ljava/lang/Object;Ljava/lang/Object;)I`
- `binarySearch0([BIIB)I`
- `binarySearch0([CIIC)I`
- `binarySearch0([DIID)I`
- `binarySearch0([FIIF)I`
- `binarySearch0([IIII)I`
- `binarySearch0([JIIJ)I`
- `binarySearch0([Ljava/lang/Object;IILjava/lang/Object;)I`
- `binarySearch0([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I`
- `binarySearch0([SIIS)I`
- `checkLength(II)V`
- `copyOf([BI)[B`
- `copyOf([CI)[C`
- `copyOf([II)[I`
- `copyOf([JI)[J`
- `copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;`
- `copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;`
- `copyOfRange([BII)[B`
- `copyOfRange([III)[I`
- `copyOfRange([Ljava/lang/Object;II)[Ljava/lang/Object;`
- `copyOfRange([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;`
- `copyOfRangeBoolean([ZII)[Z`
- `copyOfRangeByte([BII)[B`
- `copyOfRangeChar([CII)[C`
- `copyOfRangeDouble([DII)[D`
- `copyOfRangeFloat([FII)[F`
- `copyOfRangeInt([III)[I`
- `copyOfRangeLong([JII)[J`
- `copyOfRangeShort([SII)[S`
- `equals([B[B)Z`
- `equals([Ljava/lang/Object;[Ljava/lang/Object;)Z`
- `fill([BB)V`
- `fill([BIIB)V`
- `fill([CC)V`
- `fill([II)V`
- `fill([IIII)V`
- `fill([Ljava/lang/Object;IILjava/lang/Object;)V`
- `fill([Ljava/lang/Object;Ljava/lang/Object;)V`
- `hashCode([Ljava/lang/Object;)I`
- `legacyMergeSort([Ljava/lang/Object;)V`
- `legacyMergeSort([Ljava/lang/Object;II)V`
- `legacyMergeSort([Ljava/lang/Object;IILjava/util/Comparator;)V`
- `legacyMergeSort([Ljava/lang/Object;Ljava/util/Comparator;)V`
- `mergeSort([Ljava/lang/Object;[Ljava/lang/Object;III)V`
- `mergeSort([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V`
- `rangeCheck(III)V`
- `sort([Ljava/lang/Object;)V`
- `sort([Ljava/lang/Object;II)V`
- `sort([Ljava/lang/Object;Ljava/util/Comparator;)V`
- `spliterator([DII)Ljava/util/Spliterator$OfDouble;`
- `spliterator([III)Ljava/util/Spliterator$OfInt;`
- `spliterator([JII)Ljava/util/Spliterator$OfLong;`
- `spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;`
- `stream([DII)Ljava/util/stream/DoubleStream;`
- `stream([III)Ljava/util/stream/IntStream;`
- `stream([JII)Ljava/util/stream/LongStream;`
- `stream([Ljava/lang/Object;)Ljava/util/stream/Stream;`
- `stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;`
- `swap([Ljava/lang/Object;II)V`
- `toString([Ljava/lang/Object;)Ljava/lang/String;`

### `java/util/Arrays$ArrayList`

- `<init>()V`
- `<init>([Ljava/lang/Object;)V`

### `java/util/Base64`

- `getDecoder()Ljava/util/Base64$Decoder;`

### `java/util/Base64$Decoder`

- `decode([B)[B`
- `decode0([BII[B)I`
- `decodeBlock([BII[BIZZ)I`
- `decodedOutLength([BII)I`

### `java/util/Calendar`

- `<init>()V`
- `<init>(Ljava/util/TimeZone;Ljava/util/Locale;)V`
- `setWeekCountData(Ljava/util/Locale;)V`

### `java/util/Collection`

- `<init>()V`
- `forEach(Ljava/util/function/Consumer;)V`
- `getClass()Ljava/lang/Class;`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `size()I`
- `toArray()[Ljava/lang/Object;`
- `toArray([Ljava/lang/Object;)[Ljava/lang/Object;`

### `java/util/Collections`

- `emptyList()Ljava/util/List;`
- `emptyMap()Ljava/util/Map;`
- `emptySet()Ljava/util/Set;`
- `nCopies(ILjava/lang/Object;)Ljava/util/List;`
- `newSetFromMap(Ljava/util/Map;)Ljava/util/Set;`
- `reverse(Ljava/util/List;)V`
- `singletonList(Ljava/lang/Object;)Ljava/util/List;`
- `singletonMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;`
- `swap(Ljava/util/List;II)V`
- `synchronizedMap(Ljava/util/Map;)Ljava/util/Map;`
- `synchronizedSet(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;`
- `unmodifiableList(Ljava/util/List;)Ljava/util/List;`
- `unmodifiableSet(Ljava/util/Set;)Ljava/util/Set;`

### `java/util/Collections$CopiesList`

- `<init>()V`
- `<init>(ILjava/lang/Object;)V`

### `java/util/Collections$SetFromMap`

- `<init>()V`
- `<init>(Ljava/util/Map;)V`

### `java/util/Collections$SingletonList`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`

### `java/util/Collections$SingletonMap`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/Collections$SynchronizedCollection`

- `<init>(Ljava/util/Collection;)V`
- `<init>(Ljava/util/Collection;Ljava/lang/Object;)V`

### `java/util/Collections$SynchronizedMap`

- `<init>()V`
- `<init>(Ljava/util/Map;)V`

### `java/util/Collections$SynchronizedSet`

- `<init>()V`
- `<init>(Ljava/util/Set;)V`
- `<init>(Ljava/util/Set;Ljava/lang/Object;)V`

### `java/util/Collections$UnmodifiableCollection`

- `<init>(Ljava/util/Collection;)V`

### `java/util/Collections$UnmodifiableList`

- `<init>()V`
- `<init>(Ljava/util/List;)V`

### `java/util/Collections$UnmodifiableRandomAccessList`

- `<init>()V`
- `<init>(Ljava/util/List;)V`

### `java/util/Collections$UnmodifiableSet`

- `<init>()V`
- `<init>(Ljava/util/Set;)V`

### `java/util/ComparableTimSort`

- `<init>()V`
- `<init>([Ljava/lang/Object;[Ljava/lang/Object;II)V`
- `binarySort([Ljava/lang/Object;III)V`
- `countRunAndMakeAscending([Ljava/lang/Object;II)I`
- `ensureCapacity(I)[Ljava/lang/Object;`
- `gallopLeft(Ljava/lang/Comparable;[Ljava/lang/Object;III)I`
- `gallopRight(Ljava/lang/Comparable;[Ljava/lang/Object;III)I`
- `mergeAt(I)V`
- `mergeCollapse()V`
- `mergeForceCollapse()V`
- `mergeHi(IIII)V`
- `mergeLo(IIII)V`
- `minRunLength(I)I`
- `pushRun(II)V`
- `reverseRange([Ljava/lang/Object;II)V`
- `sort([Ljava/lang/Object;II[Ljava/lang/Object;II)V`

### `java/util/Comparator`

- `compare(Ljava/lang/Object;Ljava/lang/Object;)I`

### `java/util/ConcurrentModificationException`

- `<init>()V`

### `java/util/Date`

- `<init>()V`
- `<init>(IIIIII)V`
- `<init>(J)V`
- `getCalendarSystem(I)Lsun/util/calendar/BaseCalendar;`
- `getCalendarSystem(J)Lsun/util/calendar/BaseCalendar;`
- `getCalendarSystem(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar;`
- `getJulianCalendar()Lsun/util/calendar/BaseCalendar;`
- `getTimeImpl()J`
- `normalize()Lsun/util/calendar/BaseCalendar$Date;`
- `normalize(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar$Date;`
- `parse(Ljava/lang/String;)J`

### `java/util/Deque`

- `isEmpty()Z`
- `pop()Ljava/lang/Object;`
- `push(Ljava/lang/Object;)V`

### `java/util/Dictionary`

- `<init>()V`

### `java/util/DualPivotQuicksort`

- `countingSort([BII)V`
- `countingSort([CII)V`
- `countingSort([SII)V`
- `getDepth(II)I`
- `heapSort([DII)V`
- `heapSort([FII)V`
- `heapSort([III)V`
- `heapSort([JII)V`
- `insertionSort([BII)V`
- `insertionSort([CII)V`
- `insertionSort([DII)V`
- `insertionSort([FII)V`
- `insertionSort([III)V`
- `insertionSort([JII)V`
- `insertionSort([SII)V`
- `mergeParts(Ljava/util/DualPivotQuicksort$Merger;[DI[DII[DII)V`
- `mergeParts(Ljava/util/DualPivotQuicksort$Merger;[FI[FII[FII)V`
- `mergeParts(Ljava/util/DualPivotQuicksort$Merger;[II[III[III)V`
- `mergeParts(Ljava/util/DualPivotQuicksort$Merger;[JI[JII[JII)V`
- `mergeRuns([D[DIIZ[III)[D`
- `mergeRuns([F[FIIZ[III)[F`
- `mergeRuns([I[IIIZ[III)[I`
- `mergeRuns([J[JIIZ[III)[J`
- `mixedInsertionSort([DIII)V`
- `mixedInsertionSort([FIII)V`
- `mixedInsertionSort([IIII)V`
- `mixedInsertionSort([JIII)V`
- `pushDown([DIDII)V`
- `pushDown([FIFII)V`
- `pushDown([IIIII)V`
- `pushDown([JIJII)V`
- `sort(Ljava/util/DualPivotQuicksort$Sorter;[DIII)V`
- `sort(Ljava/util/DualPivotQuicksort$Sorter;[FIII)V`
- `sort(Ljava/util/DualPivotQuicksort$Sorter;[IIII)V`
- `sort(Ljava/util/DualPivotQuicksort$Sorter;[JIII)V`
- `sort([BII)V`
- `sort([CII)V`
- `sort([CIII)V`
- `sort([DIII)V`
- `sort([FIII)V`
- `sort([IIII)V`
- `sort([JIII)V`
- `sort([SII)V`
- `sort([SIII)V`
- `tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[DII)Z`
- `tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[FII)Z`
- `tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[III)Z`
- `tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[JII)Z`

### `java/util/DualPivotQuicksort$Merger`

- `<init>()V`
- `<init>(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V`
- `addToPendingCount(I)V`
- `fork()Ljava/util/concurrent/ForkJoinTask;`
- `forkMerger(Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V`
- `invoke()Ljava/lang/Object;`

### `java/util/DualPivotQuicksort$RunMerger`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;II[III)V`
- `fork()Ljava/util/concurrent/ForkJoinTask;`
- `forkMe()Ljava/util/DualPivotQuicksort$RunMerger;`
- `getDestination()Ljava/lang/Object;`
- `getRawResult()Ljava/lang/Object;`
- `join()Ljava/lang/Object;`

### `java/util/DualPivotQuicksort$Sorter`

- `<init>()V`
- `<init>(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;Ljava/lang/Object;IIII)V`
- `addToPendingCount(I)V`
- `fork()Ljava/util/concurrent/ForkJoinTask;`
- `forkSorter(III)V`
- `invoke()Ljava/lang/Object;`

### `java/util/DuplicateFormatFlagsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/EnumSet`

- `<init>()V`
- `<init>(Ljava/lang/Class;[Ljava/lang/Enum;)V`
- `add(Ljava/lang/Object;)Z`
- `clone()Ljava/util/EnumSet;`
- `copyOf(Ljava/util/Collection;)Ljava/util/EnumSet;`
- `getUniverse(Ljava/lang/Class;)[Ljava/lang/Enum;`
- `noneOf(Ljava/lang/Class;)Ljava/util/EnumSet;`
- `of(Ljava/lang/Enum;)Ljava/util/EnumSet;`

### `java/util/Enumeration`

- `hasMoreElements()Z`
- `nextElement()Ljava/lang/Object;`

### `java/util/FormatFlagsConversionMismatchException`

- `<init>()V`
- `<init>(Ljava/lang/String;C)V`

### `java/util/Formatter`

- `<init>()V`
- `<init>(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V`
- `<init>(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V`
- `<init>(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V`
- `<init>(Ljava/util/Locale;)V`
- `<init>(Ljava/util/Locale;Ljava/lang/Appendable;)V`
- `ensureOpen()V`
- `format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;`
- `format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;`
- `nonNullAppendable(Ljava/lang/Appendable;)Ljava/lang/Appendable;`
- `parse(Ljava/lang/String;)Ljava/util/List;`
- `toCharset(Ljava/lang/String;)Ljava/nio/charset/Charset;`
- `toString()Ljava/lang/String;`

### `java/util/Formatter$Conversion`

- `isCharacter(C)Z`
- `isFloat(C)Z`
- `isGeneral(C)Z`
- `isInteger(C)Z`
- `isText(C)Z`
- `isValid(C)Z`

### `java/util/Formatter$DateTime`

- `isValid(C)Z`

### `java/util/Formatter$FixedString`

- `<init>()V`
- `<init>(Ljava/lang/String;II)V`

### `java/util/Formatter$Flags`

- `add(II)I`
- `contains(II)Z`
- `containsAny(II)Z`
- `parse(C)I`
- `parse(Ljava/lang/String;II)I`
- `remove(II)I`
- `toString(I)Ljava/lang/String;`

### `java/util/Formatter$FormatSpecifier`

- `<init>()V`
- `<init>(C)V`
- `<init>(Ljava/lang/String;Ljava/util/regex/Matcher;)V`
- `checkBadFlags(I)V`
- `checkCharacter()V`
- `checkDateTime()V`
- `checkFloat()V`
- `checkGeneral()V`
- `checkInteger()V`
- `checkNumeric()V`
- `checkText()V`
- `conversion(C)V`
- `failMismatch(IC)V`
- `flags(Ljava/lang/String;II)V`
- `index(Ljava/lang/String;II)V`
- `precision(Ljava/lang/String;II)V`
- `toString()Ljava/lang/String;`
- `width(Ljava/lang/String;II)V`

### `java/util/Formatter$FormatString`

- `<init>()V`
- `index()I`
- `print(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V`
- `toString()Ljava/lang/String;`

### `java/util/FormatterClosedException`

- `<init>()V`

### `java/util/GregorianCalendar`

- `<init>()V`
- `<init>(IIIIIII)V`
- `<init>(Ljava/util/TimeZone;)V`
- `<init>(Ljava/util/TimeZone;Ljava/util/Locale;)V`
- `clear()V`
- `getTimeInMillis()J`
- `getZone()Ljava/util/TimeZone;`
- `internalSet(II)V`
- `set(II)V`
- `set(IIIIII)V`
- `setFieldsComputed(I)V`
- `setTimeInMillis(J)V`
- `setZoneShared(Z)V`

### `java/util/HashMap`

- `<init>()V`
- `<init>(I)V`
- `<init>(IF)V`
- `afterNodeAccess(Ljava/util/HashMap$Node;)V`
- `afterNodeInsertion(Z)V`
- `afterNodeRemoval(Ljava/util/HashMap$Node;)V`
- `calculateHashMapCapacity(I)I`
- `clear()V`
- `comparableClassFor(Ljava/lang/Object;)Ljava/lang/Class;`
- `compareComparables(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I`
- `containsKey(Ljava/lang/Object;)Z`
- `entrySet()Ljava/util/Set;`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getNode(Ljava/lang/Object;)Ljava/util/HashMap$Node;`
- `getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `hash(Ljava/lang/Object;)I`
- `keySet()Ljava/util/Set;`
- `keysToArray([Ljava/lang/Object;)[Ljava/lang/Object;`
- `newHashMap(I)Ljava/util/HashMap;`
- `newNode(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;`
- `newTreeNode(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;`
- `prepareArray([Ljava/lang/Object;)[Ljava/lang/Object;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putMapEntries(Ljava/util/Map;Z)V`
- `putVal(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/lang/Object;`
- `remove(Ljava/lang/Object;)Ljava/lang/Object;`
- `removeNode(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/util/HashMap$Node;`
- `replacementNode(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;`
- `replacementTreeNode(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;`
- `resize()[Ljava/util/HashMap$Node;`
- `size()I`
- `tableSizeFor(I)I`
- `treeifyBin([Ljava/util/HashMap$Node;I)V`

### `java/util/HashMap$EntrySet`

- `<init>()V`
- `<init>(Ljava/util/HashMap;)V`

### `java/util/HashMap$KeySet`

- `<init>()V`
- `<init>(Ljava/util/HashMap;)V`

### `java/util/HashMap$Node`

- `<init>()V`
- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V`

### `java/util/HashMap$TreeNode`

- `<init>()V`
- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V`
- `balanceDeletion(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;`
- `balanceInsertion(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;`
- `checkInvariants(Ljava/util/HashMap$TreeNode;)Z`
- `find(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/HashMap$TreeNode;`
- `getTreeNode(ILjava/lang/Object;)Ljava/util/HashMap$TreeNode;`
- `moveRootToFront([Ljava/util/HashMap$Node;Ljava/util/HashMap$TreeNode;)V`
- `putTreeVal(Ljava/util/HashMap;[Ljava/util/HashMap$Node;ILjava/lang/Object;Ljava/lang/Object;)Ljava/util/HashMap$TreeNode;`
- `removeTreeNode(Ljava/util/HashMap;[Ljava/util/HashMap$Node;Z)V`
- `root()Ljava/util/HashMap$TreeNode;`
- `rotateLeft(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;`
- `rotateRight(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;`
- `split(Ljava/util/HashMap;[Ljava/util/HashMap$Node;II)V`
- `tieBreakOrder(Ljava/lang/Object;Ljava/lang/Object;)I`
- `treeify([Ljava/util/HashMap$Node;)V`
- `untreeify(Ljava/util/HashMap;)Ljava/util/HashMap$Node;`

### `java/util/HashSet`

- `<init>()V`
- `<init>(I)V`
- `<init>(IFZ)V`
- `<init>(Ljava/util/Collection;)V`
- `add(Ljava/lang/Object;)Z`
- `addAll(Ljava/util/Collection;)Z`
- `contains(Ljava/lang/Object;)Z`
- `iterator()Ljava/util/Iterator;`
- `newHashSet(I)Ljava/util/HashSet;`
- `toArray()[Ljava/lang/Object;`

### `java/util/Hashtable`

- `<init>()V`
- `<init>(I)V`
- `<init>(IF)V`
- `<init>(Ljava/lang/Void;)V`
- `addEntry(ILjava/lang/Object;Ljava/lang/Object;I)V`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putAll(Ljava/util/Map;)V`
- `rehash()V`

### `java/util/Hashtable$Entry`

- `<init>()V`
- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V`

### `java/util/HexFormat`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[B)V`
- `checkMaxArraySize(J)I`
- `formatHex(Ljava/lang/Appendable;[B)Ljava/lang/Appendable;`
- `formatHex(Ljava/lang/Appendable;[BII)Ljava/lang/Appendable;`
- `formatHex([B)Ljava/lang/String;`
- `formatHex([BII)Ljava/lang/String;`
- `formatOptDelimiter([BII)Ljava/lang/String;`
- `fromHexDigit(I)I`
- `isHexDigit(I)Z`
- `of()Ljava/util/HexFormat;`
- `toHexDigits(Ljava/lang/Appendable;B)Ljava/lang/Appendable;`
- `toHexDigits(S)Ljava/lang/String;`
- `toHighHexDigit(I)C`
- `toLowHexDigit(I)C`
- `withPrefix(Ljava/lang/String;)Ljava/util/HexFormat;`
- `withUpperCase()Ljava/util/HexFormat;`

### `java/util/IdentityHashMap`

- `<init>()V`
- `<init>(I)V`
- `capacity(I)I`
- `hash(Ljava/lang/Object;I)I`
- `init(I)V`
- `maskNull(Ljava/lang/Object;)Ljava/lang/Object;`
- `nextKeyIndex(II)I`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putAll(Ljava/util/Map;)V`
- `resize(I)Z`

### `java/util/IllegalFormatArgumentIndexException`

- `<init>()V`
- `<init>(I)V`

### `java/util/IllegalFormatException`

- `<init>()V`

### `java/util/IllegalFormatFlagsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/IllegalFormatPrecisionException`

- `<init>()V`
- `<init>(I)V`

### `java/util/IllegalFormatWidthException`

- `<init>()V`
- `<init>(I)V`

### `java/util/IllformedLocaleException`

- `<init>()V`
- `<init>(Ljava/lang/String;I)V`

### `java/util/ImmutableCollections`

- `listCopy(Ljava/util/Collection;)Ljava/util/List;`
- `listFromArray([Ljava/lang/Object;)Ljava/util/List;`
- `listFromTrustedArray([Ljava/lang/Object;)Ljava/util/List;`

### `java/util/ImmutableCollections$AbstractImmutableCollection`

- `<init>()V`

### `java/util/ImmutableCollections$AbstractImmutableList`

- `<init>()V`

### `java/util/ImmutableCollections$AbstractImmutableMap`

- `<init>()V`

### `java/util/ImmutableCollections$AbstractImmutableSet`

- `<init>()V`

### `java/util/ImmutableCollections$List12`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/ImmutableCollections$ListN`

- `<init>()V`
- `<init>([Ljava/lang/Object;Z)V`

### `java/util/ImmutableCollections$Map1`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/ImmutableCollections$MapN`

- `<init>()V`
- `<init>([Ljava/lang/Object;)V`
- `probe(Ljava/lang/Object;)I`

### `java/util/ImmutableCollections$Set12`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/ImmutableCollections$SetN`

- `<init>()V`
- `<init>([Ljava/lang/Object;)V`
- `probe(Ljava/lang/Object;)I`

### `java/util/Iterator`

- `<init>()V`
- `hasNext()Z`
- `next()Ljava/lang/Object;`

### `java/util/JumboEnumSet`

- `<init>()V`
- `<init>(Ljava/lang/Class;[Ljava/lang/Enum;)V`

### `java/util/KeyValueHolder`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/LinkedHashMap`

- `<init>()V`
- `<init>(IF)V`
- `putMapEntries(Ljava/util/Map;Z)V`

### `java/util/LinkedHashMap$Entry`

- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V`

### `java/util/LinkedHashSet`

- `<init>()V`
- `<init>(I)V`
- `<init>(IF)V`
- `add(Ljava/lang/Object;)Z`
- `addAll(Ljava/util/Collection;)Z`
- `iterator()Ljava/util/Iterator;`

### `java/util/LinkedList`

- `<init>()V`
- `<init>(Ljava/util/Collection;)V`
- `add(Ljava/lang/Object;)Z`
- `addAll(ILjava/util/Collection;)Z`
- `addAll(Ljava/util/Collection;)Z`
- `checkPositionIndex(I)V`
- `isEmpty()Z`
- `isPositionIndex(I)Z`
- `iterator()Ljava/util/Iterator;`
- `linkBefore(Ljava/lang/Object;Ljava/util/LinkedList$Node;)V`
- `linkLast(Ljava/lang/Object;)V`
- `node(I)Ljava/util/LinkedList$Node;`
- `outOfBoundsMsg(I)Ljava/lang/String;`
- `removeIf(Ljava/util/function/Predicate;)Z`
- `toArray([Ljava/lang/Object;)[Ljava/lang/Object;`

### `java/util/LinkedList$Node`

- `<init>()V`
- `<init>(Ljava/util/LinkedList$Node;Ljava/lang/Object;Ljava/util/LinkedList$Node;)V`

### `java/util/List`

- `<init>()V`
- `add(Ljava/lang/Object;)Z`
- `addAll(Ljava/util/Collection;)Z`
- `clear()V`
- `contains(Ljava/lang/Object;)Z`
- `copyOf(Ljava/util/Collection;)Ljava/util/List;`
- `equals(Ljava/lang/Object;)Z`
- `get(I)Ljava/lang/Object;`
- `getClass()Ljava/lang/Class;`
- `indexOf(Ljava/lang/Object;)I`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `listIterator()Ljava/util/ListIterator;`
- `listIterator(I)Ljava/util/ListIterator;`
- `of()Ljava/util/List;`
- `of(Ljava/lang/Object;)Ljava/util/List;`
- `of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;`
- `of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;`
- `of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;`
- `of([Ljava/lang/Object;)Ljava/util/List;`
- `remove(I)Ljava/lang/Object;`
- `set(ILjava/lang/Object;)Ljava/lang/Object;`
- `size()I`
- `stream()Ljava/util/stream/Stream;`
- `subList(II)Ljava/util/List;`
- `toArray([Ljava/lang/Object;)[Ljava/lang/Object;`

### `java/util/ListIterator`

- `next()Ljava/lang/Object;`
- `previous()Ljava/lang/Object;`
- `set(Ljava/lang/Object;)V`

### `java/util/Locale`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `convertOldISOCodes(Ljava/lang/String;)Ljava/lang/String;`
- `equals(Ljava/lang/Object;)Z`
- `getBaseLocale()Lsun/util/locale/BaseLocale;`
- `getCompatibilityExtensions(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/LocaleExtensions;`
- `getDefault()Ljava/util/Locale;`
- `getDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;`
- `getDefaultExtensions(Ljava/lang/String;)Ljava/util/Optional;`
- `getDisplayLocale()Ljava/util/Locale;`
- `getFormatLocale()Ljava/util/Locale;`
- `getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;`
- `getInstance(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;`
- `getLanguage()Ljava/lang/String;`
- `getLocaleExtensions()Lsun/util/locale/LocaleExtensions;`
- `getUnicodeLocaleType(Ljava/lang/String;)Ljava/lang/String;`
- `hasExtensions()Z`
- `initDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;`
- `isUnicodeExtensionKey(Ljava/lang/String;)Z`
- `stripExtensions()Ljava/util/Locale;`

### `java/util/Locale$Builder`

- `build()Ljava/util/Locale;`
- `clear()Ljava/util/Locale$Builder;`
- `setLocale(Ljava/util/Locale;)Ljava/util/Locale$Builder;`
- `setRegion(Ljava/lang/String;)Ljava/util/Locale$Builder;`

### `java/util/Locale$Cache`

- `get(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/Locale$LocaleKey`

- `<init>()V`
- `<init>(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V`

### `java/util/Map`

- `<init>()V`
- `clear()V`
- `computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;`
- `containsKey(Ljava/lang/Object;)Z`
- `copyOf(Ljava/util/Map;)Ljava/util/Map;`
- `entry(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map$Entry;`
- `entrySet()Ljava/util/Set;`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `isEmpty()Z`
- `keySet()Ljava/util/Set;`
- `of()Ljava/util/Map;`
- `ofEntries([Ljava/util/Map$Entry;)Ljava/util/Map;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `remove(Ljava/lang/Object;)Ljava/lang/Object;`
- `remove(Ljava/lang/Object;Ljava/lang/Object;)Z`
- `size()I`
- `values()Ljava/util/Collection;`

### `java/util/Map$Entry`

- `<init>()V`
- `getKey()Ljava/lang/Object;`
- `getValue()Ljava/lang/Object;`

### `java/util/MissingFormatArgumentException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/MissingFormatWidthException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/MissingResourceException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`

### `java/util/NavigableMap`

- `size()I`

### `java/util/NavigableSet`

- `iterator()Ljava/util/Iterator;`
- `remove(Ljava/lang/Object;)Z`

### `java/util/NoSuchElementException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/Objects`

- `checkFromIndexSize(III)I`
- `checkFromToIndex(III)I`
- `checkIndex(II)I`
- `equals(Ljava/lang/Object;Ljava/lang/Object;)Z`
- `requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;`
- `requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;`
- `requireNonNullElse(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `toString(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;`

### `java/util/Optional`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`
- `empty()Ljava/util/Optional;`
- `get()Ljava/lang/Object;`
- `isEmpty()Z`
- `isPresent()Z`
- `map(Ljava/util/function/Function;)Ljava/util/Optional;`
- `of(Ljava/lang/Object;)Ljava/util/Optional;`
- `ofNullable(Ljava/lang/Object;)Ljava/util/Optional;`
- `orElse(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/PrimitiveIterator$OfDouble`

- `<init>()V`

### `java/util/PrimitiveIterator$OfInt`

- `<init>()V`

### `java/util/PrimitiveIterator$OfLong`

- `<init>()V`

### `java/util/Properties`

- `<init>()V`
- `<init>(Ljava/util/Properties;I)V`
- `getProperty(Ljava/lang/String;)Ljava/lang/String;`
- `getProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`
- `keySet()Ljava/util/Set;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `setProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;`

### `java/util/PropertyPermission`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `getActions(I)Ljava/lang/String;`
- `getMask(Ljava/lang/String;)I`
- `getName()Ljava/lang/String;`
- `init(I)V`

### `java/util/Random`

- `<init>()V`
- `<init>(J)V`
- `initialScramble(J)J`
- `next(I)I`
- `nextBytes([B)V`
- `nextInt()I`
- `seedUniquifier()J`
- `setSeed(J)V`

### `java/util/RandomAccess`

- `<init>()V`

### `java/util/RegularEnumSet`

- `<init>()V`
- `<init>(Ljava/lang/Class;[Ljava/lang/Enum;)V`

### `java/util/ResourceBundle`

- `<init>()V`
- `getObject(Ljava/lang/String;)Ljava/lang/Object;`
- `getString(Ljava/lang/String;)Ljava/lang/String;`
- `handleGetObject(Ljava/lang/String;)Ljava/lang/Object;`

### `java/util/ResourceBundle$Control`

- `getCandidateLocales(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/List;`
- `getControl(Ljava/util/List;)Ljava/util/ResourceBundle$Control;`
- `getNoFallbackControl(Ljava/util/List;)Ljava/util/ResourceBundle$Control;`

### `java/util/ResourceBundle$Control$CandidateListCache`

- `get(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/ServiceConfigurationError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/util/ServiceLoader`

- `<init>()V`
- `<init>(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/ClassLoader;)V`
- `<init>(Ljava/lang/Class;Ljava/lang/ModuleLayer;Ljava/lang/Class;)V`
- `<init>(Ljava/lang/Module;Ljava/lang/Class;Ljava/lang/ClassLoader;)V`
- `checkCaller(Ljava/lang/Class;Ljava/lang/Class;)V`
- `fail(Ljava/lang/Class;Ljava/lang/String;)V`
- `findFirst()Ljava/util/Optional;`
- `iterator()Ljava/util/Iterator;`
- `load(Ljava/lang/Class;)Ljava/util/ServiceLoader;`
- `newLookupIterator()Ljava/util/Iterator;`

### `java/util/ServiceLoader$2`

- `<init>()V`
- `<init>(Ljava/util/ServiceLoader;Ljava/util/Iterator;Ljava/util/Iterator;)V`

### `java/util/ServiceLoader$3`

- `<init>()V`
- `<init>(Ljava/util/ServiceLoader;)V`

### `java/util/ServiceLoader$LayerLookupIterator`

- `<init>()V`
- `<init>(Ljava/util/ServiceLoader;)V`

### `java/util/ServiceLoader$LazyClassPathLookupIterator`

- `<init>()V`
- `<init>(Ljava/util/ServiceLoader;)V`

### `java/util/ServiceLoader$ModuleServicesLookupIterator`

- `<init>()V`
- `<init>(Ljava/util/ServiceLoader;)V`
- `iteratorFor(Ljava/lang/ClassLoader;)Ljava/util/Iterator;`
- `loaderFor(Ljava/lang/Module;)Ljava/lang/ClassLoader;`
- `providers(Ljava/lang/ModuleLayer;)Ljava/util/List;`

### `java/util/Set`

- `<init>()V`
- `add(Ljava/lang/Object;)Z`
- `addAll(Ljava/util/Collection;)Z`
- `clear()V`
- `contains(Ljava/lang/Object;)Z`
- `copyOf(Ljava/util/Collection;)Ljava/util/Set;`
- `forEach(Ljava/util/function/Consumer;)V`
- `getClass()Ljava/lang/Class;`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `of()Ljava/util/Set;`
- `of([Ljava/lang/Object;)Ljava/util/Set;`
- `remove(Ljava/lang/Object;)Z`
- `size()I`
- `stream()Ljava/util/stream/Stream;`
- `toArray()[Ljava/lang/Object;`
- `toArray([Ljava/lang/Object;)[Ljava/lang/Object;`

### `java/util/SortedMap`

- `<init>()V`
- `comparator()Ljava/util/Comparator;`
- `entrySet()Ljava/util/Set;`
- `isEmpty()Z`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `size()I`

### `java/util/SortedSet`

- `<init>()V`
- `add(Ljava/lang/Object;)Z`
- `comparator()Ljava/util/Comparator;`
- `iterator()Ljava/util/Iterator;`
- `size()I`

### `java/util/Spliterator`

- `characteristics()I`
- `getComparator()Ljava/util/Comparator;`

### `java/util/Spliterators`

- `checkFromToBounds(III)V`
- `emptySpliterator()Ljava/util/Spliterator;`
- `spliterator([DI)Ljava/util/Spliterator$OfDouble;`
- `spliterator([DIII)Ljava/util/Spliterator$OfDouble;`
- `spliterator([II)Ljava/util/Spliterator$OfInt;`
- `spliterator([IIII)Ljava/util/Spliterator$OfInt;`
- `spliterator([JI)Ljava/util/Spliterator$OfLong;`
- `spliterator([JIII)Ljava/util/Spliterator$OfLong;`
- `spliterator([Ljava/lang/Object;I)Ljava/util/Spliterator;`
- `spliterator([Ljava/lang/Object;III)Ljava/util/Spliterator;`

### `java/util/Spliterators$ArraySpliterator`

- `<init>()V`
- `<init>([Ljava/lang/Object;I)V`
- `<init>([Ljava/lang/Object;III)V`

### `java/util/Spliterators$DoubleArraySpliterator`

- `<init>()V`
- `<init>([DI)V`
- `<init>([DIII)V`

### `java/util/Spliterators$DoubleIteratorSpliterator`

- `<init>()V`
- `<init>(Ljava/util/PrimitiveIterator$OfDouble;JI)V`

### `java/util/Spliterators$IntArraySpliterator`

- `<init>()V`
- `<init>([II)V`
- `<init>([IIII)V`

### `java/util/Spliterators$IntIteratorSpliterator`

- `<init>()V`
- `<init>(Ljava/util/PrimitiveIterator$OfInt;JI)V`

### `java/util/Spliterators$IteratorSpliterator`

- `<init>()V`
- `<init>(Ljava/util/Collection;I)V`
- `<init>(Ljava/util/Iterator;JI)V`

### `java/util/Spliterators$LongArraySpliterator`

- `<init>()V`
- `<init>([JI)V`
- `<init>([JIII)V`

### `java/util/Spliterators$LongIteratorSpliterator`

- `<init>()V`
- `<init>(Ljava/util/PrimitiveIterator$OfLong;JI)V`

### `java/util/StringJoiner`

- `<init>()V`
- `<init>(Ljava/lang/CharSequence;)V`
- `<init>(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)V`
- `add(Ljava/lang/CharSequence;)Ljava/util/StringJoiner;`
- `checkAddLength(II)I`
- `toString()Ljava/lang/String;`

### `java/util/StringTokenizer`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Z)V`
- `countTokens()I`
- `hasMoreElements()Z`
- `hasMoreTokens()Z`
- `isDelimiter(I)Z`
- `nextToken()Ljava/lang/String;`
- `scanToken(I)I`
- `setMaxDelimCodePoint()V`
- `skipDelimiters(I)I`

### `java/util/TimSort`

- `<init>()V`
- `<init>([Ljava/lang/Object;Ljava/util/Comparator;[Ljava/lang/Object;II)V`
- `binarySort([Ljava/lang/Object;IIILjava/util/Comparator;)V`
- `countRunAndMakeAscending([Ljava/lang/Object;IILjava/util/Comparator;)I`
- `ensureCapacity(I)[Ljava/lang/Object;`
- `gallopLeft(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I`
- `gallopRight(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I`
- `mergeAt(I)V`
- `mergeCollapse()V`
- `mergeForceCollapse()V`
- `mergeHi(IIII)V`
- `mergeLo(IIII)V`
- `minRunLength(I)I`
- `pushRun(II)V`
- `reverseRange([Ljava/lang/Object;II)V`
- `sort([Ljava/lang/Object;IILjava/util/Comparator;[Ljava/lang/Object;II)V`

### `java/util/TimeZone`

- `<init>()V`
- `clone()Ljava/lang/Object;`
- `getDSTSavings()I`
- `getDefault()Ljava/util/TimeZone;`
- `getDefaultRef()Ljava/util/TimeZone;`
- `getOffset(J)I`
- `getRawOffset()I`
- `getSystemGMTOffsetID()Ljava/lang/String;`
- `getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;`
- `getTimeZone(Ljava/lang/String;)Ljava/util/TimeZone;`
- `getTimeZone(Ljava/lang/String;Z)Ljava/util/TimeZone;`
- `inDaylightTime(Ljava/util/Date;)Z`
- `parseCustomTimeZone(Ljava/lang/String;)Ljava/util/TimeZone;`
- `setDefaultZone()Ljava/util/TimeZone;`
- `useDaylightTime()Z`

### `java/util/TreeMap`

- `<init>()V`
- `<init>(Ljava/util/Comparator;)V`
- `addAllForTreeSet(Ljava/util/SortedSet;Ljava/lang/Object;)V`
- `addEntry(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;Z)V`
- `addEntryToEmptyMap(Ljava/lang/Object;Ljava/lang/Object;)V`
- `buildFromSorted(IIIILjava/util/Iterator;Ljava/io/ObjectInputStream;Ljava/lang/Object;)Ljava/util/TreeMap$Entry;`
- `buildFromSorted(ILjava/util/Iterator;Ljava/io/ObjectInputStream;Ljava/lang/Object;)V`
- `colorOf(Ljava/util/TreeMap$Entry;)Z`
- `comparator()Ljava/util/Comparator;`
- `compare(Ljava/lang/Object;Ljava/lang/Object;)I`
- `computeRedLevel(I)I`
- `entrySet()Ljava/util/Set;`
- `fixAfterInsertion(Ljava/util/TreeMap$Entry;)V`
- `leftOf(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;`
- `parentOf(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `put(Ljava/lang/Object;Ljava/lang/Object;Z)Ljava/lang/Object;`
- `putAll(Ljava/util/Map;)V`
- `rightOf(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;`
- `rotateLeft(Ljava/util/TreeMap$Entry;)V`
- `rotateRight(Ljava/util/TreeMap$Entry;)V`
- `setColor(Ljava/util/TreeMap$Entry;Z)V`

### `java/util/TreeMap$Entry`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;)V`

### `java/util/TreeMap$EntrySet`

- `<init>()V`
- `<init>(Ljava/util/TreeMap;)V`

### `java/util/TreeSet`

- `<init>()V`
- `<init>(Ljava/util/Comparator;)V`
- `<init>(Ljava/util/NavigableMap;)V`
- `addAll(Ljava/util/Collection;)Z`

### `java/util/UnknownFormatConversionException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/UnknownFormatFlagsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/Vector`

- `<init>()V`
- `<init>(I)V`
- `<init>(II)V`
- `add(Ljava/lang/Object;[Ljava/lang/Object;I)V`
- `addElement(Ljava/lang/Object;)V`
- `elements()Ljava/util/Enumeration;`
- `grow()[Ljava/lang/Object;`
- `grow(I)[Ljava/lang/Object;`
- `insertElementAt(Ljava/lang/Object;I)V`

### `java/util/Vector$1`

- `<init>()V`
- `<init>(Ljava/util/Vector;)V`

### `java/util/WeakHashMap`

- `<init>()V`
- `<init>(IF)V`
- `expungeStaleEntries()V`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getTable()[Ljava/util/WeakHashMap$Entry;`
- `hash(Ljava/lang/Object;)I`
- `indexFor(II)I`
- `maskNull(Ljava/lang/Object;)Ljava/lang/Object;`
- `matchesKey(Ljava/util/WeakHashMap$Entry;Ljava/lang/Object;)Z`
- `newTable(I)[Ljava/util/WeakHashMap$Entry;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putAll(Ljava/util/Map;)V`
- `resize(I)V`
- `transfer([Ljava/util/WeakHashMap$Entry;[Ljava/util/WeakHashMap$Entry;)V`

### `java/util/WeakHashMap$Entry`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;ILjava/util/WeakHashMap$Entry;)V`
- `get()Ljava/lang/Object;`
- `refersTo(Ljava/lang/Object;)Z`

### `java/util/concurrent/Callable`

- `call()Ljava/lang/Object;`

### `java/util/concurrent/ConcurrentHashMap`

- `<init>()V`
- `<init>(I)V`
- `<init>(IFI)V`
- `addCount(JI)V`
- `casTabAt([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)Z`
- `comparableClassFor(Ljava/lang/Object;)Ljava/lang/Class;`
- `compareComparables(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I`
- `containsKey(Ljava/lang/Object;)Z`
- `fullAddCount(JZ)V`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `helpTransfer([Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)[Ljava/util/concurrent/ConcurrentHashMap$Node;`
- `initTable()[Ljava/util/concurrent/ConcurrentHashMap$Node;`
- `keySet()Ljava/util/concurrent/ConcurrentHashMap$KeySetView;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putAll(Ljava/util/Map;)V`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putVal(Ljava/lang/Object;Ljava/lang/Object;Z)Ljava/lang/Object;`
- `remove(Ljava/lang/Object;)Ljava/lang/Object;`
- `replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z`
- `replaceNode(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `resizeStamp(I)I`
- `setTabAt([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;)V`
- `spread(I)I`
- `sumCount()J`
- `tabAt([Ljava/util/concurrent/ConcurrentHashMap$Node;I)Ljava/util/concurrent/ConcurrentHashMap$Node;`
- `tableSizeFor(I)I`
- `transfer([Ljava/util/concurrent/ConcurrentHashMap$Node;[Ljava/util/concurrent/ConcurrentHashMap$Node;)V`
- `treeifyBin([Ljava/util/concurrent/ConcurrentHashMap$Node;I)V`
- `tryPresize(I)V`
- `untreeify(Ljava/util/concurrent/ConcurrentHashMap$Node;)Ljava/util/concurrent/ConcurrentHashMap$Node;`

### `java/util/concurrent/ConcurrentHashMap$CollectionView`

- `<init>(Ljava/util/concurrent/ConcurrentHashMap;)V`

### `java/util/concurrent/ConcurrentHashMap$CounterCell`

- `<init>()V`
- `<init>(J)V`

### `java/util/concurrent/ConcurrentHashMap$ForwardingNode`

- `<init>()V`
- `<init>([Ljava/util/concurrent/ConcurrentHashMap$Node;)V`

### `java/util/concurrent/ConcurrentHashMap$KeySetView`

- `<init>()V`
- `<init>(Ljava/util/concurrent/ConcurrentHashMap;Ljava/lang/Object;)V`

### `java/util/concurrent/ConcurrentHashMap$Node`

- `<init>()V`
- `<init>(ILjava/lang/Object;Ljava/lang/Object;)V`
- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/concurrent/ConcurrentHashMap$Node;)V`
- `find(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;`

### `java/util/concurrent/ConcurrentHashMap$ReservationNode`

- `<init>()V`

### `java/util/concurrent/ConcurrentHashMap$TreeBin`

- `<init>()V`
- `<init>(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)V`
- `balanceDeletion(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`
- `balanceInsertion(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`
- `checkInvariants(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Z`
- `contendedLock()V`
- `lockRoot()V`
- `putTreeVal(ILjava/lang/Object;Ljava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`
- `removeTreeNode(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Z`
- `rotateLeft(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`
- `rotateRight(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`
- `tieBreakOrder(Ljava/lang/Object;Ljava/lang/Object;)I`
- `unlockRoot()V`

### `java/util/concurrent/ConcurrentHashMap$TreeNode`

- `<init>()V`
- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)V`
- `findTreeNode(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`

### `java/util/concurrent/ConcurrentMap`

- `<init>()V`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `remove(Ljava/lang/Object;)Ljava/lang/Object;`
- `remove(Ljava/lang/Object;Ljava/lang/Object;)Z`

### `java/util/concurrent/CopyOnWriteArrayList`

- `<init>()V`
- `getArray()[Ljava/lang/Object;`
- `setArray([Ljava/lang/Object;)V`

### `java/util/concurrent/CountedCompleter`

- `<init>(Ljava/util/concurrent/CountedCompleter;)V`

### `java/util/concurrent/ForkJoinTask`

- `<init>()V`

### `java/util/concurrent/RecursiveTask`

- `<init>()V`
- `join()Ljava/lang/Object;`

### `java/util/concurrent/ThreadLocalRandom`

- `advanceProbe(I)I`
- `current()Ljava/util/concurrent/ThreadLocalRandom;`
- `getProbe()I`
- `localInit()V`

### `java/util/concurrent/atomic/AtomicInteger`

- `addAndGet(I)I`
- `getAndAdd(I)I`

### `java/util/concurrent/atomic/AtomicLong`

- `<init>()V`
- `<init>(J)V`
- `compareAndSet(JJ)Z`
- `get()J`
- `getAndAdd(J)J`
- `incrementAndGet()J`
- `set(J)V`

### `java/util/concurrent/locks/AbstractOwnableSynchronizer`

- `<init>()V`

### `java/util/concurrent/locks/AbstractQueuedSynchronizer`

- `<init>()V`

### `java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionObject`

- `<init>()V`
- `<init>(Ljava/util/concurrent/locks/AbstractQueuedSynchronizer;)V`

### `java/util/concurrent/locks/LockSupport`

- `park(Ljava/lang/Object;)V`
- `setBlocker(Ljava/lang/Thread;Ljava/lang/Object;)V`

### `java/util/concurrent/locks/ReentrantLock`

- `<init>()V`
- `lock()V`
- `newCondition()Ljava/util/concurrent/locks/Condition;`
- `unlock()V`

### `java/util/concurrent/locks/ReentrantLock$FairSync`

- `<init>()V`

### `java/util/concurrent/locks/ReentrantLock$NonfairSync`

- `<init>()V`

### `java/util/concurrent/locks/ReentrantLock$Sync`

- `<init>()V`
- `acquire(I)V`
- `initialTryLock()Z`
- `lock()V`
- `newCondition()Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionObject;`
- `release(I)Z`

### `java/util/function/Function`

- `apply(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/function/IntFunction`

- `apply(I)Ljava/lang/Object;`

### `java/util/function/Supplier`

- `get()Ljava/lang/Object;`

### `java/util/regex/ASCII`

- `getType(I)I`
- `isAlnum(I)Z`
- `isAlpha(I)Z`
- `isAscii(I)Z`
- `isDigit(I)Z`
- `isHexDigit(I)Z`
- `isLower(I)Z`
- `isSpace(I)Z`
- `isType(II)Z`
- `isUpper(I)Z`
- `toDigit(I)I`
- `toLower(I)I`
- `toUpper(I)I`

### `java/util/regex/CharPredicates`

- `ALNUM()Ljava/util/regex/Pattern$CharPredicate;`
- `ALPHABETIC()Ljava/util/regex/Pattern$CharPredicate;`
- `ASCII_DIGIT()Ljava/util/regex/Pattern$BmpCharPredicate;`
- `ASCII_SPACE()Ljava/util/regex/Pattern$BmpCharPredicate;`
- `ASCII_WORD()Ljava/util/regex/Pattern$BmpCharPredicate;`
- `ASSIGNED()Ljava/util/regex/Pattern$CharPredicate;`
- `BLANK()Ljava/util/regex/Pattern$CharPredicate;`
- `CONTROL()Ljava/util/regex/Pattern$CharPredicate;`
- `DIGIT()Ljava/util/regex/Pattern$CharPredicate;`
- `EMOJI()Ljava/util/regex/Pattern$CharPredicate;`
- `EMOJI_COMPONENT()Ljava/util/regex/Pattern$CharPredicate;`
- `EMOJI_MODIFIER()Ljava/util/regex/Pattern$CharPredicate;`
- `EMOJI_MODIFIER_BASE()Ljava/util/regex/Pattern$CharPredicate;`
- `EMOJI_PRESENTATION()Ljava/util/regex/Pattern$CharPredicate;`
- `EXTENDED_PICTOGRAPHIC()Ljava/util/regex/Pattern$CharPredicate;`
- `GRAPH()Ljava/util/regex/Pattern$CharPredicate;`
- `HEX_DIGIT()Ljava/util/regex/Pattern$CharPredicate;`
- `IDEOGRAPHIC()Ljava/util/regex/Pattern$CharPredicate;`
- `JOIN_CONTROL()Ljava/util/regex/Pattern$CharPredicate;`
- `LETTER()Ljava/util/regex/Pattern$CharPredicate;`
- `LOWERCASE()Ljava/util/regex/Pattern$CharPredicate;`
- `NONCHARACTER_CODE_POINT()Ljava/util/regex/Pattern$CharPredicate;`
- `PRINT()Ljava/util/regex/Pattern$CharPredicate;`
- `PUNCTUATION()Ljava/util/regex/Pattern$CharPredicate;`
- `TITLECASE()Ljava/util/regex/Pattern$CharPredicate;`
- `UPPERCASE()Ljava/util/regex/Pattern$CharPredicate;`
- `WHITE_SPACE()Ljava/util/regex/Pattern$CharPredicate;`
- `WORD()Ljava/util/regex/Pattern$CharPredicate;`
- `category(I)Ljava/util/regex/Pattern$CharPredicate;`
- `ctype(I)Ljava/util/regex/Pattern$CharPredicate;`
- `forPOSIXName(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `forProperty(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `forUnicodeBlock(Ljava/lang/String;)Ljava/util/regex/Pattern$CharPredicate;`
- `forUnicodeProperty(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `forUnicodeScript(Ljava/lang/String;)Ljava/util/regex/Pattern$CharPredicate;`
- `getPosixPredicate(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `getUnicodePredicate(Ljava/lang/String;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `range(II)Ljava/util/regex/Pattern$CharPredicate;`

### `java/util/regex/IntHashSet`

- `<init>()V`
- `clear()V`

### `java/util/regex/Matcher`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V`
- `appendExpandedReplacement(Ljava/lang/Appendable;Ljava/lang/String;)V`
- `appendReplacement(Ljava/lang/StringBuilder;Ljava/lang/String;)Ljava/util/regex/Matcher;`
- `appendTail(Ljava/lang/StringBuilder;)Ljava/lang/StringBuilder;`
- `checkGroup(I)V`
- `checkMatch()V`
- `end()I`
- `end(I)I`
- `find()Z`
- `find(I)Z`
- `getMatchedGroupIndex(Ljava/lang/String;)I`
- `getSubSequence(II)Ljava/lang/CharSequence;`
- `getTextLength()I`
- `group(I)Ljava/lang/String;`
- `groupCount()I`
- `hasMatch()Z`
- `namedGroups()Ljava/util/Map;`
- `replaceAll(Ljava/lang/String;)Ljava/lang/String;`
- `replaceFirst(Ljava/lang/String;)Ljava/lang/String;`
- `reset()Ljava/util/regex/Matcher;`
- `search(I)Z`
- `start()I`
- `start(I)I`

### `java/util/regex/Pattern`

- `<init>()V`
- `<init>(Ljava/lang/String;I)V`
- `ALL()Ljava/util/regex/Pattern$CharPredicate;`
- `CIRange(II)Ljava/util/regex/Pattern$CharPredicate;`
- `CIRangeU(II)Ljava/util/regex/Pattern$CharPredicate;`
- `DOT()Ljava/util/regex/Pattern$CharPredicate;`
- `HorizWS()Ljava/util/regex/Pattern$BmpCharPredicate;`
- `N()I`
- `Range(II)Ljava/util/regex/Pattern$CharPredicate;`
- `RemoveQEQuoting()V`
- `Single(I)Ljava/util/regex/Pattern$BmpCharPredicate;`
- `SingleI(II)Ljava/util/regex/Pattern$BmpCharPredicate;`
- `SingleS(I)Ljava/util/regex/Pattern$CharPredicate;`
- `SingleU(I)Ljava/util/regex/Pattern$CharPredicate;`
- `UNIXDOT()Ljava/util/regex/Pattern$CharPredicate;`
- `VertWS()Ljava/util/regex/Pattern$BmpCharPredicate;`
- `accept(ILjava/lang/String;)V`
- `addFlag()V`
- `and(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `append(II)V`
- `atom()Ljava/util/regex/Pattern$Node;`
- `bitsOrSingle(Ljava/util/regex/Pattern$BitClass;I)Ljava/util/regex/Pattern$CharPredicate;`
- `c()I`
- `clazz(Z)Ljava/util/regex/Pattern$CharPredicate;`
- `closure(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;`
- `compile()V`
- `compile(Ljava/lang/String;)Ljava/util/regex/Pattern;`
- `composeOneStep(Ljava/lang/String;)Ljava/lang/String;`
- `countChars(Ljava/lang/CharSequence;II)I`
- `countCodePoints(Ljava/lang/CharSequence;)I`
- `createGroup(Z)Ljava/util/regex/Pattern$Node;`
- `curly(Ljava/util/regex/Pattern$Node;I)Ljava/util/regex/Pattern$Node;`
- `cursor()I`
- `error(Ljava/lang/String;)Ljava/util/regex/PatternSyntaxException;`
- `escape(ZZZ)I`
- `expr(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;`
- `family(ZZ)Ljava/util/regex/Pattern$CharPredicate;`
- `findSupplementary(II)Z`
- `getClass(I)I`
- `group0()Ljava/util/regex/Pattern$Node;`
- `groupname(I)Ljava/lang/String;`
- `has(I)Z`
- `isLineSeparator(I)Z`
- `isSupplementary(I)Z`
- `mark(I)V`
- `matcher(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;`
- `namedGroups()Ljava/util/Map;`
- `namedGroupsMap()Ljava/util/Map;`
- `negate(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;`
- `newCharProperty(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharProperty;`
- `newSlice([IIZ)Ljava/util/regex/Pattern$Node;`
- `next()I`
- `nextEscaped()I`
- `normalize(Ljava/lang/String;)Ljava/lang/String;`
- `normalizeClazz(Ljava/lang/String;IILjava/lang/StringBuilder;)V`
- `normalizeSlice(Ljava/lang/String;IILjava/lang/StringBuilder;)V`
- `o()I`
- `parsePastLine()I`
- `parsePastWhitespace(I)I`
- `peek()I`
- `peekPastLine()I`
- `peekPastWhitespace(I)I`
- `produceEquivalentAlternation(Ljava/lang/String;Ljava/util/Set;)V`
- `producePermutations(Ljava/lang/String;)[Ljava/lang/String;`
- `qtype()Ljava/util/regex/Pattern$Qtype;`
- `range(Ljava/util/regex/Pattern$BitClass;)Ljava/util/regex/Pattern$CharPredicate;`
- `read()I`
- `ref(I)Ljava/util/regex/Pattern$Node;`
- `sequence(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;`
- `setcursor(I)V`
- `single(I)Ljava/util/regex/Pattern$CharPredicate;`
- `skip()I`
- `split(Ljava/lang/CharSequence;I)[Ljava/lang/String;`
- `split(Ljava/lang/CharSequence;IZ)[Ljava/lang/String;`
- `splitWithDelimiters(Ljava/lang/CharSequence;I)[Ljava/lang/String;`
- `subFlag()V`
- `u()I`
- `union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;`
- `unread()V`
- `uxxxx()I`
- `x()I`

### `java/util/regex/Pattern$BackRef`

- `<init>()V`
- `<init>(I)V`

### `java/util/regex/Pattern$Begin`

- `<init>()V`

### `java/util/regex/Pattern$Behind`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$BehindS`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$BitClass`

- `<init>()V`
- `add(II)Ljava/util/regex/Pattern$BitClass;`

### `java/util/regex/Pattern$BmpCharPredicate`

- `<init>()V`
- `negate()Ljava/util/regex/Pattern$CharPredicate;`

### `java/util/regex/Pattern$BmpCharProperty`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$BmpCharPredicate;)V`

### `java/util/regex/Pattern$BmpCharPropertyGreedy`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$BmpCharProperty;I)V`

### `java/util/regex/Pattern$BnM`

- `<init>()V`
- `<init>([I[I[ILjava/util/regex/Pattern$Node;)V`
- `optimize(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;`

### `java/util/regex/Pattern$BnMS`

- `<init>()V`
- `<init>([I[I[ILjava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$Bound`

- `<init>()V`
- `<init>(IZ)V`

### `java/util/regex/Pattern$Branch`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;Ljava/util/regex/Pattern$Node;Ljava/util/regex/Pattern$Node;)V`
- `add(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$BranchConn`

- `<init>()V`

### `java/util/regex/Pattern$CIBackRef`

- `<init>()V`
- `<init>(IZ)V`

### `java/util/regex/Pattern$Caret`

- `<init>()V`

### `java/util/regex/Pattern$CharPredicate`

- `and(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;`
- `negate()Ljava/util/regex/Pattern$CharPredicate;`
- `union(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;`
- `union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;`

### `java/util/regex/Pattern$CharProperty`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$CharPredicate;)V`

### `java/util/regex/Pattern$CharPropertyGreedy`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$CharProperty;I)V`

### `java/util/regex/Pattern$Curly`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;IILjava/util/regex/Pattern$Qtype;)V`

### `java/util/regex/Pattern$Dollar`

- `<init>()V`
- `<init>(Z)V`

### `java/util/regex/Pattern$End`

- `<init>()V`

### `java/util/regex/Pattern$First`

- `<init>()V`

### `java/util/regex/Pattern$GraphemeBound`

- `<init>()V`

### `java/util/regex/Pattern$GroupCurly`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;IILjava/util/regex/Pattern$Qtype;IIZ)V`

### `java/util/regex/Pattern$GroupHead`

- `<init>()V`
- `<init>(I)V`

### `java/util/regex/Pattern$GroupTail`

- `<init>()V`
- `<init>(II)V`

### `java/util/regex/Pattern$LastMatch`

- `<init>()V`

### `java/util/regex/Pattern$LazyLoop`

- `<init>()V`
- `<init>(II)V`

### `java/util/regex/Pattern$LineEnding`

- `<init>()V`

### `java/util/regex/Pattern$Loop`

- `<init>()V`
- `<init>(II)V`

### `java/util/regex/Pattern$NFCCharProperty`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$CharPredicate;)V`

### `java/util/regex/Pattern$Neg`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$Node`

- `<init>()V`
- `match(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z`
- `study(Ljava/util/regex/Pattern$TreeInfo;)Z`

### `java/util/regex/Pattern$NotBehind`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$NotBehindS`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$Pos`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$Prolog`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Loop;)V`

### `java/util/regex/Pattern$Ques`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;Ljava/util/regex/Pattern$Qtype;)V`

### `java/util/regex/Pattern$Slice`

- `<init>()V`
- `<init>([I)V`

### `java/util/regex/Pattern$SliceI`

- `<init>()V`
- `<init>([I)V`

### `java/util/regex/Pattern$SliceIS`

- `<init>()V`
- `<init>([I)V`

### `java/util/regex/Pattern$SliceNode`

- `<init>([I)V`

### `java/util/regex/Pattern$SliceS`

- `<init>()V`
- `<init>([I)V`

### `java/util/regex/Pattern$SliceU`

- `<init>()V`
- `<init>([I)V`

### `java/util/regex/Pattern$SliceUS`

- `<init>()V`
- `<init>([I)V`

### `java/util/regex/Pattern$Start`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$StartS`

- `<init>()V`
- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$TreeInfo`

- `<init>()V`
- `reset()V`

### `java/util/regex/Pattern$UnixCaret`

- `<init>()V`

### `java/util/regex/Pattern$UnixDollar`

- `<init>()V`
- `<init>(Z)V`

### `java/util/regex/Pattern$XGrapheme`

- `<init>()V`

### `java/util/regex/PatternSyntaxException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;I)V`

### `java/util/spi/LocaleServiceProvider`

- `<init>()V`
- `<init>(Ljava/lang/Void;)V`
- `checkPermission()Ljava/lang/Void;`
- `getAvailableLocales()[Ljava/util/Locale;`
- `isSupportedLocale(Ljava/util/Locale;)Z`

### `java/util/stream/AbstractPipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`
- `<init>(Ljava/util/stream/AbstractPipeline;I)V`
- `opIsStateful()Z`

### `java/util/stream/Collector`

- `accumulator()Ljava/util/function/BiConsumer;`
- `characteristics()Ljava/util/Set;`
- `combiner()Ljava/util/function/BinaryOperator;`
- `finisher()Ljava/util/function/Function;`
- `supplier()Ljava/util/function/Supplier;`

### `java/util/stream/Collectors`

- `castingIdentity()Ljava/util/function/Function;`
- `groupingBy(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;`
- `groupingBy(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;`
- `joining(Ljava/lang/CharSequence;)Ljava/util/stream/Collector;`
- `joining(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;`
- `mapMerger(Ljava/util/function/BinaryOperator;)Ljava/util/function/BinaryOperator;`
- `mapping(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;`
- `toList()Ljava/util/stream/Collector;`
- `toSet()Ljava/util/stream/Collector;`

### `java/util/stream/Collectors$CollectorImpl`

- `<init>()V`
- `<init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V`
- `<init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V`

### `java/util/stream/DoublePipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/DoublePipeline$Head`

- `<init>()V`
- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/IntPipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/IntPipeline$Head`

- `<init>()V`
- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/LongPipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/LongPipeline$Head`

- `<init>()V`
- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/PipelineHelper`

- `<init>()V`

### `java/util/stream/ReferencePipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/ReferencePipeline$Head`

- `<init>()V`
- `<init>(Ljava/util/Spliterator;IZ)V`
- `<init>(Ljava/util/function/Supplier;IZ)V`

### `java/util/stream/Stream`

- `collect(Ljava/util/stream/Collector;)Ljava/lang/Object;`
- `count()J`
- `distinct()Ljava/util/stream/Stream;`
- `empty()Ljava/util/stream/Stream;`
- `filter(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;`
- `findAny()Ljava/util/Optional;`
- `findFirst()Ljava/util/Optional;`
- `flatMap(Ljava/util/function/Function;)Ljava/util/stream/Stream;`
- `forEach(Ljava/util/function/Consumer;)V`
- `iterator()Ljava/util/Iterator;`
- `map(Ljava/util/function/Function;)Ljava/util/stream/Stream;`
- `of([Ljava/lang/Object;)Ljava/util/stream/Stream;`
- `skip(J)Ljava/util/stream/Stream;`
- `toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;`

### `java/util/stream/StreamOpFlag`

- `combineOpFlags(II)I`
- `fromCharacteristics(I)I`
- `fromCharacteristics(Ljava/util/Spliterator;)I`
- `getMask(I)I`

### `java/util/stream/StreamSupport`

- `doubleStream(Ljava/util/Spliterator$OfDouble;Z)Ljava/util/stream/DoubleStream;`
- `intStream(Ljava/util/Spliterator$OfInt;Z)Ljava/util/stream/IntStream;`
- `longStream(Ljava/util/Spliterator$OfLong;Z)Ljava/util/stream/LongStream;`
- `stream(Ljava/util/Spliterator;Z)Ljava/util/stream/Stream;`

### `java/util/stream/Streams$AbstractStreamBuilderImpl`

- `<init>()V`

### `java/util/stream/Streams$StreamBuilderImpl`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`

### `java/util/zip/CRC32`

- `<init>()V`

### `java/util/zip/Inflater`

- `<init>()V`
- `<init>(Z)V`
- `init(Z)J`

### `java/util/zip/Inflater$InflaterZStreamRef`

- `<init>()V`
- `<init>(Ljava/util/zip/Inflater;J)V`

### `java/util/zip/InflaterInputStream`

- `<init>()V`
- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/io/InputStream;Ljava/util/zip/Inflater;)V`
- `<init>(Ljava/io/InputStream;Ljava/util/zip/Inflater;I)V`

### `javax/security/auth/callback/Callback`

- `<init>()V`

### `javax/security/auth/callback/CallbackHandler`

- `handle([Ljavax/security/auth/callback/Callback;)V`

### `javax/security/auth/callback/PasswordCallback`

- `<init>()V`
- `<init>(Ljava/lang/String;Z)V`
- `clearPassword()V`
- `getPassword()[C`

### `javax/security/auth/x500/X500Principal`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/util/Map;)V`
- `<init>([B)V`
- `equals(Ljava/lang/Object;)Z`
- `getName()Ljava/lang/String;`
- `getName(Ljava/lang/String;)Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `sun/invoke/util/BytecodeDescriptor`

- `parseError(Ljava/lang/String;Ljava/lang/String;)V`
- `parseMethod(Ljava/lang/String;IILjava/lang/ClassLoader;)Ljava/util/List;`
- `parseMethod(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/util/List;`
- `parseSig(Ljava/lang/String;[IILjava/lang/ClassLoader;)Ljava/lang/Class;`
- `unparse(Ljava/lang/Class;)Ljava/lang/String;`
- `unparseMethod(Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/String;`
- `unparseSig(Ljava/lang/Class;Ljava/lang/StringBuilder;)V`

### `sun/invoke/util/ValueConversions`

- `boxExact(Lsun/invoke/util/Wrapper;)Ljava/lang/invoke/MethodHandle;`
- `boxType(Lsun/invoke/util/Wrapper;)Ljava/lang/invoke/MethodType;`
- `capitalize(Ljava/lang/String;)Ljava/lang/String;`
- `convertPrimitive(Ljava/lang/Class;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `convertPrimitive(Lsun/invoke/util/Wrapper;Lsun/invoke/util/Wrapper;)Ljava/lang/invoke/MethodHandle;`
- `fromBoolean(Z)B`
- `unbox(Lsun/invoke/util/Wrapper;I)Ljava/lang/invoke/MethodHandle;`
- `unboxCast(Lsun/invoke/util/Wrapper;)Ljava/lang/invoke/MethodHandle;`
- `unboxExact(Lsun/invoke/util/Wrapper;Z)Ljava/lang/invoke/MethodHandle;`
- `unboxType(Lsun/invoke/util/Wrapper;I)Ljava/lang/invoke/MethodType;`
- `unboxWiden(Lsun/invoke/util/Wrapper;)Ljava/lang/invoke/MethodHandle;`
- `widenSubword(Ljava/lang/Object;)I`

### `sun/invoke/util/ValueConversions$WrapperCache`

- `get(Lsun/invoke/util/Wrapper;)Ljava/lang/invoke/MethodHandle;`
- `put(Lsun/invoke/util/Wrapper;Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`

### `sun/invoke/util/VerifyAccess`

- `classLoaderIsAncestor(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `getClassModifiers(Ljava/lang/Class;)I`
- `isClassAccessible(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z`
- `isMemberAccessible(Ljava/lang/Class;Ljava/lang/Class;ILjava/lang/Class;Ljava/lang/Class;I)Z`
- `isModuleAccessible(Ljava/lang/Class;Ljava/lang/Module;Ljava/lang/Module;)Z`
- `isRelatedClass(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `isSamePackage(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `isSubClass(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `isTypeVisible(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `isTypeVisible(Ljava/lang/invoke/MethodType;Ljava/lang/Class;)Z`
- `loadersAreRelated(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;Z)Z`

### `sun/invoke/util/VerifyAccess$1`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/ClassLoader;)V`

### `sun/invoke/util/VerifyType`

- `isNullConversion(Ljava/lang/Class;Ljava/lang/Class;Z)Z`
- `isNullType(Ljava/lang/Class;)Z`

### `sun/invoke/util/Wrapper`

- `asPrimitiveType(Ljava/lang/Class;)Ljava/lang/Class;`
- `asWrapperType(Ljava/lang/Class;)Ljava/lang/Class;`
- `basicTypeChar()C`
- `basicTypeChar(Ljava/lang/Class;)C`
- `basicTypeError(C)Ljava/lang/RuntimeException;`
- `basicTypeString()Ljava/lang/String;`
- `bitWidth()I`
- `boolValue(B)Z`
- `compareTo(Ljava/lang/Enum;)I`
- `convert(Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;`
- `convert(Ljava/lang/Object;Ljava/lang/Class;Z)Ljava/lang/Object;`
- `findWrapperType(Ljava/lang/Class;)Lsun/invoke/util/Wrapper;`
- `forBasicType(C)Lsun/invoke/util/Wrapper;`
- `forBasicType(Ljava/lang/Class;)Lsun/invoke/util/Wrapper;`
- `forPrimitiveType(Ljava/lang/Class;)Lsun/invoke/util/Wrapper;`
- `forWrapperType(Ljava/lang/Class;)Lsun/invoke/util/Wrapper;`
- `forceType(Ljava/lang/Class;Ljava/lang/Class;)Ljava/lang/Class;`
- `isConvertibleFrom(Lsun/invoke/util/Wrapper;)Z`
- `isDoubleWord()Z`
- `isFloating()Z`
- `isIntegral()Z`
- `isNumeric()Z`
- `isOther()Z`
- `isSigned()Z`
- `isSingleWord()Z`
- `isSubwordOrInt()Z`
- `isWrapperType(Ljava/lang/Class;)Z`
- `newClassCastException(Ljava/lang/Class;Ljava/lang/Class;)Ljava/lang/ClassCastException;`
- `newIllegalArgumentException(Ljava/lang/String;)Ljava/lang/RuntimeException;`
- `numberValue(Ljava/lang/Object;)Ljava/lang/Number;`
- `ordinal()I`
- `primitiveSimpleName()Ljava/lang/String;`
- `primitiveType()Ljava/lang/Class;`
- `stackSlots()I`
- `values()[Lsun/invoke/util/Wrapper;`
- `wrap(Ljava/lang/Object;)Ljava/lang/Object;`
- `wrapperSimpleName()Ljava/lang/String;`
- `wrapperType()Ljava/lang/Class;`
- `wrapperType(Ljava/lang/Class;)Ljava/lang/Class;`
- `wrapperTypeError(Ljava/lang/Class;)Ljava/lang/RuntimeException;`
- `zero()Ljava/lang/Object;`

### `sun/net/ApplicationProxy`

- `<init>()V`
- `<init>(Ljava/net/Proxy;)V`
- `create(Ljava/net/Proxy;)Lsun/net/ApplicationProxy;`

### `sun/net/util/IPAddressUtil`

- `checkAuth(Ljava/lang/String;)Ljava/lang/String;`
- `checkExternalForm(Ljava/net/URL;)Ljava/lang/String;`
- `checkHost(Ljava/lang/String;)Ljava/lang/String;`
- `checkHostString(Ljava/lang/String;)Ljava/lang/String;`
- `checkPrefix(Ljava/nio/CharBuffer;I)Z`
- `checkUserInfo(Ljava/lang/String;)Ljava/lang/String;`
- `convertFromIPv4MappedAddress([B)[B`
- `describeChar(C)Ljava/lang/String;`
- `digit(CI)I`
- `earlyURLParsing()Z`
- `isBsdParsableV4(Ljava/lang/String;)Z`
- `isDecimalFieldStart(Ljava/nio/CharBuffer;)Z`
- `isHexFieldStart(Ljava/nio/CharBuffer;)Z`
- `isIPv4MappedAddress([B)Z`
- `isIPv6LiteralAddress(Ljava/lang/String;)Z`
- `isOctalFieldStart(Ljava/nio/CharBuffer;)Z`
- `match(CJJ)Z`
- `parseAsciiDigit(CI)I`
- `parseAsciiHexDigit(C)I`
- `parseV4FieldBsd(ILjava/nio/CharBuffer;I)J`
- `scan(Ljava/lang/String;JJ)I`
- `scan(Ljava/lang/String;JJ[C)I`
- `textToNumericFormatV4(Ljava/lang/String;)[B`
- `textToNumericFormatV6(Ljava/lang/String;)[B`
- `validateNumericFormatV4(Ljava/lang/String;)[B`

### `sun/net/util/URLUtil`

- `urlNoFragString(Ljava/net/URL;)Ljava/lang/String;`

### `sun/net/www/ParseUtil`

- `decode(Ljava/lang/String;)Ljava/lang/String;`
- `encodePath(Ljava/lang/String;)Ljava/lang/String;`
- `encodePath(Ljava/lang/String;IC)Ljava/lang/String;`
- `encodePath(Ljava/lang/String;Z)Ljava/lang/String;`
- `escape([CCI)I`
- `fileToEncodedURL(Ljava/io/File;)Ljava/net/URL;`
- `firstEncodeIndex(Ljava/lang/String;)I`
- `match(CJJ)Z`
- `unescape(Ljava/lang/String;I)B`

### `sun/net/www/protocol/jar/Handler`

- `<init>()V`
- `checkNestedProtocol(Ljava/lang/String;)Ljava/lang/String;`

### `sun/nio/ch/Interruptible`

- `interrupt(Ljava/lang/Thread;)V`

### `sun/nio/cs/ArrayDecoder`

- `<init>()V`
- `decode([BII[C)I`
- `decodeToLatin1([BII[B)I`
- `isASCIICompatible()Z`
- `isLatin1Decodable()Z`

### `sun/nio/cs/ArrayEncoder`

- `<init>()V`
- `encodeFromLatin1([BII[B)I`
- `encodeFromUTF16([BII[B)I`
- `isASCIICompatible()Z`

### `sun/nio/cs/StandardCharsets`

- `aliases_UTF_32BE()[Ljava/lang/String;`

### `sun/nio/cs/StreamDecoder`

- `<init>()V`
- `<init>(Ljava/io/InputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)V`
- `<init>(Ljava/io/InputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetDecoder;)V`
- `close()V`
- `forInputStreamReader(Ljava/io/InputStream;Ljava/lang/Object;Ljava/lang/String;)Lsun/nio/cs/StreamDecoder;`
- `forInputStreamReader(Ljava/io/InputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)Lsun/nio/cs/StreamDecoder;`
- `forInputStreamReader(Ljava/io/InputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetDecoder;)Lsun/nio/cs/StreamDecoder;`
- `implClose()V`
- `lockedClose()V`

### `sun/nio/cs/StreamEncoder`

- `<init>()V`
- `<init>(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)V`
- `<init>(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetEncoder;)V`
- `flushBuffer()V`
- `forOutputStreamWriter(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/lang/String;)Lsun/nio/cs/StreamEncoder;`
- `forOutputStreamWriter(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)Lsun/nio/cs/StreamEncoder;`
- `forOutputStreamWriter(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetEncoder;)Lsun/nio/cs/StreamEncoder;`
- `implFlushBuffer()V`
- `isOpen()Z`
- `lockedFlushBuffer()V`
- `writeBytes()V`

### `sun/nio/cs/UTF_32BE`

- `<init>()V`

### `sun/nio/cs/UTF_8`

- `newDecoder()Ljava/nio/charset/CharsetDecoder;`
- `newEncoder()Ljava/nio/charset/CharsetEncoder;`

### `sun/nio/cs/UTF_8$Decoder`

- `<init>()V`
- `<init>(Ljava/nio/charset/Charset;)V`

### `sun/nio/cs/UTF_8$Encoder`

- `<init>()V`
- `<init>(Ljava/nio/charset/Charset;)V`

### `sun/nio/cs/Unicode`

- `<init>(Ljava/lang/String;[Ljava/lang/String;)V`

### `sun/reflect/generics/factory/CoreReflectionFactory`

- `<init>()V`
- `<init>(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)V`
- `make(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)Lsun/reflect/generics/factory/CoreReflectionFactory;`

### `sun/reflect/generics/repository/AbstractRepository`

- `<init>(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V`
- `parse(Ljava/lang/String;)Lsun/reflect/generics/tree/Tree;`

### `sun/reflect/generics/repository/ClassRepository`

- `<init>()V`
- `<init>(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V`
- `computeSuperInterfaces()[Ljava/lang/reflect/Type;`
- `getReifier()Lsun/reflect/generics/visitor/Reifier;`
- `getSuperInterfaces()[Ljava/lang/reflect/Type;`
- `getTree()Lsun/reflect/generics/tree/Tree;`
- `make(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)Lsun/reflect/generics/repository/ClassRepository;`

### `sun/reflect/generics/repository/GenericDeclRepository`

- `<init>(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V`

### `sun/reflect/generics/scope/AbstractScope`

- `<init>(Ljava/lang/reflect/GenericDeclaration;)V`

### `sun/reflect/generics/scope/ClassScope`

- `<init>()V`
- `<init>(Ljava/lang/Class;)V`
- `make(Ljava/lang/Class;)Lsun/reflect/generics/scope/ClassScope;`

### `sun/reflect/generics/tree/ClassSignature`

- `<init>()V`
- `getSuperInterfaces()[Lsun/reflect/generics/tree/ClassTypeSignature;`

### `sun/reflect/generics/tree/TypeTree`

- `accept(Lsun/reflect/generics/visitor/TypeTreeVisitor;)V`

### `sun/reflect/generics/visitor/Reifier`

- `getResult()Ljava/lang/reflect/Type;`

### `sun/reflect/misc/ReflectUtil`

- `checkPackageAccess(Ljava/lang/Class;)V`
- `checkProxyPackageAccess(Ljava/lang/ClassLoader;[Ljava/lang/Class;)V`
- `isAncestor(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`
- `isNonPublicProxyClass(Ljava/lang/Class;)Z`
- `needsPackageAccessCheck(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`
- `privateCheckPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/Class;)V`
- `privateCheckProxyPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/Class;)V`

### `sun/security/action/GetPropertyAction`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `privilegedGetProperties()Ljava/util/Properties;`
- `privilegedGetProperty(Ljava/lang/String;)Ljava/lang/String;`

### `sun/security/action/GetPropertyAction$1`

- `<init>()V`

### `sun/security/jca/GetInstance`

- `checkSuperClass(Ljava/security/Provider$Service;Ljava/lang/Class;Ljava/lang/Class;)V`
- `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;)Lsun/security/jca/GetInstance$Instance;`
- `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Object;)Lsun/security/jca/GetInstance$Instance;`
- `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Object;Ljava/lang/String;)Lsun/security/jca/GetInstance$Instance;`
- `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Object;Ljava/security/Provider;)Lsun/security/jca/GetInstance$Instance;`
- `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;)Lsun/security/jca/GetInstance$Instance;`
- `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;Ljava/security/Provider;)Lsun/security/jca/GetInstance$Instance;`
- `getInstance(Ljava/security/Provider$Service;Ljava/lang/Class;)Lsun/security/jca/GetInstance$Instance;`
- `getInstance(Ljava/security/Provider$Service;Ljava/lang/Class;Ljava/lang/Object;)Lsun/security/jca/GetInstance$Instance;`
- `getService(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service;`
- `getService(Ljava/lang/String;Ljava/lang/String;Ljava/security/Provider;)Ljava/security/Provider$Service;`
- `getServices(Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;`

### `sun/security/jca/GetInstance$Instance`

- `<init>()V`
- `<init>(Ljava/security/Provider;Ljava/lang/Object;)V`
- `toArray()[Ljava/lang/Object;`

### `sun/security/jca/ProviderConfig`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `checkSunPKCS11Solaris()V`
- `doLoadProvider()Ljava/security/Provider;`
- `expand(Ljava/lang/String;)Ljava/lang/String;`
- `getProvider()Ljava/security/Provider;`
- `isLoaded()Z`
- `shouldLoad()Z`

### `sun/security/jca/ProviderConfig$1`

- `<init>()V`
- `<init>(Lsun/security/jca/ProviderConfig;)V`

### `sun/security/jca/ProviderConfig$2`

- `<init>()V`
- `<init>(Lsun/security/jca/ProviderConfig;)V`

### `sun/security/jca/ProviderConfig$3`

- `<init>()V`
- `<init>(Lsun/security/jca/ProviderConfig;)V`

### `sun/security/jca/ProviderConfig$4`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `sun/security/jca/ProviderList`

- `<init>()V`
- `<init>([Lsun/security/jca/ProviderConfig;Z)V`
- `getIndex(Ljava/lang/String;)I`
- `getProvider(I)Ljava/security/Provider;`
- `getProvider(Ljava/lang/String;)Ljava/security/Provider;`
- `getProviderConfig(Ljava/lang/String;)Lsun/security/jca/ProviderConfig;`
- `getService(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service;`
- `getServices(Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;`
- `getServices(Ljava/lang/String;Ljava/util/List;)Ljava/util/List;`
- `getServices(Ljava/util/List;)Ljava/util/List;`
- `loadAll()I`
- `providers()Ljava/util/List;`
- `removeInvalid()Lsun/security/jca/ProviderList;`
- `toArray()[Ljava/security/Provider;`

### `sun/security/jca/ProviderList$3`

- `<init>()V`
- `<init>(Lsun/security/jca/ProviderList;)V`

### `sun/security/jca/ProviderList$PreferredEntry`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `match(Ljava/lang/String;Ljava/lang/String;)Z`
- `print(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `sun/security/jca/ProviderList$PreferredList`

- `<init>()V`
- `add(Lsun/security/jca/ProviderList$PreferredEntry;)Z`
- `getAll(Ljava/lang/String;Ljava/lang/String;)Ljava/util/ArrayList;`
- `implGetAll(Ljava/util/ArrayList;Ljava/lang/String;Ljava/lang/String;)V`
- `size()I`

### `sun/security/jca/ProviderList$ServiceList`

- `<init>()V`
- `<init>(Lsun/security/jca/ProviderList;Ljava/lang/String;Ljava/lang/String;)V`
- `<init>(Lsun/security/jca/ProviderList;Ljava/util/List;)V`

### `sun/security/jca/Providers`

- `changeThreadProviderList(Lsun/security/jca/ProviderList;)V`
- `getFullProviderList()Lsun/security/jca/ProviderList;`
- `getProviderList()Lsun/security/jca/ProviderList;`
- `getSystemProviderList()Lsun/security/jca/ProviderList;`
- `getThreadProviderList()Lsun/security/jca/ProviderList;`
- `setSystemProviderList(Lsun/security/jca/ProviderList;)V`

### `sun/security/jca/ServiceId`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`

### `sun/security/provider/NativePRNG`

- `isAvailable()Z`

### `sun/security/provider/NativePRNG$Blocking`

- `isAvailable()Z`

### `sun/security/provider/NativePRNG$NonBlocking`

- `isAvailable()Z`

### `sun/security/provider/PolicyFile`

- `<init>()V`
- `addGrantEntry(Lsun/security/provider/PolicyParser$GrantEntry;Ljava/security/KeyStore;Lsun/security/provider/PolicyFile$PolicyInfo;)V`
- `canonPath(Ljava/lang/String;)Ljava/lang/String;`
- `canonicalizeCodebase(Ljava/security/CodeSource;Z)Ljava/security/CodeSource;`
- `expandPermissionName(Lsun/security/provider/PolicyParser$PermissionEntry;Ljava/security/KeyStore;)V`
- `getCertificates(Ljava/security/KeyStore;Ljava/lang/String;Lsun/security/provider/PolicyFile$PolicyInfo;)[Ljava/security/cert/Certificate;`
- `getCodeSource(Lsun/security/provider/PolicyParser$GrantEntry;Ljava/security/KeyStore;Lsun/security/provider/PolicyFile$PolicyInfo;)Ljava/security/CodeSource;`
- `getDN(Ljava/lang/String;Ljava/security/KeyStore;)Ljava/lang/String;`
- `getInputStreamReader(Ljava/io/InputStream;)Ljava/io/InputStreamReader;`
- `getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/security/Permission;`
- `getKnownPermission(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;)Ljava/security/Permission;`
- `getSignerCertificates(Ljava/security/CodeSource;)[Ljava/security/cert/Certificate;`
- `init(Ljava/net/URL;)V`
- `initPolicyFile(Ljava/lang/String;Ljava/lang/String;Lsun/security/provider/PolicyFile$PolicyInfo;)Z`
- `initPolicyFile(Lsun/security/provider/PolicyFile$PolicyInfo;Ljava/net/URL;)V`
- `initStaticPolicy(Lsun/security/provider/PolicyFile$PolicyInfo;)V`
- `newURL(Ljava/lang/String;)Ljava/net/URL;`
- `replacePrincipals(Ljava/util/List;Ljava/security/KeyStore;)Z`

### `sun/security/provider/PolicyFile$1`

- `<init>()V`
- `<init>(Lsun/security/provider/PolicyFile;)V`

### `sun/security/provider/PolicyFile$2`

- `<init>()V`
- `<init>(Lsun/security/provider/PolicyFile;Lsun/security/provider/PolicyFile$PolicyInfo;)V`

### `sun/security/provider/PolicyFile$3`

- `<init>()V`
- `<init>(Lsun/security/provider/PolicyFile;Ljava/net/URL;Lsun/security/provider/PolicyFile$PolicyInfo;)V`

### `sun/security/provider/PolicyFile$4`

- `<init>()V`
- `<init>(Lsun/security/provider/PolicyFile;Ljava/lang/String;Lsun/security/provider/PolicyFile$PolicyInfo;Ljava/lang/String;)V`

### `sun/security/provider/PolicyFile$5`

- `<init>()V`
- `<init>(Lsun/security/provider/PolicyFile;Lsun/security/provider/PolicyFile$PolicyInfo;)V`

### `sun/security/provider/PolicyFile$PolicyEntry`

- `<init>()V`
- `<init>(Ljava/security/CodeSource;Ljava/util/List;)V`
- `add(Ljava/security/Permission;)V`

### `sun/security/provider/PolicyFile$PolicyInfo`

- `<init>()V`
- `<init>(I)V`

### `sun/security/provider/PolicyFile$SelfPermission`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/security/cert/Certificate;)V`

### `sun/security/provider/PolicyParser`

- `<init>()V`
- `<init>(Z)V`
- `add(Lsun/security/provider/PolicyParser$GrantEntry;)V`
- `expand(Ljava/lang/String;)Ljava/lang/String;`
- `expand(Ljava/lang/String;Z)Ljava/lang/String;`
- `getKeyStoreProvider()Ljava/lang/String;`
- `getKeyStoreType()Ljava/lang/String;`
- `getKeyStoreUrl()Ljava/lang/String;`
- `getStorePassURL()Ljava/lang/String;`
- `grantElements()Ljava/util/Enumeration;`
- `match(Ljava/lang/String;)Ljava/lang/String;`
- `parseDomainEntry()Lsun/security/provider/PolicyParser$DomainEntry;`
- `parseGrantEntry()Lsun/security/provider/PolicyParser$GrantEntry;`
- `parseKeyStoreEntry()V`
- `parsePermissionEntry()Lsun/security/provider/PolicyParser$PermissionEntry;`
- `parseProperties(Ljava/lang/String;)Ljava/util/Map;`
- `parseStorePassURL()V`
- `peek(Ljava/lang/String;)Z`
- `peekAndMatch(Ljava/lang/String;)Z`
- `read(Ljava/io/Reader;)V`
- `skipEntry()V`

### `sun/security/provider/PolicyParser$DomainEntry`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/util/Map;)V`
- `add(Lsun/security/provider/PolicyParser$KeyStoreEntry;)V`
- `getName()Ljava/lang/String;`

### `sun/security/provider/PolicyParser$GrantEntry`

- `<init>()V`
- `add(Lsun/security/provider/PolicyParser$PermissionEntry;)V`
- `permissionElements()Ljava/util/Enumeration;`

### `sun/security/provider/PolicyParser$KeyStoreEntry`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/util/Map;)V`
- `getName()Ljava/lang/String;`

### `sun/security/provider/PolicyParser$ParsingException`

- `<init>()V`
- `<init>(ILjava/lang/String;)V`
- `<init>(ILjava/lang/String;Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Lsun/security/util/LocalizedMessage;[Ljava/lang/Object;)V`
- `getNonlocalizedMessage()Ljava/lang/String;`
- `printStackTrace()V`

### `sun/security/provider/PolicyParser$PermissionEntry`

- `<init>()V`

### `sun/security/provider/PolicyParser$PrincipalEntry`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `getDisplayClass()Ljava/lang/String;`
- `getDisplayName()Ljava/lang/String;`
- `getDisplayName(Z)Ljava/lang/String;`
- `isReplaceName()Z`
- `isWildcardClass()Z`
- `isWildcardName()Z`
- `toString()Ljava/lang/String;`

### `sun/security/provider/Sun`

- `<init>()V`
- `putEntries(Ljava/util/Iterator;)V`
- `putService(Ljava/security/Provider$Service;)V`

### `sun/security/provider/Sun$1`

- `<init>()V`
- `<init>(Lsun/security/provider/Sun;Ljava/util/Iterator;)V`

### `sun/security/provider/SunEntries`

- `<init>()V`
- `<init>(Ljava/security/Provider;)V`
- `add(Ljava/security/Provider;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `add(Ljava/security/Provider;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/HashMap;)V`
- `addWithAlias(Ljava/security/Provider;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/HashMap;)V`
- `iterator()Ljava/util/Iterator;`

### `sun/security/rsa/SunRsaSign`

- `<init>()V`
- `putEntries(Ljava/util/Iterator;)V`
- `putService(Ljava/security/Provider$Service;)V`

### `sun/security/rsa/SunRsaSign$1`

- `<init>()V`
- `<init>(Lsun/security/rsa/SunRsaSign;Ljava/util/Iterator;)V`

### `sun/security/rsa/SunRsaSignEntries`

- `<init>()V`
- `<init>(Ljava/security/Provider;)V`
- `add(Ljava/security/Provider;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/List;Ljava/util/HashMap;)V`
- `addA(Ljava/security/Provider;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/HashMap;)V`
- `iterator()Ljava/util/Iterator;`

### `sun/security/ssl/SunJSSE`

- `<init>()V`
- `registerAlgorithms()V`

### `sun/security/util/BitArray`

- `<init>()V`
- `<init>(I)V`
- `<init>(I[B)V`
- `<init>(I[BI)V`
- `<init>(Lsun/security/util/BitArray;)V`
- `clone()Ljava/lang/Object;`
- `length()I`
- `position(I)I`
- `set(IZ)V`
- `subscript(I)I`
- `toByteArray()[B`

### `sun/security/util/CryptoAlgorithmConstraints`

- `cachedCheckAlgorithm(Ljava/lang/String;)Z`
- `checkAlgorithm(Ljava/util/Set;Ljava/lang/String;Lsun/security/util/AlgorithmDecomposer;)Z`
- `permits(Ljava/lang/String;Ljava/lang/String;)Z`

### `sun/security/util/Debug`

- `<init>()V`
- `configureExtras(Ljava/lang/String;)V`
- `extraInfo()Ljava/lang/String;`
- `formatCaller()Ljava/lang/String;`
- `getInstance(Ljava/lang/String;)Lsun/security/util/Debug;`
- `getInstance(Ljava/lang/String;Ljava/lang/String;)Lsun/security/util/Debug;`
- `isOn(Ljava/lang/String;)Z`
- `println()V`
- `println(Ljava/lang/String;)V`

### `sun/security/util/DerEncoder`

- `encode(Lsun/security/util/DerOutputStream;)V`

### `sun/security/util/DerIndefLenConverter`

- `<init>()V`
- `convertBytes([B)[B`
- `convertStream(Ljava/io/InputStream;B)[B`
- `getLengthBytes(I)[B`
- `isEOC([BI)Z`
- `isIndefinite(I)Z`
- `isLongForm(I)Z`
- `parseLength()I`
- `parseTag()V`
- `writeLength(I)V`
- `writeLengthAndValue()V`
- `writeTag()V`
- `writeValue(I)V`

### `sun/security/util/DerInputStream`

- `<init>()V`
- `<init>([B)V`
- `<init>([BIIZ)V`
- `available()I`
- `getDefiniteLength(Ljava/io/InputStream;)I`
- `getDerValue()Lsun/security/util/DerValue;`
- `getGeneralizedTime()Ljava/util/Date;`
- `getLength(Ljava/io/InputStream;)I`
- `getOID()Lsun/security/util/ObjectIdentifier;`
- `getSequence(I)[Lsun/security/util/DerValue;`
- `getSet(I)[Lsun/security/util/DerValue;`
- `getUTCTime()Ljava/util/Date;`
- `getUnalignedBitString()Lsun/security/util/BitArray;`
- `toByteArray()[B`

### `sun/security/util/DerOutputStream`

- `<init>()V`
- `buf()[B`
- `putLength(I)V`
- `putNull()Lsun/security/util/DerOutputStream;`
- `putOID(Lsun/security/util/ObjectIdentifier;)Lsun/security/util/DerOutputStream;`
- `putUnalignedBitString(Lsun/security/util/BitArray;)Lsun/security/util/DerOutputStream;`
- `size()I`
- `toByteArray()[B`
- `write(BLsun/security/util/DerOutputStream;)Lsun/security/util/DerOutputStream;`
- `write(B[B)Lsun/security/util/DerOutputStream;`
- `write(I)V`
- `write([BII)V`
- `writeBytes([B)V`
- `writeImplicit(BLsun/security/util/DerOutputStream;)Lsun/security/util/DerOutputStream;`

### `sun/security/util/DerValue`

- `<init>()V`
- `<init>(BLjava/lang/String;)V`
- `<init>(B[B)V`
- `<init>(B[BIIZ)V`
- `<init>(B[BZ)V`
- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/io/InputStream;Z)V`
- `<init>(Ljava/lang/String;)V`
- `<init>([B)V`
- `<init>([BIIZZ)V`
- `checkPaddedBits(I[BIIZ)I`
- `createTag(BZB)B`
- `data()Lsun/security/util/DerInputStream;`
- `getAsString()Ljava/lang/String;`
- `getBMPString()Ljava/lang/String;`
- `getBigInteger()Ljava/math/BigInteger;`
- `getBigIntegerInternal(BZ)Ljava/math/BigInteger;`
- `getBitString()[B`
- `getBitString(Z)[B`
- `getBoolean()Z`
- `getData()Lsun/security/util/DerInputStream;`
- `getDataBytes()[B`
- `getGeneralString()Ljava/lang/String;`
- `getGeneralizedTime()Ljava/util/Date;`
- `getIA5String()Ljava/lang/String;`
- `getInteger()I`
- `getIntegerInternal(B)I`
- `getOID()Lsun/security/util/ObjectIdentifier;`
- `getOctetString()[B`
- `getPrintableString()Ljava/lang/String;`
- `getT61String()Ljava/lang/String;`
- `getTimeInternal(Z)Ljava/util/Date;`
- `getUTCTime()Ljava/util/Date;`
- `getUTF8String()Ljava/lang/String;`
- `getUnalignedBitString()Lsun/security/util/BitArray;`
- `getUnalignedBitString(Z)Lsun/security/util/BitArray;`
- `getUniversalString()Ljava/lang/String;`
- `isConstructed()Z`
- `isConstructed(B)Z`
- `isContextSpecific()Z`
- `isContextSpecific(B)Z`
- `isPrintableString(Ljava/lang/String;)Z`
- `isPrintableStringChar(C)Z`
- `length()I`
- `readStringInternal(BLjava/nio/charset/Charset;)Ljava/lang/String;`
- `resetTag(B)V`
- `string2bytes(BLjava/lang/String;)[B`
- `subs(BI)[Lsun/security/util/DerValue;`
- `toByteArray()[B`
- `toDerInputStream()Lsun/security/util/DerInputStream;`
- `toDigit(BLjava/lang/String;)I`
- `validateBMPString()V`

### `sun/security/util/FilePermCompat`

- `newPermPlusAltPath(Ljava/security/Permission;)Ljava/security/Permission;`
- `newPermUsingAltPath(Ljava/security/Permission;)Ljava/security/Permission;`

### `sun/security/util/HexDumpEncoder`

- `<init>()V`
- `bytesPerAtom()I`
- `bytesPerLine()I`
- `encodeAtom(Ljava/io/OutputStream;[BII)V`
- `encodeBuffer(Ljava/io/InputStream;Ljava/io/OutputStream;)V`
- `encodeBuffer([B)Ljava/lang/String;`
- `encodeBuffer([BLjava/io/OutputStream;)V`
- `encodeBufferPrefix(Ljava/io/OutputStream;)V`
- `encodeLinePrefix(Ljava/io/OutputStream;I)V`
- `encodeLineSuffix(Ljava/io/OutputStream;)V`
- `getBytes(Ljava/nio/ByteBuffer;)[B`
- `hexDigit(Ljava/io/PrintStream;B)V`
- `readFully(Ljava/io/InputStream;[B)I`

### `sun/security/util/IOUtils`

- `readExactlyNBytes(Ljava/io/InputStream;I)[B`

### `sun/security/util/KnownOIDs`

- `<init>()V`
- `aliases()[Ljava/lang/String;`
- `findMatch(Ljava/lang/String;)Lsun/security/util/KnownOIDs;`
- `name()Ljava/lang/String;`
- `stdName()Ljava/lang/String;`
- `value()Ljava/lang/String;`

### `sun/security/util/LocalizedMessage`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `formatNonlocalized([Ljava/lang/Object;)Ljava/lang/String;`
- `getNonlocalized(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;`

### `sun/security/util/ObjectIdentifier`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>([B)V`
- `check([B)V`
- `checkCount(I)V`
- `checkFirstComponent(I)V`
- `checkFirstComponent(Ljava/math/BigInteger;)V`
- `checkOidSize(I)V`
- `checkOtherComponent(II)V`
- `checkOtherComponent(ILjava/math/BigInteger;)V`
- `checkSecondComponent(II)V`
- `checkSecondComponent(ILjava/math/BigInteger;)V`
- `encode(Lsun/security/util/DerOutputStream;)V`
- `equals(Ljava/lang/Object;)Z`
- `of(Ljava/lang/String;)Lsun/security/util/ObjectIdentifier;`
- `pack([BIIII)[B`
- `pack7Oid(I[BI)I`
- `pack7Oid(Ljava/math/BigInteger;[BI)I`
- `pack7Oid([BII[BI)I`
- `toString()Ljava/lang/String;`

### `sun/security/util/Password`

- `readPassword(Ljava/io/InputStream;)[C`
- `readPassword(Ljava/io/InputStream;Z)[C`

### `sun/security/util/Password$ConsoleHolder`

- `consoleIsAvailable()Z`
- `convertToBytes([C)[B`
- `readPassword()[C`

### `sun/security/util/Pem`

- `decode(Ljava/lang/String;)[B`

### `sun/security/util/PolicyUtil`

- `getInputStream(Ljava/net/URL;)Ljava/io/InputStream;`
- `getKeyStore(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/security/util/Debug;)Ljava/security/KeyStore;`

### `sun/security/util/PropertyExpander`

- `expand(Ljava/lang/String;Z)Ljava/lang/String;`

### `sun/security/util/PropertyExpander$ExpandException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `getLocalizedMessage()Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `sun/security/util/Resources`

- `getString(Ljava/lang/String;)Ljava/lang/String;`

### `sun/security/util/ResourcesMgr`

- `getBundle(Ljava/lang/String;)Ljava/util/ResourceBundle;`
- `getString(Ljava/lang/String;)Ljava/lang/String;`

### `sun/security/util/SecurityProviderConstants`

- `getAliases(Ljava/lang/String;)Ljava/util/List;`
- `store(Ljava/lang/String;Lsun/security/util/KnownOIDs;[Ljava/lang/String;)Ljava/util/List;`

### `sun/security/x509/AVA`

- `<init>()V`
- `<init>(Ljava/io/Reader;I)V`
- `<init>(Ljava/io/Reader;ILjava/util/Map;)V`
- `<init>(Ljava/io/Reader;Ljava/util/Map;)V`
- `<init>(Lsun/security/util/DerValue;)V`
- `<init>(Lsun/security/util/ObjectIdentifier;Lsun/security/util/DerValue;)V`
- `getCharset(Lsun/security/util/DerValue;Z)Ljava/nio/charset/Charset;`
- `getEmbeddedHexPair(ILjava/io/Reader;)Ljava/lang/Byte;`
- `getEmbeddedHexString(Ljava/util/List;)Ljava/lang/String;`
- `isDerString(Lsun/security/util/DerValue;Z)Z`
- `isTerminator(II)Z`
- `parseHexString(Ljava/io/Reader;I)Lsun/security/util/DerValue;`
- `parseQuotedString(Ljava/io/Reader;Ljava/lang/StringBuilder;)Lsun/security/util/DerValue;`
- `parseString(Ljava/io/Reader;IILjava/lang/StringBuilder;)Lsun/security/util/DerValue;`
- `readChar(Ljava/io/Reader;Ljava/lang/String;)I`
- `toKeyword(ILjava/util/Map;)Ljava/lang/String;`
- `toKeywordValueString(Ljava/lang/String;)Ljava/lang/String;`
- `toRFC1779String(Ljava/util/Map;)Ljava/lang/String;`
- `toRFC2253CanonicalString()Ljava/lang/String;`
- `toRFC2253String(Ljava/util/Map;)Ljava/lang/String;`
- `toString()Ljava/lang/String;`
- `trailingSpace(Ljava/io/Reader;)Z`

### `sun/security/x509/AVAComparator`

- `getInstance()Ljava/util/Comparator;`

### `sun/security/x509/AVAKeyword`

- `<init>()V`
- `getKeyword(Lsun/security/util/ObjectIdentifier;ILjava/util/Map;)Ljava/lang/String;`
- `getOID(Ljava/lang/String;ILjava/util/Map;)Lsun/security/util/ObjectIdentifier;`
- `isCompliant(I)Z`

### `sun/security/x509/AlgorithmId`

- `<init>()V`
- `<init>(Lsun/security/util/ObjectIdentifier;Lsun/security/util/DerValue;)V`
- `aliasOidsTable()Ljava/util/Map;`
- `collectOIDAliases()Ljava/util/concurrent/ConcurrentHashMap;`
- `decodeParams()V`
- `encode(Lsun/security/util/DerOutputStream;)V`
- `equals(Lsun/security/util/ObjectIdentifier;)Z`
- `equals(Lsun/security/x509/AlgorithmId;)Z`
- `getName()Ljava/lang/String;`
- `isKnownProvider(Ljava/security/Provider;)Z`
- `parse(Lsun/security/util/DerValue;)Lsun/security/x509/AlgorithmId;`

### `sun/security/x509/CertificateAlgorithmId`

- `<init>()V`
- `<init>(Lsun/security/util/DerInputStream;)V`
- `getAlgId()Lsun/security/x509/AlgorithmId;`

### `sun/security/x509/CertificateExtensions`

- `<init>()V`
- `<init>(Lsun/security/util/DerInputStream;)V`
- `getExtension(Ljava/lang/String;)Lsun/security/x509/Extension;`
- `init(Lsun/security/util/DerInputStream;)V`
- `parseExtension(Lsun/security/x509/Extension;)V`

### `sun/security/x509/CertificateSerialNumber`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`

### `sun/security/x509/CertificateValidity`

- `<init>()V`
- `<init>(Lsun/security/util/DerInputStream;)V`

### `sun/security/x509/CertificateVersion`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`
- `compare(I)I`
- `construct(Lsun/security/util/DerValue;)V`

### `sun/security/x509/CertificateX509Key`

- `<init>()V`
- `<init>(Lsun/security/util/DerInputStream;)V`

### `sun/security/x509/DNSName`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Z)V`
- `<init>(Lsun/security/util/DerValue;)V`

### `sun/security/x509/EDIPartyName`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `<init>(Lsun/security/util/DerValue;)V`

### `sun/security/x509/Extension`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`
- `<init>(Lsun/security/x509/Extension;)V`
- `getExtensionId()Lsun/security/util/ObjectIdentifier;`
- `getExtensionValue()[B`
- `getId()Ljava/lang/String;`
- `getName()Ljava/lang/String;`
- `isCritical()Z`

### `sun/security/x509/GeneralName`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`
- `<init>(Lsun/security/util/DerValue;Z)V`
- `encode(Lsun/security/util/DerOutputStream;)V`

### `sun/security/x509/GeneralNameInterface`

- `<init>()V`
- `encode(Lsun/security/util/DerOutputStream;)V`
- `getType()I`
- `toString()Ljava/lang/String;`

### `sun/security/x509/GeneralNames`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`
- `add(Lsun/security/x509/GeneralName;)Lsun/security/x509/GeneralNames;`
- `encode(Lsun/security/util/DerOutputStream;)V`
- `isEmpty()Z`

### `sun/security/x509/IPAddressName`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Lsun/security/util/DerValue;)V`
- `<init>([B)V`
- `parseIPv4(Ljava/lang/String;)V`
- `parseIPv6(Ljava/lang/String;)V`

### `sun/security/x509/OIDMap`

- `getClass(Lsun/security/util/ObjectIdentifier;)Ljava/lang/Class;`

### `sun/security/x509/OIDMap$OIDInfo`

- `<init>()V`
- `getClazz()Ljava/lang/Class;`

### `sun/security/x509/OIDName`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`

### `sun/security/x509/OtherName`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`
- `getGNI(Lsun/security/util/ObjectIdentifier;[B)Lsun/security/x509/GeneralNameInterface;`

### `sun/security/x509/RDN`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/util/Map;)V`
- `<init>(Ljava/lang/String;Ljava/util/Map;)V`
- `<init>(Lsun/security/util/DerValue;)V`
- `toRFC1779String(Ljava/util/Map;)Ljava/lang/String;`
- `toRFC2253String(Ljava/util/Map;)Ljava/lang/String;`
- `toRFC2253String(Z)Ljava/lang/String;`
- `toRFC2253StringInternal(ZLjava/util/Map;)Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `sun/security/x509/RFC822Name`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`
- `parseName(Ljava/lang/String;)V`

### `sun/security/x509/SerialNumber`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/math/BigInteger;)V`
- `<init>(Lsun/security/util/DerInputStream;)V`
- `<init>(Lsun/security/util/DerValue;)V`
- `construct(Lsun/security/util/DerValue;)V`

### `sun/security/x509/SubjectAlternativeNameExtension`

- `<init>()V`
- `<init>(Ljava/lang/Boolean;Lsun/security/x509/GeneralNames;)V`
- `encodeThis()V`
- `getNames()Lsun/security/x509/GeneralNames;`
- `isCritical()Z`

### `sun/security/x509/URIName`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/net/URI;Ljava/lang/String;Lsun/security/x509/DNSName;)V`
- `<init>(Lsun/security/util/DerValue;)V`
- `nameConstraint(Lsun/security/util/DerValue;)Lsun/security/x509/URIName;`

### `sun/security/x509/UniqueIdentity`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`

### `sun/security/x509/UnparseableExtension`

- `<init>()V`
- `<init>(Lsun/security/x509/Extension;Ljava/lang/Throwable;)V`

### `sun/security/x509/X400Address`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`

### `sun/security/x509/X500Name`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/util/Map;)V`
- `<init>(Lsun/security/util/DerInputStream;)V`
- `<init>([B)V`
- `asX500Principal()Ljavax/security/auth/x500/X500Principal;`
- `countQuotes(Ljava/lang/String;II)I`
- `equals(Ljava/lang/Object;)Z`
- `escaped(IILjava/lang/String;)Z`
- `generateDN()V`
- `generateRFC1779DN(Ljava/util/Map;)Ljava/lang/String;`
- `generateRFC2253DN(Ljava/util/Map;)Ljava/lang/String;`
- `getRFC1779Name()Ljava/lang/String;`
- `getRFC1779Name(Ljava/util/Map;)Ljava/lang/String;`
- `getRFC2253CanonicalName()Ljava/lang/String;`
- `getRFC2253Name()Ljava/lang/String;`
- `getRFC2253Name(Ljava/util/Map;)Ljava/lang/String;`
- `isEmpty()Z`
- `parseDER(Lsun/security/util/DerInputStream;)V`
- `parseDN(Ljava/lang/String;Ljava/util/Map;)V`
- `parseRFC2253DN(Ljava/lang/String;)V`
- `toString()Ljava/lang/String;`

### `sun/security/x509/X509CertImpl`

- `<init>()V`
- `getEncodedInternal()[B`
- `getEncodedInternal(Ljava/security/cert/Certificate;)[B`
- `getIssuerX500Principal(Ljava/security/cert/X509Certificate;)Ljavax/security/auth/x500/X500Principal;`
- `getSubjectX500Principal(Ljava/security/cert/X509Certificate;)Ljavax/security/auth/x500/X500Principal;`
- `getX500Principal(Ljava/security/cert/X509Certificate;Z)Ljavax/security/auth/x500/X500Principal;`
- `parse(Lsun/security/util/DerValue;)V`
- `readRFC1421Cert(Ljava/io/InputStream;)Lsun/security/util/DerValue;`

### `sun/security/x509/X509CertInfo`

- `<init>()V`
- `<init>(Lsun/security/util/DerValue;)V`
- `getAlgorithmId()Lsun/security/x509/CertificateAlgorithmId;`
- `getIssuer()Lsun/security/x509/X500Name;`
- `getSubject()Lsun/security/x509/X500Name;`
- `parse(Lsun/security/util/DerValue;)V`
- `verifyCert(Lsun/security/x509/X500Name;Lsun/security/x509/CertificateExtensions;)V`

### `sun/security/x509/X509Key`

- `<init>()V`
- `<init>(Lsun/security/x509/AlgorithmId;Lsun/security/util/BitArray;)V`
- `buildX509Key(Lsun/security/x509/AlgorithmId;Lsun/security/util/BitArray;)Ljava/security/PublicKey;`
- `encode()[B`
- `encode(Lsun/security/util/DerOutputStream;)V`
- `encode(Lsun/security/util/DerOutputStream;Lsun/security/x509/AlgorithmId;Lsun/security/util/BitArray;)V`
- `getEncodedInternal()[B`
- `getKey()Lsun/security/util/BitArray;`
- `parse(Lsun/security/util/DerValue;)Ljava/security/PublicKey;`
- `parseKeyBits()V`
- `setKey(Lsun/security/util/BitArray;)V`

### `sun/text/Normalizer`

- `getCombiningClass(I)I`

### `sun/util/calendar/AbstractCalendar`

- `<init>()V`

### `sun/util/calendar/BaseCalendar`

- `<init>()V`
- `getCalendarDate()Lsun/util/calendar/CalendarDate;`
- `getCalendarDate(JLjava/util/TimeZone;)Lsun/util/calendar/CalendarDate;`
- `getCalendarDate(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;`
- `getTime(Lsun/util/calendar/CalendarDate;)J`
- `newCalendarDate(Ljava/util/TimeZone;)Lsun/util/calendar/CalendarDate;`

### `sun/util/calendar/BaseCalendar$Date`

- `<init>()V`
- `<init>(Ljava/util/TimeZone;)V`
- `getDayOfMonth()I`
- `getEra()Lsun/util/calendar/Era;`
- `getHours()I`
- `getMillis()I`
- `getMinutes()I`
- `getMonth()I`
- `getNormalizedYear()I`
- `getSeconds()I`
- `getZone()Ljava/util/TimeZone;`
- `isNormalized()Z`
- `setDate(III)Lsun/util/calendar/CalendarDate;`
- `setMonth(I)Lsun/util/calendar/CalendarDate;`
- `setNormalizedDate(III)Lsun/util/calendar/BaseCalendar$Date;`
- `setNormalizedYear(I)V`
- `setTimeOfDay(IIII)Lsun/util/calendar/CalendarDate;`
- `setZone(Ljava/util/TimeZone;)Lsun/util/calendar/CalendarDate;`

### `sun/util/calendar/CalendarDate`

- `<init>()V`
- `<init>(Ljava/util/TimeZone;)V`
- `getYear()I`
- `setDate(III)Lsun/util/calendar/CalendarDate;`
- `setDayOfMonth(I)Lsun/util/calendar/CalendarDate;`
- `setHours(I)Lsun/util/calendar/CalendarDate;`
- `setMillis(I)Lsun/util/calendar/CalendarDate;`
- `setMinutes(I)Lsun/util/calendar/CalendarDate;`
- `setMonth(I)Lsun/util/calendar/CalendarDate;`
- `setSeconds(I)Lsun/util/calendar/CalendarDate;`
- `setTimeOfDay(IIII)Lsun/util/calendar/CalendarDate;`
- `setYear(I)Lsun/util/calendar/CalendarDate;`

### `sun/util/calendar/CalendarSystem`

- `<init>()V`
- `forName(Ljava/lang/String;)Lsun/util/calendar/CalendarSystem;`
- `getCalendarDate(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;`
- `getGregorianCalendar()Lsun/util/calendar/Gregorian;`
- `getTime(Lsun/util/calendar/CalendarDate;)J`
- `initNames()V`
- `newCalendarDate(Ljava/util/TimeZone;)Lsun/util/calendar/CalendarDate;`

### `sun/util/calendar/CalendarUtils`

- `floorDivide(II)I`
- `floorDivide(JJ)J`
- `mod(II)I`

### `sun/util/calendar/Era`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;JZ)V`
- `getName()Ljava/lang/String;`
- `getSince(Ljava/util/TimeZone;)J`

### `sun/util/calendar/Gregorian`

- `getCalendarDate()Lsun/util/calendar/Gregorian$Date;`
- `getCalendarDate(J)Lsun/util/calendar/Gregorian$Date;`
- `getCalendarDate(JLjava/util/TimeZone;)Lsun/util/calendar/Gregorian$Date;`
- `getCalendarDate(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/Gregorian$Date;`
- `newCalendarDate()Lsun/util/calendar/Gregorian$Date;`
- `newCalendarDate(Ljava/util/TimeZone;)Lsun/util/calendar/Gregorian$Date;`

### `sun/util/calendar/Gregorian$Date`

- `<init>()V`
- `<init>(Ljava/util/TimeZone;)V`

### `sun/util/calendar/ImmutableGregorianDate`

- `<init>()V`
- `<init>(Lsun/util/calendar/BaseCalendar$Date;)V`

### `sun/util/calendar/LocalGregorianCalendar`

- `<init>()V`
- `<init>(Ljava/lang/String;[Lsun/util/calendar/Era;)V`
- `convertUnicodeEscape(Ljava/lang/String;)Ljava/lang/String;`
- `getLocalGregorianCalendar(Ljava/lang/String;)Lsun/util/calendar/LocalGregorianCalendar;`
- `isValidEra(Lsun/util/calendar/Era;[Lsun/util/calendar/Era;)Z`
- `parseEraEntry(Ljava/lang/String;)Lsun/util/calendar/Era;`
- `setEras([Lsun/util/calendar/Era;)V`

### `sun/util/calendar/ZoneInfo`

- `<init>()V`
- `<init>(Ljava/lang/String;I)V`
- `<init>(Ljava/lang/String;III[J[I[IZ)V`
- `clone()Ljava/lang/Object;`
- `getTimeZone(Ljava/lang/String;)Ljava/util/TimeZone;`
- `setID(Ljava/lang/String;)V`

### `sun/util/calendar/ZoneInfoFile`

- `addTrans([JI[IIJII)I`
- `getCustomTimeZone(Ljava/lang/String;I)Lsun/util/calendar/ZoneInfo;`
- `getStandardOffset([J[IJ)I`
- `getYear(JI)I`
- `getZoneInfo(Ljava/io/DataInput;Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;`
- `getZoneInfo(Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;`
- `getZoneInfo(Ljava/lang/String;[J[I[J[I[Lsun/util/calendar/ZoneInfoFile$ZoneOffsetTransitionRule;)Lsun/util/calendar/ZoneInfo;`
- `getZoneInfo0(Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;`
- `indexOf([IIII)I`
- `readEpochSec(Ljava/io/DataInput;)J`
- `readOffset(Ljava/io/DataInput;)I`
- `toCustomID(I)Ljava/lang/String;`

### `sun/util/calendar/ZoneInfoFile$Checksum`

- `<init>()V`
- `getValue()J`
- `update(I)V`
- `update(J)V`
- `update([B)V`

### `sun/util/calendar/ZoneInfoFile$ZoneOffsetTransitionRule`

- `<init>()V`
- `<init>(Ljava/io/DataInput;)V`
- `adjust(JII)J`
- `getTransitionEpochSecond(I)J`
- `isLeapYear(I)Z`
- `lengthOfMonth(II)I`
- `nextOrSame(JI)J`
- `previousOrSame(JI)J`
- `toEpochDay(III)J`

### `sun/util/locale/BaseLocale`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V`
- `convertOldISOCodes(Ljava/lang/String;)Ljava/lang/String;`
- `equals(Ljava/lang/Object;)Z`
- `getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/BaseLocale;`
- `getLanguage()Ljava/lang/String;`
- `getRegion()Ljava/lang/String;`
- `getScript()Ljava/lang/String;`
- `getVariant()Ljava/lang/String;`
- `hashCode()I`

### `sun/util/locale/BaseLocale$Cache`

- `get(Ljava/lang/Object;)Ljava/lang/Object;`

### `sun/util/locale/BaseLocale$Key`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V`
- `hashCode(Lsun/util/locale/BaseLocale;)I`

### `sun/util/locale/Extension`

- `<init>()V`
- `<init>(C)V`
- `<init>(CLjava/lang/String;)V`
- `getValue()Ljava/lang/String;`
- `setValue(Ljava/lang/String;)V`

### `sun/util/locale/InternalLocaleBuilder`

- `<init>()V`
- `checkVariants(Ljava/lang/String;Ljava/lang/String;)I`
- `clear()Lsun/util/locale/InternalLocaleBuilder;`
- `clearExtensions()Lsun/util/locale/InternalLocaleBuilder;`
- `getBaseLocale()Lsun/util/locale/BaseLocale;`
- `getLocaleExtensions()Lsun/util/locale/LocaleExtensions;`
- `removePrivateuseVariant(Ljava/lang/String;)Ljava/lang/String;`
- `setExtensions(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;`
- `setExtensions(Ljava/util/List;Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;`
- `setLocale(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Lsun/util/locale/InternalLocaleBuilder;`
- `setRegion(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;`
- `setUnicodeLocaleExtension(Ljava/lang/String;)V`

### `sun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar`

- `<init>()V`
- `<init>(C)V`
- `<init>(Ljava/lang/String;)V`
- `value()C`

### `sun/util/locale/InternalLocaleBuilder$CaseInsensitiveString`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `value()Ljava/lang/String;`

### `sun/util/locale/LanguageTag`

- `isExtensionSingleton(Ljava/lang/String;)Z`
- `isExtensionSubtag(Ljava/lang/String;)Z`
- `isLanguage(Ljava/lang/String;)Z`
- `isPrivateusePrefix(Ljava/lang/String;)Z`
- `isPrivateusePrefixChar(C)Z`
- `isPrivateuseSubtag(Ljava/lang/String;)Z`
- `isRegion(Ljava/lang/String;)Z`
- `isScript(Ljava/lang/String;)Z`
- `isVariant(Ljava/lang/String;)Z`

### `sun/util/locale/LocaleExtensions`

- `<init>()V`
- `<init>(Ljava/util/Map;Ljava/util/Set;Ljava/util/Map;)V`
- `equals(Ljava/lang/Object;)Z`
- `getExtension(Ljava/lang/Character;)Lsun/util/locale/Extension;`
- `getKeys()Ljava/util/Set;`
- `getUnicodeLocaleType(Ljava/lang/String;)Ljava/lang/String;`
- `hashCode()I`
- `isEmpty()Z`
- `toID(Ljava/util/SortedMap;)Ljava/lang/String;`

### `sun/util/locale/LocaleSyntaxException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;I)V`
- `getErrorIndex()I`
- `getMessage()Ljava/lang/String;`

### `sun/util/locale/LocaleUtils`

- `caseIgnoreMatch(Ljava/lang/String;Ljava/lang/String;)Z`
- `isAlpha(C)Z`
- `isAlphaNumeric(C)Z`
- `isAlphaNumericString(Ljava/lang/String;)Z`
- `isAlphaString(Ljava/lang/String;)Z`
- `isEmpty(Ljava/lang/String;)Z`
- `isEmpty(Ljava/util/List;)Z`
- `isEmpty(Ljava/util/Map;)Z`
- `isEmpty(Ljava/util/Set;)Z`
- `isLower(C)Z`
- `isNumeric(C)Z`
- `isNumericString(Ljava/lang/String;)Z`
- `isUpper(C)Z`
- `toLower(C)C`
- `toLowerString(Ljava/lang/String;)Ljava/lang/String;`
- `toTitleString(Ljava/lang/String;)Ljava/lang/String;`
- `toUpper(C)C`
- `toUpperString(Ljava/lang/String;)Ljava/lang/String;`

### `sun/util/locale/StringTokenIterator`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `current()Ljava/lang/String;`
- `currentEnd()I`
- `currentStart()I`
- `hasNext()Z`
- `isDone()Z`
- `next()Ljava/lang/String;`
- `nextDelimiter(I)I`
- `setStart(I)Lsun/util/locale/StringTokenIterator;`

### `sun/util/locale/UnicodeLocaleExtension`

- `<init>()V`
- `<init>(Ljava/util/SortedSet;Ljava/util/SortedMap;)V`
- `getUnicodeLocaleAttributes()Ljava/util/Set;`
- `getUnicodeLocaleKeys()Ljava/util/Set;`
- `getUnicodeLocaleType(Ljava/lang/String;)Ljava/lang/String;`
- `isAttribute(Ljava/lang/String;)Z`
- `isKey(Ljava/lang/String;)Z`
- `isSingletonChar(C)Z`
- `setValue(Ljava/lang/String;)V`

### `sun/util/locale/provider/CalendarDataUtility`

- `findRegionOverride(Ljava/util/Locale;)Ljava/util/Locale;`
- `retrieveFirstDayOfWeek(Ljava/util/Locale;)I`
- `retrieveMinimalDaysInFirstWeek(Ljava/util/Locale;)I`

### `sun/util/locale/provider/LocaleProviderAdapter`

- `<init>()V`
- `findAdapter(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;`
- `forJRE()Lsun/util/locale/provider/LocaleProviderAdapter;`
- `forType(Lsun/util/locale/provider/LocaleProviderAdapter$Type;)Lsun/util/locale/provider/LocaleProviderAdapter;`
- `getAdapter(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;`
- `getAdapterPreference()Ljava/util/List;`
- `getBreakIteratorProvider()Ljava/text/spi/BreakIteratorProvider;`
- `getLocaleServiceProvider(Ljava/lang/Class;)Ljava/util/spi/LocaleServiceProvider;`

### `sun/util/locale/provider/LocaleProviderAdapter$Type`

- `<init>()V`
- `<init>(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `getAdapterClassName()Ljava/lang/String;`
- `ordinal()I`

### `sun/util/locale/provider/LocaleServiceProviderPool`

- `<init>()V`
- `<init>(Ljava/lang/Class;)V`
- `findProviders(Ljava/util/Locale;Z)Ljava/util/List;`
- `getLocalizedObject(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;Ljava/lang/Boolean;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;`
- `getLocalizedObjectImpl(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;ZLjava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;`
- `getLookupLocales(Ljava/util/Locale;)Ljava/util/List;`
- `getPool(Ljava/lang/Class;)Lsun/util/locale/provider/LocaleServiceProviderPool;`

### `sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter`

- `getObject(Ljava/util/spi/LocaleServiceProvider;Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;`

## 仅类级 BFS 拉入（方法级不需要）

| 类名 | 方法数 |
|------|-------:|
| `java/io/BufferedOutputStream` | 12 |
| `java/io/BufferedReader$1` | 4 |
| `java/io/ClassCache$1` | 3 |
| `java/io/Closeable` | 1 |
| `java/io/DataOutput` | 14 |
| `java/io/DataOutputStream` | 19 |
| `java/io/DeleteOnExitHook` | 4 |
| `java/io/File$TempDirectory` | 5 |
| `java/io/FileFilter` | 1 |
| `java/io/FileInputStream$1` | 2 |
| `java/io/FileOutputStream$1` | 2 |
| `java/io/FilePermissionCollection` | 8 |
| `java/io/FileReader` | 5 |
| `java/io/FileWriter` | 9 |
| `java/io/FilenameFilter` | 1 |
| `java/io/Flushable` | 1 |
| `java/io/InputStream$1` | 12 |
| `java/io/InterruptedIOException` | 2 |
| `java/io/NotSerializableException` | 2 |
| `java/io/ObjectInput` | 7 |
| `java/io/ObjectInputFilter$Config` | 13 |
| `java/io/ObjectInputFilter$Config$Global` | 18 |
| `java/io/ObjectInputFilter$Config$MergeFilter` | 3 |
| `java/io/ObjectInputFilter$Config$PredicateFilter` | 3 |
| `java/io/ObjectInputFilter$Config$RejectUndecidedFilter` | 3 |
| `java/io/ObjectInputFilter$Config$RejectUndecidedFilter$SerialInfo` | 6 |
| `java/io/ObjectInputFilter$FilterInfo` | 5 |
| `java/io/ObjectInputStream$1` | 3 |
| `java/io/ObjectInputStream$ValidationList$Callback` | 1 |
| `java/io/ObjectInputValidation` | 1 |
| `java/io/ObjectOutput` | 6 |
| `java/io/ObjectOutputStream` | 56 |
| `java/io/ObjectOutputStream$1` | 3 |
| `java/io/ObjectOutputStream$BlockDataOutputStream` | 34 |
| `java/io/ObjectOutputStream$DebugTraceInfoStack` | 5 |
| `java/io/ObjectOutputStream$HandleTable` | 9 |
| `java/io/ObjectOutputStream$PutField` | 11 |
| `java/io/ObjectOutputStream$PutFieldImpl` | 13 |
| `java/io/ObjectOutputStream$ReplaceTable` | 6 |
| `java/io/ObjectStreamClass$3` | 3 |
| `java/io/ObjectStreamClass$4` | 3 |
| `java/io/ObjectStreamClass$5` | 3 |
| `java/io/ObjectStreamClass$MemberSignature` | 3 |
| `java/io/OutputStream$1` | 5 |
| `java/io/PrintWriter` | 67 |
| `java/io/ProxyingConsole` | 11 |
| `java/io/ProxyingConsole$WrappingReader` | 3 |
| `java/io/ProxyingConsole$WrappingWriter` | 4 |
| `java/io/RandomAccessFile` | 55 |
| `java/io/RandomAccessFile$1` | 2 |
| `java/io/Reader$1` | 9 |
| `java/io/Writer$1` | 14 |
| `java/lang/ApplicationShutdownHooks` | 5 |
| `java/lang/ArrayStoreException` | 2 |
| `java/lang/BootstrapMethodError` | 4 |
| `java/lang/CharSequence$1CharIterator` | 5 |
| `java/lang/CharSequence$1CodePointIterator` | 5 |
| `java/lang/Class$2` | 3 |
| `java/lang/Class$AnnotationData` | 1 |
| `java/lang/ClassLoader$1` | 3 |
| `java/lang/ClassValue$Identity` | 1 |
| `java/lang/CloneNotSupportedException` | 2 |
| `java/lang/Cloneable` | 0 |
| `java/lang/CompoundEnumeration` | 4 |
| `java/lang/Enum$EnumDesc` | 5 |
| `java/lang/EnumConstantNotPresentException` | 3 |
| `java/lang/ExceptionInInitializerError` | 7 |
| `java/lang/FdLibm` | 6 |
| `java/lang/FdLibm$Acos` | 2 |
| `java/lang/FdLibm$Asin` | 2 |
| `java/lang/FdLibm$Atan` | 3 |
| `java/lang/FdLibm$Atan2` | 2 |
| `java/lang/FdLibm$Cbrt` | 2 |
| `java/lang/FdLibm$Cos` | 3 |
| `java/lang/FdLibm$Cosh` | 2 |
| `java/lang/FdLibm$Exp` | 3 |
| `java/lang/FdLibm$Expm1` | 2 |
| `java/lang/FdLibm$Hypot` | 3 |
| `java/lang/FdLibm$IEEEremainder` | 5 |
| `java/lang/FdLibm$KernelRemPio2` | 3 |
| `java/lang/FdLibm$Log` | 2 |
| `java/lang/FdLibm$Log10` | 2 |
| `java/lang/FdLibm$Log1p` | 2 |
| `java/lang/FdLibm$Pow` | 2 |
| `java/lang/FdLibm$RemPio2` | 3 |
| `java/lang/FdLibm$Sin` | 3 |
| `java/lang/FdLibm$Sinh` | 2 |
| `java/lang/FdLibm$Sqrt` | 2 |
| `java/lang/FdLibm$Tan` | 4 |
| `java/lang/FdLibm$Tanh` | 2 |
| `java/lang/IllegalMonitorStateException` | 2 |
| `java/lang/IllegalThreadStateException` | 2 |
| `java/lang/InterruptedException` | 2 |
| `java/lang/Iterable` | 3 |
| `java/lang/LayerInstantiationException` | 4 |
| `java/lang/LiveStackFrame` | 8 |
| `java/lang/LiveStackFrame$PrimitiveSlot` | 4 |
| `java/lang/LiveStackFrameInfo` | 8 |
| `java/lang/LiveStackFrameInfo$PrimitiveSlot32` | 4 |
| `java/lang/LiveStackFrameInfo$PrimitiveSlot64` | 4 |
| `java/lang/MatchException` | 1 |
| `java/lang/Module$1` | 3 |
| `java/lang/ModuleLayer$Controller` | 7 |
| `java/lang/NamedPackage` | 5 |
| `java/lang/NoClassDefFoundError` | 2 |
| `java/lang/Package` | 25 |
| `java/lang/Package$VersionInfo` | 3 |
| `java/lang/PinnedThreadPrinter` | 13 |
| `java/lang/PinnedThreadPrinter$Hashes` | 3 |
| `java/lang/Process` | 25 |
| `java/lang/Process$1` | 3 |
| `java/lang/Process$CharsetHolder` | 3 |
| `java/lang/Process$PipeInputStream` | 2 |
| `java/lang/ProcessBuilder` | 27 |
| `java/lang/ProcessBuilder$Redirect` | 10 |
| `java/lang/ProcessBuilder$Redirect$4` | 4 |
| `java/lang/ProcessBuilder$Redirect$5` | 5 |
| `java/lang/ProcessBuilder$Redirect$6` | 5 |
| `java/lang/ProcessBuilder$Redirect$Type` | 5 |
| `java/lang/ProcessBuilder$RedirectPipeImpl` | 4 |
| `java/lang/ProcessEnvironment` | 10 |
| `java/lang/ProcessEnvironment$ExternalData` | 5 |
| `java/lang/ProcessEnvironment$StringEntry` | 10 |
| `java/lang/ProcessEnvironment$StringEntrySet` | 10 |
| `java/lang/ProcessEnvironment$StringEntrySet$1` | 5 |
| `java/lang/ProcessEnvironment$StringEntrySet$2` | 7 |
| `java/lang/ProcessEnvironment$StringEnvironment` | 17 |
| `java/lang/ProcessEnvironment$StringKeySet` | 7 |
| `java/lang/ProcessEnvironment$StringKeySet$1` | 5 |
| `java/lang/ProcessEnvironment$StringValues` | 9 |
| `java/lang/ProcessEnvironment$StringValues$1` | 5 |
| `java/lang/ProcessEnvironment$Value` | 8 |
| `java/lang/ProcessEnvironment$Variable` | 8 |
| `java/lang/ProcessHandle` | 17 |
| `java/lang/ProcessHandleImpl` | 41 |
| `java/lang/ProcessHandleImpl$1` | 2 |
| `java/lang/ProcessHandleImpl$ExitCompletion` | 1 |
| `java/lang/ProcessHandleImpl$Info` | 12 |
| `java/lang/ProcessImpl` | 28 |
| `java/lang/ProcessImpl$DeferredCloseProcessPipeInputStream` | 11 |
| `java/lang/ProcessImpl$LaunchMechanism` | 5 |
| `java/lang/ProcessImpl$ProcessPipeInputStream` | 4 |
| `java/lang/ProcessImpl$ProcessPipeOutputStream` | 2 |
| `java/lang/PublicMethods` | 3 |
| `java/lang/Readable` | 1 |
| `java/lang/Runnable` | 1 |
| `java/lang/Runtime` | 24 |
| `java/lang/Runtime$Version` | 27 |
| `java/lang/SecurityManager$2` | 3 |
| `java/lang/Shutdown` | 10 |
| `java/lang/StackFrameInfo` | 16 |
| `java/lang/StackStreamFactory$CallerClassFinder` | 8 |
| `java/lang/StackStreamFactory$CallerClassFinder$ClassBuffer` | 7 |
| `java/lang/StackStreamFactory$FrameBuffer` | 17 |
| `java/lang/StackStreamFactory$LiveStackInfoTraverser$LiveStackFrameBuffer` | 9 |
| `java/lang/StackStreamFactory$StackFrameTraverser$StackFrameBuffer` | 9 |
| `java/lang/StackStreamFactory$WalkerState` | 5 |
| `java/lang/StackWalker$StackFrame` | 10 |
| `java/lang/StringLatin1$CharsSpliterator` | 11 |
| `java/lang/StringLatin1$LinesSpliterator` | 10 |
| `java/lang/StringUTF16$CharsSpliterator` | 11 |
| `java/lang/StringUTF16$CodePointsSpliterator` | 12 |
| `java/lang/StringUTF16$LinesSpliterator` | 10 |
| `java/lang/System$1` | 2 |
| `java/lang/System$2` | 87 |
| `java/lang/Terminator` | 4 |
| `java/lang/Terminator$1` | 2 |
| `java/lang/Thread$1` | 3 |
| `java/lang/Thread$Builder$OfVirtual` | 8 |
| `java/lang/Thread$State` | 5 |
| `java/lang/Thread$UncaughtExceptionHandler` | 1 |
| `java/lang/ThreadBuilders` | 2 |
| `java/lang/ThreadBuilders$BaseThreadBuilder` | 10 |
| `java/lang/ThreadBuilders$BaseThreadFactory` | 5 |
| `java/lang/ThreadBuilders$BoundVirtualThread` | 7 |
| `java/lang/ThreadBuilders$PlatformThreadBuilder` | 17 |
| `java/lang/ThreadBuilders$PlatformThreadFactory` | 3 |
| `java/lang/ThreadBuilders$VirtualThreadBuilder` | 13 |
| `java/lang/ThreadBuilders$VirtualThreadFactory` | 2 |
| `java/lang/ThreadLocal$SuppliedThreadLocal` | 2 |
| `java/lang/UnsatisfiedLinkError` | 2 |
| `java/lang/VersionProps` | 11 |
| `java/lang/WrongThreadException` | 4 |
| `java/lang/annotation/Annotation` | 4 |
| `java/lang/annotation/AnnotationFormatError` | 3 |
| `java/lang/annotation/AnnotationTypeMismatchException` | 3 |
| `java/lang/annotation/IncompleteAnnotationException` | 3 |
| `java/lang/annotation/Repeatable` | 1 |
| `java/lang/annotation/Retention` | 1 |
| `java/lang/constant/AsTypeMethodHandleDesc` | 5 |
| `java/lang/constant/ClassDesc` | 20 |
| `java/lang/constant/ConstantDesc` | 1 |
| `java/lang/constant/ConstantUtils` | 16 |
| `java/lang/constant/DirectMethodHandleDesc` | 6 |
| `java/lang/constant/DirectMethodHandleDesc$Kind` | 10 |
| `java/lang/constant/DirectMethodHandleDescImpl` | 15 |
| `java/lang/constant/DynamicConstantDesc` | 21 |
| `java/lang/constant/DynamicConstantDesc$AnonymousDynamicConstantDesc` | 1 |
| `java/lang/constant/MethodHandleDesc` | 9 |
| `java/lang/constant/MethodTypeDesc` | 25 |
| `java/lang/constant/MethodTypeDescImpl` | 25 |
| `java/lang/constant/MethodTypeDescImpl$1` | 3 |
| `java/lang/constant/PrimitiveClassDescImpl` | 5 |
| `java/lang/constant/ReferenceClassDescImpl` | 8 |
| `java/lang/invoke/AbstractConstantGroup` | 6 |
| `java/lang/invoke/AbstractConstantGroup$BSCIWithCache` | 6 |
| `java/lang/invoke/AbstractConstantGroup$WithCache` | 9 |
| `java/lang/invoke/BootstrapMethodInvoker` | 12 |
| `java/lang/invoke/BootstrapMethodInvoker$VM_BSCI` | 6 |
| `java/lang/invoke/BoundMethodHandle$Specializer$Factory` | 2 |
| `java/lang/invoke/CallSite` | 20 |
| `java/lang/invoke/ClassSpecializer` | 22 |
| `java/lang/invoke/ClassSpecializer$Factory` | 18 |
| `java/lang/invoke/ClassSpecializer$Factory$1Var` | 11 |
| `java/lang/invoke/ConstantBootstraps` | 13 |
| `java/lang/invoke/ConstantCallSite` | 6 |
| `java/lang/invoke/DirectMethodHandle$1` | 3 |
| `java/lang/invoke/IndirectVarHandle` | 10 |
| `java/lang/invoke/InfoFromMemberName` | 11 |
| `java/lang/invoke/InfoFromMemberName$1` | 3 |
| `java/lang/invoke/MethodHandle$1` | 3 |
| `java/lang/invoke/MethodHandleImpl$CasesHolder` | 1 |
| `java/lang/invoke/MethodHandleImpl$CountingWrapper` | 6 |
| `java/lang/invoke/MethodHandleImpl$CountingWrapper$1` | 3 |
| `java/lang/invoke/MethodHandleImpl$LoopClauses` | 3 |
| `java/lang/invoke/MethodHandleImpl$TableSwitchCacheKey` | 4 |
| `java/lang/invoke/MethodHandleInfo` | 9 |
| `java/lang/invoke/NativeMethodHandle` | 10 |
| `java/lang/invoke/SerializedLambda` | 14 |
| `java/lang/invoke/SerializedLambda$1` | 3 |
| `java/lang/invoke/VarForm` | 9 |
| `java/lang/invoke/VarHandle` | 59 |
| `java/lang/invoke/VarHandle$VarHandleDesc` | 8 |
| `java/lang/invoke/VarHandle$VarHandleDesc$Kind` | 6 |
| `java/lang/invoke/VarHandleBooleans$Array` | 37 |
| `java/lang/invoke/VarHandleBooleans$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleBooleans$FieldInstanceReadWrite` | 33 |
| `java/lang/invoke/VarHandleBooleans$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleBooleans$FieldStaticReadWrite` | 33 |
| `java/lang/invoke/VarHandleByteArrayAsChars` | 3 |
| `java/lang/invoke/VarHandleByteArrayAsChars$ArrayHandle` | 18 |
| `java/lang/invoke/VarHandleByteArrayAsChars$ByteArrayViewVarHandle` | 1 |
| `java/lang/invoke/VarHandleByteArrayAsChars$ByteBufferHandle` | 20 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles` | 4 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles$ArrayHandle` | 29 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles$ByteArrayViewVarHandle` | 1 |
| `java/lang/invoke/VarHandleByteArrayAsDoubles$ByteBufferHandle` | 31 |
| `java/lang/invoke/VarHandleByteArrayAsFloats` | 4 |
| `java/lang/invoke/VarHandleByteArrayAsFloats$ArrayHandle` | 29 |
| `java/lang/invoke/VarHandleByteArrayAsFloats$ByteArrayViewVarHandle` | 1 |
| `java/lang/invoke/VarHandleByteArrayAsFloats$ByteBufferHandle` | 31 |
| `java/lang/invoke/VarHandleByteArrayAsInts` | 3 |
| `java/lang/invoke/VarHandleByteArrayAsInts$ArrayHandle` | 45 |
| `java/lang/invoke/VarHandleByteArrayAsInts$ByteArrayViewVarHandle` | 1 |
| `java/lang/invoke/VarHandleByteArrayAsInts$ByteBufferHandle` | 47 |
| `java/lang/invoke/VarHandleByteArrayAsLongs` | 3 |
| `java/lang/invoke/VarHandleByteArrayAsLongs$ArrayHandle` | 45 |
| `java/lang/invoke/VarHandleByteArrayAsLongs$ByteArrayViewVarHandle` | 1 |
| `java/lang/invoke/VarHandleByteArrayAsLongs$ByteBufferHandle` | 47 |
| `java/lang/invoke/VarHandleByteArrayAsShorts` | 3 |
| `java/lang/invoke/VarHandleByteArrayAsShorts$ArrayHandle` | 18 |
| `java/lang/invoke/VarHandleByteArrayAsShorts$ByteArrayViewVarHandle` | 1 |
| `java/lang/invoke/VarHandleByteArrayAsShorts$ByteBufferHandle` | 20 |
| `java/lang/invoke/VarHandleByteArrayBase` | 3 |
| `java/lang/invoke/VarHandleBytes$Array` | 40 |
| `java/lang/invoke/VarHandleBytes$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleBytes$FieldInstanceReadWrite` | 36 |
| `java/lang/invoke/VarHandleBytes$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleBytes$FieldStaticReadWrite` | 36 |
| `java/lang/invoke/VarHandleChars$Array` | 40 |
| `java/lang/invoke/VarHandleChars$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleChars$FieldInstanceReadWrite` | 36 |
| `java/lang/invoke/VarHandleChars$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleChars$FieldStaticReadWrite` | 36 |
| `java/lang/invoke/VarHandleDoubles$Array` | 31 |
| `java/lang/invoke/VarHandleDoubles$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleDoubles$FieldInstanceReadWrite` | 27 |
| `java/lang/invoke/VarHandleDoubles$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleDoubles$FieldStaticReadWrite` | 27 |
| `java/lang/invoke/VarHandleFloats$Array` | 31 |
| `java/lang/invoke/VarHandleFloats$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleFloats$FieldInstanceReadWrite` | 27 |
| `java/lang/invoke/VarHandleFloats$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleFloats$FieldStaticReadWrite` | 27 |
| `java/lang/invoke/VarHandleInts$Array` | 40 |
| `java/lang/invoke/VarHandleInts$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleInts$FieldInstanceReadWrite` | 36 |
| `java/lang/invoke/VarHandleInts$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleInts$FieldStaticReadWrite` | 36 |
| `java/lang/invoke/VarHandleLongs$Array` | 40 |
| `java/lang/invoke/VarHandleLongs$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleLongs$FieldInstanceReadWrite` | 36 |
| `java/lang/invoke/VarHandleLongs$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleLongs$FieldStaticReadWrite` | 36 |
| `java/lang/invoke/VarHandleReferences$Array` | 30 |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite` | 24 |
| `java/lang/invoke/VarHandleReferences$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleReferences$FieldStaticReadWrite` | 24 |
| `java/lang/invoke/VarHandleSegmentAsBytes` | 19 |
| `java/lang/invoke/VarHandleSegmentAsChars` | 19 |
| `java/lang/invoke/VarHandleSegmentAsDoubles` | 31 |
| `java/lang/invoke/VarHandleSegmentAsFloats` | 31 |
| `java/lang/invoke/VarHandleSegmentAsInts` | 46 |
| `java/lang/invoke/VarHandleSegmentAsLongs` | 46 |
| `java/lang/invoke/VarHandleSegmentAsShorts` | 19 |
| `java/lang/invoke/VarHandleSegmentViewBase` | 2 |
| `java/lang/invoke/VarHandleShorts$Array` | 40 |
| `java/lang/invoke/VarHandleShorts$FieldInstanceReadOnly` | 13 |
| `java/lang/invoke/VarHandleShorts$FieldInstanceReadWrite` | 36 |
| `java/lang/invoke/VarHandleShorts$FieldStaticReadOnly` | 13 |
| `java/lang/invoke/VarHandleShorts$FieldStaticReadWrite` | 36 |
| `java/lang/invoke/VarHandles` | 30 |
| `java/lang/module/ModuleDescriptor$Builder` | 27 |
| `java/lang/module/ModuleDescriptor$Exports$Modifier` | 6 |
| `java/lang/module/ModuleDescriptor$Modifier` | 6 |
| `java/lang/module/ModuleDescriptor$Opens$Modifier` | 6 |
| `java/lang/module/ModuleDescriptor$Requires$Modifier` | 6 |
| `java/lang/module/ModuleFinder$1` | 3 |
| `java/lang/module/ModuleFinder$2` | 7 |
| `java/lang/ref/Reference$ReferenceHandler` | 2 |
| `java/lang/reflect/AccessFlag` | 10 |
| `java/lang/reflect/AccessibleObject$Cache` | 3 |
| `java/lang/reflect/AnnotatedArrayType` | 2 |
| `java/lang/reflect/AnnotatedElement` | 8 |
| `java/lang/reflect/AnnotatedParameterizedType` | 2 |
| `java/lang/reflect/AnnotatedType` | 5 |
| `java/lang/reflect/AnnotatedTypeVariable` | 2 |
| `java/lang/reflect/AnnotatedWildcardType` | 3 |
| `java/lang/reflect/Executable$ParameterData` | 6 |
| `java/lang/reflect/GenericArrayType` | 1 |
| `java/lang/reflect/GenericDeclaration` | 1 |
| `java/lang/reflect/GenericSignatureFormatError` | 2 |
| `java/lang/reflect/InaccessibleObjectException` | 2 |
| `java/lang/reflect/InvocationHandler` | 2 |
| `java/lang/reflect/MalformedParameterizedTypeException` | 2 |
| `java/lang/reflect/MalformedParametersException` | 2 |
| `java/lang/reflect/Member` | 5 |
| `java/lang/reflect/Parameter` | 23 |
| `java/lang/reflect/Proxy$2` | 3 |
| `java/lang/reflect/Proxy$InvocationException` | 3 |
| `java/lang/reflect/Proxy$ProxyBuilder$1` | 3 |
| `java/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext` | 7 |
| `java/lang/reflect/ProxyGenerator` | 18 |
| `java/lang/reflect/ProxyGenerator$1` | 3 |
| `java/lang/reflect/ProxyGenerator$PrimitiveTypeInfo` | 6 |
| `java/lang/reflect/ProxyGenerator$ProxyMethod` | 9 |
| `java/lang/reflect/ReflectPermission` | 2 |
| `java/lang/reflect/TypeVariable` | 4 |
| `java/lang/reflect/WildcardType` | 2 |
| `java/math/BigDecimal$LongOverflow` | 3 |
| `java/math/BigDecimal$StringBuilderHelper` | 5 |
| `java/math/BigDecimal$UnsafeHolder` | 4 |
| `java/math/BigInteger$UnsafeHolder` | 3 |
| `java/math/MathContext` | 10 |
| `java/net/Authenticator` | 18 |
| `java/net/BindException` | 2 |
| `java/net/ContentHandler` | 3 |
| `java/net/ContentHandlerFactory` | 1 |
| `java/net/DelegatingSocketImpl` | 27 |
| `java/net/FileNameMap` | 1 |
| `java/net/HostPortrange` | 13 |
| `java/net/HttpConnectSocketImpl` | 14 |
| `java/net/HttpConnectSocketImpl$2` | 3 |
| `java/net/HttpURLConnection` | 21 |
| `java/net/IDN` | 16 |
| `java/net/Inet4AddressImpl` | 9 |
| `java/net/InetAddress$CachedLocalHost` | 1 |
| `java/net/InetAddress$HostsFileResolver` | 8 |
| `java/net/InetAddress$PlatformResolver` | 3 |
| `java/net/InetAddress$ValidCachedLookup` | 3 |
| `java/net/InterfaceAddress` | 7 |
| `java/net/JarURLConnection` | 10 |
| `java/net/PasswordAuthentication` | 3 |
| `java/net/ProtocolException` | 2 |
| `java/net/ProtocolFamily` | 1 |
| `java/net/Proxy$Type` | 5 |
| `java/net/ProxySelector` | 7 |
| `java/net/ProxySelector$StaticProxySelector` | 4 |
| `java/net/Socket` | 72 |
| `java/net/Socket$SocketInputStream` | 5 |
| `java/net/Socket$SocketOutputStream` | 4 |
| `java/net/SocketImpl` | 29 |
| `java/net/SocketImplFactory` | 1 |
| `java/net/SocketOption` | 2 |
| `java/net/SocketPermission$1` | 3 |
| `java/net/SocketPermissionCollection` | 8 |
| `java/net/SocketTimeoutException` | 2 |
| `java/net/SocksSocketImpl` | 20 |
| `java/net/SocksSocketImpl$1` | 3 |
| `java/net/SocksSocketImpl$2` | 3 |
| `java/net/SocksSocketImpl$3` | 3 |
| `java/net/URL$1` | 5 |
| `java/net/URLConnection$1` | 2 |
| `java/net/URLConnection$2` | 3 |
| `java/net/URLPermission` | 13 |
| `java/net/URLPermission$Authority` | 6 |
| `java/net/UnixDomainSocketAddress` | 10 |
| `java/net/UnixDomainSocketAddress$Ser` | 2 |
| `java/net/UrlDeserializedState` | 9 |
| `java/net/spi/InetAddressResolver$LookupPolicy` | 3 |
| `java/net/spi/InetAddressResolverProvider` | 6 |
| `java/net/spi/URLStreamHandlerProvider` | 3 |
| `java/nio/Bits` | 12 |
| `java/nio/BufferMismatch` | 9 |
| `java/nio/ByteBufferAsCharBufferB` | 26 |
| `java/nio/ByteBufferAsCharBufferL` | 26 |
| `java/nio/ByteBufferAsCharBufferRB` | 21 |
| `java/nio/ByteBufferAsCharBufferRL` | 21 |
| `java/nio/ByteBufferAsDoubleBufferB` | 21 |
| `java/nio/ByteBufferAsDoubleBufferL` | 21 |
| `java/nio/ByteBufferAsDoubleBufferRB` | 16 |
| `java/nio/ByteBufferAsDoubleBufferRL` | 16 |
| `java/nio/ByteBufferAsFloatBufferB` | 21 |
| `java/nio/ByteBufferAsFloatBufferL` | 21 |
| `java/nio/ByteBufferAsFloatBufferRB` | 16 |
| `java/nio/ByteBufferAsFloatBufferRL` | 16 |
| `java/nio/ByteBufferAsIntBufferB` | 21 |
| `java/nio/ByteBufferAsIntBufferL` | 21 |
| `java/nio/ByteBufferAsIntBufferRB` | 16 |
| `java/nio/ByteBufferAsIntBufferRL` | 16 |
| `java/nio/ByteBufferAsLongBufferB` | 21 |
| `java/nio/ByteBufferAsLongBufferL` | 21 |
| `java/nio/ByteBufferAsLongBufferRB` | 16 |
| `java/nio/ByteBufferAsLongBufferRL` | 16 |
| `java/nio/ByteBufferAsShortBufferB` | 21 |
| `java/nio/ByteBufferAsShortBufferL` | 21 |
| `java/nio/ByteBufferAsShortBufferRB` | 16 |
| `java/nio/ByteBufferAsShortBufferRL` | 16 |
| `java/nio/CharBufferSpliterator` | 12 |
| `java/nio/DirectByteBuffer` | 73 |
| `java/nio/DirectByteBuffer$Deallocator` | 9 |
| `java/nio/DirectByteBufferR` | 45 |
| `java/nio/DirectCharBufferRS` | 24 |
| `java/nio/DirectCharBufferRU` | 24 |
| `java/nio/DirectCharBufferS` | 32 |
| `java/nio/DirectCharBufferU` | 32 |
| `java/nio/DirectDoubleBufferRS` | 16 |
| `java/nio/DirectDoubleBufferRU` | 16 |
| `java/nio/DirectDoubleBufferS` | 22 |
| `java/nio/DirectDoubleBufferU` | 22 |
| `java/nio/DirectFloatBufferRS` | 16 |
| `java/nio/DirectFloatBufferRU` | 16 |
| `java/nio/DirectFloatBufferS` | 22 |
| `java/nio/DirectFloatBufferU` | 22 |
| `java/nio/DirectIntBufferRS` | 16 |
| `java/nio/DirectIntBufferRU` | 16 |
| `java/nio/DirectIntBufferS` | 22 |
| `java/nio/DirectIntBufferU` | 22 |
| `java/nio/DirectLongBufferRS` | 16 |
| `java/nio/DirectLongBufferRU` | 16 |
| `java/nio/DirectLongBufferS` | 22 |
| `java/nio/DirectLongBufferU` | 22 |
| `java/nio/DirectShortBufferRS` | 16 |
| `java/nio/DirectShortBufferRU` | 16 |
| `java/nio/DirectShortBufferS` | 22 |
| `java/nio/DirectShortBufferU` | 22 |
| `java/nio/DoubleBuffer` | 60 |
| `java/nio/FloatBuffer` | 60 |
| `java/nio/HeapByteBufferR` | 38 |
| `java/nio/HeapCharBufferR` | 29 |
| `java/nio/HeapDoubleBuffer` | 26 |
| `java/nio/HeapDoubleBufferR` | 19 |
| `java/nio/HeapFloatBuffer` | 26 |
| `java/nio/HeapFloatBufferR` | 19 |
| `java/nio/HeapIntBuffer` | 26 |
| `java/nio/HeapIntBufferR` | 19 |
| `java/nio/HeapLongBuffer` | 26 |
| `java/nio/HeapLongBufferR` | 19 |
| `java/nio/HeapShortBuffer` | 26 |
| `java/nio/HeapShortBufferR` | 19 |
| `java/nio/IntBuffer` | 60 |
| `java/nio/InvalidMarkException` | 1 |
| `java/nio/LongBuffer` | 60 |
| `java/nio/MappedByteBuffer` | 43 |
| `java/nio/MappedByteBuffer$1` | 5 |
| `java/nio/ShortBuffer` | 60 |
| `java/nio/channels/AlreadyBoundException` | 1 |
| `java/nio/channels/AlreadyConnectedException` | 1 |
| `java/nio/channels/AsynchronousByteChannel` | 4 |
| `java/nio/channels/AsynchronousChannel` | 1 |
| `java/nio/channels/AsynchronousChannelGroup` | 10 |
| `java/nio/channels/AsynchronousCloseException` | 1 |
| `java/nio/channels/AsynchronousFileChannel` | 17 |
| `java/nio/channels/CancelledKeyException` | 1 |
| `java/nio/channels/Channel` | 2 |
| `java/nio/channels/Channels` | 13 |
| `java/nio/channels/Channels$1` | 4 |
| `java/nio/channels/Channels$2` | 4 |
| `java/nio/channels/Channels$ReadableByteChannelImpl` | 3 |
| `java/nio/channels/Channels$WritableByteChannelImpl` | 3 |
| `java/nio/channels/ClosedByInterruptException` | 1 |
| `java/nio/channels/ClosedChannelException` | 1 |
| `java/nio/channels/ClosedSelectorException` | 1 |
| `java/nio/channels/CompletionHandler` | 2 |
| `java/nio/channels/ConnectionPendingException` | 1 |
| `java/nio/channels/FileChannel` | 27 |
| `java/nio/channels/FileChannel$MapMode` | 3 |
| `java/nio/channels/FileLock` | 12 |
| `java/nio/channels/FileLockInterruptionException` | 1 |
| `java/nio/channels/IllegalBlockingModeException` | 1 |
| `java/nio/channels/IllegalSelectorException` | 1 |
| `java/nio/channels/NoConnectionPendingException` | 1 |
| `java/nio/channels/NonReadableChannelException` | 1 |
| `java/nio/channels/NonWritableChannelException` | 1 |
| `java/nio/channels/NotYetBoundException` | 1 |
| `java/nio/channels/NotYetConnectedException` | 1 |
| `java/nio/channels/OverlappingFileLockException` | 1 |
| `java/nio/channels/Pipe$SinkChannel` | 2 |
| `java/nio/channels/SeekableByteChannel` | 6 |
| `java/nio/channels/SelectableChannel` | 10 |
| `java/nio/channels/SelectionKey` | 17 |
| `java/nio/channels/Selector` | 17 |
| `java/nio/channels/ShutdownChannelGroupException` | 1 |
| `java/nio/channels/SocketChannel` | 25 |
| `java/nio/channels/UnresolvedAddressException` | 1 |
| `java/nio/channels/UnsupportedAddressTypeException` | 1 |
| `java/nio/channels/spi/AbstractInterruptibleChannel` | 7 |
| `java/nio/channels/spi/AbstractInterruptibleChannel$1` | 2 |
| `java/nio/channels/spi/AbstractSelectableChannel` | 18 |
| `java/nio/channels/spi/AbstractSelectionKey` | 5 |
| `java/nio/channels/spi/AbstractSelector` | 12 |
| `java/nio/channels/spi/AbstractSelector$1` | 2 |
| `java/nio/channels/spi/AsynchronousChannelProvider` | 8 |
| `java/nio/channels/spi/SelectorProvider` | 13 |
| `java/nio/charset/Charset$1` | 6 |
| `java/nio/charset/Charset$3` | 3 |
| `java/nio/file/AccessDeniedException` | 2 |
| `java/nio/file/AccessMode` | 5 |
| `java/nio/file/AtomicMoveNotSupportedException` | 1 |
| `java/nio/file/ClosedDirectoryStreamException` | 1 |
| `java/nio/file/ClosedWatchServiceException` | 1 |
| `java/nio/file/CopyMoveHelper` | 5 |
| `java/nio/file/CopyMoveHelper$CopyOptions` | 2 |
| `java/nio/file/CopyOption` | 0 |
| `java/nio/file/DirectoryIteratorException` | 4 |
| `java/nio/file/DirectoryNotEmptyException` | 1 |
| `java/nio/file/DirectoryStream` | 1 |
| `java/nio/file/DirectoryStream$Filter` | 1 |
| `java/nio/file/FileAlreadyExistsException` | 2 |
| `java/nio/file/FileChannelLinesSpliterator` | 14 |
| `java/nio/file/FileChannelLinesSpliterator$1` | 4 |
| `java/nio/file/FileStore` | 12 |
| `java/nio/file/FileSystemAlreadyExistsException` | 2 |
| `java/nio/file/FileSystemException` | 6 |
| `java/nio/file/FileSystemLoopException` | 1 |
| `java/nio/file/FileSystemNotFoundException` | 2 |
| `java/nio/file/FileSystems` | 9 |
| `java/nio/file/FileTreeIterator` | 7 |
| `java/nio/file/FileTreeWalker` | 11 |
| `java/nio/file/FileTreeWalker$DirectoryNode` | 7 |
| `java/nio/file/FileTreeWalker$Event` | 7 |
| `java/nio/file/FileTreeWalker$EventType` | 5 |
| `java/nio/file/FileVisitOption` | 5 |
| `java/nio/file/FileVisitor` | 4 |
| `java/nio/file/Files` | 85 |
| `java/nio/file/Files$1` | 3 |
| `java/nio/file/Files$2` | 4 |
| `java/nio/file/InvalidPathException` | 6 |
| `java/nio/file/LinkOption` | 5 |
| `java/nio/file/LinkPermission` | 3 |
| `java/nio/file/NoSuchFileException` | 2 |
| `java/nio/file/NotDirectoryException` | 1 |
| `java/nio/file/NotLinkException` | 2 |
| `java/nio/file/OpenOption` | 0 |
| `java/nio/file/Path$1` | 4 |
| `java/nio/file/PathMatcher` | 1 |
| `java/nio/file/Paths` | 3 |
| `java/nio/file/ProviderMismatchException` | 2 |
| `java/nio/file/ProviderNotFoundException` | 2 |
| `java/nio/file/StandardOpenOption` | 5 |
| `java/nio/file/TempFileHelper` | 6 |
| `java/nio/file/WatchEvent` | 3 |
| `java/nio/file/WatchEvent$Kind` | 2 |
| `java/nio/file/WatchEvent$Modifier` | 1 |
| `java/nio/file/WatchKey` | 5 |
| `java/nio/file/attribute/AclFileAttributeView` | 3 |
| `java/nio/file/attribute/BasicFileAttributeView` | 3 |
| `java/nio/file/attribute/BasicFileAttributes` | 9 |
| `java/nio/file/attribute/FileAttribute` | 2 |
| `java/nio/file/attribute/FileAttributeView` | 0 |
| `java/nio/file/attribute/FileOwnerAttributeView` | 3 |
| `java/nio/file/attribute/FileStoreAttributeView` | 0 |
| `java/nio/file/attribute/FileTime` | 16 |
| `java/nio/file/attribute/GroupPrincipal` | 0 |
| `java/nio/file/attribute/PosixFileAttributeView` | 5 |
| `java/nio/file/attribute/PosixFileAttributes` | 3 |
| `java/nio/file/attribute/PosixFilePermission` | 5 |
| `java/nio/file/attribute/UserPrincipal` | 0 |
| `java/nio/file/attribute/UserPrincipalNotFoundException` | 2 |
| `java/nio/file/spi/FileSystemProvider` | 36 |
| `java/nio/file/spi/FileSystemProvider$1` | 3 |
| `java/nio/file/spi/FileTypeDetector` | 4 |
| `java/security/AllPermissionCollection` | 4 |
| `java/security/AllPermissionCollection$1` | 4 |
| `java/security/BasicPermissionCollection` | 7 |
| `java/security/DigestException` | 4 |
| `java/security/InvalidAlgorithmParameterException` | 4 |
| `java/security/Key` | 3 |
| `java/security/KeyStore$Entry` | 1 |
| `java/security/KeyStore$PrivateKeyEntry` | 7 |
| `java/security/KeyStore$SecretKeyEntry` | 5 |
| `java/security/KeyStore$TrustedCertificateEntry` | 5 |
| `java/security/MessageDigest` | 21 |
| `java/security/MessageDigest$Delegate` | 11 |
| `java/security/MessageDigest$Delegate$CloneableDelegate` | 1 |
| `java/security/MessageDigestSpi` | 9 |
| `java/security/PermissionsEnumerator` | 5 |
| `java/security/Policy$PolicyDelegate` | 8 |
| `java/security/PolicySpi` | 5 |
| `java/security/PrivateKey` | 0 |
| `java/security/Provider$OPType` | 5 |
| `java/security/PublicKey` | 0 |
| `java/security/SecureRandom` | 30 |
| `java/security/SecureRandomParameters` | 0 |
| `java/security/SecureRandomSpi` | 9 |
| `java/security/Security$ProviderProperty` | 1 |
| `java/security/Signature` | 33 |
| `java/security/Signature$Delegate` | 25 |
| `java/security/Signature$Delegate$CloneableDelegate` | 1 |
| `java/security/SignatureException` | 4 |
| `java/security/SignatureSpi` | 18 |
| `java/security/Timestamp` | 8 |
| `java/security/UnrecoverableEntryException` | 2 |
| `java/security/UnrecoverableKeyException` | 2 |
| `java/security/cert/CRL` | 4 |
| `java/security/cert/CRLException` | 4 |
| `java/security/cert/CertPath$CertPathRep` | 2 |
| `java/security/cert/CertPathValidatorException` | 10 |
| `java/security/cert/Certificate$CertificateRep` | 2 |
| `java/security/cert/CertificateEncodingException` | 4 |
| `java/security/cert/CertificateExpiredException` | 2 |
| `java/security/cert/CertificateFactory` | 14 |
| `java/security/cert/CertificateFactorySpi` | 9 |
| `java/security/cert/CertificateNotYetValidException` | 2 |
| `java/security/cert/Extension` | 4 |
| `java/security/cert/PolicyQualifierInfo` | 5 |
| `java/security/cert/X509CRL` | 20 |
| `java/security/cert/X509CRLEntry` | 10 |
| `java/security/interfaces/DSAKey` | 1 |
| `java/security/interfaces/DSAParams` | 3 |
| `java/security/interfaces/DSAPublicKey` | 1 |
| `java/security/interfaces/ECKey` | 1 |
| `java/security/interfaces/ECPrivateKey` | 1 |
| `java/security/interfaces/ECPublicKey` | 1 |
| `java/security/interfaces/EdECKey` | 1 |
| `java/security/interfaces/EdECPrivateKey` | 1 |
| `java/security/interfaces/RSAKey` | 2 |
| `java/security/interfaces/XECKey` | 1 |
| `java/security/spec/AlgorithmParameterSpec` | 0 |
| `java/security/spec/ECField` | 1 |
| `java/security/spec/ECFieldF2m` | 9 |
| `java/security/spec/ECFieldFp` | 5 |
| `java/security/spec/ECGenParameterSpec` | 1 |
| `java/security/spec/ECParameterSpec` | 5 |
| `java/security/spec/ECPoint` | 7 |
| `java/security/spec/ECPrivateKeySpec` | 3 |
| `java/security/spec/ECPublicKeySpec` | 3 |
| `java/security/spec/EllipticCurve` | 9 |
| `java/security/spec/MGF1ParameterSpec` | 4 |
| `java/security/spec/NamedParameterSpec` | 3 |
| `java/security/spec/PKCS8EncodedKeySpec` | 4 |
| `java/security/spec/PSSParameterSpec` | 9 |
| `java/text/Annotation` | 3 |
| `java/text/AttributeEntry` | 8 |
| `java/text/AttributedCharacterIterator` | 9 |
| `java/text/AttributedCharacterIterator$Attribute` | 7 |
| `java/text/AttributedString` | 26 |
| `java/text/AttributedString$AttributeMap` | 3 |
| `java/text/AttributedString$AttributedStringIterator` | 25 |
| `java/text/CalendarBuilder` | 10 |
| `java/text/CharacterIterator` | 10 |
| `java/text/CharacterIteratorFieldDelegate` | 4 |
| `java/text/ChoiceFormat` | 19 |
| `java/text/CollationElementIterator` | 22 |
| `java/text/CollationKey` | 5 |
| `java/text/Collator` | 16 |
| `java/text/CompactNumberFormat` | 69 |
| `java/text/CompactNumberFormat$Patterns` | 5 |
| `java/text/DateFormat` | 31 |
| `java/text/DateFormatSymbols` | 36 |
| `java/text/DecimalFormat` | 87 |
| `java/text/DecimalFormat$FastPathData` | 1 |
| `java/text/DecimalFormatSymbols` | 53 |
| `java/text/DigitList` | 28 |
| `java/text/EntryPair` | 2 |
| `java/text/FieldPosition` | 15 |
| `java/text/FieldPosition$Delegate` | 3 |
| `java/text/Format` | 11 |
| `java/text/Format$Field` | 1 |
| `java/text/Format$FieldDelegate` | 2 |
| `java/text/MergeCollation` | 12 |
| `java/text/MessageFormat` | 30 |
| `java/text/MessageFormat$Field` | 3 |
| `java/text/NumberFormat` | 48 |
| `java/text/ParseException` | 2 |
| `java/text/ParsePosition` | 8 |
| `java/text/PatternEntry` | 12 |
| `java/text/PatternEntry$Parser` | 2 |
| `java/text/RBCollationTables` | 14 |
| `java/text/RBCollationTables$BuildAPI` | 2 |
| `java/text/RBTableBuilder` | 16 |
| `java/text/RuleBasedCollationKey` | 6 |
| `java/text/RuleBasedCollator` | 12 |
| `java/text/SimpleDateFormat` | 44 |
| `java/text/spi/CollatorProvider` | 2 |
| `java/text/spi/DateFormatProvider` | 4 |
| `java/text/spi/DateFormatSymbolsProvider` | 2 |
| `java/text/spi/DecimalFormatSymbolsProvider` | 2 |
| `java/text/spi/NumberFormatProvider` | 6 |
| `java/time/Clock$FixedClock` | 8 |
| `java/time/Clock$OffsetClock` | 8 |
| `java/time/Clock$SystemClock` | 9 |
| `java/time/Clock$TickClock` | 8 |
| `java/time/MonthDay` | 36 |
| `java/time/OffsetTime` | 69 |
| `java/time/Period` | 52 |
| `java/time/Ser` | 8 |
| `java/time/Year` | 52 |
| `java/time/YearMonth` | 59 |
| `java/time/ZoneId$1` | 4 |
| `java/time/ZoneRegion` | 11 |
| `java/time/chrono/AbstractChronology` | 29 |
| `java/time/chrono/ChronoPeriod` | 16 |
| `java/time/chrono/ChronoPeriodImpl` | 23 |
| `java/time/chrono/ChronoZonedDateTime` | 43 |
| `java/time/chrono/Chronology$1` | 4 |
| `java/time/chrono/Era` | 8 |
| `java/time/chrono/HijrahChronology` | 71 |
| `java/time/chrono/HijrahDate` | 69 |
| `java/time/chrono/HijrahEra` | 9 |
| `java/time/chrono/IsoEra` | 7 |
| `java/time/chrono/JapaneseChronology` | 41 |
| `java/time/chrono/JapaneseDate` | 68 |
| `java/time/chrono/JapaneseEra` | 20 |
| `java/time/chrono/MinguoChronology` | 36 |
| `java/time/chrono/MinguoDate` | 61 |
| `java/time/chrono/MinguoEra` | 8 |
| `java/time/chrono/Ser` | 8 |
| `java/time/chrono/ThaiBuddhistChronology` | 36 |
| `java/time/chrono/ThaiBuddhistDate` | 61 |
| `java/time/chrono/ThaiBuddhistEra` | 8 |
| `java/time/format/DateTimeFormatter$ClassicFormat` | 4 |
| `java/time/format/DateTimeFormatterBuilder` | 55 |
| `java/time/format/DateTimeFormatterBuilder$1` | 5 |
| `java/time/format/DateTimeFormatterBuilder$CharLiteralPrinterParser` | 4 |
| `java/time/format/DateTimeFormatterBuilder$ChronoPrinterParser` | 5 |
| `java/time/format/DateTimeFormatterBuilder$DayPeriod` | 17 |
| `java/time/format/DateTimeFormatterBuilder$DayPeriodPrinterParser` | 9 |
| `java/time/format/DateTimeFormatterBuilder$DefaultValueParser` | 3 |
| `java/time/format/DateTimeFormatterBuilder$FractionPrinterParser` | 12 |
| `java/time/format/DateTimeFormatterBuilder$InstantPrinterParser` | 4 |
| `java/time/format/DateTimeFormatterBuilder$LocalizedOffsetIdPrinterParser` | 6 |
| `java/time/format/DateTimeFormatterBuilder$LocalizedPrinterParser` | 8 |
| `java/time/format/DateTimeFormatterBuilder$NanosPrinterParser` | 12 |
| `java/time/format/DateTimeFormatterBuilder$NumberPrinterParser` | 12 |
| `java/time/format/DateTimeFormatterBuilder$OffsetIdPrinterParser` | 15 |
| `java/time/format/DateTimeFormatterBuilder$PadPrinterParserDecorator` | 4 |
| `java/time/format/DateTimeFormatterBuilder$PrefixTree` | 13 |
| `java/time/format/DateTimeFormatterBuilder$PrefixTree$CI` | 5 |
| `java/time/format/DateTimeFormatterBuilder$ReducedPrinterParser` | 12 |
| `java/time/format/DateTimeFormatterBuilder$StringLiteralPrinterParser` | 4 |
| `java/time/format/DateTimeFormatterBuilder$TextPrinterParser` | 5 |
| `java/time/format/DateTimeFormatterBuilder$WeekBasedFieldPrinterParser` | 10 |
| `java/time/format/DateTimeFormatterBuilder$ZoneIdPrinterParser` | 6 |
| `java/time/format/DateTimeFormatterBuilder$ZoneTextPrinterParser` | 7 |
| `java/time/format/DateTimeParseContext` | 26 |
| `java/time/format/DateTimeParseException` | 4 |
| `java/time/format/DateTimeTextProvider` | 12 |
| `java/time/format/DateTimeTextProvider$LocaleStore` | 3 |
| `java/time/format/DecimalStyle` | 19 |
| `java/time/format/FormatStyle` | 5 |
| `java/time/format/Parsed` | 22 |
| `java/time/format/ResolverStyle` | 5 |
| `java/time/format/SignStyle` | 6 |
| `java/time/format/TextStyle` | 10 |
| `java/time/format/ZoneName` | 4 |
| `java/time/temporal/WeekFields` | 16 |
| `java/time/temporal/WeekFields$ComputedDayOfField` | 34 |
| `java/time/zone/Ser` | 13 |
| `java/time/zone/ZoneRulesException` | 2 |
| `java/time/zone/ZoneRulesProvider` | 14 |
| `java/util/AbstractList$Itr` | 5 |
| `java/util/AbstractList$ListItr` | 7 |
| `java/util/AbstractList$RandomAccessSpliterator` | 11 |
| `java/util/AbstractList$RandomAccessSubList` | 3 |
| `java/util/AbstractList$SubList` | 17 |
| `java/util/AbstractList$SubList$1` | 10 |
| `java/util/AbstractMap$1` | 6 |
| `java/util/AbstractMap$1$1` | 4 |
| `java/util/AbstractMap$2` | 6 |
| `java/util/AbstractMap$2$1` | 4 |
| `java/util/AbstractMap$SimpleEntry` | 8 |
| `java/util/AbstractMap$SimpleImmutableEntry` | 8 |
| `java/util/AbstractMap$ViewCollection` | 23 |
| `java/util/AbstractQueue` | 6 |
| `java/util/ArrayDeque$DeqIterator` | 6 |
| `java/util/ArrayDeque$DeqSpliterator` | 9 |
| `java/util/ArrayDeque$DescendingIterator` | 4 |
| `java/util/ArrayList$ArrayListSpliterator` | 8 |
| `java/util/ArrayList$ListItr` | 7 |
| `java/util/ArrayList$SubList$1` | 12 |
| `java/util/ArrayList$SubList$2` | 8 |
| `java/util/ArrayPrefixHelpers$CumulateTask` | 3 |
| `java/util/ArrayPrefixHelpers$DoubleCumulateTask` | 3 |
| `java/util/ArrayPrefixHelpers$IntCumulateTask` | 3 |
| `java/util/ArrayPrefixHelpers$LongCumulateTask` | 3 |
| `java/util/Arrays$ArrayItr` | 3 |
| `java/util/ArraysParallelSortHelpers$EmptyCompleter` | 2 |
| `java/util/ArraysParallelSortHelpers$FJObject$Merger` | 2 |
| `java/util/ArraysParallelSortHelpers$FJObject$Sorter` | 2 |
| `java/util/ArraysParallelSortHelpers$Relay` | 3 |
| `java/util/Base64$DecInputStream` | 8 |
| `java/util/Base64$EncOutputStream` | 6 |
| `java/util/Base64$Encoder` | 11 |
| `java/util/BitSet` | 50 |
| `java/util/BitSet$1BitSetSpliterator` | 12 |
| `java/util/Calendar$1` | 3 |
| `java/util/Calendar$Builder` | 20 |
| `java/util/CollSer` | 4 |
| `java/util/Collections$1` | 5 |
| `java/util/Collections$2` | 6 |
| `java/util/Collections$3` | 3 |
| `java/util/Collections$AsLIFOQueue` | 25 |
| `java/util/Collections$CheckedCollection` | 25 |
| `java/util/Collections$CheckedCollection$1` | 5 |
| `java/util/Collections$CheckedList` | 16 |
| `java/util/Collections$CheckedList$1` | 11 |
| `java/util/Collections$CheckedMap` | 33 |
| `java/util/Collections$CheckedMap$CheckedEntrySet` | 20 |
| `java/util/Collections$CheckedMap$CheckedEntrySet$1` | 7 |
| `java/util/Collections$CheckedMap$CheckedEntrySet$CheckedEntry` | 8 |
| `java/util/Collections$CheckedNavigableMap` | 30 |
| `java/util/Collections$CheckedNavigableSet` | 18 |
| `java/util/Collections$CheckedQueue` | 8 |
| `java/util/Collections$CheckedRandomAccessList` | 2 |
| `java/util/Collections$CheckedSet` | 3 |
| `java/util/Collections$CheckedSortedMap` | 7 |
| `java/util/Collections$CheckedSortedSet` | 7 |
| `java/util/Collections$ReverseComparator2` | 6 |
| `java/util/Collections$SequencedSetFromMap` | 11 |
| `java/util/Collections$SingletonSet` | 8 |
| `java/util/Collections$SynchronizedList` | 17 |
| `java/util/Collections$SynchronizedNavigableMap` | 25 |
| `java/util/Collections$SynchronizedNavigableSet` | 19 |
| `java/util/Collections$SynchronizedRandomAccessList` | 4 |
| `java/util/Collections$SynchronizedSortedMap` | 8 |
| `java/util/Collections$SynchronizedSortedSet` | 8 |
| `java/util/Collections$UnmodifiableCollection$1` | 5 |
| `java/util/Collections$UnmodifiableList$1` | 11 |
| `java/util/Collections$UnmodifiableMap` | 27 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet` | 13 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1` | 6 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry` | 7 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntrySetSpliterator` | 9 |
| `java/util/Collections$UnmodifiableNavigableMap` | 20 |
| `java/util/Collections$UnmodifiableNavigableSet` | 13 |
| `java/util/Collections$UnmodifiableSequencedCollection` | 9 |
| `java/util/Collections$UnmodifiableSequencedMap` | 7 |
| `java/util/Collections$UnmodifiableSequencedSet` | 6 |
| `java/util/Collections$UnmodifiableSortedMap` | 7 |
| `java/util/Collections$UnmodifiableSortedSet` | 7 |
| `java/util/Comparators$NullComparator` | 4 |
| `java/util/Currency` | 30 |
| `java/util/Currency$1` | 3 |
| `java/util/Currency$CurrencyProperty` | 8 |
| `java/util/Currency$OtherCurrencyEntry` | 2 |
| `java/util/Currency$SpecialCaseEntry` | 6 |
| `java/util/DoubleSummaryStatistics` | 11 |
| `java/util/EmptyStackException` | 1 |
| `java/util/EnumMap` | 31 |
| `java/util/EnumMap$EntryIterator` | 4 |
| `java/util/EnumMap$EntryIterator$Entry` | 9 |
| `java/util/EnumMap$EntrySet` | 9 |
| `java/util/EnumMap$EnumMapIterator` | 4 |
| `java/util/EnumMap$KeyIterator` | 3 |
| `java/util/EnumMap$KeySet` | 6 |
| `java/util/EnumMap$ValueIterator` | 2 |
| `java/util/EnumMap$Values` | 6 |
| `java/util/EnumSet$SerializationProxy` | 3 |
| `java/util/Enumeration$1` | 3 |
| `java/util/Formattable` | 1 |
| `java/util/Formatter$FormatSpecifier$BigDecimalLayout` | 6 |
| `java/util/HashMap$EntryIterator` | 3 |
| `java/util/HashMap$EntrySpliterator` | 6 |
| `java/util/HashMap$HashIterator` | 4 |
| `java/util/HashMap$HashMapSpliterator` | 3 |
| `java/util/HashMap$KeyIterator` | 2 |
| `java/util/HashMap$KeySpliterator` | 6 |
| `java/util/HashMap$UnsafeHolder` | 3 |
| `java/util/HashMap$ValueIterator` | 2 |
| `java/util/HashMap$ValueSpliterator` | 6 |
| `java/util/HashMap$Values` | 9 |
| `java/util/Hashtable$EntrySet` | 8 |
| `java/util/Hashtable$Enumerator` | 6 |
| `java/util/Hashtable$KeySet` | 6 |
| `java/util/Hashtable$UnsafeHolder` | 3 |
| `java/util/Hashtable$ValueCollection` | 5 |
| `java/util/IdentityHashMap$EntryIterator` | 4 |
| `java/util/IdentityHashMap$EntryIterator$Entry` | 8 |
| `java/util/IdentityHashMap$EntrySet` | 10 |
| `java/util/IdentityHashMap$EntrySpliterator` | 6 |
| `java/util/IdentityHashMap$IdentityHashMapIterator` | 4 |
| `java/util/IdentityHashMap$IdentityHashMapSpliterator` | 3 |
| `java/util/IdentityHashMap$KeyIterator` | 2 |
| `java/util/IdentityHashMap$KeySet` | 11 |
| `java/util/IdentityHashMap$KeySpliterator` | 6 |
| `java/util/IdentityHashMap$ValueIterator` | 2 |
| `java/util/IdentityHashMap$ValueSpliterator` | 6 |
| `java/util/IdentityHashMap$Values` | 9 |
| `java/util/IllegalFormatCodePointException` | 3 |
| `java/util/IllegalFormatConversionException` | 4 |
| `java/util/ImmutableCollections$ListItr` | 11 |
| `java/util/ImmutableCollections$MapN$1` | 3 |
| `java/util/ImmutableCollections$MapN$MapNIterator` | 5 |
| `java/util/ImmutableCollections$Set12$1` | 3 |
| `java/util/ImmutableCollections$SetN$SetNIterator` | 3 |
| `java/util/ImmutableCollections$SubList` | 15 |
| `java/util/InputMismatchException` | 2 |
| `java/util/IntSummaryStatistics` | 10 |
| `java/util/JapaneseImperialCalendar` | 40 |
| `java/util/JumboEnumSet$EnumSetIterator` | 5 |
| `java/util/LinkedHashMap$LinkedEntryIterator` | 3 |
| `java/util/LinkedHashMap$LinkedEntrySet` | 23 |
| `java/util/LinkedHashMap$LinkedHashIterator` | 4 |
| `java/util/LinkedHashMap$LinkedKeyIterator` | 2 |
| `java/util/LinkedHashMap$LinkedKeySet` | 18 |
| `java/util/LinkedHashMap$LinkedValueIterator` | 2 |
| `java/util/LinkedHashMap$LinkedValues` | 16 |
| `java/util/LinkedHashMap$ReversedLinkedHashMapView` | 33 |
| `java/util/LinkedHashSet$1ReverseLinkedHashSetView` | 14 |
| `java/util/LinkedList$DescendingIterator` | 4 |
| `java/util/LinkedList$LLSpliterator` | 7 |
| `java/util/LinkedList$ListItr` | 12 |
| `java/util/LinkedList$ReverseOrderLinkedListView` | 63 |
| `java/util/ListResourceBundle` | 6 |
| `java/util/Locale$Category` | 5 |
| `java/util/Locale$IsoCountryCode` | 7 |
| `java/util/Locale$LanguageRange` | 11 |
| `java/util/LongSummaryStatistics` | 11 |
| `java/util/OptionalDouble` | 18 |
| `java/util/OptionalInt` | 18 |
| `java/util/OptionalLong` | 18 |
| `java/util/Properties$EntrySet` | 18 |
| `java/util/Properties$LineReader` | 3 |
| `java/util/PropertyPermissionCollection` | 8 |
| `java/util/PropertyResourceBundle` | 6 |
| `java/util/Queue` | 6 |
| `java/util/Random$RandomWrapper` | 36 |
| `java/util/RegularEnumSet$EnumSetIterator` | 5 |
| `java/util/ResourceBundle$3` | 3 |
| `java/util/ResourceBundle$4` | 3 |
| `java/util/ResourceBundle$BundleReference` | 2 |
| `java/util/ResourceBundle$CacheKey` | 17 |
| `java/util/ResourceBundle$CacheKeyReference` | 1 |
| `java/util/ResourceBundle$Control$1` | 3 |
| `java/util/ResourceBundle$Control$2` | 3 |
| `java/util/ResourceBundle$KeyElementReference` | 2 |
| `java/util/ResourceBundle$ResourceBundleControlProviderHolder` | 5 |
| `java/util/ResourceBundle$ResourceBundleProviderHelper` | 9 |
| `java/util/ResourceBundle$SingleFormatControl` | 3 |
| `java/util/ReverseOrderDequeView` | 43 |
| `java/util/ReverseOrderListView` | 38 |
| `java/util/ReverseOrderListView$DescendingIterator` | 4 |
| `java/util/ReverseOrderListView$DescendingListIterator` | 10 |
| `java/util/ReverseOrderListView$Rand` | 1 |
| `java/util/ReverseOrderSortedMapView` | 33 |
| `java/util/ReverseOrderSortedMapView$1` | 6 |
| `java/util/ReverseOrderSortedMapView$2` | 6 |
| `java/util/ReverseOrderSortedMapView$3` | 6 |
| `java/util/ReverseOrderSortedMapView$4` | 4 |
| `java/util/ReverseOrderSortedMapView$5` | 4 |
| `java/util/ReverseOrderSortedMapView$6` | 5 |
| `java/util/ReverseOrderSortedMapView$Submap` | 15 |
| `java/util/ReverseOrderSortedMapView$Submap$1` | 5 |
| `java/util/ReverseOrderSortedMapView$Submap$2` | 3 |
| `java/util/ReverseOrderSortedMapView$ViewEntry` | 7 |
| `java/util/ReverseOrderSortedSetView` | 30 |
| `java/util/ReverseOrderSortedSetView$1` | 4 |
| `java/util/ReverseOrderSortedSetView$Subset` | 13 |
| `java/util/ReverseOrderSortedSetView$Subset$1` | 3 |
| `java/util/Scanner` | 111 |
| `java/util/Scanner$FindSpliterator` | 3 |
| `java/util/Scanner$PatternLRUCache` | 4 |
| `java/util/Scanner$TokenSpliterator` | 2 |
| `java/util/SequencedCollection` | 7 |
| `java/util/SequencedMap` | 10 |
| `java/util/SequencedMap$1SeqEntrySet` | 6 |
| `java/util/SequencedMap$1SeqKeySet` | 6 |
| `java/util/SequencedMap$1SeqValues` | 3 |
| `java/util/SequencedSet` | 2 |
| `java/util/ServiceLoader$1` | 3 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator$1` | 3 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator$2` | 3 |
| `java/util/ServiceLoader$Provider` | 2 |
| `java/util/ServiceLoader$ProviderImpl` | 8 |
| `java/util/ServiceLoader$ProviderImpl$1` | 2 |
| `java/util/ServiceLoader$ProviderImpl$2` | 2 |
| `java/util/ServiceLoader$ProviderSpliterator` | 5 |
| `java/util/SimpleTimeZone` | 42 |
| `java/util/SimpleTimeZone$Cache` | 1 |
| `java/util/SortedSet$1` | 2 |
| `java/util/Spliterator$OfDouble` | 9 |
| `java/util/Spliterator$OfInt` | 9 |
| `java/util/Spliterator$OfLong` | 9 |
| `java/util/Spliterator$OfPrimitive` | 4 |
| `java/util/Spliterators$1Adapter` | 5 |
| `java/util/Spliterators$2Adapter` | 6 |
| `java/util/Spliterators$3Adapter` | 6 |
| `java/util/Spliterators$4Adapter` | 6 |
| `java/util/Spliterators$AbstractDoubleSpliterator` | 6 |
| `java/util/Spliterators$AbstractDoubleSpliterator$HoldingDoubleConsumer` | 2 |
| `java/util/Spliterators$AbstractIntSpliterator` | 6 |
| `java/util/Spliterators$AbstractIntSpliterator$HoldingIntConsumer` | 2 |
| `java/util/Spliterators$AbstractLongSpliterator` | 6 |
| `java/util/Spliterators$AbstractLongSpliterator$HoldingLongConsumer` | 2 |
| `java/util/Spliterators$AbstractSpliterator` | 4 |
| `java/util/Spliterators$AbstractSpliterator$HoldingConsumer` | 2 |
| `java/util/Stack` | 6 |
| `java/util/TreeMap$AscendingSubMap` | 16 |
| `java/util/TreeMap$AscendingSubMap$AscendingEntrySetView` | 2 |
| `java/util/TreeMap$DescendingKeyIterator` | 3 |
| `java/util/TreeMap$DescendingKeySpliterator` | 6 |
| `java/util/TreeMap$DescendingSubMap` | 16 |
| `java/util/TreeMap$DescendingSubMap$DescendingEntrySetView` | 2 |
| `java/util/TreeMap$EntryIterator` | 3 |
| `java/util/TreeMap$EntrySpliterator` | 9 |
| `java/util/TreeMap$KeyIterator` | 2 |
| `java/util/TreeMap$KeySet` | 25 |
| `java/util/TreeMap$KeySpliterator` | 7 |
| `java/util/TreeMap$NavigableSubMap` | 54 |
| `java/util/TreeMap$NavigableSubMap$DescendingSubMapEntryIterator` | 4 |
| `java/util/TreeMap$NavigableSubMap$DescendingSubMapKeyIterator` | 8 |
| `java/util/TreeMap$NavigableSubMap$EntrySetView` | 5 |
| `java/util/TreeMap$NavigableSubMap$SubMapEntryIterator` | 4 |
| `java/util/TreeMap$NavigableSubMap$SubMapIterator` | 6 |
| `java/util/TreeMap$NavigableSubMap$SubMapKeyIterator` | 9 |
| `java/util/TreeMap$PrivateEntryIterator` | 5 |
| `java/util/TreeMap$TreeMapSpliterator` | 3 |
| `java/util/TreeMap$ValueIterator` | 2 |
| `java/util/TreeMap$ValueSpliterator` | 6 |
| `java/util/TreeMap$Values` | 7 |
| `java/util/Tripwire` | 4 |
| `java/util/Vector$Itr` | 6 |
| `java/util/Vector$ListItr` | 7 |
| `java/util/Vector$VectorSpliterator` | 7 |
| `java/util/WeakHashMap$EntryIterator` | 3 |
| `java/util/WeakHashMap$EntrySet` | 10 |
| `java/util/WeakHashMap$EntrySpliterator` | 6 |
| `java/util/WeakHashMap$HashIterator` | 4 |
| `java/util/WeakHashMap$KeyIterator` | 2 |
| `java/util/WeakHashMap$KeySet` | 7 |
| `java/util/WeakHashMap$KeySpliterator` | 6 |
| `java/util/WeakHashMap$ValueIterator` | 2 |
| `java/util/WeakHashMap$ValueSpliterator` | 6 |
| `java/util/WeakHashMap$Values` | 6 |
| `java/util/WeakHashMap$WeakHashMapSpliterator` | 3 |
| `java/util/concurrent/AbstractExecutorService` | 14 |
| `java/util/concurrent/BlockingQueue` | 11 |
| `java/util/concurrent/CancellationException` | 2 |
| `java/util/concurrent/CompletableFuture` | 178 |
| `java/util/concurrent/CompletableFuture$AltResult` | 1 |
| `java/util/concurrent/CompletableFuture$AnyOf` | 3 |
| `java/util/concurrent/CompletableFuture$AsyncRun` | 7 |
| `java/util/concurrent/CompletableFuture$AsyncSupply` | 7 |
| `java/util/concurrent/CompletableFuture$AsynchronousCompletionTask` | 0 |
| `java/util/concurrent/CompletableFuture$BiAccept` | 2 |
| `java/util/concurrent/CompletableFuture$BiApply` | 2 |
| `java/util/concurrent/CompletableFuture$BiCompletion` | 1 |
| `java/util/concurrent/CompletableFuture$BiRelay` | 2 |
| `java/util/concurrent/CompletableFuture$BiRun` | 2 |
| `java/util/concurrent/CompletableFuture$Canceller` | 3 |
| `java/util/concurrent/CompletableFuture$CoCompletion` | 3 |
| `java/util/concurrent/CompletableFuture$Completion` | 9 |
| `java/util/concurrent/CompletableFuture$DelayedCompleter` | 2 |
| `java/util/concurrent/CompletableFuture$DelayedExecutor` | 2 |
| `java/util/concurrent/CompletableFuture$Delayer` | 3 |
| `java/util/concurrent/CompletableFuture$MinimalStage` | 66 |
| `java/util/concurrent/CompletableFuture$OrAccept` | 2 |
| `java/util/concurrent/CompletableFuture$OrApply` | 2 |
| `java/util/concurrent/CompletableFuture$OrRun` | 2 |
| `java/util/concurrent/CompletableFuture$Signaller` | 5 |
| `java/util/concurrent/CompletableFuture$TaskSubmitter` | 2 |
| `java/util/concurrent/CompletableFuture$Timeout` | 2 |
| `java/util/concurrent/CompletableFuture$UniAccept` | 2 |
| `java/util/concurrent/CompletableFuture$UniApply` | 2 |
| `java/util/concurrent/CompletableFuture$UniCompletion` | 3 |
| `java/util/concurrent/CompletableFuture$UniCompose` | 2 |
| `java/util/concurrent/CompletableFuture$UniComposeExceptionally` | 2 |
| `java/util/concurrent/CompletableFuture$UniExceptionally` | 2 |
| `java/util/concurrent/CompletableFuture$UniHandle` | 2 |
| `java/util/concurrent/CompletableFuture$UniRelay` | 2 |
| `java/util/concurrent/CompletableFuture$UniRun` | 2 |
| `java/util/concurrent/CompletableFuture$UniWhenComplete` | 2 |
| `java/util/concurrent/CompletionException` | 4 |
| `java/util/concurrent/CompletionStage` | 52 |
| `java/util/concurrent/ConcurrentHashMap$BaseIterator` | 4 |
| `java/util/concurrent/ConcurrentHashMap$BulkTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$EntryIterator` | 3 |
| `java/util/concurrent/ConcurrentHashMap$EntrySetView` | 12 |
| `java/util/concurrent/ConcurrentHashMap$EntrySpliterator` | 7 |
| `java/util/concurrent/ConcurrentHashMap$ForEachEntryTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$ForEachKeyTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$ForEachMappingTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedEntryTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedKeyTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedMappingTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$ForEachTransformedValueTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$ForEachValueTask` | 2 |
| `java/util/concurrent/ConcurrentHashMap$KeyIterator` | 3 |
| `java/util/concurrent/ConcurrentHashMap$KeySpliterator` | 7 |
| `java/util/concurrent/ConcurrentHashMap$MapEntry` | 7 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToDoubleTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToIntTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceEntriesToLongTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysToDoubleTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysToIntTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceKeysToLongTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToDoubleTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToIntTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceMappingsToLongTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesToDoubleTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesToIntTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$MapReduceValuesToLongTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$ReduceEntriesTask` | 4 |
| `java/util/concurrent/ConcurrentHashMap$ReduceKeysTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$ReduceValuesTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$SearchEntriesTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$SearchKeysTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$SearchMappingsTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$SearchValuesTask` | 3 |
| `java/util/concurrent/ConcurrentHashMap$Segment` | 1 |
| `java/util/concurrent/ConcurrentHashMap$TableStack` | 1 |
| `java/util/concurrent/ConcurrentHashMap$Traverser` | 4 |
| `java/util/concurrent/ConcurrentHashMap$ValueIterator` | 3 |
| `java/util/concurrent/ConcurrentHashMap$ValueSpliterator` | 7 |
| `java/util/concurrent/ConcurrentHashMap$ValuesView` | 10 |
| `java/util/concurrent/ConcurrentLinkedQueue` | 35 |
| `java/util/concurrent/ConcurrentLinkedQueue$CLQSpliterator` | 8 |
| `java/util/concurrent/ConcurrentLinkedQueue$Itr` | 4 |
| `java/util/concurrent/ConcurrentLinkedQueue$Node` | 4 |
| `java/util/concurrent/CopyOnWriteArrayList$COWIterator` | 11 |
| `java/util/concurrent/CopyOnWriteArrayList$COWSubList` | 47 |
| `java/util/concurrent/CopyOnWriteArrayList$COWSubListIterator` | 11 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed` | 44 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed$DescendingIterator` | 4 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed$DescendingListIterator` | 10 |
| `java/util/concurrent/CountDownLatch` | 6 |
| `java/util/concurrent/CountDownLatch$Sync` | 4 |
| `java/util/concurrent/Delayed` | 1 |
| `java/util/concurrent/ExecutionException` | 4 |
| `java/util/concurrent/Executor` | 1 |
| `java/util/concurrent/ExecutorCompletionService` | 9 |
| `java/util/concurrent/ExecutorCompletionService$QueueingFuture` | 2 |
| `java/util/concurrent/ExecutorService` | 13 |
| `java/util/concurrent/Executors` | 25 |
| `java/util/concurrent/Executors$1` | 2 |
| `java/util/concurrent/Executors$2` | 2 |
| `java/util/concurrent/Executors$AutoShutdownDelegatedExecutorService` | 5 |
| `java/util/concurrent/Executors$DefaultThreadFactory` | 3 |
| `java/util/concurrent/Executors$DelegatedExecutorService` | 14 |
| `java/util/concurrent/Executors$DelegatedScheduledExecutorService` | 5 |
| `java/util/concurrent/Executors$PrivilegedCallable` | 3 |
| `java/util/concurrent/Executors$PrivilegedCallable$1` | 2 |
| `java/util/concurrent/Executors$PrivilegedCallableUsingCurrentClassLoader` | 3 |
| `java/util/concurrent/Executors$PrivilegedCallableUsingCurrentClassLoader$1` | 2 |
| `java/util/concurrent/Executors$PrivilegedThreadFactory` | 2 |
| `java/util/concurrent/Executors$PrivilegedThreadFactory$1` | 2 |
| `java/util/concurrent/Executors$PrivilegedThreadFactory$1$1` | 3 |
| `java/util/concurrent/Executors$RunnableAdapter` | 3 |
| `java/util/concurrent/ForkJoinPool` | 93 |
| `java/util/concurrent/ForkJoinPool$ForkJoinWorkerThreadFactory` | 1 |
| `java/util/concurrent/ForkJoinPool$InvokeAnyRoot` | 5 |
| `java/util/concurrent/ForkJoinPool$InvokeAnyTask` | 5 |
| `java/util/concurrent/ForkJoinPool$ManagedBlocker` | 2 |
| `java/util/concurrent/ForkJoinPool$WorkQueue` | 22 |
| `java/util/concurrent/ForkJoinTask$AdaptedCallable` | 6 |
| `java/util/concurrent/ForkJoinTask$AdaptedInterruptibleCallable` | 7 |
| `java/util/concurrent/ForkJoinTask$AdaptedRunnable` | 6 |
| `java/util/concurrent/ForkJoinTask$AdaptedRunnableAction` | 8 |
| `java/util/concurrent/ForkJoinTask$Aux` | 3 |
| `java/util/concurrent/ForkJoinTask$RunnableExecuteAction` | 7 |
| `java/util/concurrent/ForkJoinWorkerThread` | 9 |
| `java/util/concurrent/Future` | 8 |
| `java/util/concurrent/Future$State` | 5 |
| `java/util/concurrent/FutureTask` | 22 |
| `java/util/concurrent/FutureTask$WaitNode` | 1 |
| `java/util/concurrent/Helpers` | 5 |
| `java/util/concurrent/LinkedBlockingDeque` | 68 |
| `java/util/concurrent/LinkedBlockingDeque$AbstractItr` | 8 |
| `java/util/concurrent/LinkedBlockingDeque$DescendingItr` | 3 |
| `java/util/concurrent/LinkedBlockingDeque$Itr` | 3 |
| `java/util/concurrent/LinkedBlockingDeque$LBDSpliterator` | 6 |
| `java/util/concurrent/LinkedBlockingDeque$Node` | 1 |
| `java/util/concurrent/LinkedBlockingQueue` | 41 |
| `java/util/concurrent/LinkedBlockingQueue$Itr` | 5 |
| `java/util/concurrent/LinkedBlockingQueue$LBQSpliterator` | 6 |
| `java/util/concurrent/LinkedBlockingQueue$Node` | 1 |
| `java/util/concurrent/LinkedTransferQueue` | 51 |
| `java/util/concurrent/LinkedTransferQueue$DualNode` | 10 |
| `java/util/concurrent/LinkedTransferQueue$Itr` | 6 |
| `java/util/concurrent/LinkedTransferQueue$LTQSpliterator` | 8 |
| `java/util/concurrent/RejectedExecutionException` | 4 |
| `java/util/concurrent/RejectedExecutionHandler` | 1 |
| `java/util/concurrent/RunnableScheduledFuture` | 1 |
| `java/util/concurrent/ScheduledExecutorService` | 4 |
| `java/util/concurrent/ScheduledFuture` | 0 |
| `java/util/concurrent/ScheduledThreadPoolExecutor` | 31 |
| `java/util/concurrent/ScheduledThreadPoolExecutor$DelayedWorkQueue` | 34 |
| `java/util/concurrent/ScheduledThreadPoolExecutor$DelayedWorkQueue$Itr` | 5 |
| `java/util/concurrent/ScheduledThreadPoolExecutor$ScheduledFutureTask` | 10 |
| `java/util/concurrent/StructureViolationException` | 2 |
| `java/util/concurrent/SynchronousQueue` | 28 |
| `java/util/concurrent/SynchronousQueue$FifoWaitQueue` | 1 |
| `java/util/concurrent/SynchronousQueue$LifoWaitQueue` | 1 |
| `java/util/concurrent/SynchronousQueue$Transferer` | 3 |
| `java/util/concurrent/SynchronousQueue$WaitQueue` | 1 |
| `java/util/concurrent/ThreadFactory` | 1 |
| `java/util/concurrent/ThreadPerTaskExecutor` | 31 |
| `java/util/concurrent/ThreadPerTaskExecutor$AnyResultHolder` | 7 |
| `java/util/concurrent/ThreadPerTaskExecutor$TaskRunner` | 2 |
| `java/util/concurrent/ThreadPerTaskExecutor$ThreadBoundFuture` | 3 |
| `java/util/concurrent/ThreadPoolExecutor` | 64 |
| `java/util/concurrent/ThreadPoolExecutor$Worker` | 10 |
| `java/util/concurrent/TimeUnit` | 21 |
| `java/util/concurrent/TimeoutException` | 2 |
| `java/util/concurrent/atomic/AtomicBoolean` | 23 |
| `java/util/concurrent/atomic/AtomicMarkableReference` | 10 |
| `java/util/concurrent/atomic/AtomicMarkableReference$Pair` | 2 |
| `java/util/concurrent/atomic/AtomicReference` | 27 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionNode` | 3 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$ExclusiveNode` | 1 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$Node` | 8 |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$SharedNode` | 1 |
| `java/util/concurrent/locks/Condition` | 7 |
| `java/util/concurrent/locks/Lock` | 6 |
| `java/util/concurrent/locks/ReadWriteLock` | 2 |
| `java/util/concurrent/locks/ReentrantReadWriteLock` | 23 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$FairSync` | 3 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$NonfairSync` | 3 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock` | 8 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$Sync` | 22 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$Sync$HoldCounter` | 1 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$Sync$ThreadLocalHoldCounter` | 3 |
| `java/util/concurrent/locks/ReentrantReadWriteLock$WriteLock` | 10 |
| `java/util/function/BiConsumer` | 3 |
| `java/util/function/BiFunction` | 3 |
| `java/util/function/BiPredicate` | 7 |
| `java/util/function/BinaryOperator` | 4 |
| `java/util/function/BooleanSupplier` | 1 |
| `java/util/function/Consumer` | 3 |
| `java/util/function/DoubleBinaryOperator` | 1 |
| `java/util/function/DoubleConsumer` | 3 |
| `java/util/function/DoubleFunction` | 1 |
| `java/util/function/DoublePredicate` | 7 |
| `java/util/function/DoubleSupplier` | 1 |
| `java/util/function/DoubleToIntFunction` | 1 |
| `java/util/function/DoubleToLongFunction` | 1 |
| `java/util/function/DoubleUnaryOperator` | 7 |
| `java/util/function/IntBinaryOperator` | 1 |
| `java/util/function/IntConsumer` | 3 |
| `java/util/function/IntPredicate` | 7 |
| `java/util/function/IntSupplier` | 1 |
| `java/util/function/IntToDoubleFunction` | 1 |
| `java/util/function/IntToLongFunction` | 1 |
| `java/util/function/IntUnaryOperator` | 7 |
| `java/util/function/LongBinaryOperator` | 1 |
| `java/util/function/LongConsumer` | 3 |
| `java/util/function/LongFunction` | 1 |
| `java/util/function/LongPredicate` | 7 |
| `java/util/function/LongSupplier` | 1 |
| `java/util/function/LongToDoubleFunction` | 1 |
| `java/util/function/LongToIntFunction` | 1 |
| `java/util/function/LongUnaryOperator` | 7 |
| `java/util/function/ObjDoubleConsumer` | 1 |
| `java/util/function/ObjIntConsumer` | 1 |
| `java/util/function/ObjLongConsumer` | 1 |
| `java/util/function/Predicate` | 10 |
| `java/util/function/ToDoubleBiFunction` | 1 |
| `java/util/function/ToDoubleFunction` | 1 |
| `java/util/function/ToIntBiFunction` | 1 |
| `java/util/function/ToIntFunction` | 1 |
| `java/util/function/ToLongBiFunction` | 1 |
| `java/util/function/ToLongFunction` | 1 |
| `java/util/function/UnaryOperator` | 2 |
| `java/util/jar/Attributes` | 25 |
| `java/util/jar/Attributes$Name` | 8 |
| `java/util/jar/JarEntry` | 7 |
| `java/util/jar/JarException` | 2 |
| `java/util/jar/JarFile` | 35 |
| `java/util/jar/JarFile$JarFileEntry` | 9 |
| `java/util/jar/JarVerifier` | 12 |
| `java/util/jar/JarVerifier$VerifierStream` | 6 |
| `java/util/jar/Manifest` | 22 |
| `java/util/jar/Manifest$FastInputStream` | 11 |
| `java/util/random/RandomGenerator` | 35 |
| `java/util/random/RandomGeneratorFactory` | 31 |
| `java/util/regex/MatchResult` | 13 |
| `java/util/regex/Matcher$1MatchResultIterator` | 5 |
| `java/util/regex/Matcher$ImmutableMatchResult` | 12 |
| `java/util/regex/Pattern$1MatcherIterator` | 4 |
| `java/util/regex/Pattern$Qtype` | 5 |
| `java/util/spi/CalendarDataProvider` | 3 |
| `java/util/spi/CalendarNameProvider` | 3 |
| `java/util/spi/CurrencyNameProvider` | 3 |
| `java/util/spi/LocaleNameProvider` | 7 |
| `java/util/spi/ResourceBundleControlProvider` | 1 |
| `java/util/spi/ResourceBundleProvider` | 1 |
| `java/util/spi/TimeZoneNameProvider` | 3 |
| `java/util/stream/AbstractShortCircuitTask` | 11 |
| `java/util/stream/AbstractSpinedBuffer` | 6 |
| `java/util/stream/AbstractTask` | 18 |
| `java/util/stream/BaseStream` | 8 |
| `java/util/stream/Collectors$1OptionalBox` | 2 |
| `java/util/stream/Collectors$1PairBox` | 4 |
| `java/util/stream/Collectors$Partition` | 7 |
| `java/util/stream/Collectors$Partition$1` | 3 |
| `java/util/stream/DistinctOps` | 2 |
| `java/util/stream/DistinctOps$1` | 6 |
| `java/util/stream/DistinctOps$1$1` | 4 |
| `java/util/stream/DistinctOps$1$2` | 4 |
| `java/util/stream/DoublePipeline$1` | 2 |
| `java/util/stream/DoublePipeline$1$1` | 2 |
| `java/util/stream/DoublePipeline$2` | 2 |
| `java/util/stream/DoublePipeline$2$1` | 2 |
| `java/util/stream/DoublePipeline$3` | 2 |
| `java/util/stream/DoublePipeline$3$1` | 2 |
| `java/util/stream/DoublePipeline$4` | 2 |
| `java/util/stream/DoublePipeline$4$1` | 2 |
| `java/util/stream/DoublePipeline$5` | 2 |
| `java/util/stream/DoublePipeline$5$1` | 4 |
| `java/util/stream/DoublePipeline$6` | 2 |
| `java/util/stream/DoublePipeline$6$1` | 3 |
| `java/util/stream/DoublePipeline$7` | 2 |
| `java/util/stream/DoublePipeline$8` | 2 |
| `java/util/stream/DoublePipeline$8$1` | 3 |
| `java/util/stream/DoublePipeline$9` | 2 |
| `java/util/stream/DoublePipeline$9$1` | 2 |
| `java/util/stream/DoublePipeline$StatefulOp` | 10 |
| `java/util/stream/DoublePipeline$StatelessOp` | 9 |
| `java/util/stream/DoubleStream` | 49 |
| `java/util/stream/DoubleStream$1` | 3 |
| `java/util/stream/DoubleStream$2` | 5 |
| `java/util/stream/DoubleStream$DoubleMapMultiConsumer` | 1 |
| `java/util/stream/FindOps` | 5 |
| `java/util/stream/ForEachOps` | 5 |
| `java/util/stream/ForEachOps$ForEachOp` | 8 |
| `java/util/stream/ForEachOps$ForEachOp$OfDouble` | 6 |
| `java/util/stream/ForEachOps$ForEachOp$OfInt` | 6 |
| `java/util/stream/ForEachOps$ForEachOp$OfLong` | 6 |
| `java/util/stream/ForEachOps$ForEachOp$OfRef` | 5 |
| `java/util/stream/ForEachOps$ForEachOrderedTask` | 7 |
| `java/util/stream/ForEachOps$ForEachTask` | 3 |
| `java/util/stream/IntPipeline$1` | 2 |
| `java/util/stream/IntPipeline$1$1` | 2 |
| `java/util/stream/IntPipeline$10` | 2 |
| `java/util/stream/IntPipeline$10$1` | 3 |
| `java/util/stream/IntPipeline$11` | 2 |
| `java/util/stream/IntPipeline$11$1` | 2 |
| `java/util/stream/IntPipeline$2` | 2 |
| `java/util/stream/IntPipeline$2$1` | 2 |
| `java/util/stream/IntPipeline$3` | 2 |
| `java/util/stream/IntPipeline$3$1` | 2 |
| `java/util/stream/IntPipeline$4` | 2 |
| `java/util/stream/IntPipeline$4$1` | 2 |
| `java/util/stream/IntPipeline$5` | 2 |
| `java/util/stream/IntPipeline$5$1` | 2 |
| `java/util/stream/IntPipeline$6` | 2 |
| `java/util/stream/IntPipeline$6$1` | 2 |
| `java/util/stream/IntPipeline$7` | 2 |
| `java/util/stream/IntPipeline$7$1` | 4 |
| `java/util/stream/IntPipeline$8` | 2 |
| `java/util/stream/IntPipeline$8$1` | 3 |
| `java/util/stream/IntPipeline$9` | 2 |
| `java/util/stream/IntPipeline$StatefulOp` | 10 |
| `java/util/stream/IntPipeline$StatelessOp` | 9 |
| `java/util/stream/IntStream` | 53 |
| `java/util/stream/IntStream$1` | 3 |
| `java/util/stream/IntStream$2` | 5 |
| `java/util/stream/IntStream$IntMapMultiConsumer` | 1 |
| `java/util/stream/LongPipeline$1` | 2 |
| `java/util/stream/LongPipeline$1$1` | 2 |
| `java/util/stream/LongPipeline$10` | 2 |
| `java/util/stream/LongPipeline$10$1` | 2 |
| `java/util/stream/LongPipeline$2` | 2 |
| `java/util/stream/LongPipeline$2$1` | 2 |
| `java/util/stream/LongPipeline$3` | 2 |
| `java/util/stream/LongPipeline$3$1` | 2 |
| `java/util/stream/LongPipeline$4` | 2 |
| `java/util/stream/LongPipeline$4$1` | 2 |
| `java/util/stream/LongPipeline$5` | 2 |
| `java/util/stream/LongPipeline$5$1` | 2 |
| `java/util/stream/LongPipeline$6` | 2 |
| `java/util/stream/LongPipeline$6$1` | 4 |
| `java/util/stream/LongPipeline$7` | 2 |
| `java/util/stream/LongPipeline$7$1` | 3 |
| `java/util/stream/LongPipeline$8` | 2 |
| `java/util/stream/LongPipeline$9` | 2 |
| `java/util/stream/LongPipeline$9$1` | 3 |
| `java/util/stream/LongPipeline$StatefulOp` | 10 |
| `java/util/stream/LongPipeline$StatelessOp` | 9 |
| `java/util/stream/LongStream` | 52 |
| `java/util/stream/LongStream$1` | 3 |
| `java/util/stream/LongStream$2` | 5 |
| `java/util/stream/LongStream$LongMapMultiConsumer` | 1 |
| `java/util/stream/MatchOps` | 9 |
| `java/util/stream/MatchOps$1MatchSink` | 2 |
| `java/util/stream/MatchOps$2MatchSink` | 2 |
| `java/util/stream/MatchOps$3MatchSink` | 2 |
| `java/util/stream/MatchOps$4MatchSink` | 2 |
| `java/util/stream/MatchOps$BooleanTerminalSink` | 3 |
| `java/util/stream/MatchOps$MatchOp` | 7 |
| `java/util/stream/MatchOps$MatchTask` | 8 |
| `java/util/stream/Node` | 10 |
| `java/util/stream/Node$Builder` | 1 |
| `java/util/stream/Node$Builder$OfDouble` | 2 |
| `java/util/stream/Node$Builder$OfInt` | 2 |
| `java/util/stream/Node$Builder$OfLong` | 2 |
| `java/util/stream/Node$OfDouble` | 10 |
| `java/util/stream/Node$OfInt` | 10 |
| `java/util/stream/Node$OfLong` | 10 |
| `java/util/stream/Node$OfPrimitive` | 11 |
| `java/util/stream/Nodes` | 27 |
| `java/util/stream/Nodes$AbstractConcNode` | 4 |
| `java/util/stream/Nodes$ArrayNode` | 8 |
| `java/util/stream/Nodes$CollectionNode` | 7 |
| `java/util/stream/Nodes$CollectorTask` | 7 |
| `java/util/stream/Nodes$CollectorTask$OfDouble` | 3 |
| `java/util/stream/Nodes$CollectorTask$OfInt` | 3 |
| `java/util/stream/Nodes$CollectorTask$OfLong` | 3 |
| `java/util/stream/Nodes$CollectorTask$OfRef` | 4 |
| `java/util/stream/Nodes$ConcNode` | 7 |
| `java/util/stream/Nodes$ConcNode$OfDouble` | 4 |
| `java/util/stream/Nodes$ConcNode$OfInt` | 4 |
| `java/util/stream/Nodes$ConcNode$OfLong` | 4 |
| `java/util/stream/Nodes$ConcNode$OfPrimitive` | 6 |
| `java/util/stream/Nodes$DoubleArrayNode` | 13 |
| `java/util/stream/Nodes$DoubleFixedNodeBuilder` | 8 |
| `java/util/stream/Nodes$DoubleSpinedNodeBuilder` | 16 |
| `java/util/stream/Nodes$FixedNodeBuilder` | 7 |
| `java/util/stream/Nodes$IntArrayNode` | 13 |
| `java/util/stream/Nodes$IntFixedNodeBuilder` | 8 |
| `java/util/stream/Nodes$IntSpinedNodeBuilder` | 16 |
| `java/util/stream/Nodes$InternalNodeSpliterator` | 7 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfDouble` | 4 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfInt` | 4 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfLong` | 4 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfPrimitive` | 4 |
| `java/util/stream/Nodes$InternalNodeSpliterator$OfRef` | 3 |
| `java/util/stream/Nodes$LongArrayNode` | 13 |
| `java/util/stream/Nodes$LongFixedNodeBuilder` | 8 |
| `java/util/stream/Nodes$LongSpinedNodeBuilder` | 16 |
| `java/util/stream/Nodes$SizedCollectorTask` | 6 |
| `java/util/stream/Nodes$SizedCollectorTask$OfDouble` | 5 |
| `java/util/stream/Nodes$SizedCollectorTask$OfInt` | 5 |
| `java/util/stream/Nodes$SizedCollectorTask$OfLong` | 5 |
| `java/util/stream/Nodes$SizedCollectorTask$OfRef` | 5 |
| `java/util/stream/Nodes$SpinedNodeBuilder` | 10 |
| `java/util/stream/Nodes$ToArrayTask` | 5 |
| `java/util/stream/Nodes$ToArrayTask$OfDouble` | 1 |
| `java/util/stream/Nodes$ToArrayTask$OfInt` | 1 |
| `java/util/stream/Nodes$ToArrayTask$OfLong` | 1 |
| `java/util/stream/Nodes$ToArrayTask$OfPrimitive` | 5 |
| `java/util/stream/Nodes$ToArrayTask$OfRef` | 5 |
| `java/util/stream/ReduceOps` | 18 |
| `java/util/stream/ReduceOps$1` | 3 |
| `java/util/stream/ReduceOps$10` | 3 |
| `java/util/stream/ReduceOps$10ReducingSink` | 5 |
| `java/util/stream/ReduceOps$11` | 3 |
| `java/util/stream/ReduceOps$11ReducingSink` | 7 |
| `java/util/stream/ReduceOps$12` | 3 |
| `java/util/stream/ReduceOps$12ReducingSink` | 7 |
| `java/util/stream/ReduceOps$13` | 8 |
| `java/util/stream/ReduceOps$13ReducingSink` | 5 |
| `java/util/stream/ReduceOps$14` | 3 |
| `java/util/stream/ReduceOps$15` | 3 |
| `java/util/stream/ReduceOps$16` | 3 |
| `java/util/stream/ReduceOps$17` | 8 |
| `java/util/stream/ReduceOps$1ReducingSink` | 5 |
| `java/util/stream/ReduceOps$2` | 3 |
| `java/util/stream/ReduceOps$2ReducingSink` | 7 |
| `java/util/stream/ReduceOps$3` | 4 |
| `java/util/stream/ReduceOps$3ReducingSink` | 5 |
| `java/util/stream/ReduceOps$4` | 3 |
| `java/util/stream/ReduceOps$4ReducingSink` | 5 |
| `java/util/stream/ReduceOps$5` | 8 |
| `java/util/stream/ReduceOps$5ReducingSink` | 7 |
| `java/util/stream/ReduceOps$6` | 3 |
| `java/util/stream/ReduceOps$6ReducingSink` | 7 |
| `java/util/stream/ReduceOps$7` | 3 |
| `java/util/stream/ReduceOps$7ReducingSink` | 5 |
| `java/util/stream/ReduceOps$8` | 3 |
| `java/util/stream/ReduceOps$8ReducingSink` | 7 |
| `java/util/stream/ReduceOps$9` | 8 |
| `java/util/stream/ReduceOps$9ReducingSink` | 7 |
| `java/util/stream/ReduceOps$AccumulatingSink` | 1 |
| `java/util/stream/ReduceOps$Box` | 2 |
| `java/util/stream/ReduceOps$CountingSink` | 6 |
| `java/util/stream/ReduceOps$CountingSink$OfDouble` | 4 |
| `java/util/stream/ReduceOps$CountingSink$OfInt` | 4 |
| `java/util/stream/ReduceOps$CountingSink$OfLong` | 4 |
| `java/util/stream/ReduceOps$CountingSink$OfRef` | 4 |
| `java/util/stream/ReduceOps$ReduceOp` | 5 |
| `java/util/stream/ReduceOps$ReduceTask` | 7 |
| `java/util/stream/ReferencePipeline$1` | 2 |
| `java/util/stream/ReferencePipeline$10` | 2 |
| `java/util/stream/ReferencePipeline$10$1` | 4 |
| `java/util/stream/ReferencePipeline$11` | 2 |
| `java/util/stream/ReferencePipeline$11$1` | 3 |
| `java/util/stream/ReferencePipeline$12` | 2 |
| `java/util/stream/ReferencePipeline$12$1` | 3 |
| `java/util/stream/ReferencePipeline$13` | 2 |
| `java/util/stream/ReferencePipeline$13$1` | 3 |
| `java/util/stream/ReferencePipeline$14` | 2 |
| `java/util/stream/ReferencePipeline$14$1` | 3 |
| `java/util/stream/ReferencePipeline$15` | 2 |
| `java/util/stream/ReferencePipeline$15$1` | 2 |
| `java/util/stream/ReferencePipeline$2` | 2 |
| `java/util/stream/ReferencePipeline$2$1` | 3 |
| `java/util/stream/ReferencePipeline$3` | 2 |
| `java/util/stream/ReferencePipeline$3$1` | 2 |
| `java/util/stream/ReferencePipeline$4` | 2 |
| `java/util/stream/ReferencePipeline$4$1` | 2 |
| `java/util/stream/ReferencePipeline$5` | 2 |
| `java/util/stream/ReferencePipeline$5$1` | 2 |
| `java/util/stream/ReferencePipeline$6` | 2 |
| `java/util/stream/ReferencePipeline$6$1` | 2 |
| `java/util/stream/ReferencePipeline$7` | 2 |
| `java/util/stream/ReferencePipeline$7$1` | 4 |
| `java/util/stream/ReferencePipeline$8` | 2 |
| `java/util/stream/ReferencePipeline$8$1` | 4 |
| `java/util/stream/ReferencePipeline$9` | 2 |
| `java/util/stream/ReferencePipeline$9$1` | 4 |
| `java/util/stream/ReferencePipeline$StatefulOp` | 5 |
| `java/util/stream/ReferencePipeline$StatelessOp` | 4 |
| `java/util/stream/Sink` | 6 |
| `java/util/stream/Sink$ChainedDouble` | 4 |
| `java/util/stream/Sink$ChainedInt` | 4 |
| `java/util/stream/Sink$ChainedLong` | 4 |
| `java/util/stream/Sink$ChainedReference` | 4 |
| `java/util/stream/SliceOps` | 10 |
| `java/util/stream/SliceOps$1` | 6 |
| `java/util/stream/SliceOps$1$1` | 4 |
| `java/util/stream/SliceOps$2` | 7 |
| `java/util/stream/SliceOps$2$1` | 4 |
| `java/util/stream/SliceOps$3` | 7 |
| `java/util/stream/SliceOps$3$1` | 4 |
| `java/util/stream/SliceOps$4` | 7 |
| `java/util/stream/SliceOps$4$1` | 4 |
| `java/util/stream/SliceOps$SliceTask` | 13 |
| `java/util/stream/SortedOps` | 6 |
| `java/util/stream/SortedOps$AbstractDoubleSortingSink` | 2 |
| `java/util/stream/SortedOps$AbstractIntSortingSink` | 2 |
| `java/util/stream/SortedOps$AbstractLongSortingSink` | 2 |
| `java/util/stream/SortedOps$AbstractRefSortingSink` | 2 |
| `java/util/stream/SortedOps$DoubleSortingSink` | 4 |
| `java/util/stream/SortedOps$IntSortingSink` | 4 |
| `java/util/stream/SortedOps$LongSortingSink` | 4 |
| `java/util/stream/SortedOps$OfDouble` | 3 |
| `java/util/stream/SortedOps$OfInt` | 3 |
| `java/util/stream/SortedOps$OfLong` | 3 |
| `java/util/stream/SortedOps$OfRef` | 4 |
| `java/util/stream/SortedOps$RefSortingSink` | 4 |
| `java/util/stream/SortedOps$SizedDoubleSortingSink` | 4 |
| `java/util/stream/SortedOps$SizedIntSortingSink` | 4 |
| `java/util/stream/SortedOps$SizedLongSortingSink` | 4 |
| `java/util/stream/SortedOps$SizedRefSortingSink` | 4 |
| `java/util/stream/SpinedBuffer` | 15 |
| `java/util/stream/SpinedBuffer$1Splitr` | 7 |
| `java/util/stream/SpinedBuffer$OfDouble` | 18 |
| `java/util/stream/SpinedBuffer$OfDouble$1Splitr` | 10 |
| `java/util/stream/SpinedBuffer$OfInt` | 18 |
| `java/util/stream/SpinedBuffer$OfInt$1Splitr` | 10 |
| `java/util/stream/SpinedBuffer$OfLong` | 18 |
| `java/util/stream/SpinedBuffer$OfLong$1Splitr` | 10 |
| `java/util/stream/SpinedBuffer$OfPrimitive` | 18 |
| `java/util/stream/SpinedBuffer$OfPrimitive$BaseSpliterator` | 11 |
| `java/util/stream/Stream$1` | 2 |
| `java/util/stream/Stream$2` | 3 |
| `java/util/stream/StreamOpFlag$MaskBuilder` | 6 |
| `java/util/stream/StreamOpFlag$Type` | 5 |
| `java/util/stream/StreamShape` | 5 |
| `java/util/stream/StreamSpliterators$AbstractWrappingSpliterator` | 13 |
| `java/util/stream/StreamSpliterators$ArrayBuffer` | 2 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfDouble` | 4 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfInt` | 4 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfLong` | 4 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfPrimitive` | 3 |
| `java/util/stream/StreamSpliterators$ArrayBuffer$OfRef` | 3 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator` | 10 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfDouble` | 4 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfInt` | 4 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfLong` | 4 |
| `java/util/stream/StreamSpliterators$DelegatingSpliterator$OfPrimitive` | 4 |
| `java/util/stream/StreamSpliterators$DistinctSpliterator` | 12 |
| `java/util/stream/StreamSpliterators$DoubleWrappingSpliterator` | 12 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator` | 3 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfDouble` | 6 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfInt` | 6 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfLong` | 6 |
| `java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef` | 3 |
| `java/util/stream/StreamSpliterators$IntWrappingSpliterator` | 12 |
| `java/util/stream/StreamSpliterators$LongWrappingSpliterator` | 12 |
| `java/util/stream/StreamSpliterators$SliceSpliterator` | 6 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfDouble` | 10 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfInt` | 10 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfLong` | 10 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfPrimitive` | 6 |
| `java/util/stream/StreamSpliterators$SliceSpliterator$OfRef` | 7 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator` | 9 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfDouble` | 12 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfInt` | 12 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfLong` | 12 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfPrimitive` | 7 |
| `java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfRef` | 6 |
| `java/util/stream/StreamSpliterators$WrappingSpliterator` | 8 |
| `java/util/stream/Streams` | 3 |
| `java/util/stream/Streams$1` | 2 |
| `java/util/stream/Streams$2` | 2 |
| `java/util/stream/Streams$ConcatSpliterator` | 7 |
| `java/util/stream/Streams$ConcatSpliterator$OfDouble` | 4 |
| `java/util/stream/Streams$ConcatSpliterator$OfInt` | 4 |
| `java/util/stream/Streams$ConcatSpliterator$OfLong` | 4 |
| `java/util/stream/Streams$ConcatSpliterator$OfPrimitive` | 4 |
| `java/util/stream/Streams$ConcatSpliterator$OfRef` | 1 |
| `java/util/stream/Streams$DoubleStreamBuilderImpl` | 10 |
| `java/util/stream/Streams$IntStreamBuilderImpl` | 10 |
| `java/util/stream/Streams$LongStreamBuilderImpl` | 10 |
| `java/util/stream/Streams$RangeIntSpliterator` | 13 |
| `java/util/stream/Streams$RangeLongSpliterator` | 14 |
| `java/util/stream/TerminalOp` | 4 |
| `java/util/stream/Tripwire` | 4 |
| `java/util/stream/WhileOps` | 10 |
| `java/util/stream/WhileOps$1` | 4 |
| `java/util/stream/WhileOps$1$1` | 4 |
| `java/util/stream/WhileOps$1Op` | 5 |
| `java/util/stream/WhileOps$1Op$1OpSink` | 3 |
| `java/util/stream/WhileOps$2` | 5 |
| `java/util/stream/WhileOps$2$1` | 4 |
| `java/util/stream/WhileOps$2Op` | 6 |
| `java/util/stream/WhileOps$2Op$1OpSink` | 3 |
| `java/util/stream/WhileOps$3` | 5 |
| `java/util/stream/WhileOps$3$1` | 4 |
| `java/util/stream/WhileOps$3Op` | 6 |
| `java/util/stream/WhileOps$3Op$1OpSink` | 3 |
| `java/util/stream/WhileOps$4` | 5 |
| `java/util/stream/WhileOps$4$1` | 4 |
| `java/util/stream/WhileOps$4Op` | 6 |
| `java/util/stream/WhileOps$4Op$1OpSink` | 3 |
| `java/util/stream/WhileOps$DropWhileOp` | 1 |
| `java/util/stream/WhileOps$DropWhileSink` | 1 |
| `java/util/stream/WhileOps$DropWhileTask` | 10 |
| `java/util/stream/WhileOps$TakeWhileTask` | 11 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator` | 9 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble` | 5 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble$Dropping` | 8 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble$Taking` | 9 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt` | 5 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Dropping` | 8 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Taking` | 9 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong` | 5 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong$Dropping` | 8 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong$Taking` | 9 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef` | 3 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Dropping` | 4 |
| `java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking` | 5 |
| `java/util/zip/DataFormatException` | 2 |
| `java/util/zip/ZipCoder` | 16 |
| `java/util/zip/ZipCoder$Comparison` | 5 |
| `java/util/zip/ZipCoder$UTF8ZipCoder` | 7 |
| `java/util/zip/ZipEntry` | 30 |
| `java/util/zip/ZipException` | 2 |
| `java/util/zip/ZipFile` | 29 |
| `java/util/zip/ZipFile$CleanableResource` | 5 |
| `java/util/zip/ZipFile$EntrySpliterator` | 2 |
| `java/util/zip/ZipFile$InflaterCleanupAction` | 2 |
| `java/util/zip/ZipFile$Source` | 26 |
| `java/util/zip/ZipFile$Source$End` | 1 |
| `java/util/zip/ZipFile$Source$Key` | 3 |
| `java/util/zip/ZipFile$ZipEntryIterator` | 8 |
| `java/util/zip/ZipFile$ZipFileInflaterInputStream` | 5 |
| `java/util/zip/ZipFile$ZipFileInputStream` | 9 |
| `java/util/zip/ZipInputStream` | 19 |
| `java/util/zip/ZipUtils` | 66 |
| `javax/crypto/Cipher` | 58 |
| `javax/crypto/Cipher$Transform` | 8 |
| `javax/crypto/CipherSpi` | 23 |
| `javax/crypto/CryptoPermission` | 20 |
| `javax/crypto/CryptoPermissionCollection` | 4 |
| `javax/crypto/CryptoPermissions` | 14 |
| `javax/crypto/CryptoPolicyParser` | 11 |
| `javax/crypto/CryptoPolicyParser$CryptoPermissionEntry` | 3 |
| `javax/crypto/CryptoPolicyParser$GrantEntry` | 2 |
| `javax/crypto/CryptoPolicyParser$ParsingException` | 3 |
| `javax/crypto/ExemptionMechanism` | 14 |
| `javax/crypto/ExemptionMechanismException` | 2 |
| `javax/crypto/ExemptionMechanismSpi` | 7 |
| `javax/crypto/JceSecurity` | 15 |
| `javax/crypto/JceSecurity$2` | 3 |
| `javax/crypto/JceSecurity$3` | 3 |
| `javax/crypto/JceSecurity$WeakIdentityWrapper` | 3 |
| `javax/crypto/JceSecurityManager` | 11 |
| `javax/crypto/NoSuchPaddingException` | 2 |
| `javax/crypto/NullCipher` | 1 |
| `javax/crypto/NullCipherSpi` | 15 |
| `javax/crypto/PermissionsEnumerator` | 5 |
| `javax/crypto/ProviderVerifier` | 7 |
| `javax/crypto/SecretKey` | 0 |
| `javax/crypto/ShortBufferException` | 2 |
| `javax/crypto/interfaces/DHKey` | 1 |
| `javax/crypto/interfaces/DHPublicKey` | 1 |
| `javax/crypto/spec/DHParameterSpec` | 5 |
| `javax/crypto/spec/DHPublicKeySpec` | 4 |
| `javax/crypto/spec/PBEParameterSpec` | 5 |
| `javax/crypto/spec/RC2ParameterSpec` | 7 |
| `javax/crypto/spec/RC5ParameterSpec` | 9 |
| `javax/net/ssl/SNIHostName` | 8 |
| `javax/net/ssl/SNIHostName$SNIHostNameMatcher` | 2 |
| `javax/net/ssl/SNIMatcher` | 3 |
| `javax/net/ssl/SNIServerName` | 7 |
| `javax/net/ssl/SSLSession` | 21 |
| `javax/security/auth/AuthPermission` | 2 |
| `javax/security/auth/PrivateCredentialPermission` | 14 |
| `javax/security/auth/PrivateCredentialPermission$CredOwner` | 3 |
| `javax/security/auth/Subject` | 28 |
| `javax/security/auth/Subject$1` | 3 |
| `javax/security/auth/Subject$2` | 3 |
| `javax/security/auth/Subject$ClassSet` | 5 |
| `javax/security/auth/Subject$ClassSet$1` | 2 |
| `javax/security/auth/Subject$SecureSet` | 20 |
| `javax/security/auth/Subject$SecureSet$1` | 4 |
| `javax/security/auth/Subject$SecureSet$2` | 2 |
| `javax/security/auth/Subject$SecureSet$3` | 2 |
| `javax/security/auth/Subject$SecureSet$4` | 2 |
| `javax/security/auth/Subject$SecureSet$5` | 2 |
| `javax/security/auth/Subject$SecureSet$6` | 2 |
| `javax/security/auth/SubjectDomainCombiner` | 7 |
| `javax/security/auth/SubjectDomainCombiner$1` | 3 |
| `javax/security/auth/SubjectDomainCombiner$2` | 3 |
| `javax/security/auth/SubjectDomainCombiner$WeakKeyValueMap` | 3 |
| `sun/invoke/util/BytecodeName` | 22 |
| `sun/net/InetAddressCachePolicy` | 9 |
| `sun/net/InetAddressCachePolicy$1` | 3 |
| `sun/net/NetHooks` | 4 |
| `sun/net/NetHooks$Provider` | 3 |
| `sun/net/NetProperties` | 6 |
| `sun/net/PlatformSocketImpl` | 0 |
| `sun/net/PortConfig` | 6 |
| `sun/net/ResolverProviderConfiguration` | 3 |
| `sun/net/ResourceManager` | 4 |
| `sun/net/SocksProxy` | 3 |
| `sun/net/ext/ExtendedSocketOptions` | 16 |
| `sun/net/ext/ExtendedSocketOptions$NoExtendedSocketOptions` | 3 |
| `sun/net/spi/DefaultProxySelector` | 11 |
| `sun/net/spi/DefaultProxySelector$2` | 3 |
| `sun/net/spi/DefaultProxySelector$3` | 3 |
| `sun/net/util/SocketExceptions` | 6 |
| `sun/net/util/SocketExceptions$1` | 3 |
| `sun/net/www/MessageHeader` | 31 |
| `sun/net/www/MessageHeader$HeaderIterator` | 5 |
| `sun/net/www/MimeEntry` | 23 |
| `sun/net/www/MimeTable` | 24 |
| `sun/net/www/URLConnection` | 20 |
| `sun/net/www/protocol/file/FileURLConnection` | 15 |
| `sun/net/www/protocol/jar/JarFileFactory` | 13 |
| `sun/net/www/protocol/jar/JarURLConnection` | 24 |
| `sun/net/www/protocol/jar/JarURLConnection$JarURLInputStream` | 2 |
| `sun/net/www/protocol/jar/URLJarFile` | 14 |
| `sun/net/www/protocol/jar/URLJarFile$1` | 3 |
| `sun/net/www/protocol/jar/URLJarFile$URLJarFileCloseController` | 1 |
| `sun/net/www/protocol/jar/URLJarFile$URLJarFileEntry` | 4 |
| `sun/net/www/protocol/jar/URLJarFileCallBack` | 1 |
| `sun/nio/ch/AllocatedNativeObject` | 2 |
| `sun/nio/ch/AsynchronousChannelGroupImpl` | 28 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$1` | 2 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$2` | 3 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$3` | 3 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$4` | 2 |
| `sun/nio/ch/AsynchronousChannelGroupImpl$4$1` | 3 |
| `sun/nio/ch/AsynchronousFileChannelImpl` | 21 |
| `sun/nio/ch/Cancellable` | 1 |
| `sun/nio/ch/ChannelInputStream` | 12 |
| `sun/nio/ch/ChannelOutputStream` | 6 |
| `sun/nio/ch/CompletedFuture` | 9 |
| `sun/nio/ch/DefaultPollerProvider` | 3 |
| `sun/nio/ch/DirectBuffer` | 3 |
| `sun/nio/ch/DummySocketImpl` | 23 |
| `sun/nio/ch/FileChannelImpl` | 46 |
| `sun/nio/ch/FileChannelImpl$1` | 2 |
| `sun/nio/ch/FileChannelImpl$2` | 5 |
| `sun/nio/ch/FileChannelImpl$3` | 5 |
| `sun/nio/ch/FileChannelImpl$Closer` | 2 |
| `sun/nio/ch/FileChannelImpl$DefaultUnmapper` | 4 |
| `sun/nio/ch/FileChannelImpl$SyncUnmapper` | 4 |
| `sun/nio/ch/FileChannelImpl$Unmapper` | 9 |
| `sun/nio/ch/FileDispatcher` | 18 |
| `sun/nio/ch/FileKey` | 7 |
| `sun/nio/ch/FileLockImpl` | 6 |
| `sun/nio/ch/FileLockTable` | 9 |
| `sun/nio/ch/FileLockTable$FileLockReference` | 2 |
| `sun/nio/ch/Groupable` | 1 |
| `sun/nio/ch/IOStatus` | 7 |
| `sun/nio/ch/IOUtil` | 41 |
| `sun/nio/ch/IOUtil$LinkedRunnable` | 8 |
| `sun/nio/ch/IOUtil$Releaser` | 8 |
| `sun/nio/ch/IOVecWrapper` | 12 |
| `sun/nio/ch/IOVecWrapper$Deallocator` | 2 |
| `sun/nio/ch/Invoker` | 15 |
| `sun/nio/ch/Invoker$2` | 2 |
| `sun/nio/ch/Invoker$3` | 2 |
| `sun/nio/ch/Invoker$GroupAndInvokeCount` | 6 |
| `sun/nio/ch/KQueue` | 15 |
| `sun/nio/ch/KQueuePoller` | 5 |
| `sun/nio/ch/NativeDispatcher` | 13 |
| `sun/nio/ch/NativeObject` | 26 |
| `sun/nio/ch/NativeThread` | 12 |
| `sun/nio/ch/NativeThreadSet` | 5 |
| `sun/nio/ch/Net` | 95 |
| `sun/nio/ch/Net$2` | 3 |
| `sun/nio/ch/NioSocketImpl` | 56 |
| `sun/nio/ch/NioSocketImpl$1` | 5 |
| `sun/nio/ch/NioSocketImpl$2` | 4 |
| `sun/nio/ch/OptionKey` | 3 |
| `sun/nio/ch/PendingFuture` | 21 |
| `sun/nio/ch/Poller` | 33 |
| `sun/nio/ch/Poller$Request` | 4 |
| `sun/nio/ch/PollerProvider` | 5 |
| `sun/nio/ch/SelChImpl` | 8 |
| `sun/nio/ch/SelectionKeyImpl` | 26 |
| `sun/nio/ch/SelectorImpl` | 24 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl` | 12 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl$1` | 2 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl$2` | 2 |
| `sun/nio/ch/SimpleAsynchronousFileChannelImpl$3` | 2 |
| `sun/nio/ch/SinkChannelImpl` | 22 |
| `sun/nio/ch/SocketAdaptor` | 53 |
| `sun/nio/ch/SocketChannelImpl` | 69 |
| `sun/nio/ch/SocketInputStream` | 7 |
| `sun/nio/ch/SocketOptionRegistry` | 2 |
| `sun/nio/ch/SocketOptionRegistry$RegistryKey` | 3 |
| `sun/nio/ch/SocketOutputStream` | 5 |
| `sun/nio/ch/Streams` | 3 |
| `sun/nio/ch/ThreadPool` | 14 |
| `sun/nio/ch/UnixDomainSockets` | 23 |
| `sun/nio/ch/UnixDomainSocketsUtil` | 4 |
| `sun/nio/ch/Util` | 25 |
| `sun/nio/ch/Util$2` | 17 |
| `sun/nio/ch/Util$3` | 3 |
| `sun/nio/ch/Util$4` | 3 |
| `sun/nio/ch/Util$BufferCache` | 8 |
| `sun/nio/cs/CESU_8` | 6 |
| `sun/nio/cs/CESU_8$Decoder` | 17 |
| `sun/nio/cs/CESU_8$Encoder` | 11 |
| `sun/nio/cs/HistoricallyNamedCharset` | 1 |
| `sun/nio/cs/ISO_8859_1` | 6 |
| `sun/nio/cs/ISO_8859_1$Decoder` | 5 |
| `sun/nio/cs/ISO_8859_1$Encoder` | 10 |
| `sun/nio/cs/ISO_8859_15` | 5 |
| `sun/nio/cs/ISO_8859_16` | 5 |
| `sun/nio/cs/MS1252` | 5 |
| `sun/nio/cs/SingleByte` | 4 |
| `sun/nio/cs/SingleByte$Decoder` | 12 |
| `sun/nio/cs/SingleByte$Encoder` | 12 |
| `sun/nio/cs/StandardCharsets$1` | 5 |
| `sun/nio/cs/StandardCharsets$Aliases` | 2 |
| `sun/nio/cs/StandardCharsets$Cache` | 2 |
| `sun/nio/cs/StandardCharsets$Classes` | 2 |
| `sun/nio/cs/StringUTF16` | 3 |
| `sun/nio/cs/Surrogate$Parser` | 9 |
| `sun/nio/cs/ThreadLocalCoders` | 4 |
| `sun/nio/cs/ThreadLocalCoders$Cache` | 5 |
| `sun/nio/cs/US_ASCII` | 6 |
| `sun/nio/cs/US_ASCII$Decoder` | 4 |
| `sun/nio/cs/US_ASCII$Encoder` | 7 |
| `sun/nio/cs/UTF_16` | 5 |
| `sun/nio/cs/UTF_16$Decoder` | 1 |
| `sun/nio/cs/UTF_16$Encoder` | 1 |
| `sun/nio/cs/UTF_16BE` | 5 |
| `sun/nio/cs/UTF_16BE$Decoder` | 1 |
| `sun/nio/cs/UTF_16BE$Encoder` | 1 |
| `sun/nio/cs/UTF_16LE` | 5 |
| `sun/nio/cs/UTF_16LE$Decoder` | 1 |
| `sun/nio/cs/UTF_16LE$Encoder` | 1 |
| `sun/nio/cs/UTF_16LE_BOM` | 4 |
| `sun/nio/cs/UTF_16LE_BOM$Decoder` | 1 |
| `sun/nio/cs/UTF_16LE_BOM$Encoder` | 1 |
| `sun/nio/cs/UTF_32` | 5 |
| `sun/nio/cs/UTF_32BE_BOM` | 5 |
| `sun/nio/cs/UTF_32Coder$Decoder` | 4 |
| `sun/nio/cs/UTF_32Coder$Encoder` | 4 |
| `sun/nio/cs/UTF_32LE` | 5 |
| `sun/nio/cs/UTF_32LE_BOM` | 5 |
| `sun/nio/cs/UnicodeDecoder` | 5 |
| `sun/nio/cs/UnicodeEncoder` | 5 |
| `sun/nio/fs/AbstractBasicFileAttributeView` | 6 |
| `sun/nio/fs/AbstractBasicFileAttributeView$AttributesBuilder` | 5 |
| `sun/nio/fs/AbstractFileSystemProvider` | 12 |
| `sun/nio/fs/AbstractFileTypeDetector` | 7 |
| `sun/nio/fs/AbstractUserDefinedFileAttributeView` | 6 |
| `sun/nio/fs/AbstractWatchKey` | 9 |
| `sun/nio/fs/AbstractWatchKey$Event` | 5 |
| `sun/nio/fs/AbstractWatchService` | 12 |
| `sun/nio/fs/AbstractWatchService$1` | 3 |
| `sun/nio/fs/BasicFileAttributesHolder` | 2 |
| `sun/nio/fs/BsdFileAttributeViews` | 6 |
| `sun/nio/fs/BsdFileAttributeViews$Basic` | 2 |
| `sun/nio/fs/BsdFileAttributeViews$Posix` | 2 |
| `sun/nio/fs/BsdFileAttributeViews$Unix` | 2 |
| `sun/nio/fs/BsdFileStore` | 5 |
| `sun/nio/fs/BsdFileSystem` | 10 |
| `sun/nio/fs/BsdFileSystemProvider` | 7 |
| `sun/nio/fs/BsdNativeDispatcher` | 14 |
| `sun/nio/fs/BsdUserDefinedFileAttributeView` | 2 |
| `sun/nio/fs/Cancellable` | 9 |
| `sun/nio/fs/DefaultFileSystemProvider` | 4 |
| `sun/nio/fs/DynamicFileAttributeView` | 2 |
| `sun/nio/fs/ExtendedOptions$InternalOption` | 8 |
| `sun/nio/fs/ExtendedOptions$Wrapper` | 2 |
| `sun/nio/fs/FileOwnerAttributeViewImpl` | 7 |
| `sun/nio/fs/Globs` | 8 |
| `sun/nio/fs/MacOSXFileSystem` | 6 |
| `sun/nio/fs/MacOSXFileSystemProvider` | 5 |
| `sun/nio/fs/MacOSXNativeDispatcher` | 2 |
| `sun/nio/fs/MimeTypesFileTypeDetector` | 5 |
| `sun/nio/fs/MimeTypesFileTypeDetector$1` | 3 |
| `sun/nio/fs/NativeBuffer` | 9 |
| `sun/nio/fs/NativeBuffer$Deallocator` | 2 |
| `sun/nio/fs/NativeBuffers` | 8 |
| `sun/nio/fs/PollingWatchService` | 4 |
| `sun/nio/fs/PollingWatchService$1` | 2 |
| `sun/nio/fs/PollingWatchService$2` | 3 |
| `sun/nio/fs/PollingWatchService$3` | 3 |
| `sun/nio/fs/PollingWatchService$CacheEntry` | 4 |
| `sun/nio/fs/PollingWatchService$PollingWatchKey` | 8 |
| `sun/nio/fs/PollingWatchService$PollingWatchKey$1` | 2 |
| `sun/nio/fs/UTIFileTypeDetector` | 4 |
| `sun/nio/fs/UnixChannelFactory` | 6 |
| `sun/nio/fs/UnixChannelFactory$Flags` | 2 |
| `sun/nio/fs/UnixDirectoryStream` | 9 |
| `sun/nio/fs/UnixDirectoryStream$UnixDirectoryIterator` | 8 |
| `sun/nio/fs/UnixException` | 11 |
| `sun/nio/fs/UnixFileAttributeViews` | 5 |
| `sun/nio/fs/UnixFileAttributeViews$Basic` | 4 |
| `sun/nio/fs/UnixFileAttributeViews$Posix` | 17 |
| `sun/nio/fs/UnixFileAttributeViews$Unix` | 5 |
| `sun/nio/fs/UnixFileAttributes` | 30 |
| `sun/nio/fs/UnixFileAttributes$UnixAsBasicFileAttributes` | 11 |
| `sun/nio/fs/UnixFileKey` | 4 |
| `sun/nio/fs/UnixFileModeAttribute` | 3 |
| `sun/nio/fs/UnixFileStore` | 27 |
| `sun/nio/fs/UnixFileStore$1` | 3 |
| `sun/nio/fs/UnixFileStoreAttributes` | 6 |
| `sun/nio/fs/UnixFileSystem` | 40 |
| `sun/nio/fs/UnixFileSystem$1` | 2 |
| `sun/nio/fs/UnixFileSystem$2` | 2 |
| `sun/nio/fs/UnixFileSystem$3` | 2 |
| `sun/nio/fs/UnixFileSystem$4` | 2 |
| `sun/nio/fs/UnixFileSystem$FileStoreIterator` | 7 |
| `sun/nio/fs/UnixFileSystem$Flags` | 3 |
| `sun/nio/fs/UnixFileSystemProvider` | 39 |
| `sun/nio/fs/UnixFileSystemProvider$1` | 2 |
| `sun/nio/fs/UnixFileSystemProvider$2` | 2 |
| `sun/nio/fs/UnixMountEntry` | 8 |
| `sun/nio/fs/UnixNativeDispatcher` | 98 |
| `sun/nio/fs/UnixPath` | 55 |
| `sun/nio/fs/UnixSecureDirectoryStream` | 22 |
| `sun/nio/fs/UnixSecureDirectoryStream$BasicFileAttributeViewImpl` | 7 |
| `sun/nio/fs/UnixSecureDirectoryStream$PosixFileAttributeViewImpl` | 12 |
| `sun/nio/fs/UnixUriUtils` | 10 |
| `sun/nio/fs/UnixUserDefinedFileAttributeView` | 22 |
| `sun/nio/fs/UnixUserPrincipals` | 8 |
| `sun/nio/fs/UnixUserPrincipals$Group` | 1 |
| `sun/nio/fs/UnixUserPrincipals$User` | 8 |
| `sun/nio/fs/Util` | 9 |
| `sun/reflect/annotation/AnnotatedTypeFactory` | 5 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedArrayTypeImpl` | 7 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedParameterizedTypeImpl` | 7 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedTypeBaseImpl` | 17 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedTypeVariableImpl` | 5 |
| `sun/reflect/annotation/AnnotatedTypeFactory$AnnotatedWildcardTypeImpl` | 10 |
| `sun/reflect/annotation/AnnotationInvocationHandler` | 32 |
| `sun/reflect/annotation/AnnotationInvocationHandler$1` | 3 |
| `sun/reflect/annotation/AnnotationInvocationHandler$UnsafeAccessor` | 4 |
| `sun/reflect/annotation/AnnotationParser` | 42 |
| `sun/reflect/annotation/AnnotationParser$1` | 3 |
| `sun/reflect/annotation/AnnotationSupport` | 10 |
| `sun/reflect/annotation/AnnotationSupport$1` | 3 |
| `sun/reflect/annotation/AnnotationType` | 10 |
| `sun/reflect/annotation/AnnotationType$1` | 3 |
| `sun/reflect/annotation/AnnotationTypeMismatchExceptionProxy` | 4 |
| `sun/reflect/annotation/EnumConstantNotPresentExceptionProxy` | 3 |
| `sun/reflect/annotation/ExceptionProxy` | 2 |
| `sun/reflect/annotation/TypeAnnotation` | 7 |
| `sun/reflect/annotation/TypeAnnotation$LocationInfo` | 12 |
| `sun/reflect/annotation/TypeAnnotation$LocationInfo$Location` | 2 |
| `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTargetInfo` | 7 |
| `sun/reflect/annotation/TypeAnnotationParser` | 17 |
| `sun/reflect/annotation/TypeNotPresentExceptionProxy` | 5 |
| `sun/reflect/generics/factory/GenericsFactory` | 15 |
| `sun/reflect/generics/parser/SignatureParser` | 40 |
| `sun/reflect/generics/reflectiveObjects/GenericArrayTypeImpl` | 6 |
| `sun/reflect/generics/reflectiveObjects/LazyReflectiveObjectGenerator` | 4 |
| `sun/reflect/generics/reflectiveObjects/ParameterizedTypeImpl` | 10 |
| `sun/reflect/generics/reflectiveObjects/TypeVariableImpl` | 18 |
| `sun/reflect/generics/reflectiveObjects/WildcardTypeImpl` | 8 |
| `sun/reflect/generics/repository/ConstructorRepository` | 8 |
| `sun/reflect/generics/repository/FieldRepository` | 6 |
| `sun/reflect/generics/repository/MethodRepository` | 4 |
| `sun/reflect/generics/scope/ConstructorScope` | 4 |
| `sun/reflect/generics/scope/DummyScope` | 4 |
| `sun/reflect/generics/scope/MethodScope` | 4 |
| `sun/reflect/generics/scope/Scope` | 1 |
| `sun/reflect/generics/tree/ArrayTypeSignature` | 4 |
| `sun/reflect/generics/tree/BooleanSignature` | 4 |
| `sun/reflect/generics/tree/BottomSignature` | 4 |
| `sun/reflect/generics/tree/ByteSignature` | 4 |
| `sun/reflect/generics/tree/CharSignature` | 4 |
| `sun/reflect/generics/tree/ClassTypeSignature` | 4 |
| `sun/reflect/generics/tree/DoubleSignature` | 4 |
| `sun/reflect/generics/tree/FieldTypeSignature` | 0 |
| `sun/reflect/generics/tree/FloatSignature` | 4 |
| `sun/reflect/generics/tree/FormalTypeParameter` | 5 |
| `sun/reflect/generics/tree/IntSignature` | 4 |
| `sun/reflect/generics/tree/LongSignature` | 4 |
| `sun/reflect/generics/tree/MethodTypeSignature` | 7 |
| `sun/reflect/generics/tree/ReturnType` | 0 |
| `sun/reflect/generics/tree/ShortSignature` | 4 |
| `sun/reflect/generics/tree/Signature` | 1 |
| `sun/reflect/generics/tree/SimpleClassTypeSignature` | 6 |
| `sun/reflect/generics/tree/TypeArgument` | 0 |
| `sun/reflect/generics/tree/TypeSignature` | 0 |
| `sun/reflect/generics/tree/TypeVariableSignature` | 4 |
| `sun/reflect/generics/tree/VoidDescriptor` | 4 |
| `sun/reflect/generics/tree/Wildcard` | 6 |
| `sun/reflect/generics/visitor/TypeTreeVisitor` | 17 |
| `sun/reflect/generics/visitor/Visitor` | 2 |
| `sun/security/action/GetIntegerAction` | 6 |
| `sun/security/jca/JCAUtil` | 7 |
| `sun/security/jca/ProviderConfig$ProviderLoader` | 4 |
| `sun/security/jca/ProviderConfig$ProviderLoader$1` | 3 |
| `sun/security/jca/ProviderList$2` | 3 |
| `sun/security/jca/ProviderList$ServiceList$1` | 5 |
| `sun/security/pkcs/ContentInfo` | 11 |
| `sun/security/pkcs/PKCS7` | 29 |
| `sun/security/pkcs/PKCS9Attribute` | 16 |
| `sun/security/pkcs/PKCS9Attributes` | 14 |
| `sun/security/pkcs/ParsingException` | 2 |
| `sun/security/pkcs/SignerInfo` | 25 |
| `sun/security/pkcs/SignerInfo$AlgorithmInfo` | 6 |
| `sun/security/pkcs/SigningCertificateInfo` | 4 |
| `sun/security/pkcs/SigningCertificateInfo$ESSCertId` | 2 |
| `sun/security/provider/ByteArrayAccess` | 12 |
| `sun/security/provider/DigestBase` | 15 |
| `sun/security/provider/FileInputStreamPool` | 3 |
| `sun/security/provider/FileInputStreamPool$StreamRef` | 1 |
| `sun/security/provider/FileInputStreamPool$UnclosableInputStream` | 3 |
| `sun/security/provider/NativePRNG$1` | 3 |
| `sun/security/provider/NativePRNG$RandomIO` | 7 |
| `sun/security/provider/NativePRNG$RandomIO$1` | 3 |
| `sun/security/provider/NativePRNG$Variant` | 5 |
| `sun/security/provider/PolicyFile$6` | 3 |
| `sun/security/provider/PolicyFile$7` | 3 |
| `sun/security/provider/PolicyFile$8` | 3 |
| `sun/security/provider/SHA3` | 12 |
| `sun/security/provider/SHAKE256` | 6 |
| `sun/security/provider/SecureRandom` | 8 |
| `sun/security/provider/SeedGenerator` | 7 |
| `sun/security/provider/SeedGenerator$1` | 3 |
| `sun/security/provider/VerificationProvider` | 3 |
| `sun/security/provider/VerificationProvider$1` | 3 |
| `sun/security/provider/X509Factory` | 21 |
| `sun/security/provider/certpath/X509CertPath` | 14 |
| `sun/security/provider/certpath/X509CertificatePair` | 15 |
| `sun/security/rsa/RSAUtil` | 9 |
| `sun/security/rsa/RSAUtil$KeyType` | 6 |
| `sun/security/ssl/SSLLogger` | 14 |
| `sun/security/ssl/SSLLogger$SSLSimpleFormatter` | 14 |
| `sun/security/ssl/SSLScope` | 6 |
| `sun/security/ssl/Utilities` | 14 |
| `sun/security/timestamp/HttpTimestamper` | 4 |
| `sun/security/timestamp/TSRequest` | 8 |
| `sun/security/timestamp/TSResponse` | 12 |
| `sun/security/timestamp/TSResponse$TimestampException` | 1 |
| `sun/security/timestamp/TimestampToken` | 8 |
| `sun/security/timestamp/Timestamper` | 1 |
| `sun/security/util/AbstractAlgorithmConstraints` | 4 |
| `sun/security/util/AbstractAlgorithmConstraints$1` | 3 |
| `sun/security/util/AlgorithmDecomposer` | 7 |
| `sun/security/util/AnchorCertificates` | 4 |
| `sun/security/util/ArrayUtil` | 5 |
| `sun/security/util/Cache` | 17 |
| `sun/security/util/Cache$CacheVisitor` | 1 |
| `sun/security/util/Cache$EqualByteArray` | 3 |
| `sun/security/util/ConstraintsParameters` | 5 |
| `sun/security/util/CurveDB` | 7 |
| `sun/security/util/DisabledAlgorithmConstraints` | 20 |
| `sun/security/util/DisabledAlgorithmConstraints$Constraint` | 7 |
| `sun/security/util/DisabledAlgorithmConstraints$Constraint$Operator` | 6 |
| `sun/security/util/DisabledAlgorithmConstraints$Constraints` | 5 |
| `sun/security/util/DisabledAlgorithmConstraints$DenyAfterConstraint` | 3 |
| `sun/security/util/DisabledAlgorithmConstraints$DisabledConstraint` | 3 |
| `sun/security/util/DisabledAlgorithmConstraints$KeySizeConstraint` | 5 |
| `sun/security/util/DisabledAlgorithmConstraints$UsageConstraint` | 3 |
| `sun/security/util/DisabledAlgorithmConstraints$jdkCAConstraint` | 2 |
| `sun/security/util/DomainName` | 4 |
| `sun/security/util/DomainName$CommonMatch` | 3 |
| `sun/security/util/DomainName$Match` | 2 |
| `sun/security/util/DomainName$OtherMatch` | 4 |
| `sun/security/util/DomainName$OtherRule` | 1 |
| `sun/security/util/DomainName$RegisteredDomainImpl` | 4 |
| `sun/security/util/DomainName$Rule` | 1 |
| `sun/security/util/DomainName$Rule$Type` | 5 |
| `sun/security/util/DomainName$Rules` | 9 |
| `sun/security/util/DomainName$Rules$1` | 3 |
| `sun/security/util/DomainName$Rules$RuleSet` | 11 |
| `sun/security/util/ECKeySizeParameterSpec` | 2 |
| `sun/security/util/ECUtil` | 22 |
| `sun/security/util/JarConstraintsParameters` | 11 |
| `sun/security/util/KeyUtil` | 12 |
| `sun/security/util/Length` | 1 |
| `sun/security/util/ManifestDigester` | 9 |
| `sun/security/util/ManifestDigester$Entry` | 6 |
| `sun/security/util/ManifestDigester$Position` | 1 |
| `sun/security/util/ManifestDigester$Section` | 3 |
| `sun/security/util/ManifestEntryVerifier` | 9 |
| `sun/security/util/MemoryCache` | 20 |
| `sun/security/util/MemoryCache$CacheEntry` | 5 |
| `sun/security/util/MemoryCache$HardCacheEntry` | 6 |
| `sun/security/util/MemoryCache$QueueCacheEntry` | 13 |
| `sun/security/util/MemoryCache$SoftCacheEntry` | 7 |
| `sun/security/util/MessageDigestSpi2` | 1 |
| `sun/security/util/NamedCurve` | 5 |
| `sun/security/util/RegisteredDomain` | 4 |
| `sun/security/util/SafeDHParameterSpec` | 2 |
| `sun/security/util/SecurityProperties` | 6 |
| `sun/security/util/SignatureFileVerifier` | 23 |
| `sun/security/util/SignatureUtil` | 20 |
| `sun/security/x509/AccessDescription` | 9 |
| `sun/security/x509/AuthorityInfoAccessExtension` | 7 |
| `sun/security/x509/AuthorityKeyIdentifierExtension` | 10 |
| `sun/security/x509/BasicConstraintsExtension` | 9 |
| `sun/security/x509/CRLDistributionPointsExtension` | 11 |
| `sun/security/x509/CRLExtensions` | 14 |
| `sun/security/x509/CRLNumberExtension` | 11 |
| `sun/security/x509/CRLReasonCodeExtension` | 10 |
| `sun/security/x509/CertificateIssuerExtension` | 7 |
| `sun/security/x509/CertificatePoliciesExtension` | 8 |
| `sun/security/x509/CertificatePolicyId` | 7 |
| `sun/security/x509/CertificatePolicyMap` | 6 |
| `sun/security/x509/DeltaCRLIndicatorExtension` | 4 |
| `sun/security/x509/DistributionPoint` | 13 |
| `sun/security/x509/DistributionPointName` | 9 |
| `sun/security/x509/ExtendedKeyUsageExtension` | 9 |
| `sun/security/x509/GeneralSubtree` | 9 |
| `sun/security/x509/GeneralSubtrees` | 22 |
| `sun/security/x509/IssuerAlternativeNameExtension` | 8 |
| `sun/security/x509/IssuingDistributionPointExtension` | 13 |
| `sun/security/x509/KeyIdentifier` | 8 |
| `sun/security/x509/KeyUsageExtension` | 14 |
| `sun/security/x509/NameConstraintsExtension` | 14 |
| `sun/security/x509/PolicyConstraintsExtension` | 9 |
| `sun/security/x509/PolicyInformation` | 8 |
| `sun/security/x509/PolicyMappingsExtension` | 7 |
| `sun/security/x509/PrivateKeyUsageExtension` | 10 |
| `sun/security/x509/ReasonFlags` | 14 |
| `sun/security/x509/SubjectKeyIdentifierExtension` | 7 |
| `sun/security/x509/X509CRLEntryImpl` | 27 |
| `sun/security/x509/X509CRLImpl` | 47 |
| `sun/security/x509/X509CRLImpl$TBSCertList` | 7 |
| `sun/security/x509/X509CRLImpl$X509IssuerSerial` | 8 |
| `sun/text/BreakDictionary` | 7 |
| `sun/text/CollatorUtilities` | 4 |
| `sun/text/CompactByteArray` | 15 |
| `sun/text/ComposedCharIter` | 4 |
| `sun/text/DictionaryBasedBreakIterator` | 11 |
| `sun/text/IntHashtable` | 18 |
| `sun/text/RuleBasedBreakIterator` | 32 |
| `sun/text/RuleBasedBreakIterator$SafeCharIterator` | 11 |
| `sun/text/SupplementaryCharacterData` | 4 |
| `sun/text/UCompactIntArray` | 8 |
| `sun/text/spi/JavaTimeDateTimePatternProvider` | 3 |
| `sun/util/BuddhistCalendar` | 16 |
| `sun/util/PreHashedMap` | 8 |
| `sun/util/PreHashedMap$1` | 3 |
| `sun/util/PreHashedMap$1$1` | 6 |
| `sun/util/PreHashedMap$2` | 3 |
| `sun/util/PreHashedMap$2$1` | 5 |
| `sun/util/PreHashedMap$2$1$1` | 7 |
| `sun/util/PropertyResourceBundleCharset` | 5 |
| `sun/util/PropertyResourceBundleCharset$PropertiesFileDecoder` | 3 |
| `sun/util/ResourceBundleEnumeration` | 4 |
| `sun/util/calendar/JulianCalendar` | 20 |
| `sun/util/calendar/JulianCalendar$Date` | 8 |
| `sun/util/calendar/LocalGregorianCalendar$Date` | 13 |
| `sun/util/calendar/ZoneInfoFile$1` | 3 |
| `sun/util/cldr/CLDRBaseLocaleDataMetaInfo` | 7 |
| `sun/util/cldr/CLDRCalendarDataProviderImpl` | 6 |
| `sun/util/cldr/CLDRCalendarNameProviderImpl` | 2 |
| `sun/util/cldr/CLDRLocaleProviderAdapter` | 23 |
| `sun/util/cldr/CLDRTimeZoneNameProviderImpl` | 10 |
| `sun/util/locale/LocaleMatcher` | 21 |
| `sun/util/locale/LocaleObjectCache` | 7 |
| `sun/util/locale/LocaleObjectCache$CacheEntry` | 2 |
| `sun/util/locale/ParseStatus` | 6 |
| `sun/util/locale/provider/BaseLocaleDataMetaInfo` | 5 |
| `sun/util/locale/provider/BreakIteratorProviderImpl` | 9 |
| `sun/util/locale/provider/BreakIteratorProviderImpl$CharacterIteratorCharSequence` | 4 |
| `sun/util/locale/provider/BreakIteratorProviderImpl$GraphemeBreakIterator` | 13 |
| `sun/util/locale/provider/CalendarDataProviderImpl` | 6 |
| `sun/util/locale/provider/CalendarNameProviderImpl` | 16 |
| `sun/util/locale/provider/CalendarProviderImpl` | 5 |
| `sun/util/locale/provider/CollatorProviderImpl` | 5 |
| `sun/util/locale/provider/CurrencyNameProviderImpl` | 6 |
| `sun/util/locale/provider/DateFormatProviderImpl` | 8 |
| `sun/util/locale/provider/DateFormatSymbolsProviderImpl` | 5 |
| `sun/util/locale/provider/DecimalFormatSymbolsProviderImpl` | 5 |
| `sun/util/locale/provider/JRELocaleProviderAdapter` | 41 |
| `sun/util/locale/provider/JavaTimeDateTimePatternImpl` | 9 |
| `sun/util/locale/provider/LocaleDataMetaInfo` | 4 |
| `sun/util/locale/provider/LocaleNameProviderImpl` | 11 |
| `sun/util/locale/provider/LocaleResources` | 42 |
| `sun/util/locale/provider/LocaleResources$ResourceReference` | 2 |
| `sun/util/locale/provider/NumberFormatProviderImpl` | 11 |
| `sun/util/locale/provider/ResourceBundleBasedAdapter` | 2 |
| `sun/util/locale/provider/TimeZoneNameProviderImpl` | 8 |
| `sun/util/locale/provider/TimeZoneNameUtility` | 10 |
| `sun/util/logging/PlatformLogger` | 30 |
| `sun/util/logging/PlatformLogger$Bridge` | 18 |
| `sun/util/logging/PlatformLogger$ConfigurableBridge` | 2 |
| `sun/util/logging/PlatformLogger$ConfigurableBridge$LoggerConfiguration` | 3 |
| `sun/util/resources/Bundles` | 12 |
| `sun/util/resources/Bundles$2` | 3 |
| `sun/util/resources/Bundles$BundleReference` | 2 |
| `sun/util/resources/Bundles$CacheKey` | 14 |
| `sun/util/resources/Bundles$CacheKeyReference` | 1 |
| `sun/util/resources/Bundles$Strategy` | 3 |
| `sun/util/resources/LocaleData` | 15 |
| `sun/util/resources/LocaleData$1` | 3 |
| `sun/util/resources/LocaleData$2` | 3 |
| `sun/util/resources/OpenListResourceBundle` | 10 |
| `sun/util/resources/ParallelListResourceBundle` | 12 |
| `sun/util/resources/ParallelListResourceBundle$KeySet` | 4 |
| `sun/util/resources/ParallelListResourceBundle$KeySet$1` | 5 |
| `sun/util/resources/TimeZoneNamesBundle` | 5 |
| `sun/util/spi/CalendarProvider` | 2 |
