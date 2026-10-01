# 编译前缺口扫描（API 模式）

- 入口包：java/lang, java/util（不含子包），267 个 public 类 / 3828 个 public·protected 方法
- BFS 耗时 4.5 分钟
- native-missing 35 个，boundary-stub 0 个

## native-missing（35）

| 类 | 成员 |
|----|------|
| `com/sun/media/sound/DirectAudioDeviceProvider` | `nGetNumDevices:()I` |
| `com/sun/media/sound/DirectAudioDeviceProvider` | `nNewDirectAudioDeviceInfo:(I)Lcom/sun/media/sound/DirectAudioDeviceProvider$DirectAudioDeviceInfo;` |
| `com/sun/media/sound/PortMixerProvider` | `nGetNumDevices:()I` |
| `com/sun/media/sound/PortMixerProvider` | `nNewPortMixerInfo:(I)Lcom/sun/media/sound/PortMixerProvider$PortMixerInfo;` |
| `java/lang/Class` | `getClassAccessFlagsRaw0:()I` |
| `java/lang/Class` | `getDeclaredClasses0:()[Ljava/lang/Class;` |
| `java/lang/Class` | `getNestMembers0:()[Ljava/lang/Class;` |
| `java/lang/Class` | `setSigners:([Ljava/lang/Object;)V` |
| `java/lang/ClassLoader` | `defineClass1:(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;` |
| `java/lang/ClassLoader` | `defineClass2:(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;` |
| `java/lang/ClassLoader` | `retrieveDirectives:()Ljava/lang/AssertionStatusDirectives;` |
| `java/lang/Module` | `addExports0:(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V` |
| `java/lang/Module` | `addExportsToAll0:(Ljava/lang/Module;Ljava/lang/String;)V` |
| `java/lang/Module` | `addExportsToAllUnnamed0:(Ljava/lang/Module;Ljava/lang/String;)V` |
| `java/lang/Module` | `addReads0:(Ljava/lang/Module;Ljava/lang/Module;)V` |
| `java/lang/Module` | `defineModule0:(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V` |
| `java/lang/SecurityManager` | `getClassContext:()[Ljava/lang/Class;` |
| `java/lang/StackStreamFactory` | `checkStackWalkModes:()Z` |
| `java/lang/StackStreamFactory$AbstractStackWalker` | `callStackWalk:(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;` |
| `java/lang/StackStreamFactory$AbstractStackWalker` | `fetchStackFrames:(JJII[Ljava/lang/Object;)I` |
| `java/lang/StackTraceElement` | `initStackTraceElement:(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V` |
| `java/lang/invoke/MethodHandleNatives` | `expand:(Ljava/lang/invoke/MemberName;)V` |
| `jdk/internal/misc/Unsafe` | `getBooleanVolatile:(Ljava/lang/Object;J)Z` |
| `jdk/internal/misc/Unsafe` | `getByteVolatile:(Ljava/lang/Object;J)B` |
| `jdk/internal/misc/Unsafe` | `getCharVolatile:(Ljava/lang/Object;J)C` |
| `jdk/internal/misc/Unsafe` | `getDoubleVolatile:(Ljava/lang/Object;J)D` |
| `jdk/internal/misc/Unsafe` | `getFloatVolatile:(Ljava/lang/Object;J)F` |
| `jdk/internal/misc/Unsafe` | `getShortVolatile:(Ljava/lang/Object;J)S` |
| `jdk/internal/misc/Unsafe` | `putBooleanVolatile:(Ljava/lang/Object;JZ)V` |
| `jdk/internal/misc/Unsafe` | `putByteVolatile:(Ljava/lang/Object;JB)V` |
| `jdk/internal/misc/Unsafe` | `putCharVolatile:(Ljava/lang/Object;JC)V` |
| `jdk/internal/misc/Unsafe` | `putDoubleVolatile:(Ljava/lang/Object;JD)V` |
| `jdk/internal/misc/Unsafe` | `putFloatVolatile:(Ljava/lang/Object;JF)V` |
| `jdk/internal/misc/Unsafe` | `putShortVolatile:(Ljava/lang/Object;JS)V` |
| `jdk/internal/misc/Unsafe` | `throwException:(Ljava/lang/Throwable;)V` |

## boundary-stub（0）

无

