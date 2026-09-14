# 调用链分析报告：HelloWorld

生成时间：2026-09-14

## 摘要

| 策略 | 类数 | 方法数 |
|------|-----:|-------:|
| 类级 BFS | 3129 | — |
| 方法级 BFS（调用链） | 367 | 1680 |
| 方法级 BFS（field-only stub） | 61 | — |
| 方法级 BFS 合计 | 428 | 1680 |
| 节省（方法级不需要） | 2749 | — |

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

### `java/io/BufferedWriter`

- `ensureOpen()V`
- `flushBuffer()V`
- `implFlushBuffer()V`
- `newLine()V`
- `write(Ljava/lang/String;)V`

### `java/io/DataInputStream`

- `<init>()V`
- `<init>(Ljava/io/InputStream;)V`
- `close()V`
- `readFully([B)V`
- `readFully([BII)V`
- `readInt()I`

### `java/io/EOFException`

- `<init>()V`

### `java/io/FilePermission`

- `<init>()V`

### `java/io/FilterInputStream`

- `<init>(Ljava/io/InputStream;)V`

### `java/io/IOException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `getMessage()Ljava/lang/String;`

### `java/io/InputStream`

- `<init>()V`
- `read()I`
- `read([BII)I`

### `java/io/OutputStream`

- `flush()V`
- `write(I)V`
- `write([BII)V`

### `java/io/OutputStreamWriter`

- `flushBuffer()V`

### `java/io/PrintStream`

- `ensureOpen()V`
- `implNewLine()V`
- `implWrite(Ljava/lang/String;)V`
- `implWriteln(Ljava/lang/String;)V`
- `newLine()V`
- `print(I)V`
- `print(Ljava/lang/String;)V`
- `println(I)V`
- `println(Ljava/lang/String;)V`
- `write(Ljava/lang/String;)V`
- `writeln(Ljava/lang/String;)V`

### `java/io/Writer`

- `write([CII)V`

### `java/lang/AbstractStringBuilder`

- `<init>(I)V`
- `<init>(Ljava/lang/String;)V`
- `append(C)Ljava/lang/AbstractStringBuilder;`
- `append(F)Ljava/lang/AbstractStringBuilder;`
- `append(I)Ljava/lang/AbstractStringBuilder;`
- `append(J)Ljava/lang/AbstractStringBuilder;`
- `append(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;`
- `append(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;`
- `append([C)Ljava/lang/AbstractStringBuilder;`
- `appendChars(Ljava/lang/CharSequence;II)V`
- `appendChars(Ljava/lang/String;II)V`
- `appendChars([CII)V`
- `appendCodePoint(I)Ljava/lang/AbstractStringBuilder;`
- `appendNull()Ljava/lang/AbstractStringBuilder;`
- `delete(II)Ljava/lang/AbstractStringBuilder;`
- `ensureCapacityInternal(I)V`
- `getValue()[B`
- `inflate()V`
- `inflateIfNeededFor(Ljava/lang/String;)V`
- `isLatin1()Z`
- `length()I`
- `newCapacity(I)I`
- `putStringAt(ILjava/lang/String;)V`
- `setLength(I)V`
- `shift(II)V`

### `java/lang/Appendable`

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
- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/String;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/BaseVirtualThread`

- `<init>()V`

### `java/lang/Boolean`

- `<init>()V`
- `booleanValue()Z`

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
- `codePointOf(Ljava/lang/String;)I`
- `digit(CI)I`
- `digit(II)I`
- `getName(I)Ljava/lang/String;`
- `getType(I)I`
- `highSurrogate(I)C`
- `isBmpCodePoint(I)Z`
- `isHighSurrogate(C)Z`
- `isLowSurrogate(C)Z`
- `isSupplementaryCodePoint(I)Z`
- `isSurrogate(C)Z`
- `isUpperCase(C)Z`
- `isUpperCase(I)Z`
- `isValidCodePoint(I)Z`
- `lowSurrogate(I)C`
- `toChars(I)[C`
- `toCodePoint(CC)I`
- `toLowerCase(C)C`
- `toLowerCase(I)I`
- `toSurrogates(I[CI)V`
- `toUpperCase(C)C`
- `toUpperCase(I)I`
- `toUpperCaseCharArray(I)[C`
- `toUpperCaseEx(I)I`
- `valueOf(C)Ljava/lang/Character;`

### `java/lang/Character$UnicodeBlock`

- `<init>()V`
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
- `isUpperCase(I)Z`
- `of(I)Ljava/lang/CharacterData;`
- `toLowerCase(I)I`
- `toUpperCase(I)I`
- `toUpperCaseCharArray(I)[C`
- `toUpperCaseEx(I)I`

### `java/lang/CharacterDataLatin1`

- `getProperties(I)I`
- `toLowerCase(I)I`
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
- `checkMemberAccess(Ljava/lang/SecurityManager;ILjava/lang/Class;Z)V`
- `checkPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V`
- `elementType()Ljava/lang/Class;`
- `enumConstantDirectory()Ljava/util/Map;`
- `forName(Ljava/lang/String;)Ljava/lang/Class;`
- `forName(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;`
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
- `getDeclaredMethods0(Z)[Ljava/lang/reflect/Method;`
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
- `getName()Ljava/lang/String;`
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
- `isArray()Z`
- `isAssignableFrom(Ljava/lang/Class;)Z`
- `isEnum()Z`
- `isHidden()Z`
- `isInterface()Z`
- `isLocalOrAnonymousClass()Z`
- `isPrimitive()Z`
- `isSynthetic()Z`
- `isTopLevelClass()Z`
- `isUnnamedClass()Z`
- `methodToString(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/String;`
- `newReflectionData(Ljava/lang/ref/SoftReference;I)Ljava/lang/Class$ReflectionData;`
- `privateGetDeclaredConstructors(Z)[Ljava/lang/reflect/Constructor;`
- `privateGetDeclaredMethods(Z)[Ljava/lang/reflect/Method;`
- `reflectionData()Ljava/lang/Class$ReflectionData;`

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

### `java/lang/ClassLoader`

- `checkClassLoaderPermission(Ljava/lang/ClassLoader;Ljava/lang/Class;)V`
- `getBuiltinAppClassLoader()Ljava/lang/ClassLoader;`
- `getClassLoader(Ljava/lang/Class;)Ljava/lang/ClassLoader;`
- `getParent()Ljava/lang/ClassLoader;`
- `getSystemClassLoader()Ljava/lang/ClassLoader;`
- `isAncestor(Ljava/lang/ClassLoader;)Z`
- `needsClassLoaderPermissionCheck(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`

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

- `doubleToRawLongBits(D)J`
- `longBitsToDouble(J)D`

### `java/lang/Enum`

- `<init>()V`
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
- `<init>(Ljava/lang/Throwable;)V`
- `getMessage()Ljava/lang/String;`
- `printStackTrace()V`

### `java/lang/Float`

- `isNaN(F)Z`

### `java/lang/IllegalArgumentException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/IllegalStateException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/IndexOutOfBoundsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/Integer`

- `<init>()V`
- `<init>(I)V`
- `formatUnsignedInt(II[BI)V`
- `formatUnsignedIntUTF16(II[BI)V`
- `getChars(II[B)I`
- `intValue()I`
- `numberOfLeadingZeros(I)I`
- `parseInt(Ljava/lang/CharSequence;III)I`
- `parseInt(Ljava/lang/String;)I`
- `parseInt(Ljava/lang/String;I)I`
- `stringSize(I)I`
- `toHexString(I)Ljava/lang/String;`
- `toString(I)Ljava/lang/String;`
- `toUnsignedString0(II)Ljava/lang/String;`
- `valueOf(I)Ljava/lang/Integer;`

### `java/lang/InternalError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/Long`

- `formatUnsignedLong0(JI[BII)V`
- `formatUnsignedLong0UTF16(JI[BII)V`
- `getChars(JI[B)I`
- `numberOfLeadingZeros(J)I`
- `stringSize(J)I`
- `toHexString(J)Ljava/lang/String;`
- `toUnsignedString0(JI)Ljava/lang/String;`

### `java/lang/Math`

- `addExact(II)I`
- `addExact(JJ)J`
- `ceil(D)D`
- `floorDiv(JI)J`
- `floorDiv(JJ)J`
- `floorMod(II)I`
- `floorMod(JI)I`
- `floorMod(JJ)J`
- `getExponent(D)I`
- `max(II)I`
- `min(II)I`
- `min(JJ)J`
- `multiplyExact(II)I`

### `java/lang/NegativeArraySizeException`

- `<init>()V`

### `java/lang/NoSuchMethodException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `printStackTrace()V`

### `java/lang/NullPointerException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/Number`

- `<init>()V`

### `java/lang/NumberFormatException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `forCharSequence(Ljava/lang/CharSequence;III)Ljava/lang/NumberFormatException;`
- `forInputString(Ljava/lang/String;I)Ljava/lang/NumberFormatException;`

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

### `java/lang/RuntimeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/RuntimePermission`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/SecurityException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/SecurityManager`

- `checkAccess(Ljava/lang/Thread;)V`
- `checkPackageAccess(Ljava/lang/String;)V`
- `checkPermission(Ljava/security/Permission;)V`
- `getPackages(Ljava/lang/String;)[Ljava/lang/String;`

### `java/lang/SecurityManager$1`

- `<init>()V`
- `<init>(Ljava/lang/SecurityManager;)V`

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

### `java/lang/StackWalker`

- `getContScope()Ljdk/internal/vm/ContinuationScope;`
- `getContinuation()Ljdk/internal/vm/Continuation;`
- `getInstance()Ljava/lang/StackWalker;`
- `hasLocalsOperandsOption()Z`
- `hasOption(Ljava/lang/StackWalker$Option;)Z`
- `walk(Ljava/util/function/Function;)Ljava/lang/Object;`

### `java/lang/StrictMath`

- `ceil(D)D`
- `floorOrCeil(DDDD)D`

### `java/lang/String`

- `<init>()V`
- `<init>(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/StringBuilder;)V`
- `<init>([BB)V`
- `<init>([BIII)V`
- `<init>([C)V`
- `<init>([CIILjava/lang/Void;)V`
- `<init>([III)V`
- `charAt(I)C`
- `checkBoundsBeginEnd(III)V`
- `checkBoundsOffCount(III)I`
- `checkIndex(II)V`
- `codePointAt(I)I`
- `codePointBefore(I)I`
- `codePointCount(II)I`
- `coder()B`
- `compareTo(Ljava/lang/String;)I`
- `concat(Ljava/lang/String;)Ljava/lang/String;`
- `contains(Ljava/lang/CharSequence;)Z`
- `encode(Ljava/nio/charset/Charset;B[B)[B`
- `encode8859_1(B[B)[B`
- `encode8859_1(B[BZ)[B`
- `encodeASCII(B[B)[B`
- `encodeUTF8(B[BZ)[B`
- `encodeUTF8_UTF16([BZ)[B`
- `encodeWithEncoder(Ljava/nio/charset/Charset;B[BZ)[B`
- `equals(Ljava/lang/Object;)Z`
- `format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;`
- `getBytes(Ljava/nio/charset/Charset;)[B`
- `getBytes([BIB)V`
- `hashCode()I`
- `indexOf(I)I`
- `indexOf(II)I`
- `indexOf(Ljava/lang/String;)I`
- `intern()Ljava/lang/String;`
- `isEmpty()Z`
- `isLatin1()Z`
- `lastIndexOf(I)I`
- `lastIndexOf(II)I`
- `lastIndexOf(Ljava/lang/String;)I`
- `lastIndexOf(Ljava/lang/String;I)I`
- `lastIndexOf([BBILjava/lang/String;I)I`
- `length()I`
- `replace(CC)Ljava/lang/String;`
- `replaceAll(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;`
- `replaceNegatives([BI)V`
- `safeTrim([BIZ)[B`
- `scale(IF)I`
- `startsWith(Ljava/lang/String;)Z`
- `startsWith(Ljava/lang/String;I)Z`
- `substring(I)Ljava/lang/String;`
- `substring(II)Ljava/lang/String;`
- `throwUnmappable(I)V`
- `toLowerCase(Ljava/util/Locale;)Ljava/lang/String;`
- `toUpperCase(Ljava/util/Locale;)Ljava/lang/String;`
- `trim()Ljava/lang/String;`
- `value()[B`
- `valueOf(C)Ljava/lang/String;`
- `valueOf(I)Ljava/lang/String;`
- `valueOf(Ljava/lang/Object;)Ljava/lang/String;`

### `java/lang/StringBuilder`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/lang/String;)V`
- `append(C)Ljava/lang/StringBuilder;`
- `append(F)Ljava/lang/StringBuilder;`
- `append(I)Ljava/lang/StringBuilder;`
- `append(J)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/Object;)Ljava/lang/StringBuilder;`
- `append(Ljava/lang/String;)Ljava/lang/StringBuilder;`
- `appendCodePoint(I)Ljava/lang/StringBuilder;`
- `delete(II)Ljava/lang/StringBuilder;`
- `length()I`
- `setLength(I)V`
- `toString()Ljava/lang/String;`

### `java/lang/StringCoding`

- `countPositives([BII)I`
- `hasNegatives([BII)Z`
- `implEncodeISOArray([BI[BII)I`

### `java/lang/StringConcatHelper`

- `checkOverflow(J)J`
- `initialCoder()J`
- `mix(JLjava/lang/String;)J`
- `newArray(J)[B`
- `newString([BJ)Ljava/lang/String;`
- `prepend(J[BLjava/lang/String;)J`
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
- `compareToUTF16Values([B[BII)I`
- `equals([B[B)Z`
- `fillNull([BII)V`
- `getChar([BI)C`
- `hashCode([B)I`
- `indexOf([BIII)I`
- `indexOf([BI[BII)I`
- `indexOf([B[B)I`
- `indexOfChar([BIII)I`
- `inflate([BI[BII)V`
- `inflate([BI[CII)V`
- `lastIndexOf([BII)I`
- `lastIndexOf([BI[BII)I`
- `length([B)I`
- `newString([BII)Ljava/lang/String;`
- `replace([BCC)Ljava/lang/String;`
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
- `codePointAt([BII)I`
- `codePointAt([BIIZ)I`
- `codePointBefore([BI)I`
- `codePointBefore([BIZ)I`
- `codePointCount([BII)I`
- `codePointCount([BIIZ)I`
- `coderFromArrayLen([BI)B`
- `compareTo([B[B)I`
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
- `getChar([BI)C`
- `getChars(III[B)I`
- `getChars(II[B)I`
- `getChars(JII[B)I`
- `getChars(JI[B)I`
- `getChars([BII[CI)V`
- `hashCode([B)I`
- `indexOf([BIII)I`
- `indexOf([B[B)I`
- `indexOfChar([BIII)I`
- `indexOfCharUnsafe([BIII)I`
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
- `putCharsSB([BILjava/lang/CharSequence;II)V`
- `putCharsSB([BI[CII)V`
- `replace([BCC)Ljava/lang/String;`
- `toBytes(C)[B`
- `toBytes([CII)[B`
- `toBytes([III)[B`
- `toChars([B)[C`
- `toLowerCase(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;`
- `toLowerCaseEx(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;`
- `toUpperCase(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;`
- `toUpperCaseEx(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;`
- `trim([B)Ljava/lang/String;`

### `java/lang/System`

- `allowSecurityManager()Z`
- `arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V`
- `currentTimeMillis()J`
- `getSecurityManager()Ljava/lang/SecurityManager;`
- `identityHashCode(Ljava/lang/Object;)I`
- `lineSeparator()Ljava/lang/String;`
- `nanoTime()J`

### `java/lang/Thread`

- `checkAccess()V`
- `currentThread()Ljava/lang/Thread;`
- `dumpStack()V`
- `getName()Ljava/lang/String;`
- `getThreadGroup()Ljava/lang/ThreadGroup;`
- `interrupt()V`
- `interrupt0()V`
- `isTerminated()Z`
- `isVirtual()Z`
- `threadId()J`
- `threadState()Ljava/lang/Thread$State;`
- `virtualThreadGroup()Ljava/lang/ThreadGroup;`
- `yield()V`
- `yield0()V`

### `java/lang/Throwable`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`
- `addSuppressed(Ljava/lang/Throwable;)V`
- `fillInStackTrace()Ljava/lang/Throwable;`
- `fillInStackTrace(I)Ljava/lang/Throwable;`
- `getLocalizedMessage()Ljava/lang/String;`
- `getMessage()Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `java/lang/UnsupportedOperationException`

- `<init>()V`

### `java/lang/VirtualMachineError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/VirtualThread`

- `<init>()V`
- `continuationScope()Ljdk/internal/vm/ContinuationScope;`
- `notifyJvmtiMount(Z)V`
- `notifyJvmtiUnmount(Z)V`
- `setState(I)V`
- `state()I`
- `tryYield()V`
- `yieldContinuation()Z`

### `java/lang/ref/Cleaner`

- `register(Ljava/lang/Object;Ljava/lang/Runnable;)Ljava/lang/ref/Cleaner$Cleanable;`

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
- `get()Ljava/lang/Object;`

### `java/lang/ref/WeakReference`

- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`

### `java/lang/reflect/Array`

- `newArray(Ljava/lang/Class;I)Ljava/lang/Object;`
- `newInstance(Ljava/lang/Class;I)Ljava/lang/Object;`

### `java/lang/reflect/Constructor`

- `<init>()V`
- `acquireConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V`
- `getConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;`
- `newInstance([Ljava/lang/Object;)Ljava/lang/Object;`
- `newInstanceWithCaller([Ljava/lang/Object;ZLjava/lang/Class;)Ljava/lang/Object;`
- `setConstructorAccessor(Ljdk/internal/reflect/ConstructorAccessor;)V`

### `java/lang/reflect/Method`

- `acquireMethodAccessor()Ljdk/internal/reflect/MethodAccessor;`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V`
- `getDeclaringClass()Ljava/lang/Class;`
- `getMethodAccessor()Ljdk/internal/reflect/MethodAccessor;`
- `getModifiers()I`
- `getName()Ljava/lang/String;`
- `getReturnType()Ljava/lang/Class;`
- `invoke(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;`
- `isCallerSensitive()Z`
- `setMethodAccessor(Ljdk/internal/reflect/MethodAccessor;)V`

### `java/lang/reflect/Modifier`

- `isFinal(I)Z`
- `isPublic(I)Z`
- `isStatic(I)Z`

### `java/lang/reflect/ParameterizedType`

- `<init>()V`
- `getActualTypeArguments()[Ljava/lang/reflect/Type;`
- `getRawType()Ljava/lang/reflect/Type;`

### `java/lang/reflect/Proxy`

- `isProxyClass(Ljava/lang/Class;)Z`

### `java/lang/reflect/Proxy$ProxyBuilder`

- `isProxyClass(Ljava/lang/Class;)Z`

### `java/lang/reflect/Type`

- `<init>()V`

### `java/net/URL`

- `<init>()V`
- `getAuthority()Ljava/lang/String;`
- `getPath()Ljava/lang/String;`
- `getProtocol()Ljava/lang/String;`
- `getQuery()Ljava/lang/String;`
- `getRef()Ljava/lang/String;`
- `toExternalForm()Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `java/net/URLStreamHandler`

- `toExternalForm(Ljava/net/URL;)Ljava/lang/String;`

### `java/nio/Buffer`

- `<init>(IIIILjava/lang/foreign/MemorySegment;)V`
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
- `array()[B`
- `arrayOffset()I`
- `base()Ljava/lang/Object;`
- `clear()Ljava/nio/ByteBuffer;`
- `flip()Ljava/nio/ByteBuffer;`
- `isReadOnly()Z`
- `limit()I`
- `position()I`
- `position(I)Ljava/nio/ByteBuffer;`
- `put(IB)Ljava/nio/ByteBuffer;`
- `put([B)Ljava/nio/ByteBuffer;`
- `put([BII)Ljava/nio/ByteBuffer;`
- `putArray(I[BII)Ljava/nio/ByteBuffer;`
- `remaining()I`
- `session()Ljdk/internal/foreign/MemorySessionImpl;`
- `wrap([B)Ljava/nio/ByteBuffer;`
- `wrap([BII)Ljava/nio/ByteBuffer;`

### `java/nio/ByteOrder`

- `nativeOrder()Ljava/nio/ByteOrder;`

### `java/nio/CharBuffer`

- `<init>(IIII[CILjava/lang/foreign/MemorySegment;)V`
- `hasRemaining()Z`
- `position()I`
- `position(I)Ljava/nio/CharBuffer;`
- `remaining()I`
- `wrap([CII)Ljava/nio/CharBuffer;`

### `java/nio/HeapByteBuffer`

- `<init>()V`
- `<init>([BIILjava/lang/foreign/MemorySegment;)V`

### `java/nio/HeapCharBuffer`

- `<init>()V`
- `<init>([CIILjava/lang/foreign/MemorySegment;)V`

### `java/nio/ReadOnlyBufferException`

- `<init>()V`

### `java/nio/channels/WritableByteChannel`

- `write(Ljava/nio/ByteBuffer;)I`

### `java/nio/charset/CharacterCodingException`

- `<init>()V`

### `java/nio/charset/Charset`

- `newEncoder()Ljava/nio/charset/CharsetEncoder;`

### `java/nio/charset/CharsetEncoder`

- `encode(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;`
- `encodeLoop(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;`
- `flush(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;`
- `implFlush(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;`
- `implOnMalformedInput(Ljava/nio/charset/CodingErrorAction;)V`
- `implOnUnmappableCharacter(Ljava/nio/charset/CodingErrorAction;)V`
- `maxBytesPerChar()F`
- `onMalformedInput(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;`
- `onUnmappableCharacter(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;`
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

### `java/nio/charset/MalformedInputException`

- `<init>()V`
- `<init>(I)V`

### `java/nio/charset/UnmappableCharacterException`

- `<init>()V`
- `<init>(I)V`

### `java/security/AccessControlContext`

- `calculateFields(Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)V`
- `checkPermission(Ljava/security/Permission;)V`
- `checkPermission2(Ljava/security/Permission;)V`
- `combine([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)[Ljava/security/ProtectionDomain;`
- `containsAllPDs([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)Z`
- `getDebug()Lsun/security/util/Debug;`
- `isPrivileged()Z`
- `optimize()Ljava/security/AccessControlContext;`

### `java/security/AccessControlContext$1`

- `<init>()V`
- `<init>(Ljava/security/AccessControlContext;Lsun/security/util/Debug;Ljava/security/ProtectionDomain;)V`

### `java/security/AccessControlException`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/security/Permission;)V`

### `java/security/AccessController`

- `checkPermission(Ljava/security/Permission;)V`
- `doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;`
- `ensureMaterializedForStackWalk(Ljava/lang/Object;)V`
- `executePrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;`
- `getInheritedAccessControlContext()Ljava/security/AccessControlContext;`
- `getStackAccessControlContext()Ljava/security/AccessControlContext;`
- `isPrivileged()Z`

### `java/security/AllPermission`

- `<init>()V`

### `java/security/BasicPermission`

- `<init>(Ljava/lang/String;)V`
- `init(Ljava/lang/String;)V`

### `java/security/CodeSource`

- `getLocation()Ljava/net/URL;`

### `java/security/DomainCombiner`

- `combine([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)[Ljava/security/ProtectionDomain;`

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

### `java/security/PrivilegedAction`

- `run()Ljava/lang/Object;`

### `java/security/ProtectionDomain`

- `<init>()V`
- `getCodeSource()Ljava/security/CodeSource;`
- `getPermissions()Ljava/security/PermissionCollection;`
- `implies(Ljava/security/Permission;)Z`
- `impliesWithAltFilePerm(Ljava/security/Permission;)Z`

### `java/security/UnresolvedPermission`

- `<init>()V`
- `resolve(Ljava/security/Permission;[Ljava/security/cert/Certificate;)Ljava/security/Permission;`

### `java/security/UnresolvedPermissionCollection`

- `<init>()V`
- `getUnresolvedPermissions(Ljava/security/Permission;)Ljava/util/List;`

### `java/security/cert/Certificate`

- `<init>()V`
- `equals(Ljava/lang/Object;)Z`
- `getEncoded()[B`

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

### `java/time/DateTimeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/time/DayOfWeek`

- `getValue()I`
- `ordinal()I`

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
- `create(III)Ljava/time/LocalDate;`
- `isLeapYear()Z`
- `lengthOfMonth()I`
- `of(ILjava/time/Month;I)Ljava/time/LocalDate;`
- `ofEpochDay(J)Ljava/time/LocalDate;`
- `plusDays(J)Ljava/time/LocalDate;`
- `toEpochDay()J`
- `with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalDate;`

### `java/time/LocalDateTime`

- `<init>()V`
- `<init>(Ljava/time/LocalDate;Ljava/time/LocalTime;)V`
- `getNano()I`
- `of(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;`
- `ofEpochSecond(JILjava/time/ZoneOffset;)Ljava/time/LocalDateTime;`
- `plusSeconds(J)Ljava/time/LocalDateTime;`
- `plusWithOverflow(Ljava/time/LocalDate;JJJJI)Ljava/time/LocalDateTime;`
- `toEpochSecond(Ljava/time/ZoneOffset;)J`
- `with(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;`

### `java/time/LocalTime`

- `<init>()V`
- `<init>(IIII)V`
- `create(IIII)Ljava/time/LocalTime;`
- `from(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalTime;`
- `getNano()I`
- `ofNanoOfDay(J)Ljava/time/LocalTime;`
- `toNanoOfDay()J`

### `java/time/Month`

- `getValue()I`
- `length(Z)I`
- `name()Ljava/lang/String;`
- `of(I)Ljava/time/Month;`
- `ordinal()I`

### `java/time/ZoneId`

- `<init>()V`
- `getRules()Ljava/time/zone/ZoneRules;`
- `normalized()Ljava/time/ZoneId;`

### `java/time/ZoneOffset`

- `<init>()V`
- `equals(Ljava/lang/Object;)Z`
- `getTotalSeconds()I`

### `java/time/chrono/ChronoLocalDate`

- `atTime(Ljava/time/LocalTime;)Ljava/time/chrono/ChronoLocalDateTime;`

### `java/time/chrono/ChronoLocalDateTimeImpl`

- `<init>()V`
- `<init>(Ljava/time/chrono/ChronoLocalDate;Ljava/time/LocalTime;)V`
- `of(Ljava/time/chrono/ChronoLocalDate;Ljava/time/LocalTime;)Ljava/time/chrono/ChronoLocalDateTimeImpl;`

### `java/time/chrono/ChronoZonedDateTimeImpl`

- `<init>()V`
- `<init>(Ljava/time/chrono/ChronoLocalDateTimeImpl;Ljava/time/ZoneOffset;Ljava/time/ZoneId;)V`
- `ofInstant(Ljava/time/chrono/Chronology;Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/chrono/ChronoZonedDateTimeImpl;`

### `java/time/chrono/Chronology`

- `<init>()V`
- `date(Ljava/time/temporal/TemporalAccessor;)Ljava/time/chrono/ChronoLocalDate;`
- `localDateTime(Ljava/time/temporal/TemporalAccessor;)Ljava/time/chrono/ChronoLocalDateTime;`
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
- `checkValidValue(J)J`
- `isDateBased()Z`
- `ordinal()I`
- `range()Ljava/time/temporal/ValueRange;`
- `values()[Ljava/time/temporal/ChronoField;`

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

### `java/time/temporal/TemporalField`

- `range()Ljava/time/temporal/ValueRange;`
- `rangeRefinedBy(Ljava/time/temporal/TemporalAccessor;)Ljava/time/temporal/ValueRange;`

### `java/time/temporal/TemporalQueries`

- `chronology()Ljava/time/temporal/TemporalQuery;`
- `localTime()Ljava/time/temporal/TemporalQuery;`
- `precision()Ljava/time/temporal/TemporalQuery;`
- `zoneId()Ljava/time/temporal/TemporalQuery;`

### `java/time/temporal/TemporalQuery`

- `queryFrom(Ljava/time/temporal/TemporalAccessor;)Ljava/lang/Object;`

### `java/time/temporal/UnsupportedTemporalTypeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/time/temporal/ValueRange`

- `checkValidValue(JLjava/time/temporal/TemporalField;)J`
- `genInvalidFieldMessage(Ljava/time/temporal/TemporalField;J)Ljava/lang/String;`
- `getMaximum()J`
- `getMinimum()J`
- `isIntValue()Z`
- `isValidValue(J)Z`

### `java/time/zone/ZoneOffsetTransition`

- `<init>()V`
- `<init>(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)V`
- `getOffsetAfter()Ljava/time/ZoneOffset;`
- `getOffsetBefore()Ljava/time/ZoneOffset;`
- `toEpochSecond()J`

### `java/time/zone/ZoneOffsetTransitionRule`

- `createTransition(I)Ljava/time/zone/ZoneOffsetTransition;`

### `java/time/zone/ZoneOffsetTransitionRule$TimeDefinition`

- `createDateTime(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)Ljava/time/LocalDateTime;`
- `ordinal()I`

### `java/time/zone/ZoneRules`

- `findTransitionArray(I)[Ljava/time/zone/ZoneOffsetTransition;`
- `findYear(JLjava/time/ZoneOffset;)I`
- `getOffset(Ljava/time/Instant;)Ljava/time/ZoneOffset;`
- `isFixedOffset()Z`

### `java/util/AbstractCollection`

- `<init>()V`

### `java/util/AbstractList`

- `<init>()V`

### `java/util/AbstractMap`

- `<init>()V`

### `java/util/AbstractSet`

- `<init>()V`

### `java/util/ArrayList`

- `<init>()V`
- `<init>(I)V`
- `<init>(Ljava/util/Collection;)V`
- `add(Ljava/lang/Object;)Z`
- `add(Ljava/lang/Object;[Ljava/lang/Object;I)V`
- `grow()[Ljava/lang/Object;`
- `grow(I)[Ljava/lang/Object;`

### `java/util/Arrays`

- `binarySearch([JJ)I`
- `binarySearch0([JIIJ)I`
- `checkLength(II)V`
- `copyOf([BI)[B`
- `copyOf([II)[I`
- `copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;`
- `copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;`
- `copyOfRange([BII)[B`
- `copyOfRangeByte([BII)[B`
- `equals([B[B)Z`
- `equals([Ljava/lang/Object;[Ljava/lang/Object;)Z`
- `fill([BIIB)V`
- `fill([II)V`
- `rangeCheck(III)V`
- `spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;`
- `stream([Ljava/lang/Object;)Ljava/util/stream/Stream;`
- `stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;`

### `java/util/Collection`

- `<init>()V`
- `getClass()Ljava/lang/Class;`
- `toArray()[Ljava/lang/Object;`

### `java/util/Collections`

- `emptyMap()Ljava/util/Map;`
- `emptySet()Ljava/util/Set;`
- `synchronizedMap(Ljava/util/Map;)Ljava/util/Map;`

### `java/util/Collections$SynchronizedMap`

- `<init>()V`
- `<init>(Ljava/util/Map;)V`

### `java/util/DuplicateFormatFlagsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/Enumeration`

- `hasMoreElements()Z`
- `nextElement()Ljava/lang/Object;`

### `java/util/FormatFlagsConversionMismatchException`

- `<init>()V`
- `<init>(Ljava/lang/String;C)V`

### `java/util/Formatter`

- `<init>()V`
- `<init>(Ljava/util/Locale;Ljava/lang/Appendable;)V`
- `ensureOpen()V`
- `format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;`
- `format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;`
- `parse(Ljava/lang/String;)Ljava/util/List;`
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

### `java/util/HashMap`

- `<init>()V`
- `<init>(I)V`
- `<init>(IF)V`
- `calculateHashMapCapacity(I)I`
- `comparableClassFor(Ljava/lang/Object;)Ljava/lang/Class;`
- `compareComparables(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getNode(Ljava/lang/Object;)Ljava/util/HashMap$Node;`
- `hash(Ljava/lang/Object;)I`
- `keySet()Ljava/util/Set;`
- `newHashMap(I)Ljava/util/HashMap;`
- `tableSizeFor(I)I`

### `java/util/HashMap$KeySet`

- `<init>()V`
- `<init>(Ljava/util/HashMap;)V`

### `java/util/HashMap$TreeNode`

- `<init>()V`
- `find(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/HashMap$TreeNode;`
- `getTreeNode(ILjava/lang/Object;)Ljava/util/HashMap$TreeNode;`
- `root()Ljava/util/HashMap$TreeNode;`

### `java/util/HashSet`

- `<init>()V`
- `<init>(I)V`
- `<init>(IFZ)V`
- `iterator()Ljava/util/Iterator;`
- `newHashSet(I)Ljava/util/HashSet;`

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

### `java/util/ImmutableCollections$AbstractImmutableMap`

- `<init>()V`

### `java/util/ImmutableCollections$Map1`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/ImmutableCollections$MapN`

- `<init>()V`
- `<init>([Ljava/lang/Object;)V`
- `probe(Ljava/lang/Object;)I`

### `java/util/Iterator`

- `hasNext()Z`
- `next()Ljava/lang/Object;`

### `java/util/LinkedHashMap`

- `<init>()V`
- `<init>(IF)V`

### `java/util/LinkedHashSet`

- `<init>()V`

### `java/util/List`

- `<init>()V`
- `add(Ljava/lang/Object;)Z`
- `clear()V`
- `equals(Ljava/lang/Object;)Z`
- `get(I)Ljava/lang/Object;`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `size()I`
- `subList(II)Ljava/util/List;`

### `java/util/Locale`

- `<init>()V`
- `convertOldISOCodes(Ljava/lang/String;)Ljava/lang/String;`
- `equals(Ljava/lang/Object;)Z`
- `getBaseLocale()Lsun/util/locale/BaseLocale;`
- `getCompatibilityExtensions(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/LocaleExtensions;`
- `getDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;`
- `getDefaultExtensions(Ljava/lang/String;)Ljava/util/Optional;`
- `getDisplayLocale()Ljava/util/Locale;`
- `getFormatLocale()Ljava/util/Locale;`
- `getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;`
- `getInstance(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;`
- `getLanguage()Ljava/lang/String;`
- `getLocaleExtensions()Lsun/util/locale/LocaleExtensions;`
- `hasExtensions()Z`
- `initDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;`
- `stripExtensions()Ljava/util/Locale;`

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
- `entrySet()Ljava/util/Set;`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `isEmpty()Z`
- `of()Ljava/util/Map;`
- `ofEntries([Ljava/util/Map$Entry;)Ljava/util/Map;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

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

### `java/util/NoSuchElementException`

- `<init>()V`

### `java/util/Objects`

- `checkFromIndexSize(III)I`
- `checkFromToIndex(III)I`
- `equals(Ljava/lang/Object;Ljava/lang/Object;)Z`
- `requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;`
- `requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;`
- `requireNonNullElse(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/Optional`

- `<init>()V`
- `<init>(Ljava/lang/Object;)V`
- `empty()Ljava/util/Optional;`
- `ofNullable(Ljava/lang/Object;)Ljava/util/Optional;`
- `orElse(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/Random`

- `<init>()V`
- `<init>(J)V`
- `initialScramble(J)J`
- `seedUniquifier()J`
- `setSeed(J)V`

### `java/util/ResourceBundle$Control`

- `getCandidateLocales(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/List;`
- `getControl(Ljava/util/List;)Ljava/util/ResourceBundle$Control;`

### `java/util/ResourceBundle$Control$CandidateListCache`

- `get(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/ServiceConfigurationError`

- `<init>()V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/util/Set`

- `add(Ljava/lang/Object;)Z`
- `clear()V`
- `contains(Ljava/lang/Object;)Z`
- `forEach(Ljava/util/function/Consumer;)V`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `toArray([Ljava/lang/Object;)[Ljava/lang/Object;`

### `java/util/SortedMap`

- `entrySet()Ljava/util/Set;`
- `isEmpty()Z`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/SortedSet`

- `add(Ljava/lang/Object;)Z`

### `java/util/Spliterator`

- `characteristics()I`
- `getComparator()Ljava/util/Comparator;`

### `java/util/Spliterators`

- `checkFromToBounds(III)V`
- `spliterator([Ljava/lang/Object;III)Ljava/util/Spliterator;`

### `java/util/Spliterators$ArraySpliterator`

- `<init>()V`
- `<init>([Ljava/lang/Object;III)V`

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

### `java/util/TreeMap`

- `<init>()V`

### `java/util/TreeSet`

- `<init>()V`
- `<init>(Ljava/util/NavigableMap;)V`

### `java/util/UnknownFormatConversionException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/util/UnknownFormatFlagsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

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
- `resize(I)V`
- `transfer([Ljava/util/WeakHashMap$Entry;[Ljava/util/WeakHashMap$Entry;)V`

### `java/util/WeakHashMap$Entry`

- `<init>()V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;ILjava/util/WeakHashMap$Entry;)V`
- `get()Ljava/lang/Object;`
- `refersTo(Ljava/lang/Object;)Z`

### `java/util/concurrent/ConcurrentHashMap`

- `<init>()V`
- `<init>(I)V`
- `<init>(IFI)V`
- `addCount(JI)V`
- `casTabAt([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)Z`
- `comparableClassFor(Ljava/lang/Object;)Ljava/lang/Class;`
- `compareComparables(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I`
- `fullAddCount(JZ)V`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `helpTransfer([Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)[Ljava/util/concurrent/ConcurrentHashMap$Node;`
- `initTable()[Ljava/util/concurrent/ConcurrentHashMap$Node;`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putVal(Ljava/lang/Object;Ljava/lang/Object;Z)Ljava/lang/Object;`
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

### `java/util/concurrent/ConcurrentHashMap$CounterCell`

- `<init>()V`
- `<init>(J)V`

### `java/util/concurrent/ConcurrentHashMap$ForwardingNode`

- `<init>()V`
- `<init>([Ljava/util/concurrent/ConcurrentHashMap$Node;)V`

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
- `balanceInsertion(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`
- `checkInvariants(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Z`
- `contendedLock()V`
- `lockRoot()V`
- `putTreeVal(ILjava/lang/Object;Ljava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`
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
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/concurrent/ThreadLocalRandom`

- `advanceProbe(I)I`
- `getProbe()I`
- `localInit()V`

### `java/util/concurrent/atomic/AtomicInteger`

- `addAndGet(I)I`

### `java/util/concurrent/atomic/AtomicLong`

- `<init>()V`
- `<init>(J)V`
- `compareAndSet(JJ)Z`
- `get()J`
- `getAndAdd(J)J`
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
- `getTextLength()I`
- `groupCount()I`
- `hasMatch()Z`
- `namedGroups()Ljava/util/Map;`
- `replaceAll(Ljava/lang/String;)Ljava/lang/String;`
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

- `getAvailableLocales()[Ljava/util/Locale;`
- `isSupportedLocale(Ljava/util/Locale;)Z`

### `java/util/stream/AbstractPipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`

### `java/util/stream/Collectors`

- `joining(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;`

### `java/util/stream/Collectors$CollectorImpl`

- `<init>()V`
- `<init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V`

### `java/util/stream/PipelineHelper`

- `<init>()V`

### `java/util/stream/ReferencePipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`

### `java/util/stream/ReferencePipeline$Head`

- `<init>()V`
- `<init>(Ljava/util/Spliterator;IZ)V`

### `java/util/stream/Stream`

- `collect(Ljava/util/stream/Collector;)Ljava/lang/Object;`
- `map(Ljava/util/function/Function;)Ljava/util/stream/Stream;`

### `java/util/stream/StreamOpFlag`

- `fromCharacteristics(Ljava/util/Spliterator;)I`

### `java/util/stream/StreamSupport`

- `stream(Ljava/util/Spliterator;Z)Ljava/util/stream/Stream;`

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

### `sun/nio/ch/Interruptible`

- `interrupt(Ljava/lang/Thread;)V`

### `sun/nio/cs/ArrayEncoder`

- `<init>()V`
- `encodeFromLatin1([BII[B)I`
- `encodeFromUTF16([BII[B)I`
- `isASCIICompatible()Z`

### `sun/nio/cs/StreamEncoder`

- `flushBuffer()V`
- `implFlushBuffer()V`
- `isOpen()Z`
- `lockedFlushBuffer()V`
- `writeBytes()V`

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

- `checkProxyPackageAccess(Ljava/lang/ClassLoader;[Ljava/lang/Class;)V`
- `isAncestor(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`
- `isNonPublicProxyClass(Ljava/lang/Class;)Z`
- `needsPackageAccessCheck(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`
- `privateCheckPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/Class;)V`
- `privateCheckProxyPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/Class;)V`

### `sun/security/provider/PolicyFile`

- `<init>()V`
- `init(Ljava/net/URL;)V`
- `initPolicyFile(Ljava/lang/String;Ljava/lang/String;Lsun/security/provider/PolicyFile$PolicyInfo;)Z`
- `initPolicyFile(Lsun/security/provider/PolicyFile$PolicyInfo;Ljava/net/URL;)V`
- `initStaticPolicy(Lsun/security/provider/PolicyFile$PolicyInfo;)V`

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

### `sun/security/provider/PolicyFile$PolicyInfo`

- `<init>()V`
- `<init>(I)V`

### `sun/security/util/Debug`

- `<init>()V`
- `configureExtras(Ljava/lang/String;)V`
- `extraInfo()Ljava/lang/String;`
- `formatCaller()Ljava/lang/String;`
- `getInstance(Ljava/lang/String;)Lsun/security/util/Debug;`
- `getInstance(Ljava/lang/String;Ljava/lang/String;)Lsun/security/util/Debug;`
- `isOn(Ljava/lang/String;)Z`
- `println(Ljava/lang/String;)V`

### `sun/security/util/FilePermCompat`

- `newPermUsingAltPath(Ljava/security/Permission;)Ljava/security/Permission;`

### `sun/security/x509/X509CertImpl`

- `<init>()V`
- `getEncodedInternal()[B`
- `getEncodedInternal(Ljava/security/cert/Certificate;)[B`

### `sun/text/Normalizer`

- `getCombiningClass(I)I`

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
- `setValue(Ljava/lang/String;)V`

### `sun/util/locale/InternalLocaleBuilder`

- `<init>()V`
- `clearExtensions()Lsun/util/locale/InternalLocaleBuilder;`
- `getLocaleExtensions()Lsun/util/locale/LocaleExtensions;`
- `removePrivateuseVariant(Ljava/lang/String;)Ljava/lang/String;`
- `setExtensions(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;`
- `setExtensions(Ljava/util/List;Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;`
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
- `isPrivateusePrefix(Ljava/lang/String;)Z`
- `isPrivateusePrefixChar(C)Z`
- `isPrivateuseSubtag(Ljava/lang/String;)Z`

### `sun/util/locale/LocaleExtensions`

- `<init>()V`
- `<init>(Ljava/util/Map;Ljava/util/Set;Ljava/util/Map;)V`
- `equals(Ljava/lang/Object;)Z`
- `hashCode()I`
- `isEmpty()Z`
- `toID(Ljava/util/SortedMap;)Ljava/lang/String;`

### `sun/util/locale/LocaleSyntaxException`

- `<init>()V`
- `<init>(Ljava/lang/String;I)V`

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
- `isAttribute(Ljava/lang/String;)Z`
- `isKey(Ljava/lang/String;)Z`
- `isSingletonChar(C)Z`
- `setValue(Ljava/lang/String;)V`

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
- `getAdapterClassName()Ljava/lang/String;`
- `ordinal()I`

## Field-only Stub 类（仅通过字段访问发现）

| 类名 | 方法数 | native 数 |
|------|-------:|----------:|
| `java/lang/Byte` | 27 | 0 |
| `java/lang/Character$CharacterCache` | 2 | 0 |
| `java/lang/CharacterData00` | 32 | 0 |
| `java/lang/CharacterData01` | 29 | 0 |
| `java/lang/CharacterData02` | 29 | 0 |
| `java/lang/CharacterData03` | 29 | 0 |
| `java/lang/CharacterData0E` | 29 | 0 |
| `java/lang/CharacterDataPrivateUse` | 26 | 0 |
| `java/lang/CharacterDataUndefined` | 26 | 0 |
| `java/lang/Integer$IntegerCache` | 2 | 0 |
| `java/lang/StackWalker$ExtendedOption` | 5 | 0 |
| `java/lang/StackWalker$Option` | 5 | 0 |
| `java/lang/Thread$Constants` | 2 | 0 |
| `java/lang/Thread$FieldHolder` | 1 | 0 |
| `java/lang/Thread$State` | 5 | 0 |
| `java/nio/charset/CoderResult$Cache` | 2 | 0 |
| `java/nio/charset/CodingErrorAction` | 3 | 0 |
| `java/text/Normalizer$Form` | 5 | 0 |
| `java/util/HashMap$Node` | 7 | 0 |
| `java/util/ImmutableCollections` | 7 | 0 |
| `java/util/Locale$Category` | 5 | 0 |
| `java/util/ResourceBundle$SingleFormatControl` | 3 | 0 |
| `java/util/regex/Pattern$LookBehindEndNode` | 3 | 0 |
| `java/util/regex/Pattern$Qtype` | 5 | 0 |
| `java/util/zip/ZipUtils` | 66 | 0 |
| `jdk/internal/access/JavaIOFilePermissionAccess` | 2 | 0 |
| `jdk/internal/access/JavaLangAccess` | 86 | 0 |
| `jdk/internal/access/JavaSecurityAccess` | 4 | 0 |
| `jdk/internal/access/JavaSecurityAccess$ProtectionDomainCache` | 2 | 0 |
| `jdk/internal/access/SharedSecrets` | 72 | 0 |
| `jdk/internal/icu/lang/UCharacter` | 12 | 0 |
| `jdk/internal/icu/text/NormalizerBase` | 30 | 0 |
| `jdk/internal/loader/AbstractClassLoaderValue$Sub` | 6 | 0 |
| `jdk/internal/loader/ClassLoaderValue` | 4 | 0 |
| `jdk/internal/loader/ClassLoaders` | 7 | 0 |
| `jdk/internal/math/FloatToDecimal` | 19 | 0 |
| `jdk/internal/misc/InternalLock` | 8 | 0 |
| `jdk/internal/misc/PreviewFeatures` | 5 | 1 |
| `jdk/internal/misc/ScopedMemoryAccess` | 378 | 2 |
| `jdk/internal/misc/Unsafe` | 410 | 68 |
| `jdk/internal/misc/VM` | 35 | 8 |
| `jdk/internal/misc/VirtualThreads` | 6 | 0 |
| `jdk/internal/ref/CleanerFactory` | 3 | 0 |
| `jdk/internal/ref/CleanerImpl$PhantomCleanableRef` | 5 | 0 |
| `jdk/internal/reflect/ConstructorAccessor` | 1 | 0 |
| `jdk/internal/reflect/MethodAccessor` | 2 | 0 |
| `jdk/internal/reflect/Reflection` | 23 | 3 |
| `jdk/internal/reflect/ReflectionFactory` | 40 | 0 |
| `jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction` | 3 | 0 |
| `jdk/internal/util/ArraysSupport` | 31 | 0 |
| `jdk/internal/util/ByteArray` | 23 | 0 |
| `jdk/internal/util/Preconditions` | 17 | 0 |
| `jdk/internal/util/StaticProperty` | 21 | 0 |
| `jdk/internal/util/random/RandomSupport` | 30 | 0 |
| `jdk/internal/util/regex/Grapheme` | 5 | 0 |
| `jdk/internal/vm/Continuation` | 46 | 6 |
| `sun/nio/cs/ISO_8859_1` | 6 | 0 |
| `sun/nio/cs/US_ASCII` | 6 | 0 |
| `sun/nio/cs/UTF_8` | 7 | 0 |
| `sun/security/util/Debug$FormatHolder` | 2 | 0 |
| `sun/security/util/SecurityConstants` | 2 | 0 |

## 仅类级 BFS 拉入（方法级不需要）

| 类名 | 方法数 |
|------|-------:|
| `com/sun/crypto/provider/SunJCE` | 7 |
| `com/sun/crypto/provider/SunJCE$1` | 3 |
| `java/io/BufferedInputStream` | 25 |
| `java/io/BufferedOutputStream` | 12 |
| `java/io/BufferedReader` | 24 |
| `java/io/BufferedReader$1` | 4 |
| `java/io/ByteArrayInputStream` | 14 |
| `java/io/ByteArrayOutputStream` | 15 |
| `java/io/ClassCache` | 4 |
| `java/io/ClassCache$1` | 3 |
| `java/io/ClassCache$CacheRef` | 4 |
| `java/io/Closeable` | 1 |
| `java/io/Console` | 23 |
| `java/io/DataInput` | 15 |
| `java/io/DataOutput` | 14 |
| `java/io/DataOutputStream` | 19 |
| `java/io/DeleteOnExitHook` | 4 |
| `java/io/Externalizable` | 2 |
| `java/io/File` | 64 |
| `java/io/File$TempDirectory` | 5 |
| `java/io/FileCleanable` | 6 |
| `java/io/FileDescriptor` | 17 |
| `java/io/FileFilter` | 1 |
| `java/io/FileInputStream` | 26 |
| `java/io/FileInputStream$1` | 2 |
| `java/io/FileNotFoundException` | 3 |
| `java/io/FileOutputStream` | 17 |
| `java/io/FileOutputStream$1` | 2 |
| `java/io/FilePermission$2` | 3 |
| `java/io/FilePermissionCollection` | 8 |
| `java/io/FileReader` | 5 |
| `java/io/FileSystem` | 30 |
| `java/io/FileWriter` | 9 |
| `java/io/FilenameFilter` | 1 |
| `java/io/FilterOutputStream` | 6 |
| `java/io/Flushable` | 1 |
| `java/io/InputStream$1` | 12 |
| `java/io/InputStreamReader` | 11 |
| `java/io/InterruptedIOException` | 2 |
| `java/io/InvalidClassException` | 5 |
| `java/io/InvalidObjectException` | 2 |
| `java/io/NotActiveException` | 2 |
| `java/io/NotSerializableException` | 2 |
| `java/io/ObjectInput` | 7 |
| `java/io/ObjectInputFilter` | 5 |
| `java/io/ObjectInputFilter$Config` | 13 |
| `java/io/ObjectInputFilter$Config$Global` | 18 |
| `java/io/ObjectInputFilter$Config$MergeFilter` | 3 |
| `java/io/ObjectInputFilter$Config$PredicateFilter` | 3 |
| `java/io/ObjectInputFilter$Config$RejectUndecidedFilter` | 3 |
| `java/io/ObjectInputFilter$Config$RejectUndecidedFilter$SerialInfo` | 6 |
| `java/io/ObjectInputFilter$FilterInfo` | 5 |
| `java/io/ObjectInputFilter$Status` | 5 |
| `java/io/ObjectInputStream` | 66 |
| `java/io/ObjectInputStream$1` | 3 |
| `java/io/ObjectInputStream$BlockDataInputStream` | 43 |
| `java/io/ObjectInputStream$FieldValues` | 15 |
| `java/io/ObjectInputStream$FilterValues` | 6 |
| `java/io/ObjectInputStream$GetField` | 12 |
| `java/io/ObjectInputStream$HandleTable` | 11 |
| `java/io/ObjectInputStream$HandleTable$HandleList` | 4 |
| `java/io/ObjectInputStream$PeekInputStream` | 9 |
| `java/io/ObjectInputStream$ValidationList` | 4 |
| `java/io/ObjectInputStream$ValidationList$1` | 3 |
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
| `java/io/ObjectStreamClass` | 80 |
| `java/io/ObjectStreamClass$1` | 3 |
| `java/io/ObjectStreamClass$2` | 3 |
| `java/io/ObjectStreamClass$3` | 3 |
| `java/io/ObjectStreamClass$4` | 3 |
| `java/io/ObjectStreamClass$5` | 3 |
| `java/io/ObjectStreamClass$ClassDataSlot` | 1 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache` | 4 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key` | 6 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key$Impl` | 4 |
| `java/io/ObjectStreamClass$DeserializationConstructorsCache$Key$Lookup` | 4 |
| `java/io/ObjectStreamClass$ExceptionInfo` | 2 |
| `java/io/ObjectStreamClass$FieldReflector` | 9 |
| `java/io/ObjectStreamClass$FieldReflectorKey` | 3 |
| `java/io/ObjectStreamClass$MemberSignature` | 3 |
| `java/io/ObjectStreamClass$RecordSupport` | 5 |
| `java/io/ObjectStreamException` | 4 |
| `java/io/ObjectStreamField` | 16 |
| `java/io/OptionalDataException` | 2 |
| `java/io/OutputStream$1` | 5 |
| `java/io/PrintWriter` | 67 |
| `java/io/ProxyingConsole` | 11 |
| `java/io/ProxyingConsole$WrappingReader` | 3 |
| `java/io/ProxyingConsole$WrappingWriter` | 4 |
| `java/io/PushbackInputStream` | 16 |
| `java/io/RandomAccessFile` | 55 |
| `java/io/RandomAccessFile$1` | 2 |
| `java/io/Reader` | 16 |
| `java/io/Reader$1` | 9 |
| `java/io/SerialCallbackContext` | 6 |
| `java/io/Serializable` | 0 |
| `java/io/StreamCorruptedException` | 2 |
| `java/io/StreamTokenizer` | 20 |
| `java/io/StringReader` | 10 |
| `java/io/UTFDataFormatException` | 2 |
| `java/io/UncheckedIOException` | 5 |
| `java/io/UnsupportedEncodingException` | 2 |
| `java/io/WriteAbortedException` | 3 |
| `java/io/Writer$1` | 14 |
| `java/lang/AbstractMethodError` | 2 |
| `java/lang/ApplicationShutdownHooks` | 5 |
| `java/lang/ArrayStoreException` | 2 |
| `java/lang/BootstrapMethodError` | 4 |
| `java/lang/CharSequence$1CharIterator` | 5 |
| `java/lang/CharSequence$1CodePointIterator` | 5 |
| `java/lang/Character$Subset` | 4 |
| `java/lang/Class$1` | 3 |
| `java/lang/Class$2` | 3 |
| `java/lang/Class$AnnotationData` | 1 |
| `java/lang/ClassCastException` | 2 |
| `java/lang/ClassFormatError` | 2 |
| `java/lang/ClassLoader$1` | 3 |
| `java/lang/ClassLoader$ParallelLoaders` | 4 |
| `java/lang/ClassNotFoundException` | 7 |
| `java/lang/ClassValue` | 16 |
| `java/lang/ClassValue$ClassValueMap` | 22 |
| `java/lang/ClassValue$Entry` | 10 |
| `java/lang/ClassValue$Identity` | 1 |
| `java/lang/ClassValue$Version` | 4 |
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
| `java/lang/IllegalAccessError` | 2 |
| `java/lang/IllegalAccessException` | 2 |
| `java/lang/IllegalCallerException` | 4 |
| `java/lang/IllegalMonitorStateException` | 2 |
| `java/lang/IllegalThreadStateException` | 2 |
| `java/lang/IncompatibleClassChangeError` | 2 |
| `java/lang/InstantiationException` | 2 |
| `java/lang/InterruptedException` | 2 |
| `java/lang/Iterable` | 3 |
| `java/lang/LayerInstantiationException` | 4 |
| `java/lang/LinkageError` | 3 |
| `java/lang/LiveStackFrame` | 8 |
| `java/lang/LiveStackFrame$PrimitiveSlot` | 4 |
| `java/lang/LiveStackFrameInfo` | 8 |
| `java/lang/LiveStackFrameInfo$PrimitiveSlot32` | 4 |
| `java/lang/LiveStackFrameInfo$PrimitiveSlot64` | 4 |
| `java/lang/MatchException` | 1 |
| `java/lang/Module` | 71 |
| `java/lang/Module$1` | 3 |
| `java/lang/Module$EnableNativeAccess` | 4 |
| `java/lang/ModuleLayer` | 28 |
| `java/lang/ModuleLayer$Controller` | 7 |
| `java/lang/NamedPackage` | 5 |
| `java/lang/NoClassDefFoundError` | 2 |
| `java/lang/NoSuchFieldError` | 2 |
| `java/lang/NoSuchFieldException` | 2 |
| `java/lang/NoSuchMethodError` | 2 |
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
| `java/lang/Short` | 28 |
| `java/lang/Shutdown` | 10 |
| `java/lang/StackFrameInfo` | 16 |
| `java/lang/StackStreamFactory$CallerClassFinder` | 8 |
| `java/lang/StackStreamFactory$CallerClassFinder$ClassBuffer` | 7 |
| `java/lang/StackStreamFactory$FrameBuffer` | 17 |
| `java/lang/StackStreamFactory$LiveStackInfoTraverser$LiveStackFrameBuffer` | 9 |
| `java/lang/StackStreamFactory$StackFrameTraverser$StackFrameBuffer` | 9 |
| `java/lang/StackStreamFactory$WalkerState` | 5 |
| `java/lang/StackTraceElement` | 24 |
| `java/lang/StackTraceElement$HashedModules` | 4 |
| `java/lang/StackWalker$StackFrame` | 10 |
| `java/lang/StringBuffer` | 101 |
| `java/lang/StringLatin1$CharsSpliterator` | 11 |
| `java/lang/StringLatin1$LinesSpliterator` | 10 |
| `java/lang/StringUTF16$CharsSpliterator` | 11 |
| `java/lang/StringUTF16$CodePointsSpliterator` | 12 |
| `java/lang/StringUTF16$LinesSpliterator` | 10 |
| `java/lang/System$1` | 2 |
| `java/lang/System$2` | 87 |
| `java/lang/System$Logger` | 10 |
| `java/lang/System$Logger$Level` | 7 |
| `java/lang/System$LoggerFinder` | 9 |
| `java/lang/Terminator` | 4 |
| `java/lang/Terminator$1` | 2 |
| `java/lang/Thread$1` | 3 |
| `java/lang/Thread$Builder$OfVirtual` | 8 |
| `java/lang/Thread$ThreadIdentifiers` | 3 |
| `java/lang/Thread$ThreadNumbering` | 3 |
| `java/lang/Thread$UncaughtExceptionHandler` | 1 |
| `java/lang/ThreadBuilders` | 2 |
| `java/lang/ThreadBuilders$BaseThreadBuilder` | 10 |
| `java/lang/ThreadBuilders$BaseThreadFactory` | 5 |
| `java/lang/ThreadBuilders$BoundVirtualThread` | 7 |
| `java/lang/ThreadBuilders$PlatformThreadBuilder` | 17 |
| `java/lang/ThreadBuilders$PlatformThreadFactory` | 3 |
| `java/lang/ThreadBuilders$VirtualThreadBuilder` | 13 |
| `java/lang/ThreadBuilders$VirtualThreadFactory` | 2 |
| `java/lang/ThreadGroup` | 40 |
| `java/lang/ThreadLocal` | 25 |
| `java/lang/ThreadLocal$SuppliedThreadLocal` | 2 |
| `java/lang/ThreadLocal$ThreadLocalMap` | 17 |
| `java/lang/ThreadLocal$ThreadLocalMap$Entry` | 1 |
| `java/lang/Throwable$PrintStreamOrWriter` | 4 |
| `java/lang/Throwable$WrappedPrintStream` | 3 |
| `java/lang/Throwable$WrappedPrintWriter` | 3 |
| `java/lang/TypeNotPresentException` | 2 |
| `java/lang/UnsatisfiedLinkError` | 2 |
| `java/lang/UnsupportedClassVersionError` | 2 |
| `java/lang/VersionProps` | 11 |
| `java/lang/VirtualThread$VThreadContinuation` | 3 |
| `java/lang/VirtualThread$VThreadContinuation$1` | 2 |
| `java/lang/Void` | 2 |
| `java/lang/WeakPairMap` | 9 |
| `java/lang/WeakPairMap$Pair` | 6 |
| `java/lang/WeakPairMap$Pair$Lookup` | 5 |
| `java/lang/WeakPairMap$Pair$Weak` | 6 |
| `java/lang/WeakPairMap$Pair$Weak$1` | 2 |
| `java/lang/WeakPairMap$WeakRefPeer` | 2 |
| `java/lang/WrongThreadException` | 4 |
| `java/lang/annotation/Annotation` | 4 |
| `java/lang/annotation/AnnotationFormatError` | 3 |
| `java/lang/annotation/AnnotationTypeMismatchException` | 3 |
| `java/lang/annotation/IncompleteAnnotationException` | 3 |
| `java/lang/annotation/Repeatable` | 1 |
| `java/lang/annotation/Retention` | 1 |
| `java/lang/constant/AsTypeMethodHandleDesc` | 5 |
| `java/lang/constant/ClassDesc` | 20 |
| `java/lang/constant/Constable` | 1 |
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
| `java/lang/invoke/BoundMethodHandle` | 31 |
| `java/lang/invoke/BoundMethodHandle$Specializer` | 8 |
| `java/lang/invoke/BoundMethodHandle$Specializer$Factory` | 2 |
| `java/lang/invoke/BoundMethodHandle$SpeciesData` | 10 |
| `java/lang/invoke/BoundMethodHandle$Species_L` | 11 |
| `java/lang/invoke/CallSite` | 20 |
| `java/lang/invoke/ClassSpecializer` | 22 |
| `java/lang/invoke/ClassSpecializer$Factory` | 18 |
| `java/lang/invoke/ClassSpecializer$Factory$1Var` | 11 |
| `java/lang/invoke/ClassSpecializer$SpeciesData` | 24 |
| `java/lang/invoke/ConstantBootstraps` | 13 |
| `java/lang/invoke/ConstantCallSite` | 6 |
| `java/lang/invoke/DelegatingMethodHandle` | 19 |
| `java/lang/invoke/DirectMethodHandle` | 44 |
| `java/lang/invoke/DirectMethodHandle$1` | 3 |
| `java/lang/invoke/DirectMethodHandle$Accessor` | 6 |
| `java/lang/invoke/DirectMethodHandle$Constructor` | 5 |
| `java/lang/invoke/DirectMethodHandle$Interface` | 6 |
| `java/lang/invoke/DirectMethodHandle$Special` | 7 |
| `java/lang/invoke/DirectMethodHandle$StaticAccessor` | 6 |
| `java/lang/invoke/IndirectVarHandle` | 10 |
| `java/lang/invoke/InfoFromMemberName` | 11 |
| `java/lang/invoke/InfoFromMemberName$1` | 3 |
| `java/lang/invoke/InvokerBytecodeGenerator` | 83 |
| `java/lang/invoke/InvokerBytecodeGenerator$BytecodeGenerationException` | 1 |
| `java/lang/invoke/InvokerBytecodeGenerator$ClassData` | 3 |
| `java/lang/invoke/Invokers` | 40 |
| `java/lang/invoke/LambdaForm` | 83 |
| `java/lang/invoke/LambdaForm$BasicType` | 18 |
| `java/lang/invoke/LambdaForm$Kind` | 6 |
| `java/lang/invoke/LambdaForm$Name` | 38 |
| `java/lang/invoke/LambdaForm$NamedFunction` | 28 |
| `java/lang/invoke/LambdaFormBuffer` | 30 |
| `java/lang/invoke/LambdaFormEditor` | 33 |
| `java/lang/invoke/LambdaFormEditor$1` | 3 |
| `java/lang/invoke/LambdaFormEditor$Transform` | 7 |
| `java/lang/invoke/LambdaFormEditor$TransformKey` | 27 |
| `java/lang/invoke/MemberName` | 83 |
| `java/lang/invoke/MemberName$Factory` | 5 |
| `java/lang/invoke/MethodHandle` | 61 |
| `java/lang/invoke/MethodHandle$1` | 3 |
| `java/lang/invoke/MethodHandleImpl` | 62 |
| `java/lang/invoke/MethodHandleImpl$ArrayAccess` | 9 |
| `java/lang/invoke/MethodHandleImpl$ArrayAccessor` | 33 |
| `java/lang/invoke/MethodHandleImpl$AsVarargsCollector` | 11 |
| `java/lang/invoke/MethodHandleImpl$BindCaller` | 11 |
| `java/lang/invoke/MethodHandleImpl$BindCaller$InjectedInvokerHolder` | 3 |
| `java/lang/invoke/MethodHandleImpl$CasesHolder` | 1 |
| `java/lang/invoke/MethodHandleImpl$CountingWrapper` | 6 |
| `java/lang/invoke/MethodHandleImpl$CountingWrapper$1` | 3 |
| `java/lang/invoke/MethodHandleImpl$Intrinsic` | 5 |
| `java/lang/invoke/MethodHandleImpl$IntrinsicMethodHandle` | 9 |
| `java/lang/invoke/MethodHandleImpl$LoopClauses` | 3 |
| `java/lang/invoke/MethodHandleImpl$TableSwitchCacheKey` | 4 |
| `java/lang/invoke/MethodHandleImpl$WrappedMember` | 6 |
| `java/lang/invoke/MethodHandleInfo` | 9 |
| `java/lang/invoke/MethodHandleNatives` | 50 |
| `java/lang/invoke/MethodHandleStatics` | 17 |
| `java/lang/invoke/MethodHandles` | 113 |
| `java/lang/invoke/MethodHandles$Lookup` | 90 |
| `java/lang/invoke/MethodHandles$Lookup$ClassDefiner` | 8 |
| `java/lang/invoke/MethodHandles$Lookup$ClassFile` | 6 |
| `java/lang/invoke/MethodHandles$Lookup$ClassOption` | 6 |
| `java/lang/invoke/MethodType` | 80 |
| `java/lang/invoke/MethodTypeForm` | 16 |
| `java/lang/invoke/NativeMethodHandle` | 10 |
| `java/lang/invoke/SerializedLambda` | 14 |
| `java/lang/invoke/SerializedLambda$1` | 3 |
| `java/lang/invoke/SimpleMethodHandle` | 12 |
| `java/lang/invoke/VarForm` | 9 |
| `java/lang/invoke/VarHandle` | 59 |
| `java/lang/invoke/VarHandle$AccessDescriptor` | 1 |
| `java/lang/invoke/VarHandle$AccessMode` | 8 |
| `java/lang/invoke/VarHandle$AccessType` | 8 |
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
| `java/lang/invoke/WrongMethodTypeException` | 4 |
| `java/lang/module/Configuration` | 19 |
| `java/lang/module/FindException` | 4 |
| `java/lang/module/ModuleDescriptor` | 38 |
| `java/lang/module/ModuleDescriptor$Builder` | 27 |
| `java/lang/module/ModuleDescriptor$Exports` | 12 |
| `java/lang/module/ModuleDescriptor$Exports$Modifier` | 6 |
| `java/lang/module/ModuleDescriptor$Modifier` | 6 |
| `java/lang/module/ModuleDescriptor$Opens` | 12 |
| `java/lang/module/ModuleDescriptor$Opens$Modifier` | 6 |
| `java/lang/module/ModuleDescriptor$Provides` | 9 |
| `java/lang/module/ModuleDescriptor$Requires` | 13 |
| `java/lang/module/ModuleDescriptor$Requires$Modifier` | 6 |
| `java/lang/module/ModuleDescriptor$Version` | 11 |
| `java/lang/module/ModuleFinder` | 5 |
| `java/lang/module/ModuleFinder$1` | 3 |
| `java/lang/module/ModuleFinder$2` | 7 |
| `java/lang/module/ModuleReference` | 4 |
| `java/lang/module/ResolutionException` | 4 |
| `java/lang/module/ResolvedModule` | 9 |
| `java/lang/module/Resolver` | 40 |
| `java/lang/ref/Cleaner$Cleanable` | 1 |
| `java/lang/ref/Reference$ReferenceHandler` | 2 |
| `java/lang/reflect/AccessFlag` | 10 |
| `java/lang/reflect/AccessibleObject` | 29 |
| `java/lang/reflect/AccessibleObject$Cache` | 3 |
| `java/lang/reflect/AnnotatedArrayType` | 2 |
| `java/lang/reflect/AnnotatedElement` | 8 |
| `java/lang/reflect/AnnotatedParameterizedType` | 2 |
| `java/lang/reflect/AnnotatedType` | 5 |
| `java/lang/reflect/AnnotatedTypeVariable` | 2 |
| `java/lang/reflect/AnnotatedWildcardType` | 3 |
| `java/lang/reflect/Executable` | 49 |
| `java/lang/reflect/Executable$ParameterData` | 6 |
| `java/lang/reflect/Field` | 54 |
| `java/lang/reflect/GenericArrayType` | 1 |
| `java/lang/reflect/GenericDeclaration` | 1 |
| `java/lang/reflect/GenericSignatureFormatError` | 2 |
| `java/lang/reflect/InaccessibleObjectException` | 2 |
| `java/lang/reflect/InvocationHandler` | 2 |
| `java/lang/reflect/InvocationTargetException` | 5 |
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
| `java/lang/reflect/RecordComponent` | 15 |
| `java/lang/reflect/ReflectPermission` | 2 |
| `java/lang/reflect/TypeVariable` | 4 |
| `java/lang/reflect/UndeclaredThrowableException` | 6 |
| `java/lang/reflect/WildcardType` | 2 |
| `java/math/BigDecimal` | 171 |
| `java/math/BigDecimal$LongOverflow` | 3 |
| `java/math/BigDecimal$StringBuilderHelper` | 5 |
| `java/math/BigDecimal$UnsafeHolder` | 4 |
| `java/math/BigInteger` | 159 |
| `java/math/BigInteger$RecursiveOp` | 7 |
| `java/math/BigInteger$RecursiveOp$RecursiveMultiply` | 3 |
| `java/math/BigInteger$RecursiveOp$RecursiveSquare` | 3 |
| `java/math/BigInteger$UnsafeHolder` | 3 |
| `java/math/BitSieve` | 10 |
| `java/math/MathContext` | 10 |
| `java/math/MutableBigInteger` | 83 |
| `java/math/RoundingMode` | 6 |
| `java/math/SignedMutableBigInteger` | 8 |
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
| `java/net/Inet4Address` | 22 |
| `java/net/Inet4AddressImpl` | 9 |
| `java/net/Inet6Address` | 37 |
| `java/net/Inet6Address$Inet6AddressHolder` | 19 |
| `java/net/InetAddress` | 54 |
| `java/net/InetAddress$Addresses` | 1 |
| `java/net/InetAddress$CachedLocalHost` | 1 |
| `java/net/InetAddress$CachedLookup` | 6 |
| `java/net/InetAddress$HostsFileResolver` | 8 |
| `java/net/InetAddress$InetAddressHolder` | 7 |
| `java/net/InetAddress$NameServiceAddresses` | 2 |
| `java/net/InetAddress$PlatformResolver` | 3 |
| `java/net/InetAddress$ValidCachedLookup` | 3 |
| `java/net/InetAddressImpl` | 6 |
| `java/net/InetSocketAddress` | 19 |
| `java/net/InetSocketAddress$InetSocketAddressHolder` | 9 |
| `java/net/InterfaceAddress` | 7 |
| `java/net/JarURLConnection` | 10 |
| `java/net/MalformedURLException` | 2 |
| `java/net/NetPermission` | 2 |
| `java/net/NetworkInterface` | 44 |
| `java/net/NetworkInterface$1` | 3 |
| `java/net/PasswordAuthentication` | 3 |
| `java/net/ProtocolException` | 2 |
| `java/net/ProtocolFamily` | 1 |
| `java/net/Proxy` | 8 |
| `java/net/Proxy$Type` | 5 |
| `java/net/ProxySelector` | 7 |
| `java/net/ProxySelector$StaticProxySelector` | 4 |
| `java/net/Socket` | 72 |
| `java/net/Socket$SocketInputStream` | 5 |
| `java/net/Socket$SocketOutputStream` | 4 |
| `java/net/SocketAddress` | 1 |
| `java/net/SocketException` | 4 |
| `java/net/SocketImpl` | 29 |
| `java/net/SocketImplFactory` | 1 |
| `java/net/SocketOption` | 2 |
| `java/net/SocketPermission` | 31 |
| `java/net/SocketPermission$1` | 3 |
| `java/net/SocketPermissionCollection` | 8 |
| `java/net/SocketTimeoutException` | 2 |
| `java/net/SocksSocketImpl` | 20 |
| `java/net/SocksSocketImpl$1` | 3 |
| `java/net/SocksSocketImpl$2` | 3 |
| `java/net/SocksSocketImpl$3` | 3 |
| `java/net/URI` | 76 |
| `java/net/URI$Parser` | 25 |
| `java/net/URISyntaxException` | 6 |
| `java/net/URL$1` | 5 |
| `java/net/URL$2` | 3 |
| `java/net/URLConnection` | 65 |
| `java/net/URLConnection$1` | 2 |
| `java/net/URLConnection$2` | 3 |
| `java/net/URLPermission` | 13 |
| `java/net/URLPermission$Authority` | 6 |
| `java/net/URLStreamHandlerFactory` | 1 |
| `java/net/UnixDomainSocketAddress` | 10 |
| `java/net/UnixDomainSocketAddress$Ser` | 2 |
| `java/net/UnknownHostException` | 2 |
| `java/net/UnknownServiceException` | 2 |
| `java/net/UrlDeserializedState` | 9 |
| `java/net/spi/InetAddressResolver` | 2 |
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
| `java/nio/StringCharBuffer` | 26 |
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
| `java/nio/channels/ReadableByteChannel` | 1 |
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
| `java/nio/charset/Charset$2` | 3 |
| `java/nio/charset/Charset$3` | 3 |
| `java/nio/charset/CharsetDecoder` | 26 |
| `java/nio/charset/IllegalCharsetNameException` | 2 |
| `java/nio/charset/UnsupportedCharsetException` | 2 |
| `java/nio/charset/spi/CharsetProvider` | 5 |
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
| `java/nio/file/FileSystem` | 13 |
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
| `java/nio/file/Path` | 32 |
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
| `java/security/AlgorithmParameters` | 13 |
| `java/security/AlgorithmParametersSpi` | 8 |
| `java/security/AllPermissionCollection` | 4 |
| `java/security/AllPermissionCollection$1` | 4 |
| `java/security/BasicPermissionCollection` | 7 |
| `java/security/CodeSigner` | 7 |
| `java/security/DigestException` | 4 |
| `java/security/GeneralSecurityException` | 4 |
| `java/security/InvalidAlgorithmParameterException` | 4 |
| `java/security/InvalidKeyException` | 4 |
| `java/security/InvalidParameterException` | 4 |
| `java/security/Key` | 3 |
| `java/security/KeyException` | 4 |
| `java/security/KeyFactory` | 13 |
| `java/security/KeyFactorySpi` | 5 |
| `java/security/KeyStore` | 35 |
| `java/security/KeyStore$CallbackHandlerProtection` | 2 |
| `java/security/KeyStore$Entry` | 1 |
| `java/security/KeyStore$LoadStoreParameter` | 1 |
| `java/security/KeyStore$PasswordProtection` | 7 |
| `java/security/KeyStore$PrivateKeyEntry` | 7 |
| `java/security/KeyStore$SecretKeyEntry` | 5 |
| `java/security/KeyStore$TrustedCertificateEntry` | 5 |
| `java/security/KeyStoreException` | 4 |
| `java/security/KeyStoreSpi` | 25 |
| `java/security/MessageDigest` | 21 |
| `java/security/MessageDigest$Delegate` | 11 |
| `java/security/MessageDigest$Delegate$CloneableDelegate` | 1 |
| `java/security/MessageDigestSpi` | 9 |
| `java/security/NoSuchAlgorithmException` | 4 |
| `java/security/NoSuchProviderException` | 2 |
| `java/security/PermissionsEnumerator` | 5 |
| `java/security/Policy$PolicyDelegate` | 8 |
| `java/security/PolicySpi` | 5 |
| `java/security/Principal` | 5 |
| `java/security/PrivateKey` | 0 |
| `java/security/PrivilegedActionException` | 6 |
| `java/security/PrivilegedExceptionAction` | 1 |
| `java/security/ProtectionDomain$Key` | 1 |
| `java/security/Provider` | 66 |
| `java/security/Provider$EngineDescription` | 1 |
| `java/security/Provider$OPType` | 5 |
| `java/security/Provider$Service` | 25 |
| `java/security/Provider$ServiceKey` | 5 |
| `java/security/Provider$UString` | 4 |
| `java/security/ProviderException` | 4 |
| `java/security/PublicKey` | 0 |
| `java/security/SecureRandom` | 30 |
| `java/security/SecureRandomParameters` | 0 |
| `java/security/SecureRandomSpi` | 9 |
| `java/security/Security` | 29 |
| `java/security/Security$Criteria` | 3 |
| `java/security/Security$ProviderProperty` | 1 |
| `java/security/SecurityPermission` | 2 |
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
| `java/security/cert/CertPath` | 10 |
| `java/security/cert/CertPath$CertPathRep` | 2 |
| `java/security/cert/CertPathValidatorException` | 10 |
| `java/security/cert/Certificate$CertificateRep` | 2 |
| `java/security/cert/CertificateEncodingException` | 4 |
| `java/security/cert/CertificateException` | 4 |
| `java/security/cert/CertificateExpiredException` | 2 |
| `java/security/cert/CertificateFactory` | 14 |
| `java/security/cert/CertificateFactorySpi` | 9 |
| `java/security/cert/CertificateNotYetValidException` | 2 |
| `java/security/cert/CertificateParsingException` | 4 |
| `java/security/cert/Extension` | 4 |
| `java/security/cert/PolicyQualifierInfo` | 5 |
| `java/security/cert/X509CRL` | 20 |
| `java/security/cert/X509CRLEntry` | 10 |
| `java/security/cert/X509Certificate` | 24 |
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
| `java/security/spec/EncodedKeySpec` | 7 |
| `java/security/spec/InvalidKeySpecException` | 4 |
| `java/security/spec/InvalidParameterSpecException` | 2 |
| `java/security/spec/MGF1ParameterSpec` | 4 |
| `java/security/spec/NamedParameterSpec` | 3 |
| `java/security/spec/PKCS8EncodedKeySpec` | 4 |
| `java/security/spec/PSSParameterSpec` | 9 |
| `java/security/spec/X509EncodedKeySpec` | 4 |
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
| `java/time/Duration` | 75 |
| `java/time/MonthDay` | 36 |
| `java/time/OffsetDateTime` | 95 |
| `java/time/OffsetTime` | 69 |
| `java/time/Period` | 52 |
| `java/time/Ser` | 8 |
| `java/time/Year` | 52 |
| `java/time/YearMonth` | 59 |
| `java/time/ZoneId$1` | 4 |
| `java/time/ZoneRegion` | 11 |
| `java/time/ZonedDateTime` | 104 |
| `java/time/chrono/AbstractChronology` | 29 |
| `java/time/chrono/ChronoLocalDateImpl` | 28 |
| `java/time/chrono/ChronoLocalDateTime` | 35 |
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
| `java/time/temporal/ChronoUnit` | 13 |
| `java/time/temporal/Temporal` | 8 |
| `java/time/temporal/TemporalAmount` | 4 |
| `java/time/temporal/TemporalUnit` | 8 |
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
| `java/util/AbstractSequentialList` | 8 |
| `java/util/ArrayDeque` | 64 |
| `java/util/ArrayDeque$DeqIterator` | 6 |
| `java/util/ArrayDeque$DeqSpliterator` | 9 |
| `java/util/ArrayDeque$DescendingIterator` | 4 |
| `java/util/ArrayList$ArrayListSpliterator` | 8 |
| `java/util/ArrayList$Itr` | 6 |
| `java/util/ArrayList$ListItr` | 7 |
| `java/util/ArrayList$SubList` | 30 |
| `java/util/ArrayList$SubList$1` | 12 |
| `java/util/ArrayList$SubList$2` | 8 |
| `java/util/ArrayPrefixHelpers$CumulateTask` | 3 |
| `java/util/ArrayPrefixHelpers$DoubleCumulateTask` | 3 |
| `java/util/ArrayPrefixHelpers$IntCumulateTask` | 3 |
| `java/util/ArrayPrefixHelpers$LongCumulateTask` | 3 |
| `java/util/Arrays$ArrayItr` | 3 |
| `java/util/Arrays$ArrayList` | 13 |
| `java/util/ArraysParallelSortHelpers$EmptyCompleter` | 2 |
| `java/util/ArraysParallelSortHelpers$FJObject$Merger` | 2 |
| `java/util/ArraysParallelSortHelpers$FJObject$Sorter` | 2 |
| `java/util/ArraysParallelSortHelpers$Relay` | 3 |
| `java/util/Base64` | 8 |
| `java/util/Base64$DecInputStream` | 8 |
| `java/util/Base64$Decoder` | 10 |
| `java/util/Base64$EncOutputStream` | 6 |
| `java/util/Base64$Encoder` | 11 |
| `java/util/BitSet` | 50 |
| `java/util/BitSet$1BitSetSpliterator` | 12 |
| `java/util/Calendar` | 91 |
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
| `java/util/Collections$CopiesList` | 19 |
| `java/util/Collections$ReverseComparator2` | 6 |
| `java/util/Collections$SequencedSetFromMap` | 11 |
| `java/util/Collections$SetFromMap` | 23 |
| `java/util/Collections$SingletonList` | 11 |
| `java/util/Collections$SingletonMap` | 21 |
| `java/util/Collections$SingletonSet` | 8 |
| `java/util/Collections$SynchronizedCollection` | 23 |
| `java/util/Collections$SynchronizedList` | 17 |
| `java/util/Collections$SynchronizedNavigableMap` | 25 |
| `java/util/Collections$SynchronizedNavigableSet` | 19 |
| `java/util/Collections$SynchronizedRandomAccessList` | 4 |
| `java/util/Collections$SynchronizedSet` | 4 |
| `java/util/Collections$SynchronizedSortedMap` | 8 |
| `java/util/Collections$SynchronizedSortedSet` | 8 |
| `java/util/Collections$UnmodifiableCollection` | 21 |
| `java/util/Collections$UnmodifiableCollection$1` | 5 |
| `java/util/Collections$UnmodifiableList` | 16 |
| `java/util/Collections$UnmodifiableList$1` | 11 |
| `java/util/Collections$UnmodifiableMap` | 27 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet` | 13 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1` | 6 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry` | 7 |
| `java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntrySetSpliterator` | 9 |
| `java/util/Collections$UnmodifiableNavigableMap` | 20 |
| `java/util/Collections$UnmodifiableNavigableSet` | 13 |
| `java/util/Collections$UnmodifiableRandomAccessList` | 3 |
| `java/util/Collections$UnmodifiableSequencedCollection` | 9 |
| `java/util/Collections$UnmodifiableSequencedMap` | 7 |
| `java/util/Collections$UnmodifiableSequencedSet` | 6 |
| `java/util/Collections$UnmodifiableSet` | 3 |
| `java/util/Collections$UnmodifiableSortedMap` | 7 |
| `java/util/Collections$UnmodifiableSortedSet` | 7 |
| `java/util/ComparableTimSort` | 16 |
| `java/util/Comparator` | 25 |
| `java/util/Comparators$NullComparator` | 4 |
| `java/util/ConcurrentModificationException` | 4 |
| `java/util/Currency` | 30 |
| `java/util/Currency$1` | 3 |
| `java/util/Currency$CurrencyProperty` | 8 |
| `java/util/Currency$OtherCurrencyEntry` | 2 |
| `java/util/Currency$SpecialCaseEntry` | 6 |
| `java/util/Date` | 49 |
| `java/util/Deque` | 30 |
| `java/util/Dictionary` | 8 |
| `java/util/DoubleSummaryStatistics` | 11 |
| `java/util/DualPivotQuicksort` | 49 |
| `java/util/DualPivotQuicksort$Merger` | 3 |
| `java/util/DualPivotQuicksort$RunMerger` | 4 |
| `java/util/DualPivotQuicksort$Sorter` | 4 |
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
| `java/util/EnumSet` | 23 |
| `java/util/EnumSet$SerializationProxy` | 3 |
| `java/util/Enumeration$1` | 3 |
| `java/util/Formattable` | 1 |
| `java/util/Formatter$FormatSpecifier$BigDecimalLayout` | 6 |
| `java/util/GregorianCalendar` | 62 |
| `java/util/HashMap$EntryIterator` | 3 |
| `java/util/HashMap$EntrySet` | 8 |
| `java/util/HashMap$EntrySpliterator` | 6 |
| `java/util/HashMap$HashIterator` | 4 |
| `java/util/HashMap$HashMapSpliterator` | 3 |
| `java/util/HashMap$KeyIterator` | 2 |
| `java/util/HashMap$KeySpliterator` | 6 |
| `java/util/HashMap$UnsafeHolder` | 3 |
| `java/util/HashMap$ValueIterator` | 2 |
| `java/util/HashMap$ValueSpliterator` | 6 |
| `java/util/HashMap$Values` | 9 |
| `java/util/Hashtable` | 46 |
| `java/util/Hashtable$Entry` | 8 |
| `java/util/Hashtable$EntrySet` | 8 |
| `java/util/Hashtable$Enumerator` | 6 |
| `java/util/Hashtable$KeySet` | 6 |
| `java/util/Hashtable$UnsafeHolder` | 3 |
| `java/util/Hashtable$ValueCollection` | 5 |
| `java/util/HexFormat` | 45 |
| `java/util/IdentityHashMap` | 36 |
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
| `java/util/IllformedLocaleException` | 4 |
| `java/util/ImmutableCollections$AbstractImmutableCollection` | 8 |
| `java/util/ImmutableCollections$AbstractImmutableList` | 18 |
| `java/util/ImmutableCollections$AbstractImmutableSet` | 3 |
| `java/util/ImmutableCollections$List12` | 11 |
| `java/util/ImmutableCollections$ListItr` | 11 |
| `java/util/ImmutableCollections$ListN` | 10 |
| `java/util/ImmutableCollections$MapN$1` | 3 |
| `java/util/ImmutableCollections$MapN$MapNIterator` | 5 |
| `java/util/ImmutableCollections$Set12` | 11 |
| `java/util/ImmutableCollections$Set12$1` | 3 |
| `java/util/ImmutableCollections$SetN` | 11 |
| `java/util/ImmutableCollections$SetN$SetNIterator` | 3 |
| `java/util/ImmutableCollections$SubList` | 15 |
| `java/util/InputMismatchException` | 2 |
| `java/util/IntSummaryStatistics` | 10 |
| `java/util/JapaneseImperialCalendar` | 40 |
| `java/util/JumboEnumSet` | 20 |
| `java/util/JumboEnumSet$EnumSetIterator` | 5 |
| `java/util/KeyValueHolder` | 7 |
| `java/util/LinkedHashMap$Entry` | 1 |
| `java/util/LinkedHashMap$LinkedEntryIterator` | 3 |
| `java/util/LinkedHashMap$LinkedEntrySet` | 23 |
| `java/util/LinkedHashMap$LinkedHashIterator` | 4 |
| `java/util/LinkedHashMap$LinkedKeyIterator` | 2 |
| `java/util/LinkedHashMap$LinkedKeySet` | 18 |
| `java/util/LinkedHashMap$LinkedValueIterator` | 2 |
| `java/util/LinkedHashMap$LinkedValues` | 16 |
| `java/util/LinkedHashMap$ReversedLinkedHashMapView` | 33 |
| `java/util/LinkedHashSet$1ReverseLinkedHashSetView` | 14 |
| `java/util/LinkedList` | 61 |
| `java/util/LinkedList$DescendingIterator` | 4 |
| `java/util/LinkedList$LLSpliterator` | 7 |
| `java/util/LinkedList$ListItr` | 12 |
| `java/util/LinkedList$Node` | 1 |
| `java/util/LinkedList$ReverseOrderLinkedListView` | 63 |
| `java/util/ListIterator` | 9 |
| `java/util/ListResourceBundle` | 6 |
| `java/util/Locale$Builder` | 14 |
| `java/util/Locale$IsoCountryCode` | 7 |
| `java/util/Locale$LanguageRange` | 11 |
| `java/util/LongSummaryStatistics` | 11 |
| `java/util/MissingResourceException` | 4 |
| `java/util/NavigableMap` | 24 |
| `java/util/NavigableSet` | 21 |
| `java/util/OptionalDouble` | 18 |
| `java/util/OptionalInt` | 18 |
| `java/util/OptionalLong` | 18 |
| `java/util/PrimitiveIterator$OfDouble` | 6 |
| `java/util/PrimitiveIterator$OfInt` | 6 |
| `java/util/PrimitiveIterator$OfLong` | 6 |
| `java/util/Properties` | 62 |
| `java/util/Properties$EntrySet` | 18 |
| `java/util/Properties$LineReader` | 3 |
| `java/util/PropertyPermission` | 13 |
| `java/util/PropertyPermissionCollection` | 8 |
| `java/util/PropertyResourceBundle` | 6 |
| `java/util/Queue` | 6 |
| `java/util/Random$RandomWrapper` | 36 |
| `java/util/RandomAccess` | 0 |
| `java/util/RegularEnumSet` | 17 |
| `java/util/RegularEnumSet$EnumSetIterator` | 5 |
| `java/util/ResourceBundle` | 50 |
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
| `java/util/ServiceLoader` | 25 |
| `java/util/ServiceLoader$1` | 3 |
| `java/util/ServiceLoader$2` | 4 |
| `java/util/ServiceLoader$3` | 4 |
| `java/util/ServiceLoader$LayerLookupIterator` | 6 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator` | 10 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator$1` | 3 |
| `java/util/ServiceLoader$LazyClassPathLookupIterator$2` | 3 |
| `java/util/ServiceLoader$ModuleServicesLookupIterator` | 8 |
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
| `java/util/Spliterators$DoubleArraySpliterator` | 13 |
| `java/util/Spliterators$DoubleIteratorSpliterator` | 12 |
| `java/util/Spliterators$IntArraySpliterator` | 13 |
| `java/util/Spliterators$IntIteratorSpliterator` | 12 |
| `java/util/Spliterators$IteratorSpliterator` | 9 |
| `java/util/Spliterators$LongArraySpliterator` | 13 |
| `java/util/Spliterators$LongIteratorSpliterator` | 12 |
| `java/util/Stack` | 6 |
| `java/util/TimSort` | 16 |
| `java/util/TimeZone` | 34 |
| `java/util/TreeMap$AscendingSubMap` | 16 |
| `java/util/TreeMap$AscendingSubMap$AscendingEntrySetView` | 2 |
| `java/util/TreeMap$DescendingKeyIterator` | 3 |
| `java/util/TreeMap$DescendingKeySpliterator` | 6 |
| `java/util/TreeMap$DescendingSubMap` | 16 |
| `java/util/TreeMap$DescendingSubMap$DescendingEntrySetView` | 2 |
| `java/util/TreeMap$Entry` | 7 |
| `java/util/TreeMap$EntryIterator` | 3 |
| `java/util/TreeMap$EntrySet` | 7 |
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
| `java/util/Vector` | 69 |
| `java/util/Vector$1` | 3 |
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
| `java/util/concurrent/Callable` | 1 |
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
| `java/util/concurrent/ConcurrentHashMap$CollectionView` | 14 |
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
| `java/util/concurrent/ConcurrentHashMap$KeySetView` | 13 |
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
| `java/util/concurrent/CopyOnWriteArrayList` | 70 |
| `java/util/concurrent/CopyOnWriteArrayList$COWIterator` | 11 |
| `java/util/concurrent/CopyOnWriteArrayList$COWSubList` | 47 |
| `java/util/concurrent/CopyOnWriteArrayList$COWSubListIterator` | 11 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed` | 44 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed$DescendingIterator` | 4 |
| `java/util/concurrent/CopyOnWriteArrayList$Reversed$DescendingListIterator` | 10 |
| `java/util/concurrent/CountDownLatch` | 6 |
| `java/util/concurrent/CountDownLatch$Sync` | 4 |
| `java/util/concurrent/CountedCompleter` | 26 |
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
| `java/util/concurrent/ForkJoinTask` | 68 |
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
| `java/util/concurrent/RecursiveTask` | 5 |
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
| `java/util/concurrent/locks/ReentrantLock$FairSync` | 3 |
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
| `java/util/function/IntFunction` | 1 |
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
| `java/util/function/Supplier` | 1 |
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
| `java/util/stream/Collector` | 7 |
| `java/util/stream/Collectors$1OptionalBox` | 2 |
| `java/util/stream/Collectors$1PairBox` | 4 |
| `java/util/stream/Collectors$Partition` | 7 |
| `java/util/stream/Collectors$Partition$1` | 3 |
| `java/util/stream/DistinctOps` | 2 |
| `java/util/stream/DistinctOps$1` | 6 |
| `java/util/stream/DistinctOps$1$1` | 4 |
| `java/util/stream/DistinctOps$1$2` | 4 |
| `java/util/stream/DoublePipeline` | 62 |
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
| `java/util/stream/DoublePipeline$Head` | 12 |
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
| `java/util/stream/IntPipeline` | 61 |
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
| `java/util/stream/IntPipeline$Head` | 12 |
| `java/util/stream/IntPipeline$StatefulOp` | 10 |
| `java/util/stream/IntPipeline$StatelessOp` | 9 |
| `java/util/stream/IntStream` | 53 |
| `java/util/stream/IntStream$1` | 3 |
| `java/util/stream/IntStream$2` | 5 |
| `java/util/stream/IntStream$IntMapMultiConsumer` | 1 |
| `java/util/stream/LongPipeline` | 60 |
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
| `java/util/stream/LongPipeline$Head` | 12 |
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
| `java/util/stream/Streams$AbstractStreamBuilderImpl` | 4 |
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
| `java/util/stream/Streams$StreamBuilderImpl` | 7 |
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
| `java/util/zip/CRC32` | 14 |
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
| `javax/security/auth/callback/Callback` | 0 |
| `javax/security/auth/callback/CallbackHandler` | 1 |
| `javax/security/auth/callback/PasswordCallback` | 9 |
| `javax/security/auth/x500/X500Principal` | 14 |
| `sun/invoke/util/BytecodeDescriptor` | 10 |
| `sun/invoke/util/BytecodeName` | 22 |
| `sun/invoke/util/ValueConversions` | 101 |
| `sun/invoke/util/ValueConversions$WrapperCache` | 3 |
| `sun/invoke/util/VerifyAccess` | 16 |
| `sun/invoke/util/VerifyAccess$1` | 3 |
| `sun/invoke/util/VerifyType` | 4 |
| `sun/invoke/util/Wrapper` | 56 |
| `sun/net/ApplicationProxy` | 2 |
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
| `sun/net/util/IPAddressUtil` | 36 |
| `sun/net/util/SocketExceptions` | 6 |
| `sun/net/util/SocketExceptions$1` | 3 |
| `sun/net/util/URLUtil` | 4 |
| `sun/net/www/MessageHeader` | 31 |
| `sun/net/www/MessageHeader$HeaderIterator` | 5 |
| `sun/net/www/MimeEntry` | 23 |
| `sun/net/www/MimeTable` | 24 |
| `sun/net/www/ParseUtil` | 22 |
| `sun/net/www/URLConnection` | 20 |
| `sun/net/www/protocol/file/FileURLConnection` | 15 |
| `sun/net/www/protocol/jar/Handler` | 12 |
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
| `sun/nio/cs/ArrayDecoder` | 4 |
| `sun/nio/cs/CESU_8` | 6 |
| `sun/nio/cs/CESU_8$Decoder` | 17 |
| `sun/nio/cs/CESU_8$Encoder` | 11 |
| `sun/nio/cs/HistoricallyNamedCharset` | 1 |
| `sun/nio/cs/ISO_8859_1$Decoder` | 5 |
| `sun/nio/cs/ISO_8859_1$Encoder` | 10 |
| `sun/nio/cs/ISO_8859_15` | 5 |
| `sun/nio/cs/ISO_8859_16` | 5 |
| `sun/nio/cs/MS1252` | 5 |
| `sun/nio/cs/SingleByte` | 4 |
| `sun/nio/cs/SingleByte$Decoder` | 12 |
| `sun/nio/cs/SingleByte$Encoder` | 12 |
| `sun/nio/cs/StandardCharsets` | 52 |
| `sun/nio/cs/StandardCharsets$1` | 5 |
| `sun/nio/cs/StandardCharsets$Aliases` | 2 |
| `sun/nio/cs/StandardCharsets$Cache` | 2 |
| `sun/nio/cs/StandardCharsets$Classes` | 2 |
| `sun/nio/cs/StreamDecoder` | 28 |
| `sun/nio/cs/StringUTF16` | 3 |
| `sun/nio/cs/Surrogate$Parser` | 9 |
| `sun/nio/cs/ThreadLocalCoders` | 4 |
| `sun/nio/cs/ThreadLocalCoders$Cache` | 5 |
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
| `sun/nio/cs/UTF_32BE` | 5 |
| `sun/nio/cs/UTF_32BE_BOM` | 5 |
| `sun/nio/cs/UTF_32Coder$Decoder` | 4 |
| `sun/nio/cs/UTF_32Coder$Encoder` | 4 |
| `sun/nio/cs/UTF_32LE` | 5 |
| `sun/nio/cs/UTF_32LE_BOM` | 5 |
| `sun/nio/cs/UTF_8$Decoder` | 18 |
| `sun/nio/cs/UTF_8$Encoder` | 9 |
| `sun/nio/cs/Unicode` | 2 |
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
| `sun/security/action/GetPropertyAction` | 9 |
| `sun/security/action/GetPropertyAction$1` | 3 |
| `sun/security/jca/GetInstance` | 16 |
| `sun/security/jca/GetInstance$Instance` | 2 |
| `sun/security/jca/JCAUtil` | 7 |
| `sun/security/jca/ProviderConfig` | 15 |
| `sun/security/jca/ProviderConfig$1` | 3 |
| `sun/security/jca/ProviderConfig$2` | 3 |
| `sun/security/jca/ProviderConfig$3` | 3 |
| `sun/security/jca/ProviderConfig$4` | 3 |
| `sun/security/jca/ProviderConfig$ProviderLoader` | 4 |
| `sun/security/jca/ProviderConfig$ProviderLoader$1` | 3 |
| `sun/security/jca/ProviderList` | 23 |
| `sun/security/jca/ProviderList$2` | 3 |
| `sun/security/jca/ProviderList$3` | 4 |
| `sun/security/jca/ProviderList$PreferredEntry` | 4 |
| `sun/security/jca/ProviderList$PreferredList` | 8 |
| `sun/security/jca/ProviderList$ServiceList` | 9 |
| `sun/security/jca/ProviderList$ServiceList$1` | 5 |
| `sun/security/jca/Providers` | 15 |
| `sun/security/jca/ServiceId` | 1 |
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
| `sun/security/provider/NativePRNG` | 8 |
| `sun/security/provider/NativePRNG$1` | 3 |
| `sun/security/provider/NativePRNG$Blocking` | 6 |
| `sun/security/provider/NativePRNG$NonBlocking` | 6 |
| `sun/security/provider/NativePRNG$RandomIO` | 7 |
| `sun/security/provider/NativePRNG$RandomIO$1` | 3 |
| `sun/security/provider/NativePRNG$Variant` | 5 |
| `sun/security/provider/PolicyFile$6` | 3 |
| `sun/security/provider/PolicyFile$7` | 3 |
| `sun/security/provider/PolicyFile$8` | 3 |
| `sun/security/provider/PolicyFile$PolicyEntry` | 6 |
| `sun/security/provider/PolicyFile$SelfPermission` | 11 |
| `sun/security/provider/PolicyParser` | 33 |
| `sun/security/provider/PolicyParser$DomainEntry` | 6 |
| `sun/security/provider/PolicyParser$GrantEntry` | 10 |
| `sun/security/provider/PolicyParser$KeyStoreEntry` | 4 |
| `sun/security/provider/PolicyParser$ParsingException` | 5 |
| `sun/security/provider/PolicyParser$PermissionEntry` | 5 |
| `sun/security/provider/PolicyParser$PrincipalEntry` | 14 |
| `sun/security/provider/SHA3` | 12 |
| `sun/security/provider/SHAKE256` | 6 |
| `sun/security/provider/SecureRandom` | 8 |
| `sun/security/provider/SeedGenerator` | 7 |
| `sun/security/provider/SeedGenerator$1` | 3 |
| `sun/security/provider/Sun` | 2 |
| `sun/security/provider/Sun$1` | 3 |
| `sun/security/provider/SunEntries` | 8 |
| `sun/security/provider/VerificationProvider` | 3 |
| `sun/security/provider/VerificationProvider$1` | 3 |
| `sun/security/provider/X509Factory` | 21 |
| `sun/security/provider/certpath/X509CertPath` | 14 |
| `sun/security/provider/certpath/X509CertificatePair` | 15 |
| `sun/security/rsa/RSAUtil` | 9 |
| `sun/security/rsa/RSAUtil$KeyType` | 6 |
| `sun/security/rsa/SunRsaSign` | 2 |
| `sun/security/rsa/SunRsaSign$1` | 3 |
| `sun/security/rsa/SunRsaSignEntries` | 4 |
| `sun/security/ssl/SSLLogger` | 14 |
| `sun/security/ssl/SSLLogger$SSLSimpleFormatter` | 14 |
| `sun/security/ssl/SSLScope` | 6 |
| `sun/security/ssl/SunJSSE` | 5 |
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
| `sun/security/util/BitArray` | 18 |
| `sun/security/util/Cache` | 17 |
| `sun/security/util/Cache$CacheVisitor` | 1 |
| `sun/security/util/Cache$EqualByteArray` | 3 |
| `sun/security/util/ConstraintsParameters` | 5 |
| `sun/security/util/CryptoAlgorithmConstraints` | 9 |
| `sun/security/util/CurveDB` | 7 |
| `sun/security/util/DerEncoder` | 1 |
| `sun/security/util/DerIndefLenConverter` | 15 |
| `sun/security/util/DerInputStream` | 40 |
| `sun/security/util/DerOutputStream` | 39 |
| `sun/security/util/DerValue` | 67 |
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
| `sun/security/util/HexDumpEncoder` | 17 |
| `sun/security/util/IOUtils` | 2 |
| `sun/security/util/JarConstraintsParameters` | 11 |
| `sun/security/util/KeyUtil` | 12 |
| `sun/security/util/KnownOIDs` | 12 |
| `sun/security/util/Length` | 1 |
| `sun/security/util/LocalizedMessage` | 6 |
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
| `sun/security/util/ObjectIdentifier` | 28 |
| `sun/security/util/Password` | 3 |
| `sun/security/util/Password$ConsoleHolder` | 5 |
| `sun/security/util/Pem` | 2 |
| `sun/security/util/PolicyUtil` | 3 |
| `sun/security/util/PropertyExpander` | 3 |
| `sun/security/util/PropertyExpander$ExpandException` | 1 |
| `sun/security/util/RegisteredDomain` | 4 |
| `sun/security/util/Resources` | 3 |
| `sun/security/util/ResourcesMgr` | 5 |
| `sun/security/util/SafeDHParameterSpec` | 2 |
| `sun/security/util/SecurityProperties` | 6 |
| `sun/security/util/SecurityProviderConstants` | 7 |
| `sun/security/util/SignatureFileVerifier` | 23 |
| `sun/security/util/SignatureUtil` | 20 |
| `sun/security/x509/AVA` | 33 |
| `sun/security/x509/AVAComparator` | 5 |
| `sun/security/x509/AVAKeyword` | 7 |
| `sun/security/x509/AccessDescription` | 9 |
| `sun/security/x509/AlgorithmId` | 27 |
| `sun/security/x509/AuthorityInfoAccessExtension` | 7 |
| `sun/security/x509/AuthorityKeyIdentifierExtension` | 10 |
| `sun/security/x509/BasicConstraintsExtension` | 9 |
| `sun/security/x509/CRLDistributionPointsExtension` | 11 |
| `sun/security/x509/CRLExtensions` | 14 |
| `sun/security/x509/CRLNumberExtension` | 11 |
| `sun/security/x509/CRLReasonCodeExtension` | 10 |
| `sun/security/x509/CertificateAlgorithmId` | 6 |
| `sun/security/x509/CertificateExtensions` | 17 |
| `sun/security/x509/CertificateIssuerExtension` | 7 |
| `sun/security/x509/CertificatePoliciesExtension` | 8 |
| `sun/security/x509/CertificatePolicyId` | 7 |
| `sun/security/x509/CertificatePolicyMap` | 6 |
| `sun/security/x509/CertificateSerialNumber` | 9 |
| `sun/security/x509/CertificateValidity` | 8 |
| `sun/security/x509/CertificateVersion` | 10 |
| `sun/security/x509/CertificateX509Key` | 6 |
| `sun/security/x509/DNSName` | 11 |
| `sun/security/x509/DeltaCRLIndicatorExtension` | 4 |
| `sun/security/x509/DistributionPoint` | 13 |
| `sun/security/x509/DistributionPointName` | 9 |
| `sun/security/x509/EDIPartyName` | 12 |
| `sun/security/x509/ExtendedKeyUsageExtension` | 9 |
| `sun/security/x509/Extension` | 16 |
| `sun/security/x509/GeneralName` | 9 |
| `sun/security/x509/GeneralNameInterface` | 3 |
| `sun/security/x509/GeneralNames` | 12 |
| `sun/security/x509/GeneralSubtree` | 9 |
| `sun/security/x509/GeneralSubtrees` | 22 |
| `sun/security/x509/IPAddressName` | 14 |
| `sun/security/x509/IssuerAlternativeNameExtension` | 8 |
| `sun/security/x509/IssuingDistributionPointExtension` | 13 |
| `sun/security/x509/KeyIdentifier` | 8 |
| `sun/security/x509/KeyUsageExtension` | 14 |
| `sun/security/x509/NameConstraintsExtension` | 14 |
| `sun/security/x509/OIDMap` | 8 |
| `sun/security/x509/OIDMap$OIDInfo` | 3 |
| `sun/security/x509/OIDName` | 11 |
| `sun/security/x509/OtherName` | 12 |
| `sun/security/x509/PolicyConstraintsExtension` | 9 |
| `sun/security/x509/PolicyInformation` | 8 |
| `sun/security/x509/PolicyMappingsExtension` | 7 |
| `sun/security/x509/PrivateKeyUsageExtension` | 10 |
| `sun/security/x509/RDN` | 21 |
| `sun/security/x509/RFC822Name` | 11 |
| `sun/security/x509/ReasonFlags` | 14 |
| `sun/security/x509/SerialNumber` | 9 |
| `sun/security/x509/SubjectAlternativeNameExtension` | 8 |
| `sun/security/x509/SubjectKeyIdentifierExtension` | 7 |
| `sun/security/x509/URIName` | 16 |
| `sun/security/x509/UniqueIdentity` | 7 |
| `sun/security/x509/UnparseableExtension` | 3 |
| `sun/security/x509/X400Address` | 6 |
| `sun/security/x509/X500Name` | 60 |
| `sun/security/x509/X509CRLEntryImpl` | 27 |
| `sun/security/x509/X509CRLImpl` | 47 |
| `sun/security/x509/X509CRLImpl$TBSCertList` | 7 |
| `sun/security/x509/X509CRLImpl$X509IssuerSerial` | 8 |
| `sun/security/x509/X509CertInfo` | 33 |
| `sun/security/x509/X509Key` | 22 |
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
| `sun/util/calendar/AbstractCalendar` | 21 |
| `sun/util/calendar/BaseCalendar` | 19 |
| `sun/util/calendar/BaseCalendar$Date` | 10 |
| `sun/util/calendar/CalendarDate` | 45 |
| `sun/util/calendar/CalendarSystem` | 21 |
| `sun/util/calendar/CalendarUtils` | 12 |
| `sun/util/calendar/Era` | 10 |
| `sun/util/calendar/Gregorian` | 14 |
| `sun/util/calendar/Gregorian$Date` | 4 |
| `sun/util/calendar/ImmutableGregorianDate` | 47 |
| `sun/util/calendar/JulianCalendar` | 20 |
| `sun/util/calendar/JulianCalendar$Date` | 8 |
| `sun/util/calendar/LocalGregorianCalendar` | 28 |
| `sun/util/calendar/LocalGregorianCalendar$Date` | 13 |
| `sun/util/calendar/ZoneInfo` | 30 |
| `sun/util/calendar/ZoneInfoFile` | 22 |
| `sun/util/calendar/ZoneInfoFile$1` | 3 |
| `sun/util/calendar/ZoneInfoFile$Checksum` | 3 |
| `sun/util/calendar/ZoneInfoFile$ZoneOffsetTransitionRule` | 8 |
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
| `sun/util/locale/provider/CalendarDataUtility` | 10 |
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
| `sun/util/locale/provider/LocaleServiceProviderPool` | 14 |
| `sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter` | 1 |
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

## 仅方法级 BFS 发现（类级未发现）

- `java/lang/Character$CharacterCache`
- `java/lang/CharacterData00`
- `java/lang/CharacterData01`
- `java/lang/CharacterData02`
- `java/lang/CharacterData03`
- `java/lang/CharacterData0E`
- `java/lang/CharacterDataPrivateUse`
- `java/lang/CharacterDataUndefined`
- `java/lang/Integer$IntegerCache`
- `java/lang/StackWalker$ExtendedOption`
- `java/lang/Thread$Constants`
- `java/nio/charset/CoderResult$Cache`
- `java/nio/charset/CodingErrorAction`
- `java/text/Normalizer$Form`
- `java/util/regex/Pattern$LookBehindEndNode`
- `jdk/internal/access/JavaIOFilePermissionAccess`
- `jdk/internal/access/JavaLangAccess`
- `jdk/internal/access/JavaSecurityAccess`
- `jdk/internal/access/JavaSecurityAccess$ProtectionDomainCache`
- `jdk/internal/access/SharedSecrets`
- `jdk/internal/icu/lang/UCharacter`
- `jdk/internal/icu/text/NormalizerBase`
- `jdk/internal/loader/AbstractClassLoaderValue$Sub`
- `jdk/internal/loader/ClassLoaderValue`
- `jdk/internal/loader/ClassLoaders`
- `jdk/internal/math/FloatToDecimal`
- `jdk/internal/misc/InternalLock`
- `jdk/internal/misc/PreviewFeatures`
- `jdk/internal/misc/ScopedMemoryAccess`
- `jdk/internal/misc/Unsafe`
- `jdk/internal/misc/VM`
- `jdk/internal/misc/VirtualThreads`
- `jdk/internal/ref/CleanerFactory`
- `jdk/internal/ref/CleanerImpl$PhantomCleanableRef`
- `jdk/internal/reflect/ConstructorAccessor`
- `jdk/internal/reflect/MethodAccessor`
- `jdk/internal/reflect/Reflection`
- `jdk/internal/reflect/ReflectionFactory`
- `jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction`
- `jdk/internal/util/ArraysSupport`
- `jdk/internal/util/ByteArray`
- `jdk/internal/util/Preconditions`
- `jdk/internal/util/StaticProperty`
- `jdk/internal/util/random/RandomSupport`
- `jdk/internal/util/regex/Grapheme`
- `jdk/internal/vm/Continuation`
- `sun/security/util/Debug$FormatHolder`
- `sun/security/util/SecurityConstants`
