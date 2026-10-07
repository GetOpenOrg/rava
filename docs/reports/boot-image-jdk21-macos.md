# 构建期引导映像审计（JDK 21，macos）

- 结论：**通过**
- 摘要：`f1e75fdf4f1dfa83727395409ee0e89d`
- 求值耗时 145 ms，指令步数 1602115，堆对象 10729

## 阶段（累计值）

| 阶段 | 结局 | 步数 | 堆对象 | 已初始化类 | 耗时 ms |
|---|---|---:|---:|---:|---:|
| VM 预初始化（9 类 + 3 个 VM 构造对象） | 完成 | 2065 | 100 | 62 | 39 |
| `java/lang/System.initPhase1:()V` | 完成 | 77544 | 1002 | 173 | 81 |
| `java/lang/System.initPhase2:(ZZ)I` | 完成 → I(0) | 1600559 | 10690 | 246 | 145 |
| `java/lang/System.initPhase3:()V` | 完成 | 1602115 | 10729 | 247 | 145 |

## 映像规模

- 可达对象 8248，槽位 69451，静态字段 354，非数组类型 137；驻留字符串 1258，镜像 78，身份哈希 87
- 已初始化类 249（其中运行期初始化 2）
- VM 表：{"addExports0": 273, "addExportsToAll0": 226, "addReads0": 150, "defineModule0": 62, "packages": 771, "setBootLoaderUnnamedModule0": 1}；VM 单元：[("next_thread_id", 1)]

## 运行期初始化类（2）

- `jdk/internal/util/StaticProperty`：延迟值参与求值：运行期重放会改写的字段 java/util/concurrent/ConcurrentHashMap$Node.val
- `jdk/internal/loader/NativeLibraries`：延迟值参与求值：运行期初始化类的静态字段 jdk/internal/loader/ClassLoaderHelper.hasDynamicLoaderCache

## 运行期初始化尝试（含被外层撤回吸收的，4）

- `jdk/internal/util/StaticProperty`：延迟值参与求值：运行期重放会改写的字段 java/util/concurrent/ConcurrentHashMap$Node.val
- `jdk/internal/util/OSVersion`：延迟值参与求值：运行期初始化类的静态字段 jdk/internal/util/StaticProperty.OS_VERSION 的结果参与身份运算
- `jdk/internal/loader/ClassLoaderHelper`：延迟值参与求值：运行期初始化类的静态字段 jdk/internal/util/OSVersion.CURRENT_OSVERSION 的结果参与身份运算
- `jdk/internal/loader/NativeLibraries`：延迟值参与求值：运行期初始化类的静态字段 jdk/internal/loader/ClassLoaderHelper.hasDynamicLoaderCache

## 残差调用（2）

- `java/lang/System.initPhase1:()V@147` → `java/lang/System.newPrintStream:(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;`（结果为占位对象）：延迟值参与求值：stdout_encoding 的内容被读取
- `java/lang/System.initPhase1:()V@164` → `java/lang/System.newPrintStream:(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;`（结果为占位对象）：延迟值参与求值：stderr_encoding 的内容被读取

## 残差区段（2）

- `java/lang/System.initPhase1:()V` [36, 70)：延迟值参与求值：sun_jnu_encoding 的内容被读取
- `java/lang/System.initPhase3:()V` [367, 401)：延迟值参与求值：运行期重放会改写的静态字段 java/lang/System.notSupportedJnuEncoding

## 运行期初始化类的静态读取（占位对象，3）

- `jdk/internal/util/StaticProperty.JAVA_HOME`
- `jdk/internal/util/StaticProperty.USER_DIR`
- `jdk/internal/util/StaticProperty.JAVA_HOME`

## 启动重放 native（5）

- `java/lang/Terminator.setup:()V`
- `jdk/internal/misc/VM.initializeOSEnvironment:()V`
- `jdk/internal/loader/URLClassPath.toFileURL:(Ljava/lang/String;)Ljava/net/URL;`（结果为占位对象）
- `java/lang/Thread.setPriority0:(I)V`
- `java/lang/Thread.start0:()V`

## 污点审计（宿主源 3，污点表达式 5，区间判定分支 0，两侧合并分支 0，选择表达式 0）

- 映像中污点值（重算槽与重放输入之外）：0；不可独立重算：0；宿主标量构建期取零值：0
- 宿主源命中：
  - `java/io/FileDescriptor.getAppend:(I)Z` ← `java/io/FileDescriptor.<init>:(I)V`
  - `java/lang/Runtime.availableProcessors:()I` ← `java/util/concurrent/ConcurrentHashMap.<clinit>:()V`
  - `java/lang/Runtime.maxMemory:()J` ← `jdk/internal/misc/VM.saveProperties:(Ljava/util/Map;)V`

## 启动重算槽（5）

- `jdk/internal/misc/VM.directMemory` = java/lang/Runtime.maxMemory:()J(<java/lang/Runtime>) ∈ [1, 9223372036854775807]
- `java/util/concurrent/ConcurrentHashMap.NCPU` = java/lang/Runtime.availableProcessors:()I(<java/lang/Runtime>) ∈ [1, 2147483647]
- `java/io/FileDescriptor.in.append` = java/io/FileDescriptor.getAppend:(I)Z(0) ∈ [0, 1]
- `java/io/FileDescriptor.out.append` = java/io/FileDescriptor.getAppend:(I)Z(1) ∈ [0, 1]
- `java/io/FileDescriptor.err.append` = java/io/FileDescriptor.getAppend:(I)Z(2) ∈ [0, 1]

## 重放输入中的污点（0）


## 运行期初始化级联链（2）

- `jdk/internal/util/StaticProperty`（源：延迟值参与求值：运行期重放会改写的字段 java/util/concurrent/ConcurrentHashMap$Node.val）
- `jdk/internal/loader/NativeLibraries` ← `jdk/internal/loader/ClassLoaderHelper` ← `jdk/internal/util/OSVersion` ← `jdk/internal/util/StaticProperty`（源：延迟值参与求值：运行期重放会改写的字段 java/util/concurrent/ConcurrentHashMap$Node.val）

## 启动重放序列（19）

1. 重算 `jdk/internal/misc/VM.directMemory` = java/lang/Runtime.maxMemory:()J(<java/lang/Runtime>) ∈ [1, 9223372036854775807]
2. 重算 `java/util/concurrent/ConcurrentHashMap.NCPU` = java/lang/Runtime.availableProcessors:()I(<java/lang/Runtime>) ∈ [1, 2147483647]
3. 重算 `java/io/FileDescriptor.in.append` = java/io/FileDescriptor.getAppend:(I)Z(0) ∈ [0, 1]
4. 重算 `java/io/FileDescriptor.out.append` = java/io/FileDescriptor.getAppend:(I)Z(1) ∈ [0, 1]
5. 重算 `java/io/FileDescriptor.err.append` = java/io/FileDescriptor.getAppend:(I)Z(2) ∈ [0, 1]
6. 残差区段 `java/lang/System.initPhase1:()V` [36, 70)
7. 运行期初始化 `jdk/internal/util/StaticProperty`（首次主动使用时）
8. 回填 `jdk/internal/util/StaticProperty.JAVA_HOME`
9. 残差调用 `java/lang/System.initPhase1:()V@147` → `java/lang/System.newPrintStream:(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;`
10. 残差调用 `java/lang/System.initPhase1:()V@164` → `java/lang/System.newPrintStream:(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;`
11. 重放 native `java/lang/Terminator.setup:()V`
12. 重放 native `jdk/internal/misc/VM.initializeOSEnvironment:()V`
13. 运行期初始化 `jdk/internal/loader/NativeLibraries`（首次主动使用时）
14. 回填 `jdk/internal/util/StaticProperty.USER_DIR`
15. 重放 native `jdk/internal/loader/URLClassPath.toFileURL:(Ljava/lang/String;)Ljava/net/URL;`
16. 重放 native `java/lang/Thread.setPriority0:(I)V`
17. 重放 native `java/lang/Thread.start0:()V`
18. 回填 `jdk/internal/util/StaticProperty.JAVA_HOME`
19. 残差区段 `java/lang/System.initPhase3:()V` [367, 401)

## 残差重放读后写审计（U8，6 个残差，读集 115 个位置，交集 0）


## 映像可达对象的类型（162 种，对象数）

`[B` 1506、`[C` 4、`[I` 9、`[Ljava/io/ObjectStreamField;` 4、`[Ljava/lang/Character;` 1、`[Ljava/lang/Class;` 1、`[Ljava/lang/Integer;` 1、`[Ljava/lang/Object;` 214、`[Ljava/lang/StackTraceElement;` 1、`[Ljava/lang/String;` 1、`[Ljava/lang/Thread$State;` 1、`[Ljava/lang/invoke/MethodHandle;` 17、`[Ljava/lang/module/ModuleDescriptor$Modifier;` 1、`[Ljava/lang/module/ModuleDescriptor$Requires$Modifier;` 1、`[Ljava/lang/ref/WeakReference;` 1、`[Ljava/lang/reflect/AccessFlag$Location;` 1、`[Ljava/lang/reflect/AccessFlag;` 1、`[Ljava/security/Principal;` 3、`[Ljava/security/cert/Certificate;` 1、`[Ljava/util/HashMap$Node;` 262、`[Ljava/util/Hashtable$Entry;` 1、`[Ljava/util/WeakHashMap$Entry;` 1、`[Ljava/util/concurrent/ConcurrentHashMap$Node;` 11、`[Ljdk/internal/module/ServicesCatalog;` 1、`[[Ljava/lang/invoke/MethodHandle;` 2、`java/io/BufferedInputStream` 1、`java/io/FileDescriptor` 3、`java/io/FileDescriptor$1` 1、`java/io/FileInputStream` 1、`java/io/FileOutputStream` 2、`java/io/ObjectStreamField` 10、`java/io/PrintStream` 2、`java/io/UnixFileSystem` 1、`java/lang/Boolean` 2、`java/lang/Character` 128、`java/lang/CharacterDataLatin1` 1、`java/lang/Class` 78、`java/lang/Integer` 256、`java/lang/Module` 68、`java/lang/Module$ArchivedData` 1、`java/lang/ModuleLayer` 2、`java/lang/Object` 44、`java/lang/Runtime` 1、`java/lang/String` 1435、`java/lang/String$CaseInsensitiveComparator` 1、`java/lang/System$2` 1、`java/lang/Thread` 1、`java/lang/Thread$FieldHolder` 2、`java/lang/Thread$State` 6、`java/lang/ThreadGroup` 2、`java/lang/invoke/MemberName$Factory` 1、`java/lang/invoke/MethodHandles$Lookup` 2、`java/lang/module/Configuration` 2、`java/lang/module/ModuleDescriptor` 62、`java/lang/module/ModuleDescriptor$1` 1、`java/lang/module/ModuleDescriptor$Exports` 367、`java/lang/module/ModuleDescriptor$Modifier` 4、`java/lang/module/ModuleDescriptor$Opens` 4、`java/lang/module/ModuleDescriptor$Provides` 62、`java/lang/module/ModuleDescriptor$Requires` 137、`java/lang/module/ModuleDescriptor$Requires$Modifier` 4、`java/lang/module/ModuleDescriptor$Version` 1、`java/lang/module/ResolvedModule` 62、`java/lang/ref/NativeReferenceQueue` 1、`java/lang/ref/NativeReferenceQueue$Lock` 1、`java/lang/ref/Reference$1` 1、`java/lang/ref/Reference$ReferenceHandler` 1、`java/lang/ref/ReferenceQueue` 1、`java/lang/ref/ReferenceQueue$Null` 2、`java/lang/ref/WeakReference` 1、`java/lang/reflect/AccessFlag` 23、`java/lang/reflect/AccessFlag$1` 1、`java/lang/reflect/AccessFlag$10` 1、`java/lang/reflect/AccessFlag$11` 1、`java/lang/reflect/AccessFlag$12` 1、`java/lang/reflect/AccessFlag$13` 1、`java/lang/reflect/AccessFlag$14` 1、`java/lang/reflect/AccessFlag$15` 1、`java/lang/reflect/AccessFlag$16` 1、`java/lang/reflect/AccessFlag$17` 1、`java/lang/reflect/AccessFlag$18` 1、`java/lang/reflect/AccessFlag$2` 1、`java/lang/reflect/AccessFlag$3` 1、`java/lang/reflect/AccessFlag$4` 1、`java/lang/reflect/AccessFlag$5` 1、`java/lang/reflect/AccessFlag$6` 1、`java/lang/reflect/AccessFlag$7` 1、`java/lang/reflect/AccessFlag$8` 1、`java/lang/reflect/AccessFlag$9` 1、`java/lang/reflect/AccessFlag$Location` 9、`java/lang/reflect/ReflectAccess` 1、`java/net/URI` 62、`java/net/URI$1` 1、`java/net/URL` 1、`java/net/URL$3` 1、`java/net/URL$DefaultFactory` 1、`java/security/AccessControlContext` 2、`java/security/CodeSource` 3、`java/security/ProtectionDomain` 3、`java/security/ProtectionDomain$JavaSecurityAccessImpl` 1、`java/security/ProtectionDomain$Key` 3、`java/util/ArrayDeque` 1、`java/util/ArrayList` 8、`java/util/Collections$EmptyList` 1、`java/util/Collections$EmptyMap` 1、`java/util/Collections$EmptySet` 1、`java/util/Collections$SetFromMap` 1、`java/util/Collections$UnmodifiableMap` 1、`java/util/HashMap` 265、`java/util/HashMap$EntrySet` 1、`java/util/HashMap$KeySet` 1、`java/util/HashMap$Node` 1011、`java/util/HashSet` 203、`java/util/Hashtable` 1、`java/util/HexFormat` 2、`java/util/ImmutableCollections$List12` 52、`java/util/ImmutableCollections$ListN` 14、`java/util/ImmutableCollections$MapN` 4、`java/util/ImmutableCollections$Set12` 251、`java/util/ImmutableCollections$SetN` 158、`java/util/Optional` 1、`java/util/Properties` 1、`java/util/WeakHashMap` 1、`java/util/WeakHashMap$Entry` 5、`java/util/WeakHashMap$KeySet` 1、`java/util/concurrent/ConcurrentHashMap` 32、`java/util/concurrent/ConcurrentHashMap$Node` 911、`java/util/concurrent/CopyOnWriteArrayList` 30、`java/util/concurrent/atomic/AtomicInteger` 1、`java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionObject` 1、`java/util/concurrent/locks/ReentrantLock` 2、`java/util/concurrent/locks/ReentrantLock$NonfairSync` 2、`jdk/internal/loader/ArchivedClassLoaders` 1、`jdk/internal/loader/BuiltinClassLoader$LoadedModule` 61、`jdk/internal/loader/ClassLoaderValue` 2、`jdk/internal/loader/ClassLoaders$AppClassLoader` 1、`jdk/internal/loader/ClassLoaders$BootClassLoader` 1、`jdk/internal/loader/ClassLoaders$PlatformClassLoader` 1、`jdk/internal/loader/NativeLibraries` 4、`jdk/internal/loader/URLClassPath` 1、`jdk/internal/misc/InternalLock` 1、`jdk/internal/misc/Unsafe` 1、`jdk/internal/module/ArchivedBootLayer` 1、`jdk/internal/module/ArchivedModuleGraph` 1、`jdk/internal/module/ModuleHashes` 1、`jdk/internal/module/ModuleLoaderMap$Mapper` 1、`jdk/internal/module/ModulePatcher` 1、`jdk/internal/module/ModuleReferenceImpl` 62、`jdk/internal/module/ModuleTarget` 1、`jdk/internal/module/ServicesCatalog` 3、`jdk/internal/module/ServicesCatalog$ServiceProvider` 62、`jdk/internal/module/SystemModuleFinders$2` 62、`jdk/internal/module/SystemModuleFinders$3` 60、`jdk/internal/module/SystemModuleFinders$SystemModuleFinder` 1、`jdk/internal/reflect/ReflectionFactory` 1、`jdk/internal/reflect/ReflectionFactory$Config` 1、`jdk/internal/util/ClassFileDumper` 1、`jdk/internal/util/Preconditions$1` 1、`jdk/internal/util/Preconditions$2` 1、`jdk/internal/util/Preconditions$3` 1、`jdk/internal/util/Preconditions$4` 3、`sun/net/www/protocol/jar/Handler` 1

## 映像可达 lambda 对象（0）


## 已初始化类（构建期，按完成次序）

java/lang/Object java/lang/CharSequence java/util/Comparator java/lang/String$CaseInsensitiveComparator java/lang/String java/lang/System java/lang/reflect/Type java/lang/reflect/AnnotatedElement java/lang/Class java/lang/ThreadGroup java/lang/Thread jdk/internal/misc/CDS java/lang/Module$ArchivedData java/util/Set java/util/Collection java/lang/Iterable java/util/AbstractCollection java/util/ImmutableCollections$AbstractImmutableCollection java/util/ImmutableCollections$AbstractImmutableSet java/util/ImmutableCollections$Set12 java/util/Objects java/util/List java/util/SequencedCollection java/util/ImmutableCollections$AbstractImmutableList java/util/ImmutableCollections$ListN java/util/ImmutableCollections$SetN java/util/Map java/util/AbstractMap java/util/ImmutableCollections$AbstractImmutableMap java/util/ImmutableCollections$MapN java/util/ImmutableCollections java/lang/Module jdk/internal/misc/UnsafeConstants java/lang/reflect/ReflectAccess jdk/internal/access/SharedSecrets jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction java/security/AccessController java/lang/StringLatin1 java/lang/Number java/lang/Float java/lang/Double java/lang/Math jdk/internal/reflect/Reflection java/lang/Record jdk/internal/reflect/ReflectionFactory$Config jdk/internal/reflect/ReflectionFactory java/lang/ref/Reference$1 java/lang/ref/Reference java/lang/reflect/AccessibleObject java/lang/reflect/Member java/lang/reflect/Executable java/lang/reflect/Method java/lang/ref/FinalReference java/lang/ref/ReferenceQueue$Null java/lang/ref/ReferenceQueue java/lang/ref/NativeReferenceQueue java/lang/ref/NativeReferenceQueue$Lock java/lang/ref/Finalizer jdk/internal/misc/VM java/lang/ref/WeakReference java/lang/Thread$FieldHolder java/security/AccessControlContext java/lang/System$2 jdk/internal/util/SystemProps jdk/internal/util/SystemProps$Raw java/util/HashMap java/lang/Integer jdk/internal/misc/Unsafe jdk/internal/util/ArraysSupport java/util/HashMap$Node java/lang/StringConcatHelper java/lang/Byte java/lang/VersionProps java/lang/Runtime java/util/function/Function jdk/internal/util/Preconditions$1 java/util/function/BiFunction jdk/internal/util/Preconditions$4 jdk/internal/util/Preconditions$2 jdk/internal/util/Preconditions$3 jdk/internal/util/Preconditions java/util/Arrays java/lang/Character java/lang/CharacterData java/lang/CharacterDataLatin1 java/util/Dictionary java/util/Hashtable java/util/Properties java/util/concurrent/ConcurrentMap java/io/ObjectStreamField java/util/concurrent/ConcurrentHashMap java/util/AbstractSet java/util/HashMap$EntrySet java/util/HashMap$HashIterator java/util/Iterator java/util/HashMap$EntryIterator java/util/concurrent/ConcurrentHashMap$Node java/io/InputStream java/io/FileInputStream java/io/FileDescriptor$1 java/io/FileDescriptor java/io/OutputStream java/io/FileOutputStream java/io/FilterInputStream java/io/BufferedInputStream jdk/internal/misc/InternalLock java/util/concurrent/locks/ReentrantLock java/util/concurrent/locks/AbstractOwnableSynchronizer java/util/concurrent/locks/AbstractQueuedSynchronizer java/util/concurrent/locks/ReentrantLock$Sync java/util/concurrent/locks/ReentrantLock$NonfairSync java/lang/Terminator java/lang/Enum java/lang/Thread$State java/lang/ref/Reference$ReferenceHandler java/lang/Thread$ThreadIdentifiers java/lang/ClassLoader jdk/internal/loader/ArchivedClassLoaders java/util/WeakHashMap java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionObject java/util/Collections$EmptySet java/util/AbstractList java/util/Collections$EmptyList java/util/Collections$EmptyMap java/util/Collections java/util/Collections$SetFromMap java/util/WeakHashMap$KeySet java/lang/Boolean java/util/WeakHashMap$Entry java/lang/ClassLoader$ParallelLoaders java/security/SecureClassLoader jdk/internal/loader/BuiltinClassLoader jdk/internal/loader/ClassLoaders$BootClassLoader java/util/ArrayList sun/security/action/GetPropertyAction java/security/ProtectionDomain$JavaSecurityAccessImpl java/security/ProtectionDomain java/security/CodeSource java/security/ProtectionDomain$Key jdk/internal/loader/ClassLoaders$PlatformClassLoader java/lang/AbstractStringBuilder java/lang/StringBuilder java/lang/invoke/MemberName java/lang/invoke/MemberName$Factory java/lang/invoke/MethodHandles java/lang/StrictMath java/util/ImmutableCollections$MapN$1 java/util/ImmutableCollections$MapN$MapNIterator java/util/KeyValueHolder java/util/HexFormat java/lang/Character$CharacterCache jdk/internal/util/ClassFileDumper java/util/concurrent/atomic/AtomicInteger java/lang/invoke/MethodHandles$Lookup sun/invoke/util/VerifyAccess java/lang/reflect/Modifier java/net/URL$DefaultFactory java/net/URL$3 java/net/URL jdk/internal/loader/URLClassPath java/io/DefaultFileSystem java/io/FileSystem java/io/UnixFileSystem java/io/File java/util/Deque java/util/ArrayDeque java/net/URLStreamHandler sun/net/www/protocol/jar/Handler jdk/internal/loader/ClassLoaders$AppClassLoader jdk/internal/loader/AbstractClassLoaderValue jdk/internal/loader/ClassLoaderValue jdk/internal/module/ServicesCatalog jdk/internal/loader/ClassLoaders java/lang/module/ModuleDescriptor$1 java/lang/module/ModuleDescriptor jdk/internal/module/ModulePatcher java/util/HashSet jdk/internal/module/ModuleBootstrap jdk/internal/module/ModuleBootstrap$Counters jdk/internal/module/ArchivedBootLayer jdk/internal/module/ArchivedModuleGraph java/net/URI$1 java/net/URI jdk/internal/module/SystemModuleFinders jdk/internal/module/SystemModulesMap jdk/internal/module/SystemModules$default jdk/internal/module/Builder java/lang/module/ModuleDescriptor$Exports java/util/ImmutableCollections$SetN$SetNIterator java/util/ImmutableCollections$List12 java/lang/module/ModuleDescriptor$Provides java/lang/module/ModuleDescriptor$Version java/lang/Integer$IntegerCache java/lang/reflect/AccessFlag$Location java/lang/reflect/AccessFlag$1 java/lang/reflect/AccessFlag$2 java/lang/reflect/AccessFlag$3 java/lang/reflect/AccessFlag$4 java/lang/reflect/AccessFlag$5 java/lang/reflect/AccessFlag$6 java/lang/reflect/AccessFlag$7 java/lang/reflect/AccessFlag$8 java/lang/reflect/AccessFlag$9 java/lang/reflect/AccessFlag$10 java/lang/reflect/AccessFlag$11 java/lang/reflect/AccessFlag$12 java/lang/reflect/AccessFlag$13 java/lang/reflect/AccessFlag$14 java/lang/reflect/AccessFlag$15 java/lang/reflect/AccessFlag$16 java/lang/reflect/AccessFlag$17 java/lang/reflect/AccessFlag$18 java/lang/reflect/AccessFlag java/lang/module/ModuleDescriptor$Modifier java/lang/module/ModuleDescriptor$Requires$Modifier java/lang/module/ModuleDescriptor$Requires java/util/ImmutableCollections$Set12$1 java/lang/module/ModuleDescriptor$Opens jdk/internal/module/ModuleTarget jdk/internal/module/ModuleHashes$Builder jdk/internal/module/ModuleHashes java/util/Collections$UnmodifiableMap jdk/internal/module/SystemModuleFinders$2 java/lang/module/ModuleReference jdk/internal/module/ModuleReferenceImpl jdk/internal/module/SystemModuleFinders$3 jdk/internal/module/SystemModuleFinders$SystemModuleFinder java/util/Optional java/lang/Module$EnableNativeAccess jdk/internal/loader/BootLoader jdk/internal/loader/BuiltinClassLoader$LoadedModule jdk/internal/module/Modules java/lang/module/Configuration java/util/AbstractMap$1 java/util/AbstractMap$1$1 java/lang/module/ResolvedModule jdk/internal/module/ModuleLoaderMap jdk/internal/module/ModuleLoaderMap$Mapper jdk/internal/module/ModuleLoaderMap$Modules java/lang/ModuleLayer java/util/ImmutableCollections$ListItr jdk/internal/module/ServicesCatalog$ServiceProvider java/util/concurrent/CopyOnWriteArrayList java/util/HashMap$KeySet java/util/HashMap$KeyIterator java/lang/ModuleLayer$Controller java/lang/invoke/StringConcatFactory

## 闭包

- 入口 `HelloWorld`：闭包 469 类；引导映像求值 154 ms
- 映像类型 137 个，不在闭包 52 个：`java/lang/Module$ArchivedData` `java/lang/Thread$State` `java/lang/module/Configuration` `java/lang/module/ModuleDescriptor$1` `java/lang/module/ModuleDescriptor$Exports` `java/lang/module/ModuleDescriptor$Modifier` `java/lang/module/ModuleDescriptor$Opens` `java/lang/module/ModuleDescriptor$Provides` `java/lang/module/ModuleDescriptor$Requires` `java/lang/module/ModuleDescriptor$Requires$Modifier` `java/lang/module/ModuleDescriptor$Version` `java/lang/module/ResolvedModule` `java/lang/ref/NativeReferenceQueue` `java/lang/ref/NativeReferenceQueue$Lock` `java/lang/ref/Reference$ReferenceHandler` `java/lang/reflect/AccessFlag` `java/lang/reflect/AccessFlag$1` `java/lang/reflect/AccessFlag$10` `java/lang/reflect/AccessFlag$11` `java/lang/reflect/AccessFlag$12` `java/lang/reflect/AccessFlag$13` `java/lang/reflect/AccessFlag$14` `java/lang/reflect/AccessFlag$15` `java/lang/reflect/AccessFlag$16` `java/lang/reflect/AccessFlag$17` `java/lang/reflect/AccessFlag$18` `java/lang/reflect/AccessFlag$2` `java/lang/reflect/AccessFlag$3` `java/lang/reflect/AccessFlag$4` `java/lang/reflect/AccessFlag$5` `java/lang/reflect/AccessFlag$6` `java/lang/reflect/AccessFlag$7` `java/lang/reflect/AccessFlag$8` `java/lang/reflect/AccessFlag$9` `java/lang/reflect/AccessFlag$Location` `java/net/URI` `java/net/URI$1` `java/util/Collections$UnmodifiableMap` `java/util/HashMap$KeySet` `java/util/concurrent/CopyOnWriteArrayList` `jdk/internal/loader/BuiltinClassLoader$LoadedModule` `jdk/internal/module/ArchivedBootLayer` `jdk/internal/module/ArchivedModuleGraph` `jdk/internal/module/ModuleHashes` `jdk/internal/module/ModuleLoaderMap$Mapper` `jdk/internal/module/ModulePatcher` `jdk/internal/module/ModuleReferenceImpl` `jdk/internal/module/ModuleTarget` `jdk/internal/module/ServicesCatalog$ServiceProvider` `jdk/internal/module/SystemModuleFinders$2` `jdk/internal/module/SystemModuleFinders$3` `jdk/internal/module/SystemModuleFinders$SystemModuleFinder`
- 运行期部分入口类 7 个，不在闭包 1 个：`java/lang/Terminator`
