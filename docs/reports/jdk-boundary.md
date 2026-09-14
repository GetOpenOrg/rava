# JDK 边界方法扫描报告

生成时间：2026-09-14

扫描范围：`java/ + javax/ 全部`

## 摘要

| 指标 | 数量 | 占比 |
|------|-----:|-----:|
| 扫描类数 | 7,784 | 100% |
| 扫描方法数 | 74,451 | 100% |
| native 方法 | 388 | 0.5% |
| 边界方法（调内部类） | 5,314 | 7.1% |
| **总需手写** | **5,702** | **7.7%** |
| 纯可翻译方法 | 68,749 | 92.3% |
| 被引用的内部类数 | 654 | — |

## 被引用的内部包分布

| 包 | 引用次数 | 内部类数 |
|-----|-----:|-----:|
| `sun (other)` | 2,788 | 208 |
| `jdk/internal (other)` | 2,511 | 160 |
| `jdk/internal/misc` | 2,256 | 15 |
| `sun/util` | 1,620 | 36 |
| `com/sun (other)` | 1,436 | 96 |
| `sun/security` | 980 | 72 |
| `jdk/internal/reflect` | 208 | 6 |
| `sun/reflect` | 162 | 18 |
| `sun/nio` | 140 | 21 |
| `jdk/internal/loader` | 99 | 12 |
| `jdk/internal/vm` | 42 | 9 |
| `jdk/xml/internal` | 28 | 1 |

## 内部类方法可实现性分析

共 654 个内部类，10,358 个方法（不含 `<clinit>`）。

| 类型 | 数量 | 占比 | 说明 |
|------|-----:|-----:|------|
| ACC_NATIVE | 139 | 1.3% | 无字节码，**必须手写 Rust** |
| 纯字节码（无内部依赖） | 6,057 | 58.5% | 可直接从字节码翻译 |
| 有内部依赖（seed 范围内） | 1,721 | 16.6% | 翻译时需先实现其依赖 |
| 外部内部依赖（更深层） | 1,815 | 17.5% | 调用未分析的内部类，需额外跟踪 |
| 抽象/接口方法 | 626 | 6.0% | 由具体子类实现，本身无需手写 |
| **可从字节码翻译合计** | **7,778** | **75.1%** | |
| **必须手写合计** | **1,954** | **18.9%** | |

### 全为 ACC_NATIVE 的内部类

| 内部类 | 被引用次数 | native 方法数 |
|--------|----------:|-------------:|

## 内部包实现阶段规划

> 拓扑排序结果。阶段 1 为叶子层（无内部依赖），可最先实现；
> "手写"= ACC_NATIVE + 外部内部依赖；"翻译"= 纯字节码 + seed 内依赖。

| 阶段 | 类数 | 手写方法 | 可翻译方法 | 新解锁 | 累计进度 |
|------|-----:|-------:|----------:|------:|--------:|
| 阶段 1 | 349 | 728 | 2,243 | 1,482 | 27.9% |
| 阶段 2 | 68 | 340 | 1,263 | 1,557 | 57.2% |
| 阶段 3 | 42 | 99 | 512 | 399 | 64.7% |
| 阶段 4 | 24 | 138 | 379 | 218 | 68.8% |
| 阶段 5 | 10 | 47 | 97 | 74 | 70.2% |
| 阶段 6 | 3 | 4 | 50 | 12 | 70.4% |
| 阶段 7 (循环依赖) | 158 | 598 | 3,234 | 1,572 | 100.0% |

## 各阶段详情

### 阶段 1 

**349 个内部类，新解锁 1482 个边界方法**

| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |
|--------|-------:|-----:|-----:|-----:|---------:|
| `com/sun/jmx/defaults/JmxProperties` | 704 | 0 | 1 | 0 | 0 |
| `jdk/internal/util/Preconditions` | 537 | 1 | 15 | 0 | 0 |
| `com/sun/jmx/remote/util/ClassLogger` | 237 | 0 | 43 | 0 | 0 |
| `sun/security/util/Debug` | 204 | 1 | 20 | 0 | 0 |
| `jdk/internal/misc/InternalLock` | 184 | 0 | 7 | 0 | 0 |
| `sun/util/logging/PlatformLogger$Level` | 166 | 0 | 7 | 0 | 0 |
| `sun/invoke/util/Wrapper` | 140 | 0 | 55 | 0 | 0 |
| `sun/security/jca/GetInstance$Instance` | 127 | 0 | 2 | 0 | 0 |
| `jdk/internal/util/ByteArray` | 99 | 0 | 22 | 0 | 0 |
| `sun/util/locale/provider/LocaleProviderAdapter` | 71 | 7 | 4 | 17 | 0 |
| `jdk/internal/access/JavaLangAccess` | 63 | 0 | 0 | 86 | 0 |
| `jdk/internal/access/JavaNioAccess` | 58 | 0 | 0 | 19 | 0 |
| `jdk/internal/access/SharedSecrets` | 58 | 0 | 72 | 0 | 0 |
| `sun/awt/util/IdentityArrayList` | 52 | 0 | 26 | 0 | 0 |
| `sun/awt/EventQueueItem` | 51 | 0 | 1 | 0 | 0 |
| `jdk/internal/misc/VM` | 48 | 10 | 24 | 0 | 0 |
| `sun/security/util/SecurityConstants` | 46 | 0 | 1 | 0 | 0 |
| `sun/font/TextLineComponent` | 46 | 0 | 0 | 22 | 0 |
| `sun/swing/MenuItemLayoutHelper$RectSize` | 44 | 0 | 11 | 0 | 0 |
| `sun/awt/AWTAccessor` | 41 | 0 | 66 | 0 | 0 |
| `sun/java2d/StateTrackable$State` | 40 | 0 | 4 | 0 | 0 |
| `sun/nio/cs/UTF_8` | 36 | 4 | 2 | 0 | 0 |
| `jdk/internal/reflect/FieldAccessor` | 36 | 0 | 0 | 18 | 0 |
| `com/sun/jmx/mbeanserver/Util` | 32 | 0 | 17 | 0 | 0 |
| `sun/awt/AWTAccessor$MouseEventAccessor` | 28 | 0 | 0 | 2 | 0 |
| `sun/nio/ch/DirectBuffer` | 28 | 0 | 0 | 3 | 0 |
| `sun/swing/SwingUtilities2$Section` | 28 | 0 | 4 | 0 | 0 |
| `jdk/xml/internal/SecuritySupport` | 28 | 0 | 41 | 0 | 0 |
| `jdk/internal/util/StaticProperty` | 28 | 0 | 20 | 0 | 0 |
| `sun/swing/UIAction` | 27 | 0 | 9 | 0 | 0 |
| `sun/util/calendar/CalendarUtils` | 26 | 0 | 12 | 0 | 0 |
| `sun/awt/ComponentFactory` | 26 | 1 | 27 | 0 | 0 |
| `sun/font/Font2D` | 23 | 15 | 22 | 2 | 0 |
| `sun/java2d/pipe/Region` | 23 | 7 | 57 | 0 | 0 |
| `sun/datatransfer/DataFlavorUtil` | 23 | 6 | 9 | 0 | 0 |
| `jdk/internal/icu/text/BidiBase` | 22 | 32 | 50 | 0 | 0 |
| `sun/util/locale/LocaleSyntaxException` | 19 | 0 | 3 | 0 | 0 |
| `sun/net/util/IPAddressUtil` | 19 | 4 | 31 | 0 | 0 |
| `sun/swing/MenuItemLayoutHelper$LayoutResult` | 18 | 0 | 15 | 0 | 0 |
| `com/sun/beans/introspect/PropertyInfo$Name` | 18 | 0 | 4 | 0 | 0 |
| `jdk/internal/util/ByteArrayLittleEndian` | 18 | 0 | 22 | 0 | 0 |
| `sun/font/BidiUtils` | 17 | 0 | 9 | 0 | 0 |
| `com/sun/jndi/ldap/BerEncoder` | 17 | 3 | 19 | 0 | 0 |
| `sun/awt/AWTAccessor$ComponentAccessor` | 16 | 0 | 0 | 35 | 0 |
| `jdk/internal/access/JavaSecurityAccess` | 16 | 0 | 0 | 4 | 0 |
| `jdk/internal/perf/PerfCounter` | 16 | 7 | 9 | 0 | 0 |
| `sun/java2d/cmm/PCMM` | 15 | 0 | 0 | 5 | 0 |
| `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` | 15 | 0 | 5 | 0 | 0 |
| `sun/awt/PeerEvent` | 15 | 0 | 4 | 0 | 0 |
| `sun/awt/AWTPermissions` | 15 | 0 | 1 | 0 | 0 |
| `jdk/internal/access/JavaIOFileDescriptorAccess` | 15 | 0 | 0 | 10 | 0 |
| `jdk/internal/event/VirtualThreadEndEvent` | 15 | 1 | 1 | 0 | 0 |
| `sun/swing/PrintingStatus` | 13 | 4 | 6 | 0 | 0 |
| `jdk/internal/event/DeserializationEvent` | 13 | 1 | 0 | 0 | 0 |
| `jdk/internal/access/JavaObjectInputStreamAccess` | 13 | 0 | 0 | 1 | 0 |
| `sun/management/spi/PlatformMBeanProvider$PlatformComponent` | 12 | 0 | 3 | 4 | 0 |
| `jdk/internal/util/OperatingSystem` | 12 | 0 | 10 | 0 | 0 |
| `jdk/internal/ref/CleanerFactory` | 12 | 0 | 2 | 0 | 0 |
| `com/sun/jndi/ldap/BerDecoder` | 12 | 8 | 8 | 0 | 0 |
| `sun/security/util/HexDumpEncoder` | 12 | 0 | 17 | 0 | 0 |
| `sun/java2d/cmm/ColorTransform` | 12 | 0 | 0 | 7 | 0 |
| `sun/text/UCompactIntArray` | 11 | 0 | 8 | 0 | 0 |
| `sun/swing/SwingAccessor` | 11 | 0 | 20 | 0 | 0 |
| `sun/font/TextLabelFactory` | 11 | 2 | 4 | 0 | 0 |
| `jdk/internal/org/objectweb/asm/Type` | 11 | 0 | 36 | 0 | 0 |
| `sun/awt/dnd/SunDropTargetEvent` | 10 | 3 | 3 | 0 | 0 |
| `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` | 10 | 0 | 0 | 8 | 0 |
| `jdk/internal/module/Checks` | 10 | 0 | 11 | 0 | 0 |
| `jdk/internal/misc/Signal` | 10 | 5 | 7 | 0 | 0 |
| `jdk/internal/loader/ClassLoaderValue` | 10 | 1 | 3 | 0 | 0 |
| `jdk/internal/io/JdkConsole` | 10 | 0 | 0 | 10 | 0 |
| `sun/print/ServiceDialog` | 10 | 8 | 17 | 0 | 0 |
| `sun/security/x509/GeneralNameInterface` | 10 | 0 | 0 | 3 | 0 |
| `sun/datatransfer/DesktopDatatransferService` | 10 | 0 | 0 | 7 | 0 |
| `sun/swing/AccumulativeRunnable` | 9 | 0 | 5 | 1 | 0 |
| `sun/font/LayoutPathImpl` | 9 | 1 | 5 | 4 | 0 |
| `sun/font/FontManagerFactory` | 9 | 1 | 1 | 0 | 0 |
| `sun/font/FontManager` | 9 | 0 | 0 | 7 | 0 |
| `com/sun/jmx/remote/internal/ClientListenerInfo` | 9 | 0 | 10 | 0 | 0 |
| `com/sun/imageio/stream/StreamFinalizer` | 8 | 0 | 2 | 0 | 0 |
| `jdk/internal/loader/AbstractClassLoaderValue$Sub` | 8 | 2 | 4 | 0 | 0 |
| `sun/swing/ImageIconUIResource` | 8 | 0 | 2 | 0 | 0 |
| `com/sun/jmx/mbeanserver/GetPropertyAction` | 8 | 0 | 3 | 0 | 0 |
| `com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl` | 8 | 4 | 11 | 0 | 0 |
| `jdk/internal/loader/Resource` | 8 | 1 | 7 | 5 | 0 |
| `jdk/internal/access/JavaUtilZipFileAccess` | 8 | 0 | 0 | 10 | 0 |
| `jdk/internal/util/NullableKeyValueHolder` | 8 | 0 | 9 | 0 | 0 |
| `sun/net/www/MessageHeader` | 8 | 1 | 30 | 0 | 0 |
| `sun/awt/UngrabEvent` | 8 | 0 | 2 | 0 | 0 |
| `jdk/internal/event/SecurityProviderServiceEvent` | 7 | 1 | 1 | 0 | 0 |
| `sun/font/CodePointIterator` | 7 | 4 | 1 | 5 | 0 |
| `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget` | 7 | 0 | 4 | 0 | 0 |
| `sun/swing/BakedArrayList` | 7 | 0 | 5 | 0 | 0 |
| `jdk/internal/misc/ThreadTracker` | 7 | 4 | 1 | 0 | 0 |
| `jdk/internal/event/ProcessStartEvent` | 7 | 1 | 0 | 0 | 0 |
| `sun/util/locale/LocaleUtils` | 7 | 0 | 20 | 0 | 0 |
| `jdk/internal/util/xml/PropertiesDefaultHandler` | 6 | 8 | 4 | 0 | 0 |
| `com/sun/beans/finder/BeanInfoFinder` | 6 | 5 | 2 | 0 | 0 |
| `sun/net/InetAddressCachePolicy` | 6 | 1 | 7 | 0 | 0 |
| `jdk/internal/org/objectweb/asm/FieldVisitor` | 6 | 0 | 6 | 0 | 0 |
| `sun/awt/geom/AreaOp$XorOp` | 6 | 1 | 1 | 0 | 0 |
| `sun/management/MemoryUsageCompositeData` | 6 | 1 | 9 | 0 | 0 |
| `sun/management/MonitorInfoCompositeData` | 6 | 3 | 7 | 0 | 0 |
| `com/sun/naming/internal/FactoryEnumeration` | 6 | 1 | 2 | 0 | 0 |
| `jdk/internal/access/JavaLangInvokeAccess` | 6 | 0 | 0 | 21 | 0 |
| `com/sun/beans/finder/PropertyEditorFinder` | 6 | 6 | 2 | 0 | 0 |
| `com/sun/imageio/stream/CloseableDisposerRecord` | 6 | 0 | 2 | 0 | 0 |
| `jdk/internal/event/VirtualThreadSubmitFailedEvent` | 6 | 1 | 0 | 0 | 0 |
| `jdk/internal/event/ThreadSleepEvent` | 6 | 1 | 1 | 0 | 0 |
| `sun/font/LayoutPathImpl$SegmentPathBuilder` | 6 | 2 | 6 | 0 | 0 |
| `com/sun/imageio/stream/StreamCloser` | 6 | 3 | 2 | 0 | 0 |
| `com/sun/media/sound/JDK13Services` | 6 | 2 | 5 | 0 | 0 |
| `sun/util/locale/LocaleMatcher` | 6 | 2 | 19 | 0 | 0 |
| `sun/swing/plaf/synth/StyleAssociation` | 6 | 0 | 7 | 0 | 0 |
| `sun/awt/OSInfo$OSType` | 6 | 0 | 4 | 0 | 0 |
| `jdk/internal/module/ModuleReferenceImpl` | 6 | 1 | 10 | 0 | 0 |
| `sun/font/Font2DHandle` | 5 | 0 | 1 | 0 | 0 |
| `sun/swing/DefaultLayoutStyle` | 5 | 0 | 16 | 0 | 0 |
| `sun/nio/cs/ArrayDecoder` | 5 | 0 | 3 | 1 | 0 |
| `sun/awt/datatransfer/TransferableProxy` | 5 | 1 | 3 | 0 | 0 |
| `com/sun/beans/finder/PersistenceDelegateFinder` | 5 | 4 | 2 | 0 | 0 |
| `jdk/internal/event/VirtualThreadStartEvent` | 5 | 1 | 1 | 0 | 0 |
| `jdk/internal/event/SecurityPropertyModificationEvent` | 5 | 1 | 0 | 0 | 0 |
| `sun/util/locale/ParseStatus` | 5 | 0 | 6 | 0 | 0 |
| `jdk/internal/util/FormatConcatItem` | 5 | 0 | 0 | 2 | 0 |
| `sun/swing/text/UndoableEditLockSupport` | 5 | 0 | 0 | 2 | 0 |
| `jdk/internal/access/JavaTemplateAccess` | 5 | 0 | 0 | 3 | 0 |
| `com/sun/org/apache/xerces/internal/jaxp/validation/XMLSchemaFactory` | 4 | 13 | 2 | 0 | 0 |
| `jdk/internal/reflect/MethodAccessor` | 4 | 0 | 0 | 2 | 0 |
| `jdk/internal/module/ModuleHashes` | 4 | 0 | 13 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfIntImpl` | 4 | 15 | 3 | 0 | 0 |
| `com/sun/media/sound/MidiUtils` | 4 | 2 | 11 | 0 | 0 |
| `sun/awt/event/IgnorePaintEvent` | 4 | 0 | 1 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfLongImpl` | 4 | 15 | 3 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfDoubleImpl` | 4 | 15 | 3 | 0 | 0 |
| `sun/reflect/generics/repository/FieldRepository` | 4 | 3 | 3 | 0 | 0 |
| `jdk/internal/event/VirtualThreadPinnedEvent` | 4 | 1 | 0 | 0 | 0 |
| `sun/swing/plaf/synth/DefaultSynthStyle$StateInfo` | 4 | 0 | 15 | 0 | 0 |
| `sun/java2d/cmm/ProfileDeferralInfo` | 4 | 0 | 1 | 0 | 0 |
| `jdk/internal/net/http/ResponseSubscribers$SubscriberAdapter` | 4 | 1 | 6 | 0 | 0 |
| `sun/swing/SwingAccessor$UIDefaultsAccessor` | 4 | 0 | 0 | 1 | 0 |
| `jdk/internal/access/JavaLangRefAccess` | 4 | 0 | 0 | 4 | 0 |
| `com/sun/org/apache/xpath/internal/jaxp/XPathFactoryImpl` | 4 | 9 | 0 | 0 | 0 |
| `sun/nio/cs/US_ASCII` | 4 | 3 | 2 | 0 | 0 |
| `sun/swing/icon/SortArrowIcon` | 4 | 0 | 6 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfBooleanImpl` | 4 | 15 | 3 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfCharImpl` | 4 | 15 | 3 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfFloatImpl` | 4 | 15 | 3 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfShortImpl` | 4 | 15 | 3 | 0 | 0 |
| `sun/text/ComposedCharIter` | 4 | 0 | 3 | 0 | 0 |
| `jdk/internal/access/JavaUtilResourceBundleAccess` | 4 | 0 | 0 | 6 | 0 |
| `sun/nio/ch/Interruptible` | 4 | 0 | 0 | 1 | 0 |
| `sun/text/IntHashtable` | 4 | 0 | 17 | 0 | 0 |
| `sun/awt/PaintEventDispatcher` | 4 | 0 | 6 | 0 | 0 |
| `sun/util/ResourceBundleEnumeration` | 4 | 0 | 4 | 0 | 0 |
| `sun/awt/PlatformGraphicsInfo` | 4 | 3 | 4 | 0 | 0 |
| `jdk/internal/module/ModuleInfo$Attributes` | 4 | 0 | 5 | 0 | 0 |
| `jdk/internal/net/http/ResponseSubscribers$NullSubscriber` | 4 | 1 | 6 | 0 | 0 |
| `jdk/internal/net/http/RequestPublishers$PublisherAdapter` | 4 | 0 | 3 | 0 | 0 |
| `com/sun/org/apache/xerces/internal/jaxp/DocumentBuilderFactoryImpl` | 4 | 6 | 5 | 0 | 0 |
| `jdk/internal/util/regex/Grapheme` | 4 | 0 | 4 | 0 | 0 |
| `jdk/internal/foreign/layout/ValueLayouts$OfByteImpl` | 4 | 15 | 3 | 0 | 0 |
| `jdk/internal/math/FloatingDecimal$BinaryToASCIIConverter` | 4 | 0 | 0 | 8 | 0 |
| `sun/util/PropertyResourceBundleCharset` | 3 | 1 | 4 | 0 | 0 |
| `jdk/internal/access/foreign/UnmapperProxy` | 3 | 0 | 0 | 4 | 0 |
| `sun/net/util/URLUtil` | 3 | 0 | 4 | 0 | 0 |
| `sun/security/provider/certpath/CertPathHelper` | 3 | 0 | 3 | 2 | 0 |
| `sun/awt/geom/AreaOp$SubOp` | 3 | 1 | 1 | 0 | 0 |
| `com/sun/media/sound/ReferenceCountingDevice` | 3 | 0 | 0 | 2 | 0 |
| `sun/net/www/protocol/jar/Handler` | 3 | 1 | 11 | 0 | 0 |
| `sun/nio/fs/AbstractFileSystemProvider` | 3 | 2 | 7 | 3 | 0 |
| `jdk/internal/ref/Cleaner` | 3 | 1 | 4 | 0 | 0 |
| `sun/awt/AWTAccessor$ContainerAccessor` | 3 | 0 | 0 | 4 | 0 |
| `sun/swing/plaf/synth/Paint9Painter$PaintType` | 3 | 0 | 4 | 0 | 0 |
| `sun/nio/cs/ArrayEncoder` | 3 | 0 | 3 | 1 | 0 |
| `jdk/internal/util/SystemProps` | 3 | 1 | 5 | 0 | 0 |
| `jdk/internal/classfile/ClassBuilder` | 3 | 13 | 5 | 5 | 0 |
| `jdk/internal/access/JavaUtilJarAccess` | 3 | 0 | 0 | 5 | 0 |
| `com/sun/jmx/mbeanserver/MXBeanMappingFactory` | 3 | 0 | 1 | 1 | 0 |
| `com/sun/naming/internal/VersionHelper` | 3 | 1 | 21 | 0 | 0 |
| `sun/font/EAttribute` | 3 | 0 | 6 | 0 | 0 |
| `jdk/internal/logger/DefaultLoggerFinder` | 3 | 2 | 5 | 0 | 0 |
| `jdk/internal/module/ServicesCatalog$ServiceProvider` | 3 | 0 | 5 | 0 | 0 |
| `sun/awt/AWTAccessor$AWTEventAccessor` | 3 | 0 | 0 | 6 | 0 |
| `sun/awt/geom/AreaOp$AddOp` | 3 | 1 | 1 | 0 | 0 |
| `sun/awt/AWTAccessor$InputEventAccessor` | 3 | 0 | 0 | 3 | 0 |
| `sun/security/util/IOUtils` | 3 | 0 | 2 | 0 | 0 |
| `sun/java2d/pipe/hw/ExtendedBufferCapabilities$VSyncType` | 3 | 0 | 5 | 0 | 0 |
| `sun/awt/geom/AreaOp$IntOp` | 3 | 1 | 1 | 0 | 0 |
| `jdk/internal/module/ModuleLoaderMap` | 3 | 4 | 1 | 0 | 0 |
| `sun/nio/fs/DefaultFileSystemProvider` | 3 | 1 | 2 | 0 | 0 |
| `sun/awt/FwDispatcher` | 3 | 0 | 0 | 3 | 0 |
| `com/sun/imageio/spi/FileImageOutputStreamSpi` | 2 | 0 | 3 | 0 | 0 |
| `sun/text/spi/JavaTimeDateTimePatternProvider` | 2 | 0 | 2 | 1 | 0 |
| `com/sun/org/apache/xerces/internal/jaxp/datatype/DatatypeFactoryImpl` | 2 | 15 | 1 | 0 | 0 |
| `com/sun/jmx/remote/security/MBeanServerFileAccessController` | 2 | 9 | 5 | 0 | 0 |
| `sun/nio/ch/DefaultSelectorProvider` | 2 | 0 | 2 | 0 | 0 |
| `com/sun/imageio/plugins/tiff/TIFFImageReaderSpi` | 2 | 2 | 3 | 0 | 0 |
| `sun/security/jca/ServiceId` | 2 | 0 | 1 | 0 | 0 |
| `jdk/internal/net/http/ResponseBodyHandlers$PushPromisesHandlerWithMap` | 2 | 0 | 2 | 0 | 0 |
| `sun/reflect/generics/repository/GenericDeclRepository` | 2 | 2 | 1 | 0 | 0 |
| `jdk/internal/foreign/FunctionDescriptorImpl` | 2 | 0 | 19 | 0 | 0 |
| `jdk/internal/foreign/layout/MemoryLayoutUtil` | 2 | 0 | 3 | 0 | 0 |
| `sun/awt/SubRegionShowable` | 2 | 0 | 0 | 2 | 0 |
| `sun/security/util/ManifestDigester` | 2 | 6 | 3 | 0 | 0 |
| `com/sun/media/sound/MidiDeviceTransmitterEnvelope` | 2 | 0 | 6 | 0 | 0 |
| `com/sun/rowset/internal/SyncResolverImpl` | 2 | 11 | 199 | 0 | 0 |
| `sun/net/ApplicationProxy` | 2 | 0 | 2 | 0 | 0 |
| `com/sun/xml/internal/stream/XMLInputFactoryImpl` | 2 | 26 | 2 | 0 | 0 |
| `com/sun/rowset/providers/RIOptimisticProvider` | 2 | 4 | 7 | 0 | 0 |
| `jdk/internal/net/http/RequestPublishers$InputStreamPublisher` | 2 | 2 | 3 | 0 | 0 |
| `sun/awt/ConstrainableGraphics` | 2 | 0 | 0 | 1 | 0 |
| `com/sun/imageio/plugins/common/StandardMetadataFormat` | 2 | 0 | 3 | 0 | 0 |
| `sun/swing/SwingAccessor$JTextComponentAccessor` | 2 | 0 | 0 | 2 | 0 |
| `sun/swing/MenuItemCheckIconFactory` | 2 | 0 | 0 | 2 | 0 |
| `com/sun/rowset/RowSetFactoryImpl` | 2 | 5 | 1 | 0 | 0 |
| `sun/rmi/runtime/Log` | 2 | 2 | 4 | 5 | 0 |
| `com/sun/jmx/remote/internal/rmi/RMIExporter` | 2 | 0 | 0 | 2 | 0 |
| `com/sun/imageio/spi/InputStreamImageInputStreamSpi` | 2 | 0 | 5 | 0 | 0 |
| `sun/awt/image/VSyncedBSManager` | 2 | 1 | 3 | 2 | 0 |
| `sun/security/provider/SecureRandom` | 2 | 2 | 6 | 0 | 0 |
| `jdk/internal/access/JavaUtilCollectionAccess` | 2 | 0 | 0 | 2 | 0 |
| `com/sun/beans/util/Cache` | 2 | 7 | 4 | 1 | 0 |
| `sun/net/ResolverProviderConfiguration` | 2 | 0 | 3 | 0 | 0 |
| `jdk/internal/module/ModuleTarget` | 2 | 0 | 2 | 0 | 0 |
| `jdk/internal/foreign/abi/LinkerOptions$FirstVariadicArg` | 2 | 0 | 6 | 0 | 0 |
| `jdk/internal/foreign/abi/LinkerOptions$CaptureCallState` | 2 | 0 | 6 | 0 | 0 |
| `com/sun/imageio/plugins/wbmp/WBMPImageWriterSpi` | 2 | 1 | 4 | 0 | 0 |
| `com/sun/imageio/plugins/wbmp/WBMPImageReaderSpi` | 2 | 1 | 5 | 0 | 0 |
| `sun/reflect/generics/reflectiveObjects/ParameterizedTypeImpl` | 2 | 0 | 10 | 0 | 0 |
| `sun/net/www/protocol/jrt/Handler` | 2 | 1 | 1 | 0 | 0 |
| `com/sun/imageio/plugins/jpeg/JPEGImageReaderSpi` | 2 | 2 | 2 | 0 | 0 |
| `com/sun/imageio/plugins/gif/GIFImageReaderSpi` | 2 | 2 | 2 | 0 | 0 |
| `jdk/internal/util/ReferencedKeySet` | 2 | 12 | 2 | 0 | 0 |
| `com/sun/imageio/spi/FileImageInputStreamSpi` | 2 | 0 | 3 | 0 | 0 |
| `jdk/internal/icu/text/UCharacterIterator` | 2 | 5 | 6 | 6 | 0 |
| `com/sun/imageio/plugins/tiff/TIFFImageWriterSpi` | 2 | 1 | 4 | 0 | 0 |
| `jdk/internal/io/JdkConsoleProvider` | 2 | 0 | 0 | 1 | 0 |
| `sun/nio/cs/ThreadLocalCoders` | 2 | 2 | 1 | 0 | 0 |
| `sun/reflect/annotation/TypeNotPresentExceptionProxy` | 2 | 1 | 4 | 0 | 0 |
| `com/sun/imageio/plugins/gif/GIFImageWriterSpi` | 2 | 2 | 2 | 0 | 0 |
| `com/sun/xml/internal/stream/events/XMLEventFactoryImpl` | 2 | 21 | 6 | 0 | 0 |
| `sun/util/locale/LocaleObjectCache` | 2 | 3 | 3 | 1 | 0 |
| `sun/awt/AWTAccessor$WindowAccessor` | 2 | 0 | 0 | 8 | 0 |
| `sun/net/PortConfig` | 2 | 2 | 3 | 0 | 0 |
| `sun/rmi/transport/tcp/TCPDirectSocketFactory` | 2 | 0 | 3 | 0 | 0 |
| `com/sun/imageio/spi/RAFImageOutputStreamSpi` | 2 | 0 | 3 | 0 | 0 |
| `com/sun/imageio/plugins/png/PNGImageReaderSpi` | 2 | 2 | 2 | 0 | 0 |
| `sun/nio/ch/Streams` | 2 | 2 | 1 | 0 | 0 |
| `jdk/internal/net/http/ResponseSubscribers$MappingSubscriber` | 2 | 1 | 8 | 0 | 0 |
| `com/sun/jmx/remote/security/JMXSubjectDomainCombiner` | 2 | 0 | 4 | 0 | 0 |
| `com/sun/imageio/spi/OutputStreamImageOutputStreamSpi` | 2 | 0 | 5 | 0 | 0 |
| `jdk/internal/icu/impl/Punycode` | 2 | 2 | 8 | 0 | 0 |
| `jdk/internal/logger/SurrogateLogger` | 2 | 3 | 3 | 0 | 0 |
| `jdk/internal/net/http/RequestPublishers$IterablePublisher` | 2 | 2 | 3 | 0 | 0 |
| `sun/security/action/GetIntegerAction` | 2 | 0 | 6 | 0 | 0 |
| `com/sun/imageio/spi/RAFImageInputStreamSpi` | 2 | 0 | 3 | 0 | 0 |
| `com/sun/imageio/plugins/jpeg/JPEGImageWriterSpi` | 2 | 2 | 3 | 0 | 0 |
| `com/sun/media/sound/MidiDeviceReceiverEnvelope` | 2 | 0 | 5 | 0 | 0 |
| `jdk/internal/vm/ContinuationSupport` | 2 | 1 | 3 | 0 | 0 |
| `sun/security/action/GetBooleanAction` | 2 | 0 | 4 | 0 | 0 |
| `sun/invoke/WrapperInstance` | 2 | 0 | 0 | 2 | 0 |
| `jdk/internal/net/http/RequestPublishers$EmptyPublisher` | 2 | 1 | 2 | 0 | 0 |
| `jdk/internal/net/http/ResponseSubscribers$ConsumerSubscriber` | 2 | 1 | 6 | 0 | 0 |
| `com/sun/imageio/plugins/bmp/BMPImageWriterSpi` | 2 | 1 | 4 | 0 | 0 |
| `jdk/internal/math/FloatToDecimal` | 2 | 2 | 17 | 0 | 0 |
| `jdk/internal/net/http/BufferingSubscriber` | 2 | 5 | 7 | 0 | 0 |
| `com/sun/xml/internal/stream/XMLOutputFactoryImpl` | 2 | 9 | 5 | 0 | 0 |
| `jdk/internal/access/JavaIOPrintStreamAccess` | 2 | 0 | 0 | 1 | 0 |
| `com/sun/imageio/plugins/png/PNGImageWriterSpi` | 2 | 1 | 3 | 0 | 0 |
| `sun/management/LockInfoCompositeData` | 2 | 1 | 5 | 0 | 0 |
| `sun/java2d/DisposerRecord` | 2 | 0 | 0 | 1 | 0 |
| `com/sun/jmx/remote/util/ClassLoaderWithRepository` | 2 | 0 | 2 | 0 | 0 |
| `com/sun/imageio/plugins/bmp/BMPImageReaderSpi` | 2 | 2 | 3 | 0 | 0 |
| `jdk/internal/net/http/ResponseSubscribers$HttpResponseInputStream` | 2 | 6 | 7 | 0 | 0 |
| `sun/java2d/HeadlessGraphicsEnvironment` | 2 | 0 | 9 | 0 | 0 |
| `sun/security/util/RegisteredDomain` | 2 | 1 | 0 | 3 | 0 |
| `com/sun/imageio/plugins/tiff/TIFFIFD` | 2 | 1 | 20 | 0 | 0 |
| `jdk/internal/logger/LocalizedLoggerWrapper` | 2 | 1 | 16 | 0 | 0 |
| `jdk/internal/net/http/RequestPublishers$FilePublisher` | 1 | 2 | 9 | 0 | 0 |
| `com/sun/jmx/remote/internal/NotificationBuffer` | 1 | 0 | 0 | 2 | 0 |
| `jdk/internal/vm/ContinuationScope` | 1 | 0 | 6 | 0 | 0 |
| `sun/util/spi/CalendarProvider` | 1 | 0 | 1 | 1 | 0 |
| `sun/awt/ExtendedKeyCodes` | 1 | 0 | 2 | 0 | 0 |
| `sun/awt/RequestFocusController` | 1 | 0 | 0 | 1 | 0 |
| `jdk/internal/foreign/layout/StructLayoutImpl` | 1 | 10 | 3 | 0 | 0 |
| `sun/swing/plaf/GTKKeybindings` | 1 | 0 | 2 | 0 | 0 |
| `sun/datatransfer/DataFlavorUtil$RMI` | 1 | 0 | 8 | 0 | 0 |
| `jdk/internal/icu/impl/NormalizerImpl` | 1 | 21 | 46 | 0 | 0 |
| `sun/util/locale/provider/ResourceBundleBasedAdapter` | 1 | 0 | 0 | 2 | 0 |
| `sun/swing/plaf/WindowsKeybindings` | 1 | 0 | 2 | 0 | 0 |
| `sun/rmi/server/WeakClassHashMap` | 1 | 1 | 1 | 1 | 0 |
| `sun/nio/fs/DefaultFileTypeDetector` | 1 | 1 | 1 | 0 | 0 |
| `jdk/internal/misc/PreviewFeatures` | 1 | 1 | 3 | 0 | 0 |
| `sun/nio/fs/BasicFileAttributesHolder` | 1 | 0 | 0 | 2 | 0 |
| `jdk/internal/foreign/layout/PaddingLayoutImpl` | 1 | 9 | 6 | 0 | 0 |
| `sun/nio/ch/SelectorProviderImpl` | 1 | 8 | 1 | 1 | 0 |
| `sun/awt/image/SurfaceManager$ImageAccessor` | 1 | 0 | 1 | 2 | 0 |
| `jdk/internal/classfile/Classfile$Option` | 1 | 10 | 0 | 0 | 0 |
| `com/sun/media/sound/JavaSoundAudioClip` | 1 | 8 | 10 | 0 | 0 |
| `sun/print/DialogOwnerAccessor` | 1 | 0 | 3 | 1 | 0 |
| `sun/swing/SwingUtilities2$RepaintListener` | 1 | 0 | 0 | 1 | 0 |
| `jdk/internal/reflect/ConstructorAccessor` | 1 | 0 | 0 | 1 | 0 |
| `jdk/internal/foreign/layout/UnionLayoutImpl` | 1 | 10 | 3 | 0 | 0 |
| `jdk/internal/access/JavaIORandomAccessFileAccess` | 1 | 0 | 0 | 1 | 0 |
| `sun/nio/ch/SelectorImpl` | 1 | 5 | 14 | 4 | 0 |
| `sun/security/util/SecurityProperties` | 1 | 0 | 5 | 0 | 0 |
| `sun/invoke/util/BytecodeName` | 1 | 0 | 21 | 0 | 0 |
| `sun/net/SocksProxy` | 1 | 0 | 3 | 0 | 0 |
| `com/sun/beans/finder/PrimitiveWrapperMap` | 1 | 0 | 3 | 0 | 0 |
| `sun/management/spi/PlatformMBeanProvider` | 1 | 0 | 1 | 1 | 0 |
| `sun/awt/TimedWindowEvent` | 1 | 0 | 3 | 0 | 0 |
| `jdk/internal/vm/ScopedValueContainer$BindingsSnapshot` | 1 | 0 | 6 | 0 | 0 |
| `sun/print/PlatformPrinterJobProxy` | 1 | 1 | 1 | 0 | 0 |
| `sun/net/spi/DefaultProxySelector` | 1 | 4 | 6 | 0 | 0 |
| `sun/awt/AWTAccessor$KeyEventAccessor` | 1 | 0 | 0 | 5 | 0 |
| `sun/font/FontAccess` | 1 | 0 | 3 | 5 | 0 |
| `jdk/internal/foreign/abi/LinkerOptions$IsTrivial` | 1 | 0 | 5 | 0 | 0 |
| `sun/swing/text/TextComponentPrintable` | 1 | 11 | 6 | 0 | 0 |
| `jdk/internal/foreign/MemorySessionImpl$ResourceList$ResourceCleanup` | 1 | 1 | 1 | 1 | 0 |
| `sun/management/Util` | 1 | 0 | 7 | 0 | 0 |
| `jdk/internal/net/http/LineSubscriberAdapter` | 1 | 5 | 3 | 0 | 0 |
| `sun/security/krb5/KrbException` | 1 | 6 | 11 | 0 | 0 |
| `sun/nio/ch/DefaultAsynchronousChannelProvider` | 1 | 1 | 1 | 0 | 0 |
| `sun/swing/plaf/synth/SynthFileChooserUI` | 1 | 5 | 30 | 3 | 0 |
| `jdk/internal/access/JavaLangReflectAccess` | 1 | 0 | 0 | 19 | 0 |
| `jdk/internal/loader/NativeLibrary` | 1 | 1 | 2 | 2 | 0 |
| `com/sun/media/sound/AutoConnectSequencer` | 1 | 0 | 0 | 1 | 0 |
| `sun/awt/AWTAccessor$InvocationEventAccessor` | 1 | 0 | 0 | 1 | 0 |
| `sun/security/util/MessageDigestSpi2` | 1 | 0 | 0 | 1 | 0 |
| `sun/awt/AWTAccessor$DragSourceContextAccessor` | 1 | 0 | 0 | 1 | 0 |
| `sun/awt/KeyboardFocusManagerPeerProvider` | 1 | 0 | 0 | 1 | 0 |
| `jdk/internal/access/JavaUtilConcurrentTLRAccess` | 1 | 0 | 0 | 1 | 0 |
| `sun/awt/image/MultiResolutionToolkitImage` | 1 | 3 | 6 | 0 | 0 |
| `com/sun/jmx/mbeanserver/DescriptorCache` | 1 | 0 | 5 | 0 | 0 |
| `jdk/internal/classfile/ClassModel` | 1 | 2 | 0 | 11 | 0 |
| `sun/awt/InputMethodSupport` | 1 | 0 | 0 | 4 | 0 |
| `com/sun/jmx/mbeanserver/MXBeanMapping` | 1 | 0 | 6 | 2 | 0 |
| `sun/awt/AWTAccessor$EventQueueAccessor` | 1 | 0 | 0 | 8 | 0 |
| `sun/swing/SwingAccessor$JComponentAccessor` | 1 | 0 | 0 | 2 | 0 |
| `sun/util/resources/LocaleData` | 1 | 13 | 1 | 0 | 0 |
| `sun/java2d/cmm/ProfileDataVerifier` | 1 | 0 | 5 | 0 | 0 |
| `jdk/internal/foreign/abi/NativeEntryPoint` | 1 | 4 | 6 | 0 | 0 |
| `sun/swing/SwingAccessor$KeyStrokeAccessor` | 1 | 0 | 0 | 1 | 0 |
| `jdk/internal/access/JavaAWTAccess` | 1 | 0 | 0 | 1 | 0 |
| `jdk/internal/access/JavaIOAccess` | 1 | 0 | 0 | 2 | 0 |
| `com/sun/imageio/plugins/bmp/BMPCompressionTypes` | 1 | 0 | 4 | 0 | 0 |
| `jdk/internal/access/JavaIOPrintWriterAccess` | 1 | 0 | 0 | 1 | 0 |
| `jdk/internal/foreign/abi/CapturableState` | 1 | 0 | 13 | 0 | 0 |

**新解锁的边界方法（按公开 API 类）：**

`java/applet/Applet`：
- `newAudioClip(Ljava/net/URL;)Ljava/applet/AudioClip;`

`java/awt/AWTEvent`：
- `copyPrivateDataInto(Ljava/awt/AWTEvent;)V`
- `dispatched()V`

`java/awt/Button`：
- `addNotify()V`

`java/awt/Canvas`：
- `addNotify()V`

`java/awt/Checkbox`：
- `addNotify()V`

`java/awt/CheckboxMenuItem`：
- `addNotify()V`

`java/awt/Choice`：
- `addNotify()V`

`java/awt/Component`：
- `addNotify()V`
- `doSwingSerialization()V`
- `findUnderMouseInWindow(Ljava/awt/PointerInfo;)Ljava/awt/Component;`
- `getGraphics()Ljava/awt/Graphics;`
- `getGraphics_NoClientCode()Ljava/awt/Graphics;`
- `getNormalShape()Lsun/java2d/pipe/Region;`
- `isNonOpaqueForMixing()Z`
- `setMixingCutoutShape(Ljava/awt/Shape;)V`

`java/awt/Component$FlipBufferStrategy`：
- `destroyBuffers()V`

`java/awt/Container`：
- `getOpaqueShape()Lsun/java2d/pipe/Region;`

`java/awt/DefaultKeyboardFocusManager`：
- `initStatic()V`

`java/awt/Dialog`：
- `addNotify()V`
- `blockWindow(Ljava/awt/Window;)V`
- `blockWindows(Ljava/util/List;)V`
- `checkModalityPermission(Ljava/awt/Dialog$ModalityType;)V`
- `checkShouldBeBlocked(Ljava/awt/Window;)V`
- `conditionalShow(Ljava/awt/Component;Ljava/util/concurrent/atomic/AtomicLong;)Z`
- `hideAndDisposePreHandler()V`
- `modalHide()V`
- `modalShow()V`
- `toBack()V`
- *（+1 个）*

`java/awt/EventQueue`：
- `cacheEQItem(Lsun/awt/EventQueueItem;)V`
- `coalesceMouseEvent(Ljava/awt/event/MouseEvent;)Z`
- `coalesceOtherEvent(Ljava/awt/AWTEvent;I)Z`
- `coalescePaintEvent(Ljava/awt/event/PaintEvent;)Z`
- `coalescePeerEvent(Lsun/awt/PeerEvent;)Z`
- `createSecondaryLoop(Ljava/awt/Conditional;Ljava/awt/EventFilter;J)Ljava/awt/SecondaryLoop;`
- `dispatchEvent(Ljava/awt/AWTEvent;)V`
- `getNextEventPrivate()Ljava/awt/AWTEvent;`
- `getPriority(Ljava/awt/AWTEvent;)I`
- `isDispatchThreadImpl()Z`
- *（+3 个）*

`java/awt/EventQueue$4`：
- `run()Ljava/lang/Void;`

`java/awt/EventQueue$5`：
- `run()Ljava/lang/Void;`

`java/awt/FileDialog`：
- `addNotify()V`

`java/awt/Font`：
- `<init>(Ljava/io/File;IZLsun/font/CreatedFontTracker;)V`
- `<init>(Ljava/lang/String;IFZLsun/font/Font2DHandle;)V`
- `<init>(Lsun/font/Font2D;)V`
- `canDisplay(C)Z`
- `canDisplay(I)Z`
- `canDisplayUpTo(Ljava/lang/String;)I`
- `canDisplayUpTo(Ljava/text/CharacterIterator;II)I`
- `canDisplayUpTo([CII)I`
- `createFonts(Ljava/io/File;)[Ljava/awt/Font;`
- `getBaselineFor(C)B`
- *（+9 个）*

`java/awt/Font$FontAccessImpl`：
- `<init>()V`

`java/awt/Frame`：
- `addNotify()V`

`java/awt/GraphicsEnvironment`：
- `getHeadlessMessage()Ljava/lang/String;`
- `lambda$getHeadlessProperty$0()Ljava/lang/Void;`
- `preferLocaleFonts()V`
- `preferProportionalFonts()V`
- `registerFont(Ljava/awt/Font;)Z`

`java/awt/GraphicsEnvironment$LocalGE`：
- `createGE()Ljava/awt/GraphicsEnvironment;`

`java/awt/Image$1`：
- `<init>()V`

`java/awt/KeyboardFocusManager`：
- `initPeer()V`
- `isProxyActiveImpl(Ljava/awt/event/KeyEvent;)Z`

`java/awt/Label`：
- `addNotify()V`

`java/awt/LightweightDispatcher`：
- `processDropTargetEvent(Lsun/awt/dnd/SunDropTargetEvent;)Z`

`java/awt/List`：
- `addNotify()V`

`java/awt/MediaTracker`：
- `getResolutionVariant(Ljava/awt/Image;)Ljava/awt/Image;`

`java/awt/Menu`：
- `addNotify()V`

`java/awt/MenuBar`：
- `addNotify()V`

`java/awt/MenuItem`：
- `addNotify()V`

`java/awt/MouseInfo`：
- `getPointerInfo()Ljava/awt/PointerInfo;`

`java/awt/Panel`：
- `addNotify()V`

`java/awt/PopupMenu`：
- `addNotify()V`

`java/awt/Robot`：
- `checkRobotAllowed()V`
- `checkScreenCaptureAllowed()V`
- `init(Ljava/awt/GraphicsDevice;)V`

`java/awt/ScrollPane`：
- `addNotify()V`

`java/awt/Scrollbar`：
- `addNotify()V`

`java/awt/SystemTray`：
- `checkSystemTrayAllowed()V`

`java/awt/TextArea`：
- `addNotify()V`

`java/awt/TextComponent`：
- `canAccessClipboard()Z`
- `enableInputMethodsIfNecessary()V`

`java/awt/TextField`：
- `addNotify()V`

`java/awt/Toolkit`：
- `addAWTEventListener(Ljava/awt/event/AWTEventListener;J)V`
- `getAWTEventListeners()[Ljava/awt/event/AWTEventListener;`
- `getAWTEventListeners(J)[Ljava/awt/event/AWTEventListener;`
- `getSystemEventQueue()Ljava/awt/EventQueue;`
- `initStatic()V`
- `removeAWTEventListener(Ljava/awt/event/AWTEventListener;)V`

`java/awt/Window`：
- `addNotify()V`
- `getAllUnblockedWindows()Lsun/awt/util/IdentityArrayList;`
- `getAllWindows()Lsun/awt/util/IdentityArrayList;`
- `removeNotify()V`
- `setAlwaysOnTop(Z)V`
- `setModalExclusionType(Ljava/awt/Dialog$ModalExclusionType;)V`
- `setShape(Ljava/awt/Shape;)V`

`java/awt/color/ICC_Profile`：
- `getColorSpaceType()I`
- `getNumComponents()I`
- `getProfileClass()I`

`java/awt/datatransfer/Clipboard`：
- `lambda$fireFlavorsChanged$2(Ljava/awt/datatransfer/FlavorListener;)V`
- `setContents(Ljava/awt/datatransfer/Transferable;Ljava/awt/datatransfer/ClipboardOwner;)V`

`java/awt/datatransfer/DataFlavor`：
- `equals(Ljava/awt/datatransfer/DataFlavor;)Z`
- `getTextPlainUnicodeFlavor()Ljava/awt/datatransfer/DataFlavor;`
- `hashCode()I`
- `isFlavorTextType()Z`
- `isRepresentationClassRemote()Z`
- `paramString()Ljava/lang/String;`
- `selectBestTextFlavor([Ljava/awt/datatransfer/DataFlavor;)Ljava/awt/datatransfer/DataFlavor;`

*（还有更多，省略）*

### 阶段 2 

**68 个内部类，新解锁 1557 个边界方法**

| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |
|--------|-------:|-----:|-----:|-----:|---------:|
| `jdk/internal/misc/Unsafe` | 1379 | 84 | 325 | 0 | 2 |
| `jdk/internal/org/objectweb/asm/MethodVisitor` | 341 | 0 | 34 | 0 | 1 |
| `jdk/internal/reflect/Reflection` | 138 | 3 | 19 | 0 | 4 |
| `sun/util/locale/BaseLocale` | 51 | 0 | 11 | 0 | 1 |
| `jdk/internal/icu/text/NormalizerBase` | 50 | 4 | 25 | 0 | 1 |
| `sun/security/util/ResourcesMgr` | 48 | 0 | 4 | 0 | 1 |
| `sun/java2d/StateTrackableDelegate` | 42 | 0 | 9 | 0 | 1 |
| `sun/security/action/GetPropertyAction` | 26 | 1 | 8 | 0 | 1 |
| `com/sun/jmx/remote/util/EnvHelp` | 23 | 1 | 20 | 0 | 2 |
| `sun/util/locale/provider/LocaleServiceProviderPool` | 20 | 4 | 9 | 0 | 1 |
| `sun/management/ThreadInfoCompositeData` | 20 | 4 | 21 | 0 | 2 |
| `sun/management/ManagementFactoryHelper` | 16 | 21 | 11 | 0 | 2 |
| `sun/security/util/ManifestEntryVerifier` | 16 | 1 | 7 | 0 | 1 |
| `sun/security/util/KnownOIDs` | 16 | 0 | 11 | 0 | 1 |
| `sun/nio/cs/StreamEncoder` | 14 | 1 | 30 | 0 | 1 |
| `sun/swing/plaf/synth/DefaultSynthStyle` | 13 | 0 | 31 | 0 | 1 |
| `com/sun/jmx/remote/internal/ServerCommunicatorAdmin` | 11 | 0 | 5 | 1 | 1 |
| `sun/nio/cs/StreamDecoder` | 11 | 1 | 26 | 0 | 1 |
| `sun/nio/cs/ISO_8859_1` | 10 | 3 | 2 | 0 | 1 |
| `jdk/internal/module/ServicesCatalog` | 9 | 0 | 9 | 0 | 2 |
| `com/sun/beans/TypeResolver` | 8 | 0 | 10 | 0 | 1 |
| `sun/invoke/util/BytecodeDescriptor` | 8 | 0 | 10 | 0 | 1 |
| `jdk/internal/misc/CDS` | 8 | 9 | 15 | 0 | 3 |
| `sun/invoke/util/ValueConversions` | 8 | 2 | 98 | 0 | 1 |
| `sun/security/util/FilePermCompat` | 8 | 0 | 3 | 0 | 1 |
| `sun/java2d/Disposer` | 7 | 4 | 9 | 0 | 1 |
| `sun/invoke/util/VerifyType` | 7 | 0 | 4 | 0 | 1 |
| `jdk/internal/misc/VirtualThreads` | 7 | 0 | 5 | 0 | 1 |
| `sun/security/jca/ProviderList` | 7 | 12 | 10 | 0 | 2 |
| `com/sun/imageio/plugins/tiff/TIFFImageMetadata` | 6 | 1 | 29 | 0 | 1 |
| `sun/reflect/generics/repository/ConstructorRepository` | 6 | 3 | 5 | 0 | 1 |
| `sun/reflect/generics/repository/ClassRepository` | 6 | 3 | 5 | 0 | 1 |
| `com/sun/naming/internal/ResourceManager` | 5 | 0 | 9 | 0 | 2 |
| `jdk/internal/module/Resources` | 5 | 0 | 7 | 0 | 1 |
| `sun/java2d/pipe/hw/ExtendedBufferCapabilities` | 5 | 0 | 7 | 0 | 1 |
| `com/sun/jmx/remote/security/SubjectDelegator` | 5 | 1 | 4 | 0 | 1 |
| `sun/reflect/generics/factory/CoreReflectionFactory` | 5 | 4 | 16 | 0 | 1 |
| `sun/management/MemoryNotifInfoCompositeData` | 5 | 1 | 7 | 0 | 1 |
| `sun/awt/OSInfo` | 5 | 1 | 3 | 0 | 2 |
| `jdk/internal/vm/StackableScope` | 4 | 0 | 16 | 0 | 1 |
| `jdk/internal/net/http/common/Utils` | 4 | 17 | 66 | 0 | 1 |
| `sun/net/www/ParseUtil` | 3 | 0 | 21 | 0 | 1 |
| `sun/util/resources/Bundles` | 3 | 5 | 6 | 0 | 1 |
| `jdk/internal/math/FloatingDecimal` | 3 | 7 | 8 | 0 | 1 |
| `com/sun/jmx/mbeanserver/MXBeanProxy` | 3 | 1 | 1 | 0 | 1 |
| `sun/rmi/server/Util` | 3 | 1 | 14 | 0 | 1 |
| `sun/nio/ch/NioSocketImpl` | 2 | 30 | 25 | 0 | 3 |
| `jdk/internal/loader/RawNativeLibraries` | 2 | 4 | 3 | 0 | 2 |
| `sun/rmi/transport/ObjectTable` | 2 | 8 | 4 | 0 | 1 |
| `com/sun/imageio/plugins/tiff/TIFFFieldNode` | 2 | 0 | 15 | 0 | 1 |
| `sun/font/LayoutPathImpl$EmptyPath` | 2 | 0 | 7 | 0 | 1 |
| `jdk/internal/event/EventHelper` | 2 | 0 | 9 | 0 | 2 |
| `com/sun/jmx/remote/security/JMXPluggableAuthenticator` | 2 | 2 | 3 | 0 | 1 |
| `com/sun/jmx/mbeanserver/JmxMBeanServer` | 2 | 9 | 44 | 0 | 1 |
| `com/sun/org/apache/xalan/internal/xsltc/trax/TransformerFactoryImpl` | 2 | 19 | 15 | 0 | 1 |
| `sun/security/krb5/Asn1Exception` | 2 | 0 | 1 | 0 | 1 |
| `jdk/internal/foreign/abi/SharedUtils` | 2 | 8 | 26 | 0 | 3 |
| `jdk/internal/module/ModuleResolution` | 1 | 0 | 14 | 0 | 1 |
| `jdk/internal/net/http/RequestPublishers` | 1 | 0 | 2 | 0 | 1 |
| `jdk/internal/classfile/Classfile` | 1 | 3 | 12 | 0 | 1 |
| `sun/rmi/server/MarshalInputStream` | 1 | 0 | 13 | 0 | 1 |
| `sun/net/www/URLConnection` | 1 | 0 | 19 | 0 | 1 |
| `sun/net/www/MimeTable` | 1 | 12 | 11 | 0 | 1 |
| `sun/awt/image/MultiResolutionCachedImage` | 1 | 3 | 17 | 0 | 1 |
| `jdk/internal/misc/CarrierThreadLocal` | 1 | 0 | 5 | 0 | 1 |
| `sun/security/provider/SunEntries` | 1 | 2 | 5 | 0 | 1 |
| `sun/util/logging/internal/LoggingProviderImpl` | 1 | 3 | 3 | 0 | 1 |
| `jdk/internal/logger/BootstrapLogger` | 1 | 37 | 5 | 0 | 2 |

**新解锁的边界方法（按公开 API 类）：**

`java/awt/Component$FlipBufferStrategy`：
- `createBuffers(ILjava/awt/BufferCapabilities;)V`

`java/awt/Component$ProxyCapabilities`：
- `<init>(Ljava/awt/BufferCapabilities;)V`

`java/awt/Cursor`：
- `setPData(J)V`

`java/awt/Window`：
- `initDeserializedWindow()V`
- `setWarningString()V`

`java/awt/image/DataBuffer`：
- `<init>(Lsun/java2d/StateTrackable$State;II)V`
- `<init>(Lsun/java2d/StateTrackable$State;III)V`
- `<init>(Lsun/java2d/StateTrackable$State;IIII)V`
- `<init>(Lsun/java2d/StateTrackable$State;III[I)V`

`java/awt/image/DataBufferByte`：
- `getBankData()[[B`
- `getData()[B`
- `getData(I)[B`
- `setElem(II)V`
- `setElem(III)V`

`java/awt/image/DataBufferDouble`：
- `getBankData()[[D`
- `getData()[D`
- `getData(I)[D`
- `setElem(II)V`
- `setElem(III)V`
- `setElemDouble(ID)V`
- `setElemDouble(IID)V`
- `setElemFloat(IF)V`
- `setElemFloat(IIF)V`

`java/awt/image/DataBufferFloat`：
- `getBankData()[[F`
- `getData()[F`
- `getData(I)[F`
- `setElem(II)V`
- `setElem(III)V`
- `setElemDouble(ID)V`
- `setElemDouble(IID)V`
- `setElemFloat(IF)V`
- `setElemFloat(IIF)V`

`java/awt/image/DataBufferInt`：
- `getBankData()[[I`
- `getData()[I`
- `getData(I)[I`
- `setElem(II)V`
- `setElem(III)V`

`java/awt/image/DataBufferShort`：
- `getBankData()[[S`
- `getData()[S`
- `getData(I)[S`
- `setElem(II)V`
- `setElem(III)V`

`java/awt/image/DataBufferUShort`：
- `getBankData()[[S`
- `getData()[S`
- `getData(I)[S`
- `setElem(II)V`
- `setElem(III)V`

`java/beans/FeatureDescriptor`：
- `getParameterTypes(Ljava/lang/Class;Ljava/lang/reflect/Method;)[Ljava/lang/Class;`
- `getReturnType(Ljava/lang/Class;Ljava/lang/reflect/Method;)Ljava/lang/Class;`

`java/beans/Introspector`：
- `isEventHandler(Ljava/lang/reflect/Method;)Z`

`java/io/BufferedInputStream`：
- `close()V`
- `getBufIfOpen(Z)[B`

`java/io/File`：
- `readObject(Ljava/io/ObjectInputStream;)V`

`java/io/FilePermission`：
- `equals(Ljava/lang/Object;)Z`
- `hashCode()I`
- `impliesIgnoreMask(Ljava/io/FilePermission;)Z`
- `init(I)V`

`java/io/InputStreamReader`：
- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/io/InputStream;Ljava/lang/String;)V`
- `<init>(Ljava/io/InputStream;Ljava/nio/charset/Charset;)V`
- `<init>(Ljava/io/InputStream;Ljava/nio/charset/CharsetDecoder;)V`
- `close()V`
- `getEncoding()Ljava/lang/String;`
- `read()I`
- `read(Ljava/nio/CharBuffer;)I`
- `read([CII)I`
- `ready()Z`

`java/io/ObjectInputStream`：
- `freeze()V`

`java/io/ObjectStreamClass$FieldReflector`：
- `<init>([Ljava/io/ObjectStreamField;)V`
- `getObjFieldValues(Ljava/lang/Object;[Ljava/lang/Object;)V`
- `getPrimFieldValues(Ljava/lang/Object;[B)V`
- `setObjFieldValues(Ljava/lang/Object;[Ljava/lang/Object;Z)V`
- `setPrimFieldValues(Ljava/lang/Object;[B)V`

`java/io/OutputStreamWriter`：
- `<init>(Ljava/io/OutputStream;)V`
- `<init>(Ljava/io/OutputStream;Ljava/lang/String;)V`
- `<init>(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V`
- `<init>(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V`
- `append(Ljava/lang/CharSequence;)Ljava/io/Writer;`
- `close()V`
- `flush()V`
- `flushBuffer()V`
- `getEncoding()Ljava/lang/String;`
- `write(I)V`
- *（+2 个）*

`java/io/UnixFileSystem`：
- `<init>()V`

`java/lang/CharacterName`：
- `getCodePoint(Ljava/lang/String;)I`

`java/lang/Class`：
- `forName(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;`
- `forName(Ljava/lang/String;)Ljava/lang/Class;`
- `forName(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;`
- `getClassLoader()Ljava/lang/ClassLoader;`
- `getClasses()[Ljava/lang/Class;`
- `getConstructors()[Ljava/lang/reflect/Constructor;`
- `getDeclaredClasses()[Ljava/lang/Class;`
- `getDeclaredConstructors()[Ljava/lang/reflect/Constructor;`
- `getDeclaredFields()[Ljava/lang/reflect/Field;`
- `getDeclaredMethods()[Ljava/lang/reflect/Method;`
- *（+15 个）*

`java/lang/Class$Atomic`：
- `casAnnotationData(Ljava/lang/Class;Ljava/lang/Class$AnnotationData;Ljava/lang/Class$AnnotationData;)Z`
- `casAnnotationType(Ljava/lang/Class;Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z`
- `casReflectionData(Ljava/lang/Class;Ljava/lang/ref/SoftReference;Ljava/lang/ref/SoftReference;)Z`

`java/lang/ClassLoader`：
- `getParent()Ljava/lang/ClassLoader;`
- `getPlatformClassLoader()Ljava/lang/ClassLoader;`
- `getSystemClassLoader()Ljava/lang/ClassLoader;`
- `registerAsParallelCapable()Z`
- `trySetObjectField(Ljava/lang/String;Ljava/lang/Object;)Z`

*（还有更多，省略）*

### 阶段 3 

**42 个内部类，新解锁 399 个边界方法**

| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |
|--------|-------:|-----:|-----:|-----:|---------:|
| `jdk/internal/util/ArraysSupport` | 118 | 0 | 30 | 0 | 2 |
| `sun/reflect/misc/ReflectUtil` | 67 | 0 | 16 | 0 | 2 |
| `jdk/internal/org/objectweb/asm/Label` | 52 | 4 | 8 | 0 | 1 |
| `sun/awt/shell/ShellFolder` | 35 | 8 | 33 | 6 | 2 |
| `sun/util/locale/provider/CalendarDataUtility` | 34 | 0 | 9 | 0 | 1 |
| `jdk/internal/util/random/RandomSupport` | 34 | 2 | 27 | 0 | 1 |
| `com/sun/jmx/remote/internal/ClientCommunicatorAdmin` | 32 | 2 | 2 | 3 | 2 |
| `jdk/internal/reflect/ReflectionFactory` | 27 | 7 | 32 | 0 | 6 |
| `sun/invoke/util/VerifyAccess` | 23 | 1 | 14 | 0 | 2 |
| `jdk/internal/loader/URLClassPath` | 22 | 6 | 14 | 0 | 3 |
| `sun/security/jca/JCAUtil` | 15 | 1 | 5 | 0 | 2 |
| `sun/java2d/cmm/CMSManager` | 15 | 0 | 4 | 0 | 1 |
| `sun/security/util/CryptoAlgorithmConstraints` | 12 | 1 | 7 | 0 | 2 |
| `jdk/internal/vm/Continuation` | 11 | 9 | 36 | 0 | 4 |
| `jdk/internal/util/ClassFileDumper` | 10 | 2 | 8 | 0 | 1 |
| `jdk/internal/loader/ClassLoaders` | 8 | 0 | 6 | 0 | 1 |
| `sun/text/CollatorUtilities` | 7 | 0 | 3 | 0 | 1 |
| `com/sun/jmx/remote/internal/ServerNotifForwarder` | 7 | 2 | 12 | 0 | 3 |
| `sun/reflect/generics/repository/MethodRepository` | 6 | 1 | 3 | 0 | 1 |
| `sun/text/Normalizer` | 5 | 1 | 3 | 0 | 1 |
| `jdk/internal/misc/CarrierThread` | 5 | 1 | 7 | 0 | 1 |
| `jdk/internal/module/ModuleInfo` | 4 | 5 | 11 | 0 | 5 |
| `sun/security/provider/PolicyFile` | 4 | 7 | 28 | 0 | 5 |
| `com/sun/naming/internal/NamingManagerHelper` | 4 | 0 | 12 | 0 | 3 |
| `jdk/internal/net/http/RequestPublishers$ByteArrayPublisher` | 4 | 1 | 5 | 0 | 1 |
| `jdk/internal/foreign/layout/ValueLayouts$OfAddressImpl` | 4 | 14 | 7 | 0 | 1 |
| `jdk/internal/net/http/ResponseSubscribers$ByteArraySubscriber` | 4 | 1 | 7 | 0 | 1 |
| `jdk/internal/net/http/HttpRequestBuilderImpl` | 4 | 7 | 28 | 0 | 1 |
| `jdk/internal/misc/TerminatingThreadLocal` | 3 | 0 | 8 | 0 | 1 |
| `sun/net/www/protocol/file/Handler` | 2 | 1 | 5 | 0 | 1 |
| `jdk/internal/net/http/ResponseSubscribers` | 2 | 3 | 8 | 0 | 3 |
| `sun/java2d/pipe/RenderingEngine` | 2 | 1 | 3 | 5 | 1 |
| `jdk/internal/misc/InnocuousThread` | 2 | 3 | 11 | 0 | 1 |
| `sun/rmi/server/MarshalOutputStream` | 2 | 1 | 6 | 0 | 1 |
| `jdk/internal/vm/ScopedValueContainer` | 2 | 0 | 11 | 0 | 4 |
| `jdk/internal/net/http/HttpClientBuilderImpl` | 2 | 1 | 22 | 0 | 1 |
| `jdk/internal/io/JdkConsoleImpl` | 2 | 3 | 15 | 0 | 6 |
| `sun/security/util/PropertyExpander` | 1 | 0 | 3 | 0 | 1 |
| `com/sun/jmx/remote/internal/ClientNotifForwarder` | 1 | 0 | 14 | 4 | 3 |
| `jdk/internal/logger/LoggerFinderLoader` | 1 | 0 | 11 | 0 | 4 |
| `jdk/internal/net/http/ResponseSubscribers$PathSubscriber` | 1 | 1 | 13 | 0 | 1 |
| `jdk/internal/logger/LazyLoggers` | 1 | 2 | 5 | 0 | 2 |

**新解锁的边界方法（按公开 API 类）：**

`java/awt/BasicStroke`：
- `createStrokedShape(Ljava/awt/Shape;)Ljava/awt/Shape;`

`java/awt/color/ICC_ColorSpace`：
- `fromCIEXYZ([F)[F`
- `fromRGB([F)[F`
- `toCIEXYZ([F)[F`
- `toRGB([F)[F`

`java/awt/color/ICC_Profile`：
- `cmmProfile()Lsun/java2d/cmm/Profile;`
- `getData()[B`
- `getData(Lsun/java2d/cmm/Profile;I)[B`
- `getInstance([B)Ljava/awt/color/ICC_Profile;`
- `setData(I[B)V`

`java/awt/datatransfer/DataFlavor`：
- `tryToLoadClass(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;`

`java/awt/image/ColorConvertOp`：
- `filter(Ljava/awt/image/Raster;Ljava/awt/image/WritableRaster;)Ljava/awt/image/WritableRaster;`
- `updateBITransform(Ljava/awt/color/ICC_Profile;Ljava/awt/color/ICC_Profile;)V`

`java/awt/image/ColorModel`：
- `getGray16TosRGB8LUT(Ljava/awt/color/ICC_ColorSpace;)[B`
- `getGray8TosRGB8LUT(Ljava/awt/color/ICC_ColorSpace;)[B`
- `getLinearGray16ToOtherGray16LUT(Ljava/awt/color/ICC_ColorSpace;)[S`
- `getLinearGray16ToOtherGray8LUT(Ljava/awt/color/ICC_ColorSpace;)[B`

`java/beans/EventHandler`：
- `getClassLoader(Ljava/lang/Class;)Ljava/lang/ClassLoader;`

`java/beans/Introspector`：
- `getBeanInfo(Ljava/lang/Class;)Ljava/beans/BeanInfo;`

`java/beans/MetaData$StaticFieldsPersistenceDelegate`：
- `installFields(Ljava/beans/Encoder;Ljava/lang/Class;)V`

`java/beans/MethodRef`：
- `get()Ljava/lang/reflect/Method;`

`java/beans/PropertyDescriptor`：
- `createPropertyEditor(Ljava/lang/Object;)Ljava/beans/PropertyEditor;`

`java/io/BufferedInputStream`：
- `fill()V`

`java/io/ByteArrayOutputStream`：
- `ensureCapacity(I)V`

`java/io/Console`：
- `instantiateConsole(Z)Ljava/io/Console;`

`java/io/FileInputStream`：
- `readAllBytes()[B`

`java/io/ObjectInputStream`：
- `readNonProxyDesc(Z)Ljava/io/ObjectStreamClass;`
- `readProxyDesc(Z)Ljava/io/ObjectStreamClass;`

`java/io/ObjectOutputStream`：
- `writeNonProxyDesc(Ljava/io/ObjectStreamClass;Z)V`
- `writeProxyDesc(Ljava/io/ObjectStreamClass;Z)V`

`java/io/ObjectStreamClass`：
- `forClass()Ljava/lang/Class;`
- `getSerializableConstructor(Ljava/lang/Class;)Ljava/lang/reflect/Constructor;`

`java/io/ObjectStreamField`：
- `getType()Ljava/lang/Class;`

`java/lang/AbstractStringBuilder`：
- `newCapacity(I)I`

`java/lang/CharacterName`：
- `hashN([BII)I`

`java/lang/Class`：
- `checkPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V`
- `checkPackageAccessForPermittedSubclasses(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V`
- `copyConstructors([Ljava/lang/reflect/Constructor;)[Ljava/lang/reflect/Constructor;`
- `copyFields([Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;`
- `copyMethods([Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;`
- `getConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;`
- `getConstructor0([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;`
- `getDeclaredConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;`
- `getDeclaredField(Ljava/lang/String;)Ljava/lang/reflect/Field;`
- `getDeclaredMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;`
- *（+8 个）*

`java/lang/ClassLoader`：
- `checkPackageAccess(Ljava/lang/Class;Ljava/security/ProtectionDomain;)V`
- `getBuiltinAppClassLoader()Ljava/lang/ClassLoader;`
- `getBuiltinPlatformClassLoader()Ljava/lang/ClassLoader;`

`java/lang/ConditionalSpecialCasing`：
- `isAfterI(Ljava/lang/String;I)Z`
- `isAfterSoftDotted(Ljava/lang/String;I)Z`
- `isBeforeDot(Ljava/lang/String;I)Z`
- `isMoreAbove(Ljava/lang/String;I)Z`

`java/lang/LiveStackFrame`：
- `getStackWalker(Ljdk/internal/vm/Continuation;)Ljava/lang/StackWalker;`

`java/lang/Module`：
- `<init>(Ljava/lang/ModuleLayer;Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;Ljava/net/URI;)V`

`java/lang/ProcessHandleImpl`：
- `lambda$static$0(JLjava/lang/Runnable;)Ljava/lang/Thread;`

`java/lang/PublicMethods$Key`：
- `<init>(Ljava/lang/reflect/Method;)V`
- `matches(Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Z`

`java/lang/ScopedValue$Carrier`：
- `runWith(Ljava/lang/ScopedValue$Snapshot;Ljava/lang/Runnable;)V`
- `runWith(Ljava/lang/ScopedValue$Snapshot;Ljava/util/concurrent/Callable;)Ljava/lang/Object;`

`java/lang/StackStreamFactory$AbstractStackWalker`：
- `getNextBatch()I`
- `hasMoreContinuations()Z`
- `walk()Ljava/lang/Object;`

`java/lang/String`：
- `nonSyncContentEquals(Ljava/lang/AbstractStringBuilder;)Z`
- `regionMatches(ILjava/lang/String;II)Z`
- `startsWith(Ljava/lang/String;I)Z`

`java/lang/StringLatin1`：
- `compareTo([B[BII)I`
- `hashCode([B)I`
- `replace([BI[BI[BI)Ljava/lang/String;`

`java/lang/StringUTF16`：
- `hashCode([B)I`
- `replace([BIZ[BIZ[BIZ)Ljava/lang/String;`

`java/lang/System`：
- `getLogger(Ljava/lang/String;)Ljava/lang/System$Logger;`

`java/lang/System$LoggerFinder`：
- `lambda$accessProvider$0()Ljava/lang/System$LoggerFinder;`

`java/lang/ThreadLocal`：
- `setInitialValue(Ljava/lang/Thread;)Ljava/lang/Object;`

`java/lang/VirtualThread`：
- `afterYield()V`
- `lambda$createDefaultScheduler$0(Ljava/util/concurrent/ForkJoinPool;)Ljava/util/concurrent/ForkJoinWorkerThread;`
- `lambda$createDelayedTaskScheduler$5(Ljava/lang/Runnable;)Ljava/lang/Thread;`
- `runContinuation()V`
- `tryGetStackTrace()[Ljava/lang/StackTraceElement;`
- `yieldContinuation()Z`

`java/lang/VirtualThread$VThreadContinuation`：
- `<init>(Ljava/lang/VirtualThread;Ljava/lang/Runnable;)V`

`java/lang/foreign/MemorySegment`：
- `get(Ljava/lang/foreign/AddressLayout;J)Ljava/lang/foreign/MemorySegment;`
- `set(Ljava/lang/foreign/AddressLayout;JLjava/lang/foreign/MemorySegment;)V`

`java/lang/invoke/DirectMethodHandle`：
- `maybeCompile(Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;)V`
- `shouldBeInitialized(Ljava/lang/invoke/MemberName;)Z`

`java/lang/invoke/InvokerBytecodeGenerator`：
- `<init>(Ljava/lang/invoke/LambdaForm;ILjava/lang/String;Ljava/lang/String;Ljava/lang/invoke/MethodType;)V`
- `classData(Ljava/lang/Object;)Ljava/lang/String;`
- `emitGuardWithCatch(I)Ljava/lang/invoke/LambdaForm$Name;`
- `emitLoop(I)Ljava/lang/invoke/LambdaForm$Name;`
- `emitSelectAlternative(Ljava/lang/invoke/LambdaForm$Name;Ljava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaForm$Name;`
- `emitTableSwitch(II)Ljava/lang/invoke/LambdaForm$Name;`
- `emitTryFinally(I)Ljava/lang/invoke/LambdaForm$Name;`
- `getInternalName(Ljava/lang/Class;)Ljava/lang/String;`
- `isStaticallyInvocable(Ljava/lang/invoke/MemberName;)Z`
- `isStaticallyNameable(Ljava/lang/Class;)Z`

`java/lang/invoke/MemberName`：
- `checkForTypeAlias(Ljava/lang/Class;)V`

`java/lang/invoke/MethodHandle`：
- `isBuiltinLoader(Ljava/lang/ClassLoader;)Z`

`java/lang/invoke/MethodHandleProxies`：
- `asInterfaceInstance(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;)Ljava/lang/Object;`

`java/lang/invoke/MethodHandleStatics`：
- `debugEnabled()Z`

`java/lang/invoke/MethodHandles$Lookup`：
- `accessFailedMessage(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)Ljava/lang/String;`
- `checkAccess(BLjava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `checkSecurityManager(Ljava/lang/Class;)V`
- `checkSecurityManager(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `ensureInitialized(Ljava/lang/Class;)Ljava/lang/Class;`
- `in(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;`
- `isClassAccessible(Ljava/lang/Class;)Z`
- `restrictProtectedReceiver(Ljava/lang/invoke/MemberName;)Z`

`java/lang/invoke/MethodHandles$Lookup$ClassDefiner`：
- `defineClass(ZLjava/lang/Object;)Ljava/lang/Class;`

`java/lang/module/ModuleDescriptor`：
- `read(Ljava/io/InputStream;)Ljava/lang/module/ModuleDescriptor;`
- `read(Ljava/io/InputStream;Ljava/util/function/Supplier;)Ljava/lang/module/ModuleDescriptor;`
- `read(Ljava/nio/ByteBuffer;)Ljava/lang/module/ModuleDescriptor;`
- `read(Ljava/nio/ByteBuffer;Ljava/util/function/Supplier;)Ljava/lang/module/ModuleDescriptor;`

`java/lang/reflect/Constructor`：
- `acquireConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;`

*（还有更多，省略）*

### 阶段 4 

**24 个内部类，新解锁 218 个边界方法**

| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |
|--------|-------:|-----:|-----:|-----:|---------:|
| `sun/util/logging/PlatformLogger` | 384 | 5 | 24 | 0 | 2 |
| `jdk/internal/misc/Blocker` | 120 | 0 | 5 | 0 | 3 |
| `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` | 24 | 3 | 25 | 0 | 1 |
| `sun/reflect/misc/MethodUtil` | 19 | 1 | 9 | 0 | 1 |
| `sun/swing/FilePane` | 18 | 25 | 26 | 0 | 1 |
| `com/sun/jmx/mbeanserver/MBeanSupport` | 14 | 5 | 12 | 4 | 1 |
| `jdk/internal/org/objectweb/asm/ClassReader` | 8 | 11 | 39 | 0 | 4 |
| `sun/util/BuddhistCalendar` | 7 | 0 | 16 | 0 | 1 |
| `jdk/internal/loader/NativeLibraries` | 7 | 9 | 5 | 0 | 4 |
| `sun/rmi/server/LoaderHandler` | 5 | 5 | 16 | 0 | 2 |
| `com/sun/beans/finder/ClassFinder` | 5 | 1 | 4 | 0 | 1 |
| `jdk/internal/loader/BuiltinClassLoader` | 2 | 6 | 33 | 0 | 7 |
| `jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction` | 2 | 0 | 3 | 0 | 1 |
| `jdk/internal/icu/text/StringPrep` | 2 | 5 | 3 | 0 | 2 |
| `com/sun/jmx/remote/util/OrderClassLoaders` | 2 | 0 | 2 | 0 | 1 |
| `com/sun/jmx/mbeanserver/DefaultMXBeanMappingFactory` | 2 | 4 | 17 | 0 | 5 |
| `jdk/internal/net/http/RequestPublishers$StringPublisher` | 2 | 0 | 1 | 0 | 1 |
| `jdk/internal/module/ModulePath` | 1 | 4 | 33 | 0 | 5 |
| `jdk/internal/net/http/ResponseBodyHandlers$FileDownloadBodyHandler` | 1 | 1 | 11 | 0 | 1 |
| `com/sun/jndi/ldap/LdapCtx` | 1 | 43 | 67 | 0 | 1 |
| `jdk/internal/net/http/ResponseBodyHandlers$PathBodyHandler` | 1 | 1 | 2 | 0 | 1 |
| `com/sun/jmx/remote/internal/ArrayNotificationBuffer` | 1 | 6 | 19 | 0 | 3 |
| `com/sun/beans/finder/MethodFinder` | 1 | 2 | 6 | 0 | 4 |
| `com/sun/beans/finder/ConstructorFinder` | 1 | 1 | 1 | 0 | 3 |

**新解锁的边界方法（按公开 API 类）：**

`java/awt/AttributeValue`：
- `<init>(I[Ljava/lang/String;)V`

`java/awt/Component`：
- `applyCompoundShape(Lsun/java2d/pipe/Region;)V`
- `applyCurrentShape()V`
- `calculateCurrentShape()Lsun/java2d/pipe/Region;`
- `createHierarchyEvents(ILjava/awt/Component;Ljava/awt/Container;JZ)I`
- `dispatchMouseWheelToAncestor(Ljava/awt/event/MouseWheelEvent;)Z`
- `isRequestFocusAccepted(ZZLjava/awt/event/FocusEvent$Cause;)Z`
- `mixOnHiding(Z)V`
- `mixOnReshaping()V`
- `mixOnShowing()V`
- `mixOnZOrderChanging(II)V`
- *（+4 个）*

`java/awt/Container`：
- `adjustListeningChildren(JI)V`
- `countHierarchyMembers()I`
- `mixOnHiding(Z)V`
- `mixOnReshaping()V`
- `mixOnShowing()V`
- `mixOnValidating()V`
- `mixOnZOrderChanging(II)V`
- `numListening(J)I`
- `recursiveApplyCurrentShape(II)V`
- `recursiveSubtractAndApplyShape(Lsun/java2d/pipe/Region;II)V`

`java/awt/ContainerOrderFocusTraversalPolicy`：
- `getComponentAfter(Ljava/awt/Container;Ljava/awt/Component;)Ljava/awt/Component;`
- `getComponentBefore(Ljava/awt/Container;Ljava/awt/Component;)Ljava/awt/Component;`
- `getComponentDownCycle(Ljava/awt/Component;I)Ljava/awt/Component;`
- `getFirstComponent(Ljava/awt/Container;)Ljava/awt/Component;`
- `getLastComponent(Ljava/awt/Container;)Ljava/awt/Component;`

`java/awt/Cursor`：
- `getSystemCustomCursor(Ljava/lang/String;)Ljava/awt/Cursor;`

`java/awt/DefaultKeyboardFocusManager`：
- `dequeueKeyEvents(JLjava/awt/Component;)V`
- `dumpMarkers()V`
- `enqueueKeyEvents(JLjava/awt/Component;)V`
- `pumpApprovedKeyEvents()V`
- `typeAheadAssertions(Ljava/awt/Component;Ljava/awt/AWTEvent;)Z`

`java/awt/EventDispatchThread`：
- `addEventFilter(Ljava/awt/EventFilter;)V`
- `processException(Ljava/lang/Throwable;)V`
- `pumpOneEventForFilters(I)V`
- `removeEventFilter(Ljava/awt/EventFilter;)V`

`java/awt/EventQueue`：
- `dispatchEventImpl(Ljava/awt/AWTEvent;Ljava/lang/Object;)V`
- `getEventLog()Lsun/util/logging/PlatformLogger;`

`java/awt/KeyboardFocusManager`：
- `removeLastFocusRequest(Ljava/awt/Component;)V`
- `retargetFocusEvent(Ljava/awt/AWTEvent;)Ljava/awt/AWTEvent;`
- `setGlobalActiveWindow(Ljava/awt/Window;)V`
- `setNativeFocusOwner(Ljava/awt/Component;)V`

`java/awt/KeyboardFocusManager$HeavyweightFocusRequest`：
- `<init>(Ljava/awt/Component;Ljava/awt/Component;ZLjava/awt/event/FocusEvent$Cause;)V`
- `addLightweightRequest(Ljava/awt/Component;ZLjava/awt/event/FocusEvent$Cause;)Z`

`java/awt/LightweightDispatcher`：
- `processMouseEvent(Ljava/awt/event/MouseEvent;)Z`

`java/awt/SplashScreen`：
- `getImageURL()Ljava/net/URL;`

`java/awt/WaitDispatchSupport`：
- `enter()Z`
- `exit()Z`
- `wakeupEDT()V`

`java/awt/WaitDispatchSupport$1`：
- `evaluate()Z`

`java/awt/WaitDispatchSupport$2`：
- `run()V`

`java/awt/WaitDispatchSupport$5`：
- `run()V`

`java/awt/Window`：
- `setGraphicsConfiguration(Ljava/awt/GraphicsConfiguration;)V`

`java/awt/event/InputEvent`：
- `canAccessSystemClipboard()Z`

`java/beans/Beans`：
- `instantiate(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/beans/beancontext/BeanContext;Ljava/beans/AppletInitializer;)Ljava/lang/Object;`

`java/beans/DefaultPersistenceDelegate`：
- `initBean(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;Ljava/beans/Encoder;)V`
- `instantiate(Ljava/lang/Object;Ljava/beans/Encoder;)Ljava/beans/Expression;`

`java/beans/EventHandler`：
- `applyGetters(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;`
- `invokeInternal(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;`

`java/beans/Introspector`：
- `findCustomizerClass(Ljava/lang/Class;)Ljava/lang/Class;`
- `instantiate(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Object;`

`java/beans/ObjectInputStreamWithLoader`：
- `resolveClass(Ljava/io/ObjectStreamClass;)Ljava/lang/Class;`

`java/beans/Statement`：
- `getMethod(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;`
- `invokeInternal()Ljava/lang/Object;`

`java/io/FileDescriptor`：
- `sync()V`

`java/io/FileInputStream`：
- `available()I`
- `length()J`
- `open(Ljava/lang/String;)V`
- `position()J`
- `read()I`
- `read([B)I`
- `read([BII)I`
- `skip(J)J`

`java/io/FileOutputStream`：
- `open(Ljava/lang/String;Z)V`
- `write(I)V`
- `write([B)V`
- `write([BII)V`

`java/io/RandomAccessFile`：
- `length()J`
- `open(Ljava/lang/String;I)V`
- `read()I`
- `readBytes([BII)I`
- `seek(J)V`
- `setLength(J)V`
- `write(I)V`
- `writeBytes([BII)V`

`java/io/UnixFileSystem`：
- `canonicalize(Ljava/lang/String;)Ljava/lang/String;`
- `checkAccess(Ljava/io/File;I)Z`
- `createDirectory(Ljava/io/File;)Z`
- `createFileExclusively(Ljava/lang/String;)Z`
- `delete(Ljava/io/File;)Z`
- `getBooleanAttributes(Ljava/io/File;)I`
- `getLastModifiedTime(Ljava/io/File;)J`
- `getLength(Ljava/io/File;)J`
- `getSpace(Ljava/io/File;I)J`
- `hasBooleanAttributes(Ljava/io/File;I)Z`
- *（+5 个）*

`java/lang/Class`：
- `getReflectionFactory()Ljdk/internal/reflect/ReflectionFactory;`

`java/lang/ClassLoader`：
- `<init>(Ljava/lang/Void;Ljava/lang/String;Ljava/lang/ClassLoader;)V`

`java/lang/Object`：
- `wait(J)V`

`java/lang/invoke/MethodHandles$Lookup$ClassFile`：
- `readClassFile([B)Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;`

`java/lang/module/ModuleFinder`：
- `of([Ljava/nio/file/Path;)Ljava/lang/module/ModuleFinder;`

`java/net/CookieManager`：
- `put(Ljava/net/URI;Ljava/util/Map;)V`

`java/net/IDN`：
- `toASCIIInternal(Ljava/lang/String;I)Ljava/lang/String;`
- `toUnicodeInternal(Ljava/lang/String;I)Ljava/lang/String;`

`java/net/InetAddress$PlatformResolver`：
- `lookupByAddress([B)Ljava/lang/String;`
- `lookupByName(Ljava/lang/String;Ljava/net/spi/InetAddressResolver$LookupPolicy;)Ljava/util/stream/Stream;`

`java/net/http/HttpRequest$BodyPublishers`：
- `ofString(Ljava/lang/String;Ljava/nio/charset/Charset;)Ljava/net/http/HttpRequest$BodyPublisher;`

`java/net/http/HttpResponse$BodyHandlers`：
- `ofFile(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/net/http/HttpResponse$BodyHandler;`
- `ofFileDownload(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/net/http/HttpResponse$BodyHandler;`

`java/nio/MappedMemoryUtils`：
- `force(Ljava/io/FileDescriptor;JZJJ)V`

`java/rmi/server/RMIClassLoader`：
- `getSecurityContext(Ljava/lang/ClassLoader;)Ljava/lang/Object;`

`java/rmi/server/RMIClassLoader$2`：
- `getClassAnnotation(Ljava/lang/Class;)Ljava/lang/String;`
- `getClassLoader(Ljava/lang/String;)Ljava/lang/ClassLoader;`
- `loadClass(Ljava/lang/String;Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;`
- `loadProxyClass(Ljava/lang/String;[Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;`

*（还有更多，省略）*

### 阶段 5 

**10 个内部类，新解锁 74 个边界方法**

| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |
|--------|-------:|-----:|-----:|-----:|---------:|
| `jdk/internal/org/objectweb/asm/ClassWriter` | 51 | 29 | 7 | 0 | 1 |
| `com/sun/beans/decoder/DocumentHandler` | 38 | 3 | 19 | 0 | 3 |
| `jdk/internal/loader/BootLoader` | 26 | 3 | 18 | 0 | 5 |
| `sun/font/FontUtilities` | 5 | 3 | 15 | 0 | 4 |
| `com/sun/jmx/mbeanserver/StandardMBeanSupport` | 4 | 1 | 5 | 0 | 1 |
| `com/sun/jmx/mbeanserver/MXBeanSupport` | 4 | 3 | 5 | 0 | 2 |
| `sun/awt/DebugSettings` | 2 | 4 | 13 | 0 | 2 |
| `jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator` | 1 | 1 | 6 | 0 | 2 |
| `sun/awt/ScrollPaneWheelScroller` | 1 | 0 | 5 | 0 | 2 |
| `sun/awt/SunGraphicsCallback` | 1 | 0 | 4 | 1 | 3 |

**新解锁的边界方法（按公开 API 类）：**

`java/awt/Font`：
- `textRequiresLayout([CII)Z`

`java/awt/GraphicsCallback`：
- `<init>()V`

`java/awt/ScrollPane`：
- `processMouseWheelEvent(Ljava/awt/event/MouseWheelEvent;)V`

`java/awt/Window`：
- `preProcessKeyEvent(Ljava/awt/event/KeyEvent;)V`

`java/beans/XMLDecoder`：
- `<init>(Lorg/xml/sax/InputSource;Ljava/lang/Object;Ljava/beans/ExceptionListener;Ljava/lang/ClassLoader;)V`
- `createHandler(Ljava/lang/Object;Ljava/beans/ExceptionListener;Ljava/lang/ClassLoader;)Lorg/xml/sax/helpers/DefaultHandler;`
- `getExceptionListener()Ljava/beans/ExceptionListener;`
- `parsingComplete()Z`
- `setExceptionListener(Ljava/beans/ExceptionListener;)V`

`java/beans/XMLDecoder$1`：
- `run()Ljava/lang/Void;`

`java/lang/Class`：
- `forName(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;`
- `getPackage()Ljava/lang/Package;`
- `getResource(Ljava/lang/String;)Ljava/net/URL;`
- `getResourceAsStream(Ljava/lang/String;)Ljava/io/InputStream;`

`java/lang/ClassLoader`：
- `findNative(Ljava/lang/ClassLoader;Ljava/lang/String;)J`
- `getPackage(Ljava/lang/String;)Ljava/lang/Package;`
- `getPackages()[Ljava/lang/Package;`
- `getResource(Ljava/lang/String;)Ljava/net/URL;`
- `getResources(Ljava/lang/String;)Ljava/util/Enumeration;`
- `loadLibrary(Ljava/lang/Class;Ljava/io/File;)Ljdk/internal/loader/NativeLibrary;`
- `loadLibrary(Ljava/lang/Class;Ljava/lang/String;)Ljdk/internal/loader/NativeLibrary;`

`java/lang/Module`：
- `defineModules(Ljava/lang/module/Configuration;Ljava/util/function/Function;Ljava/lang/ModuleLayer;)Ljava/util/Map;`
- `getPackages()Ljava/util/Set;`
- `getResourceAsStream(Ljava/lang/String;)Ljava/io/InputStream;`

`java/lang/Package`：
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/net/URL;Ljava/lang/ClassLoader;)V`
- `getPackage(Ljava/lang/String;)Ljava/lang/Package;`
- `getPackageInfo()Ljava/lang/Class;`
- `getPackages()[Ljava/lang/Package;`

`java/lang/invoke/ClassSpecializer$Factory`：
- `generateConcreteSpeciesCodeFile(Ljava/lang/String;Ljava/lang/invoke/ClassSpecializer$SpeciesData;)[B`
- `loadSpecies(Ljava/lang/invoke/ClassSpecializer$SpeciesData;)Ljava/lang/invoke/ClassSpecializer$SpeciesData;`

`java/lang/invoke/GenerateJLIClassesHelper`：
- `generateCodeBytesForLFs(Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/invoke/LambdaForm;)[B`

`java/lang/invoke/InnerClassLambdaMetafactory`：
- `<init>(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/invoke/MethodType;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;Z[Ljava/lang/Class;[Ljava/lang/invoke/MethodType;)V`
- `generateClassInitializer()V`
- `generateConstructor()V`
- `generateInnerClass()Ljava/lang/Class;`
- `generateSerializationFriendlyMethods()V`
- `generateSerializationHostileMethods()V`

`java/lang/invoke/InvokerBytecodeGenerator`：
- `bogusMethod(Ljava/lang/Object;)V`
- `classFilePrologue()Ljdk/internal/org/objectweb/asm/ClassWriter;`
- `clinit(Ljdk/internal/org/objectweb/asm/ClassWriter;Ljava/lang/String;Ljava/util/List;)V`
- `generateLambdaFormInterpreterEntryPointBytes()[B`
- `generateNamedFunctionInvokerImpl(Ljava/lang/invoke/MethodTypeForm;)[B`
- `methodPrologue()V`
- `toByteArray()[B`

`java/lang/invoke/MethodHandleImpl$BindCaller`：
- `generateInvokerTemplate()[B`

`java/lang/reflect/ProxyGenerator`：
- `<init>(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/util/List;I)V`

`java/lang/reflect/ProxyGenerator$ProxyMethod`：
- `generateMethod(Ljdk/internal/org/objectweb/asm/ClassWriter;Ljava/lang/String;)V`

`java/util/ResourceBundle`：
- `getBundleImpl(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Class;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;`

`java/util/ResourceBundle$ResourceBundleProviderHelper`：
- `lambda$loadPropertyResourceBundle$2(Ljava/lang/String;Ljava/lang/Module;Ljava/lang/Module;)Ljava/io/InputStream;`
- `loadResourceBundle(Ljava/lang/Module;Ljava/lang/Module;Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;`

`java/util/ServiceLoader$LazyClassPathLookupIterator`：
- `nextProviderClass()Ljava/lang/Class;`

`java/util/ServiceLoader$ModuleServicesLookupIterator`：
- `iteratorFor(Ljava/lang/ClassLoader;)Ljava/util/Iterator;`

`java/util/SplittableRandom$AbstractSplittableGeneratorProxy`：
- `<init>(Ljava/util/SplittableRandom;)V`

`java/util/zip/ZipUtils`：
- `loadLibrary()V`

`javax/management/StandardMBean`：
- `setImplementation(Ljava/lang/Object;)V`

`javax/swing/plaf/nimbus/NimbusDefaults`：
- `<init>()V`

`javax/swing/plaf/synth/SynthParser`：
- `characters([CII)V`
- `endDocument()V`
- `endElement(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `error(Lorg/xml/sax/SAXParseException;)V`
- `fatalError(Lorg/xml/sax/SAXParseException;)V`
- `getHandler()Lcom/sun/beans/decoder/DocumentHandler;`
- `ignorableWhitespace([CII)V`
- `lookup(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;`
- `notationDecl(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `processingInstruction(Ljava/lang/String;Ljava/lang/String;)V`
- *（+7 个）*

`javax/swing/text/StyleContext`：
- `getFont(Ljava/lang/String;II)Ljava/awt/Font;`

### 阶段 6 

**3 个内部类，新解锁 12 个边界方法**

| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |
|--------|-------:|-----:|-----:|-----:|---------:|
| `sun/font/FontResolver` | 12 | 0 | 10 | 0 | 3 |
| `com/sun/jmx/mbeanserver/Introspector` | 11 | 3 | 18 | 0 | 6 |
| `sun/font/FontDesignMetrics` | 3 | 1 | 22 | 0 | 2 |

**新解锁的边界方法（按公开 API 类）：**

`java/awt/Component`：
- `getFontMetrics(Ljava/awt/Font;)Ljava/awt/FontMetrics;`

`java/awt/font/StyledParagraph`：
- `addFonts([CLjava/util/Map;II)V`

`java/awt/font/TextLayout`：
- `singleFont([CIILjava/util/Map;)Ljava/awt/Font;`

`java/awt/font/TextLine`：
- `getFontAtCurrentPos(Ljava/text/AttributedCharacterIterator;)Ljava/awt/Font;`

`javax/management/JMX`：
- `createProxy(Ljavax/management/MBeanServerConnection;Ljavax/management/ObjectName;Ljava/lang/Class;ZZ)Ljava/lang/Object;`
- `isMXBeanInterface(Ljava/lang/Class;)Z`

`javax/management/MBeanAttributeInfo`：
- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/reflect/Method;Ljava/lang/reflect/Method;)V`

`javax/management/MBeanConstructorInfo`：
- `<init>(Ljava/lang/String;Ljava/lang/reflect/Constructor;)V`

`javax/management/MBeanOperationInfo`：
- `<init>(Ljava/lang/String;Ljava/lang/reflect/Method;)V`
- `parameters([Ljava/lang/Class;[[Ljava/lang/annotation/Annotation;)[Ljavax/management/MBeanParameterInfo;`

`javax/management/StandardMBean`：
- `construct(Ljava/lang/Object;Ljava/lang/Class;ZZ)V`

`javax/management/monitor/Monitor`：
- `getComparableFromAttribute(Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Comparable;`

### 阶段 7 （循环依赖组）

**158 个内部类，新解锁 1572 个边界方法**

| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |
|--------|-------:|-----:|-----:|-----:|---------:|
| `jdk/internal/misc/ScopedMemoryAccess` | 466 | 184 | 193 | 0 | 6 |
| `sun/swing/SwingUtilities2` | 338 | 5 | 82 | 0 | 10 |
| `jdk/internal/foreign/AbstractMemorySegmentImpl` | 305 | 0 | 68 | 5 | 16 |
| `sun/awt/AppContext` | 297 | 8 | 11 | 0 | 1 |
| `sun/util/calendar/LocalGregorianCalendar$Date` | 170 | 0 | 13 | 0 | 3 |
| `sun/util/calendar/BaseCalendar$Date` | 160 | 0 | 8 | 2 | 1 |
| `sun/swing/DefaultLookup` | 132 | 0 | 16 | 0 | 1 |
| `sun/util/calendar/CalendarDate` | 118 | 0 | 45 | 0 | 2 |
| `sun/awt/SunToolkit` | 106 | 22 | 114 | 10 | 16 |
| `sun/util/calendar/LocalGregorianCalendar` | 102 | 0 | 27 | 0 | 6 |
| `sun/util/calendar/BaseCalendar` | 80 | 1 | 17 | 0 | 3 |
| `sun/font/CoreMetrics` | 78 | 0 | 6 | 0 | 1 |
| `sun/font/AttributeValues` | 72 | 0 | 98 | 0 | 2 |
| `sun/awt/geom/Curve` | 68 | 4 | 28 | 22 | 1 |
| `sun/security/jca/GetInstance` | 67 | 0 | 16 | 0 | 3 |
| `jdk/internal/foreign/MemorySessionImpl` | 54 | 8 | 19 | 3 | 4 |
| `sun/swing/MenuItemLayoutHelper` | 54 | 2 | 107 | 0 | 4 |
| `sun/security/util/DerValue` | 49 | 2 | 65 | 0 | 7 |
| `sun/security/x509/X500Name` | 37 | 14 | 45 | 0 | 4 |
| `sun/security/util/DerInputStream` | 30 | 0 | 40 | 0 | 1 |
| `sun/util/locale/InternalLocaleBuilder` | 29 | 1 | 19 | 0 | 5 |
| `jdk/internal/foreign/Utils` | 28 | 0 | 25 | 0 | 4 |
| `sun/awt/image/ByteInterleavedRaster` | 27 | 0 | 31 | 0 | 1 |
| `sun/util/calendar/Era` | 26 | 0 | 10 | 0 | 2 |
| `sun/util/calendar/Gregorian` | 25 | 2 | 12 | 0 | 1 |
| `sun/security/krb5/PrincipalName` | 21 | 1 | 30 | 0 | 6 |
| `sun/util/locale/provider/LocaleResources` | 20 | 3 | 38 | 0 | 5 |
| `sun/security/util/DerOutputStream` | 19 | 4 | 34 | 0 | 2 |
| `sun/awt/image/IntegerInterleavedRaster` | 19 | 0 | 19 | 0 | 1 |
| `sun/util/locale/LocaleExtensions` | 18 | 3 | 13 | 0 | 3 |
| `sun/security/krb5/EncryptionKey` | 18 | 5 | 17 | 0 | 8 |
| `sun/awt/image/SunWritableRaster` | 18 | 5 | 9 | 0 | 3 |
| `com/sun/beans/introspect/PropertyInfo` | 17 | 6 | 13 | 0 | 3 |
| `sun/util/calendar/ZoneInfo` | 16 | 0 | 30 | 0 | 4 |
| `sun/font/CreatedFontTracker` | 16 | 3 | 8 | 0 | 1 |
| `jdk/internal/module/Modules` | 15 | 1 | 19 | 0 | 7 |
| `jdk/internal/foreign/LayoutPath$PathElementImpl` | 15 | 0 | 4 | 0 | 1 |
| `jdk/internal/misc/ThreadFlock` | 15 | 3 | 22 | 0 | 4 |
| `sun/util/locale/LanguageTag` | 14 | 0 | 40 | 0 | 4 |
| `sun/util/locale/provider/TimeZoneNameUtility` | 14 | 0 | 9 | 0 | 3 |
| `sun/security/krb5/Credentials` | 14 | 19 | 17 | 0 | 3 |
| `sun/security/x509/AlgorithmId` | 13 | 0 | 26 | 0 | 5 |
| `sun/security/x509/X509CertImpl` | 13 | 28 | 53 | 0 | 26 |
| `sun/security/util/SignatureFileVerifier` | 13 | 0 | 22 | 0 | 4 |
| `sun/java2d/SunGraphics2D` | 12 | 13 | 110 | 0 | 7 |
| `sun/awt/image/ShortInterleavedRaster` | 12 | 0 | 23 | 0 | 1 |
| `com/sun/beans/introspect/ClassInfo` | 12 | 1 | 6 | 0 | 4 |
| `sun/rmi/server/UnicastServerRef` | 12 | 0 | 23 | 0 | 7 |
| `sun/reflect/annotation/AnnotationParser` | 11 | 15 | 26 | 0 | 4 |
| `com/sun/java/swing/SwingUtilities3` | 10 | 0 | 11 | 0 | 6 |
| `sun/security/util/ObjectIdentifier` | 10 | 1 | 26 | 0 | 4 |
| `sun/awt/AWTAutoShutdown` | 10 | 2 | 15 | 0 | 3 |
| `jdk/internal/vm/ThreadContainer` | 10 | 0 | 10 | 1 | 2 |
| `sun/security/x509/NameConstraintsExtension` | 9 | 0 | 14 | 0 | 17 |
| `jdk/internal/foreign/LayoutPath` | 9 | 0 | 25 | 0 | 2 |
| `sun/security/jca/Providers` | 9 | 1 | 13 | 0 | 4 |
| `jdk/internal/math/FormattedFPDecimal` | 9 | 1 | 21 | 0 | 1 |
| `sun/awt/image/ImagingLib` | 9 | 7 | 4 | 0 | 1 |
| `sun/reflect/annotation/TypeAnnotationParser` | 9 | 5 | 11 | 0 | 5 |
| `sun/java2d/SunGraphicsEnvironment` | 8 | 8 | 18 | 3 | 5 |
| `jdk/internal/vm/SharedThreadContainer` | 8 | 0 | 11 | 0 | 3 |
| `sun/font/FontLineMetrics` | 8 | 0 | 15 | 0 | 1 |
| `sun/awt/HeadlessToolkit` | 8 | 0 | 53 | 0 | 2 |
| `sun/font/StandardGlyphVector` | 8 | 18 | 56 | 0 | 4 |
| `sun/awt/image/IntegerComponentRaster` | 7 | 1 | 20 | 0 | 1 |
| `sun/rmi/server/UnicastServerRef2` | 7 | 0 | 7 | 0 | 3 |
| `sun/awt/geom/Crossings` | 7 | 0 | 13 | 2 | 2 |
| `sun/rmi/transport/LiveRef` | 7 | 3 | 16 | 0 | 2 |
| `sun/reflect/annotation/AnnotationSupport` | 6 | 1 | 8 | 0 | 3 |
| `sun/nio/ch/FileChannelImpl` | 6 | 14 | 31 | 0 | 8 |
| `com/sun/beans/introspect/EventSetInfo` | 6 | 7 | 4 | 0 | 1 |
| `sun/util/calendar/CalendarSystem` | 6 | 1 | 3 | 16 | 1 |
| `sun/awt/image/BytePackedRaster` | 6 | 1 | 27 | 0 | 1 |
| `sun/util/calendar/ZoneInfoFile` | 6 | 2 | 19 | 0 | 1 |
| `jdk/internal/ref/PhantomCleanable` | 5 | 0 | 9 | 1 | 1 |
| `sun/security/x509/PrivateKeyUsageExtension` | 5 | 0 | 10 | 0 | 4 |
| `sun/swing/table/DefaultTableCellHeaderRenderer` | 5 | 1 | 5 | 0 | 1 |
| `sun/reflect/annotation/AnnotationType` | 5 | 0 | 9 | 0 | 3 |
| `sun/security/krb5/Realm` | 5 | 4 | 13 | 0 | 5 |
| `sun/security/x509/X509CRLImpl` | 4 | 18 | 29 | 0 | 9 |
| `sun/util/calendar/JulianCalendar` | 4 | 2 | 17 | 0 | 3 |
| `sun/security/x509/URIName` | 4 | 0 | 16 | 0 | 5 |
| `sun/security/x509/DNSName` | 4 | 0 | 11 | 0 | 3 |
| `sun/security/x509/IPAddressName` | 4 | 2 | 12 | 0 | 4 |
| `sun/security/x509/CertificatePolicySet` | 4 | 0 | 5 | 0 | 3 |
| `sun/security/util/SignatureUtil` | 4 | 4 | 16 | 0 | 3 |
| `sun/font/AttributeMap` | 4 | 0 | 7 | 0 | 1 |
| `sun/rmi/registry/RegistryImpl` | 4 | 2 | 16 | 0 | 6 |
| `jdk/internal/ref/CleanerImpl` | 4 | 1 | 4 | 0 | 3 |
| `sun/awt/dnd/SunDragSourceContextPeer` | 4 | 2 | 27 | 2 | 2 |
| `sun/awt/EmbeddedFrame` | 4 | 1 | 36 | 2 | 3 |
| `sun/security/krb5/internal/ktab/KeyTab` | 4 | 7 | 16 | 0 | 3 |
| `sun/swing/plaf/synth/Paint9Painter` | 4 | 0 | 8 | 0 | 2 |
| `sun/security/x509/RFC822Name` | 4 | 0 | 11 | 0 | 3 |
| `sun/security/x509/OIDName` | 4 | 0 | 11 | 0 | 4 |
| `sun/security/x509/CRLNumberExtension` | 3 | 4 | 7 | 0 | 4 |
| `sun/security/x509/GeneralName` | 3 | 0 | 9 | 0 | 12 |
| `sun/font/GraphicComponent` | 3 | 0 | 32 | 0 | 3 |
| `sun/security/x509/SubjectAlternativeNameExtension` | 3 | 0 | 8 | 0 | 5 |
| `sun/security/x509/CertificatePoliciesExtension` | 3 | 0 | 8 | 0 | 5 |
| `sun/awt/image/SurfaceManager` | 3 | 2 | 11 | 2 | 2 |
| `sun/rmi/server/UnicastRef2` | 3 | 0 | 5 | 0 | 2 |
| `sun/reflect/generics/scope/ClassScope` | 3 | 1 | 2 | 0 | 2 |
| `jdk/internal/loader/Loader` | 3 | 7 | 24 | 0 | 6 |
| `sun/swing/CachedPainter` | 3 | 4 | 5 | 1 | 2 |
| `sun/font/Decoration` | 3 | 4 | 4 | 0 | 1 |
| `sun/font/GlyphLayout` | 3 | 4 | 2 | 0 | 4 |
| `sun/security/x509/ExtendedKeyUsageExtension` | 3 | 0 | 9 | 0 | 6 |
| `jdk/internal/vm/ThreadContainers` | 3 | 0 | 13 | 0 | 3 |
| `sun/awt/image/ByteComponentRaster` | 3 | 1 | 24 | 0 | 1 |
| `sun/security/x509/EDIPartyName` | 2 | 0 | 12 | 0 | 4 |
| `jdk/internal/module/ModuleBootstrap` | 2 | 2 | 37 | 0 | 10 |
| `sun/java2d/SunCompositeContext` | 2 | 2 | 2 | 0 | 1 |
| `sun/awt/image/ByteBandedRaster` | 2 | 0 | 25 | 0 | 1 |
| `jdk/internal/ref/CleanerImpl$PhantomCleanableRef` | 2 | 0 | 5 | 0 | 1 |
| `sun/swing/plaf/DesktopProperty` | 2 | 3 | 11 | 0 | 1 |
| `sun/security/x509/CertificatePolicyId` | 2 | 0 | 7 | 0 | 3 |
| `jdk/internal/math/DoubleToDecimal` | 2 | 1 | 20 | 0 | 1 |
| `sun/security/x509/GeneralSubtree` | 2 | 0 | 9 | 0 | 4 |
| `jdk/internal/loader/LoaderPool` | 2 | 0 | 4 | 0 | 1 |
| `sun/rmi/server/UnicastRef` | 2 | 0 | 19 | 0 | 4 |
| `sun/security/x509/InvalidityDateExtension` | 2 | 0 | 9 | 0 | 3 |
| `sun/awt/image/ShortBandedRaster` | 2 | 0 | 25 | 0 | 1 |
| `sun/security/x509/OtherName` | 2 | 0 | 12 | 0 | 5 |
| `sun/awt/geom/AreaOp$NZWindOp` | 2 | 0 | 4 | 0 | 2 |
| `sun/awt/im/InputMethodContext` | 2 | 7 | 18 | 0 | 2 |
| `sun/awt/geom/AreaOp$EOWindOp` | 2 | 0 | 4 | 0 | 1 |
| `sun/awt/image/SunVolatileImage` | 2 | 5 | 21 | 0 | 3 |
| `sun/awt/image/OffScreenImageSource` | 2 | 0 | 9 | 0 | 2 |
| `sun/rmi/transport/tcp/TCPEndpoint` | 2 | 1 | 33 | 0 | 2 |
| `jdk/internal/foreign/SlicingAllocator` | 2 | 0 | 3 | 0 | 2 |
| `sun/security/x509/GeneralSubtrees` | 2 | 0 | 22 | 0 | 15 |
| `sun/awt/geom/Crossings$EvenOdd` | 2 | 0 | 3 | 0 | 1 |
| `sun/java2d/SurfaceData` | 2 | 11 | 34 | 5 | 4 |
| `sun/security/x509/X400Address` | 2 | 0 | 6 | 0 | 2 |
| `jdk/internal/foreign/NativeMemorySegmentImpl` | 1 | 1 | 14 | 0 | 6 |
| `jdk/internal/foreign/HeapMemorySegmentImpl$OfByte` | 1 | 2 | 7 | 0 | 2 |
| `sun/rmi/transport/tcp/TCPTransport` | 1 | 4 | 15 | 0 | 2 |
| `sun/security/x509/Extension` | 1 | 0 | 16 | 0 | 4 |
| `jdk/internal/foreign/HeapMemorySegmentImpl$OfInt` | 1 | 2 | 7 | 0 | 2 |
| `sun/security/jgss/krb5/Krb5Util` | 1 | 4 | 5 | 0 | 4 |
| `sun/security/x509/X509Key` | 1 | 2 | 20 | 0 | 5 |
| `jdk/internal/foreign/layout/SequenceLayoutImpl` | 1 | 7 | 15 | 0 | 1 |
| `sun/awt/geom/AreaOp` | 1 | 3 | 5 | 3 | 1 |
| `jdk/internal/module/SystemModuleFinders` | 1 | 4 | 6 | 0 | 7 |
| `jdk/internal/foreign/HeapMemorySegmentImpl$OfShort` | 1 | 2 | 7 | 0 | 2 |
| `sun/reflect/generics/scope/MethodScope` | 1 | 1 | 3 | 0 | 1 |
| `sun/security/x509/GeneralNames` | 1 | 0 | 12 | 0 | 4 |
| `sun/security/util/AnchorCertificates` | 1 | 0 | 3 | 0 | 2 |
| `sun/reflect/generics/scope/ConstructorScope` | 1 | 1 | 3 | 0 | 1 |
| `sun/awt/image/ShortComponentRaster` | 1 | 1 | 24 | 0 | 1 |
| `sun/awt/im/InputContext` | 1 | 9 | 35 | 0 | 4 |
| `sun/security/x509/X509CRLEntryImpl` | 1 | 8 | 19 | 0 | 7 |
| `jdk/internal/foreign/HeapMemorySegmentImpl$OfLong` | 1 | 2 | 7 | 0 | 2 |
| `sun/security/x509/PolicyInformation` | 1 | 0 | 8 | 0 | 4 |
| `jdk/internal/foreign/HeapMemorySegmentImpl$OfChar` | 1 | 2 | 7 | 0 | 2 |
| `jdk/internal/foreign/HeapMemorySegmentImpl$OfDouble` | 1 | 2 | 7 | 0 | 2 |
| `jdk/internal/foreign/HeapMemorySegmentImpl$OfFloat` | 1 | 2 | 7 | 0 | 2 |

**新解锁的边界方法（按公开 API 类）：**

`java/awt/AWTKeyStroke`：
- `getCachedStroke(CIIZ)Ljava/awt/AWTKeyStroke;`

`java/awt/AlphaComposite`：
- `createContext(Ljava/awt/image/ColorModel;Ljava/awt/image/ColorModel;Ljava/awt/RenderingHints;)Ljava/awt/CompositeContext;`

`java/awt/ColorPaintContext`：
- `getRaster(IIII)Ljava/awt/image/Raster;`

`java/awt/Component`：
- `<init>()V`
- `createBufferStrategy(ILjava/awt/BufferCapabilities;)V`
- `dispatchEventImpl(Ljava/awt/AWTEvent;)V`
- `getContainingWindow()Ljava/awt/Window;`
- `getNextFocusCandidate()Ljava/awt/Component;`
- `isMixingNeeded()Z`
- `readObject(Ljava/io/ObjectInputStream;)V`
- `repaint(JIIII)V`
- `requestFocusHelper(ZZLjava/awt/event/FocusEvent$Cause;)Z`

`java/awt/Component$BltBufferStrategy`：
- `getDrawGraphics()Ljava/awt/Graphics;`

`java/awt/Container`：
- `startLWModal()V`
- `stopLWModal()V`

`java/awt/DefaultKeyboardFocusManager`：
- `dispatchEvent(Ljava/awt/AWTEvent;)Z`
- `doRestoreFocus(Ljava/awt/Component;Ljava/awt/Component;Z)Z`
- `repostIfFollowsKeyEvents(Ljava/awt/event/WindowEvent;)Z`
- `sendMessage(Ljava/awt/Component;Ljava/awt/AWTEvent;)Z`

`java/awt/DefaultKeyboardFocusManager$3`：
- `evaluate()Z`

`java/awt/DefaultKeyboardFocusManager$4`：
- `run()V`

`java/awt/Desktop`：
- `<init>()V`
- `getDesktop()Ljava/awt/Desktop;`
- `isDesktopSupported()Z`

`java/awt/Dialog`：
- `<init>(Ljava/awt/Window;Ljava/lang/String;Ljava/awt/Dialog$ModalityType;)V`
- `<init>(Ljava/awt/Window;Ljava/lang/String;Ljava/awt/Dialog$ModalityType;Ljava/awt/GraphicsConfiguration;)V`
- `modalityPopped()V`
- `modalityPushed()V`
- `readObject(Ljava/io/ObjectInputStream;)V`
- `show()V`

`java/awt/EventDispatchThread`：
- `filterAndCheckEvent(Ljava/awt/AWTEvent;)Z`

`java/awt/EventDispatchThread$HierarchyEventFilter`：
- `acceptEvent(Ljava/awt/AWTEvent;)Ljava/awt/EventFilter$FilterAction;`

`java/awt/EventQueue`：
- `<init>()V`
- `detachDispatchThread(Ljava/awt/EventDispatchThread;)V`
- `getNextEvent()Ljava/awt/AWTEvent;`
- `getNextEvent(I)Ljava/awt/AWTEvent;`
- `initDispatchThread()V`
- `pop()V`
- `postEvent(Ljava/awt/AWTEvent;)V`
- `postEvent(Ljava/awt/AWTEvent;I)V`
- `postEventPrivate(Ljava/awt/AWTEvent;)V`
- `push(Ljava/awt/EventQueue;)V`
- *（+1 个）*

`java/awt/EventQueue$6`：
- `run()Ljava/awt/EventDispatchThread;`

`java/awt/Font`：
- `<init>(Ljava/awt/Font;)V`
- `<init>(Ljava/util/Map;)V`
- `<init>(Lsun/font/AttributeValues;Ljava/lang/String;IZLsun/font/Font2DHandle;)V`
- `applyStyle(ILsun/font/AttributeValues;)V`
- `applyTransform(Ljava/awt/geom/AffineTransform;Lsun/font/AttributeValues;)V`
- `createFont(ILjava/io/InputStream;)Ljava/awt/Font;`
- `createFont0(ILjava/io/InputStream;ZLsun/font/CreatedFontTracker;)[Ljava/awt/Font;`
- `createFonts(Ljava/io/InputStream;)[Ljava/awt/Font;`
- `createGlyphVector(Ljava/awt/font/FontRenderContext;Ljava/lang/String;)Ljava/awt/font/GlyphVector;`
- `createGlyphVector(Ljava/awt/font/FontRenderContext;Ljava/text/CharacterIterator;)Ljava/awt/font/GlyphVector;`
- *（+24 个）*

`java/awt/Frame`：
- `init(Ljava/lang/String;Ljava/awt/GraphicsConfiguration;)V`

`java/awt/GradientPaintContext`：
- `getRaster(IIII)Ljava/awt/image/Raster;`

`java/awt/GraphicsConfiguration`：
- `createCompatibleVolatileImage(IILjava/awt/ImageCapabilities;I)Ljava/awt/image/VolatileImage;`

`java/awt/GraphicsDevice`：
- `getFullScreenWindow()Ljava/awt/Window;`
- `isWindowOpacitySupported()Z`
- `isWindowPerpixelTranslucencySupported()Z`
- `isWindowShapingSupported()Z`
- `setFullScreenWindow(Ljava/awt/Window;)V`

`java/awt/GraphicsEnvironment`：
- `getCenterPoint()Ljava/awt/Point;`
- `getMaximumWindowBounds()Ljava/awt/Rectangle;`

`java/awt/Image`：
- `flush()V`
- `getCapabilities(Ljava/awt/GraphicsConfiguration;)Ljava/awt/ImageCapabilities;`
- `setAccelerationPriority(F)V`

`java/awt/KeyboardFocusManager`：
- `focusedWindowChanged(Ljava/awt/Component;Ljava/awt/Component;)Z`
- `getActiveWindow()Ljava/awt/Window;`
- `getCurrentFocusCycleRoot()Ljava/awt/Container;`
- `getCurrentKeyboardFocusManager()Ljava/awt/KeyboardFocusManager;`
- `getCurrentKeyboardFocusManager(Lsun/awt/AppContext;)Ljava/awt/KeyboardFocusManager;`
- `getFocusOwner()Ljava/awt/Component;`
- `getFocusedWindow()Ljava/awt/Window;`
- `getPermanentFocusOwner()Ljava/awt/Component;`
- `isTemporary(Ljava/awt/Component;Ljava/awt/Component;)Z`
- `markClearGlobalFocusOwner()Ljava/awt/Window;`
- *（+4 个）*

`java/awt/LightweightDispatcher`：
- `eventDispatched(Ljava/awt/AWTEvent;)V`
- `retargetMouseEvent(Ljava/awt/Component;ILjava/awt/event/MouseEvent;)V`

`java/awt/MenuComponent`：
- `<init>()V`
- `readObject(Ljava/io/ObjectInputStream;)V`

`java/awt/Polygon`：
- `contains(DDDD)Z`
- `getCrossings(DDDD)Lsun/awt/geom/Crossings;`
- `intersects(DDDD)Z`

`java/awt/Robot`：
- `createCompatibleImage(Ljava/awt/Rectangle;Z)[Ljava/awt/image/BufferedImage;`
- `getPixelColor(II)Ljava/awt/Color;`
- `initLegalButtonMask()V`
- `waitForIdle()V`

`java/awt/ScrollPane`：
- `addImpl(Ljava/awt/Component;Ljava/lang/Object;I)V`

`java/awt/SentEvent`：
- `dispatch()V`
- `dispose()V`

`java/awt/SequencedEvent`：
- `<init>(Ljava/awt/AWTEvent;)V`
- `dispatch()V`
- `dispose()V`
- `isOwnerAppContextDisposed(Ljava/awt/SequencedEvent;)Z`

*（还有更多，省略）*

## 边界方法详细列表（按公开 API 类）

> 公开 API 中直接调用了内部类的方法，需要手写 native 实现。

### `java/applet/Applet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newAudioClip(Ljava/net/URL;)Ljava/applet/AudioClip;` | `com/sun/media/sound/JavaSoundAudioClip` |

### `java/awt/AWTEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `copyPrivateDataInto(Ljava/awt/AWTEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$InputEventAccessor` |
| `dispatched()V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$InputEventAccessor` |

### `java/awt/AWTKeyStroke`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCachedStroke(CIIZ)Ljava/awt/AWTKeyStroke;` | `sun/awt/AppContext`, `sun/swing/SwingAccessor`, `sun/swing/SwingAccessor$KeyStrokeAccessor` |

### `java/awt/AlphaComposite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createContext(Ljava/awt/image/ColorModel;Ljava/awt/image/ColorModel;Ljava/awt/RenderingHints;)Ljava/awt/CompositeContext;` | `sun/java2d/SunCompositeContext` |

### `java/awt/AttributeValue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I[Ljava/lang/String;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/BasicStroke`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createStrokedShape(Ljava/awt/Shape;)Ljava/awt/Shape;` | `sun/java2d/pipe/RenderingEngine` |

### `java/awt/Button`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/Canvas`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/Checkbox`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/CheckboxMenuItem`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/Choice`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/ColorPaintContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getRaster(IIII)Ljava/awt/image/Raster;` | `sun/awt/image/IntegerComponentRaster` |

### `java/awt/Component`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/AppContext` |
| `addNotify()V` | `sun/awt/ComponentFactory` |
| `applyCompoundShape(Lsun/java2d/pipe/Region;)V` | `sun/java2d/pipe/Region`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `applyCurrentShape()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `calculateCurrentShape()Lsun/java2d/pipe/Region;` | `sun/java2d/pipe/Region`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `createBufferStrategy(ILjava/awt/BufferCapabilities;)V` | `sun/java2d/SunGraphicsEnvironment` |
| `createHierarchyEvents(ILjava/awt/Component;Ljava/awt/Container;JZ)I` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `dispatchEventImpl(Ljava/awt/AWTEvent;)V` | `sun/awt/AppContext`, `sun/awt/SunToolkit`, `sun/awt/dnd/SunDropTargetEvent`, `sun/awt/im/InputContext`, +2 |
| `dispatchMouseWheelToAncestor(Ljava/awt/event/MouseWheelEvent;)Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `doSwingSerialization()V` | `sun/swing/SwingAccessor`, `sun/swing/SwingAccessor$JComponentAccessor` |
| `findUnderMouseInWindow(Ljava/awt/PointerInfo;)Ljava/awt/Component;` | `sun/awt/ComponentFactory` |
| `getContainingWindow()Ljava/awt/Window;` | `sun/awt/SunToolkit` |
| `getFontMetrics(Ljava/awt/Font;)Ljava/awt/FontMetrics;` | `sun/font/FontDesignMetrics` |
| `getGraphics()Ljava/awt/Graphics;` | `sun/awt/ConstrainableGraphics` |
| `getGraphics_NoClientCode()Ljava/awt/Graphics;` | `sun/awt/ConstrainableGraphics` |
| `getNextFocusCandidate()Ljava/awt/Component;` | `sun/awt/EmbeddedFrame`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getNormalShape()Lsun/java2d/pipe/Region;` | `sun/java2d/pipe/Region` |
| `isMixingNeeded()Z` | `sun/awt/SunToolkit`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `isNonOpaqueForMixing()Z` | `sun/java2d/pipe/Region` |
| `isRequestFocusAccepted(ZZLjava/awt/event/FocusEvent$Cause;)Z` | `sun/awt/RequestFocusController`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnHiding(Z)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnReshaping()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnShowing()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnZOrderChanging(II)V` | `sun/java2d/pipe/Region`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `numListening(J)I` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/awt/AppContext` |
| `repaint(JIIII)V` | `sun/awt/SunToolkit` |
| `requestFocusHelper(ZZLjava/awt/event/FocusEvent$Cause;)Z` | `sun/awt/SunToolkit`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `setMixingCutoutShape(Ljava/awt/Shape;)V` | `sun/java2d/pipe/Region` |
| `subtractAndApplyShape(Lsun/java2d/pipe/Region;)V` | `sun/java2d/pipe/Region`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `transferFocus(Z)Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `transferFocusBackward(Z)Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/Component$BltBufferStrategy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDrawGraphics()Ljava/awt/Graphics;` | `sun/java2d/SunGraphics2D` |

### `java/awt/Component$FlipBufferStrategy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createBuffers(ILjava/awt/BufferCapabilities;)V` | `sun/awt/image/VSyncedBSManager`, `sun/java2d/pipe/hw/ExtendedBufferCapabilities`, `sun/java2d/pipe/hw/ExtendedBufferCapabilities$VSyncType` |
| `destroyBuffers()V` | `sun/awt/image/VSyncedBSManager` |

### `java/awt/Component$ProxyCapabilities`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/BufferCapabilities;)V` | `sun/java2d/pipe/hw/ExtendedBufferCapabilities` |

### `java/awt/Container`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `adjustListeningChildren(JI)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `countHierarchyMembers()I` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getOpaqueShape()Lsun/java2d/pipe/Region;` | `sun/java2d/pipe/Region` |
| `mixOnHiding(Z)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnReshaping()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnShowing()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnValidating()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `mixOnZOrderChanging(II)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `numListening(J)I` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `recursiveApplyCurrentShape(II)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `recursiveSubtractAndApplyShape(Lsun/java2d/pipe/Region;II)V` | `sun/java2d/pipe/Region`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `startLWModal()V` | `sun/awt/AppContext`, `sun/awt/PeerEvent` |
| `stopLWModal()V` | `sun/awt/PeerEvent`, `sun/awt/SunToolkit` |

### `java/awt/ContainerOrderFocusTraversalPolicy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getComponentAfter(Ljava/awt/Container;Ljava/awt/Component;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getComponentBefore(Ljava/awt/Container;Ljava/awt/Component;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getComponentDownCycle(Ljava/awt/Component;I)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getFirstComponent(Ljava/awt/Container;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getLastComponent(Ljava/awt/Container;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/Cursor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getSystemCustomCursor(Ljava/lang/String;)Ljava/awt/Cursor;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `setPData(J)V` | `sun/java2d/Disposer` |

### `java/awt/DefaultKeyboardFocusManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `dequeueKeyEvents(JLjava/awt/Component;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `dispatchEvent(Ljava/awt/AWTEvent;)Z` | `sun/awt/SunToolkit`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `doRestoreFocus(Ljava/awt/Component;Ljava/awt/Component;Z)Z` | `sun/awt/SunToolkit` |
| `dumpMarkers()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `enqueueKeyEvents(JLjava/awt/Component;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `initStatic()V` | `sun/awt/AWTAccessor` |
| `pumpApprovedKeyEvents()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `repostIfFollowsKeyEvents(Ljava/awt/event/WindowEvent;)Z` | `sun/awt/AppContext`, `sun/awt/SunToolkit`, `sun/awt/TimedWindowEvent` |
| `sendMessage(Ljava/awt/Component;Ljava/awt/AWTEvent;)Z` | `sun/awt/AppContext`, `sun/awt/SunToolkit` |
| `typeAheadAssertions(Ljava/awt/Component;Ljava/awt/AWTEvent;)Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/DefaultKeyboardFocusManager$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `evaluate()Z` | `sun/awt/AppContext` |

### `java/awt/DefaultKeyboardFocusManager$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `sun/awt/AppContext` |

### `java/awt/Desktop`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/SunToolkit` |
| `getDesktop()Ljava/awt/Desktop;` | `sun/awt/AppContext` |
| `isDesktopSupported()Z` | `sun/awt/SunToolkit` |

### `java/awt/Dialog`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/Window;Ljava/lang/String;Ljava/awt/Dialog$ModalityType;)V` | `sun/awt/SunToolkit`, `sun/awt/util/IdentityArrayList` |
| `<init>(Ljava/awt/Window;Ljava/lang/String;Ljava/awt/Dialog$ModalityType;Ljava/awt/GraphicsConfiguration;)V` | `sun/awt/SunToolkit`, `sun/awt/util/IdentityArrayList` |
| `addNotify()V` | `sun/awt/ComponentFactory` |
| `blockWindow(Ljava/awt/Window;)V` | `sun/awt/util/IdentityArrayList` |
| `blockWindows(Ljava/util/List;)V` | `sun/awt/util/IdentityArrayList` |
| `checkModalityPermission(Ljava/awt/Dialog$ModalityType;)V` | `sun/awt/AWTPermissions` |
| `checkShouldBeBlocked(Ljava/awt/Window;)V` | `sun/awt/util/IdentityArrayList` |
| `conditionalShow(Ljava/awt/Component;Ljava/util/concurrent/atomic/AtomicLong;)Z` | `sun/awt/util/IdentityArrayList` |
| `hideAndDisposePreHandler()V` | `sun/awt/util/IdentityArrayList` |
| `modalHide()V` | `sun/awt/util/IdentityArrayList` |
| `modalShow()V` | `sun/awt/util/IdentityArrayList` |
| `modalityPopped()V` | `sun/awt/SunToolkit` |
| `modalityPushed()V` | `sun/awt/SunToolkit` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/awt/SunToolkit`, `sun/awt/util/IdentityArrayList` |
| `show()V` | `sun/awt/AppContext` |
| `toBack()V` | `sun/awt/util/IdentityArrayList` |
| `unblockWindow(Ljava/awt/Window;)V` | `sun/awt/util/IdentityArrayList` |

### `java/awt/EventDispatchThread`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addEventFilter(Ljava/awt/EventFilter;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `filterAndCheckEvent(Ljava/awt/AWTEvent;)Z` | `sun/awt/dnd/SunDragSourceContextPeer` |
| `processException(Ljava/lang/Throwable;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `pumpOneEventForFilters(I)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `removeEventFilter(Ljava/awt/EventFilter;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/EventDispatchThread$HierarchyEventFilter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acceptEvent(Ljava/awt/AWTEvent;)Ljava/awt/EventFilter$FilterAction;` | `sun/awt/SunToolkit` |

### `java/awt/EventQueue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/AppContext` |
| `cacheEQItem(Lsun/awt/EventQueueItem;)V` | `sun/awt/EventQueueItem` |
| `coalesceMouseEvent(Ljava/awt/event/MouseEvent;)Z` | `sun/awt/EventQueueItem` |
| `coalesceOtherEvent(Ljava/awt/AWTEvent;I)Z` | `sun/awt/EventQueueItem` |
| `coalescePaintEvent(Ljava/awt/event/PaintEvent;)Z` | `sun/awt/EventQueueItem` |
| `coalescePeerEvent(Lsun/awt/PeerEvent;)Z` | `sun/awt/EventQueueItem`, `sun/awt/PeerEvent` |
| `createSecondaryLoop(Ljava/awt/Conditional;Ljava/awt/EventFilter;J)Ljava/awt/SecondaryLoop;` | `sun/awt/FwDispatcher` |
| `detachDispatchThread(Ljava/awt/EventDispatchThread;)V` | `sun/awt/AWTAutoShutdown`, `sun/awt/SunToolkit` |
| `dispatchEvent(Ljava/awt/AWTEvent;)V` | `jdk/internal/access/JavaSecurityAccess` |
| `dispatchEventImpl(Ljava/awt/AWTEvent;Ljava/lang/Object;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getEventLog()Lsun/util/logging/PlatformLogger;` | `sun/util/logging/PlatformLogger` |
| `getNextEvent()Ljava/awt/AWTEvent;` | `sun/awt/AWTAutoShutdown`, `sun/awt/SunToolkit` |
| `getNextEvent(I)Ljava/awt/AWTEvent;` | `sun/awt/EventQueueItem`, `sun/awt/SunToolkit` |
| `getNextEventPrivate()Ljava/awt/AWTEvent;` | `sun/awt/EventQueueItem` |
| `getPriority(Ljava/awt/AWTEvent;)I` | `sun/awt/PeerEvent` |
| `initDispatchThread()V` | `sun/awt/AppContext` |
| `isDispatchThreadImpl()Z` | `sun/awt/FwDispatcher` |
| `peekEvent()Ljava/awt/AWTEvent;` | `sun/awt/EventQueueItem` |
| `peekEvent(I)Ljava/awt/AWTEvent;` | `sun/awt/EventQueueItem` |
| `pop()V` | `sun/awt/AppContext`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `postEvent(Ljava/awt/AWTEvent;)V` | `sun/awt/SunToolkit` |
| `postEvent(Ljava/awt/AWTEvent;I)V` | `sun/awt/AWTAutoShutdown`, `sun/awt/EventQueueItem` |
| `postEventPrivate(Ljava/awt/AWTEvent;)V` | `sun/awt/AWTAutoShutdown` |
| `push(Ljava/awt/EventQueue;)V` | `sun/awt/AppContext`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `removeSourceEvents(Ljava/lang/Object;Z)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$InvocationEventAccessor`, `sun/awt/EventQueueItem`, `sun/awt/SunToolkit`, +1 |
| `uncacheEQItem(Lsun/awt/EventQueueItem;)V` | `sun/awt/EventQueueItem` |

### `java/awt/EventQueue$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `sun/awt/FwDispatcher` |

### `java/awt/EventQueue$5`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `jdk/internal/access/JavaSecurityAccess` |

### `java/awt/EventQueue$6`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/awt/EventDispatchThread;` | `sun/awt/AWTAutoShutdown` |

### `java/awt/FileDialog`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/Font`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/Font;)V` | `sun/font/AttributeValues` |
| `<init>(Ljava/io/File;IZLsun/font/CreatedFontTracker;)V` | `sun/font/Font2D`, `sun/font/Font2DHandle`, `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `<init>(Ljava/lang/String;IFZLsun/font/Font2DHandle;)V` | `sun/font/Font2D`, `sun/font/Font2DHandle`, `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `<init>(Ljava/util/Map;)V` | `sun/font/AttributeValues` |
| `<init>(Lsun/font/AttributeValues;Ljava/lang/String;IZLsun/font/Font2DHandle;)V` | `sun/font/AttributeValues`, `sun/font/Font2DHandle`, `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `<init>(Lsun/font/Font2D;)V` | `sun/font/Font2D` |
| `applyStyle(ILsun/font/AttributeValues;)V` | `sun/font/AttributeValues` |
| `applyTransform(Ljava/awt/geom/AffineTransform;Lsun/font/AttributeValues;)V` | `sun/font/AttributeValues` |
| `canDisplay(C)Z` | `sun/font/Font2D` |
| `canDisplay(I)Z` | `sun/font/Font2D` |
| `canDisplayUpTo(Ljava/lang/String;)I` | `sun/font/Font2D` |
| `canDisplayUpTo(Ljava/text/CharacterIterator;II)I` | `sun/font/Font2D` |
| `canDisplayUpTo([CII)I` | `sun/font/Font2D` |
| `createFont(ILjava/io/InputStream;)Ljava/awt/Font;` | `sun/font/CreatedFontTracker` |
| `createFont0(ILjava/io/InputStream;ZLsun/font/CreatedFontTracker;)[Ljava/awt/Font;` | `sun/font/CreatedFontTracker`, `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `createFonts(Ljava/io/File;)[Ljava/awt/Font;` | `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `createFonts(Ljava/io/InputStream;)[Ljava/awt/Font;` | `sun/font/CreatedFontTracker` |
| `createGlyphVector(Ljava/awt/font/FontRenderContext;Ljava/lang/String;)Ljava/awt/font/GlyphVector;` | `sun/font/StandardGlyphVector` |
| `createGlyphVector(Ljava/awt/font/FontRenderContext;Ljava/text/CharacterIterator;)Ljava/awt/font/GlyphVector;` | `sun/font/StandardGlyphVector` |
| `createGlyphVector(Ljava/awt/font/FontRenderContext;[C)Ljava/awt/font/GlyphVector;` | `sun/font/StandardGlyphVector` |
| `createGlyphVector(Ljava/awt/font/FontRenderContext;[I)Ljava/awt/font/GlyphVector;` | `sun/font/StandardGlyphVector` |
| `defaultLineMetrics(Ljava/awt/font/FontRenderContext;)Lsun/font/FontLineMetrics;` | `sun/font/AttributeValues`, `sun/font/CoreMetrics`, `sun/font/Font2D`, `sun/font/FontLineMetrics` |
| `deriveFont(F)Ljava/awt/Font;` | `sun/font/AttributeValues` |
| `deriveFont(I)Ljava/awt/Font;` | `sun/font/AttributeValues` |
| `deriveFont(IF)Ljava/awt/Font;` | `sun/font/AttributeValues` |
| `deriveFont(ILjava/awt/geom/AffineTransform;)Ljava/awt/Font;` | `sun/font/AttributeValues` |
| `deriveFont(Ljava/awt/geom/AffineTransform;)Ljava/awt/Font;` | `sun/font/AttributeValues` |
| `deriveFont(Ljava/util/Map;)Ljava/awt/Font;` | `sun/font/AttributeValues` |
| `equals(Ljava/lang/Object;)Z` | `sun/font/AttributeValues` |
| `getAttributeValues()Lsun/font/AttributeValues;` | `sun/font/AttributeValues` |
| `getAttributes()Ljava/util/Map;` | `sun/font/AttributeMap` |
| `getBaselineFor(C)B` | `sun/font/Font2D` |
| `getFamily(Ljava/util/Locale;)Ljava/lang/String;` | `sun/font/Font2D` |
| `getFont(Ljava/util/Map;)Ljava/awt/Font;` | `sun/font/AttributeMap`, `sun/font/AttributeValues`, `sun/font/EAttribute` |
| `getFont2D()Lsun/font/Font2D;` | `sun/font/Font2D`, `sun/font/Font2DHandle`, `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `getFontName(Ljava/util/Locale;)Ljava/lang/String;` | `sun/font/Font2D` |
| `getFontPeer()Ljava/awt/peer/FontPeer;` | `sun/awt/ComponentFactory` |
| `getItalicAngle(Ljava/awt/font/FontRenderContext;)F` | `sun/font/Font2D` |
| `getLineMetrics(Ljava/lang/String;IILjava/awt/font/FontRenderContext;)Ljava/awt/font/LineMetrics;` | `sun/font/FontLineMetrics` |
| `getLineMetrics(Ljava/lang/String;Ljava/awt/font/FontRenderContext;)Ljava/awt/font/LineMetrics;` | `sun/font/FontLineMetrics` |
| `getLineMetrics(Ljava/text/CharacterIterator;IILjava/awt/font/FontRenderContext;)Ljava/awt/font/LineMetrics;` | `sun/font/FontLineMetrics` |
| `getLineMetrics([CIILjava/awt/font/FontRenderContext;)Ljava/awt/font/LineMetrics;` | `sun/font/FontLineMetrics` |
| `getMaxCharBounds(Ljava/awt/font/FontRenderContext;)Ljava/awt/geom/Rectangle2D;` | `sun/font/Font2D` |
| `getMissingGlyphCode()I` | `sun/font/Font2D` |
| `getNumGlyphs()I` | `sun/font/Font2D` |
| `getPSName()Ljava/lang/String;` | `sun/font/Font2D` |
| `getStringBounds([CIILjava/awt/font/FontRenderContext;)Ljava/awt/geom/Rectangle2D;` | `sun/font/AttributeValues`, `sun/font/FontDesignMetrics`, `sun/font/FontUtilities` |
| `getTransform()Ljava/awt/geom/AffineTransform;` | `sun/font/AttributeValues`, `sun/font/EAttribute` |
| `hashCode()I` | `sun/font/AttributeValues` |
| `initFromValues(Lsun/font/AttributeValues;)V` | `sun/font/AttributeValues` |
| `layoutGlyphVector(Ljava/awt/font/FontRenderContext;[CIII)Ljava/awt/font/GlyphVector;` | `sun/font/GlyphLayout` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/font/AttributeValues` |
| `textRequiresLayout([CII)Z` | `sun/font/FontUtilities` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `sun/font/AttributeValues` |

### `java/awt/Font$FontAccessImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/font/FontAccess` |

### `java/awt/Frame`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |
| `init(Ljava/lang/String;Ljava/awt/GraphicsConfiguration;)V` | `sun/awt/SunToolkit` |

### `java/awt/GradientPaintContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getRaster(IIII)Ljava/awt/image/Raster;` | `sun/awt/image/IntegerComponentRaster` |

### `java/awt/GraphicsCallback`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/SunGraphicsCallback` |

### `java/awt/GraphicsConfiguration`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createCompatibleVolatileImage(IILjava/awt/ImageCapabilities;I)Ljava/awt/image/VolatileImage;` | `sun/awt/image/SunVolatileImage` |

### `java/awt/GraphicsDevice`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFullScreenWindow()Ljava/awt/Window;` | `sun/awt/AppContext` |
| `isWindowOpacitySupported()Z` | `sun/awt/SunToolkit` |
| `isWindowPerpixelTranslucencySupported()Z` | `sun/awt/SunToolkit` |
| `isWindowShapingSupported()Z` | `sun/awt/SunToolkit` |
| `setFullScreenWindow(Ljava/awt/Window;)V` | `sun/awt/AppContext`, `sun/awt/SunToolkit` |

### `java/awt/GraphicsEnvironment`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCenterPoint()Ljava/awt/Point;` | `sun/java2d/SunGraphicsEnvironment` |
| `getHeadlessMessage()Ljava/lang/String;` | `sun/awt/PlatformGraphicsInfo` |
| `getMaximumWindowBounds()Ljava/awt/Rectangle;` | `sun/java2d/SunGraphicsEnvironment` |
| `lambda$getHeadlessProperty$0()Ljava/lang/Void;` | `sun/awt/PlatformGraphicsInfo` |
| `preferLocaleFonts()V` | `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `preferProportionalFonts()V` | `sun/font/FontManager`, `sun/font/FontManagerFactory` |
| `registerFont(Ljava/awt/Font;)Z` | `sun/font/FontManager`, `sun/font/FontManagerFactory` |

### `java/awt/GraphicsEnvironment$LocalGE`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createGE()Ljava/awt/GraphicsEnvironment;` | `sun/awt/PlatformGraphicsInfo`, `sun/java2d/HeadlessGraphicsEnvironment` |

### `java/awt/Image`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `flush()V` | `sun/awt/image/SurfaceManager` |
| `getCapabilities(Ljava/awt/GraphicsConfiguration;)Ljava/awt/ImageCapabilities;` | `sun/awt/image/SurfaceManager` |
| `setAccelerationPriority(F)V` | `sun/awt/image/SurfaceManager` |

### `java/awt/Image$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/image/SurfaceManager$ImageAccessor` |

### `java/awt/KeyboardFocusManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `focusedWindowChanged(Ljava/awt/Component;Ljava/awt/Component;)Z` | `sun/awt/SunToolkit` |
| `getActiveWindow()Ljava/awt/Window;` | `sun/awt/AppContext` |
| `getCurrentFocusCycleRoot()Ljava/awt/Container;` | `sun/awt/AppContext` |
| `getCurrentKeyboardFocusManager()Ljava/awt/KeyboardFocusManager;` | `sun/awt/AppContext` |
| `getCurrentKeyboardFocusManager(Lsun/awt/AppContext;)Ljava/awt/KeyboardFocusManager;` | `sun/awt/AppContext` |
| `getFocusOwner()Ljava/awt/Component;` | `sun/awt/AppContext` |
| `getFocusedWindow()Ljava/awt/Window;` | `sun/awt/AppContext` |
| `getPermanentFocusOwner()Ljava/awt/Component;` | `sun/awt/AppContext` |
| `initPeer()V` | `sun/awt/KeyboardFocusManagerPeerProvider` |
| `isProxyActiveImpl(Ljava/awt/event/KeyEvent;)Z` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$KeyEventAccessor` |
| `isTemporary(Ljava/awt/Component;Ljava/awt/Component;)Z` | `sun/awt/SunToolkit` |
| `markClearGlobalFocusOwner()Ljava/awt/Window;` | `sun/awt/SunToolkit` |
| `processCurrentLightweightRequests()V` | `sun/awt/AppContext` |
| `processSynchronousLightweightTransfer(Ljava/awt/Component;Ljava/awt/Component;ZZJ)Z` | `sun/awt/SunToolkit` |
| `removeLastFocusRequest(Ljava/awt/Component;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `retargetFocusEvent(Ljava/awt/AWTEvent;)Ljava/awt/AWTEvent;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `setCurrentKeyboardFocusManager(Ljava/awt/KeyboardFocusManager;)V` | `sun/awt/AppContext` |
| `setGlobalActiveWindow(Ljava/awt/Window;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `setNativeFocusOwner(Ljava/awt/Component;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `shouldNativelyFocusHeavyweight(Ljava/awt/Component;Ljava/awt/Component;ZZJLjava/awt/event/FocusEvent$Cause;)I` | `sun/awt/SunToolkit`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/KeyboardFocusManager$HeavyweightFocusRequest`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/Component;Ljava/awt/Component;ZLjava/awt/event/FocusEvent$Cause;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `addLightweightRequest(Ljava/awt/Component;ZLjava/awt/event/FocusEvent$Cause;)Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/Label`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/LightweightDispatcher`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `eventDispatched(Ljava/awt/AWTEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor`, `sun/awt/AppContext`, `sun/awt/SunToolkit` |
| `processDropTargetEvent(Lsun/awt/dnd/SunDropTargetEvent;)Z` | `sun/awt/dnd/SunDropTargetEvent` |
| `processMouseEvent(Ljava/awt/event/MouseEvent;)Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `retargetMouseEvent(Ljava/awt/Component;ILjava/awt/event/MouseEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor`, `sun/awt/AppContext`, `sun/awt/dnd/SunDropTargetEvent` |

### `java/awt/List`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/MediaTracker`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getResolutionVariant(Ljava/awt/Image;)Ljava/awt/Image;` | `sun/awt/image/MultiResolutionToolkitImage` |

### `java/awt/Menu`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/MenuBar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/MenuComponent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/AppContext` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/awt/AppContext` |

### `java/awt/MenuItem`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/MouseInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPointerInfo()Ljava/awt/PointerInfo;` | `sun/awt/AWTPermissions`, `sun/awt/ComponentFactory` |

### `java/awt/Panel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/Polygon`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `contains(DDDD)Z` | `sun/awt/geom/Crossings` |
| `getCrossings(DDDD)Lsun/awt/geom/Crossings;` | `sun/awt/geom/Crossings`, `sun/awt/geom/Crossings$EvenOdd` |
| `intersects(DDDD)Z` | `sun/awt/geom/Crossings` |

### `java/awt/PopupMenu`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/Robot`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkRobotAllowed()V` | `sun/awt/AWTPermissions` |
| `checkScreenCaptureAllowed()V` | `sun/awt/AWTPermissions` |
| `createCompatibleImage(Ljava/awt/Rectangle;Z)[Ljava/awt/image/BufferedImage;` | `sun/awt/image/SunWritableRaster`, `sun/java2d/SunGraphicsEnvironment` |
| `getPixelColor(II)Ljava/awt/Color;` | `sun/java2d/SunGraphicsEnvironment` |
| `init(Ljava/awt/GraphicsDevice;)V` | `sun/awt/ComponentFactory` |
| `initLegalButtonMask()V` | `sun/awt/SunToolkit` |
| `waitForIdle()V` | `sun/awt/SunToolkit` |

### `java/awt/ScrollPane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addImpl(Ljava/awt/Component;Ljava/lang/Object;I)V` | `sun/awt/SunToolkit` |
| `addNotify()V` | `sun/awt/ComponentFactory` |
| `processMouseWheelEvent(Ljava/awt/event/MouseWheelEvent;)V` | `sun/awt/ScrollPaneWheelScroller` |

### `java/awt/Scrollbar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/SentEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `dispatch()V` | `sun/awt/SunToolkit` |
| `dispose()V` | `sun/awt/SunToolkit` |

### `java/awt/SequencedEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/AWTEvent;)V` | `sun/awt/SunToolkit` |
| `dispatch()V` | `sun/awt/AppContext` |
| `dispose()V` | `sun/awt/SunToolkit` |
| `isOwnerAppContextDisposed(Ljava/awt/SequencedEvent;)Z` | `sun/awt/AppContext` |

### `java/awt/SplashScreen`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getImageURL()Ljava/net/URL;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `update()V` | `sun/awt/image/SunWritableRaster` |

### `java/awt/SystemTray`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `add(Ljava/awt/TrayIcon;)V` | `sun/awt/AppContext` |
| `addNotify()V` | `sun/awt/HeadlessToolkit`, `sun/awt/SunToolkit` |
| `checkSystemTrayAllowed()V` | `sun/awt/AWTPermissions` |
| `getCurrentChangeSupport()Ljava/beans/PropertyChangeSupport;` | `sun/awt/AppContext` |
| `getTrayIcons()[Ljava/awt/TrayIcon;` | `sun/awt/AppContext` |
| `isSupported()Z` | `sun/awt/HeadlessToolkit`, `sun/awt/SunToolkit` |
| `remove(Ljava/awt/TrayIcon;)V` | `sun/awt/AppContext` |

### `java/awt/Taskbar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/SunToolkit` |
| `getTaskbar()Ljava/awt/Taskbar;` | `sun/awt/AppContext` |
| `isTaskbarSupported()Z` | `sun/awt/SunToolkit` |

### `java/awt/TextArea`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/TextComponent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `canAccessClipboard()Z` | `sun/awt/AWTPermissions` |
| `enableInputMethodsIfNecessary()V` | `sun/awt/InputMethodSupport` |

### `java/awt/TextField`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory` |

### `java/awt/TexturePaintContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getContext(Ljava/awt/image/BufferedImage;Ljava/awt/geom/AffineTransform;Ljava/awt/RenderingHints;Ljava/awt/Rectangle;)Ljava/awt/PaintContext;` | `sun/awt/image/ByteInterleavedRaster`, `sun/awt/image/IntegerInterleavedRaster` |
| `getRaster(IIII)Ljava/awt/image/Raster;` | `sun/awt/image/SunWritableRaster` |

### `java/awt/TexturePaintContext$Byte`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/awt/image/ByteInterleavedRaster;Ljava/awt/image/ColorModel;Ljava/awt/geom/AffineTransform;I)V` | `sun/awt/image/ByteInterleavedRaster` |
| `makeRaster(II)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ByteInterleavedRaster` |

### `java/awt/TexturePaintContext$ByteFilter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/awt/image/ByteInterleavedRaster;Ljava/awt/image/ColorModel;Ljava/awt/geom/AffineTransform;I)V` | `sun/awt/image/ByteInterleavedRaster` |
| `makeRaster(II)Ljava/awt/image/WritableRaster;` | `sun/awt/image/IntegerInterleavedRaster` |

### `java/awt/TexturePaintContext$Int`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/awt/image/IntegerInterleavedRaster;Ljava/awt/image/ColorModel;Ljava/awt/geom/AffineTransform;IZ)V` | `sun/awt/image/IntegerInterleavedRaster` |
| `makeRaster(II)Ljava/awt/image/WritableRaster;` | `sun/awt/image/IntegerInterleavedRaster` |

### `java/awt/Toolkit`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addAWTEventListener(Ljava/awt/event/AWTEventListener;J)V` | `sun/awt/AWTPermissions` |
| `getAWTEventListeners()[Ljava/awt/event/AWTEventListener;` | `sun/awt/AWTPermissions` |
| `getAWTEventListeners(J)[Ljava/awt/event/AWTEventListener;` | `sun/awt/AWTPermissions` |
| `getDefaultToolkit()Ljava/awt/Toolkit;` | `sun/awt/HeadlessToolkit`, `sun/awt/PlatformGraphicsInfo` |
| `getDesktopProperty(Ljava/lang/String;)Ljava/lang/Object;` | `sun/awt/HeadlessToolkit` |
| `getSystemEventQueue()Ljava/awt/EventQueue;` | `sun/awt/AWTPermissions` |
| `initStatic()V` | `sun/awt/AWTAccessor` |
| `notifyAWTEventListeners(Ljava/awt/AWTEvent;)V` | `sun/awt/HeadlessToolkit` |
| `removeAWTEventListener(Ljava/awt/event/AWTEventListener;)V` | `sun/awt/AWTPermissions` |
| `setDesktopProperty(Ljava/lang/String;Ljava/lang/Object;)V` | `sun/awt/HeadlessToolkit` |

### `java/awt/Toolkit$DesktopPropertyChangeSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addPropertyChangeListener(Ljava/beans/PropertyChangeListener;)V` | `sun/awt/AppContext` |
| `addPropertyChangeListener(Ljava/lang/String;Ljava/beans/PropertyChangeListener;)V` | `sun/awt/AppContext` |
| `firePropertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/awt/AppContext`, `sun/awt/PeerEvent`, `sun/awt/SunToolkit` |
| `getPropertyChangeListeners()[Ljava/beans/PropertyChangeListener;` | `sun/awt/AppContext` |
| `getPropertyChangeListeners(Ljava/lang/String;)[Ljava/beans/PropertyChangeListener;` | `sun/awt/AppContext` |
| `removePropertyChangeListener(Ljava/beans/PropertyChangeListener;)V` | `sun/awt/AppContext` |
| `removePropertyChangeListener(Ljava/lang/String;Ljava/beans/PropertyChangeListener;)V` | `sun/awt/AppContext` |

### `java/awt/Toolkit$DesktopPropertyChangeSupport$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `sun/awt/AppContext` |

### `java/awt/TrayIcon`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/AppContext`, `sun/awt/SunToolkit` |
| `addNotify()V` | `sun/awt/HeadlessToolkit`, `sun/awt/SunToolkit` |

### `java/awt/WaitDispatchSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `enter()Z` | `sun/awt/PeerEvent`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `exit()Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `wakeupEDT()V` | `sun/awt/PeerEvent`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/WaitDispatchSupport$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `evaluate()Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/WaitDispatchSupport$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `sun/util/logging/PlatformLogger` |

### `java/awt/WaitDispatchSupport$5`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `sun/util/logging/PlatformLogger` |

### `java/awt/Window`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/ComponentFactory`, `sun/awt/util/IdentityArrayList` |
| `addToWindowList()V` | `sun/awt/AppContext` |
| `closeSplashScreen()V` | `sun/awt/SunToolkit` |
| `getAllUnblockedWindows()Lsun/awt/util/IdentityArrayList;` | `sun/awt/util/IdentityArrayList` |
| `getAllWindows()Lsun/awt/util/IdentityArrayList;` | `sun/awt/util/IdentityArrayList` |
| `getWindows()[Ljava/awt/Window;` | `sun/awt/AppContext` |
| `getWindows(Lsun/awt/AppContext;)[Ljava/awt/Window;` | `sun/awt/AppContext` |
| `init(Ljava/awt/GraphicsConfiguration;)V` | `sun/awt/SunToolkit`, `sun/java2d/Disposer` |
| `initDeserializedWindow()V` | `sun/java2d/Disposer` |
| `preProcessKeyEvent(Ljava/awt/event/KeyEvent;)V` | `sun/awt/DebugSettings` |
| `removeFromWindowList(Lsun/awt/AppContext;Ljava/lang/ref/WeakReference;)V` | `sun/awt/AppContext` |
| `removeNotify()V` | `sun/awt/util/IdentityArrayList` |
| `setAlwaysOnTop(Z)V` | `sun/awt/AWTPermissions` |
| `setGraphicsConfiguration(Ljava/awt/GraphicsConfiguration;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `setLayersOpaque(Ljava/awt/Component;Z)V` | `sun/awt/SunToolkit` |
| `setLocationRelativeTo(Ljava/awt/Component;)V` | `sun/awt/SunToolkit` |
| `setModalExclusionType(Ljava/awt/Dialog$ModalExclusionType;)V` | `sun/awt/AWTPermissions` |
| `setShape(Ljava/awt/Shape;)V` | `sun/java2d/pipe/Region` |
| `setWarningString()V` | `sun/awt/AWTPermissions`, `sun/security/action/GetPropertyAction` |

### `java/awt/color/ICC_ColorSpace`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `fromCIEXYZ([F)[F` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |
| `fromRGB([F)[F` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |
| `toCIEXYZ([F)[F` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |
| `toRGB([F)[F` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |

### `java/awt/color/ICC_Profile`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `cmmProfile()Lsun/java2d/cmm/Profile;` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/PCMM`, `sun/java2d/cmm/ProfileDeferralInfo` |
| `getColorSpaceType()I` | `sun/java2d/cmm/ProfileDeferralInfo` |
| `getData()[B` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/PCMM` |
| `getData(Lsun/java2d/cmm/Profile;I)[B` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/PCMM` |
| `getInstance([B)Ljava/awt/color/ICC_Profile;` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/PCMM`, `sun/java2d/cmm/ProfileDataVerifier` |
| `getNumComponents()I` | `sun/java2d/cmm/ProfileDeferralInfo` |
| `getProfileClass()I` | `sun/java2d/cmm/ProfileDeferralInfo` |
| `setData(I[B)V` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/PCMM` |

### `java/awt/datatransfer/Clipboard`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$fireFlavorsChanged$2(Ljava/awt/datatransfer/FlavorListener;)V` | `sun/datatransfer/DataFlavorUtil`, `sun/datatransfer/DesktopDatatransferService` |
| `setContents(Ljava/awt/datatransfer/Transferable;Ljava/awt/datatransfer/ClipboardOwner;)V` | `sun/datatransfer/DataFlavorUtil`, `sun/datatransfer/DesktopDatatransferService` |

### `java/awt/datatransfer/DataFlavor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `equals(Ljava/awt/datatransfer/DataFlavor;)Z` | `sun/datatransfer/DataFlavorUtil` |
| `getTextPlainUnicodeFlavor()Ljava/awt/datatransfer/DataFlavor;` | `sun/datatransfer/DataFlavorUtil`, `sun/datatransfer/DesktopDatatransferService` |
| `hashCode()I` | `sun/datatransfer/DataFlavorUtil` |
| `isFlavorTextType()Z` | `sun/datatransfer/DataFlavorUtil` |
| `isRepresentationClassRemote()Z` | `sun/datatransfer/DataFlavorUtil$RMI` |
| `paramString()Ljava/lang/String;` | `sun/datatransfer/DataFlavorUtil` |
| `selectBestTextFlavor([Ljava/awt/datatransfer/DataFlavor;)Ljava/awt/datatransfer/DataFlavor;` | `sun/datatransfer/DataFlavorUtil` |
| `tryToLoadClass(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `java/awt/datatransfer/SystemFlavorMap`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `convertMimeTypeToDataFlavors(Ljava/lang/String;)Ljava/util/Set;` | `sun/datatransfer/DataFlavorUtil` |
| `flavorToNativeLookup(Ljava/awt/datatransfer/DataFlavor;Z)Ljava/util/LinkedHashSet;` | `sun/datatransfer/DataFlavorUtil`, `sun/datatransfer/DesktopDatatransferService` |
| `getDefaultFlavorMap()Ljava/awt/datatransfer/FlavorMap;` | `sun/datatransfer/DataFlavorUtil`, `sun/datatransfer/DesktopDatatransferService` |
| `getNativesForFlavor(Ljava/awt/datatransfer/DataFlavor;)Ljava/util/List;` | `sun/datatransfer/DataFlavorUtil` |
| `initSystemFlavorMap()V` | `sun/datatransfer/DataFlavorUtil`, `sun/datatransfer/DesktopDatatransferService` |
| `nativeToFlavorLookup(Ljava/lang/String;)Ljava/util/LinkedHashSet;` | `sun/datatransfer/DataFlavorUtil`, `sun/datatransfer/DesktopDatatransferService` |

### `java/awt/dnd/DragSource`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDragThreshold()I` | `sun/security/action/GetIntegerAction` |
| `startDrag(Ljava/awt/dnd/DragGestureEvent;Ljava/awt/Cursor;Ljava/awt/Image;Ljava/awt/Point;Ljava/awt/datatransfer/Transferable;Ljava/awt/dnd/DragSourceListener;Ljava/awt/datatransfer/FlavorMap;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$DragSourceContextAccessor`, `sun/awt/dnd/SunDragSourceContextPeer` |

### `java/awt/dnd/DragSourceContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/dnd/DragGestureEvent;Ljava/awt/Cursor;Ljava/awt/Image;Ljava/awt/Point;Ljava/awt/datatransfer/Transferable;Ljava/awt/dnd/DragSourceListener;)V` | `sun/awt/ComponentFactory` |

### `java/awt/dnd/DropTarget`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotify()V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |

### `java/awt/dnd/DropTargetContext$TransferableProxy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/dnd/DropTargetContext;Ljava/awt/datatransfer/Transferable;Z)V` | `sun/awt/datatransfer/TransferableProxy` |
| `getTransferData(Ljava/awt/datatransfer/DataFlavor;)Ljava/lang/Object;` | `sun/awt/datatransfer/TransferableProxy` |
| `getTransferDataFlavors()[Ljava/awt/datatransfer/DataFlavor;` | `sun/awt/datatransfer/TransferableProxy` |
| `isDataFlavorSupported(Ljava/awt/datatransfer/DataFlavor;)Z` | `sun/awt/datatransfer/TransferableProxy` |

### `java/awt/event/FocusEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getOppositeComponent()Ljava/awt/Component;` | `sun/awt/AppContext`, `sun/awt/SunToolkit` |
| `readResolve()Ljava/lang/Object;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$AWTEventAccessor` |

### `java/awt/event/InputEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `canAccessSystemClipboard()Z` | `sun/awt/AWTPermissions`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/awt/event/InputMethodEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getMostRecentEventTimeForSource(Ljava/lang/Object;)J` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$EventQueueAccessor`, `sun/awt/SunToolkit` |

### `java/awt/event/KeyEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getExtendedKeyCodeForChar(I)I` | `sun/awt/ExtendedKeyCodes` |

### `java/awt/event/WindowEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getOppositeWindow()Ljava/awt/Window;` | `sun/awt/AppContext`, `sun/awt/SunToolkit` |

### `java/awt/font/StyledParagraph`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/text/AttributedCharacterIterator;[C)V` | `sun/font/Decoration` |
| `addFonts([CLjava/util/Map;II)V` | `sun/font/CodePointIterator`, `sun/font/FontResolver` |
| `insertChar(Ljava/text/AttributedCharacterIterator;[CILjava/awt/font/StyledParagraph;)Ljava/awt/font/StyledParagraph;` | `sun/font/Decoration`, `sun/font/FontResolver` |

### `java/awt/font/TextLayout`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `fastInit([CLjava/awt/Font;Ljava/util/Map;Ljava/awt/font/FontRenderContext;)V` | `sun/font/CoreMetrics` |
| `getBlackBoxBounds(II)Ljava/awt/Shape;` | `sun/font/LayoutPathImpl` |
| `getCaretInfoTestInternal(Ljava/awt/font/TextHitInfo;Ljava/awt/geom/Rectangle2D;)[F` | `sun/font/CoreMetrics` |
| `getLogicalHighlightShape(IILjava/awt/geom/Rectangle2D;)Ljava/awt/Shape;` | `sun/font/LayoutPathImpl` |
| `getOutline(Ljava/awt/geom/AffineTransform;)Ljava/awt/Shape;` | `sun/font/LayoutPathImpl` |
| `getVisualHighlightShape(Ljava/awt/font/TextHitInfo;Ljava/awt/font/TextHitInfo;Ljava/awt/geom/Rectangle2D;)Ljava/awt/Shape;` | `sun/font/LayoutPathImpl` |
| `hitTestChar(FFLjava/awt/geom/Rectangle2D;)Ljava/awt/font/TextHitInfo;` | `sun/font/CoreMetrics`, `sun/font/LayoutPathImpl` |
| `paragraphInit(BLsun/font/CoreMetrics;Ljava/util/Map;[C)V` | `sun/font/AttributeValues`, `sun/font/CoreMetrics` |
| `pathToShape([DZLsun/font/LayoutPathImpl;)Ljava/awt/geom/GeneralPath;` | `sun/font/LayoutPathImpl` |
| `singleFont([CIILjava/util/Map;)Ljava/awt/Font;` | `sun/font/CodePointIterator`, `sun/font/FontResolver` |
| `standardInit(Ljava/text/AttributedCharacterIterator;[CLjava/awt/font/FontRenderContext;)V` | `sun/font/CoreMetrics`, `sun/font/GraphicComponent` |

### `java/awt/font/TextLine`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `applyFunctionAtIndex(ILjava/awt/font/TextLine$Function;)F` | `sun/font/TextLineComponent` |
| `caretAtOffsetIsValid(I)Z` | `sun/font/TextLineComponent` |
| `checkCtorArgs()V` | `sun/font/TextLineComponent` |
| `computeComponentOrder([Lsun/font/TextLineComponent;[I)[I` | `sun/font/BidiUtils`, `sun/font/TextLineComponent` |
| `createComponentsOnRun(II[C[I[BLsun/font/TextLabelFactory;Ljava/awt/Font;Lsun/font/CoreMetrics;Ljava/awt/font/FontRenderContext;Lsun/font/Decoration;[Lsun/font/TextLineComponent;I)[Lsun/font/TextLineComponent;` | `sun/font/CoreMetrics`, `sun/font/TextLabelFactory` |
| `createLineFromText([CLjava/awt/font/StyledParagraph;Lsun/font/TextLabelFactory;Z[F)Ljava/awt/font/TextLine;` | `sun/font/BidiUtils`, `sun/font/TextLabelFactory` |
| `draw(Ljava/awt/Graphics2D;FF)V` | `sun/font/LayoutPathImpl`, `sun/font/TextLineComponent` |
| `fastCreateTextLine(Ljava/awt/font/FontRenderContext;[CLjava/awt/Font;Lsun/font/CoreMetrics;Ljava/util/Map;)Ljava/awt/font/TextLine;` | `sun/font/AttributeValues`, `sun/font/BidiUtils`, `sun/font/CoreMetrics`, `sun/font/Decoration`, +1 |
| `getAdvanceBetween([Lsun/font/TextLineComponent;II)F` | `sun/font/TextLineComponent` |
| `getCharAngle(I)F` | `sun/font/CoreMetrics` |
| `getCharAscent(I)F` | `sun/font/CoreMetrics` |
| `getCharBounds(I)Ljava/awt/geom/Rectangle2D;` | `sun/font/TextLineComponent` |
| `getCharDescent(I)F` | `sun/font/CoreMetrics` |
| `getCharShift(I)F` | `sun/font/CoreMetrics` |
| `getComponentShift(I)F` | `sun/font/CoreMetrics`, `sun/font/TextLineComponent` |
| `getComponents(Ljava/awt/font/StyledParagraph;[CII[I[BLsun/font/TextLabelFactory;)[Lsun/font/TextLineComponent;` | `sun/font/GraphicComponent`, `sun/font/TextLabelFactory` |
| `getCoreMetricsAt(I)Lsun/font/CoreMetrics;` | `sun/font/TextLineComponent` |
| `getFontAtCurrentPos(Ljava/text/AttributedCharacterIterator;)Ljava/awt/Font;` | `sun/font/CodePointIterator`, `sun/font/FontResolver` |
| `getItalicBounds()Ljava/awt/geom/Rectangle2D;` | `sun/font/TextLineComponent` |
| `getJustifiedLine(FFII)Ljava/awt/font/TextLine;` | `sun/font/TextLineComponent` |
| `getOutline(Ljava/awt/geom/AffineTransform;)Ljava/awt/Shape;` | `sun/font/TextLineComponent` |
| `getPixelBounds(Ljava/awt/font/FontRenderContext;FF)Ljava/awt/Rectangle;` | `sun/font/LayoutPathImpl`, `sun/font/TextLineComponent` |
| `getVisualBounds()Ljava/awt/geom/Rectangle2D;` | `sun/font/LayoutPathImpl`, `sun/font/TextLineComponent` |
| `init()V` | `sun/font/CoreMetrics`, `sun/font/LayoutPathImpl$EmptyPath`, `sun/font/LayoutPathImpl$SegmentPathBuilder`, `sun/font/TextLineComponent` |
| `standardCreateTextLine(Ljava/awt/font/FontRenderContext;Ljava/text/AttributedCharacterIterator;[C[F)Ljava/awt/font/TextLine;` | `sun/font/TextLabelFactory` |
| `visualToLogical(I)I` | `sun/font/BidiUtils` |

### `java/awt/font/TextLine$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `computeFunction(Ljava/awt/font/TextLine;II)F` | `sun/font/TextLineComponent` |

### `java/awt/font/TextLine$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `computeFunction(Ljava/awt/font/TextLine;II)F` | `sun/font/TextLineComponent` |

### `java/awt/font/TextLine$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `computeFunction(Ljava/awt/font/TextLine;II)F` | `sun/font/TextLineComponent` |

### `java/awt/font/TextLine$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `computeFunction(Ljava/awt/font/TextLine;II)F` | `sun/font/TextLineComponent` |

### `java/awt/font/TextMeasurer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `calcLineBreak(IF)I` | `sun/font/TextLineComponent` |
| `generateComponents(II)V` | `sun/font/BidiUtils`, `sun/font/TextLabelFactory` |
| `initAll(Ljava/text/AttributedCharacterIterator;)V` | `sun/font/AttributeValues` |
| `makeComponentsOnRange(II)[Lsun/font/TextLineComponent;` | `sun/font/TextLineComponent` |
| `makeTextLineOnRange(II)Ljava/awt/font/TextLine;` | `sun/font/BidiUtils` |

### `java/awt/geom/Area`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `add(Ljava/awt/geom/Area;)V` | `sun/awt/geom/AreaOp$AddOp` |
| `contains(DD)Z` | `sun/awt/geom/Curve` |
| `contains(DDDD)Z` | `sun/awt/geom/Crossings` |
| `equals(Ljava/awt/geom/Area;)Z` | `sun/awt/geom/AreaOp$XorOp` |
| `exclusiveOr(Ljava/awt/geom/Area;)V` | `sun/awt/geom/AreaOp$XorOp` |
| `getCachedBounds()Ljava/awt/geom/Rectangle2D;` | `sun/awt/geom/Curve` |
| `intersect(Ljava/awt/geom/Area;)V` | `sun/awt/geom/AreaOp$IntOp` |
| `intersects(DDDD)Z` | `sun/awt/geom/Crossings` |
| `isPolygonal()Z` | `sun/awt/geom/Curve` |
| `isRectangular()Z` | `sun/awt/geom/Curve` |
| `isSingular()Z` | `sun/awt/geom/Curve` |
| `pathToCurves(Ljava/awt/geom/PathIterator;)Ljava/util/Vector;` | `sun/awt/geom/AreaOp`, `sun/awt/geom/AreaOp$EOWindOp`, `sun/awt/geom/AreaOp$NZWindOp`, `sun/awt/geom/Curve` |
| `subtract(Ljava/awt/geom/Area;)V` | `sun/awt/geom/AreaOp$SubOp` |

### `java/awt/geom/AreaIterator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `currentSegment([D)I` | `sun/awt/geom/Curve` |
| `next()V` | `sun/awt/geom/Curve` |

### `java/awt/geom/CubicCurve2D`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `contains(DD)Z` | `sun/awt/geom/Curve` |
| `rectCrossings(DDDD)I` | `sun/awt/geom/Curve` |

### `java/awt/geom/Path2D`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `contains(Ljava/awt/geom/PathIterator;DD)Z` | `sun/awt/geom/Curve` |
| `contains(Ljava/awt/geom/PathIterator;DDDD)Z` | `sun/awt/geom/Curve` |
| `getBounds2D(Ljava/awt/geom/PathIterator;)Ljava/awt/geom/Rectangle2D;` | `sun/awt/geom/Curve` |
| `intersects(Ljava/awt/geom/PathIterator;DDDD)Z` | `sun/awt/geom/Curve` |

### `java/awt/geom/Path2D$Double`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `pointCrossings(DD)I` | `sun/awt/geom/Curve` |
| `rectCrossings(DDDD)I` | `sun/awt/geom/Curve` |

### `java/awt/geom/Path2D$Float`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `pointCrossings(DD)I` | `sun/awt/geom/Curve` |
| `rectCrossings(DDDD)I` | `sun/awt/geom/Curve` |

### `java/awt/im/InputContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance()Ljava/awt/im/InputContext;` | `sun/awt/im/InputMethodContext` |

### `java/awt/image/AffineTransformOp`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `filter(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;)Ljava/awt/image/BufferedImage;` | `sun/awt/image/ImagingLib` |
| `filter(Ljava/awt/image/Raster;Ljava/awt/image/WritableRaster;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ImagingLib` |

### `java/awt/image/BandCombineOp`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `filter(Ljava/awt/image/Raster;Ljava/awt/image/WritableRaster;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ImagingLib` |

### `java/awt/image/BufferedImage`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/awt/image/ColorModel;Ljava/awt/image/WritableRaster;ZLjava/util/Hashtable;)V` | `sun/awt/image/ByteComponentRaster`, `sun/awt/image/IntegerComponentRaster`, `sun/awt/image/ShortComponentRaster` |
| `getSource()Ljava/awt/image/ImageProducer;` | `sun/awt/image/OffScreenImageSource` |

### `java/awt/image/ColorConvertOp`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `ICCBIFilter(Ljava/awt/image/BufferedImage;Ljava/awt/color/ColorSpace;Ljava/awt/image/BufferedImage;Ljava/awt/color/ColorSpace;)Ljava/awt/image/BufferedImage;` | `sun/java2d/cmm/ColorTransform` |
| `filter(Ljava/awt/image/Raster;Ljava/awt/image/WritableRaster;)Ljava/awt/image/WritableRaster;` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |
| `nonICCBIFilter(Ljava/awt/image/BufferedImage;Ljava/awt/color/ColorSpace;Ljava/awt/image/BufferedImage;Ljava/awt/color/ColorSpace;)Ljava/awt/image/BufferedImage;` | `sun/java2d/cmm/ColorTransform` |
| `updateBITransform(Ljava/awt/color/ICC_Profile;Ljava/awt/color/ICC_Profile;)V` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/PCMM` |

### `java/awt/image/ColorModel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getGray16TosRGB8LUT(Ljava/awt/color/ICC_ColorSpace;)[B` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |
| `getGray8TosRGB8LUT(Ljava/awt/color/ICC_ColorSpace;)[B` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |
| `getLinearGray16ToOtherGray16LUT(Ljava/awt/color/ICC_ColorSpace;)[S` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |
| `getLinearGray16ToOtherGray8LUT(Ljava/awt/color/ICC_ColorSpace;)[B` | `sun/java2d/cmm/CMSManager`, `sun/java2d/cmm/ColorTransform`, `sun/java2d/cmm/PCMM` |

### `java/awt/image/ConvolveOp`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `filter(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;)Ljava/awt/image/BufferedImage;` | `sun/awt/image/ImagingLib` |
| `filter(Ljava/awt/image/Raster;Ljava/awt/image/WritableRaster;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ImagingLib` |

### `java/awt/image/DataBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(II)V` | `sun/java2d/StateTrackable$State` |
| `<init>(III)V` | `sun/java2d/StateTrackable$State` |
| `<init>(IIII)V` | `sun/java2d/StateTrackable$State` |
| `<init>(III[I)V` | `sun/java2d/StateTrackable$State` |
| `<init>(Lsun/java2d/StateTrackable$State;II)V` | `sun/java2d/StateTrackableDelegate` |
| `<init>(Lsun/java2d/StateTrackable$State;III)V` | `sun/java2d/StateTrackableDelegate` |
| `<init>(Lsun/java2d/StateTrackable$State;IIII)V` | `sun/java2d/StateTrackableDelegate` |
| `<init>(Lsun/java2d/StateTrackable$State;III[I)V` | `sun/java2d/StateTrackableDelegate` |

### `java/awt/image/DataBufferByte`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `sun/java2d/StateTrackable$State` |
| `<init>(II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([BI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([BII)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[BI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[BI[I)V` | `sun/java2d/StateTrackable$State` |
| `getBankData()[[B` | `sun/java2d/StateTrackableDelegate` |
| `getData()[B` | `sun/java2d/StateTrackableDelegate` |
| `getData(I)[B` | `sun/java2d/StateTrackableDelegate` |
| `setElem(II)V` | `sun/java2d/StateTrackableDelegate` |
| `setElem(III)V` | `sun/java2d/StateTrackableDelegate` |

### `java/awt/image/DataBufferDouble`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `sun/java2d/StateTrackable$State` |
| `<init>(II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([DI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([DII)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[DI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[DI[I)V` | `sun/java2d/StateTrackable$State` |
| `getBankData()[[D` | `sun/java2d/StateTrackableDelegate` |
| `getData()[D` | `sun/java2d/StateTrackableDelegate` |
| `getData(I)[D` | `sun/java2d/StateTrackableDelegate` |
| `setElem(II)V` | `sun/java2d/StateTrackableDelegate` |
| `setElem(III)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemDouble(ID)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemDouble(IID)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemFloat(IF)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemFloat(IIF)V` | `sun/java2d/StateTrackableDelegate` |

### `java/awt/image/DataBufferFloat`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `sun/java2d/StateTrackable$State` |
| `<init>(II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([FI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([FII)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[FI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[FI[I)V` | `sun/java2d/StateTrackable$State` |
| `getBankData()[[F` | `sun/java2d/StateTrackableDelegate` |
| `getData()[F` | `sun/java2d/StateTrackableDelegate` |
| `getData(I)[F` | `sun/java2d/StateTrackableDelegate` |
| `setElem(II)V` | `sun/java2d/StateTrackableDelegate` |
| `setElem(III)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemDouble(ID)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemDouble(IID)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemFloat(IF)V` | `sun/java2d/StateTrackableDelegate` |
| `setElemFloat(IIF)V` | `sun/java2d/StateTrackableDelegate` |

### `java/awt/image/DataBufferInt`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `sun/java2d/StateTrackable$State` |
| `<init>(II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([III)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[II[I)V` | `sun/java2d/StateTrackable$State` |
| `getBankData()[[I` | `sun/java2d/StateTrackableDelegate` |
| `getData()[I` | `sun/java2d/StateTrackableDelegate` |
| `getData(I)[I` | `sun/java2d/StateTrackableDelegate` |
| `setElem(II)V` | `sun/java2d/StateTrackableDelegate` |
| `setElem(III)V` | `sun/java2d/StateTrackableDelegate` |

### `java/awt/image/DataBufferShort`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `sun/java2d/StateTrackable$State` |
| `<init>(II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([SI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([SII)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[SI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[SI[I)V` | `sun/java2d/StateTrackable$State` |
| `getBankData()[[S` | `sun/java2d/StateTrackableDelegate` |
| `getData()[S` | `sun/java2d/StateTrackableDelegate` |
| `getData(I)[S` | `sun/java2d/StateTrackableDelegate` |
| `setElem(II)V` | `sun/java2d/StateTrackableDelegate` |
| `setElem(III)V` | `sun/java2d/StateTrackableDelegate` |

### `java/awt/image/DataBufferUShort`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `sun/java2d/StateTrackable$State` |
| `<init>(II)V` | `sun/java2d/StateTrackable$State` |
| `<init>([SI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([SII)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[SI)V` | `sun/java2d/StateTrackable$State` |
| `<init>([[SI[I)V` | `sun/java2d/StateTrackable$State` |
| `getBankData()[[S` | `sun/java2d/StateTrackableDelegate` |
| `getData()[S` | `sun/java2d/StateTrackableDelegate` |
| `getData(I)[S` | `sun/java2d/StateTrackableDelegate` |
| `setElem(II)V` | `sun/java2d/StateTrackableDelegate` |
| `setElem(III)V` | `sun/java2d/StateTrackableDelegate` |

### `java/awt/image/LookupOp`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `filter(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;)Ljava/awt/image/BufferedImage;` | `sun/awt/image/ImagingLib` |
| `filter(Ljava/awt/image/Raster;Ljava/awt/image/WritableRaster;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ImagingLib` |

### `java/awt/image/Raster`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createBandedRaster(Ljava/awt/image/DataBuffer;III[I[ILjava/awt/Point;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ByteBandedRaster`, `sun/awt/image/ShortBandedRaster`, `sun/awt/image/SunWritableRaster` |
| `createCompatibleWritableRaster()Ljava/awt/image/WritableRaster;` | `sun/awt/image/SunWritableRaster` |
| `createCompatibleWritableRaster(II)Ljava/awt/image/WritableRaster;` | `sun/awt/image/SunWritableRaster` |
| `createInterleavedRaster(Ljava/awt/image/DataBuffer;IIII[ILjava/awt/Point;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ByteInterleavedRaster`, `sun/awt/image/ShortInterleavedRaster`, `sun/awt/image/SunWritableRaster` |
| `createPackedRaster(Ljava/awt/image/DataBuffer;IIILjava/awt/Point;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/BytePackedRaster`, `sun/awt/image/SunWritableRaster` |
| `createPackedRaster(Ljava/awt/image/DataBuffer;III[ILjava/awt/Point;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ByteInterleavedRaster`, `sun/awt/image/IntegerInterleavedRaster`, `sun/awt/image/ShortInterleavedRaster`, `sun/awt/image/SunWritableRaster` |
| `createRaster(Ljava/awt/image/SampleModel;Ljava/awt/image/DataBuffer;Ljava/awt/Point;)Ljava/awt/image/Raster;` | `sun/awt/image/ByteInterleavedRaster`, `sun/awt/image/BytePackedRaster`, `sun/awt/image/IntegerInterleavedRaster`, `sun/awt/image/ShortInterleavedRaster` |
| `createWritableRaster(Ljava/awt/image/SampleModel;Ljava/awt/image/DataBuffer;Ljava/awt/Point;)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ByteInterleavedRaster`, `sun/awt/image/BytePackedRaster`, `sun/awt/image/IntegerInterleavedRaster`, `sun/awt/image/ShortInterleavedRaster`, +1 |

### `java/awt/image/RescaleOp`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `filter(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;)Ljava/awt/image/BufferedImage;` | `sun/awt/image/ImagingLib` |
| `filterRasterImpl(Ljava/awt/image/Raster;Ljava/awt/image/WritableRaster;IZ)Ljava/awt/image/WritableRaster;` | `sun/awt/image/ImagingLib` |

### `java/awt/print/PrinterJob`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPrinterJob()Ljava/awt/print/PrinterJob;` | `sun/print/PlatformPrinterJobProxy` |

### `java/beans/Beans`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `instantiate(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/beans/beancontext/BeanContext;Ljava/beans/AppletInitializer;)Ljava/lang/Object;` | `com/sun/beans/finder/ClassFinder` |

### `java/beans/DefaultPersistenceDelegate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `initBean(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;Ljava/beans/Encoder;)V` | `sun/reflect/misc/MethodUtil`, `sun/reflect/misc/ReflectUtil` |
| `instantiate(Ljava/lang/Object;Ljava/beans/Encoder;)Ljava/beans/Expression;` | `sun/reflect/misc/MethodUtil` |

### `java/beans/Encoder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `com/sun/beans/finder/PersistenceDelegateFinder` |
| `getPersistenceDelegate(Ljava/lang/Class;)Ljava/beans/PersistenceDelegate;` | `com/sun/beans/finder/PersistenceDelegateFinder` |
| `setPersistenceDelegate(Ljava/lang/Class;Ljava/beans/PersistenceDelegate;)V` | `com/sun/beans/finder/PersistenceDelegateFinder` |

### `java/beans/EventHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `applyGetters(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;` | `sun/reflect/misc/MethodUtil` |
| `getClassLoader(Ljava/lang/Class;)Ljava/lang/ClassLoader;` | `sun/reflect/misc/ReflectUtil` |
| `invokeInternal(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;` | `sun/reflect/misc/MethodUtil` |

### `java/beans/EventSetDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Lcom/sun/beans/introspect/EventSetInfo;[Ljava/lang/reflect/Method;)V` | `com/sun/beans/introspect/EventSetInfo` |

### `java/beans/FeatureDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getParameterTypes(Ljava/lang/Class;Ljava/lang/reflect/Method;)[Ljava/lang/Class;` | `com/sun/beans/TypeResolver` |
| `getReturnType(Ljava/lang/Class;Ljava/lang/reflect/Method;)Ljava/lang/Class;` | `com/sun/beans/TypeResolver` |

### `java/beans/GenericBeanInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getTargetBeanInfo()Ljava/beans/BeanInfo;` | `com/sun/beans/finder/BeanInfoFinder` |

### `java/beans/IndexedPropertyDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/Map$Entry;Z)V` | `com/sun/beans/introspect/PropertyInfo` |

### `java/beans/Introspector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `findCustomizerClass(Ljava/lang/Class;)Ljava/lang/Class;` | `com/sun/beans/finder/ClassFinder` |
| `findExplicitBeanInfo(Ljava/lang/Class;)Ljava/beans/BeanInfo;` | `com/sun/beans/finder/BeanInfoFinder` |
| `flushCaches()V` | `com/sun/beans/introspect/ClassInfo` |
| `flushFromCaches(Ljava/lang/Class;)V` | `com/sun/beans/introspect/ClassInfo` |
| `getBeanInfo(Ljava/lang/Class;)Ljava/beans/BeanInfo;` | `sun/reflect/misc/ReflectUtil` |
| `getBeanInfoSearchPath()[Ljava/lang/String;` | `com/sun/beans/finder/BeanInfoFinder` |
| `getTargetEventInfo()[Ljava/beans/EventSetDescriptor;` | `com/sun/beans/introspect/ClassInfo`, `com/sun/beans/introspect/EventSetInfo` |
| `getTargetMethodInfo()[Ljava/beans/MethodDescriptor;` | `com/sun/beans/introspect/ClassInfo` |
| `getTargetPropertyInfo()[Ljava/beans/PropertyDescriptor;` | `com/sun/beans/introspect/ClassInfo`, `com/sun/beans/introspect/PropertyInfo` |
| `instantiate(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/beans/finder/ClassFinder` |
| `internalFindMethod(Ljava/lang/Class;Ljava/lang/String;I[Ljava/lang/Class;)Ljava/lang/reflect/Method;` | `com/sun/beans/TypeResolver`, `com/sun/beans/introspect/ClassInfo` |
| `isEventHandler(Ljava/lang/reflect/Method;)Z` | `com/sun/beans/TypeResolver` |
| `setBeanInfoSearchPath([Ljava/lang/String;)V` | `com/sun/beans/finder/BeanInfoFinder` |

### `java/beans/MetaData$StaticFieldsPersistenceDelegate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `installFields(Ljava/beans/Encoder;Ljava/lang/Class;)V` | `sun/reflect/misc/ReflectUtil` |

### `java/beans/MetaData$java_lang_Class_PersistenceDelegate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `instantiate(Ljava/lang/Object;Ljava/beans/Encoder;)Ljava/beans/Expression;` | `com/sun/beans/finder/PrimitiveWrapperMap` |

### `java/beans/MethodRef`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()Ljava/lang/reflect/Method;` | `sun/reflect/misc/ReflectUtil` |

### `java/beans/ObjectInputStreamWithLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `resolveClass(Ljava/io/ObjectStreamClass;)Ljava/lang/Class;` | `com/sun/beans/finder/ClassFinder` |

### `java/beans/PropertyDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/Map$Entry;Z)V` | `com/sun/beans/introspect/PropertyInfo`, `com/sun/beans/introspect/PropertyInfo$Name` |
| `createPropertyEditor(Ljava/lang/Object;)Ljava/beans/PropertyEditor;` | `sun/reflect/misc/ReflectUtil` |

### `java/beans/PropertyEditorManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `findEditor(Ljava/lang/Class;)Ljava/beans/PropertyEditor;` | `com/sun/beans/finder/PropertyEditorFinder` |
| `getEditorSearchPath()[Ljava/lang/String;` | `com/sun/beans/finder/PropertyEditorFinder` |
| `registerEditor(Ljava/lang/Class;Ljava/lang/Class;)V` | `com/sun/beans/finder/PropertyEditorFinder` |
| `setEditorSearchPath([Ljava/lang/String;)V` | `com/sun/beans/finder/PropertyEditorFinder` |

### `java/beans/Statement`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getMethod(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;` | `com/sun/beans/finder/MethodFinder` |
| `invokeInternal()Ljava/lang/Object;` | `com/sun/beans/finder/ClassFinder`, `com/sun/beans/finder/ConstructorFinder`, `sun/reflect/misc/MethodUtil`, `sun/reflect/misc/ReflectUtil` |

### `java/beans/ThreadGroupContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getBeanInfoFinder()Lcom/sun/beans/finder/BeanInfoFinder;` | `com/sun/beans/finder/BeanInfoFinder` |
| `getPropertyEditorFinder()Lcom/sun/beans/finder/PropertyEditorFinder;` | `com/sun/beans/finder/PropertyEditorFinder` |

### `java/beans/XMLDecoder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lorg/xml/sax/InputSource;Ljava/lang/Object;Ljava/beans/ExceptionListener;Ljava/lang/ClassLoader;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `createHandler(Ljava/lang/Object;Ljava/beans/ExceptionListener;Ljava/lang/ClassLoader;)Lorg/xml/sax/helpers/DefaultHandler;` | `com/sun/beans/decoder/DocumentHandler` |
| `getExceptionListener()Ljava/beans/ExceptionListener;` | `com/sun/beans/decoder/DocumentHandler` |
| `parsingComplete()Z` | `com/sun/beans/decoder/DocumentHandler` |
| `setExceptionListener(Ljava/beans/ExceptionListener;)V` | `com/sun/beans/decoder/DocumentHandler` |

### `java/beans/XMLDecoder$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `com/sun/beans/decoder/DocumentHandler` |

### `java/io/BufferedInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;I)V` | `jdk/internal/misc/InternalLock` |
| `available()I` | `jdk/internal/misc/InternalLock` |
| `close()V` | `jdk/internal/misc/Unsafe` |
| `fill()V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/ArraysSupport` |
| `getBufIfOpen(Z)[B` | `jdk/internal/misc/Unsafe` |
| `mark(I)V` | `jdk/internal/misc/InternalLock` |
| `read()I` | `jdk/internal/misc/InternalLock` |
| `read([BII)I` | `jdk/internal/misc/InternalLock` |
| `reset()V` | `jdk/internal/misc/InternalLock` |
| `skip(J)J` | `jdk/internal/misc/InternalLock` |
| `transferTo(Ljava/io/OutputStream;)J` | `jdk/internal/misc/InternalLock` |

### `java/io/BufferedOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/OutputStream;II)V` | `jdk/internal/misc/InternalLock` |
| `flush()V` | `jdk/internal/misc/InternalLock` |
| `initialBufferSize()I` | `jdk/internal/misc/VM` |
| `write(I)V` | `jdk/internal/misc/InternalLock` |
| `write([BII)V` | `jdk/internal/misc/InternalLock` |

### `java/io/BufferedReader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `close()V` | `jdk/internal/misc/InternalLock` |
| `mark(I)V` | `jdk/internal/misc/InternalLock` |
| `read()I` | `jdk/internal/misc/InternalLock` |
| `read([CII)I` | `jdk/internal/misc/InternalLock` |
| `readLine(Z[Z)Ljava/lang/String;` | `jdk/internal/misc/InternalLock` |
| `ready()Z` | `jdk/internal/misc/InternalLock` |
| `reset()V` | `jdk/internal/misc/InternalLock` |
| `skip(J)J` | `jdk/internal/misc/InternalLock` |

### `java/io/BufferedWriter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `close()V` | `jdk/internal/misc/InternalLock` |
| `flush()V` | `jdk/internal/misc/InternalLock` |
| `flushBuffer()V` | `jdk/internal/misc/InternalLock` |
| `initialBufferSize()I` | `jdk/internal/misc/VM` |
| `write(I)V` | `jdk/internal/misc/InternalLock` |
| `write(Ljava/lang/String;II)V` | `jdk/internal/misc/InternalLock` |
| `write([CII)V` | `jdk/internal/misc/InternalLock` |

### `java/io/ByteArrayOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `ensureCapacity(I)V` | `jdk/internal/util/ArraysSupport` |

### `java/io/Console`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `instantiateConsole(Z)Ljava/io/Console;` | `jdk/internal/io/JdkConsoleImpl` |
| `lambda$instantiateConsole$0(Ljava/lang/String;Ljdk/internal/io/JdkConsoleProvider;)Z` | `jdk/internal/io/JdkConsoleProvider` |
| `lambda$instantiateConsole$1(ZLjdk/internal/io/JdkConsoleProvider;)Ljdk/internal/io/JdkConsole;` | `jdk/internal/io/JdkConsoleProvider` |

### `java/io/DataInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readChar()C` | `jdk/internal/util/ByteArray` |
| `readDouble()D` | `jdk/internal/util/ByteArray` |
| `readFloat()F` | `jdk/internal/util/ByteArray` |
| `readInt()I` | `jdk/internal/util/ByteArray` |
| `readLong()J` | `jdk/internal/util/ByteArray` |
| `readShort()S` | `jdk/internal/util/ByteArray` |
| `readUnsignedShort()I` | `jdk/internal/util/ByteArray` |

### `java/io/DataOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `writeChar(I)V` | `jdk/internal/util/ByteArray` |
| `writeChars(Ljava/lang/String;)V` | `jdk/internal/util/ByteArray` |
| `writeDouble(D)V` | `jdk/internal/util/ByteArray` |
| `writeFloat(F)V` | `jdk/internal/util/ByteArray` |
| `writeInt(I)V` | `jdk/internal/util/ByteArray` |
| `writeLong(J)V` | `jdk/internal/util/ByteArray` |
| `writeShort(I)V` | `jdk/internal/util/ByteArray` |
| `writeUTF(Ljava/lang/String;Ljava/io/DataOutput;)I` | `jdk/internal/util/ByteArray` |

### `java/io/File`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/misc/Unsafe` |

### `java/io/FileCleanable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/FileDescriptor;Ljava/lang/ref/Cleaner;IJ)V` | `jdk/internal/ref/PhantomCleanable` |
| `register(Ljava/io/FileDescriptor;)V` | `jdk/internal/access/JavaIOFileDescriptorAccess`, `jdk/internal/ref/CleanerFactory` |

### `java/io/FileDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `registerCleanup(Ljdk/internal/ref/PhantomCleanable;)V` | `jdk/internal/ref/PhantomCleanable` |
| `set(I)V` | `jdk/internal/ref/PhantomCleanable` |
| `setHandle(J)V` | `jdk/internal/ref/PhantomCleanable` |
| `sync()V` | `jdk/internal/misc/Blocker` |
| `unregisterCleanup()V` | `jdk/internal/ref/PhantomCleanable` |

### `java/io/FileInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `available()I` | `jdk/internal/misc/Blocker` |
| `getChannel()Ljava/nio/channels/FileChannel;` | `sun/nio/ch/FileChannelImpl` |
| `length()J` | `jdk/internal/misc/Blocker` |
| `open(Ljava/lang/String;)V` | `jdk/internal/misc/Blocker` |
| `position()J` | `jdk/internal/misc/Blocker` |
| `read()I` | `jdk/internal/misc/Blocker` |
| `read([B)I` | `jdk/internal/misc/Blocker` |
| `read([BII)I` | `jdk/internal/misc/Blocker` |
| `readAllBytes()[B` | `jdk/internal/util/ArraysSupport` |
| `skip(J)J` | `jdk/internal/misc/Blocker` |

### `java/io/FileOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getChannel()Ljava/nio/channels/FileChannel;` | `sun/nio/ch/FileChannelImpl` |
| `open(Ljava/lang/String;Z)V` | `jdk/internal/misc/Blocker` |
| `write(I)V` | `jdk/internal/access/JavaIOFileDescriptorAccess`, `jdk/internal/misc/Blocker` |
| `write([B)V` | `jdk/internal/access/JavaIOFileDescriptorAccess`, `jdk/internal/misc/Blocker` |
| `write([BII)V` | `jdk/internal/access/JavaIOFileDescriptorAccess`, `jdk/internal/misc/Blocker` |

### `java/io/FilePermission`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `equals(Ljava/lang/Object;)Z` | `sun/security/util/FilePermCompat` |
| `hashCode()I` | `sun/security/util/FilePermCompat` |
| `impliesIgnoreMask(Ljava/io/FilePermission;)Z` | `sun/security/util/FilePermCompat` |
| `init(I)V` | `sun/security/util/FilePermCompat` |

### `java/io/InputStreamReader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;)V` | `sun/nio/cs/StreamDecoder` |
| `<init>(Ljava/io/InputStream;Ljava/lang/String;)V` | `sun/nio/cs/StreamDecoder` |
| `<init>(Ljava/io/InputStream;Ljava/nio/charset/Charset;)V` | `sun/nio/cs/StreamDecoder` |
| `<init>(Ljava/io/InputStream;Ljava/nio/charset/CharsetDecoder;)V` | `sun/nio/cs/StreamDecoder` |
| `close()V` | `sun/nio/cs/StreamDecoder` |
| `getEncoding()Ljava/lang/String;` | `sun/nio/cs/StreamDecoder` |
| `lockFor(Ljava/io/InputStreamReader;)Ljava/lang/Object;` | `jdk/internal/misc/InternalLock` |
| `read()I` | `sun/nio/cs/StreamDecoder` |
| `read(Ljava/nio/CharBuffer;)I` | `sun/nio/cs/StreamDecoder` |
| `read([CII)I` | `sun/nio/cs/StreamDecoder` |
| `ready()Z` | `sun/nio/cs/StreamDecoder` |

### `java/io/ObjectInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `filterCheck(Ljava/lang/Class;I)V` | `jdk/internal/event/DeserializationEvent` |
| `freeze()V` | `jdk/internal/misc/Unsafe` |
| `latestUserDefinedLoader()Ljava/lang/ClassLoader;` | `jdk/internal/misc/VM` |
| `readNonProxyDesc(Z)Ljava/io/ObjectStreamClass;` | `sun/reflect/misc/ReflectUtil` |
| `readProxyDesc(Z)Ljava/io/ObjectStreamClass;` | `sun/reflect/misc/ReflectUtil` |

### `java/io/ObjectInputStream$BlockDataInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readBlockHeader(Z)I` | `jdk/internal/util/ByteArray` |
| `readBooleans([ZII)V` | `jdk/internal/util/ByteArray` |
| `readChar()C` | `jdk/internal/util/ByteArray` |
| `readChars([CII)V` | `jdk/internal/util/ByteArray` |
| `readDouble()D` | `jdk/internal/util/ByteArray` |
| `readDoubles([DII)V` | `jdk/internal/util/ByteArray` |
| `readFloat()F` | `jdk/internal/util/ByteArray` |
| `readFloats([FII)V` | `jdk/internal/util/ByteArray` |
| `readInt()I` | `jdk/internal/util/ByteArray` |
| `readInts([III)V` | `jdk/internal/util/ByteArray` |
| `readLong()J` | `jdk/internal/util/ByteArray` |
| `readLongs([JII)V` | `jdk/internal/util/ByteArray` |
| `readShort()S` | `jdk/internal/util/ByteArray` |
| `readShorts([SII)V` | `jdk/internal/util/ByteArray` |
| `readUnsignedShort()I` | `jdk/internal/util/ByteArray` |

### `java/io/ObjectInputStream$FieldValues`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/String;C)C` | `jdk/internal/util/ByteArray` |
| `get(Ljava/lang/String;D)D` | `jdk/internal/util/ByteArray` |
| `get(Ljava/lang/String;F)F` | `jdk/internal/util/ByteArray` |
| `get(Ljava/lang/String;I)I` | `jdk/internal/util/ByteArray` |
| `get(Ljava/lang/String;J)J` | `jdk/internal/util/ByteArray` |
| `get(Ljava/lang/String;S)S` | `jdk/internal/util/ByteArray` |
| `get(Ljava/lang/String;Z)Z` | `jdk/internal/util/ByteArray` |

### `java/io/ObjectOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `writeNonProxyDesc(Ljava/io/ObjectStreamClass;Z)V` | `sun/reflect/misc/ReflectUtil` |
| `writeProxyDesc(Ljava/io/ObjectStreamClass;Z)V` | `sun/reflect/misc/ReflectUtil` |

### `java/io/ObjectOutputStream$BlockDataOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `writeBlockHeader(I)V` | `jdk/internal/util/ByteArray` |
| `writeBoolean(Z)V` | `jdk/internal/util/ByteArray` |
| `writeBooleans([ZII)V` | `jdk/internal/util/ByteArray` |
| `writeChar(I)V` | `jdk/internal/util/ByteArray` |
| `writeChars([CII)V` | `jdk/internal/util/ByteArray` |
| `writeDouble(D)V` | `jdk/internal/util/ByteArray` |
| `writeDoubles([DII)V` | `jdk/internal/util/ByteArray` |
| `writeFloat(F)V` | `jdk/internal/util/ByteArray` |
| `writeFloats([FII)V` | `jdk/internal/util/ByteArray` |
| `writeInt(I)V` | `jdk/internal/util/ByteArray` |
| `writeInts([III)V` | `jdk/internal/util/ByteArray` |
| `writeLong(J)V` | `jdk/internal/util/ByteArray` |
| `writeLongs([JII)V` | `jdk/internal/util/ByteArray` |
| `writeShort(I)V` | `jdk/internal/util/ByteArray` |
| `writeShorts([SII)V` | `jdk/internal/util/ByteArray` |

### `java/io/ObjectOutputStream$PutFieldImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `put(Ljava/lang/String;C)V` | `jdk/internal/util/ByteArray` |
| `put(Ljava/lang/String;D)V` | `jdk/internal/util/ByteArray` |
| `put(Ljava/lang/String;F)V` | `jdk/internal/util/ByteArray` |
| `put(Ljava/lang/String;I)V` | `jdk/internal/util/ByteArray` |
| `put(Ljava/lang/String;J)V` | `jdk/internal/util/ByteArray` |
| `put(Ljava/lang/String;S)V` | `jdk/internal/util/ByteArray` |
| `put(Ljava/lang/String;Z)V` | `jdk/internal/util/ByteArray` |

### `java/io/ObjectStreamClass`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `forClass()Ljava/lang/Class;` | `jdk/internal/reflect/Reflection`, `sun/reflect/misc/ReflectUtil` |
| `getSerializableConstructor(Ljava/lang/Class;)Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/ReflectionFactory` |
| `newInstance()Ljava/lang/Object;` | `jdk/internal/access/JavaSecurityAccess`, `jdk/internal/access/SharedSecrets` |

### `java/io/ObjectStreamClass$FieldReflector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>([Ljava/io/ObjectStreamField;)V` | `jdk/internal/misc/Unsafe` |
| `getObjFieldValues(Ljava/lang/Object;[Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `getPrimFieldValues(Ljava/lang/Object;[B)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/ByteArray` |
| `setObjFieldValues(Ljava/lang/Object;[Ljava/lang/Object;Z)V` | `jdk/internal/misc/Unsafe` |
| `setPrimFieldValues(Ljava/lang/Object;[B)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/ByteArray` |

### `java/io/ObjectStreamField`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getType()Ljava/lang/Class;` | `jdk/internal/reflect/Reflection`, `sun/reflect/misc/ReflectUtil` |

### `java/io/OutputStreamWriter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/OutputStream;)V` | `sun/nio/cs/StreamEncoder` |
| `<init>(Ljava/io/OutputStream;Ljava/lang/String;)V` | `sun/nio/cs/StreamEncoder` |
| `<init>(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V` | `sun/nio/cs/StreamEncoder` |
| `<init>(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V` | `sun/nio/cs/StreamEncoder` |
| `append(Ljava/lang/CharSequence;)Ljava/io/Writer;` | `sun/nio/cs/StreamEncoder` |
| `close()V` | `sun/nio/cs/StreamEncoder` |
| `flush()V` | `sun/nio/cs/StreamEncoder` |
| `flushBuffer()V` | `sun/nio/cs/StreamEncoder` |
| `getEncoding()Ljava/lang/String;` | `sun/nio/cs/StreamEncoder` |
| `lockFor(Ljava/io/OutputStreamWriter;)Ljava/lang/Object;` | `jdk/internal/misc/InternalLock` |
| `write(I)V` | `sun/nio/cs/StreamEncoder` |
| `write(Ljava/lang/String;II)V` | `sun/nio/cs/StreamEncoder` |
| `write([CII)V` | `sun/nio/cs/StreamEncoder` |

### `java/io/PrintStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V` | `jdk/internal/misc/InternalLock` |
| `<init>(ZLjava/io/OutputStream;)V` | `jdk/internal/misc/InternalLock` |
| `close()V` | `jdk/internal/misc/InternalLock` |
| `flush()V` | `jdk/internal/misc/InternalLock` |
| `format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;` | `jdk/internal/misc/InternalLock` |
| `format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;` | `jdk/internal/misc/InternalLock` |
| `newLine()V` | `jdk/internal/misc/InternalLock` |
| `write(I)V` | `jdk/internal/misc/InternalLock` |
| `write(Ljava/lang/String;)V` | `jdk/internal/misc/InternalLock` |
| `write([BII)V` | `jdk/internal/misc/InternalLock` |
| `write([C)V` | `jdk/internal/misc/InternalLock` |
| `writeln(Ljava/lang/String;)V` | `jdk/internal/misc/InternalLock` |
| `writeln([C)V` | `jdk/internal/misc/InternalLock` |

### `java/io/PrintWriter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `close()V` | `jdk/internal/misc/InternalLock` |
| `flush()V` | `jdk/internal/misc/InternalLock` |
| `format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;` | `jdk/internal/misc/InternalLock` |
| `format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;` | `jdk/internal/misc/InternalLock` |
| `newLine()V` | `jdk/internal/misc/InternalLock` |
| `println(C)V` | `jdk/internal/misc/InternalLock` |
| `println(D)V` | `jdk/internal/misc/InternalLock` |
| `println(F)V` | `jdk/internal/misc/InternalLock` |
| `println(I)V` | `jdk/internal/misc/InternalLock` |
| `println(J)V` | `jdk/internal/misc/InternalLock` |
| `println(Ljava/lang/Object;)V` | `jdk/internal/misc/InternalLock` |
| `println(Ljava/lang/String;)V` | `jdk/internal/misc/InternalLock` |
| `println(Z)V` | `jdk/internal/misc/InternalLock` |
| `println([C)V` | `jdk/internal/misc/InternalLock` |
| `write(I)V` | `jdk/internal/misc/InternalLock` |
| `write(Ljava/lang/String;II)V` | `jdk/internal/misc/InternalLock` |
| `write([CII)V` | `jdk/internal/misc/InternalLock` |

### `java/io/ProxyingConsole`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljdk/internal/io/JdkConsole;)V` | `jdk/internal/io/JdkConsole` |
| `charset()Ljava/nio/charset/Charset;` | `jdk/internal/io/JdkConsole` |
| `flush()V` | `jdk/internal/io/JdkConsole` |
| `format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/Console;` | `jdk/internal/io/JdkConsole` |
| `printf(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/Console;` | `jdk/internal/io/JdkConsole` |
| `readLine()Ljava/lang/String;` | `jdk/internal/io/JdkConsole` |
| `readLine(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;` | `jdk/internal/io/JdkConsole` |
| `readPassword()[C` | `jdk/internal/io/JdkConsole` |
| `readPassword(Ljava/lang/String;[Ljava/lang/Object;)[C` | `jdk/internal/io/JdkConsole` |

### `java/io/PushbackInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;I)V` | `jdk/internal/misc/InternalLock` |
| `close()V` | `jdk/internal/misc/InternalLock` |

### `java/io/RandomAccessFile`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getChannel()Ljava/nio/channels/FileChannel;` | `sun/nio/ch/FileChannelImpl` |
| `length()J` | `jdk/internal/misc/Blocker` |
| `open(Ljava/lang/String;I)V` | `jdk/internal/misc/Blocker` |
| `read()I` | `jdk/internal/misc/Blocker` |
| `readBytes([BII)I` | `jdk/internal/misc/Blocker` |
| `readDouble()D` | `jdk/internal/util/ByteArray` |
| `readFloat()F` | `jdk/internal/util/ByteArray` |
| `readInt()I` | `jdk/internal/util/ByteArray` |
| `readLong()J` | `jdk/internal/util/ByteArray` |
| `seek(J)V` | `jdk/internal/misc/Blocker` |
| `setLength(J)V` | `jdk/internal/misc/Blocker` |
| `write(I)V` | `jdk/internal/misc/Blocker` |
| `writeBytes([BII)V` | `jdk/internal/misc/Blocker` |
| `writeDouble(D)V` | `jdk/internal/util/ByteArray` |
| `writeFloat(F)V` | `jdk/internal/util/ByteArray` |
| `writeInt(I)V` | `jdk/internal/util/ByteArray` |
| `writeLong(J)V` | `jdk/internal/util/ByteArray` |

### `java/io/Reader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/Reader;)V` | `jdk/internal/misc/InternalLock` |
| `skip(J)J` | `jdk/internal/misc/InternalLock` |

### `java/io/UnixFileSystem`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `jdk/internal/util/StaticProperty`, `sun/security/action/GetPropertyAction` |
| `canonicalize(Ljava/lang/String;)Ljava/lang/String;` | `jdk/internal/misc/Blocker` |
| `checkAccess(Ljava/io/File;I)Z` | `jdk/internal/misc/Blocker` |
| `createDirectory(Ljava/io/File;)Z` | `jdk/internal/misc/Blocker` |
| `createFileExclusively(Ljava/lang/String;)Z` | `jdk/internal/misc/Blocker` |
| `delete(Ljava/io/File;)Z` | `jdk/internal/misc/Blocker` |
| `getBooleanAttributes(Ljava/io/File;)I` | `jdk/internal/misc/Blocker` |
| `getLastModifiedTime(Ljava/io/File;)J` | `jdk/internal/misc/Blocker` |
| `getLength(Ljava/io/File;)J` | `jdk/internal/misc/Blocker` |
| `getSpace(Ljava/io/File;I)J` | `jdk/internal/misc/Blocker` |
| `hasBooleanAttributes(Ljava/io/File;I)Z` | `jdk/internal/misc/Blocker` |
| `list(Ljava/io/File;)[Ljava/lang/String;` | `jdk/internal/misc/Blocker` |
| `rename(Ljava/io/File;Ljava/io/File;)Z` | `jdk/internal/misc/Blocker` |
| `setLastModifiedTime(Ljava/io/File;J)Z` | `jdk/internal/misc/Blocker` |
| `setPermission(Ljava/io/File;IZZ)Z` | `jdk/internal/misc/Blocker` |
| `setReadOnly(Ljava/io/File;)Z` | `jdk/internal/misc/Blocker` |

### `java/io/Writer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/Writer;)V` | `jdk/internal/misc/InternalLock` |
| `write(I)V` | `jdk/internal/misc/InternalLock` |
| `write(Ljava/lang/String;II)V` | `jdk/internal/misc/InternalLock` |

### `java/lang/AbstractStringBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `append(D)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/math/DoubleToDecimal` |
| `append(F)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/math/FloatToDecimal` |
| `append(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/util/Preconditions` |
| `append([CII)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/util/Preconditions` |
| `codePointCount(II)I` | `jdk/internal/util/Preconditions` |
| `delete(II)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/util/Preconditions` |
| `getChars(II[CI)V` | `jdk/internal/util/Preconditions` |
| `insert(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/util/Preconditions` |
| `insert(I[CII)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/util/Preconditions` |
| `newCapacity(I)I` | `jdk/internal/util/ArraysSupport` |
| `replace(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;` | `jdk/internal/util/Preconditions` |
| `substring(II)Ljava/lang/String;` | `jdk/internal/util/Preconditions` |

### `java/lang/CharacterName`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCodePoint(Ljava/lang/String;)I` | `sun/nio/cs/ISO_8859_1` |
| `hashN([BII)I` | `jdk/internal/util/ArraysSupport` |

### `java/lang/Class`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkMemberAccess(Ljava/lang/SecurityManager;ILjava/lang/Class;Z)V` | `sun/security/util/SecurityConstants` |
| `checkPackageAccess(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V` | `sun/reflect/misc/ReflectUtil` |
| `checkPackageAccessForPermittedSubclasses(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V` | `sun/reflect/misc/ReflectUtil` |
| `copyConstructors([Ljava/lang/reflect/Constructor;)[Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/ReflectionFactory` |
| `copyFields([Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;` | `jdk/internal/reflect/ReflectionFactory` |
| `copyMethods([Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;` | `jdk/internal/reflect/ReflectionFactory` |
| `createAnnotationData(I)Ljava/lang/Class$AnnotationData;` | `sun/reflect/annotation/AnnotationParser`, `sun/reflect/annotation/AnnotationType` |
| `descriptorString()Ljava/lang/String;` | `sun/invoke/util/Wrapper` |
| `forName(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `forName(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;` | `jdk/internal/loader/BootLoader`, `sun/security/util/SecurityConstants` |
| `forName(Ljava/lang/String;)Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `forName(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `forName(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;` | `sun/security/util/SecurityConstants` |
| `getAnnotatedInterfaces()[Ljava/lang/reflect/AnnotatedType;` | `sun/reflect/annotation/TypeAnnotationParser` |
| `getAnnotatedSuperclass()Ljava/lang/reflect/AnnotatedType;` | `sun/reflect/annotation/TypeAnnotationParser` |
| `getAnnotations()[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationParser` |
| `getAnnotationsByType(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationSupport` |
| `getClassLoader()Ljava/lang/ClassLoader;` | `jdk/internal/reflect/Reflection` |
| `getClasses()[Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory` |
| `getConstructor0([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/ReflectionFactory` |
| `getConstructors()[Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/Reflection` |
| `getDeclaredAnnotations()[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationParser` |
| `getDeclaredAnnotationsByType(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationSupport` |
| `getDeclaredClasses()[Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getDeclaredConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory` |
| `getDeclaredConstructors()[Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/Reflection` |
| `getDeclaredField(Ljava/lang/String;)Ljava/lang/reflect/Field;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory` |
| `getDeclaredFields()[Ljava/lang/reflect/Field;` | `jdk/internal/reflect/Reflection` |
| `getDeclaredMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory` |
| `getDeclaredMethods()[Ljava/lang/reflect/Method;` | `jdk/internal/reflect/Reflection` |
| `getDeclaredPublicMethods(Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;` | `jdk/internal/reflect/ReflectionFactory` |
| `getDeclaringClass()Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getEnclosingClass()Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getEnclosingConstructor()Ljava/lang/reflect/Constructor;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory`, `sun/reflect/generics/repository/ConstructorRepository` |
| `getEnclosingMethod()Ljava/lang/reflect/Method;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory`, `sun/reflect/generics/repository/MethodRepository` |
| `getExecutableTypeAnnotationBytes(Ljava/lang/reflect/Executable;)[B` | `jdk/internal/reflect/ReflectionFactory` |
| `getFactory()Lsun/reflect/generics/factory/GenericsFactory;` | `sun/reflect/generics/factory/CoreReflectionFactory`, `sun/reflect/generics/scope/ClassScope` |
| `getField(Ljava/lang/String;)Ljava/lang/reflect/Field;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory` |
| `getFields()[Ljava/lang/reflect/Field;` | `jdk/internal/reflect/Reflection` |
| `getGenericInfo()Lsun/reflect/generics/repository/ClassRepository;` | `sun/reflect/generics/repository/ClassRepository` |
| `getGenericInterfaces()[Ljava/lang/reflect/Type;` | `sun/reflect/generics/repository/ClassRepository` |
| `getGenericSuperclass()Ljava/lang/reflect/Type;` | `sun/reflect/generics/repository/ClassRepository` |
| `getMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;` | `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory` |
| `getMethods()[Ljava/lang/reflect/Method;` | `jdk/internal/reflect/Reflection` |
| `getNestHost()Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getNestMembers()[Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getPackage()Ljava/lang/Package;` | `jdk/internal/loader/BootLoader` |
| `getPermittedSubclasses()[Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getProtectionDomain()Ljava/security/ProtectionDomain;` | `sun/security/util/SecurityConstants` |
| `getRecordComponents()[Ljava/lang/reflect/RecordComponent;` | `jdk/internal/reflect/Reflection` |
| `getReflectionFactory()Ljdk/internal/reflect/ReflectionFactory;` | `jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction` |
| `getResource(Ljava/lang/String;)Ljava/net/URL;` | `jdk/internal/loader/BootLoader`, `jdk/internal/module/Resources`, `jdk/internal/reflect/Reflection` |
| `getResourceAsStream(Ljava/lang/String;)Ljava/io/InputStream;` | `jdk/internal/loader/BootLoader`, `jdk/internal/loader/BuiltinClassLoader`, `jdk/internal/module/Resources`, `jdk/internal/reflect/Reflection` |
| `getTypeParameters()[Ljava/lang/reflect/TypeVariable;` | `sun/reflect/generics/repository/ClassRepository` |
| `isOpenToCaller(Ljava/lang/String;Ljava/lang/Class;)Z` | `jdk/internal/module/Resources` |
| `isUnnamedClass()Z` | `jdk/internal/misc/PreviewFeatures` |
| `newInstance()Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/reflect/Reflection`, `jdk/internal/reflect/ReflectionFactory` |
| `privateGetDeclaredFields(Z)[Ljava/lang/reflect/Field;` | `jdk/internal/reflect/Reflection` |
| `privateGetDeclaredMethods(Z)[Ljava/lang/reflect/Method;` | `jdk/internal/reflect/Reflection` |
| `searchMethods([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;` | `jdk/internal/reflect/ReflectionFactory` |

### `java/lang/Class$Atomic`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casAnnotationData(Ljava/lang/Class;Ljava/lang/Class$AnnotationData;Ljava/lang/Class$AnnotationData;)Z` | `jdk/internal/misc/Unsafe` |
| `casAnnotationType(Ljava/lang/Class;Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z` | `jdk/internal/misc/Unsafe` |
| `casReflectionData(Ljava/lang/Class;Ljava/lang/ref/SoftReference;Ljava/lang/ref/SoftReference;)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/ClassLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/Void;Ljava/lang/String;Ljava/lang/ClassLoader;)V` | `jdk/internal/loader/NativeLibraries` |
| `checkClassLoaderPermission(Ljava/lang/ClassLoader;Ljava/lang/Class;)V` | `sun/security/util/SecurityConstants` |
| `checkPackageAccess(Ljava/lang/Class;Ljava/security/ProtectionDomain;)V` | `sun/reflect/misc/ReflectUtil` |
| `findNative(Ljava/lang/ClassLoader;Ljava/lang/String;)J` | `jdk/internal/loader/BootLoader`, `jdk/internal/loader/NativeLibraries` |
| `getBuiltinAppClassLoader()Ljava/lang/ClassLoader;` | `jdk/internal/loader/ClassLoaders` |
| `getBuiltinPlatformClassLoader()Ljava/lang/ClassLoader;` | `jdk/internal/loader/ClassLoaders` |
| `getPackage(Ljava/lang/String;)Ljava/lang/Package;` | `jdk/internal/loader/BootLoader` |
| `getPackages()[Ljava/lang/Package;` | `jdk/internal/loader/BootLoader` |
| `getParent()Ljava/lang/ClassLoader;` | `jdk/internal/reflect/Reflection` |
| `getPlatformClassLoader()Ljava/lang/ClassLoader;` | `jdk/internal/reflect/Reflection` |
| `getResource(Ljava/lang/String;)Ljava/net/URL;` | `jdk/internal/loader/BootLoader` |
| `getResources(Ljava/lang/String;)Ljava/util/Enumeration;` | `jdk/internal/loader/BootLoader` |
| `getSystemClassLoader()Ljava/lang/ClassLoader;` | `jdk/internal/misc/VM`, `jdk/internal/reflect/Reflection` |
| `initSystemClassLoader()Ljava/lang/ClassLoader;` | `jdk/internal/misc/VM` |
| `loadClass(Ljava/lang/String;Z)Ljava/lang/Class;` | `jdk/internal/perf/PerfCounter` |
| `loadLibrary(Ljava/lang/Class;Ljava/io/File;)Ljdk/internal/loader/NativeLibrary;` | `jdk/internal/loader/BootLoader`, `jdk/internal/loader/NativeLibraries` |
| `loadLibrary(Ljava/lang/Class;Ljava/lang/String;)Ljdk/internal/loader/NativeLibrary;` | `jdk/internal/loader/BootLoader`, `jdk/internal/loader/NativeLibraries`, `jdk/internal/util/StaticProperty` |
| `registerAsParallelCapable()Z` | `jdk/internal/reflect/Reflection` |
| `trySetObjectField(Ljava/lang/String;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/ClassValue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `initializeMap(Ljava/lang/Class;)Ljava/lang/ClassValue$ClassValueMap;` | `jdk/internal/misc/Unsafe` |

### `java/lang/ConditionalSpecialCasing`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isAfterI(Ljava/lang/String;I)Z` | `sun/text/Normalizer` |
| `isAfterSoftDotted(Ljava/lang/String;I)Z` | `sun/text/Normalizer` |
| `isBeforeDot(Ljava/lang/String;I)Z` | `sun/text/Normalizer` |
| `isMoreAbove(Ljava/lang/String;I)Z` | `sun/text/Normalizer` |

### `java/lang/Double`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `parseDouble(Ljava/lang/String;)D` | `jdk/internal/math/FloatingDecimal` |
| `toString(D)Ljava/lang/String;` | `jdk/internal/math/DoubleToDecimal` |

### `java/lang/Float`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `parseFloat(Ljava/lang/String;)F` | `jdk/internal/math/FloatingDecimal` |
| `toString(F)Ljava/lang/String;` | `jdk/internal/math/FloatToDecimal` |

### `java/lang/LiveStackFrame`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getStackWalker(Ljdk/internal/vm/Continuation;)Ljava/lang/StackWalker;` | `jdk/internal/vm/Continuation` |

### `java/lang/Module`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/ModuleLayer;Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;Ljava/net/URI;)V` | `jdk/internal/loader/ClassLoaders` |
| `addExports(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Module;` | `jdk/internal/reflect/Reflection` |
| `addOpens(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Module;` | `jdk/internal/reflect/Reflection` |
| `addReads(Ljava/lang/Module;)Ljava/lang/Module;` | `jdk/internal/reflect/Reflection` |
| `addUses(Ljava/lang/Class;)Ljava/lang/Module;` | `jdk/internal/reflect/Reflection` |
| `defineModules(Ljava/lang/module/Configuration;Ljava/util/function/Function;Ljava/lang/ModuleLayer;)Ljava/util/Map;` | `jdk/internal/loader/BootLoader`, `jdk/internal/loader/ClassLoaders`, `jdk/internal/module/ModuleLoaderMap`, `jdk/internal/module/ServicesCatalog` |
| `ensureNativeAccess(Ljava/lang/Class;Ljava/lang/String;)V` | `jdk/internal/module/ModuleBootstrap` |
| `getClassLoader()Ljava/lang/ClassLoader;` | `sun/security/util/SecurityConstants` |
| `getPackages()Ljava/util/Set;` | `jdk/internal/loader/BootLoader` |
| `getResourceAsStream(Ljava/lang/String;)Ljava/io/InputStream;` | `jdk/internal/loader/BootLoader`, `jdk/internal/loader/BuiltinClassLoader`, `jdk/internal/module/Resources`, `jdk/internal/reflect/Reflection` |
| `implAddOpensToAllUnnamed(Ljava/util/Set;Ljava/util/Set;)V` | `jdk/internal/misc/VM` |
| `lambda$loadModuleInfoClass$4(Ljdk/internal/classfile/ClassBuilder;Ljdk/internal/classfile/ClassElement;)V` | `jdk/internal/classfile/ClassBuilder` |
| `loadModuleInfoClass(Ljava/io/InputStream;)Ljava/lang/Class;` | `jdk/internal/classfile/ClassModel`, `jdk/internal/classfile/Classfile`, `jdk/internal/classfile/Classfile$Option` |

### `java/lang/Module$EnableNativeAccess`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isNativeAccessEnabled(Ljava/lang/Module;)Z` | `jdk/internal/misc/Unsafe` |
| `trySetEnableNativeAccess(Ljava/lang/Module;)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/ModuleLayer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `bindToLoader(Ljava/lang/ClassLoader;)V` | `jdk/internal/loader/ClassLoaderValue` |
| `checkCreateClassLoaderPermission()V` | `sun/security/util/SecurityConstants` |
| `checkGetClassLoaderPermission()V` | `sun/security/util/SecurityConstants` |
| `defineModulesWithManyLoaders(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer$Controller;` | `jdk/internal/loader/LoaderPool` |
| `defineModulesWithOneLoader(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer$Controller;` | `jdk/internal/loader/Loader` |
| `getServicesCatalog()Ljdk/internal/module/ServicesCatalog;` | `jdk/internal/module/ServicesCatalog` |
| `layers(Ljava/lang/ClassLoader;)Ljava/util/stream/Stream;` | `jdk/internal/loader/ClassLoaderValue` |

### `java/lang/ModuleLayer$Controller`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `enableNativeAccess(Ljava/lang/Module;)Ljava/lang/ModuleLayer$Controller;` | `jdk/internal/reflect/Reflection` |

### `java/lang/Object`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `wait(J)V` | `jdk/internal/misc/Blocker` |

### `java/lang/Package`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/net/URL;Ljava/lang/ClassLoader;)V` | `jdk/internal/loader/BootLoader` |
| `getPackage(Ljava/lang/String;)Ljava/lang/Package;` | `jdk/internal/loader/BootLoader`, `jdk/internal/reflect/Reflection` |
| `getPackageInfo()Ljava/lang/Class;` | `jdk/internal/loader/BootLoader` |
| `getPackages()[Ljava/lang/Package;` | `jdk/internal/loader/BootLoader`, `jdk/internal/reflect/Reflection` |

### `java/lang/PinnedThreadPrinter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `printStackTrace(Ljava/io/PrintStream;Ljdk/internal/vm/Continuation$Pinned;Z)V` | `jdk/internal/access/JavaIOPrintStreamAccess`, `jdk/internal/misc/InternalLock` |

### `java/lang/ProcessBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `start([Ljava/lang/ProcessBuilder$Redirect;)Ljava/lang/Process;` | `jdk/internal/event/ProcessStartEvent` |

### `java/lang/ProcessHandleImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$static$0(JLjava/lang/Runnable;)Ljava/lang/Thread;` | `jdk/internal/misc/InnocuousThread` |

### `java/lang/ProcessImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `destroy(Z)V` | `jdk/internal/util/OperatingSystem` |
| `initStreams([IZ)V` | `jdk/internal/util/OperatingSystem` |
| `launchMechanism()Ljava/lang/ProcessImpl$LaunchMechanism;` | `jdk/internal/util/OperatingSystem`, `sun/security/action/GetPropertyAction` |
| `newFileDescriptor(I)Ljava/io/FileDescriptor;` | `jdk/internal/access/JavaIOFileDescriptorAccess` |
| `start([Ljava/lang/String;Ljava/util/Map;Ljava/lang/String;[Ljava/lang/ProcessBuilder$Redirect;Z)Ljava/lang/Process;` | `jdk/internal/access/JavaIOFileDescriptorAccess` |

### `java/lang/PublicMethods$Key`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/reflect/Method;)V` | `jdk/internal/reflect/ReflectionFactory` |
| `matches(Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Z` | `jdk/internal/reflect/ReflectionFactory` |

### `java/lang/Runtime`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `load(Ljava/lang/String;)V` | `jdk/internal/reflect/Reflection` |
| `loadLibrary(Ljava/lang/String;)V` | `jdk/internal/reflect/Reflection` |
| `runFinalization()V` | `jdk/internal/access/JavaLangRefAccess`, `jdk/internal/access/SharedSecrets` |

### `java/lang/ScopedValue$Cache`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `chooseVictim()Z` | `jdk/internal/access/JavaUtilConcurrentTLRAccess` |

### `java/lang/ScopedValue$Carrier`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `runWith(Ljava/lang/ScopedValue$Snapshot;Ljava/lang/Runnable;)V` | `jdk/internal/vm/ScopedValueContainer` |
| `runWith(Ljava/lang/ScopedValue$Snapshot;Ljava/util/concurrent/Callable;)Ljava/lang/Object;` | `jdk/internal/vm/ScopedValueContainer` |

### `java/lang/SecurityManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNonExportedPackages(Ljava/lang/ModuleLayer;)V` | `jdk/internal/module/ModuleLoaderMap` |
| `checkAccess(Ljava/lang/Thread;)V` | `sun/security/util/SecurityConstants` |
| `checkAccess(Ljava/lang/ThreadGroup;)V` | `sun/security/util/SecurityConstants` |
| `checkCreateClassLoader()V` | `sun/security/util/SecurityConstants` |

### `java/lang/Shutdown`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `add(IZLjava/lang/Runnable;)V` | `jdk/internal/misc/VM` |
| `runHooks()V` | `jdk/internal/misc/VM` |

### `java/lang/StackFrameInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/StackWalker;)V` | `jdk/internal/access/JavaLangInvokeAccess` |
| `declaringClass()Ljava/lang/Class;` | `jdk/internal/access/JavaLangInvokeAccess` |
| `getContinuationScopeName()Ljava/lang/String;` | `jdk/internal/vm/ContinuationScope` |
| `getDescriptor()Ljava/lang/String;` | `jdk/internal/access/JavaLangInvokeAccess` |
| `getMethodName()Ljava/lang/String;` | `jdk/internal/access/JavaLangInvokeAccess` |
| `getMethodType()Ljava/lang/invoke/MethodType;` | `jdk/internal/access/JavaLangInvokeAccess` |
| `isNativeMethod()Z` | `jdk/internal/access/JavaLangInvokeAccess` |

### `java/lang/StackStreamFactory$AbstractStackWalker`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getNextBatch()I` | `jdk/internal/vm/Continuation` |
| `hasMoreContinuations()Z` | `jdk/internal/vm/Continuation` |
| `walk()Ljava/lang/Object;` | `jdk/internal/vm/Continuation` |

### `java/lang/StackTraceElement`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isHashedInJavaBase(Ljava/lang/Module;)Z` | `jdk/internal/misc/VM` |

### `java/lang/StackTraceElement$HashedModules`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `hashedModules()Ljava/util/Set;` | `jdk/internal/module/ModuleHashes`, `jdk/internal/module/ModuleReferenceImpl` |

### `java/lang/String`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/nio/charset/Charset;[BII)V` | `sun/nio/cs/ArrayDecoder`, `sun/nio/cs/ISO_8859_1`, `sun/nio/cs/US_ASCII`, `sun/nio/cs/UTF_8` |
| `checkBoundsBeginEnd(III)V` | `jdk/internal/util/Preconditions` |
| `checkBoundsOffCount(III)I` | `jdk/internal/util/Preconditions` |
| `checkIndex(II)V` | `jdk/internal/util/Preconditions` |
| `checkOffset(II)V` | `jdk/internal/util/Preconditions` |
| `encode(Ljava/nio/charset/Charset;B[B)[B` | `sun/nio/cs/ISO_8859_1`, `sun/nio/cs/US_ASCII`, `sun/nio/cs/UTF_8` |
| `encodeWithEncoder(Ljava/nio/charset/Charset;B[BZ)[B` | `sun/nio/cs/ArrayEncoder` |
| `getBytesNoRepl1(Ljava/lang/String;Ljava/nio/charset/Charset;)[B` | `sun/nio/cs/ISO_8859_1`, `sun/nio/cs/US_ASCII`, `sun/nio/cs/UTF_8` |
| `newStringNoRepl1([BLjava/nio/charset/Charset;)Ljava/lang/String;` | `sun/nio/cs/ArrayDecoder`, `sun/nio/cs/ISO_8859_1`, `sun/nio/cs/US_ASCII`, `sun/nio/cs/UTF_8` |
| `nonSyncContentEquals(Ljava/lang/AbstractStringBuilder;)Z` | `jdk/internal/util/ArraysSupport` |
| `regionMatches(ILjava/lang/String;II)Z` | `jdk/internal/util/ArraysSupport` |
| `startsWith(Ljava/lang/String;I)Z` | `jdk/internal/util/ArraysSupport` |

### `java/lang/StringConcatHelper`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mix(JLjdk/internal/util/FormatConcatItem;)J` | `jdk/internal/util/FormatConcatItem` |
| `newArray(J)[B` | `jdk/internal/misc/Unsafe` |
| `prepend(J[BLjdk/internal/util/FormatConcatItem;)J` | `jdk/internal/util/FormatConcatItem` |

### `java/lang/StringLatin1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareTo([B[BII)I` | `jdk/internal/util/ArraysSupport` |
| `hashCode([B)I` | `jdk/internal/util/ArraysSupport` |
| `replace([BI[BI[BI)Ljava/lang/String;` | `jdk/internal/util/ArraysSupport` |

### `java/lang/StringTemplate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `combine(Ljava/util/List;)Ljava/lang/StringTemplate;` | `jdk/internal/access/JavaTemplateAccess`, `jdk/internal/access/SharedSecrets` |
| `combine([Ljava/lang/StringTemplate;)Ljava/lang/StringTemplate;` | `jdk/internal/access/JavaTemplateAccess`, `jdk/internal/access/SharedSecrets` |
| `interpolate(Ljava/util/List;Ljava/util/List;)Ljava/lang/String;` | `jdk/internal/access/JavaTemplateAccess`, `jdk/internal/access/SharedSecrets` |
| `of(Ljava/lang/String;)Ljava/lang/StringTemplate;` | `jdk/internal/access/JavaTemplateAccess`, `jdk/internal/access/SharedSecrets` |
| `of(Ljava/util/List;Ljava/util/List;)Ljava/lang/StringTemplate;` | `jdk/internal/access/JavaTemplateAccess`, `jdk/internal/access/SharedSecrets` |

### `java/lang/StringUTF16`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `hashCode([B)I` | `jdk/internal/util/ArraysSupport` |
| `replace([BIZ[BIZ[BIZ)Ljava/lang/String;` | `jdk/internal/util/ArraysSupport` |

### `java/lang/System`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `console()Ljava/io/Console;` | `jdk/internal/access/JavaIOAccess`, `jdk/internal/access/SharedSecrets` |
| `getLogger(Ljava/lang/String;)Ljava/lang/System$Logger;` | `jdk/internal/logger/LazyLoggers`, `jdk/internal/reflect/Reflection` |
| `getLogger(Ljava/lang/String;Ljava/util/ResourceBundle;)Ljava/lang/System$Logger;` | `jdk/internal/reflect/Reflection` |
| `implSetSecurityManager(Ljava/lang/SecurityManager;)V` | `sun/nio/fs/DefaultFileSystemProvider` |
| `initPhase1()V` | `jdk/internal/access/JavaLangRefAccess`, `jdk/internal/access/SharedSecrets`, `jdk/internal/misc/VM`, `jdk/internal/util/StaticProperty`, +1 |
| `initPhase2(ZZ)I` | `jdk/internal/misc/VM`, `jdk/internal/module/ModuleBootstrap` |
| `initPhase3()V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VM`, `jdk/internal/util/SystemProps` |
| `load(Ljava/lang/String;)V` | `jdk/internal/reflect/Reflection` |
| `loadLibrary(Ljava/lang/String;)V` | `jdk/internal/reflect/Reflection` |
| `logInitException(ZZLjava/lang/String;Ljava/lang/Throwable;)V` | `jdk/internal/misc/VM` |
| `newPrintStream(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;` | `sun/nio/cs/UTF_8` |
| `setJavaLangAccess()V` | `jdk/internal/access/SharedSecrets` |
| `setProperties(Ljava/util/Properties;)V` | `jdk/internal/util/SystemProps` |
| `setSecurityManager(Ljava/lang/SecurityManager;)V` | `jdk/internal/reflect/Reflection` |

### `java/lang/System$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Object;` | `sun/security/util/SecurityConstants` |

### `java/lang/System$LoggerFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getLocalizedLogger(Ljava/lang/String;Ljava/util/ResourceBundle;Ljava/lang/Module;)Ljava/lang/System$Logger;` | `jdk/internal/logger/LocalizedLoggerWrapper` |
| `lambda$accessProvider$0()Ljava/lang/System$LoggerFinder;` | `jdk/internal/logger/LoggerFinderLoader` |

### `java/lang/Terminator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setup()V` | `jdk/internal/misc/Signal` |

### `java/lang/Terminator$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `handle(Ljdk/internal/misc/Signal;)V` | `jdk/internal/misc/Signal` |

### `java/lang/Thread`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/ThreadGroup;Ljava/lang/String;ILjava/lang/Runnable;JLjava/security/AccessControlContext;)V` | `jdk/internal/misc/VM`, `sun/security/util/SecurityConstants` |
| `afterSleep(Ljdk/internal/event/ThreadSleepEvent;)V` | `jdk/internal/event/ThreadSleepEvent` |
| `beforeSleep(J)Ljdk/internal/event/ThreadSleepEvent;` | `jdk/internal/event/ThreadSleepEvent` |
| `exit()V` | `jdk/internal/misc/CarrierThreadLocal`, `jdk/internal/misc/TerminatingThreadLocal`, `jdk/internal/vm/StackableScope`, `jdk/internal/vm/ThreadContainer` |
| `getAllStackTraces()Ljava/util/Map;` | `sun/security/util/SecurityConstants` |
| `getContextClassLoader()Ljava/lang/ClassLoader;` | `jdk/internal/reflect/Reflection` |
| `getStackTrace()[Ljava/lang/StackTraceElement;` | `sun/security/util/SecurityConstants` |
| `inheritScopedValueBindings(Ljdk/internal/vm/ThreadContainer;)V` | `jdk/internal/vm/ScopedValueContainer$BindingsSnapshot`, `jdk/internal/vm/ThreadContainer` |
| `interrupt()V` | `sun/nio/ch/Interruptible` |
| `start(Ljdk/internal/vm/ThreadContainer;)V` | `jdk/internal/vm/ThreadContainer` |
| `threadState()Ljava/lang/Thread$State;` | `jdk/internal/misc/VM` |

### `java/lang/Thread$ThreadIdentifiers`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `next()J` | `jdk/internal/misc/Unsafe` |

### `java/lang/Thread$ThreadNumbering`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `next()I` | `jdk/internal/misc/Unsafe` |

### `java/lang/ThreadBuilders`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newVirtualThread(Ljava/util/concurrent/Executor;Ljava/lang/String;ILjava/lang/Runnable;)Ljava/lang/Thread;` | `jdk/internal/vm/ContinuationSupport` |

### `java/lang/ThreadBuilders$BoundVirtualThread`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `park()V` | `jdk/internal/misc/Unsafe` |
| `parkNanos(J)V` | `jdk/internal/misc/Unsafe` |
| `unpark()V` | `jdk/internal/misc/Unsafe` |

### `java/lang/ThreadBuilders$VirtualThreadBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/concurrent/Executor;)V` | `jdk/internal/vm/ContinuationSupport` |

### `java/lang/ThreadGroup`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/ThreadGroup;Ljava/lang/String;IZ)V` | `jdk/internal/misc/VM` |

### `java/lang/ThreadLocal`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setInitialValue(Ljava/lang/Thread;)Ljava/lang/Object;` | `jdk/internal/misc/TerminatingThreadLocal` |
| `traceVirtualThreadLocals()Z` | `sun/security/action/GetPropertyAction` |

### `java/lang/Throwable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `printStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V` | `jdk/internal/misc/InternalLock` |

### `java/lang/Throwable$PrintStreamOrWriter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isLockedByCurrentThread()Z` | `jdk/internal/misc/InternalLock` |

### `java/lang/Throwable$WrappedPrintStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lock()Ljava/lang/Object;` | `jdk/internal/access/JavaIOPrintStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/lang/Throwable$WrappedPrintWriter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lock()Ljava/lang/Object;` | `jdk/internal/access/JavaIOPrintWriterAccess`, `jdk/internal/access/SharedSecrets` |

### `java/lang/VirtualThread`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `afterDone(Z)V` | `jdk/internal/vm/ThreadContainer` |
| `afterYield()V` | `jdk/internal/misc/CarrierThread` |
| `compareAndSetState(II)Z` | `jdk/internal/misc/Unsafe` |
| `createDelayedTaskScheduler()Ljava/util/concurrent/ScheduledExecutorService;` | `sun/security/action/GetPropertyAction` |
| `getAndSetParkPermit(Z)Z` | `jdk/internal/misc/Unsafe` |
| `getTermination()Ljava/util/concurrent/CountDownLatch;` | `jdk/internal/misc/Unsafe` |
| `interrupt()V` | `sun/nio/ch/Interruptible` |
| `lambda$createDefaultScheduler$0(Ljava/util/concurrent/ForkJoinPool;)Ljava/util/concurrent/ForkJoinWorkerThread;` | `jdk/internal/misc/CarrierThread` |
| `lambda$createDelayedTaskScheduler$5(Ljava/lang/Runnable;)Ljava/lang/Thread;` | `jdk/internal/misc/InnocuousThread` |
| `parkOnCarrierThread(ZJ)V` | `jdk/internal/event/VirtualThreadPinnedEvent`, `jdk/internal/misc/Unsafe` |
| `run(Ljava/lang/Runnable;)V` | `jdk/internal/event/VirtualThreadEndEvent`, `jdk/internal/event/VirtualThreadStartEvent`, `jdk/internal/vm/StackableScope` |
| `runContinuation()V` | `jdk/internal/vm/Continuation` |
| `start()V` | `jdk/internal/vm/ThreadContainers` |
| `start(Ljdk/internal/vm/ThreadContainer;)V` | `jdk/internal/vm/ThreadContainer` |
| `submitFailed(Ljava/util/concurrent/RejectedExecutionException;)V` | `jdk/internal/event/VirtualThreadSubmitFailedEvent` |
| `tracePinningMode()I` | `sun/security/action/GetPropertyAction` |
| `tryGetStackTrace()[Ljava/lang/StackTraceElement;` | `jdk/internal/vm/Continuation` |
| `unpark()V` | `jdk/internal/misc/Unsafe` |
| `yieldContinuation()Z` | `jdk/internal/vm/Continuation` |

### `java/lang/VirtualThread$VThreadContinuation`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/VirtualThread;Ljava/lang/Runnable;)V` | `jdk/internal/vm/Continuation` |

### `java/lang/constant/ClassDesc`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `displayName()Ljava/lang/String;` | `sun/invoke/util/Wrapper` |

### `java/lang/constant/PrimitiveClassDescImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Class;` | `sun/invoke/util/Wrapper` |

### `java/lang/foreign/Arena`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `allocate(JJ)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/MemorySessionImpl` |
| `ofAuto()Ljava/lang/foreign/Arena;` | `jdk/internal/foreign/MemorySessionImpl`, `jdk/internal/ref/CleanerFactory` |
| `ofConfined()Ljava/lang/foreign/Arena;` | `jdk/internal/foreign/MemorySessionImpl` |
| `ofShared()Ljava/lang/foreign/Arena;` | `jdk/internal/foreign/MemorySessionImpl` |

### `java/lang/foreign/FunctionDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `of(Ljava/lang/foreign/MemoryLayout;[Ljava/lang/foreign/MemoryLayout;)Ljava/lang/foreign/FunctionDescriptor;` | `jdk/internal/foreign/FunctionDescriptorImpl` |
| `ofVoid([Ljava/lang/foreign/MemoryLayout;)Ljava/lang/foreign/FunctionDescriptor;` | `jdk/internal/foreign/FunctionDescriptorImpl` |

### `java/lang/foreign/Linker`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nativeLinker()Ljava/lang/foreign/Linker;` | `jdk/internal/foreign/abi/SharedUtils` |

### `java/lang/foreign/Linker$Option`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `captureCallState([Ljava/lang/String;)Ljava/lang/foreign/Linker$Option;` | `jdk/internal/foreign/abi/LinkerOptions$CaptureCallState` |
| `captureStateLayout()Ljava/lang/foreign/StructLayout;` | `jdk/internal/foreign/abi/CapturableState` |
| `firstVariadicArg(I)Ljava/lang/foreign/Linker$Option;` | `jdk/internal/foreign/abi/LinkerOptions$FirstVariadicArg` |
| `isTrivial()Ljava/lang/foreign/Linker$Option;` | `jdk/internal/foreign/abi/LinkerOptions$IsTrivial` |

### `java/lang/foreign/MemoryLayout`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `byteOffset([Ljava/lang/foreign/MemoryLayout$PathElement;)J` | `jdk/internal/foreign/LayoutPath`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `byteOffsetHandle([Ljava/lang/foreign/MemoryLayout$PathElement;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/foreign/LayoutPath`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `computePathOp(Ljdk/internal/foreign/LayoutPath;Ljava/util/function/Function;Ljava/util/Set;[Ljava/lang/foreign/MemoryLayout$PathElement;)Ljava/lang/Object;` | `jdk/internal/foreign/LayoutPath$PathElementImpl`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `lambda$sequenceLayout$0(JLjava/lang/foreign/MemoryLayout;)Ljava/lang/foreign/SequenceLayout;` | `jdk/internal/foreign/layout/SequenceLayoutImpl` |
| `lambda$structLayout$1([Ljava/lang/foreign/MemoryLayout;)Ljava/lang/foreign/StructLayout;` | `jdk/internal/foreign/layout/StructLayoutImpl` |
| `paddingLayout(J)Ljava/lang/foreign/PaddingLayout;` | `jdk/internal/foreign/layout/MemoryLayoutUtil`, `jdk/internal/foreign/layout/PaddingLayoutImpl` |
| `select([Ljava/lang/foreign/MemoryLayout$PathElement;)Ljava/lang/foreign/MemoryLayout;` | `jdk/internal/foreign/LayoutPath`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `sequenceLayout(JLjava/lang/foreign/MemoryLayout;)Ljava/lang/foreign/SequenceLayout;` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/MemoryLayoutUtil` |
| `sliceHandle([Ljava/lang/foreign/MemoryLayout$PathElement;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/foreign/LayoutPath`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `structLayout([Ljava/lang/foreign/MemoryLayout;)Ljava/lang/foreign/StructLayout;` | `jdk/internal/foreign/Utils` |
| `unionLayout([Ljava/lang/foreign/MemoryLayout;)Ljava/lang/foreign/UnionLayout;` | `jdk/internal/foreign/layout/UnionLayoutImpl` |
| `varHandle([Ljava/lang/foreign/MemoryLayout$PathElement;)Ljava/lang/invoke/VarHandle;` | `jdk/internal/foreign/LayoutPath` |

### `java/lang/foreign/MemoryLayout$PathElement`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `dereferenceElement()Ljava/lang/foreign/MemoryLayout$PathElement;` | `jdk/internal/foreign/LayoutPath$PathElementImpl`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `groupElement(J)Ljava/lang/foreign/MemoryLayout$PathElement;` | `jdk/internal/foreign/LayoutPath$PathElementImpl`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `groupElement(Ljava/lang/String;)Ljava/lang/foreign/MemoryLayout$PathElement;` | `jdk/internal/foreign/LayoutPath$PathElementImpl`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `lambda$groupElement$0(Ljava/lang/String;Ljdk/internal/foreign/LayoutPath;)Ljdk/internal/foreign/LayoutPath;` | `jdk/internal/foreign/LayoutPath` |
| `lambda$groupElement$1(JLjdk/internal/foreign/LayoutPath;)Ljdk/internal/foreign/LayoutPath;` | `jdk/internal/foreign/LayoutPath` |
| `lambda$sequenceElement$2(JLjdk/internal/foreign/LayoutPath;)Ljdk/internal/foreign/LayoutPath;` | `jdk/internal/foreign/LayoutPath` |
| `lambda$sequenceElement$3(JJLjdk/internal/foreign/LayoutPath;)Ljdk/internal/foreign/LayoutPath;` | `jdk/internal/foreign/LayoutPath` |
| `sequenceElement()Ljava/lang/foreign/MemoryLayout$PathElement;` | `jdk/internal/foreign/LayoutPath$PathElementImpl`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `sequenceElement(J)Ljava/lang/foreign/MemoryLayout$PathElement;` | `jdk/internal/foreign/LayoutPath$PathElementImpl`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |
| `sequenceElement(JJ)Ljava/lang/foreign/MemoryLayout$PathElement;` | `jdk/internal/foreign/LayoutPath$PathElementImpl`, `jdk/internal/foreign/LayoutPath$PathElementImpl$PathKind` |

### `java/lang/foreign/MemorySegment`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `copy(Ljava/lang/Object;ILjava/lang/foreign/MemorySegment;Ljava/lang/foreign/ValueLayout;JI)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `copy(Ljava/lang/foreign/MemorySegment;Ljava/lang/foreign/ValueLayout;JLjava/lang/Object;II)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `copy(Ljava/lang/foreign/MemorySegment;Ljava/lang/foreign/ValueLayout;JLjava/lang/foreign/MemorySegment;Ljava/lang/foreign/ValueLayout;JJ)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `get(Ljava/lang/foreign/AddressLayout;J)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/layout/ValueLayouts$OfAddressImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfBoolean;J)Z` | `jdk/internal/foreign/layout/ValueLayouts$OfBooleanImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfByte;J)B` | `jdk/internal/foreign/layout/ValueLayouts$OfByteImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfChar;J)C` | `jdk/internal/foreign/layout/ValueLayouts$OfCharImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfDouble;J)D` | `jdk/internal/foreign/layout/ValueLayouts$OfDoubleImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfFloat;J)F` | `jdk/internal/foreign/layout/ValueLayouts$OfFloatImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfInt;J)I` | `jdk/internal/foreign/layout/ValueLayouts$OfIntImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfLong;J)J` | `jdk/internal/foreign/layout/ValueLayouts$OfLongImpl` |
| `get(Ljava/lang/foreign/ValueLayout$OfShort;J)S` | `jdk/internal/foreign/layout/ValueLayouts$OfShortImpl` |
| `getAtIndex(Ljava/lang/foreign/AddressLayout;J)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfAddressImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfBoolean;J)Z` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfBooleanImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfByte;J)B` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfByteImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfChar;J)C` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfCharImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfDouble;J)D` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfDoubleImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfFloat;J)F` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfFloatImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfInt;J)I` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfIntImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfLong;J)J` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfLongImpl` |
| `getAtIndex(Ljava/lang/foreign/ValueLayout$OfShort;J)S` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfShortImpl` |
| `getUtf8String(J)Ljava/lang/String;` | `jdk/internal/foreign/abi/SharedUtils` |
| `mismatch(Ljava/lang/foreign/MemorySegment;JJLjava/lang/foreign/MemorySegment;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `ofAddress(J)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/NativeMemorySegmentImpl` |
| `ofArray([B)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/HeapMemorySegmentImpl$OfByte` |
| `ofArray([C)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/HeapMemorySegmentImpl$OfChar` |
| `ofArray([D)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/HeapMemorySegmentImpl$OfDouble` |
| `ofArray([F)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/HeapMemorySegmentImpl$OfFloat` |
| `ofArray([I)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/HeapMemorySegmentImpl$OfInt` |
| `ofArray([J)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/HeapMemorySegmentImpl$OfLong` |
| `ofArray([S)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/HeapMemorySegmentImpl$OfShort` |
| `ofBuffer(Ljava/nio/Buffer;)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/foreign/AddressLayout;JLjava/lang/foreign/MemorySegment;)V` | `jdk/internal/foreign/layout/ValueLayouts$OfAddressImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfBoolean;JZ)V` | `jdk/internal/foreign/layout/ValueLayouts$OfBooleanImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfByte;JB)V` | `jdk/internal/foreign/layout/ValueLayouts$OfByteImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfChar;JC)V` | `jdk/internal/foreign/layout/ValueLayouts$OfCharImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfDouble;JD)V` | `jdk/internal/foreign/layout/ValueLayouts$OfDoubleImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfFloat;JF)V` | `jdk/internal/foreign/layout/ValueLayouts$OfFloatImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfInt;JI)V` | `jdk/internal/foreign/layout/ValueLayouts$OfIntImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfLong;JJ)V` | `jdk/internal/foreign/layout/ValueLayouts$OfLongImpl` |
| `set(Ljava/lang/foreign/ValueLayout$OfShort;JS)V` | `jdk/internal/foreign/layout/ValueLayouts$OfShortImpl` |
| `setAtIndex(Ljava/lang/foreign/AddressLayout;JLjava/lang/foreign/MemorySegment;)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfAddressImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfBoolean;JZ)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfBooleanImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfByte;JB)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfByteImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfChar;JC)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfCharImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfDouble;JD)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfDoubleImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfFloat;JF)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfFloatImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfInt;JI)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfIntImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfLong;JJ)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfLongImpl` |
| `setAtIndex(Ljava/lang/foreign/ValueLayout$OfShort;JS)V` | `jdk/internal/foreign/Utils`, `jdk/internal/foreign/layout/ValueLayouts$OfShortImpl` |
| `setUtf8String(JLjava/lang/String;)V` | `jdk/internal/foreign/Utils` |

### `java/lang/foreign/SegmentAllocator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `allocateUtf8String(Ljava/lang/String;)Ljava/lang/foreign/MemorySegment;` | `jdk/internal/foreign/Utils` |
| `slicingAllocator(Ljava/lang/foreign/MemorySegment;)Ljava/lang/foreign/SegmentAllocator;` | `jdk/internal/foreign/SlicingAllocator` |

### `java/lang/foreign/SymbolLookup`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$libraryLookup$3(Ljdk/internal/loader/NativeLibrary;Ljava/lang/foreign/Arena;Ljava/lang/String;)Ljava/util/Optional;` | `jdk/internal/foreign/Utils`, `jdk/internal/loader/NativeLibrary` |
| `lambda$loaderLookup$2(Ljava/lang/ClassLoader;Ljava/lang/foreign/Arena;Ljava/lang/String;)Ljava/util/Optional;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `jdk/internal/foreign/Utils` |
| `libraryLookup(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/lang/foreign/Arena;)Ljava/lang/foreign/SymbolLookup;` | `jdk/internal/foreign/MemorySessionImpl`, `jdk/internal/loader/RawNativeLibraries` |
| `libraryLookup(Ljava/lang/String;Ljava/lang/foreign/Arena;)Ljava/lang/foreign/SymbolLookup;` | `jdk/internal/foreign/Utils`, `jdk/internal/reflect/Reflection` |
| `libraryLookup(Ljava/nio/file/Path;Ljava/lang/foreign/Arena;)Ljava/lang/foreign/SymbolLookup;` | `jdk/internal/reflect/Reflection` |
| `loaderLookup()Ljava/lang/foreign/SymbolLookup;` | `jdk/internal/foreign/MemorySessionImpl`, `jdk/internal/reflect/Reflection` |

### `java/lang/foreign/SymbolLookup$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljdk/internal/loader/RawNativeLibraries;Ljdk/internal/loader/NativeLibrary;)V` | `jdk/internal/foreign/MemorySessionImpl$ResourceList$ResourceCleanup` |
| `cleanup()V` | `jdk/internal/loader/RawNativeLibraries` |

### `java/lang/invoke/AbstractValidatingLambdaMetafactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isAdaptableTo(Ljava/lang/Class;Ljava/lang/Class;Z)Z` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/BootstrapMethodInvoker`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `widenAndCast(Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/CallSite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;)V` | `jdk/internal/misc/Unsafe` |
| `getTargetOffset()J` | `jdk/internal/misc/Unsafe` |
| `getTargetVolatile()Ljava/lang/invoke/MethodHandle;` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/ClassSpecializer$Factory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `emitIntConstant(ILjdk/internal/org/objectweb/asm/MethodVisitor;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `generateConcreteSpeciesCodeFile(Ljava/lang/String;Ljava/lang/invoke/ClassSpecializer$SpeciesData;)[B` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/FieldVisitor`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `linkCodeToSpeciesData(Ljava/lang/Class;Ljava/lang/invoke/ClassSpecializer$SpeciesData;Z)V` | `jdk/internal/misc/Unsafe` |
| `loadSpecies(Ljava/lang/invoke/ClassSpecializer$SpeciesData;)Ljava/lang/invoke/ClassSpecializer$SpeciesData;` | `jdk/internal/loader/BootLoader` |
| `readSpeciesDataFromCode(Ljava/lang/Class;)Ljava/lang/invoke/ClassSpecializer$SpeciesData;` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/ClassSpecializer$Factory$1Var`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `emitFieldInsn(ILjdk/internal/org/objectweb/asm/MethodVisitor;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitVarInstruction(ILjdk/internal/org/objectweb/asm/MethodVisitor;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |

### `java/lang/invoke/ClassSpecializer$SpeciesData`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `deriveTypeString()Ljava/lang/String;` | `sun/invoke/util/BytecodeName` |

### `java/lang/invoke/ConstantBootstraps`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getStaticFinal(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;` | `sun/invoke/util/Wrapper` |
| `primitiveClass(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/ConstantCallSite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/invoke/MethodHandle;)V` | `jdk/internal/misc/Unsafe` |
| `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;)V` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/DirectMethodHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `allocateInstance(Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `checkInitialized(Ljava/lang/invoke/MemberName;)Z` | `jdk/internal/misc/Unsafe` |
| `ftypeKind(Ljava/lang/Class;)I` | `sun/invoke/util/Wrapper` |
| `getFieldKind(ZZLsun/invoke/util/Wrapper;)Ljava/lang/invoke/LambdaForm$Kind;` | `sun/invoke/util/Wrapper` |
| `makePreparedFieldLambdaForm(BZI)Ljava/lang/invoke/LambdaForm;` | `sun/invoke/util/Wrapper` |
| `maybeCompile(Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;)V` | `sun/invoke/util/VerifyAccess` |
| `shouldBeInitialized(Ljava/lang/invoke/MemberName;)Z` | `jdk/internal/misc/Unsafe`, `sun/invoke/util/VerifyAccess` |

### `java/lang/invoke/GenerateJLIClassesHelper`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `generateCodeBytesForLFs(Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/invoke/LambdaForm;)[B` | `jdk/internal/org/objectweb/asm/ClassWriter` |
| `generateDirectMethodHandleHolderClassBytes(Ljava/lang/String;[Ljava/lang/invoke/MethodType;[I)[B` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/InnerClassLambdaMetafactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/invoke/MethodType;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;Z[Ljava/lang/Class;[Ljava/lang/invoke/MethodType;)V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `sun/invoke/util/BytecodeDescriptor`, `sun/invoke/util/VerifyAccess` |
| `generateClassInitializer()V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/FieldVisitor`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `generateConstructor()V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `generateInnerClass()Ljava/lang/Class;` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/FieldVisitor` |
| `generateSerializationFriendlyMethods()V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/Type` |
| `generateSerializationHostileMethods()V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `spinInnerClass()Ljava/lang/Class;` | `jdk/internal/misc/CDS` |

### `java/lang/invoke/InvokerBytecodeGenerator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/invoke/MethodType;)V` | `sun/invoke/util/Wrapper` |
| `<init>(Ljava/lang/invoke/LambdaForm;ILjava/lang/String;Ljava/lang/String;Ljava/lang/invoke/MethodType;)V` | `jdk/internal/util/ClassFileDumper` |
| `addMethod()V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `arrayTypeCode(Lsun/invoke/util/Wrapper;)B` | `sun/invoke/util/Wrapper` |
| `bogusMethod(Ljava/lang/Object;)V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/MethodVisitor`, `jdk/internal/util/ClassFileDumper` |
| `checkActualReceiver()Z` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `checkClassName(Ljava/lang/String;)Z` | `jdk/internal/org/objectweb/asm/Type` |
| `classData(Ljava/lang/Object;)Ljava/lang/String;` | `jdk/internal/util/ClassFileDumper` |
| `classFilePrologue()Ljdk/internal/org/objectweb/asm/ClassWriter;` | `jdk/internal/org/objectweb/asm/ClassWriter` |
| `clinit(Ljdk/internal/org/objectweb/asm/ClassWriter;Ljava/lang/String;Ljava/util/List;)V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/FieldVisitor`, `jdk/internal/org/objectweb/asm/MethodVisitor`, `jdk/internal/org/objectweb/asm/Type` |
| `emitArrayOp(Ljava/lang/invoke/LambdaForm$Name;I)V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `emitBoxing(Lsun/invoke/util/Wrapper;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `emitConst(Ljava/lang/Object;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitGuardWithCatch(I)Ljava/lang/invoke/LambdaForm$Name;` | `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitI2X(Lsun/invoke/util/Wrapper;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `emitIconstInsn(Ljdk/internal/org/objectweb/asm/MethodVisitor;I)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitImplicitConversion(Ljava/lang/invoke/LambdaForm$BasicType;Ljava/lang/Class;Ljava/lang/Object;)V` | `sun/invoke/util/VerifyType`, `sun/invoke/util/Wrapper` |
| `emitInvoke(Ljava/lang/invoke/LambdaForm$Name;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitLoadInsn(Ljava/lang/invoke/LambdaForm$BasicType;I)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitLoop(I)Ljava/lang/invoke/LambdaForm$Name;` | `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitLoopHandleInvoke(Ljava/lang/invoke/LambdaForm$Name;IILjava/lang/invoke/LambdaForm$Name;ZLjava/lang/invoke/MethodType;[Ljava/lang/Class;II)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitPopInsn(Ljava/lang/invoke/LambdaForm$BasicType;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitPrimCast(Lsun/invoke/util/Wrapper;Lsun/invoke/util/Wrapper;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `emitPushArgument(Ljava/lang/Class;Ljava/lang/Object;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `emitPushClauseArray(II)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitReferenceCast(Ljava/lang/Class;Ljava/lang/Object;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitReturn(Ljava/lang/invoke/LambdaForm$Name;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitReturnInsn(Ljava/lang/invoke/LambdaForm$BasicType;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitSelectAlternative(Ljava/lang/invoke/LambdaForm$Name;Ljava/lang/invoke/LambdaForm$Name;)Ljava/lang/invoke/LambdaForm$Name;` | `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitStaticInvoke(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/LambdaForm$Name;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitStoreInsn(Ljava/lang/invoke/LambdaForm$BasicType;I)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitTableSwitch(II)Ljava/lang/invoke/LambdaForm$Name;` | `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitTryFinally(I)Ljava/lang/invoke/LambdaForm$Name;` | `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitUnboxing(Lsun/invoke/util/Wrapper;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `emitX2I(Lsun/invoke/util/Wrapper;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `emitZero(Ljava/lang/invoke/LambdaForm$BasicType;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `generateLambdaFormInterpreterEntryPointBytes()[B` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `generateNamedFunctionInvokerImpl(Ljava/lang/invoke/MethodTypeForm;)[B` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/MethodVisitor`, `sun/invoke/util/Wrapper` |
| `getInternalName(Ljava/lang/Class;)Ljava/lang/String;` | `sun/invoke/util/VerifyAccess` |
| `isStaticallyInvocable(Ljava/lang/invoke/MemberName;)Z` | `sun/invoke/util/VerifyAccess` |
| `isStaticallyNameable(Ljava/lang/Class;)Z` | `sun/invoke/util/VerifyAccess` |
| `methodEpilogue()V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `methodPrologue()V` | `jdk/internal/org/objectweb/asm/ClassWriter` |
| `resolveFrom(Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)Ljava/lang/invoke/MemberName;` | `jdk/internal/misc/Unsafe` |
| `toByteArray()[B` | `jdk/internal/org/objectweb/asm/ClassWriter` |

### `java/lang/invoke/LambdaForm`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkInt(Ljava/lang/Class;Ljava/lang/Object;)Z` | `sun/invoke/util/Wrapper` |
| `compileToBytecode()V` | `jdk/internal/perf/PerfCounter` |
| `createFormsFor(Ljava/lang/invoke/LambdaForm$BasicType;)V` | `jdk/internal/misc/Unsafe`, `sun/invoke/util/Wrapper` |
| `failedCompilationCounter()Ljdk/internal/perf/PerfCounter;` | `jdk/internal/perf/PerfCounter` |

### `java/lang/invoke/LambdaForm$BasicType`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `basicType(Ljava/lang/Class;)Ljava/lang/invoke/LambdaForm$BasicType;` | `sun/invoke/util/Wrapper` |
| `basicTypeSlots()I` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/LambdaFormEditor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `spreadArgumentsForm(ILjava/lang/Class;I)Ljava/lang/invoke/LambdaForm;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/LambdaProxyClassArchive`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `find(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;Z[Ljava/lang/Class;[Ljava/lang/invoke/MethodType;)Ljava/lang/Class;` | `jdk/internal/misc/CDS` |
| `register(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;Z[Ljava/lang/Class;[Ljava/lang/invoke/MethodType;Ljava/lang/Class;)Z` | `jdk/internal/misc/CDS` |

### `java/lang/invoke/MemberName`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkForTypeAlias(Ljava/lang/Class;)V` | `sun/invoke/util/VerifyAccess` |

### `java/lang/invoke/MethodHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isBuiltinLoader(Ljava/lang/ClassLoader;)Z` | `jdk/internal/loader/ClassLoaders` |
| `updateForm(Ljava/util/function/Function;)V` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/MethodHandleImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `computeValueConversions(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;ZZ)[Ljava/lang/Object;` | `sun/invoke/util/VerifyType` |
| `unboxResultHandle(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;` | `sun/invoke/util/ValueConversions`, `sun/invoke/util/Wrapper` |
| `valueConversion(Ljava/lang/Class;Ljava/lang/Class;ZZ)Ljava/lang/Object;` | `sun/invoke/util/ValueConversions`, `sun/invoke/util/VerifyType`, `sun/invoke/util/Wrapper` |
| `varargsArray(Ljava/lang/Class;I)Ljava/lang/invoke/MethodHandle;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/MethodHandleImpl$ArrayAccessor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `name(Ljava/lang/Class;Ljava/lang/invoke/MethodHandleImpl$ArrayAccess;)Ljava/lang/String;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/MethodHandleImpl$BindCaller`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkCallerClass(Ljava/lang/Class;)Z` | `jdk/internal/reflect/Reflection` |
| `generateInvokerTemplate()[B` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/MethodVisitor` |

### `java/lang/invoke/MethodHandleNatives`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCharType(Ljava/lang/Class;)C` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/MethodHandleProxies`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `asInterfaceInstance(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection`, `sun/reflect/misc/ReflectUtil` |
| `wrapperInstanceTarget(Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;` | `sun/invoke/WrapperInstance` |
| `wrapperInstanceType(Ljava/lang/Object;)Ljava/lang/Class;` | `sun/invoke/WrapperInstance` |

### `java/lang/invoke/MethodHandleProxies$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `invoke(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/access/JavaLangReflectAccess` |

### `java/lang/invoke/MethodHandleStatics`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `debugEnabled()Z` | `jdk/internal/util/ClassFileDumper` |
| `traceLambdaForm(Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/Class;Ljava/lang/invoke/MemberName;)V` | `jdk/internal/misc/CDS` |
| `traceSpeciesType(Ljava/lang/String;Ljava/lang/Class;)V` | `jdk/internal/misc/CDS` |

### `java/lang/invoke/MethodHandles`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `classData(Ljava/lang/Class;)Ljava/lang/Object;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `jdk/internal/misc/Unsafe` |
| `constant(Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;` | `sun/invoke/util/Wrapper` |
| `identity(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;` | `sun/invoke/util/Wrapper` |
| `insertArgumentPrimitive(Ljava/lang/invoke/BoundMethodHandle;ILjava/lang/Class;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;` | `sun/invoke/util/ValueConversions`, `sun/invoke/util/Wrapper` |
| `lookup()Ljava/lang/invoke/MethodHandles$Lookup;` | `jdk/internal/reflect/Reflection` |
| `memorySegmentViewVarHandle(Ljava/lang/foreign/ValueLayout;)Ljava/lang/invoke/VarHandle;` | `jdk/internal/foreign/Utils` |
| `privateLookupIn(Ljava/lang/Class;Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/invoke/MethodHandles$Lookup;` | `sun/security/util/SecurityConstants` |
| `reflectAs(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;)Ljava/lang/reflect/Member;` | `sun/security/util/SecurityConstants` |
| `zero(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;` | `sun/invoke/util/Wrapper` |
| `zero(Lsun/invoke/util/Wrapper;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/MethodHandles$Lookup`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `accessFailedMessage(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)Ljava/lang/String;` | `sun/invoke/util/VerifyAccess` |
| `checkAccess(BLjava/lang/Class;Ljava/lang/invoke/MemberName;)V` | `sun/invoke/util/VerifyAccess` |
| `checkSecurityManager(Ljava/lang/Class;)V` | `sun/invoke/util/VerifyAccess`, `sun/reflect/misc/ReflectUtil`, `sun/security/util/SecurityConstants` |
| `checkSecurityManager(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)V` | `sun/invoke/util/VerifyAccess`, `sun/reflect/misc/ReflectUtil`, `sun/security/util/SecurityConstants` |
| `ensureInitialized(Ljava/lang/Class;)Ljava/lang/Class;` | `jdk/internal/misc/Unsafe`, `sun/invoke/util/VerifyAccess` |
| `in(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;` | `sun/invoke/util/VerifyAccess` |
| `isClassAccessible(Ljava/lang/Class;)Z` | `sun/invoke/util/VerifyAccess` |
| `lookupClassProtectionDomain()Ljava/security/ProtectionDomain;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |
| `makeHiddenClassDefiner(Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;Ljava/util/Set;ZLjdk/internal/util/ClassFileDumper;)Ljava/lang/invoke/MethodHandles$Lookup$ClassDefiner;` | `jdk/internal/misc/VM` |
| `restrictProtectedReceiver(Ljava/lang/invoke/MemberName;)Z` | `sun/invoke/util/VerifyAccess` |

### `java/lang/invoke/MethodHandles$Lookup$ClassDefiner`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `defineClass(ZLjava/lang/Object;)Ljava/lang/Class;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `jdk/internal/util/ClassFileDumper` |

### `java/lang/invoke/MethodHandles$Lookup$ClassFile`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readClassFile([B)Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;` | `jdk/internal/misc/VM`, `jdk/internal/org/objectweb/asm/ClassReader`, `jdk/internal/org/objectweb/asm/Type` |

### `java/lang/invoke/MethodType`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `canConvert(Ljava/lang/Class;Ljava/lang/Class;)Z` | `sun/invoke/util/Wrapper` |
| `fromDescriptor(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/invoke/MethodType;` | `sun/invoke/util/BytecodeDescriptor` |
| `fromMethodDescriptorString(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/invoke/MethodType;` | `sun/security/util/SecurityConstants` |
| `isViewableAs(Ljava/lang/invoke/MethodType;Z)Z` | `sun/invoke/util/VerifyType` |
| `makeImpl(Ljava/lang/Class;[Ljava/lang/Class;Z)Ljava/lang/invoke/MethodType;` | `jdk/internal/util/ReferencedKeySet` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/misc/Unsafe` |
| `toFieldDescriptorString(Ljava/lang/Class;)Ljava/lang/String;` | `sun/invoke/util/BytecodeDescriptor` |
| `toMethodDescriptorString()Ljava/lang/String;` | `sun/invoke/util/BytecodeDescriptor` |

### `java/lang/invoke/MethodTypeForm`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/invoke/MethodType;)V` | `sun/invoke/util/Wrapper` |
| `canonicalize(Ljava/lang/Class;I)Ljava/lang/Class;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/NativeMethodHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `make(Ljdk/internal/foreign/abi/NativeEntryPoint;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/foreign/abi/NativeEntryPoint` |

### `java/lang/invoke/StringConcatFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `generateMHInlineCopy(Ljava/lang/invoke/MethodType;[Ljava/lang/String;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |
| `makeConcatWithTemplate(Ljava/util/List;Ljava/util/List;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |
| `mixer(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess`, `sun/invoke/util/Wrapper` |
| `newArray()Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |
| `newArrayWithSuffix(Ljava/lang/String;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |
| `newString()Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |
| `newStringifier()Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |
| `objectStringifier()Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |
| `prepender(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess`, `sun/invoke/util/Wrapper` |
| `simpleConcat()Ljava/lang/invoke/MethodHandle;` | `jdk/internal/access/JavaLangAccess` |

### `java/lang/invoke/TypeConvertingMethodAdapter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljdk/internal/org/objectweb/asm/MethodVisitor;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `boxIfTypePrimitive(Ljdk/internal/org/objectweb/asm/Type;)V` | `jdk/internal/org/objectweb/asm/Type` |
| `boxingDescriptor(Lsun/invoke/util/Wrapper;)Ljava/lang/String;` | `sun/invoke/util/Wrapper` |
| `convertType(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V` | `sun/invoke/util/BytecodeDescriptor`, `sun/invoke/util/Wrapper` |
| `iconst(I)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `initWidening(Lsun/invoke/util/Wrapper;I[Lsun/invoke/util/Wrapper;)V` | `sun/invoke/util/Wrapper` |
| `toWrapper(Ljava/lang/String;)Lsun/invoke/util/Wrapper;` | `sun/invoke/util/Wrapper` |
| `unboxMethod(Lsun/invoke/util/Wrapper;)Ljava/lang/String;` | `sun/invoke/util/Wrapper` |
| `unboxingDescriptor(Lsun/invoke/util/Wrapper;)Ljava/lang/String;` | `sun/invoke/util/Wrapper` |
| `widen(Lsun/invoke/util/Wrapper;Lsun/invoke/util/Wrapper;)V` | `sun/invoke/util/Wrapper` |
| `wrapperName(Lsun/invoke/util/Wrapper;)Ljava/lang/String;` | `sun/invoke/util/Wrapper` |
| `wrapperOrNullFromDescriptor(Ljava/lang/String;)Lsun/invoke/util/Wrapper;` | `sun/invoke/util/Wrapper` |

### `java/lang/invoke/VarHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireFence()V` | `jdk/internal/misc/Unsafe` |
| `fullFence()V` | `jdk/internal/misc/Unsafe` |
| `loadLoadFence()V` | `jdk/internal/misc/Unsafe` |
| `releaseFence()V` | `jdk/internal/misc/Unsafe` |
| `storeStoreFence()V` | `jdk/internal/misc/Unsafe` |
| `updateVarForm(Ljava/lang/invoke/VarForm;)V` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBooleans$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZ)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IZZ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleBooleans$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBooleans$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)Z` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Z)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBooleans$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)Z` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)Z` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)Z` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBooleans$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Z)Z` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Z)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Z)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Z)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Z)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;ZZ)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsChars$ArrayHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address([BI)J` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/Unsafe` |
| `index([BI)I` | `jdk/internal/util/Preconditions` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsChars$ByteBufferHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address(Ljava/nio/ByteBuffer;I)J` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `index(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `indexRO(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/misc/Unsafe` |
| `session(Ljava/nio/ByteBuffer;)Ljdk/internal/foreign/MemorySessionImpl;` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsDoubles$ArrayHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address([BI)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/Unsafe` |
| `index([BI)I` | `jdk/internal/util/Preconditions` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsDoubles$ByteBufferHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address(Ljava/nio/ByteBuffer;I)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `index(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `indexRO(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/misc/Unsafe` |
| `session(Ljava/nio/ByteBuffer;)Ljdk/internal/foreign/MemorySessionImpl;` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsFloats$ArrayHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address([BI)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/Unsafe` |
| `index([BI)I` | `jdk/internal/util/Preconditions` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsFloats$ByteBufferHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address(Ljava/nio/ByteBuffer;I)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `index(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `indexRO(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/misc/Unsafe` |
| `session(Ljava/nio/ByteBuffer;)Ljdk/internal/foreign/MemorySessionImpl;` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsInts$ArrayHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address([BI)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndAddConvEndianWithCAS([BII)I` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndConvEndianWithCAS([BII)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrConvEndianWithCAS([BII)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorConvEndianWithCAS([BII)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `index([BI)I` | `jdk/internal/util/Preconditions` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsInts$ByteBufferHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address(Ljava/nio/ByteBuffer;I)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAddConvEndianWithCAS(Ljava/nio/ByteBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndConvEndianWithCAS(Ljava/nio/ByteBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrConvEndianWithCAS(Ljava/nio/ByteBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorConvEndianWithCAS(Ljava/nio/ByteBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `index(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `indexRO(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/misc/Unsafe` |
| `session(Ljava/nio/ByteBuffer;)Ljdk/internal/foreign/MemorySessionImpl;` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsLongs$ArrayHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address([BI)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddConvEndianWithCAS([BIJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndConvEndianWithCAS([BIJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrConvEndianWithCAS([BIJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorConvEndianWithCAS([BIJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/Unsafe` |
| `index([BI)I` | `jdk/internal/util/Preconditions` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsLongs$ByteBufferHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address(Ljava/nio/ByteBuffer;I)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAddConvEndianWithCAS(Ljava/nio/ByteBuffer;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndConvEndianWithCAS(Ljava/nio/ByteBuffer;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrConvEndianWithCAS(Ljava/nio/ByteBuffer;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorConvEndianWithCAS(Ljava/nio/ByteBuffer;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `index(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `indexRO(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/misc/Unsafe` |
| `session(Ljava/nio/ByteBuffer;)Ljdk/internal/foreign/MemorySessionImpl;` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsShorts$ArrayHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address([BI)J` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/Unsafe` |
| `index([BI)I` | `jdk/internal/util/Preconditions` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleByteArrayAsShorts$ByteBufferHandle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `address(Ljava/nio/ByteBuffer;I)J` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `index(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `indexRO(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/misc/Unsafe` |
| `session(Ljava/nio/ByteBuffer;)Ljdk/internal/foreign/MemorySessionImpl;` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBytes$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)B` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IB)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IBB)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleBytes$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)B` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)B` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)B` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)B` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBytes$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)B` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)B` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)B` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)B` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;B)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;BB)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBytes$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)B` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)B` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)B` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)B` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleBytes$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;BB)B` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;BB)B` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;BB)B` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;BB)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;B)B` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;B)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;B)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;B)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;B)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;BB)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;BB)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;BB)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;BB)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleChars$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)C` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IC)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ICC)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleChars$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)C` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)C` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)C` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)C` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleChars$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)C` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)C` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)C` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)C` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;C)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;CC)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleChars$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)C` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)C` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)C` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)C` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleChars$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;CC)C` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;CC)C` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;CC)C` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;CC)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;C)C` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;C)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;C)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;C)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;C)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;CC)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;CC)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;CC)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;CC)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleDoubles$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)D` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ID)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IDD)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleDoubles$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)D` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)D` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)D` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)D` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleDoubles$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)D` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;D)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;DD)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleDoubles$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)D` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)D` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)D` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)D` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleDoubles$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;DD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;DD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;DD)D` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;DD)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;D)D` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;D)D` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;D)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;D)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;D)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;D)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;DD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;DD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;DD)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;DD)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleFloats$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)F` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IF)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IFF)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleFloats$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)F` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)F` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)F` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)F` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleFloats$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)F` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;F)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;FF)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleFloats$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)F` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)F` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)F` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)F` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleFloats$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;FF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;FF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;FF)F` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;FF)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;F)F` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;F)F` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;F)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;F)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;F)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;F)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;FF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;FF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;FF)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;FF)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleInts$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleInts$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)I` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)I` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)I` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)I` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleInts$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleInts$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)I` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)I` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)I` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)I` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleInts$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;II)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;I)I` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;I)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;I)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;I)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;I)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;II)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleLongs$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleLongs$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)J` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)J` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)J` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)J` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleLongs$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleLongs$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)J` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)J` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)J` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)J` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleLongs$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;J)J` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;J)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;J)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;J)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;J)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;JJ)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleReferences$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleReferences$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleReferences$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleSegmentAsBytes`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAddress(Ljava/lang/Object;JJZ)Ljdk/internal/foreign/AbstractMemorySegmentImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)B` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)B` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)B` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)B` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `offsetNoVMAlignCheck(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JB)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JB)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JB)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JB)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |

### `java/lang/invoke/VarHandleSegmentAsChars`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAddress(Ljava/lang/Object;JJZ)Ljdk/internal/foreign/AbstractMemorySegmentImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)C` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)C` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)C` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)C` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `offsetNoVMAlignCheck(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JC)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JC)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JC)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JC)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |

### `java/lang/invoke/VarHandleSegmentAsDoubles`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAddress(Ljava/lang/Object;JJZ)Ljdk/internal/foreign/AbstractMemorySegmentImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JD)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JD)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JD)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)D` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `offsetNoVMAlignCheck(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JD)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JD)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JD)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JD)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JDD)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |

### `java/lang/invoke/VarHandleSegmentAsFloats`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAddress(Ljava/lang/Object;JJZ)Ljdk/internal/foreign/AbstractMemorySegmentImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JF)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JF)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JF)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)F` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `offsetNoVMAlignCheck(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JF)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JF)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JF)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JF)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JFF)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |

### `java/lang/invoke/VarHandleSegmentAsInts`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAddress(Ljava/lang/Object;JJZ)Ljdk/internal/foreign/AbstractMemorySegmentImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAddConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAndConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOrConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXorConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)I` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `offsetNoVMAlignCheck(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JI)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JII)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |

### `java/lang/invoke/VarHandleSegmentAsLongs`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAddress(Ljava/lang/Object;JJZ)Ljdk/internal/foreign/AbstractMemorySegmentImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAddConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAndConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOrConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXorConvEndianWithCAS(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `offsetNoVMAlignCheck(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJJ)Z` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |

### `java/lang/invoke/VarHandleSegmentAsShorts`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAddress(Ljava/lang/Object;JJZ)Ljdk/internal/foreign/AbstractMemorySegmentImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)S` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)S` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)S` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)S` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `offsetNoVMAlignCheck(Ljdk/internal/foreign/AbstractMemorySegmentImpl;JJ)J` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JS)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JS)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JS)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JS)V` | `jdk/internal/foreign/AbstractMemorySegmentImpl`, `jdk/internal/misc/ScopedMemoryAccess` |

### `java/lang/invoke/VarHandleShorts$Array`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IS)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ISS)Z` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/Preconditions` |

### `java/lang/invoke/VarHandleShorts$FieldInstanceReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)S` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)S` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)S` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)S` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleShorts$FieldInstanceReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)S` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)S` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)S` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)S` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;S)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;SS)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleShorts$FieldStaticReadOnly`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/invoke/VarHandle;)S` | `jdk/internal/misc/Unsafe` |
| `getAcquire(Ljava/lang/invoke/VarHandle;)S` | `jdk/internal/misc/Unsafe` |
| `getOpaque(Ljava/lang/invoke/VarHandle;)S` | `jdk/internal/misc/Unsafe` |
| `getVolatile(Ljava/lang/invoke/VarHandle;)S` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandleShorts$FieldStaticReadWrite`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compareAndExchange(Ljava/lang/invoke/VarHandle;SS)S` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(Ljava/lang/invoke/VarHandle;SS)S` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(Ljava/lang/invoke/VarHandle;SS)S` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(Ljava/lang/invoke/VarHandle;SS)Z` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndAddAcquire(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndAddRelease(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAnd(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndAcquire(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseAndRelease(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOr(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrAcquire(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRelease(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXor(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorAcquire(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseXorRelease(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndSetAcquire(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `getAndSetRelease(Ljava/lang/invoke/VarHandle;S)S` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/invoke/VarHandle;S)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(Ljava/lang/invoke/VarHandle;S)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(Ljava/lang/invoke/VarHandle;S)V` | `jdk/internal/misc/Unsafe` |
| `setVolatile(Ljava/lang/invoke/VarHandle;S)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/invoke/VarHandle;SS)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(Ljava/lang/invoke/VarHandle;SS)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(Ljava/lang/invoke/VarHandle;SS)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(Ljava/lang/invoke/VarHandle;SS)Z` | `jdk/internal/misc/Unsafe` |

### `java/lang/invoke/VarHandles`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFieldFromReceiverAndOffset(Ljava/lang/Class;JLjava/lang/Class;)Ljava/lang/reflect/Field;` | `jdk/internal/misc/Unsafe` |
| `getStaticFieldFromBaseAndOffset(Ljava/lang/Class;JLjava/lang/Class;)Ljava/lang/reflect/Field;` | `jdk/internal/misc/Unsafe` |
| `insertCoordinates(Ljava/lang/invoke/VarHandle;I[Ljava/lang/Object;)Ljava/lang/invoke/VarHandle;` | `sun/invoke/util/Wrapper` |
| `makeArrayElementHandle(Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;` | `jdk/internal/misc/Unsafe` |
| `makeFieldHandle(Ljava/lang/invoke/MemberName;Ljava/lang/Class;Ljava/lang/Class;Z)Ljava/lang/invoke/VarHandle;` | `jdk/internal/misc/Unsafe` |
| `memorySegmentViewHandle(Ljava/lang/Class;JLjava/nio/ByteOrder;)Ljava/lang/invoke/VarHandle;` | `jdk/internal/foreign/Utils` |

### `java/lang/management/DefaultPlatformMBeanProvider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/management/spi/PlatformMBeanProvider` |
| `init()Ljava/util/List;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$10`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$11`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$5`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$6`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$7`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$8`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/DefaultPlatformMBeanProvider$9`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nameToMBeanMap()Ljava/util/Map;` | `sun/management/ManagementFactoryHelper` |

### `java/lang/management/LockInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `from(Ljavax/management/openmbean/CompositeData;)Ljava/lang/management/LockInfo;` | `sun/management/LockInfoCompositeData` |

### `java/lang/management/ManagementFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPlatformMXBean(Ljava/lang/Class;)Ljava/lang/management/PlatformManagedObject;` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `getPlatformMXBean(Ljavax/management/MBeanServerConnection;Ljava/lang/Class;)Ljava/lang/management/PlatformManagedObject;` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `getProxyNames(Lsun/management/spi/PlatformMBeanProvider$PlatformComponent;Ljavax/management/MBeanServerConnection;Ljava/lang/Class;)Ljava/util/stream/Stream;` | `sun/management/Util`, `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `lambda$getPlatformMBeanServer$0(Lsun/management/spi/PlatformMBeanProvider$PlatformComponent;)Ljava/util/stream/Stream;` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `lambda$getPlatformMXBeans$3(Ljava/lang/Class;Lsun/management/spi/PlatformMBeanProvider$PlatformComponent;)Ljava/util/stream/Stream;` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `lambda$getPlatformManagementInterfaces$4(Lsun/management/spi/PlatformMBeanProvider$PlatformComponent;)Ljava/util/stream/Stream;` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `newPlatformMXBeanProxy(Ljavax/management/MBeanServerConnection;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;` | `jdk/internal/misc/VM` |

### `java/lang/management/ManagementFactory$PlatformMBeanFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `findSingleton(Ljava/lang/Class;)Lsun/management/spi/PlatformMBeanProvider$PlatformComponent;` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `lambda$findFirst$0(Ljava/lang/String;Lsun/management/spi/PlatformMBeanProvider$PlatformComponent;)Z` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |
| `lambda$findSingleton$1(Ljava/lang/String;Lsun/management/spi/PlatformMBeanProvider$PlatformComponent;)Z` | `sun/management/spi/PlatformMBeanProvider$PlatformComponent` |

### `java/lang/management/MemoryNotificationInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/openmbean/CompositeData;)V` | `sun/management/MemoryNotifInfoCompositeData` |
| `from(Ljavax/management/openmbean/CompositeData;)Ljava/lang/management/MemoryNotificationInfo;` | `sun/management/MemoryNotifInfoCompositeData` |

### `java/lang/management/MemoryUsage`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/openmbean/CompositeData;)V` | `sun/management/MemoryUsageCompositeData` |
| `from(Ljavax/management/openmbean/CompositeData;)Ljava/lang/management/MemoryUsage;` | `sun/management/MemoryUsageCompositeData` |

### `java/lang/management/MonitorInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `from(Ljavax/management/openmbean/CompositeData;)Ljava/lang/management/MonitorInfo;` | `sun/management/MonitorInfoCompositeData` |

### `java/lang/management/ThreadInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/openmbean/CompositeData;)V` | `sun/management/ThreadInfoCompositeData` |
| `from(Ljavax/management/openmbean/CompositeData;)Ljava/lang/management/ThreadInfo;` | `sun/management/ThreadInfoCompositeData` |
| `initialize(Ljava/lang/Thread;ILjava/lang/Object;Ljava/lang/Thread;JJJJ[Ljava/lang/StackTraceElement;[Ljava/lang/management/MonitorInfo;[Ljava/lang/management/LockInfo;)V` | `sun/management/ManagementFactoryHelper` |

### `java/lang/module/Configuration`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/module/ModuleFinder;Ljava/util/Map;)V` | `jdk/internal/module/ModuleReferenceImpl`, `jdk/internal/module/ModuleTarget` |

### `java/lang/module/ModuleDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `read(Ljava/io/InputStream;)Ljava/lang/module/ModuleDescriptor;` | `jdk/internal/module/ModuleInfo`, `jdk/internal/module/ModuleInfo$Attributes` |
| `read(Ljava/io/InputStream;Ljava/util/function/Supplier;)Ljava/lang/module/ModuleDescriptor;` | `jdk/internal/module/ModuleInfo`, `jdk/internal/module/ModuleInfo$Attributes` |
| `read(Ljava/nio/ByteBuffer;)Ljava/lang/module/ModuleDescriptor;` | `jdk/internal/module/ModuleInfo`, `jdk/internal/module/ModuleInfo$Attributes` |
| `read(Ljava/nio/ByteBuffer;Ljava/util/function/Supplier;)Ljava/lang/module/ModuleDescriptor;` | `jdk/internal/module/ModuleInfo`, `jdk/internal/module/ModuleInfo$Attributes` |

### `java/lang/module/ModuleDescriptor$Builder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;ZLjava/util/Set;)V` | `jdk/internal/module/Checks` |
| `exports(Ljava/util/Set;Ljava/lang/String;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `exports(Ljava/util/Set;Ljava/lang/String;Ljava/util/Set;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `mainClass(Ljava/lang/String;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `opens(Ljava/util/Set;Ljava/lang/String;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `opens(Ljava/util/Set;Ljava/lang/String;Ljava/util/Set;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `provides(Ljava/lang/String;Ljava/util/List;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `requires(Ljava/util/Set;Ljava/lang/String;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `requires(Ljava/util/Set;Ljava/lang/String;Ljava/lang/module/ModuleDescriptor$Version;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |
| `uses(Ljava/lang/String;)Ljava/lang/module/ModuleDescriptor$Builder;` | `jdk/internal/module/Checks` |

### `java/lang/module/ModuleFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `of([Ljava/nio/file/Path;)Ljava/lang/module/ModuleFinder;` | `jdk/internal/module/ModulePath` |
| `ofSystem()Ljava/lang/module/ModuleFinder;` | `jdk/internal/module/SystemModuleFinders` |

### `java/lang/module/Resolver`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addFoundModule(Ljava/lang/module/ModuleReference;)V` | `jdk/internal/module/ModuleReferenceImpl` |
| `bind(Z)Ljava/lang/module/Resolver;` | `jdk/internal/module/ModuleReferenceImpl`, `jdk/internal/module/ModuleResolution` |
| `checkHashes()V` | `jdk/internal/module/ModuleHashes`, `jdk/internal/module/ModuleReferenceImpl` |
| `checkTargetPlatform(Ljava/lang/String;Ljdk/internal/module/ModuleTarget;)V` | `jdk/internal/module/ModuleTarget` |

### `java/lang/ref/Cleaner`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `jdk/internal/ref/CleanerImpl` |
| `create()Ljava/lang/ref/Cleaner;` | `jdk/internal/ref/CleanerImpl` |
| `create(Ljava/util/concurrent/ThreadFactory;)Ljava/lang/ref/Cleaner;` | `jdk/internal/ref/CleanerImpl` |
| `register(Ljava/lang/Object;Ljava/lang/Runnable;)Ljava/lang/ref/Cleaner$Cleanable;` | `jdk/internal/ref/CleanerImpl$PhantomCleanableRef` |

### `java/lang/ref/Finalizer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `runFinalization()V` | `jdk/internal/misc/VM` |
| `runFinalizer(Ljdk/internal/access/JavaLangAccess;)V` | `jdk/internal/access/JavaLangAccess` |

### `java/lang/ref/Finalizer$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `jdk/internal/access/SharedSecrets` |

### `java/lang/ref/Finalizer$FinalizerThread`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `jdk/internal/access/SharedSecrets` |

### `java/lang/ref/Reference`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `processPendingReferences()V` | `jdk/internal/ref/Cleaner` |

### `java/lang/ref/Reference$ReferenceHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `jdk/internal/misc/Unsafe` |

### `java/lang/ref/ReferenceQueue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `enqueue0(Ljava/lang/ref/Reference;)Z` | `jdk/internal/misc/VM` |
| `poll0()Ljava/lang/ref/Reference;` | `jdk/internal/misc/VM` |

### `java/lang/reflect/AccessibleObject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `canAccess(Ljava/lang/Object;)Z` | `jdk/internal/reflect/Reflection` |
| `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V` | `jdk/internal/reflect/Reflection` |
| `checkCanSetAccessible(Ljava/lang/Class;Ljava/lang/Class;Z)Z` | `jdk/internal/reflect/Reflection` |
| `checkPermission()V` | `sun/security/util/SecurityConstants` |
| `printStackTraceWhenAccessFails()Z` | `jdk/internal/misc/VM`, `sun/security/action/GetPropertyAction` |
| `setAccessible([Ljava/lang/reflect/AccessibleObject;Z)V` | `jdk/internal/reflect/Reflection` |
| `slowVerifyAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z` | `jdk/internal/reflect/Reflection` |
| `trySetAccessible()Z` | `jdk/internal/reflect/Reflection` |

### `java/lang/reflect/AnnotatedElement`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAnnotationsByType(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationType` |
| `getDeclaredAnnotationsByType(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationSupport` |

### `java/lang/reflect/Constructor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;` | `jdk/internal/misc/VM`, `jdk/internal/reflect/ReflectionFactory` |
| `getAnnotatedReceiverType()Ljava/lang/reflect/AnnotatedType;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget`, `sun/reflect/annotation/TypeAnnotationParser` |
| `getFactory()Lsun/reflect/generics/factory/GenericsFactory;` | `sun/reflect/generics/factory/CoreReflectionFactory`, `sun/reflect/generics/scope/ConstructorScope` |
| `getGenericInfo()Lsun/reflect/generics/repository/ConstructorRepository;` | `sun/reflect/generics/repository/ConstructorRepository` |
| `getTypeParameters()[Ljava/lang/reflect/TypeVariable;` | `sun/reflect/generics/repository/ConstructorRepository`, `sun/reflect/generics/repository/GenericDeclRepository` |
| `newInstance([Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `newInstanceWithCaller([Ljava/lang/Object;ZLjava/lang/Class;)Ljava/lang/Object;` | `jdk/internal/reflect/ConstructorAccessor` |
| `setAccessible(Z)V` | `jdk/internal/reflect/Reflection` |

### `java/lang/reflect/Executable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `declaredAnnotations()Ljava/util/Map;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/AnnotationParser` |
| `getAnnotatedExceptionTypes()[Ljava/lang/reflect/AnnotatedType;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget`, `sun/reflect/annotation/TypeAnnotationParser` |
| `getAnnotatedParameterTypes()[Ljava/lang/reflect/AnnotatedType;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget`, `sun/reflect/annotation/TypeAnnotationParser` |
| `getAnnotatedReceiverType()Ljava/lang/reflect/AnnotatedType;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget`, `sun/reflect/annotation/TypeAnnotationParser` |
| `getAnnotatedReturnType0(Ljava/lang/reflect/Type;)Ljava/lang/reflect/AnnotatedType;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget`, `sun/reflect/annotation/TypeAnnotationParser` |
| `getAnnotationsByType(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationSupport` |
| `getDeclaredAnnotations()[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationParser` |
| `getGenericExceptionTypes()[Ljava/lang/reflect/Type;` | `sun/reflect/generics/repository/ConstructorRepository` |
| `getGenericParameterTypes()[Ljava/lang/reflect/Type;` | `sun/reflect/generics/repository/ConstructorRepository` |
| `parameterize(Ljava/lang/Class;)Ljava/lang/reflect/Type;` | `sun/reflect/generics/reflectiveObjects/ParameterizedTypeImpl` |
| `parseParameterAnnotations([B)[[Ljava/lang/annotation/Annotation;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/AnnotationParser` |

### `java/lang/reflect/Field`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireFieldAccessor()Ljdk/internal/reflect/FieldAccessor;` | `jdk/internal/reflect/ReflectionFactory` |
| `acquireOverrideFieldAccessor()Ljdk/internal/reflect/FieldAccessor;` | `jdk/internal/reflect/ReflectionFactory` |
| `declaredAnnotations()Ljava/util/Map;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/AnnotationParser` |
| `get(Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getAnnotatedType()Ljava/lang/reflect/AnnotatedType;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget`, `sun/reflect/annotation/TypeAnnotationParser` |
| `getAnnotationsByType(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationSupport` |
| `getBoolean(Ljava/lang/Object;)Z` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getByte(Ljava/lang/Object;)B` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getChar(Ljava/lang/Object;)C` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getDeclaredAnnotations()[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationParser` |
| `getDouble(Ljava/lang/Object;)D` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getFactory()Lsun/reflect/generics/factory/GenericsFactory;` | `sun/reflect/generics/factory/CoreReflectionFactory`, `sun/reflect/generics/scope/ClassScope` |
| `getFloat(Ljava/lang/Object;)F` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getGenericInfo()Lsun/reflect/generics/repository/FieldRepository;` | `sun/reflect/generics/repository/FieldRepository` |
| `getGenericType()Ljava/lang/reflect/Type;` | `sun/reflect/generics/repository/FieldRepository` |
| `getInt(Ljava/lang/Object;)I` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getLong(Ljava/lang/Object;)J` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `getShort(Ljava/lang/Object;)S` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `set(Ljava/lang/Object;Ljava/lang/Object;)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setAccessible(Z)V` | `jdk/internal/reflect/Reflection` |
| `setBoolean(Ljava/lang/Object;Z)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setByte(Ljava/lang/Object;B)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setChar(Ljava/lang/Object;C)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setDouble(Ljava/lang/Object;D)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setFloat(Ljava/lang/Object;F)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setInt(Ljava/lang/Object;I)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setLong(Ljava/lang/Object;J)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |
| `setShort(Ljava/lang/Object;S)V` | `jdk/internal/reflect/FieldAccessor`, `jdk/internal/reflect/Reflection` |

### `java/lang/reflect/InvocationHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `invokeDefault(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |

### `java/lang/reflect/Method`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireMethodAccessor()Ljdk/internal/reflect/MethodAccessor;` | `jdk/internal/misc/VM`, `jdk/internal/reflect/ReflectionFactory` |
| `getDefaultValue()Ljava/lang/Object;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/AnnotationParser`, `sun/reflect/annotation/AnnotationType`, +1 |
| `getFactory()Lsun/reflect/generics/factory/GenericsFactory;` | `sun/reflect/generics/factory/CoreReflectionFactory`, `sun/reflect/generics/scope/MethodScope` |
| `getGenericInfo()Lsun/reflect/generics/repository/MethodRepository;` | `sun/reflect/generics/repository/MethodRepository` |
| `getGenericReturnType()Ljava/lang/reflect/Type;` | `sun/reflect/generics/repository/MethodRepository` |
| `getTypeParameters()[Ljava/lang/reflect/TypeVariable;` | `sun/reflect/generics/repository/GenericDeclRepository`, `sun/reflect/generics/repository/MethodRepository` |
| `invoke(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/reflect/MethodAccessor`, `jdk/internal/reflect/Reflection` |
| `invoke(Ljava/lang/Object;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;` | `jdk/internal/reflect/MethodAccessor` |
| `isCallerSensitive()Z` | `jdk/internal/reflect/Reflection` |
| `setAccessible(Z)V` | `jdk/internal/reflect/Reflection` |

### `java/lang/reflect/Parameter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAnnotationsByType(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationSupport` |

### `java/lang/reflect/Proxy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkNewProxyPermission(Ljava/lang/Class;Ljava/lang/Class;)V` | `sun/reflect/misc/ReflectUtil` |
| `checkProxyAccess(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V` | `sun/reflect/misc/ReflectUtil`, `sun/security/util/SecurityConstants` |
| `getInvocationHandler(Ljava/lang/Object;)Ljava/lang/reflect/InvocationHandler;` | `jdk/internal/reflect/Reflection`, `sun/reflect/misc/ReflectUtil` |
| `getProxyClass(Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/Class;` | `jdk/internal/reflect/Reflection` |
| `getProxyConstructor(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/reflect/Constructor;` | `jdk/internal/loader/AbstractClassLoaderValue$Sub`, `jdk/internal/loader/ClassLoaderValue` |
| `lambda$getProxyConstructor$0(Ljava/lang/ClassLoader;Ljdk/internal/loader/AbstractClassLoaderValue$Sub;)Ljava/lang/reflect/Constructor;` | `jdk/internal/loader/AbstractClassLoaderValue$Sub` |
| `lambda$getProxyConstructor$1(Ljava/lang/ClassLoader;Ljdk/internal/loader/AbstractClassLoaderValue$Sub;)Ljava/lang/reflect/Constructor;` | `jdk/internal/loader/AbstractClassLoaderValue$Sub` |
| `newProxyInstance(Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |

### `java/lang/reflect/Proxy$ProxyBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/ClassLoader;Ljava/util/List;)V` | `jdk/internal/misc/VM` |
| `defineProxyClass(Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;Ljava/util/List;)Ljava/lang/Class;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/loader/AbstractClassLoaderValue$Sub`, `jdk/internal/loader/ClassLoaderValue` |
| `ensureAccess(Ljava/lang/Module;Ljava/lang/Class;)V` | `jdk/internal/module/Modules` |
| `ensureVisible(Ljava/lang/ClassLoader;Ljava/lang/Class;)V` | `jdk/internal/access/JavaLangAccess` |
| `getDynamicModule(Ljava/lang/ClassLoader;)Ljava/lang/Module;` | `jdk/internal/loader/ClassLoaderValue` |
| `isProxyClass(Ljava/lang/Class;)Z` | `jdk/internal/loader/AbstractClassLoaderValue$Sub`, `jdk/internal/loader/ClassLoaderValue` |
| `lambda$getDynamicModule$1(Ljava/lang/ClassLoader;Ljdk/internal/loader/ClassLoaderValue;)Ljava/lang/Module;` | `jdk/internal/module/Modules` |
| `proxyClassContext(Ljava/lang/ClassLoader;Ljava/util/List;Ljava/util/Set;)Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;` | `jdk/internal/module/Modules` |

### `java/lang/reflect/ProxyGenerator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/util/List;I)V` | `jdk/internal/org/objectweb/asm/ClassWriter` |
| `generateConstructor()V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `generateLookupAccessor()V` | `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor`, `jdk/internal/org/objectweb/asm/Type` |
| `generateStaticInitializer()V` | `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor`, `jdk/internal/org/objectweb/asm/Type` |

### `java/lang/reflect/ProxyGenerator$PrimitiveTypeInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;ILjava/lang/Class;II)V` | `sun/invoke/util/Wrapper` |

### `java/lang/reflect/ProxyGenerator$ProxyMethod`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `codeClassForName(Ljdk/internal/org/objectweb/asm/MethodVisitor;Ljava/lang/Class;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `codeFieldInitialization(Ljdk/internal/org/objectweb/asm/MethodVisitor;Ljava/lang/String;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `codeUnwrapReturnValue(Ljdk/internal/org/objectweb/asm/MethodVisitor;Ljava/lang/Class;)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `codeWrapArgument(Ljdk/internal/org/objectweb/asm/MethodVisitor;Ljava/lang/Class;I)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `emitIconstInsn(Ljdk/internal/org/objectweb/asm/MethodVisitor;I)V` | `jdk/internal/org/objectweb/asm/MethodVisitor` |
| `generateMethod(Ljdk/internal/org/objectweb/asm/ClassWriter;Ljava/lang/String;)V` | `jdk/internal/org/objectweb/asm/ClassWriter`, `jdk/internal/org/objectweb/asm/Label`, `jdk/internal/org/objectweb/asm/MethodVisitor` |

### `java/lang/reflect/RecordComponent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `declaredAnnotations()Ljava/util/Map;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/AnnotationParser` |
| `getAnnotatedType()Ljava/lang/reflect/AnnotatedType;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets`, `sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget`, `sun/reflect/annotation/TypeAnnotationParser` |
| `getDeclaredAnnotations()[Ljava/lang/annotation/Annotation;` | `sun/reflect/annotation/AnnotationParser` |
| `getFactory()Lsun/reflect/generics/factory/GenericsFactory;` | `sun/reflect/generics/factory/CoreReflectionFactory`, `sun/reflect/generics/scope/ClassScope` |
| `getGenericInfo()Lsun/reflect/generics/repository/FieldRepository;` | `sun/reflect/generics/repository/FieldRepository` |
| `getGenericType()Ljava/lang/reflect/Type;` | `sun/reflect/generics/repository/FieldRepository` |

### `java/lang/reflect/UndeclaredThrowableException`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |

### `java/lang/runtime/Carriers$CarrierObject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInteger(I)I` | `jdk/internal/misc/Unsafe` |
| `getLong(I)J` | `jdk/internal/misc/Unsafe` |
| `getObject(I)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `offsetToInt(I)J` | `jdk/internal/misc/Unsafe` |
| `offsetToLong(I)J` | `jdk/internal/misc/Unsafe` |
| `offsetToObject(I)J` | `jdk/internal/misc/Unsafe` |
| `putInteger(II)Ljava/lang/runtime/Carriers$CarrierObject;` | `jdk/internal/misc/Unsafe` |
| `putLong(IJ)Ljava/lang/runtime/Carriers$CarrierObject;` | `jdk/internal/misc/Unsafe` |
| `putObject(ILjava/lang/Object;)Ljava/lang/runtime/Carriers$CarrierObject;` | `jdk/internal/misc/Unsafe` |

### `java/lang/runtime/SwitchBootstraps`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mappedEnumLookup(Ljava/lang/Enum;Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/Class;[Ljava/lang/Enum$EnumDesc;Ljava/lang/runtime/SwitchBootstraps$EnumMap;)I` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |

### `java/lang/runtime/TemplateSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `interpolate(Ljava/util/List;Ljava/util/List;)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |

### `java/math/BigDecimal$UnsafeHolder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setIntValAndScale(Ljava/math/BigDecimal;Ljava/math/BigInteger;I)V` | `jdk/internal/misc/Unsafe` |
| `setIntValVolatile(Ljava/math/BigDecimal;Ljava/math/BigInteger;)V` | `jdk/internal/misc/Unsafe` |

### `java/math/BigInteger$UnsafeHolder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `putSignAndMag(Ljava/math/BigInteger;I[I)V` | `jdk/internal/misc/Unsafe` |

### `java/net/CookieHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefault()Ljava/net/CookieHandler;` | `sun/security/util/SecurityConstants` |
| `setDefault(Ljava/net/CookieHandler;)V` | `sun/security/util/SecurityConstants` |

### `java/net/CookieManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `put(Ljava/net/URI;Ljava/util/Map;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/net/DatagramPacket`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setData([BII)V` | `jdk/internal/util/Preconditions` |
| `setLength(I)V` | `jdk/internal/util/Preconditions` |

### `java/net/DatagramSocket`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createDelegate(Ljava/net/SocketAddress;Ljava/lang/Class;)Ljava/net/DatagramSocket;` | `sun/nio/ch/DefaultSelectorProvider`, `sun/nio/ch/SelectorProviderImpl` |

### `java/net/HostPortrange`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;)V` | `sun/net/util/IPAddressUtil` |

### `java/net/IDN`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toASCIIInternal(Ljava/lang/String;I)Ljava/lang/String;` | `jdk/internal/icu/impl/Punycode`, `jdk/internal/icu/text/StringPrep`, `jdk/internal/icu/text/UCharacterIterator` |
| `toUnicodeInternal(Ljava/lang/String;I)Ljava/lang/String;` | `jdk/internal/icu/impl/Punycode`, `jdk/internal/icu/text/StringPrep`, `jdk/internal/icu/text/UCharacterIterator` |

### `java/net/Inet6Address`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/misc/Unsafe` |

### `java/net/InetAddress`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `builtinConfiguration()Ljava/net/spi/InetAddressResolverProvider$Configuration;` | `sun/net/ResolverProviderConfiguration` |
| `checkNumericZone(Ljava/lang/String;)I` | `sun/net/util/IPAddressUtil` |
| `getAllByName(Ljava/lang/String;)[Ljava/net/InetAddress;` | `sun/net/util/IPAddressUtil` |
| `getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;` | `sun/net/util/IPAddressUtil` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/misc/Unsafe` |
| `resolver()Ljava/net/spi/InetAddressResolver;` | `jdk/internal/misc/VM` |

### `java/net/InetAddress$HostsFileResolver`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createAddressByteArray(Ljava/lang/String;)[B` | `sun/net/util/IPAddressUtil` |
| `lookupByAddress([B)Ljava/lang/String;` | `sun/nio/cs/UTF_8` |
| `lookupByName(Ljava/lang/String;Ljava/net/spi/InetAddressResolver$LookupPolicy;)Ljava/util/stream/Stream;` | `sun/nio/cs/UTF_8` |

### `java/net/InetAddress$NameServiceAddresses`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()[Ljava/net/InetAddress;` | `sun/net/InetAddressCachePolicy` |

### `java/net/InetAddress$PlatformResolver`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lookupByAddress([B)Ljava/lang/String;` | `jdk/internal/misc/Blocker` |
| `lookupByName(Ljava/lang/String;Ljava/net/spi/InetAddressResolver$LookupPolicy;)Ljava/util/stream/Stream;` | `jdk/internal/misc/Blocker` |

### `java/net/InetAddress$ValidCachedLookup`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()[Ljava/net/InetAddress;` | `sun/net/InetAddressCachePolicy` |

### `java/net/InetSocketAddress`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/misc/Unsafe` |

### `java/net/JarURLConnection`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `parseSpecs(Ljava/net/URL;)V` | `sun/net/www/ParseUtil` |

### `java/net/ProxySelector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefault()Ljava/net/ProxySelector;` | `sun/security/util/SecurityConstants` |
| `setDefault(Ljava/net/ProxySelector;)V` | `sun/security/util/SecurityConstants` |

### `java/net/ResponseCache`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefault()Ljava/net/ResponseCache;` | `sun/security/util/SecurityConstants` |
| `setDefault(Ljava/net/ResponseCache;)V` | `sun/security/util/SecurityConstants` |

### `java/net/ServerSocket`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkPermission()Ljava/lang/Void;` | `sun/security/util/SecurityConstants` |

### `java/net/Socket`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/net/Proxy;)V` | `sun/net/ApplicationProxy` |
| `checkPermission(Ljava/net/SocketImpl;)Ljava/lang/Void;` | `sun/security/util/SecurityConstants` |

### `java/net/SocketImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createPlatformSocketImpl(Z)Ljava/net/SocketImpl;` | `sun/nio/ch/NioSocketImpl` |

### `java/net/SocketPermission`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `authorizedIPv4(Ljava/lang/String;[B)Z` | `sun/security/util/Debug` |
| `authorizedIPv6(Ljava/lang/String;[B)Z` | `sun/security/util/Debug` |
| `getDebug()Lsun/security/util/Debug;` | `sun/security/util/Debug` |
| `init(Ljava/lang/String;I)V` | `sun/net/util/IPAddressUtil` |
| `isUntrusted()Z` | `sun/net/www/URLConnection`, `sun/security/util/Debug` |
| `match(Ljava/lang/String;Ljava/lang/String;)Z` | `sun/security/util/RegisteredDomain` |

### `java/net/SocketPermission$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Integer;` | `sun/net/PortConfig` |

### `java/net/SocksSocketImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `authenticate(BLjava/io/InputStream;Ljava/io/BufferedOutputStream;J)Z` | `jdk/internal/util/StaticProperty` |
| `connect(Ljava/net/SocketAddress;I)V` | `sun/net/www/ParseUtil` |
| `getUserName()Ljava/lang/String;` | `jdk/internal/util/StaticProperty` |
| `useV4(Ljava/net/Proxy;)Z` | `sun/net/SocksProxy`, `sun/net/spi/DefaultProxySelector` |

### `java/net/URI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `decode(Ljava/lang/String;Z)Ljava/lang/String;` | `sun/nio/cs/UTF_8` |
| `encode(Ljava/lang/String;)Ljava/lang/String;` | `sun/nio/cs/UTF_8` |
| `quote(Ljava/lang/String;JJ)Ljava/lang/String;` | `sun/nio/cs/UTF_8` |

### `java/net/URL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V` | `sun/net/util/IPAddressUtil`, `sun/net/www/protocol/jar/Handler` |
| `checkSpecifyHandler(Ljava/lang/SecurityManager;)V` | `sun/security/util/SecurityConstants` |
| `endLookup(Ljava/lang/Object;)V` | `jdk/internal/misc/ThreadTracker` |
| `getURLStreamHandler(Ljava/lang/String;)Ljava/net/URLStreamHandler;` | `jdk/internal/misc/VM` |
| `isBuiltinStreamHandler(Ljava/net/URLStreamHandler;)Z` | `jdk/internal/misc/VM` |
| `lookupViaProperty(Ljava/lang/String;)Ljava/net/URLStreamHandler;` | `sun/security/action/GetPropertyAction` |
| `openConnection(Ljava/net/Proxy;)Ljava/net/URLConnection;` | `sun/net/ApplicationProxy` |
| `toURI()Ljava/net/URI;` | `sun/net/util/IPAddressUtil` |
| `tryBeginLookup()Ljava/lang/Object;` | `jdk/internal/misc/ThreadTracker` |

### `java/net/URL$DefaultFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createURLStreamHandler(Ljava/lang/String;)Ljava/net/URLStreamHandler;` | `sun/net/www/protocol/file/Handler`, `sun/net/www/protocol/jar/Handler`, `sun/net/www/protocol/jrt/Handler` |

### `java/net/URLClassLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;[Ljava/net/URL;Ljava/lang/ClassLoader;)V` | `jdk/internal/loader/URLClassPath` |
| `<init>(Ljava/lang/String;[Ljava/net/URL;Ljava/lang/ClassLoader;Ljava/net/URLStreamHandlerFactory;)V` | `jdk/internal/loader/URLClassPath` |
| `<init>(Ljava/lang/String;[Ljava/net/URL;Ljava/lang/ClassLoader;Ljava/security/AccessControlContext;)V` | `jdk/internal/loader/URLClassPath` |
| `<init>([Ljava/net/URL;)V` | `jdk/internal/loader/URLClassPath` |
| `<init>([Ljava/net/URL;Ljava/lang/ClassLoader;)V` | `jdk/internal/loader/URLClassPath` |
| `<init>([Ljava/net/URL;Ljava/lang/ClassLoader;Ljava/net/URLStreamHandlerFactory;)V` | `jdk/internal/loader/URLClassPath` |
| `<init>([Ljava/net/URL;Ljava/security/AccessControlContext;)V` | `jdk/internal/loader/URLClassPath` |
| `addURL(Ljava/net/URL;)V` | `jdk/internal/loader/URLClassPath` |
| `close()V` | `jdk/internal/loader/URLClassPath` |
| `defineClass(Ljava/lang/String;Ljdk/internal/loader/Resource;)Ljava/lang/Class;` | `jdk/internal/loader/Resource`, `jdk/internal/perf/PerfCounter` |
| `definePackage(Ljava/lang/String;Ljava/util/jar/Manifest;Ljava/net/URL;)Ljava/lang/Package;` | `jdk/internal/access/JavaUtilJarAccess`, `jdk/internal/access/SharedSecrets` |
| `findResource(Ljava/lang/String;)Ljava/net/URL;` | `jdk/internal/loader/URLClassPath` |
| `findResources(Ljava/lang/String;)Ljava/util/Enumeration;` | `jdk/internal/loader/URLClassPath` |
| `getPermissions(Ljava/security/CodeSource;)Ljava/security/PermissionCollection;` | `sun/net/www/ParseUtil` |
| `getURLs()[Ljava/net/URL;` | `jdk/internal/loader/URLClassPath` |
| `isSealed(Ljava/lang/String;Ljava/util/jar/Manifest;)Z` | `jdk/internal/access/JavaUtilJarAccess`, `jdk/internal/access/SharedSecrets` |

### `java/net/URLClassLoader$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Class;` | `jdk/internal/loader/Resource`, `jdk/internal/loader/URLClassPath` |

### `java/net/URLClassLoader$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/net/URL;` | `jdk/internal/loader/URLClassPath` |

### `java/net/URLClassLoader$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `next()Z` | `jdk/internal/loader/URLClassPath` |

### `java/net/URLConnection`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addRequestProperty(Ljava/lang/String;Ljava/lang/String;)V` | `sun/net/www/MessageHeader` |
| `getContentHandlerPkgPrefixes()Ljava/lang/String;` | `sun/security/action/GetPropertyAction` |
| `getPermission()Ljava/security/Permission;` | `sun/security/util/SecurityConstants` |
| `getRequestProperties()Ljava/util/Map;` | `sun/net/www/MessageHeader` |
| `getRequestProperty(Ljava/lang/String;)Ljava/lang/String;` | `sun/net/www/MessageHeader` |
| `setRequestProperty(Ljava/lang/String;Ljava/lang/String;)V` | `sun/net/www/MessageHeader` |

### `java/net/URLConnection$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/net/www/MimeTable` |

### `java/net/URLStreamHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `parseURL(Ljava/net/URL;Ljava/lang/String;II)V` | `sun/net/util/IPAddressUtil` |
| `setURL(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | `sun/net/util/IPAddressUtil` |

### `java/net/http/HttpClient`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newBuilder()Ljava/net/http/HttpClient$Builder;` | `jdk/internal/net/http/HttpClientBuilderImpl` |

### `java/net/http/HttpRequest`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newBuilder()Ljava/net/http/HttpRequest$Builder;` | `jdk/internal/net/http/HttpRequestBuilderImpl` |
| `newBuilder(Ljava/net/URI;)Ljava/net/http/HttpRequest$Builder;` | `jdk/internal/net/http/HttpRequestBuilderImpl` |

### `java/net/http/HttpRequest$BodyPublishers`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `concat([Ljava/net/http/HttpRequest$BodyPublisher;)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers` |
| `fromPublisher(Ljava/util/concurrent/Flow$Publisher;)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$PublisherAdapter` |
| `fromPublisher(Ljava/util/concurrent/Flow$Publisher;J)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$PublisherAdapter` |
| `noBody()Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$EmptyPublisher` |
| `ofByteArray([B)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$ByteArrayPublisher` |
| `ofByteArray([BII)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$ByteArrayPublisher` |
| `ofByteArrays(Ljava/lang/Iterable;)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$IterablePublisher` |
| `ofFile(Ljava/nio/file/Path;)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$FilePublisher` |
| `ofInputStream(Ljava/util/function/Supplier;)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$InputStreamPublisher` |
| `ofString(Ljava/lang/String;Ljava/nio/charset/Charset;)Ljava/net/http/HttpRequest$BodyPublisher;` | `jdk/internal/net/http/RequestPublishers$StringPublisher` |

### `java/net/http/HttpResponse$BodyHandlers`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$fromLineSubscriber$4(Ljava/util/concurrent/Flow$Subscriber;Ljava/net/http/HttpResponse$ResponseInfo;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/common/Utils` |
| `lambda$fromLineSubscriber$5(Ljava/util/concurrent/Flow$Subscriber;Ljava/util/function/Function;Ljava/lang/String;Ljava/net/http/HttpResponse$ResponseInfo;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/common/Utils` |
| `lambda$ofLines$10(Ljava/net/http/HttpResponse$ResponseInfo;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/common/Utils` |
| `lambda$ofString$13(Ljava/net/http/HttpResponse$ResponseInfo;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/common/Utils` |
| `ofFile(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/net/http/HttpResponse$BodyHandler;` | `jdk/internal/net/http/ResponseBodyHandlers$PathBodyHandler` |
| `ofFileDownload(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/net/http/HttpResponse$BodyHandler;` | `jdk/internal/net/http/ResponseBodyHandlers$FileDownloadBodyHandler` |

### `java/net/http/HttpResponse$BodySubscribers`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `buffering(Ljava/net/http/HttpResponse$BodySubscriber;I)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/BufferingSubscriber` |
| `discarding()Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$NullSubscriber` |
| `fromLineSubscriber(Ljava/util/concurrent/Flow$Subscriber;Ljava/util/function/Function;Ljava/nio/charset/Charset;Ljava/lang/String;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/LineSubscriberAdapter` |
| `fromSubscriber(Ljava/util/concurrent/Flow$Subscriber;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$SubscriberAdapter` |
| `fromSubscriber(Ljava/util/concurrent/Flow$Subscriber;Ljava/util/function/Function;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$SubscriberAdapter` |
| `mapping(Ljava/net/http/HttpResponse$BodySubscriber;Ljava/util/function/Function;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$MappingSubscriber` |
| `ofByteArray()Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$ByteArraySubscriber` |
| `ofByteArrayConsumer(Ljava/util/function/Consumer;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$ConsumerSubscriber` |
| `ofFile(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$PathSubscriber` |
| `ofInputStream()Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$HttpResponseInputStream` |
| `ofLines(Ljava/nio/charset/Charset;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers` |
| `ofPublisher()Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers` |
| `ofString(Ljava/nio/charset/Charset;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$ByteArraySubscriber` |
| `replacing(Ljava/lang/Object;)Ljava/net/http/HttpResponse$BodySubscriber;` | `jdk/internal/net/http/ResponseSubscribers$NullSubscriber` |

### `java/net/http/HttpResponse$PushPromiseHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `of(Ljava/util/function/Function;Ljava/util/concurrent/ConcurrentMap;)Ljava/net/http/HttpResponse$PushPromiseHandler;` | `jdk/internal/net/http/ResponseBodyHandlers$PushPromisesHandlerWithMap` |

### `java/nio/Bits`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `pageSize()I` | `jdk/internal/misc/Unsafe` |
| `reserveMemory(JJ)V` | `jdk/internal/access/JavaLangRefAccess`, `jdk/internal/access/SharedSecrets`, `jdk/internal/misc/VM` |

### `java/nio/Buffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkIndex(I)I` | `jdk/internal/util/Preconditions` |
| `checkIndex(II)I` | `jdk/internal/util/Preconditions` |
| `checkSession()V` | `jdk/internal/foreign/MemorySessionImpl` |
| `session()Ljdk/internal/foreign/MemorySessionImpl;` | `jdk/internal/foreign/AbstractMemorySegmentImpl` |

### `java/nio/Buffer$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireSession(Ljava/nio/Buffer;)V` | `jdk/internal/foreign/MemorySessionImpl` |
| `isThreadConfined(Ljava/nio/Buffer;)Z` | `jdk/internal/foreign/MemorySessionImpl` |
| `newMappedByteBuffer(Ljdk/internal/access/foreign/UnmapperProxy;JILjava/lang/Object;Ljava/lang/foreign/MemorySegment;)Ljava/nio/ByteBuffer;` | `jdk/internal/access/foreign/UnmapperProxy` |
| `releaseSession(Ljava/nio/Buffer;)V` | `jdk/internal/foreign/MemorySessionImpl` |

### `java/nio/BufferMismatch`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mismatch(Ljava/nio/ByteBuffer;ILjava/nio/ByteBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/util/ArraysSupport` |
| `mismatch(Ljava/nio/CharBuffer;ILjava/nio/CharBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/util/ArraysSupport` |
| `mismatch(Ljava/nio/DoubleBuffer;ILjava/nio/DoubleBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/util/ArraysSupport` |
| `mismatch(Ljava/nio/FloatBuffer;ILjava/nio/FloatBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/util/ArraysSupport` |
| `mismatch(Ljava/nio/IntBuffer;ILjava/nio/IntBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/util/ArraysSupport` |
| `mismatch(Ljava/nio/LongBuffer;ILjava/nio/LongBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/util/ArraysSupport` |
| `mismatch(Ljava/nio/ShortBuffer;ILjava/nio/ShortBuffer;II)I` | `jdk/internal/misc/ScopedMemoryAccess`, `jdk/internal/util/ArraysSupport` |

### `java/nio/ByteBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArray(I[BII)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putArray(I[BII)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putBuffer(ILjava/nio/ByteBuffer;II)V` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsCharBufferB`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getUnchecked(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(C)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IC)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsCharBufferL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getUnchecked(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(C)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IC)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsDoubleBufferB`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(D)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(ID)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsDoubleBufferL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(D)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(ID)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsFloatBufferB`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(F)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IF)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsFloatBufferL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(F)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IF)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsIntBufferB`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(I)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(II)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsIntBufferL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(I)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(II)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsLongBufferB`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IJ)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(J)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsLongBufferL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IJ)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(J)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsShortBufferB`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IS)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(S)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/ByteBufferAsShortBufferL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get()S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IS)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(S)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/CharBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArray(I[CII)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putArray(I[CII)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putBuffer(ILjava/nio/CharBuffer;II)V` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectByteBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VM`, `jdk/internal/ref/Cleaner` |
| `<init>(IJLjava/io/FileDescriptor;Ljava/lang/Runnable;ZLjava/lang/foreign/MemorySegment;)V` | `jdk/internal/ref/Cleaner` |
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/io/FileDescriptor;ZLjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/MappedByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()B` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)B` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getChar(J)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getDouble(J)D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getFloat(J)F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getInt(J)I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getLong(J)J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getShort(J)S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(B)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IB)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putChar(JC)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putDouble(JD)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putFloat(JF)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putInt(JI)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putLong(JJ)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putShort(JS)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectByteBuffer$Deallocator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `jdk/internal/misc/Unsafe` |

### `java/nio/DirectCharBufferS`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getUnchecked(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(C)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IC)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectCharBufferU`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getUnchecked(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(C)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IC)Ljava/nio/CharBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectDoubleBufferS`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(D)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(ID)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectDoubleBufferU`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(D)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(ID)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectFloatBufferS`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(F)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IF)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectFloatBufferU`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(F)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IF)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectIntBufferS`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(I)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(II)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectIntBufferU`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(I)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(II)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectLongBufferS`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IJ)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(J)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectLongBufferU`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IJ)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(J)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectShortBufferS`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IS)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(S)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DirectShortBufferU`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/nio/ch/DirectBuffer;IIIIILjava/lang/foreign/MemorySegment;)V` | `sun/nio/ch/DirectBuffer` |
| `address()J` | `jdk/internal/foreign/MemorySessionImpl` |
| `compact()Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get()S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `get(I)S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(IS)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `put(S)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/DoubleBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArray(I[DII)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putArray(I[DII)Ljava/nio/DoubleBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putBuffer(ILjava/nio/DoubleBuffer;II)V` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/FloatBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArray(I[FII)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putArray(I[FII)Ljava/nio/FloatBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putBuffer(ILjava/nio/FloatBuffer;II)V` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/HeapByteBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getChar()C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getChar(I)C` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getDouble()D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getDouble(I)D` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getFloat()F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getFloat(I)F` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getInt()I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getInt(I)I` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getLong()J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getLong(I)J` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getShort()S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `getShort(I)S` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putChar(C)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putChar(IC)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putDouble(D)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putDouble(ID)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putFloat(F)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putFloat(IF)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putInt(I)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putInt(II)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putLong(IJ)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putLong(J)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putShort(IS)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putShort(S)Ljava/nio/ByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/IntBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArray(I[III)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putArray(I[III)Ljava/nio/IntBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putBuffer(ILjava/nio/IntBuffer;II)V` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/LongBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArray(I[JII)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putArray(I[JII)Ljava/nio/LongBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putBuffer(ILjava/nio/LongBuffer;II)V` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/MappedByteBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `force(II)Ljava/nio/MappedByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `isLoaded()Z` | `jdk/internal/misc/ScopedMemoryAccess` |
| `load()Ljava/nio/MappedByteBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/MappedByteBuffer$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `unmap()V` | `jdk/internal/misc/Unsafe` |

### `java/nio/MappedMemoryUtils`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `force(Ljava/io/FileDescriptor;JZJJ)V` | `jdk/internal/misc/Blocker`, `jdk/internal/misc/Unsafe` |
| `load(JZJ)V` | `jdk/internal/misc/Unsafe` |

### `java/nio/ShortBuffer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArray(I[SII)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putArray(I[SII)Ljava/nio/ShortBuffer;` | `jdk/internal/misc/ScopedMemoryAccess` |
| `putBuffer(ILjava/nio/ShortBuffer;II)V` | `jdk/internal/misc/ScopedMemoryAccess` |

### `java/nio/channels/Channels`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newInputStream(Ljava/nio/channels/ReadableByteChannel;)Ljava/io/InputStream;` | `sun/nio/ch/Streams` |
| `newOutputStream(Ljava/nio/channels/WritableByteChannel;)Ljava/io/OutputStream;` | `sun/nio/ch/Streams` |
| `newReader(Ljava/nio/channels/ReadableByteChannel;Ljava/nio/charset/CharsetDecoder;I)Ljava/io/Reader;` | `sun/nio/cs/StreamDecoder` |
| `newWriter(Ljava/nio/channels/WritableByteChannel;Ljava/nio/charset/CharsetEncoder;I)Ljava/io/Writer;` | `sun/nio/cs/StreamEncoder` |

### `java/nio/channels/spi/AbstractInterruptibleChannel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `begin()V` | `sun/nio/ch/Interruptible` |
| `blockedOn(Lsun/nio/ch/Interruptible;)V` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |

### `java/nio/channels/spi/AbstractSelectionKey`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `cancel()V` | `sun/nio/ch/SelectorImpl` |

### `java/nio/channels/spi/AbstractSelector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `begin()V` | `sun/nio/ch/Interruptible` |

### `java/nio/channels/spi/AsynchronousChannelProvider$ProviderHolder$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/nio/channels/spi/AsynchronousChannelProvider;` | `sun/nio/ch/DefaultAsynchronousChannelProvider` |

### `java/nio/channels/spi/SelectorProvider$Holder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$provider$0()Ljava/nio/channels/spi/SelectorProvider;` | `sun/nio/ch/DefaultSelectorProvider` |

### `java/nio/charset/Charset`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;[Ljava/lang/String;)V` | `jdk/internal/misc/VM` |
| `decode(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;` | `sun/nio/cs/ThreadLocalCoders` |
| `defaultCharset()Ljava/nio/charset/Charset;` | `jdk/internal/util/StaticProperty`, `sun/nio/cs/UTF_8` |
| `encode(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;` | `sun/nio/cs/ThreadLocalCoders` |
| `endLookup(Ljava/lang/Object;)V` | `jdk/internal/misc/ThreadTracker` |
| `lookupExtendedCharset(Ljava/lang/String;)Ljava/nio/charset/Charset;` | `jdk/internal/misc/VM` |
| `lookupViaProviders(Ljava/lang/String;)Ljava/nio/charset/Charset;` | `jdk/internal/misc/VM` |
| `tryBeginLookup()Ljava/lang/Object;` | `jdk/internal/misc/ThreadTracker` |

### `java/nio/charset/CharsetDecoder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `decode(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;` | `jdk/internal/util/ArraysSupport` |

### `java/nio/charset/CharsetEncoder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `encode(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;` | `jdk/internal/util/ArraysSupport` |

### `java/nio/file/FileChannelLinesSpliterator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `unmap()V` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/access/SharedSecrets`, `jdk/internal/access/foreign/UnmapperProxy` |

### `java/nio/file/FileSystems`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefault()Ljava/nio/file/FileSystem;` | `jdk/internal/misc/VM`, `sun/nio/fs/DefaultFileSystemProvider` |

### `java/nio/file/FileSystems$DefaultFileSystemHolder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultProvider()Ljava/nio/file/spi/FileSystemProvider;` | `sun/nio/fs/DefaultFileSystemProvider` |

### `java/nio/file/FileTreeWalker`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAttributes(Ljava/nio/file/Path;Z)Ljava/nio/file/attribute/BasicFileAttributes;` | `sun/nio/fs/BasicFileAttributesHolder` |

### `java/nio/file/Files`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isExecutable(Ljava/nio/file/Path;)Z` | `sun/nio/fs/AbstractFileSystemProvider` |
| `isReadable(Ljava/nio/file/Path;)Z` | `sun/nio/fs/AbstractFileSystemProvider` |
| `isWritable(Ljava/nio/file/Path;)Z` | `sun/nio/fs/AbstractFileSystemProvider` |
| `lines(Ljava/nio/file/Path;)Ljava/util/stream/Stream;` | `sun/nio/cs/UTF_8` |
| `newBufferedReader(Ljava/nio/file/Path;)Ljava/io/BufferedReader;` | `sun/nio/cs/UTF_8` |
| `newBufferedWriter(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/io/BufferedWriter;` | `sun/nio/cs/UTF_8` |
| `read(Ljava/io/InputStream;I)[B` | `jdk/internal/util/ArraysSupport` |
| `readAllBytes(Ljava/nio/file/Path;)[B` | `sun/nio/ch/FileChannelImpl` |
| `readAllLines(Ljava/nio/file/Path;)Ljava/util/List;` | `sun/nio/cs/UTF_8` |
| `readString(Ljava/nio/file/Path;)Ljava/lang/String;` | `sun/nio/cs/UTF_8` |
| `readString(Ljava/nio/file/Path;Ljava/nio/charset/Charset;)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |
| `write(Ljava/nio/file/Path;Ljava/lang/Iterable;[Ljava/nio/file/OpenOption;)Ljava/nio/file/Path;` | `sun/nio/cs/UTF_8` |
| `writeString(Ljava/nio/file/Path;Ljava/lang/CharSequence;Ljava/nio/charset/Charset;[Ljava/nio/file/OpenOption;)Ljava/nio/file/Path;` | `jdk/internal/access/JavaLangAccess` |
| `writeString(Ljava/nio/file/Path;Ljava/lang/CharSequence;[Ljava/nio/file/OpenOption;)Ljava/nio/file/Path;` | `sun/nio/cs/UTF_8` |

### `java/nio/file/Files$FileTypeDetectors$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/nio/file/spi/FileTypeDetector;` | `sun/nio/fs/DefaultFileTypeDetector` |

### `java/nio/file/spi/FileSystemProvider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newInputStream(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/io/InputStream;` | `sun/nio/ch/FileChannelImpl` |
| `newOutputStream(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/io/OutputStream;` | `sun/nio/ch/FileChannelImpl` |

### `java/rmi/MarshalledObject$MarshalledObjectInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;Ljava/io/InputStream;Ljava/io/ObjectInputFilter;)V` | `sun/rmi/server/MarshalInputStream` |

### `java/rmi/MarshalledObject$MarshalledObjectOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/OutputStream;Ljava/io/OutputStream;)V` | `sun/rmi/server/MarshalOutputStream` |
| `flush()V` | `sun/rmi/server/MarshalOutputStream` |

### `java/rmi/registry/LocateRegistry`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createRegistry(I)Ljava/rmi/registry/Registry;` | `sun/rmi/registry/RegistryImpl` |
| `createRegistry(ILjava/rmi/server/RMIClientSocketFactory;Ljava/rmi/server/RMIServerSocketFactory;)Ljava/rmi/registry/Registry;` | `sun/rmi/registry/RegistryImpl` |
| `getRegistry(Ljava/lang/String;ILjava/rmi/server/RMIClientSocketFactory;)Ljava/rmi/registry/Registry;` | `sun/rmi/server/UnicastRef`, `sun/rmi/server/UnicastRef2`, `sun/rmi/server/Util`, `sun/rmi/transport/LiveRef`, +1 |

### `java/rmi/server/RMIClassLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getSecurityContext(Ljava/lang/ClassLoader;)Ljava/lang/Object;` | `sun/rmi/server/LoaderHandler` |

### `java/rmi/server/RMIClassLoader$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getClassAnnotation(Ljava/lang/Class;)Ljava/lang/String;` | `sun/rmi/server/LoaderHandler` |
| `getClassLoader(Ljava/lang/String;)Ljava/lang/ClassLoader;` | `sun/rmi/server/LoaderHandler` |
| `loadClass(Ljava/lang/String;Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;` | `sun/rmi/server/LoaderHandler` |
| `loadProxyClass(Ljava/lang/String;[Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;` | `sun/rmi/server/LoaderHandler` |

### `java/rmi/server/RMISocketFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultSocketFactory()Ljava/rmi/server/RMISocketFactory;` | `sun/rmi/transport/tcp/TCPDirectSocketFactory` |

### `java/rmi/server/RemoteObject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toString()Ljava/lang/String;` | `sun/rmi/server/Util` |
| `toStub(Ljava/rmi/Remote;)Ljava/rmi/Remote;` | `sun/rmi/transport/ObjectTable` |

### `java/rmi/server/RemoteObjectInvocationHandler$MethodToHash_Maps`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/rmi/server/WeakClassHashMap` |

### `java/rmi/server/RemoteObjectInvocationHandler$MethodToHash_Maps$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/lang/Object;)Ljava/lang/Long;` | `sun/rmi/server/Util` |

### `java/rmi/server/RemoteServer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getClientHost()Ljava/lang/String;` | `sun/rmi/transport/tcp/TCPTransport` |
| `getLog()Ljava/io/PrintStream;` | `sun/rmi/runtime/Log`, `sun/rmi/server/UnicastServerRef` |
| `setLog(Ljava/io/OutputStream;)V` | `sun/rmi/runtime/Log`, `sun/rmi/server/UnicastServerRef` |

### `java/rmi/server/UnicastRemoteObject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `exportObject(Ljava/rmi/Remote;)Ljava/rmi/server/RemoteStub;` | `sun/rmi/server/UnicastServerRef` |
| `exportObject(Ljava/rmi/Remote;I)Ljava/rmi/Remote;` | `sun/rmi/server/UnicastServerRef` |
| `exportObject(Ljava/rmi/Remote;ILjava/io/ObjectInputFilter;)Ljava/rmi/Remote;` | `sun/rmi/server/UnicastServerRef`, `sun/rmi/transport/LiveRef` |
| `exportObject(Ljava/rmi/Remote;ILjava/rmi/server/RMIClientSocketFactory;Ljava/rmi/server/RMIServerSocketFactory;)Ljava/rmi/Remote;` | `sun/rmi/server/UnicastServerRef2` |
| `exportObject(Ljava/rmi/Remote;ILjava/rmi/server/RMIClientSocketFactory;Ljava/rmi/server/RMIServerSocketFactory;Ljava/io/ObjectInputFilter;)Ljava/rmi/Remote;` | `sun/rmi/server/UnicastServerRef2` |
| `exportObject(Ljava/rmi/Remote;Lsun/rmi/server/UnicastServerRef;)Ljava/rmi/Remote;` | `sun/rmi/server/UnicastServerRef` |
| `unexportObject(Ljava/rmi/Remote;Z)Z` | `sun/rmi/transport/ObjectTable` |

### `java/security/AccessControlContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/security/AccessControlContext;Ljava/security/DomainCombiner;Z)V` | `sun/security/util/SecurityConstants` |
| `<init>(Ljava/security/ProtectionDomain;Ljava/security/DomainCombiner;Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)V` | `sun/security/util/FilePermCompat` |
| `checkPermission(Ljava/security/Permission;)V` | `sun/security/util/Debug` |
| `getDebug()Lsun/security/util/Debug;` | `sun/security/util/Debug` |
| `getDomainCombiner()Ljava/security/DomainCombiner;` | `sun/security/util/SecurityConstants` |
| `optimize()Ljava/security/AccessControlContext;` | `sun/security/util/Debug` |

### `java/security/AccessControlContext$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `sun/security/util/Debug` |

### `java/security/AccessController`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkContext(Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/security/AccessControlContext;` | `sun/security/util/SecurityConstants` |
| `checkPermission(Ljava/security/Permission;)V` | `sun/security/util/Debug` |
| `createWrapper(Ljava/security/DomainCombiner;Ljava/lang/Class;Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/security/AccessControlContext;` | `sun/security/util/SecurityConstants` |
| `doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivileged(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivileged(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivileged(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivilegedWithCombiner(Ljava/security/PrivilegedAction;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivilegedWithCombiner(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivilegedWithCombiner(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |
| `doPrivilegedWithCombiner(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;` | `jdk/internal/reflect/Reflection` |

### `java/security/AlgorithmParameterGenerator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `init(I)V` | `sun/security/jca/JCAUtil` |
| `init(Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/jca/JCAUtil` |

### `java/security/AllPermissionCollection$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nextElement()Ljava/security/Permission;` | `sun/security/util/SecurityConstants` |

### `java/security/CodeSource`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/net/URL;[Ljava/security/CodeSigner;)V` | `sun/net/util/URLUtil` |
| `<init>(Ljava/net/URL;[Ljava/security/cert/Certificate;)V` | `sun/net/util/URLUtil` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/net/util/URLUtil`, `sun/security/util/IOUtils` |

### `java/security/KeyFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/KeyFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `java/security/KeyPairGenerator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljava/security/KeyPairGenerator;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyPairGenerator;` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/KeyPairGenerator;` | `sun/security/jca/GetInstance` |
| `getInstance(Lsun/security/jca/GetInstance$Instance;Ljava/lang/String;)Ljava/security/KeyPairGenerator;` | `sun/security/jca/GetInstance$Instance`, `sun/security/util/Debug` |
| `initialize(I)V` | `sun/security/jca/JCAUtil` |
| `initialize(Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/jca/JCAUtil` |

### `java/security/KeyPairGenerator$Delegate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/security/jca/GetInstance$Instance;Ljava/util/Iterator;Ljava/lang/String;)V` | `sun/security/jca/GetInstance$Instance`, `sun/security/util/Debug` |

### `java/security/KeyStore`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/security/KeyStoreSpi;Ljava/security/Provider;Ljava/lang/String;)V` | `sun/security/util/Debug` |
| `getInstance(Ljava/io/File;[CLjava/security/KeyStore$LoadStoreParameter;Z)Ljava/security/KeyStore;` | `sun/security/util/CryptoAlgorithmConstraints`, `sun/security/util/Debug` |
| `getInstance(Ljava/lang/String;)Ljava/security/KeyStore;` | `sun/security/util/CryptoAlgorithmConstraints` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyStore;` | `sun/security/util/CryptoAlgorithmConstraints` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/KeyStore;` | `sun/security/util/CryptoAlgorithmConstraints` |

### `java/security/MessageDigest`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljava/security/MessageDigest;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance`, `sun/security/util/CryptoAlgorithmConstraints`, `sun/security/util/Debug` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/MessageDigest;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance`, `sun/security/util/CryptoAlgorithmConstraints` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/MessageDigest;` | `sun/security/util/CryptoAlgorithmConstraints` |

### `java/security/MessageDigest$Delegate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `engineUpdate(Ljavax/crypto/SecretKey;)V` | `sun/security/util/MessageDigestSpi2` |

### `java/security/MessageDigestSpi`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `engineUpdate(Ljava/nio/ByteBuffer;)V` | `sun/security/jca/JCAUtil` |

### `java/security/PKCS12Attribute`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;)V` | `sun/security/util/ObjectIdentifier` |
| `encode(Lsun/security/util/ObjectIdentifier;[Ljava/lang/String;)[B` | `sun/security/util/DerOutputStream` |
| `parse([B)V` | `sun/security/util/Debug`, `sun/security/util/DerInputStream`, `sun/security/util/DerValue`, `sun/security/util/ObjectIdentifier` |

### `java/security/Policy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;Ljava/security/Policy$Parameters;)Ljava/security/Policy;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Policy$Parameters;Ljava/lang/String;)Ljava/security/Policy;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Policy$Parameters;Ljava/security/Provider;)Ljava/security/Policy;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getPolicy()Ljava/security/Policy;` | `sun/security/util/SecurityConstants` |
| `initPolicy(Ljava/security/Policy;)V` | `sun/security/util/SecurityConstants` |
| `loadPolicyProvider()Ljava/security/Policy;` | `sun/security/provider/PolicyFile`, `sun/security/util/Debug` |

### `java/security/Policy$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/security/Policy;` | `sun/security/util/Debug` |

### `java/security/PrivilegedActionException`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |

### `java/security/ProtectionDomain`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `impliesWithAltFilePerm(Ljava/security/Permission;)Z` | `sun/security/util/FilePermCompat` |
| `seeAllp()Z` | `sun/security/util/SecurityConstants` |

### `java/security/Provider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkAndUpdateSecureRandom(Ljava/lang/String;Ljava/lang/String;Z)V` | `sun/security/util/Debug` |
| `clear()V` | `sun/security/util/Debug` |
| `compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `getService(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service;` | `jdk/internal/event/SecurityProviderServiceEvent` |
| `getTypeAndAlgorithm(Ljava/lang/String;)[Ljava/lang/String;` | `sun/security/util/Debug` |
| `load(Ljava/io/InputStream;)V` | `sun/security/util/Debug` |
| `merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `putAll(Ljava/util/Map;)V` | `sun/security/util/Debug` |
| `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `putService(Ljava/security/Provider$Service;)V` | `sun/security/util/Debug` |
| `remove(Ljava/lang/Object;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `remove(Ljava/lang/Object;Ljava/lang/Object;)Z` | `sun/security/util/Debug` |
| `removeService(Ljava/security/Provider$Service;)V` | `sun/security/util/Debug` |
| `replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `sun/security/util/Debug` |
| `replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `sun/security/util/Debug` |
| `replaceAll(Ljava/util/function/BiFunction;)V` | `sun/security/util/Debug` |

### `java/security/SecureClassLoader$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `apply(Ljava/security/SecureClassLoader$CodeSourceKey;)Ljava/security/ProtectionDomain;` | `sun/security/util/Debug` |

### `java/security/SecureRandom`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/security/SecureRandomSpi;Ljava/security/Provider;Ljava/lang/String;)V` | `sun/security/util/Debug` |
| `getDefaultPRNG(Z[B)V` | `sun/security/jca/ProviderList`, `sun/security/jca/Providers`, `sun/security/provider/SecureRandom`, `sun/security/provider/SunEntries` |
| `getInstance(Ljava/lang/String;)Ljava/security/SecureRandom;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/SecureRandom;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/SecureRandom;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/SecureRandomParameters;)Ljava/security/SecureRandom;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/SecureRandomParameters;Ljava/lang/String;)Ljava/security/SecureRandom;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/SecureRandomParameters;Ljava/security/Provider;)Ljava/security/SecureRandom;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `java/security/Security`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getImpl(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/Object;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getImpl(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)[Ljava/lang/Object;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getImpl(Ljava/lang/String;Ljava/lang/String;Ljava/security/Provider;)[Ljava/lang/Object;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getImpl(Ljava/lang/String;Ljava/lang/String;Ljava/security/Provider;Ljava/lang/Object;)[Ljava/lang/Object;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getProvider(Ljava/lang/String;)Ljava/security/Provider;` | `sun/security/jca/ProviderList`, `sun/security/jca/Providers` |
| `getProviderProperty(Ljava/lang/String;)Ljava/security/Security$ProviderProperty;` | `sun/security/jca/ProviderList`, `sun/security/jca/Providers` |
| `getProviders()[Ljava/security/Provider;` | `sun/security/jca/ProviderList`, `sun/security/jca/Providers` |
| `initialize()V` | `sun/security/util/Debug` |
| `insertProviderAt(Ljava/security/Provider;I)I` | `sun/security/jca/ProviderList`, `sun/security/jca/Providers` |
| `invalidateSMCache(Ljava/lang/String;)V` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |
| `loadProps(Ljava/io/File;Ljava/lang/String;Z)Z` | `sun/security/util/Debug`, `sun/security/util/PropertyExpander` |
| `removeProvider(Ljava/lang/String;)V` | `sun/security/jca/ProviderList`, `sun/security/jca/Providers` |
| `securityPropFile(Ljava/lang/String;)Ljava/io/File;` | `jdk/internal/util/StaticProperty` |
| `setProperty(Ljava/lang/String;Ljava/lang/String;)V` | `jdk/internal/event/EventHelper`, `jdk/internal/event/SecurityPropertyModificationEvent` |

### `java/security/Signature`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljava/security/Signature;` | `sun/security/jca/GetInstance`, `sun/security/util/CryptoAlgorithmConstraints` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Signature;` | `sun/security/jca/GetInstance`, `sun/security/util/CryptoAlgorithmConstraints` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/Signature;` | `sun/security/jca/GetInstance`, `sun/security/util/CryptoAlgorithmConstraints` |
| `getInstance(Lsun/security/jca/GetInstance$Instance;Ljava/lang/String;)Ljava/security/Signature;` | `sun/security/jca/GetInstance$Instance` |
| `getPublicKeyFromCert(Ljava/security/cert/Certificate;)Ljava/security/PublicKey;` | `sun/security/util/KnownOIDs` |
| `initSign(Ljava/security/PrivateKey;)V` | `sun/security/util/Debug` |
| `initSign(Ljava/security/PrivateKey;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug` |
| `initSign(Ljava/security/PrivateKey;Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug` |
| `initVerify(Ljava/security/PublicKey;)V` | `sun/security/util/Debug` |
| `initVerify(Ljava/security/PublicKey;Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/util/Debug` |
| `initVerify(Ljava/security/cert/Certificate;)V` | `sun/security/util/Debug` |
| `initVerify(Ljava/security/cert/Certificate;Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/util/Debug` |
| `isSpi(Ljava/security/Provider$Service;)Z` | `sun/security/util/Debug` |

### `java/security/Signature$Delegate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `chooseFirstProvider()V` | `sun/security/util/Debug` |

### `java/security/SignatureSpi`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `engineUpdate(Ljava/nio/ByteBuffer;)V` | `sun/security/jca/JCAUtil` |

### `java/security/UnresolvedPermission`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/security/util/IOUtils` |
| `resolve(Ljava/security/Permission;[Ljava/security/cert/Certificate;)Ljava/security/Permission;` | `sun/security/util/Debug` |

### `java/security/cert/CertPathBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljava/security/cert/CertPathBuilder;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/cert/CertPathBuilder;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/cert/CertPathBuilder;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `java/security/cert/CertPathHelperImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/security/provider/certpath/CertPathHelper` |
| `initialize()V` | `sun/security/provider/certpath/CertPathHelper` |

### `java/security/cert/CertPathValidator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljava/security/cert/CertPathValidator;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/cert/CertPathValidator;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/cert/CertPathValidator;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `java/security/cert/CertStore`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;Ljava/security/cert/CertStoreParameters;)Ljava/security/cert/CertStore;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/cert/CertStoreParameters;Ljava/lang/String;)Ljava/security/cert/CertStore;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/cert/CertStoreParameters;Ljava/security/Provider;)Ljava/security/cert/CertStore;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `java/security/cert/Certificate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `equals(Ljava/lang/Object;)Z` | `sun/security/x509/X509CertImpl` |
| `hashCode()I` | `sun/security/x509/X509CertImpl` |

### `java/security/cert/CertificateFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `generateCertificate(Ljava/io/InputStream;)Ljava/security/cert/Certificate;` | `sun/security/jca/JCAUtil` |
| `getInstance(Ljava/lang/String;)Ljava/security/cert/CertificateFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljava/security/cert/CertificateFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/cert/CertificateFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `java/security/cert/CertificateRevokedException`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInvalidityDate()Ljava/util/Date;` | `sun/security/util/KnownOIDs`, `sun/security/x509/InvalidityDateExtension` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/security/util/IOUtils`, `sun/security/util/ObjectIdentifier`, `sun/security/x509/Extension` |

### `java/security/cert/PolicyQualifierInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>([B)V` | `sun/security/util/DerInputStream`, `sun/security/util/DerValue`, `sun/security/util/ObjectIdentifier` |
| `toString()Ljava/lang/String;` | `sun/security/util/HexDumpEncoder` |

### `java/security/cert/TrustAnchor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isJdkCA()Z` | `sun/security/util/AnchorCertificates` |
| `setNameConstraints([B)V` | `sun/security/x509/NameConstraintsExtension` |

### `java/security/cert/X509CRL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `equals(Ljava/lang/Object;)Z` | `sun/security/x509/X509CRLImpl` |
| `getIssuerX500Principal()Ljavax/security/auth/x500/X500Principal;` | `sun/security/x509/X509CRLImpl` |
| `hashCode()I` | `sun/security/x509/X509CRLImpl` |
| `verify(Ljava/security/PublicKey;Ljava/security/Provider;)V` | `sun/security/util/SignatureUtil` |

### `java/security/cert/X509CRLEntry`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getRevocationReason()Ljava/security/cert/CRLReason;` | `sun/security/x509/X509CRLEntryImpl` |

### `java/security/cert/X509CRLSelector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addIssuerName(Ljava/lang/String;)V` | `sun/security/x509/X500Name` |
| `addIssuerName([B)V` | `sun/security/x509/X500Name` |
| `match(Ljava/security/cert/CRL;)Z` | `sun/security/util/Debug`, `sun/security/util/DerInputStream`, `sun/security/util/KnownOIDs`, `sun/security/x509/CRLNumberExtension` |
| `parseIssuerNames(Ljava/util/Collection;)Ljava/util/HashSet;` | `sun/security/x509/X500Name` |

### `java/security/cert/X509CertSelector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `cloneAndCheckNames(Ljava/util/Collection;)Ljava/util/Set;` | `sun/security/util/Debug` |
| `getExtensionObject(Ljava/security/cert/X509Certificate;Lsun/security/util/KnownOIDs;)Ljava/security/cert/Extension;` | `sun/security/util/DerInputStream`, `sun/security/util/KnownOIDs`, `sun/security/x509/CertificatePoliciesExtension`, `sun/security/x509/ExtendedKeyUsageExtension`, +4 |
| `getSubjectPublicKeyAlgID()Ljava/lang/String;` | `sun/security/util/ObjectIdentifier` |
| `makeGeneralNameInterface(ILjava/lang/Object;)Lsun/security/x509/GeneralNameInterface;` | `sun/security/util/Debug`, `sun/security/util/DerValue`, `sun/security/x509/DNSName`, `sun/security/x509/EDIPartyName`, +8 |
| `match(Ljava/security/cert/Certificate;)Z` | `sun/security/util/Debug` |
| `matchAuthorityKeyID(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/DerInputStream` |
| `matchBasicConstraints(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug` |
| `matchExcluded(Lsun/security/x509/GeneralSubtrees;)Z` | `sun/security/util/Debug`, `sun/security/x509/GeneralName`, `sun/security/x509/GeneralNameInterface`, `sun/security/x509/GeneralSubtree`, +1 |
| `matchExtendedKeyUsage(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/KnownOIDs`, `sun/security/x509/ExtendedKeyUsageExtension` |
| `matchKeyUsage(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug` |
| `matchNameConstraints(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/x509/NameConstraintsExtension` |
| `matchPathToNames(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/KnownOIDs`, `sun/security/x509/NameConstraintsExtension` |
| `matchPermitted(Lsun/security/x509/GeneralSubtrees;)Z` | `sun/security/util/Debug`, `sun/security/x509/GeneralName`, `sun/security/x509/GeneralNameInterface`, `sun/security/x509/GeneralSubtree`, +1 |
| `matchPolicy(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/KnownOIDs`, `sun/security/x509/CertificatePoliciesExtension`, `sun/security/x509/CertificatePolicySet`, +1 |
| `matchPrivateKeyValid(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/KnownOIDs`, `sun/security/x509/PrivateKeyUsageExtension` |
| `matchSubjectAlternativeNames(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/KnownOIDs`, `sun/security/x509/GeneralName`, `sun/security/x509/GeneralNameInterface`, +2 |
| `matchSubjectKeyID(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/DerInputStream` |
| `matchSubjectPublicKeyAlgID(Ljava/security/cert/X509Certificate;)Z` | `sun/security/util/Debug`, `sun/security/util/DerInputStream`, `sun/security/util/DerValue`, `sun/security/util/ObjectIdentifier`, +1 |
| `setExtendedKeyUsage(Ljava/util/Set;)V` | `sun/security/util/ObjectIdentifier` |
| `setIssuer(Ljava/lang/String;)V` | `sun/security/x509/X500Name` |
| `setNameConstraints([B)V` | `sun/security/x509/NameConstraintsExtension` |
| `setPolicy(Ljava/util/Set;)V` | `sun/security/util/ObjectIdentifier`, `sun/security/x509/CertificatePolicyId`, `sun/security/x509/CertificatePolicySet` |
| `setSubject(Ljava/lang/String;)V` | `sun/security/x509/X500Name` |
| `setSubjectPublicKey([B)V` | `sun/security/util/DerValue`, `sun/security/x509/X509Key` |
| `setSubjectPublicKeyAlgID(Ljava/lang/String;)V` | `sun/security/util/ObjectIdentifier` |
| `toString()Ljava/lang/String;` | `sun/security/util/HexDumpEncoder` |

### `java/security/cert/X509Certificate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getExtendedKeyUsage()Ljava/util/List;` | `sun/security/x509/X509CertImpl` |
| `getIssuerAlternativeNames()Ljava/util/Collection;` | `sun/security/x509/X509CertImpl` |
| `getIssuerX500Principal()Ljavax/security/auth/x500/X500Principal;` | `sun/security/x509/X509CertImpl` |
| `getSubjectAlternativeNames()Ljava/util/Collection;` | `sun/security/x509/X509CertImpl` |
| `getSubjectX500Principal()Ljavax/security/auth/x500/X500Principal;` | `sun/security/x509/X509CertImpl` |
| `verify(Ljava/security/PublicKey;Ljava/security/Provider;)V` | `sun/security/util/SignatureUtil` |

### `java/sql/DriverManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `deregisterDriver(Ljava/sql/Driver;)V` | `jdk/internal/reflect/Reflection` |
| `drivers()Ljava/util/stream/Stream;` | `jdk/internal/reflect/Reflection` |
| `getConnection(Ljava/lang/String;)Ljava/sql/Connection;` | `jdk/internal/reflect/Reflection` |
| `getConnection(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/sql/Connection;` | `jdk/internal/reflect/Reflection` |
| `getConnection(Ljava/lang/String;Ljava/util/Properties;)Ljava/sql/Connection;` | `jdk/internal/reflect/Reflection` |
| `getDriver(Ljava/lang/String;)Ljava/sql/Driver;` | `jdk/internal/reflect/Reflection` |
| `getDrivers()Ljava/util/Enumeration;` | `jdk/internal/reflect/Reflection` |

### `java/text/Bidi`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;I)V` | `jdk/internal/icu/text/BidiBase` |
| `<init>(Ljava/text/AttributedCharacterIterator;)V` | `jdk/internal/icu/text/BidiBase` |
| `<init>([CI[BIII)V` | `jdk/internal/icu/text/BidiBase` |
| `baseIsLeftToRight()Z` | `jdk/internal/icu/text/BidiBase` |
| `createLineBidi(II)Ljava/text/Bidi;` | `jdk/internal/icu/text/BidiBase` |
| `getBaseLevel()I` | `jdk/internal/icu/text/BidiBase` |
| `getLength()I` | `jdk/internal/icu/text/BidiBase` |
| `getLevelAt(I)I` | `jdk/internal/icu/text/BidiBase` |
| `getRunCount()I` | `jdk/internal/icu/text/BidiBase` |
| `getRunLevel(I)I` | `jdk/internal/icu/text/BidiBase` |
| `getRunLimit(I)I` | `jdk/internal/icu/text/BidiBase` |
| `getRunStart(I)I` | `jdk/internal/icu/text/BidiBase` |
| `isLeftToRight()Z` | `jdk/internal/icu/text/BidiBase` |
| `isMixed()Z` | `jdk/internal/icu/text/BidiBase` |
| `isRightToLeft()Z` | `jdk/internal/icu/text/BidiBase` |
| `reorderVisually([BI[Ljava/lang/Object;II)V` | `jdk/internal/icu/text/BidiBase` |
| `requiresBidi([CII)Z` | `jdk/internal/icu/text/BidiBase` |
| `toString()Ljava/lang/String;` | `jdk/internal/icu/text/BidiBase` |

### `java/text/BreakIterator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createBreakInstance(Ljava/util/Locale;I)Ljava/text/BreakIterator;` | `sun/util/locale/provider/LocaleProviderAdapter` |
| `createBreakInstance(Lsun/util/locale/provider/LocaleProviderAdapter;Ljava/util/Locale;I)Ljava/text/BreakIterator;` | `sun/util/locale/provider/LocaleProviderAdapter` |
| `getAvailableLocales()[Ljava/util/Locale;` | `sun/util/locale/provider/LocaleServiceProviderPool` |

### `java/text/CollationElementIterator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/text/RuleBasedCollator;)V` | `jdk/internal/icu/text/NormalizerBase`, `sun/text/CollatorUtilities` |
| `<init>(Ljava/text/CharacterIterator;Ljava/text/RuleBasedCollator;)V` | `jdk/internal/icu/text/NormalizerBase`, `sun/text/CollatorUtilities` |
| `getOffset()I` | `jdk/internal/icu/text/NormalizerBase` |
| `next()I` | `jdk/internal/icu/text/NormalizerBase`, `sun/text/CollatorUtilities` |
| `nextContractChar(I)I` | `jdk/internal/icu/text/NormalizerBase` |
| `prevContractChar(I)I` | `jdk/internal/icu/text/NormalizerBase` |
| `previous()I` | `jdk/internal/icu/text/NormalizerBase`, `sun/text/CollatorUtilities` |
| `reset()V` | `jdk/internal/icu/text/NormalizerBase`, `sun/text/CollatorUtilities` |
| `setOffset(I)V` | `jdk/internal/icu/text/NormalizerBase` |
| `setText(Ljava/lang/String;)V` | `jdk/internal/icu/text/NormalizerBase`, `sun/text/CollatorUtilities` |
| `setText(Ljava/text/CharacterIterator;)V` | `jdk/internal/icu/text/NormalizerBase`, `sun/text/CollatorUtilities` |

### `java/text/Collator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAvailableLocales()[Ljava/util/Locale;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `getInstance(Ljava/util/Locale;)Ljava/text/Collator;` | `sun/util/locale/provider/LocaleProviderAdapter` |

### `java/text/DateFormat`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(IIILjava/util/Locale;)Ljava/text/DateFormat;` | `sun/util/locale/provider/LocaleProviderAdapter` |
| `get(Lsun/util/locale/provider/LocaleProviderAdapter;IILjava/util/Locale;)Ljava/text/DateFormat;` | `sun/util/locale/provider/LocaleProviderAdapter` |
| `getAvailableLocales()[Ljava/util/Locale;` | `sun/util/locale/provider/LocaleServiceProviderPool` |

### `java/text/DateFormatSymbols`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAvailableLocales()[Ljava/util/Locale;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `getProviderInstance(Ljava/util/Locale;)Ljava/text/DateFormatSymbols;` | `sun/util/locale/provider/LocaleProviderAdapter` |
| `getZoneStringsImpl(Z)[[Ljava/lang/String;` | `sun/util/locale/provider/TimeZoneNameUtility` |
| `initializeData(Ljava/util/Locale;)V` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/ResourceBundleBasedAdapter`, `sun/util/resources/LocaleData` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `sun/util/locale/provider/TimeZoneNameUtility` |

### `java/text/DecimalFormat`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/text/DecimalFormatSymbols`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAvailableLocales()[Ljava/util/Locale;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `getInstance(Ljava/util/Locale;)Ljava/text/DecimalFormatSymbols;` | `sun/util/locale/provider/LocaleProviderAdapter` |
| `initialize(Ljava/util/Locale;)V` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |
| `initializeCurrency(Ljava/util/Locale;)V` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/text/DigitList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `set(ZDIZ)V` | `jdk/internal/math/FloatingDecimal`, `jdk/internal/math/FloatingDecimal$BinaryToASCIIConverter` |

### `java/text/Normalizer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isNormalized(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;)Z` | `jdk/internal/icu/text/NormalizerBase` |
| `normalize(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;)Ljava/lang/String;` | `jdk/internal/icu/text/NormalizerBase` |

### `java/text/NumberFormat`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAvailableLocales()[Ljava/util/Locale;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `getInstance(Ljava/util/Locale;Ljava/text/NumberFormat$Style;I)Ljava/text/NumberFormat;` | `sun/util/locale/provider/LocaleProviderAdapter` |
| `getInstance(Lsun/util/locale/provider/LocaleProviderAdapter;Ljava/util/Locale;Ljava/text/NumberFormat$Style;I)Ljava/text/NumberFormat;` | `sun/util/locale/provider/LocaleProviderAdapter` |

### `java/text/RBCollationTables`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getContractValues(I)Ljava/util/Vector;` | `sun/text/UCompactIntArray` |
| `getUnicodeOrder(I)I` | `sun/text/UCompactIntArray` |
| `usedInContractSeq(I)Z` | `sun/text/IntHashtable` |

### `java/text/RBTableBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/text/RBCollationTables$BuildAPI;)V` | `sun/text/IntHashtable` |
| `addComposedChars()V` | `sun/text/ComposedCharIter` |
| `addContractFlags(Ljava/lang/String;)V` | `sun/text/IntHashtable` |
| `addContractOrder(Ljava/lang/String;IZ)V` | `sun/text/UCompactIntArray` |
| `addOrder(II)V` | `sun/text/UCompactIntArray` |
| `build(Ljava/lang/String;I)V` | `jdk/internal/icu/impl/NormalizerImpl`, `sun/text/UCompactIntArray` |
| `getCharOrder(I)I` | `sun/text/UCompactIntArray` |
| `getContractValues(I)Ljava/util/Vector;` | `sun/text/UCompactIntArray` |

### `java/text/SimpleDateFormat`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |
| `matchZoneString(Ljava/lang/String;I[Ljava/lang/String;)I` | `sun/util/locale/provider/TimeZoneNameUtility` |
| `subFormat(IILjava/text/Format$FieldDelegate;Ljava/lang/StringBuffer;Z)V` | `sun/util/calendar/CalendarUtils`, `sun/util/calendar/ZoneInfoFile` |

### `java/time/Clock`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `currentInstant()Ljava/time/Instant;` | `jdk/internal/misc/VM` |

### `java/time/chrono/AbstractChronology`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `initCache()Z` | `sun/util/logging/PlatformLogger` |

### `java/time/chrono/HijrahChronology`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$registerCustomChrono$6(Ljava/lang/String;)V` | `sun/util/logging/PlatformLogger` |
| `lambda$registerCustomChrono$7()Ljava/lang/Void;` | `sun/util/logging/PlatformLogger` |
| `loadCalendarData()V` | `sun/util/logging/PlatformLogger` |

### `java/time/chrono/JapaneseChronology`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `prolepticYear(Ljava/time/chrono/Era;I)I` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `prolepticYearLenient(Ljava/time/chrono/JapaneseEra;I)I` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era` |
| `range(Ljava/time/temporal/ChronoField;)Ljava/time/temporal/ValueRange;` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era` |

### `java/time/chrono/JapaneseDate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/time/LocalDate;)V` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `of(Ljava/time/chrono/JapaneseEra;III)Ljava/time/chrono/JapaneseDate;` | `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `ofYearDay(Ljava/time/chrono/JapaneseEra;II)Ljava/time/chrono/JapaneseDate;` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `toPrivateJapaneseDate(Ljava/time/LocalDate;)Lsun/util/calendar/LocalGregorianCalendar$Date;` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date` |

### `java/time/chrono/JapaneseEra`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAbbreviation()Ljava/lang/String;` | `sun/util/calendar/Era` |
| `getName()Ljava/lang/String;` | `sun/util/calendar/Era` |
| `toJapaneseEra(Lsun/util/calendar/Era;)Ljava/time/chrono/JapaneseEra;` | `sun/util/calendar/Era` |

### `java/time/format/DateTimeFormatter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `localizedBy(Ljava/util/Locale;)Ljava/time/format/DateTimeFormatter;` | `sun/util/locale/provider/TimeZoneNameUtility` |

### `java/time/format/DateTimeFormatterBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getLocalizedDateTimePattern(Ljava/lang/String;Ljava/time/chrono/Chronology;Ljava/util/Locale;)Ljava/lang/String;` | `sun/text/spi/JavaTimeDateTimePatternProvider`, `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter` |
| `getLocalizedDateTimePattern(Ljava/time/format/FormatStyle;Ljava/time/format/FormatStyle;Ljava/time/chrono/Chronology;Ljava/util/Locale;)Ljava/lang/String;` | `sun/text/spi/JavaTimeDateTimePatternProvider`, `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter` |

### `java/time/format/DateTimeFormatterBuilder$DayPeriod`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$getDayPeriodMap$2(Ljava/util/Locale;)Ljava/util/Map;` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/time/format/DateTimeFormatterBuilder$DayPeriodPrinterParser`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$findDayPeriodStore$1(ILjava/util/Locale;Ljava/util/Map;Ljava/util/Map;Ljava/time/format/DateTimeFormatterBuilder$DayPeriod;Ljava/lang/Long;)V` | `sun/util/locale/provider/CalendarDataUtility` |

### `java/time/format/DateTimeFormatterBuilder$ZoneTextPrinterParser`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDisplayName(Ljava/lang/String;ILjava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/TimeZoneNameUtility` |
| `getTree(Ljava/time/format/DateTimeParseContext;)Ljava/time/format/DateTimeFormatterBuilder$PrefixTree;` | `sun/util/locale/provider/TimeZoneNameUtility` |
| `lambda$getTree$1(Ljava/util/Locale;Ljava/time/format/DateTimeFormatterBuilder$PrefixTree;Ljava/lang/String;)V` | `sun/util/locale/provider/TimeZoneNameUtility` |

### `java/time/format/DateTimeTextProvider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createStore(Ljava/time/temporal/TemporalField;Ljava/util/Locale;)Ljava/lang/Object;` | `sun/util/locale/provider/CalendarDataUtility` |
| `getLocalizedResource(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/Object;` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |
| `getText(Ljava/time/chrono/Chronology;Ljava/time/temporal/TemporalField;JLjava/time/format/TextStyle;Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/CalendarDataUtility` |
| `getTextIterator(Ljava/time/chrono/Chronology;Ljava/time/temporal/TemporalField;Ljava/time/format/TextStyle;Ljava/util/Locale;)Ljava/util/Iterator;` | `sun/util/locale/provider/CalendarDataUtility` |

### `java/time/temporal/ChronoField`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDisplayName(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/time/temporal/IsoFields$Field$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDisplayName(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/time/temporal/WeekFields`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `of(Ljava/util/Locale;)Ljava/time/temporal/WeekFields;` | `sun/util/locale/provider/CalendarDataUtility` |

### `java/time/temporal/WeekFields$ComputedDayOfField`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDisplayName(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/time/zone/TzdbZoneRulesProvider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `jdk/internal/util/StaticProperty` |

### `java/util/AbstractCollection`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `finishToArray([Ljava/lang/Object;Ljava/util/Iterator;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |

### `java/util/ArrayDeque`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/ArrayList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `grow(I)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/Arrays`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compare([BII[BII)I` | `jdk/internal/util/ArraysSupport` |
| `compare([B[B)I` | `jdk/internal/util/ArraysSupport` |
| `compare([CII[CII)I` | `jdk/internal/util/ArraysSupport` |
| `compare([C[C)I` | `jdk/internal/util/ArraysSupport` |
| `compare([DII[DII)I` | `jdk/internal/util/ArraysSupport` |
| `compare([D[D)I` | `jdk/internal/util/ArraysSupport` |
| `compare([FII[FII)I` | `jdk/internal/util/ArraysSupport` |
| `compare([F[F)I` | `jdk/internal/util/ArraysSupport` |
| `compare([III[III)I` | `jdk/internal/util/ArraysSupport` |
| `compare([I[I)I` | `jdk/internal/util/ArraysSupport` |
| `compare([JII[JII)I` | `jdk/internal/util/ArraysSupport` |
| `compare([J[J)I` | `jdk/internal/util/ArraysSupport` |
| `compare([SII[SII)I` | `jdk/internal/util/ArraysSupport` |
| `compare([S[S)I` | `jdk/internal/util/ArraysSupport` |
| `compare([ZII[ZII)I` | `jdk/internal/util/ArraysSupport` |
| `compare([Z[Z)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([BII[BII)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([B[B)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([III[III)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([I[I)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([JII[JII)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([J[J)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([SII[SII)I` | `jdk/internal/util/ArraysSupport` |
| `compareUnsigned([S[S)I` | `jdk/internal/util/ArraysSupport` |
| `equals([BII[BII)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([B[B)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([CII[CII)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([C[C)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([DII[DII)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([D[D)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([FII[FII)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([F[F)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([III[III)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([I[I)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([JII[JII)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([J[J)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([SII[SII)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([S[S)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([ZII[ZII)Z` | `jdk/internal/util/ArraysSupport` |
| `equals([Z[Z)Z` | `jdk/internal/util/ArraysSupport` |
| `hashCode([B)I` | `jdk/internal/util/ArraysSupport` |
| `hashCode([C)I` | `jdk/internal/util/ArraysSupport` |
| `hashCode([I)I` | `jdk/internal/util/ArraysSupport` |
| `hashCode([S)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([BII[BII)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([B[B)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([CII[CII)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([C[C)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([DII[DII)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([D[D)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([FII[FII)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([F[F)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([III[III)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([I[I)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([JII[JII)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([J[J)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([SII[SII)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([S[S)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([ZII[ZII)I` | `jdk/internal/util/ArraysSupport` |
| `mismatch([Z[Z)I` | `jdk/internal/util/ArraysSupport` |

### `java/util/Base64$Decoder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `decode(Ljava/lang/String;)[B` | `sun/nio/cs/ISO_8859_1` |

### `java/util/Base64$EncOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `write([BII)V` | `jdk/internal/util/Preconditions` |

### `java/util/Calendar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createCalendar(Ljava/util/TimeZone;Ljava/util/Locale;)Ljava/util/Calendar;` | `sun/util/BuddhistCalendar`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/spi/CalendarProvider` |
| `defaultTimeZone(Ljava/util/Locale;)Ljava/util/TimeZone;` | `sun/util/locale/provider/TimeZoneNameUtility` |
| `getDisplayName(IILjava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/CalendarDataUtility` |
| `getDisplayNames(IILjava/util/Locale;)Ljava/util/Map;` | `sun/util/locale/provider/CalendarDataUtility` |
| `setTimeInMillis(J)V` | `sun/util/calendar/ZoneInfo` |
| `setWeekCountData(Ljava/util/Locale;)V` | `sun/util/locale/provider/CalendarDataUtility` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `sun/util/calendar/ZoneInfo` |

### `java/util/Calendar$Builder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `build()Ljava/util/Calendar;` | `sun/util/BuddhistCalendar` |

### `java/util/CollSer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/Collections$CopiesList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/Currency`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDisplayName(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `getInstance(Ljava/util/Locale;)Ljava/util/Currency;` | `sun/util/locale/provider/CalendarDataUtility` |
| `getSymbol(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/CalendarDataUtility`, `sun/util/locale/provider/LocaleServiceProviderPool` |

### `java/util/Currency$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `jdk/internal/util/StaticProperty` |

### `java/util/Currency$CurrencyProperty`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `info(Ljava/lang/String;Ljava/lang/Throwable;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `java/util/Date`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(IIIIII)V` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarUtils` |
| `UTC(IIIIII)J` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarUtils` |
| `clone()Ljava/lang/Object;` | `sun/util/calendar/BaseCalendar$Date` |
| `getCalendarDate()Lsun/util/calendar/BaseCalendar$Date;` | `sun/util/calendar/BaseCalendar` |
| `getCalendarSystem(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar;` | `sun/util/calendar/BaseCalendar$Date` |
| `getDate()I` | `sun/util/calendar/BaseCalendar$Date` |
| `getDay()I` | `sun/util/calendar/BaseCalendar$Date` |
| `getHours()I` | `sun/util/calendar/BaseCalendar$Date` |
| `getJulianCalendar()Lsun/util/calendar/BaseCalendar;` | `sun/util/calendar/CalendarSystem` |
| `getMillisOf(Ljava/util/Date;)J` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date` |
| `getMinutes()I` | `sun/util/calendar/BaseCalendar$Date` |
| `getMonth()I` | `sun/util/calendar/BaseCalendar$Date` |
| `getSeconds()I` | `sun/util/calendar/BaseCalendar$Date` |
| `getTimeImpl()J` | `sun/util/calendar/BaseCalendar$Date` |
| `getTimezoneOffset()I` | `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/ZoneInfo` |
| `getYear()I` | `sun/util/calendar/BaseCalendar$Date` |
| `normalize()Lsun/util/calendar/BaseCalendar$Date;` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarSystem` |
| `normalize(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar$Date;` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date` |
| `parse(Ljava/lang/String;)J` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarDate` |
| `setDate(I)V` | `sun/util/calendar/BaseCalendar$Date` |
| `setHours(I)V` | `sun/util/calendar/BaseCalendar$Date` |
| `setMinutes(I)V` | `sun/util/calendar/BaseCalendar$Date` |
| `setMonth(I)V` | `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarUtils` |
| `setSeconds(I)V` | `sun/util/calendar/BaseCalendar$Date` |
| `setYear(I)V` | `sun/util/calendar/BaseCalendar$Date` |
| `toGMTString()Ljava/lang/String;` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarUtils` |
| `toString()Ljava/lang/String;` | `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarUtils` |

### `java/util/EnumMap`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getKeyUniverse(Ljava/lang/Class;)[Ljava/lang/Enum;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/EnumSet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getUniverse(Ljava/lang/Class;)[Ljava/lang/Enum;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/FormatItem`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `stringMix(JLjava/lang/String;)J` | `jdk/internal/access/JavaLangAccess` |

### `java/util/FormatItem$FormatItemDecimal`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mix(J)J` | `jdk/internal/access/JavaLangAccess` |

### `java/util/FormatItem$FormatItemFillLeft`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `prepend(J[B)J` | `jdk/internal/util/FormatConcatItem` |

### `java/util/FormatItem$FormatItemFillRight`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `prepend(J[B)J` | `jdk/internal/util/FormatConcatItem` |

### `java/util/FormatItem$FormatItemFormatSpecifier`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mix(J)J` | `jdk/internal/access/JavaLangAccess` |
| `prepend(J[B)J` | `jdk/internal/access/JavaLangAccess` |

### `java/util/FormatItem$FormatItemModifier`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljdk/internal/util/FormatConcatItem;)V` | `jdk/internal/util/FormatConcatItem` |

### `java/util/Formatter$FormatSpecifier`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `localizedMagnitude(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/lang/CharSequence;IIILjava/util/Locale;)Ljava/lang/StringBuilder;` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |
| `print(Ljava/util/Formatter;Ljava/lang/StringBuilder;DLjava/util/Locale;ICIZ)V` | `jdk/internal/math/FormattedFPDecimal` |

### `java/util/FormatterBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `groupSize(Ljava/util/Locale;Ljava/text/DecimalFormatSymbols;)I` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/util/GregorianCalendar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(IIIIIII)V` | `sun/util/calendar/Gregorian` |
| `<init>(Ljava/util/TimeZone;Ljava/util/Locale;)V` | `sun/util/calendar/Gregorian` |
| `<init>(Ljava/util/TimeZone;Ljava/util/Locale;Z)V` | `sun/util/calendar/Gregorian` |
| `actualMonthLength()I` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/Gregorian` |
| `clone()Ljava/lang/Object;` | `sun/util/calendar/BaseCalendar$Date` |
| `computeFields(II)I` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarUtils`, `sun/util/calendar/Gregorian`, +2 |
| `computeTime()V` | `sun/util/calendar/ZoneInfo` |
| `getActualMaximum(I)I` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarDate`, `sun/util/calendar/CalendarSystem`, +1 |
| `getActualMinimum(I)I` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date` |
| `getCalendarDate(J)Lsun/util/calendar/BaseCalendar$Date;` | `sun/util/calendar/BaseCalendar` |
| `getCurrentFixedDate()J` | `sun/util/calendar/BaseCalendar` |
| `getFixedDate(Lsun/util/calendar/BaseCalendar;II)J` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/CalendarUtils` |
| `getFixedDateJan1(Lsun/util/calendar/BaseCalendar$Date;J)J` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date` |
| `getFixedDateMonth1(Lsun/util/calendar/BaseCalendar$Date;J)J` | `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/JulianCalendar` |
| `getGreatestMinimum(I)I` | `sun/util/calendar/BaseCalendar$Date` |
| `getJulianCalendarSystem()Lsun/util/calendar/BaseCalendar;` | `sun/util/calendar/CalendarSystem`, `sun/util/calendar/JulianCalendar` |
| `getTimeZone()Ljava/util/TimeZone;` | `sun/util/calendar/BaseCalendar$Date` |
| `getWeekNumber(JJ)I` | `sun/util/calendar/CalendarUtils`, `sun/util/calendar/Gregorian` |
| `isInvalidWeek1()Z` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/Gregorian` |
| `isLeapYear(I)Z` | `sun/util/calendar/BaseCalendar$Date` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/util/calendar/Gregorian` |
| `roll(II)V` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarDate` |
| `setGregorianChange(J)V` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarUtils` |
| `setTimeZone(Ljava/util/TimeZone;)V` | `sun/util/calendar/BaseCalendar$Date` |

### `java/util/HashMap`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/HashMap$UnsafeHolder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `putLoadFactor(Ljava/util/HashMap;F)V` | `jdk/internal/misc/Unsafe` |

### `java/util/HashSet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/Hashtable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readHashtable(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/Hashtable$UnsafeHolder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `putLoadFactor(Ljava/util/Hashtable;F)V` | `jdk/internal/misc/Unsafe` |

### `java/util/HexFormat`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `formatOptDelimiter([BII)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |
| `toHexDigits(B)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |
| `toHexDigits(I)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |
| `toHexDigits(J)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |
| `toHexDigits(JI)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |
| `toHexDigits(S)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |

### `java/util/IdentityHashMap`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/JapaneseImperialCalendar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/TimeZone;Ljava/util/Locale;)V` | `sun/util/calendar/LocalGregorianCalendar` |
| `<init>(Ljava/util/TimeZone;Ljava/util/Locale;Z)V` | `sun/util/calendar/LocalGregorianCalendar` |
| `actualMonthLength()I` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar` |
| `add(II)V` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `clone()Ljava/lang/Object;` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `computeFields(II)I` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date`, +1 |
| `computeTime()V` | `sun/util/calendar/ZoneInfo` |
| `getActualMaximum(I)I` | `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/Gregorian`, +2 |
| `getActualMinimum(I)I` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `getCalendarDate(J)Lsun/util/calendar/LocalGregorianCalendar$Date;` | `sun/util/calendar/LocalGregorianCalendar` |
| `getDisplayName(IILjava/util/Locale;)Ljava/lang/String;` | `sun/util/calendar/Era`, `sun/util/locale/provider/CalendarDataUtility` |
| `getDisplayNames(IILjava/util/Locale;)Ljava/util/Map;` | `sun/util/calendar/Era`, `sun/util/locale/provider/CalendarDataUtility` |
| `getEraIndex(Lsun/util/calendar/LocalGregorianCalendar$Date;)I` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `getFixedDate(III)J` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/CalendarUtils`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar`, +1 |
| `getFixedDateJan1(Lsun/util/calendar/LocalGregorianCalendar$Date;J)J` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/Gregorian`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `getFixedDateMonth1(Lsun/util/calendar/LocalGregorianCalendar$Date;J)J` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `getMaximum(I)I` | `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `getTimeZone()Ljava/util/TimeZone;` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `getTransitionEraIndex(Lsun/util/calendar/LocalGregorianCalendar$Date;)I` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `getWeekNumber(JJ)I` | `sun/util/calendar/CalendarUtils`, `sun/util/calendar/LocalGregorianCalendar` |
| `getYearOffsetInMillis(Lsun/util/calendar/CalendarDate;)J` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/LocalGregorianCalendar` |
| `hashCode()I` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `isTransitionYear(I)Z` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era` |
| `monthLength(I)I` | `sun/util/calendar/LocalGregorianCalendar$Date` |
| `monthLength(II)I` | `sun/util/calendar/CalendarUtils` |
| `pinDayOfMonth(Lsun/util/calendar/LocalGregorianCalendar$Date;)V` | `sun/util/calendar/LocalGregorianCalendar`, `sun/util/calendar/LocalGregorianCalendar$Date` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/util/calendar/LocalGregorianCalendar` |
| `roll(II)V` | `sun/util/calendar/CalendarDate`, `sun/util/calendar/Era`, `sun/util/calendar/Gregorian`, `sun/util/calendar/LocalGregorianCalendar`, +1 |
| `setTimeZone(Ljava/util/TimeZone;)V` | `sun/util/calendar/LocalGregorianCalendar$Date` |

### `java/util/ListResourceBundle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getKeys()Ljava/util/Enumeration;` | `sun/util/ResourceBundleEnumeration` |

### `java/util/Locale`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | `sun/util/locale/BaseLocale` |
| `availableLocales()Ljava/util/stream/Stream;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `caseFoldLanguageTag(Ljava/lang/String;)Ljava/lang/String;` | `sun/util/locale/LanguageTag` |
| `convertOldISOCodes(Ljava/lang/String;)Ljava/lang/String;` | `sun/util/locale/BaseLocale`, `sun/util/locale/LocaleUtils` |
| `createConstant(B)Ljava/util/Locale;` | `sun/util/locale/BaseLocale` |
| `equals(Ljava/lang/Object;)Z` | `sun/util/locale/BaseLocale`, `sun/util/locale/LocaleExtensions` |
| `filter(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;` | `sun/util/locale/LocaleMatcher` |
| `filterTags(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;` | `sun/util/locale/LocaleMatcher` |
| `forLanguageTag(Ljava/lang/String;)Ljava/util/Locale;` | `sun/util/locale/BaseLocale`, `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LanguageTag` |
| `getAvailableLocales()[Ljava/util/Locale;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `getCompatibilityExtensions(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/LocaleExtensions;` | `sun/util/locale/LocaleExtensions`, `sun/util/locale/LocaleUtils` |
| `getCountry()Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getDefaultExtensions(Ljava/lang/String;)Ljava/util/Optional;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleUtils` |
| `getDisplayCountry(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getDisplayKeyTypeExtensionString(Ljava/lang/String;Lsun/util/locale/provider/LocaleResources;Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/LocaleExtensions`, `sun/util/locale/provider/LocaleResources`, `sun/util/locale/provider/TimeZoneNameUtility` |
| `getDisplayLanguage(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getDisplayName(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/LocaleExtensions`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |
| `getDisplayScript(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getDisplayString(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;I)Ljava/lang/String;` | `sun/util/locale/provider/LocaleServiceProviderPool` |
| `getDisplayVariant(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/BaseLocale`, `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |
| `getDisplayVariantArray(Ljava/util/Locale;)[Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getExtension(C)Ljava/lang/String;` | `sun/util/locale/LocaleExtensions` |
| `getExtensionKeys()Ljava/util/Set;` | `sun/util/locale/LocaleExtensions` |
| `getISO3Country()Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getISO3Language()Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;` | `sun/util/locale/BaseLocale` |
| `getLanguage()Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getScript()Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `getUnicodeLocaleAttributes()Ljava/util/Set;` | `sun/util/locale/LocaleExtensions` |
| `getUnicodeLocaleKeys()Ljava/util/Set;` | `sun/util/locale/LocaleExtensions` |
| `getUnicodeLocaleType(Ljava/lang/String;)Ljava/lang/String;` | `sun/util/locale/LocaleExtensions` |
| `getVariant()Ljava/lang/String;` | `sun/util/locale/BaseLocale` |
| `hashCode()I` | `sun/util/locale/BaseLocale`, `sun/util/locale/LocaleExtensions` |
| `initDefault()Ljava/util/Locale;` | `jdk/internal/util/StaticProperty` |
| `initDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;` | `jdk/internal/util/StaticProperty` |
| `isUnicodeExtensionKey(Ljava/lang/String;)Z` | `sun/util/locale/LocaleUtils` |
| `lambda$getDisplayKeyTypeExtensionString$2(Ljava/util/Locale;Ljava/lang/String;)Ljava/lang/String;` | `sun/util/locale/provider/TimeZoneNameUtility` |
| `lookup(Ljava/util/List;Ljava/util/Collection;)Ljava/util/Locale;` | `sun/util/locale/LocaleMatcher` |
| `lookupTag(Ljava/util/List;Ljava/util/Collection;)Ljava/lang/String;` | `sun/util/locale/LocaleMatcher` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/util/locale/BaseLocale`, `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `readResolve()Ljava/lang/Object;` | `sun/util/locale/BaseLocale` |
| `toLanguageTag()Ljava/lang/String;` | `sun/util/locale/LanguageTag` |
| `toString()Ljava/lang/String;` | `sun/util/locale/BaseLocale`, `sun/util/locale/LocaleExtensions` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `sun/util/locale/BaseLocale`, `sun/util/locale/LocaleExtensions` |

### `java/util/Locale$Builder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/util/locale/InternalLocaleBuilder` |
| `addUnicodeLocaleAttribute(Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `build()Ljava/util/Locale;` | `sun/util/locale/BaseLocale`, `sun/util/locale/InternalLocaleBuilder` |
| `clear()Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder` |
| `clearExtensions()Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder` |
| `removeUnicodeLocaleAttribute(Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `setExtension(CLjava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `setLanguage(Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `setLanguageTag(Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LanguageTag`, `sun/util/locale/ParseStatus` |
| `setLocale(Ljava/util/Locale;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `setRegion(Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `setScript(Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `setUnicodeLocaleKeyword(Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |
| `setVariant(Ljava/lang/String;)Ljava/util/Locale$Builder;` | `sun/util/locale/InternalLocaleBuilder`, `sun/util/locale/LocaleSyntaxException` |

### `java/util/Locale$Cache`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/util/locale/LocaleObjectCache` |

### `java/util/Locale$LanguageRange`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mapEquivalents(Ljava/util/List;Ljava/util/Map;)Ljava/util/List;` | `sun/util/locale/LocaleMatcher` |
| `parse(Ljava/lang/String;)Ljava/util/List;` | `sun/util/locale/LocaleMatcher` |

### `java/util/Locale$LocaleKey`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V` | `sun/util/locale/BaseLocale`, `sun/util/locale/LocaleExtensions` |
| `equals(Ljava/lang/Object;)Z` | `sun/util/locale/BaseLocale`, `sun/util/locale/LocaleExtensions` |

### `java/util/Objects`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkFromIndexSize(III)I` | `jdk/internal/util/Preconditions` |
| `checkFromIndexSize(JJJ)J` | `jdk/internal/util/Preconditions` |
| `checkFromToIndex(III)I` | `jdk/internal/util/Preconditions` |
| `checkFromToIndex(JJJ)J` | `jdk/internal/util/Preconditions` |
| `checkIndex(II)I` | `jdk/internal/util/Preconditions` |
| `checkIndex(JJ)J` | `jdk/internal/util/Preconditions` |

### `java/util/PriorityQueue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `grow(I)V` | `jdk/internal/util/ArraysSupport` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/Properties`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/Properties;I)V` | `jdk/internal/misc/Unsafe` |
| `loadFromXML(Ljava/io/InputStream;)V` | `jdk/internal/util/xml/PropertiesDefaultHandler` |
| `readHashtable(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |
| `store(Ljava/io/OutputStream;Ljava/lang/String;)V` | `sun/nio/cs/ISO_8859_1` |
| `storeToXML(Ljava/io/OutputStream;Ljava/lang/String;)V` | `sun/nio/cs/UTF_8` |
| `storeToXML(Ljava/io/OutputStream;Ljava/lang/String;Ljava/nio/charset/Charset;)V` | `jdk/internal/util/xml/PropertiesDefaultHandler` |
| `writeDateComment(Ljava/io/BufferedWriter;)V` | `jdk/internal/util/StaticProperty` |

### `java/util/Properties$LineReader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readLine()I` | `jdk/internal/util/ArraysSupport` |

### `java/util/PropertyResourceBundle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;)V` | `sun/nio/cs/ISO_8859_1`, `sun/util/PropertyResourceBundleCharset` |
| `getKeys()Ljava/util/Enumeration;` | `sun/util/ResourceBundleEnumeration` |

### `java/util/Random`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `doubles()Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `doubles(DD)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `doubles(J)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `doubles(JDD)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `ints()Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `ints(II)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `ints(J)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `ints(JII)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `longs()Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `longs(J)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `longs(JJ)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `longs(JJJ)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `resetSeed(J)V` | `jdk/internal/misc/Unsafe` |

### `java/util/ResourceBundle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `clearCache()V` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;Ljava/lang/Module;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/ClassLoader;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Module;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundle(Ljava/lang/String;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |
| `getBundleFromModule(Ljava/lang/Class;Ljava/lang/Module;Ljava/lang/String;Ljava/util/Locale;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;` | `sun/security/util/SecurityConstants` |
| `getBundleImpl(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Class;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;` | `jdk/internal/loader/BootLoader` |
| `getServiceLoader(Ljava/lang/Module;Ljava/lang/String;)Ljava/util/ServiceLoader;` | `jdk/internal/reflect/Reflection` |
| `loadBundle(Ljava/util/ResourceBundle$CacheKey;Ljava/util/List;Ljava/util/ResourceBundle$Control;Ljava/lang/Module;Ljava/lang/Module;)Ljava/util/ResourceBundle;` | `jdk/internal/reflect/Reflection` |

### `java/util/ResourceBundle$Control`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/String;Ljava/lang/ClassLoader;Z)Ljava/util/ResourceBundle;` | `sun/util/resources/Bundles` |

### `java/util/ResourceBundle$Control$CandidateListCache`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/util/locale/LocaleObjectCache` |
| `createObject(Lsun/util/locale/BaseLocale;)Ljava/util/List;` | `sun/util/locale/BaseLocale` |

### `java/util/ResourceBundle$ResourceBundleProviderHelper`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$loadPropertyResourceBundle$2(Ljava/lang/String;Ljava/lang/Module;Ljava/lang/Module;)Ljava/io/InputStream;` | `jdk/internal/loader/BootLoader` |
| `loadResourceBundle(Ljava/lang/Module;Ljava/lang/Module;Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;` | `jdk/internal/loader/BootLoader`, `sun/security/util/SecurityConstants` |

### `java/util/ReverseOrderDequeView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toArray()[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `toArray([Ljava/lang/Object;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |

### `java/util/ReverseOrderListView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addAll(ILjava/util/Collection;)Z` | `jdk/internal/util/ArraysSupport` |
| `addAll(Ljava/util/Collection;)Z` | `jdk/internal/util/ArraysSupport` |
| `toArray()[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `toArray([Ljava/lang/Object;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |

### `java/util/ReverseOrderSortedSetView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toArray()[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `toArray([Ljava/lang/Object;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |

### `java/util/Scanner`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `useLocale(Ljava/util/Locale;)Ljava/util/Scanner;` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `java/util/SequencedMap`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `firstEntry()Ljava/util/Map$Entry;` | `jdk/internal/util/NullableKeyValueHolder` |
| `lastEntry()Ljava/util/Map$Entry;` | `jdk/internal/util/NullableKeyValueHolder` |
| `pollFirstEntry()Ljava/util/Map$Entry;` | `jdk/internal/util/NullableKeyValueHolder` |
| `pollLastEntry()Ljava/util/Map$Entry;` | `jdk/internal/util/NullableKeyValueHolder` |

### `java/util/ServiceLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/ClassLoader;)V` | `jdk/internal/misc/VM` |
| `checkCaller(Ljava/lang/Class;Ljava/lang/Class;)V` | `jdk/internal/reflect/Reflection` |
| `findStaticProviderMethod(Ljava/lang/Class;)Ljava/lang/reflect/Method;` | `jdk/internal/access/JavaLangAccess` |
| `load(Ljava/lang/Class;)Ljava/util/ServiceLoader;` | `jdk/internal/reflect/Reflection` |
| `load(Ljava/lang/Class;Ljava/lang/ClassLoader;)Ljava/util/ServiceLoader;` | `jdk/internal/reflect/Reflection` |
| `load(Ljava/lang/ModuleLayer;Ljava/lang/Class;)Ljava/util/ServiceLoader;` | `jdk/internal/reflect/Reflection` |
| `loadInstalled(Ljava/lang/Class;)Ljava/util/ServiceLoader;` | `jdk/internal/reflect/Reflection` |
| `loadProvider(Ljdk/internal/module/ServicesCatalog$ServiceProvider;)Ljava/util/ServiceLoader$Provider;` | `jdk/internal/module/ServicesCatalog$ServiceProvider` |

### `java/util/ServiceLoader$LayerLookupIterator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `providers(Ljava/lang/ModuleLayer;)Ljava/util/Iterator;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/module/ServicesCatalog` |

### `java/util/ServiceLoader$LazyClassPathLookupIterator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `nextProviderClass()Ljava/lang/Class;` | `jdk/internal/loader/BootLoader`, `jdk/internal/loader/ClassLoaders` |
| `parse(Ljava/net/URL;)Ljava/util/Iterator;` | `sun/nio/cs/UTF_8` |

### `java/util/ServiceLoader$ModuleServicesLookupIterator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `iteratorFor(Ljava/lang/ClassLoader;)Ljava/util/Iterator;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/loader/BootLoader`, `jdk/internal/loader/ClassLoaders`, `jdk/internal/module/ServicesCatalog`, +1 |
| `providers(Ljava/lang/ModuleLayer;)Ljava/util/List;` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/module/ServicesCatalog` |

### `java/util/SimpleTimeZone`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getOffset(IIIIII)I` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarSystem`, `sun/util/calendar/CalendarUtils` |
| `getOffsets(J[I)I` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date`, `sun/util/calendar/CalendarSystem` |
| `getTransition(Lsun/util/calendar/BaseCalendar;Lsun/util/calendar/BaseCalendar$Date;IIIIII)J` | `sun/util/calendar/BaseCalendar`, `sun/util/calendar/BaseCalendar$Date` |

### `java/util/SplittableRandom$AbstractSplittableGeneratorProxy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/SplittableRandom;)V` | `jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator` |

### `java/util/StringJoiner`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `compactElts()V` | `jdk/internal/access/JavaLangAccess` |
| `toString()Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |

### `java/util/TimeZone`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAvailableIDs()[Ljava/lang/String;` | `sun/util/calendar/ZoneInfo` |
| `getAvailableIDs(I)[Ljava/lang/String;` | `sun/util/calendar/ZoneInfo` |
| `getDisplayName(ZILjava/util/Locale;)Ljava/lang/String;` | `sun/util/calendar/ZoneInfoFile`, `sun/util/locale/provider/TimeZoneNameUtility` |
| `getDisplayNames(Ljava/lang/String;Ljava/util/Locale;)[Ljava/lang/String;` | `sun/util/locale/provider/TimeZoneNameUtility` |
| `getTimeZone(Ljava/lang/String;Z)Ljava/util/TimeZone;` | `sun/util/calendar/ZoneInfo` |
| `getTimeZone(Ljava/time/ZoneId;)Ljava/util/TimeZone;` | `sun/util/calendar/ZoneInfo` |
| `parseCustomTimeZone(Ljava/lang/String;)Ljava/util/TimeZone;` | `sun/util/calendar/ZoneInfo`, `sun/util/calendar/ZoneInfoFile` |
| `setDefaultZone()Ljava/util/TimeZone;` | `jdk/internal/util/StaticProperty`, `sun/security/action/GetPropertyAction` |
| `toZoneId0()Ljava/time/ZoneId;` | `sun/util/calendar/ZoneInfoFile` |

### `java/util/Timer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Z)V` | `jdk/internal/ref/CleanerFactory` |

### `java/util/Tripwire`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `trip(Ljava/lang/Class;Ljava/lang/String;)V` | `sun/util/logging/PlatformLogger` |

### `java/util/UUID`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toString()Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |

### `java/util/Vector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `grow(I)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |

### `java/util/concurrent/ConcurrentHashMap`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addCount(JI)V` | `jdk/internal/misc/Unsafe` |
| `casTabAt([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `fullAddCount(JZ)V` | `jdk/internal/misc/Unsafe` |
| `helpTransfer([Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)[Ljava/util/concurrent/ConcurrentHashMap$Node;` | `jdk/internal/misc/Unsafe` |
| `initTable()[Ljava/util/concurrent/ConcurrentHashMap$Node;` | `jdk/internal/misc/Unsafe` |
| `setTabAt([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;)V` | `jdk/internal/misc/Unsafe` |
| `tabAt([Ljava/util/concurrent/ConcurrentHashMap$Node;I)Ljava/util/concurrent/ConcurrentHashMap$Node;` | `jdk/internal/misc/Unsafe` |
| `transfer([Ljava/util/concurrent/ConcurrentHashMap$Node;[Ljava/util/concurrent/ConcurrentHashMap$Node;)V` | `jdk/internal/misc/Unsafe` |
| `tryPresize(I)V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/ConcurrentHashMap$TreeBin`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `contendedLock()V` | `jdk/internal/misc/Unsafe` |
| `find(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;` | `jdk/internal/misc/Unsafe` |
| `lockRoot()V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/CopyOnWriteArrayList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/concurrent/CopyOnWriteArrayList$Reversed`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addAll(ILjava/util/Collection;)Z` | `jdk/internal/util/ArraysSupport` |
| `addAll(Ljava/util/Collection;)Z` | `jdk/internal/util/ArraysSupport` |
| `toArray()[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |
| `toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;` | `jdk/internal/util/ArraysSupport` |

### `java/util/concurrent/CopyOnWriteArraySet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/CountedCompleter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addToPendingCount(I)V` | `jdk/internal/misc/Unsafe` |
| `compareAndSetPendingCount(II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPendingCount(II)Z` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/Executors$AutoShutdownDelegatedExecutorService`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/concurrent/ExecutorService;)V` | `jdk/internal/ref/CleanerFactory` |

### `java/util/concurrent/Executors$PrivilegedCallableUsingCurrentClassLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/concurrent/Callable;)V` | `sun/security/util/SecurityConstants` |

### `java/util/concurrent/Executors$PrivilegedThreadFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/security/util/SecurityConstants` |

### `java/util/concurrent/ForkJoinPool`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(B)V` | `jdk/internal/vm/SharedThreadContainer` |
| `<init>(ILjava/util/concurrent/ForkJoinPool$ForkJoinWorkerThreadFactory;Ljava/lang/Thread$UncaughtExceptionHandler;ZIIILjava/util/function/Predicate;JLjava/util/concurrent/TimeUnit;)V` | `jdk/internal/vm/SharedThreadContainer` |
| `compareAndExchangeCtl(JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndSetCtl(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `createWorker()Z` | `jdk/internal/vm/SharedThreadContainer` |
| `externalSubmit(Ljava/util/concurrent/ForkJoinTask;)Ljava/util/concurrent/ForkJoinTask;` | `jdk/internal/misc/Unsafe` |
| `getAndAddCtl(J)J` | `jdk/internal/misc/Unsafe` |
| `getAndAddPoolIds(I)I` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrRunState(I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSetParallelism(I)I` | `jdk/internal/misc/Unsafe` |
| `getParallelismOpaque()I` | `jdk/internal/misc/Unsafe` |
| `hasTasks(Z)Z` | `jdk/internal/misc/Unsafe` |
| `helpComplete(Ljava/util/concurrent/ForkJoinTask;Ljava/util/concurrent/ForkJoinPool$WorkQueue;ZZ)I` | `jdk/internal/misc/Unsafe` |
| `helpJoin(Ljava/util/concurrent/ForkJoinTask;Ljava/util/concurrent/ForkJoinPool$WorkQueue;Z)I` | `jdk/internal/misc/Unsafe` |
| `helpQuiesce(Ljava/util/concurrent/ForkJoinPool$WorkQueue;JZ)I` | `jdk/internal/misc/Unsafe` |
| `incrementThreadIds()J` | `jdk/internal/misc/Unsafe` |
| `poolSubmit(ZLjava/util/concurrent/ForkJoinTask;)Ljava/util/concurrent/ForkJoinTask;` | `jdk/internal/misc/Unsafe` |
| `registerWorker(Ljava/util/concurrent/ForkJoinPool$WorkQueue;)V` | `jdk/internal/misc/Unsafe` |
| `scan(Ljava/util/concurrent/ForkJoinPool$WorkQueue;II)I` | `jdk/internal/misc/Unsafe` |
| `tryTerminate(ZZ)Z` | `jdk/internal/vm/SharedThreadContainer` |

### `java/util/concurrent/ForkJoinPool$DefaultForkJoinWorkerThreadFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newThread(Ljava/util/concurrent/ForkJoinPool;)Ljava/util/concurrent/ForkJoinWorkerThread;` | `jdk/internal/access/JavaLangAccess` |

### `java/util/concurrent/ForkJoinPool$WorkQueue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casSlotToNull([Ljava/util/concurrent/ForkJoinTask;ILjava/util/concurrent/ForkJoinTask;)Z` | `jdk/internal/misc/Unsafe` |
| `forcePhaseActive()V` | `jdk/internal/misc/Unsafe` |
| `getAndClearSlot([Ljava/util/concurrent/ForkJoinTask;I)Ljava/util/concurrent/ForkJoinTask;` | `jdk/internal/misc/Unsafe` |
| `getAndSetAccess(I)I` | `jdk/internal/misc/Unsafe` |
| `helpAsyncBlocker(Ljava/util/concurrent/ForkJoinPool$ManagedBlocker;)V` | `jdk/internal/misc/Unsafe` |
| `nextLocalTask(I)Ljava/util/concurrent/ForkJoinTask;` | `jdk/internal/misc/Unsafe` |
| `poll(Ljava/util/concurrent/ForkJoinPool;)Ljava/util/concurrent/ForkJoinTask;` | `jdk/internal/misc/Unsafe` |
| `releaseAccess()V` | `jdk/internal/misc/Unsafe` |
| `tryPoll()Ljava/util/concurrent/ForkJoinTask;` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/ForkJoinTask`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casAux(Ljava/util/concurrent/ForkJoinTask$Aux;Ljava/util/concurrent/ForkJoinTask$Aux;)Z` | `jdk/internal/misc/Unsafe` |
| `casStatus(II)Z` | `jdk/internal/misc/Unsafe` |
| `fork()Ljava/util/concurrent/ForkJoinTask;` | `jdk/internal/misc/Unsafe` |
| `getAndBitwiseOrStatus(I)I` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/ForkJoinTask$Aux`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casNext(Ljava/util/concurrent/ForkJoinTask$Aux;Ljava/util/concurrent/ForkJoinTask$Aux;)Z` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/PriorityBlockingQueue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |
| `tryGrow([Ljava/lang/Object;I)V` | `jdk/internal/util/ArraysSupport` |

### `java/util/concurrent/StructuredTaskScope`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/util/concurrent/ThreadFactory;)V` | `jdk/internal/misc/ThreadFlock` |
| `close()V` | `jdk/internal/misc/ThreadFlock` |
| `ensureJoinedIfOwner(I)V` | `jdk/internal/misc/ThreadFlock` |
| `ensureOwner()V` | `jdk/internal/misc/ThreadFlock` |
| `ensureOwnerOrContainsThread()V` | `jdk/internal/misc/ThreadFlock` |
| `fork(Ljava/util/concurrent/Callable;)Ljava/util/concurrent/StructuredTaskScope$Subtask;` | `jdk/internal/misc/ThreadFlock` |
| `implInterruptAll()V` | `jdk/internal/misc/ThreadFlock` |
| `implJoin(Ljava/time/Duration;)V` | `jdk/internal/misc/ThreadFlock` |
| `implShutdown()Z` | `jdk/internal/misc/ThreadFlock` |
| `shutdown()V` | `jdk/internal/misc/ThreadFlock` |
| `toString()Ljava/lang/String;` | `jdk/internal/misc/ThreadFlock` |

### `java/util/concurrent/ThreadLocalRandom`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `advanceProbe(I)I` | `jdk/internal/misc/Unsafe` |
| `current()Ljava/util/concurrent/ThreadLocalRandom;` | `jdk/internal/misc/Unsafe` |
| `doubles()Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `doubles(DD)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `doubles(J)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `doubles(JDD)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `eraseThreadLocals(Ljava/lang/Thread;)V` | `jdk/internal/misc/Unsafe` |
| `getProbe()I` | `jdk/internal/misc/Unsafe` |
| `ints()Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `ints(II)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `ints(J)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `ints(JII)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `localInit()V` | `jdk/internal/misc/Unsafe`, `jdk/internal/util/random/RandomSupport` |
| `longs()Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `longs(J)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `longs(JJ)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `longs(JJJ)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator` |
| `nextLong()J` | `jdk/internal/util/random/RandomSupport` |
| `nextSecondarySeed()I` | `jdk/internal/misc/Unsafe` |
| `nextSeed()J` | `jdk/internal/misc/Unsafe` |
| `setInheritedAccessControlContext(Ljava/lang/Thread;Ljava/security/AccessControlContext;)V` | `jdk/internal/misc/Unsafe` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/ThreadPerTaskExecutor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/concurrent/ThreadFactory;)V` | `jdk/internal/vm/ThreadContainer` |
| `create(Ljava/util/concurrent/ThreadFactory;)Ljava/util/concurrent/ThreadPerTaskExecutor;` | `jdk/internal/vm/ThreadContainers` |
| `start(Ljava/lang/Thread;)V` | `jdk/internal/access/JavaLangAccess` |
| `tryTerminate()V` | `jdk/internal/vm/ThreadContainers` |

### `java/util/concurrent/ThreadPoolExecutor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(IIJLjava/util/concurrent/TimeUnit;Ljava/util/concurrent/BlockingQueue;Ljava/util/concurrent/ThreadFactory;Ljava/util/concurrent/RejectedExecutionHandler;)V` | `jdk/internal/vm/SharedThreadContainer` |
| `addWorker(Ljava/lang/Runnable;Z)Z` | `jdk/internal/vm/SharedThreadContainer` |
| `tryTerminate()V` | `jdk/internal/vm/SharedThreadContainer` |

### `java/util/concurrent/atomic/AtomicInteger`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addAndGet(I)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(II)I` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(II)Z` | `jdk/internal/misc/Unsafe` |
| `decrementAndGet()I` | `jdk/internal/misc/Unsafe` |
| `getAcquire()I` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(I)I` | `jdk/internal/misc/Unsafe` |
| `getAndDecrement()I` | `jdk/internal/misc/Unsafe` |
| `getAndIncrement()I` | `jdk/internal/misc/Unsafe` |
| `getAndSet(I)I` | `jdk/internal/misc/Unsafe` |
| `getOpaque()I` | `jdk/internal/misc/Unsafe` |
| `getPlain()I` | `jdk/internal/misc/Unsafe` |
| `incrementAndGet()I` | `jdk/internal/misc/Unsafe` |
| `lazySet(I)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(I)V` | `jdk/internal/misc/Unsafe` |
| `setPlain(I)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(I)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(II)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetVolatile(II)Z` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/atomic/AtomicIntegerFieldUpdater`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newUpdater(Ljava/lang/Class;Ljava/lang/String;)Ljava/util/concurrent/atomic/AtomicIntegerFieldUpdater;` | `jdk/internal/reflect/Reflection` |

### `java/util/concurrent/atomic/AtomicIntegerFieldUpdater$AtomicIntegerFieldUpdaterImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)V` | `jdk/internal/misc/Unsafe`, `sun/reflect/misc/ReflectUtil` |
| `compareAndSet(Ljava/lang/Object;II)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/Object;)I` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/Object;I)I` | `jdk/internal/misc/Unsafe` |
| `lazySet(Ljava/lang/Object;I)V` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/Object;I)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/Object;II)Z` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/atomic/AtomicLong`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addAndGet(J)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchange(JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeAcquire(JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndExchangeRelease(JJ)J` | `jdk/internal/misc/Unsafe` |
| `compareAndSet(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `decrementAndGet()J` | `jdk/internal/misc/Unsafe` |
| `getAcquire()J` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(J)J` | `jdk/internal/misc/Unsafe` |
| `getAndDecrement()J` | `jdk/internal/misc/Unsafe` |
| `getAndIncrement()J` | `jdk/internal/misc/Unsafe` |
| `getAndSet(J)J` | `jdk/internal/misc/Unsafe` |
| `getOpaque()J` | `jdk/internal/misc/Unsafe` |
| `getPlain()J` | `jdk/internal/misc/Unsafe` |
| `incrementAndGet()J` | `jdk/internal/misc/Unsafe` |
| `lazySet(J)V` | `jdk/internal/misc/Unsafe` |
| `set(J)V` | `jdk/internal/misc/Unsafe` |
| `setOpaque(J)V` | `jdk/internal/misc/Unsafe` |
| `setPlain(J)V` | `jdk/internal/misc/Unsafe` |
| `setRelease(J)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetAcquire(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetPlain(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetRelease(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSetVolatile(JJ)Z` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/atomic/AtomicLongFieldUpdater`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newUpdater(Ljava/lang/Class;Ljava/lang/String;)Ljava/util/concurrent/atomic/AtomicLongFieldUpdater;` | `jdk/internal/reflect/Reflection` |

### `java/util/concurrent/atomic/AtomicLongFieldUpdater$CASUpdater`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)V` | `jdk/internal/misc/Unsafe`, `sun/reflect/misc/ReflectUtil` |
| `compareAndSet(Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/Object;)J` | `jdk/internal/misc/Unsafe` |
| `getAndAdd(Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/Object;J)J` | `jdk/internal/misc/Unsafe` |
| `lazySet(Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/atomic/AtomicLongFieldUpdater$LockedUpdater`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)V` | `jdk/internal/misc/Unsafe`, `sun/reflect/misc/ReflectUtil` |
| `compareAndSet(Ljava/lang/Object;JJ)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/Object;)J` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/atomic/AtomicReferenceFieldUpdater`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newUpdater(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/String;)Ljava/util/concurrent/atomic/AtomicReferenceFieldUpdater;` | `jdk/internal/reflect/Reflection` |

### `java/util/concurrent/atomic/AtomicReferenceFieldUpdater$AtomicReferenceFieldUpdaterImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)V` | `jdk/internal/misc/Unsafe`, `sun/reflect/misc/ReflectUtil` |
| `compareAndSet(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |
| `get(Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `getAndSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `lazySet(Ljava/lang/Object;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `set(Ljava/lang/Object;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `weakCompareAndSet(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/AbstractQueuedLongSynchronizer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireOnOOME(ZJ)I` | `jdk/internal/misc/Unsafe` |
| `casTail(Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndSetState(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `reacquire(Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;J)V` | `jdk/internal/misc/Unsafe` |
| `tryInitializeHead()Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/AbstractQueuedLongSynchronizer$ConditionObject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newConditionNode()Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$ConditionNode;` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casNext(Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `casPrev(Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `clearStatus()V` | `jdk/internal/misc/Unsafe` |
| `getAndUnsetStatus(I)I` | `jdk/internal/misc/Unsafe` |
| `setPrevRelaxed(Ljava/util/concurrent/locks/AbstractQueuedLongSynchronizer$Node;)V` | `jdk/internal/misc/Unsafe` |
| `setStatusRelaxed(I)V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/AbstractQueuedSynchronizer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireOnOOME(ZI)I` | `jdk/internal/misc/Unsafe` |
| `casTail(Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `compareAndSetState(II)Z` | `jdk/internal/misc/Unsafe` |
| `reacquire(Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;I)V` | `jdk/internal/misc/Unsafe` |
| `tryInitializeHead()Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionObject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newConditionNode()Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionNode;` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/AbstractQueuedSynchronizer$Node`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casNext(Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `casPrev(Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `clearStatus()V` | `jdk/internal/misc/Unsafe` |
| `getAndUnsetStatus(I)I` | `jdk/internal/misc/Unsafe` |
| `setPrevRelaxed(Ljava/util/concurrent/locks/AbstractQueuedSynchronizer$Node;)V` | `jdk/internal/misc/Unsafe` |
| `setStatusRelaxed(I)V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/LockSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getBlocker(Ljava/lang/Thread;)Ljava/lang/Object;` | `jdk/internal/misc/Unsafe` |
| `park()V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VirtualThreads` |
| `park(Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VirtualThreads` |
| `parkNanos(J)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VirtualThreads` |
| `parkNanos(Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VirtualThreads` |
| `parkUntil(J)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VirtualThreads` |
| `parkUntil(Ljava/lang/Object;J)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VirtualThreads` |
| `setBlocker(Ljava/lang/Thread;Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `setCurrentBlocker(Ljava/lang/Object;)V` | `jdk/internal/misc/Unsafe` |
| `unpark(Ljava/lang/Thread;)V` | `jdk/internal/misc/Unsafe`, `jdk/internal/misc/VirtualThreads` |

### `java/util/concurrent/locks/StampedLock`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `acquireWrite(ZZJ)J` | `jdk/internal/misc/Unsafe` |
| `casState(JJ)Z` | `jdk/internal/misc/Unsafe` |
| `casTail(Ljava/util/concurrent/locks/StampedLock$Node;Ljava/util/concurrent/locks/StampedLock$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `readLock()J` | `jdk/internal/misc/Unsafe` |
| `tryAcquireWrite()J` | `jdk/internal/misc/Unsafe` |
| `tryConvertToOptimisticRead(J)J` | `jdk/internal/misc/Unsafe` |
| `tryConvertToWriteLock(J)J` | `jdk/internal/misc/Unsafe` |
| `tryInitializeHead()V` | `jdk/internal/misc/Unsafe` |
| `validate(J)Z` | `jdk/internal/misc/Unsafe` |
| `writeLock()J` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/StampedLock$Node`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casNext(Ljava/util/concurrent/locks/StampedLock$Node;Ljava/util/concurrent/locks/StampedLock$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `casPrev(Ljava/util/concurrent/locks/StampedLock$Node;Ljava/util/concurrent/locks/StampedLock$Node;)Z` | `jdk/internal/misc/Unsafe` |
| `clearStatus()V` | `jdk/internal/misc/Unsafe` |
| `getAndUnsetStatus(I)I` | `jdk/internal/misc/Unsafe` |
| `setPrevRelaxed(Ljava/util/concurrent/locks/StampedLock$Node;)V` | `jdk/internal/misc/Unsafe` |
| `setStatusRelaxed(I)V` | `jdk/internal/misc/Unsafe` |

### `java/util/concurrent/locks/StampedLock$ReaderNode`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `casCowaiters(Ljava/util/concurrent/locks/StampedLock$ReaderNode;Ljava/util/concurrent/locks/StampedLock$ReaderNode;)Z` | `jdk/internal/misc/Unsafe` |
| `setCowaitersRelaxed(Ljava/util/concurrent/locks/StampedLock$ReaderNode;)V` | `jdk/internal/misc/Unsafe` |

### `java/util/jar/Attributes`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `read(Ljava/util/jar/Manifest$FastInputStream;[BLjava/lang/String;I)I` | `sun/nio/cs/UTF_8`, `sun/util/logging/PlatformLogger` |
| `writeMain(Ljava/io/DataOutputStream;)V` | `sun/nio/cs/UTF_8` |

### `java/util/jar/JarFile`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `beginInit()Ljava/lang/Object;` | `jdk/internal/misc/ThreadTracker` |
| `endInit(Ljava/lang/Object;)V` | `jdk/internal/misc/ThreadTracker` |
| `entries()Ljava/util/Enumeration;` | `jdk/internal/access/JavaUtilZipFileAccess` |
| `getBytes(Ljava/util/zip/ZipEntry;)[B` | `sun/security/util/SignatureFileVerifier` |
| `getManEntry()Ljava/util/jar/JarEntry;` | `jdk/internal/access/JavaUtilZipFileAccess` |
| `getManifestFromReference()Ljava/util/jar/Manifest;` | `jdk/internal/access/JavaUtilZipFileAccess`, `sun/security/util/Debug` |
| `getVersionedEntry(Ljava/lang/String;Ljava/util/jar/JarEntry;)Ljava/util/jar/JarEntry;` | `jdk/internal/access/JavaUtilZipFileAccess` |
| `initializeVerifier()V` | `jdk/internal/access/JavaUtilZipFileAccess`, `sun/security/util/Debug`, `sun/security/util/ManifestEntryVerifier` |
| `isInitializing()Z` | `jdk/internal/misc/ThreadTracker` |
| `maybeInstantiateVerifier()V` | `jdk/internal/access/JavaUtilZipFileAccess` |
| `stream()Ljava/util/stream/Stream;` | `jdk/internal/access/JavaUtilZipFileAccess` |
| `versionedStream()Ljava/util/stream/Stream;` | `jdk/internal/access/JavaUtilZipFileAccess` |

### `java/util/jar/JarInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkManifest(Ljava/util/jar/JarEntry;)Ljava/util/jar/JarEntry;` | `sun/security/util/Debug`, `sun/security/util/ManifestEntryVerifier` |

### `java/util/jar/JarVerifier`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `beginEntry(Ljava/util/jar/JarEntry;Lsun/security/util/ManifestEntryVerifier;)V` | `sun/security/util/Debug`, `sun/security/util/ManifestEntryVerifier`, `sun/security/util/SignatureFileVerifier` |
| `processEntry(Lsun/security/util/ManifestEntryVerifier;)V` | `sun/security/util/Debug`, `sun/security/util/ManifestDigester`, `sun/security/util/ManifestEntryVerifier`, `sun/security/util/SignatureFileVerifier` |
| `update(ILsun/security/util/ManifestEntryVerifier;)V` | `sun/security/util/ManifestEntryVerifier` |
| `update(I[BIILsun/security/util/ManifestEntryVerifier;)V` | `sun/security/util/ManifestEntryVerifier` |

### `java/util/jar/JarVerifier$VerifierStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/jar/Manifest;Ljava/util/jar/JarEntry;Ljava/io/InputStream;Ljava/util/jar/JarVerifier;)V` | `sun/security/util/ManifestEntryVerifier` |

### `java/util/jar/Manifest`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getErrorPosition(Ljava/lang/String;I)Ljava/lang/String;` | `sun/security/util/SecurityProperties` |
| `parseName([BI)Ljava/lang/String;` | `sun/nio/cs/UTF_8` |
| `println72(Ljava/io/OutputStream;Ljava/lang/String;)V` | `sun/nio/cs/UTF_8` |
| `read(Ljava/io/InputStream;Ljava/lang/String;)V` | `sun/nio/cs/UTF_8` |

### `java/util/logging/FileHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `generate(Ljava/lang/String;III)Ljava/io/File;` | `jdk/internal/misc/VM` |

### `java/util/logging/Level`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `computeLocalizedLevelName(Ljava/util/Locale;)Ljava/lang/String;` | `jdk/internal/access/JavaUtilResourceBundleAccess` |

### `java/util/logging/Level$KnownLevel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `registerWithClassLoader(Ljava/util/logging/Level;)V` | `jdk/internal/loader/ClassLoaderValue` |

### `java/util/logging/LogManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getUserContext()Ljava/util/logging/LogManager$LoggerContext;` | `jdk/internal/access/JavaAWTAccess`, `jdk/internal/access/SharedSecrets` |
| `readPrimordialConfiguration()V` | `jdk/internal/logger/BootstrapLogger` |

### `java/util/logging/LogManager$LoggingProviderAccess`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `demandLoggerFor(Ljava/util/logging/LogManager;Ljava/lang/String;Ljava/lang/Module;)Ljava/util/logging/Logger;` | `jdk/internal/logger/DefaultLoggerFinder` |
| `run()Ljava/lang/Void;` | `sun/util/logging/internal/LoggingProviderImpl` |

### `java/util/logging/LogRecord$CallerFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `test(Ljava/lang/StackWalker$StackFrame;)Z` | `jdk/internal/logger/SurrogateLogger` |

### `java/util/logging/Logger`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `demandLogger(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Class;)Ljava/util/logging/Logger;` | `jdk/internal/logger/DefaultLoggerFinder` |
| `findResourceBundle(Ljava/lang/String;Z)Ljava/util/ResourceBundle;` | `jdk/internal/access/JavaUtilResourceBundleAccess` |
| `getAnonymousLogger(Ljava/lang/String;)Ljava/util/logging/Logger;` | `jdk/internal/reflect/Reflection` |
| `getLogger(Ljava/lang/String;)Ljava/util/logging/Logger;` | `jdk/internal/reflect/Reflection` |
| `getLogger(Ljava/lang/String;Ljava/lang/String;)Ljava/util/logging/Logger;` | `jdk/internal/reflect/Reflection` |
| `setupResourceInfo(Ljava/lang/String;Ljava/lang/Module;)V` | `jdk/internal/logger/DefaultLoggerFinder` |

### `java/util/logging/SimpleFormatter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `jdk/internal/logger/SurrogateLogger` |

### `java/util/prefs/FileSystemPreferences`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkLockFile0ErrorCode(I)V` | `sun/util/logging/PlatformLogger` |
| `getLogger()Lsun/util/logging/PlatformLogger;` | `sun/util/logging/PlatformLogger` |
| `syncWorld()V` | `sun/util/logging/PlatformLogger` |
| `unlockFile()V` | `sun/util/logging/PlatformLogger` |

### `java/util/prefs/FileSystemPreferences$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `sun/util/logging/PlatformLogger` |

### `java/util/prefs/FileSystemPreferences$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `sun/util/logging/PlatformLogger` |

### `java/util/prefs/FileSystemPreferences$6`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `sun/util/logging/PlatformLogger` |

### `java/util/prefs/FileSystemPreferences$9`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `sun/util/logging/PlatformLogger` |

### `java/util/prefs/Preferences`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `factory1()Ljava/util/prefs/PreferencesFactory;` | `jdk/internal/util/OperatingSystem` |

### `java/util/random/RandomGenerator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `doubles(DD)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport` |
| `doubles(J)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport` |
| `doubles(JDD)Ljava/util/stream/DoubleStream;` | `jdk/internal/util/random/RandomSupport` |
| `ints(II)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport` |
| `ints(J)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport` |
| `ints(JII)Ljava/util/stream/IntStream;` | `jdk/internal/util/random/RandomSupport` |
| `longs(J)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport` |
| `longs(JJ)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport` |
| `longs(JJJ)Ljava/util/stream/LongStream;` | `jdk/internal/util/random/RandomSupport` |
| `nextDouble(D)D` | `jdk/internal/util/random/RandomSupport` |
| `nextDouble(DD)D` | `jdk/internal/util/random/RandomSupport` |
| `nextExponential()D` | `jdk/internal/util/random/RandomSupport` |
| `nextFloat(F)F` | `jdk/internal/util/random/RandomSupport` |
| `nextFloat(FF)F` | `jdk/internal/util/random/RandomSupport` |
| `nextGaussian()D` | `jdk/internal/util/random/RandomSupport` |
| `nextGaussian(DD)D` | `jdk/internal/util/random/RandomSupport` |
| `nextInt(I)I` | `jdk/internal/util/random/RandomSupport` |
| `nextInt(II)I` | `jdk/internal/util/random/RandomSupport` |
| `nextLong(J)J` | `jdk/internal/util/random/RandomSupport` |
| `nextLong(JJ)J` | `jdk/internal/util/random/RandomSupport` |

### `java/util/random/RandomGenerator$StreamableGenerator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `rngs(J)Ljava/util/stream/Stream;` | `jdk/internal/util/random/RandomSupport` |

### `java/util/random/RandomGeneratorFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `equidistribution()I` | `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` |
| `group()Ljava/lang/String;` | `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` |
| `isHardware()Z` | `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` |
| `isStatistical()Z` | `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` |
| `isStochastic()Z` | `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` |
| `period()Ljava/math/BigInteger;` | `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` |
| `stateBits()I` | `jdk/internal/util/random/RandomSupport$RandomGeneratorProperties` |

### `java/util/regex/Pattern`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `append(II)V` | `jdk/internal/util/ArraysSupport` |
| `getClass(I)I` | `sun/text/Normalizer` |
| `normalizeSlice(Ljava/lang/String;IILjava/lang/StringBuilder;)V` | `jdk/internal/util/regex/Grapheme` |

### `java/util/regex/Pattern$GraphemeBound`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `match(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z` | `jdk/internal/util/regex/Grapheme` |

### `java/util/regex/Pattern$NFCCharProperty`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `match(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z` | `jdk/internal/util/regex/Grapheme` |

### `java/util/regex/Pattern$XGrapheme`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `match(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z` | `jdk/internal/util/regex/Grapheme` |

### `java/util/spi/AbstractResourceBundleProvider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getBundle(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;` | `sun/util/resources/Bundles` |
| `loadResourceBundle(Ljava/lang/Module;Ljava/lang/String;)Ljava/util/ResourceBundle;` | `jdk/internal/access/JavaUtilResourceBundleAccess`, `sun/security/util/SecurityConstants` |

### `java/util/stream/Collectors`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lambda$toUnmodifiableList$6(Ljava/util/ArrayList;)Ljava/util/List;` | `jdk/internal/access/JavaUtilCollectionAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/stream/ReferencePipeline`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toList()Ljava/util/List;` | `jdk/internal/access/JavaUtilCollectionAccess`, `jdk/internal/access/SharedSecrets` |

### `java/util/stream/Tripwire`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `trip(Ljava/lang/Class;Ljava/lang/String;)V` | `sun/util/logging/PlatformLogger` |

### `java/util/zip/Adler32`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `update(Ljava/nio/ByteBuffer;)V` | `jdk/internal/access/JavaNioAccess`, `sun/nio/ch/DirectBuffer` |
| `update([BII)V` | `jdk/internal/util/Preconditions` |

### `java/util/zip/CRC32`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `update(Ljava/nio/ByteBuffer;)V` | `jdk/internal/access/JavaNioAccess`, `sun/nio/ch/DirectBuffer` |
| `update([BII)V` | `jdk/internal/util/Preconditions` |
| `updateBytesCheck([BII)V` | `jdk/internal/util/Preconditions` |

### `java/util/zip/CRC32C`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `update(Ljava/nio/ByteBuffer;)V` | `jdk/internal/access/JavaNioAccess`, `sun/nio/ch/DirectBuffer` |
| `update([BII)V` | `jdk/internal/util/Preconditions` |
| `updateBytes(I[BII)I` | `jdk/internal/misc/Unsafe` |
| `updateDirectByteBuffer(IJII)I` | `jdk/internal/misc/Unsafe` |

### `java/util/zip/Deflater`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `deflate(Ljava/nio/ByteBuffer;I)I` | `jdk/internal/access/JavaNioAccess`, `sun/nio/ch/DirectBuffer` |
| `deflate([BIII)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/util/Preconditions`, `sun/nio/ch/DirectBuffer` |
| `setDictionary(Ljava/nio/ByteBuffer;)V` | `jdk/internal/access/JavaNioAccess`, `sun/nio/ch/DirectBuffer` |
| `setDictionary([BII)V` | `jdk/internal/util/Preconditions` |
| `setInput([BII)V` | `jdk/internal/util/Preconditions` |

### `java/util/zip/Deflater$DeflaterZStreamRef`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/zip/Deflater;J)V` | `jdk/internal/ref/CleanerFactory` |

### `java/util/zip/Inflater`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `inflate(Ljava/nio/ByteBuffer;)I` | `jdk/internal/access/JavaNioAccess`, `sun/nio/ch/DirectBuffer` |
| `inflate([BII)I` | `jdk/internal/access/JavaNioAccess`, `jdk/internal/util/Preconditions`, `sun/nio/ch/DirectBuffer` |
| `setDictionary(Ljava/nio/ByteBuffer;)V` | `jdk/internal/access/JavaNioAccess`, `sun/nio/ch/DirectBuffer` |
| `setDictionary([BII)V` | `jdk/internal/util/Preconditions` |
| `setInput([BII)V` | `jdk/internal/util/Preconditions` |

### `java/util/zip/Inflater$InflaterZStreamRef`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/zip/Inflater;J)V` | `jdk/internal/ref/CleanerFactory` |

### `java/util/zip/ZipCoder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `get(Ljava/nio/charset/Charset;)Ljava/util/zip/ZipCoder;` | `sun/nio/cs/UTF_8` |

### `java/util/zip/ZipCoder$UTF8ZipCoder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkedHash([BII)I` | `jdk/internal/access/JavaLangAccess`, `jdk/internal/util/ArraysSupport` |
| `compare(Ljava/lang/String;[BIIZ)Ljava/util/zip/ZipCoder$Comparison;` | `jdk/internal/access/JavaLangAccess`, `sun/nio/cs/UTF_8` |
| `getBytes(Ljava/lang/String;)[B` | `jdk/internal/access/JavaLangAccess` |
| `toString([BII)Ljava/lang/String;` | `jdk/internal/access/JavaLangAccess` |

### `java/util/zip/ZipFile`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/File;I)V` | `sun/nio/cs/UTF_8` |
| `<init>(Ljava/io/File;ILjava/nio/charset/Charset;)V` | `jdk/internal/perf/PerfCounter` |
| `getDisableZip64ExtraFieldValidation()Z` | `sun/security/action/GetPropertyAction` |
| `getZipEntry(Ljava/lang/String;I)Ljava/util/zip/ZipEntry;` | `jdk/internal/access/JavaUtilJarAccess` |

### `java/util/zip/ZipFile$CleanableResource`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/zip/ZipFile;Ljava/util/zip/ZipCoder;Ljava/io/File;I)V` | `jdk/internal/ref/CleanerFactory` |

### `java/util/zip/ZipFile$Source`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/zip/ZipFile$Source$Key;ZLjava/util/zip/ZipCoder;)V` | `jdk/internal/access/JavaIORandomAccessFileAccess`, `jdk/internal/access/SharedSecrets`, `jdk/internal/util/OperatingSystem` |
| `isSignatureRelated(II)Z` | `sun/nio/cs/UTF_8`, `sun/security/util/SignatureFileVerifier` |

### `java/util/zip/ZipFile$ZipFileInflaterInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/zip/ZipFile;Ljava/util/zip/ZipFile$ZipFileInputStream;Ljava/util/zip/ZipFile$CleanableResource;Ljava/util/zip/Inflater;I)V` | `jdk/internal/ref/CleanerFactory` |

### `java/util/zip/ZipInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;)V` | `sun/nio/cs/UTF_8` |

### `java/util/zip/ZipOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/OutputStream;)V` | `sun/nio/cs/UTF_8` |

### `java/util/zip/ZipUtils`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getBufferArray(Ljava/nio/ByteBuffer;)[B` | `jdk/internal/misc/Unsafe` |
| `getBufferOffset(Ljava/nio/ByteBuffer;)I` | `jdk/internal/misc/Unsafe` |
| `loadLibrary()V` | `jdk/internal/loader/BootLoader` |

### `javax/crypto/Cipher`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `chooseFirstProvider()V` | `sun/security/util/Debug` |
| `getInstance(Ljava/lang/String;)Ljavax/crypto/Cipher;` | `sun/security/jca/GetInstance`, `sun/security/jca/ServiceId`, `sun/security/util/CryptoAlgorithmConstraints` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/Cipher;` | `sun/security/util/CryptoAlgorithmConstraints` |
| `init(ILjava/security/Key;)V` | `sun/security/jca/JCAUtil` |
| `init(ILjava/security/Key;Ljava/security/AlgorithmParameters;)V` | `sun/security/jca/JCAUtil` |
| `init(ILjava/security/Key;Ljava/security/AlgorithmParameters;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug` |
| `init(ILjava/security/Key;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug` |
| `init(ILjava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/jca/JCAUtil` |
| `init(ILjava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug` |
| `init(ILjava/security/cert/Certificate;)V` | `sun/security/jca/JCAUtil` |
| `init(ILjava/security/cert/Certificate;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug`, `sun/security/util/KnownOIDs` |
| `passCryptoPermCheck(Ljavax/crypto/CipherSpi;Ljava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)Z` | `sun/security/util/Debug` |

### `javax/crypto/EncryptedPrivateKeyInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;[B)V` | `sun/security/x509/AlgorithmId` |
| `<init>(Ljava/security/AlgorithmParameters;[B)V` | `sun/security/x509/AlgorithmId` |
| `<init>([B)V` | `sun/security/util/DerInputStream`, `sun/security/util/DerValue`, `sun/security/x509/AlgorithmId` |
| `checkTag(Lsun/security/util/DerValue;BLjava/lang/String;)V` | `sun/security/util/DerValue` |
| `getAlgName()Ljava/lang/String;` | `sun/security/x509/AlgorithmId` |
| `getAlgParameters()Ljava/security/AlgorithmParameters;` | `sun/security/x509/AlgorithmId` |
| `getEncoded()[B` | `sun/security/util/DerOutputStream`, `sun/security/x509/AlgorithmId` |
| `pkcs8EncodingToSpec([B)Ljava/security/spec/PKCS8EncodedKeySpec;` | `sun/security/util/DerInputStream`, `sun/security/x509/AlgorithmId` |

### `javax/crypto/ExemptionMechanism`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljavax/crypto/ExemptionMechanism;` | `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/ExemptionMechanism;` | `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/ExemptionMechanism;` | `sun/security/jca/GetInstance$Instance` |

### `javax/crypto/JceSecurity`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `canUseProvider(Ljava/security/Provider;)Z` | `sun/security/util/Debug` |
| `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;)Lsun/security/jca/GetInstance$Instance;` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;)Lsun/security/jca/GetInstance$Instance;` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/String;Ljava/security/Provider;)Lsun/security/jca/GetInstance$Instance;` | `sun/security/jca/GetInstance` |
| `setupJurisdictionPolicies()V` | `jdk/internal/util/StaticProperty`, `sun/security/util/Debug` |

### `javax/crypto/JceSecurity$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `apply(Ljavax/crypto/JceSecurity$WeakIdentityWrapper;)Ljava/lang/Object;` | `sun/security/util/Debug` |

### `javax/crypto/KEM`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljavax/crypto/KEM;` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/KEM;` | `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/KEM;` | `sun/security/jca/GetInstance$Instance` |

### `javax/crypto/KeyAgreement`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `chooseFirstProvider()V` | `sun/security/util/Debug` |
| `getInstance(Ljava/lang/String;)Ljavax/crypto/KeyAgreement;` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/KeyAgreement;` | `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/KeyAgreement;` | `sun/security/jca/GetInstance$Instance` |
| `init(Ljava/security/Key;)V` | `sun/security/jca/JCAUtil` |
| `init(Ljava/security/Key;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug` |
| `init(Ljava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/jca/JCAUtil` |
| `init(Ljava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V` | `sun/security/util/Debug` |

### `javax/crypto/KeyGenerator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/security/jca/GetInstance`, `sun/security/util/Debug` |
| `<init>(Ljavax/crypto/KeyGeneratorSpi;Ljava/security/Provider;Ljava/lang/String;)V` | `sun/security/util/Debug` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/KeyGenerator;` | `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/KeyGenerator;` | `sun/security/jca/GetInstance$Instance` |
| `init(I)V` | `sun/security/jca/JCAUtil` |
| `init(Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/jca/JCAUtil` |

### `javax/crypto/Mac`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `chooseFirstProvider()V` | `sun/security/util/Debug` |
| `getInstance(Ljava/lang/String;)Ljavax/crypto/Mac;` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/Mac;` | `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/Mac;` | `sun/security/jca/GetInstance$Instance` |
| `init(Ljava/security/Key;)V` | `sun/security/util/Debug` |
| `init(Ljava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V` | `sun/security/util/Debug` |

### `javax/crypto/SecretKeyFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/security/jca/GetInstance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/SecretKeyFactory;` | `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/SecretKeyFactory;` | `sun/security/jca/GetInstance$Instance` |

### `javax/imageio/ImageIO`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCacheInfo()Ljavax/imageio/ImageIO$CacheInfo;` | `sun/awt/AppContext` |
| `getTempDir()Ljava/lang/String;` | `sun/security/action/GetPropertyAction` |

### `javax/imageio/metadata/IIOMetadataFormatImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createStandardFormat()V` | `com/sun/imageio/plugins/common/StandardMetadataFormat` |

### `javax/imageio/plugins/bmp/BMPImageWriteParam`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/util/Locale;)V` | `com/sun/imageio/plugins/bmp/BMPCompressionTypes` |

### `javax/imageio/plugins/tiff/TIFFDirectory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createFromMetadata(Ljavax/imageio/metadata/IIOMetadata;)Ljavax/imageio/plugins/tiff/TIFFDirectory;` | `com/sun/imageio/plugins/tiff/TIFFImageMetadata` |
| `getAsMetadata()Ljavax/imageio/metadata/IIOMetadata;` | `com/sun/imageio/plugins/tiff/TIFFIFD`, `com/sun/imageio/plugins/tiff/TIFFImageMetadata` |
| `getTag(I)Ljavax/imageio/plugins/tiff/TIFFTag;` | `com/sun/imageio/plugins/tiff/TIFFIFD` |

### `javax/imageio/plugins/tiff/TIFFField`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getAsNativeNode()Lorg/w3c/dom/Node;` | `com/sun/imageio/plugins/tiff/TIFFFieldNode` |

### `javax/imageio/spi/IIORegistry`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultInstance()Ljavax/imageio/spi/IIORegistry;` | `sun/awt/AppContext` |
| `registerStandardSpis()V` | `com/sun/imageio/plugins/bmp/BMPImageReaderSpi`, `com/sun/imageio/plugins/bmp/BMPImageWriterSpi`, `com/sun/imageio/plugins/gif/GIFImageReaderSpi`, `com/sun/imageio/plugins/gif/GIFImageWriterSpi`, +14 |

### `javax/imageio/stream/FileCacheImageInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;Ljava/io/File;)V` | `com/sun/imageio/stream/StreamCloser`, `com/sun/imageio/stream/StreamFinalizer`, `sun/java2d/Disposer` |
| `close()V` | `com/sun/imageio/stream/StreamCloser`, `sun/java2d/DisposerRecord` |

### `javax/imageio/stream/FileCacheImageOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/OutputStream;Ljava/io/File;)V` | `com/sun/imageio/stream/StreamCloser` |
| `close()V` | `com/sun/imageio/stream/StreamCloser` |

### `javax/imageio/stream/FileImageInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/RandomAccessFile;)V` | `com/sun/imageio/stream/CloseableDisposerRecord`, `com/sun/imageio/stream/StreamFinalizer`, `sun/java2d/Disposer` |
| `close()V` | `com/sun/imageio/stream/CloseableDisposerRecord` |

### `javax/imageio/stream/FileImageOutputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/RandomAccessFile;)V` | `com/sun/imageio/stream/CloseableDisposerRecord`, `com/sun/imageio/stream/StreamFinalizer`, `sun/java2d/Disposer` |
| `close()V` | `com/sun/imageio/stream/CloseableDisposerRecord` |

### `javax/imageio/stream/ImageInputStreamImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readInt()I` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `readShort()S` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `toChars([B[CII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `toDoubles([B[DII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `toFloats([B[FII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `toInts([B[III)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `toLongs([B[JII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `toShorts([B[SII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |

### `javax/imageio/stream/ImageOutputStreamImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `writeChars(Ljava/lang/String;)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeChars([CII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeDoubles([DII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeFloats([FII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeInt(I)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeInts([III)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeLong(J)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeLongs([JII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeShort(I)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |
| `writeShorts([SII)V` | `jdk/internal/util/ByteArray`, `jdk/internal/util/ByteArrayLittleEndian` |

### `javax/imageio/stream/MemoryCacheImageInputStream`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;)V` | `com/sun/imageio/stream/StreamFinalizer`, `sun/java2d/Disposer` |
| `close()V` | `sun/java2d/DisposerRecord` |

### `javax/management/ImmutableDescriptor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `hashCode()I` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/JMX`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createProxy(Ljavax/management/MBeanServerConnection;Ljavax/management/ObjectName;Ljava/lang/Class;ZZ)Ljava/lang/Object;` | `com/sun/jmx/mbeanserver/Introspector` |
| `isMXBeanInterface(Ljava/lang/Class;)Z` | `com/sun/jmx/mbeanserver/Introspector` |

### `javax/management/MBeanAttributeInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/reflect/Method;Ljava/lang/reflect/Method;)V` | `com/sun/jmx/mbeanserver/Introspector` |

### `javax/management/MBeanConstructorInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/reflect/Constructor;)V` | `com/sun/jmx/mbeanserver/Introspector` |

### `javax/management/MBeanOperationInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/reflect/Method;)V` | `com/sun/jmx/mbeanserver/Introspector` |
| `parameters([Ljava/lang/Class;[[Ljava/lang/annotation/Annotation;)[Ljavax/management/MBeanParameterInfo;` | `com/sun/jmx/mbeanserver/Introspector` |

### `javax/management/MBeanServerBuilder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newMBeanServer(Ljava/lang/String;Ljavax/management/MBeanServer;Ljavax/management/MBeanServerDelegate;)Ljavax/management/MBeanServer;` | `com/sun/jmx/mbeanserver/JmxMBeanServer` |
| `newMBeanServerDelegate()Ljavax/management/MBeanServerDelegate;` | `com/sun/jmx/mbeanserver/JmxMBeanServer` |

### `javax/management/MBeanServerDelegate`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getMBeanServerId()Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/MBeanServerFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkMBeanServerBuilder()V` | `com/sun/jmx/defaults/JmxProperties`, `com/sun/jmx/mbeanserver/GetPropertyAction` |
| `loadBuilderClass(Ljava/lang/String;)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |
| `mBeanServerId(Ljavax/management/MBeanServer;)Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `removeMBeanServer(Ljavax/management/MBeanServer;)V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/MBeanServerInvocationHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `findMXBeanProxy(Ljava/lang/Class;)Lcom/sun/jmx/mbeanserver/MXBeanProxy;` | `com/sun/jmx/mbeanserver/MXBeanProxy` |
| `invoke(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;` | `com/sun/jmx/mbeanserver/MXBeanProxy` |

### `javax/management/NotificationBroadcasterSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `sendNotification(Ljavax/management/Notification;)V` | `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/NotificationBroadcasterSupport$SendNotifJob`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/ObjectName`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljavax/management/ObjectName;)Ljavax/management/ObjectName;` | `com/sun/jmx/mbeanserver/Util` |
| `matchDomains(Ljavax/management/ObjectName;)Z` | `com/sun/jmx/mbeanserver/Util` |
| `matchKeys(Ljavax/management/ObjectName;)Z` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/StandardMBean`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `construct(Ljava/lang/Object;Ljava/lang/Class;ZZ)V` | `com/sun/jmx/mbeanserver/Introspector`, `com/sun/jmx/mbeanserver/MXBeanSupport`, `com/sun/jmx/mbeanserver/StandardMBeanSupport`, `com/sun/jmx/mbeanserver/Util` |
| `getAttribute(Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `getAttributes([Ljava/lang/String;)Ljavax/management/AttributeList;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `getDescriptor(Ljavax/management/MBeanInfo;Z)Ljavax/management/Descriptor;` | `com/sun/jmx/mbeanserver/DescriptorCache` |
| `getImplementation()Ljava/lang/Object;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `getImplementationClass()Ljava/lang/Class;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `getMBeanInfo()Ljavax/management/MBeanInfo;` | `com/sun/jmx/defaults/JmxProperties`, `com/sun/jmx/mbeanserver/MBeanSupport` |
| `getMBeanInterface()Ljava/lang/Class;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `invoke(Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `isMXBean()Z` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `postDeregister()V` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `postRegister(Ljava/lang/Boolean;)V` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `preRegister(Ljavax/management/MBeanServer;Ljavax/management/ObjectName;)Ljavax/management/ObjectName;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `setAttribute(Ljavax/management/Attribute;)V` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `setAttributes(Ljavax/management/AttributeList;)Ljavax/management/AttributeList;` | `com/sun/jmx/mbeanserver/MBeanSupport` |
| `setImplementation(Ljava/lang/Object;)V` | `com/sun/jmx/mbeanserver/MXBeanSupport`, `com/sun/jmx/mbeanserver/StandardMBeanSupport`, `com/sun/jmx/mbeanserver/Util` |

### `javax/management/loading/DefaultLoaderRepository`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `loadClass(Ljava/lang/String;)Ljava/lang/Class;` | `com/sun/jmx/defaults/JmxProperties` |
| `loadClassWithout(Ljava/lang/ClassLoader;Ljava/lang/String;)Ljava/lang/Class;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/loading/MLet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addURL(Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `constructParameter(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `findClass(Ljava/lang/String;Ljavax/management/loading/ClassLoaderRepository;)Ljava/lang/Class;` | `com/sun/jmx/defaults/JmxProperties` |
| `findLibrary(Ljava/lang/String;)Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `getMBeansFromURL(Ljava/lang/String;)Ljava/util/Set;` | `com/sun/jmx/defaults/JmxProperties`, `com/sun/jmx/remote/util/EnvHelp` |
| `getTmpDir()Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `loadLibraryAsResource(Ljava/lang/String;)Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `loadSerializedObject(Ljava/net/URL;Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/loading/MLetParser`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `parse(Ljava/net/URL;)Ljava/util/List;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/DescriptorSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(I)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljavax/management/modelmbean/DescriptorSupport;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>([Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>([Ljava/lang/String;[Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `clone()Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getFieldNames()[Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `getFieldValue(Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getFieldValues([Ljava/lang/String;)[Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getFields()[Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `getForm()Ljava/lang/String;` | `com/sun/jmx/mbeanserver/GetPropertyAction` |
| `hashCode()I` | `com/sun/jmx/mbeanserver/Util` |
| `isValid()Z` | `com/sun/jmx/defaults/JmxProperties` |
| `parseQuotedFieldValue(Ljava/lang/String;)Ljava/lang/Object;` | `sun/reflect/misc/ReflectUtil` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `com/sun/jmx/mbeanserver/Util` |
| `setField(Ljava/lang/String;Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setFields([Ljava/lang/String;[Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `toString()Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/ModelMBeanAttributeInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ZZZ)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ZZZLjavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/reflect/Method;Ljava/lang/reflect/Method;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/reflect/Method;Ljava/lang/reflect/Method;Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljavax/management/modelmbean/ModelMBeanAttributeInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `clone()Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getDescriptor()Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |
| `validDescriptor(Ljavax/management/Descriptor;)Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/ModelMBeanConstructorInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;[Ljavax/management/MBeanParameterInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/String;[Ljavax/management/MBeanParameterInfo;Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/reflect/Constructor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/reflect/Constructor;Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljavax/management/modelmbean/ModelMBeanConstructorInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `clone()Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getDescriptor()Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |
| `setDescriptor(Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `toString()Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `validDescriptor(Ljavax/management/Descriptor;)Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/ModelMBeanInfoSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;[Ljavax/management/modelmbean/ModelMBeanAttributeInfo;[Ljavax/management/modelmbean/ModelMBeanConstructorInfo;[Ljavax/management/modelmbean/ModelMBeanOperationInfo;[Ljavax/management/modelmbean/ModelMBeanNotificationInfo;Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljavax/management/modelmbean/ModelMBeanInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `getAttribute(Ljava/lang/String;)Ljavax/management/modelmbean/ModelMBeanAttributeInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `getConstructor(Ljava/lang/String;)Ljavax/management/modelmbean/ModelMBeanConstructorInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `getDescriptor(Ljava/lang/String;)Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |
| `getDescriptors(Ljava/lang/String;)[Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |
| `getMBeanDescriptorNoException()Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |
| `getNotification(Ljava/lang/String;)Ljavax/management/modelmbean/ModelMBeanNotificationInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `getOperation(Ljava/lang/String;)Ljavax/management/modelmbean/ModelMBeanOperationInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `setDescriptor(Ljavax/management/Descriptor;Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setDescriptors([Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setMBeanDescriptor(Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `validDescriptor(Ljavax/management/Descriptor;)Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/ModelMBeanNotificationInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>([Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `clone()Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getDescriptor()Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |
| `setDescriptor(Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `toString()Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `validDescriptor(Ljavax/management/Descriptor;)Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/ModelMBeanOperationInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;[Ljavax/management/MBeanParameterInfo;Ljava/lang/String;I)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/String;[Ljavax/management/MBeanParameterInfo;Ljava/lang/String;ILjavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/reflect/Method;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljava/lang/reflect/Method;Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljavax/management/modelmbean/ModelMBeanOperationInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `clone()Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getDescriptor()Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |
| `setDescriptor(Ljavax/management/Descriptor;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `toString()Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `validDescriptor(Ljavax/management/Descriptor;)Ljavax/management/Descriptor;` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/RequiredModelMBean`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljavax/management/modelmbean/ModelMBeanInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `addAttributeChangeNotificationListener(Ljavax/management/NotificationListener;Ljava/lang/String;Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `addNotificationListener(Ljavax/management/NotificationListener;Ljavax/management/NotificationFilter;Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `cacheResult(Ljavax/management/modelmbean/ModelMBeanOperationInfo;Ljavax/management/Descriptor;Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `findRMMBMethod(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/reflect/Method;` | `com/sun/jmx/defaults/JmxProperties`, `jdk/internal/access/JavaSecurityAccess` |
| `getAttribute(Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties`, `jdk/internal/access/JavaSecurityAccess` |
| `getAttributes([Ljava/lang/String;)Ljavax/management/AttributeList;` | `com/sun/jmx/defaults/JmxProperties` |
| `getMBeanInfo()Ljavax/management/MBeanInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `getNotificationInfo()[Ljavax/management/MBeanNotificationInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `invoke(Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties`, `jdk/internal/access/JavaSecurityAccess` |
| `invokeMethod(Ljava/lang/String;Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;` | `jdk/internal/access/JavaSecurityAccess` |
| `loadClass(Ljava/lang/String;)Ljava/lang/Class;` | `jdk/internal/access/JavaSecurityAccess` |
| `printModelMBeanInfo(Ljavax/management/modelmbean/ModelMBeanInfo;)Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `removeAttributeChangeNotificationListener(Ljavax/management/NotificationListener;Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `removeNotificationListener(Ljavax/management/NotificationListener;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `removeNotificationListener(Ljavax/management/NotificationListener;Ljavax/management/NotificationFilter;Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `resolveForCacheValue(Ljavax/management/Descriptor;)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `resolveMethod(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/reflect/Method;` | `com/sun/jmx/defaults/JmxProperties`, `jdk/internal/access/JavaSecurityAccess` |
| `sendAttributeChangeNotification(Ljavax/management/Attribute;Ljavax/management/Attribute;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendAttributeChangeNotification(Ljavax/management/AttributeChangeNotification;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendNotification(Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendNotification(Ljavax/management/Notification;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setAttribute(Ljavax/management/Attribute;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setAttributes(Ljavax/management/AttributeList;)Ljavax/management/AttributeList;` | `com/sun/jmx/defaults/JmxProperties` |
| `setManagedResource(Ljava/lang/Object;Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setModelMBeanInfo(Ljavax/management/modelmbean/ModelMBeanInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `writeToLog(Ljava/lang/String;Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/modelmbean/RequiredModelMBean$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/modelmbean/RequiredModelMBean$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `com/sun/jmx/defaults/JmxProperties`, `sun/reflect/misc/ReflectUtil` |

### `javax/management/modelmbean/RequiredModelMBean$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/modelmbean/RequiredModelMBean$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Object;` | `sun/reflect/misc/MethodUtil`, `sun/reflect/misc/ReflectUtil` |

### `javax/management/modelmbean/RequiredModelMBean$5`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/modelmbean/RequiredModelMBean$6`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/monitor/CounterMonitor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setDerivedGaugeWithDifference(Ljava/lang/Number;Ljava/lang/Number;Ljavax/management/monitor/CounterMonitor$CounterMonitorObservedObject;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `start()V` | `com/sun/jmx/defaults/JmxProperties` |
| `updateNotifications(Ljavax/management/monitor/CounterMonitor$CounterMonitorObservedObject;)Ljavax/management/monitor/MonitorNotification;` | `com/sun/jmx/defaults/JmxProperties` |
| `updateThreshold(Ljavax/management/monitor/CounterMonitor$CounterMonitorObservedObject;)V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/monitor/GaugeMonitor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isFirstGreaterThanLast(Ljava/lang/Number;Ljava/lang/Number;Ljavax/management/monitor/Monitor$NumericalType;)Z` | `com/sun/jmx/defaults/JmxProperties` |
| `isFirstStrictlyGreaterThanLast(Ljava/lang/Number;Ljava/lang/Number;Ljava/lang/String;)Z` | `com/sun/jmx/defaults/JmxProperties` |
| `setDerivedGaugeWithDifference(Ljava/lang/Number;Ljavax/management/monitor/GaugeMonitor$GaugeMonitorObservedObject;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `start()V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/monitor/Monitor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `doStart()V` | `com/sun/jmx/defaults/JmxProperties` |
| `doStop()V` | `com/sun/jmx/defaults/JmxProperties` |
| `getComparableFromAttribute(Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Comparable;` | `com/sun/jmx/mbeanserver/Introspector` |
| `monitor(Ljavax/management/monitor/Monitor$ObservedObject;I[I)V` | `com/sun/jmx/defaults/JmxProperties` |
| `preDeregister()V` | `com/sun/jmx/defaults/JmxProperties` |
| `preRegister(Ljavax/management/MBeanServer;Ljavax/management/ObjectName;)Ljavax/management/ObjectName;` | `com/sun/jmx/defaults/JmxProperties` |
| `sendNotification(Ljava/lang/String;JLjava/lang/String;Ljava/lang/Object;Ljava/lang/Object;Ljavax/management/ObjectName;Z)V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/monitor/StringMonitor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `start()V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/openmbean/CompositeDataInvocationHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `invoke(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;` | `com/sun/jmx/mbeanserver/DefaultMXBeanMappingFactory`, `com/sun/jmx/mbeanserver/MXBeanMapping`, `com/sun/jmx/mbeanserver/MXBeanMappingFactory` |

### `javax/management/openmbean/OpenMBeanAttributeInfoSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `convertFromString(Ljava/lang/String;Ljavax/management/openmbean/OpenType;)Ljava/lang/Object;` | `sun/reflect/misc/MethodUtil`, `sun/reflect/misc/ReflectUtil` |
| `convertFromStringArray(Ljava/lang/Object;Ljavax/management/openmbean/OpenType;)Ljava/lang/Object;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/openmbean/OpenType`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `checkClassNameOverride()V` | `com/sun/jmx/mbeanserver/GetPropertyAction` |

### `javax/management/openmbean/TabularDataSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/openmbean/TabularType;IF)V` | `com/sun/jmx/mbeanserver/GetPropertyAction` |
| `entrySet()Ljava/util/Set;` | `com/sun/jmx/mbeanserver/Util` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/access/JavaObjectInputStreamAccess`, `jdk/internal/access/SharedSecrets` |
| `values()Ljava/util/Collection;` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/relation/MBeanServerNotificationFilter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `com/sun/jmx/defaults/JmxProperties` |
| `disableAllObjectNames()V` | `com/sun/jmx/defaults/JmxProperties` |
| `disableObjectName(Ljavax/management/ObjectName;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `enableAllObjectNames()V` | `com/sun/jmx/defaults/JmxProperties` |
| `enableObjectName(Ljavax/management/ObjectName;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `isNotificationEnabled(Ljavax/management/Notification;)Z` | `com/sun/jmx/defaults/JmxProperties` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/relation/RelationNotification`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/relation/RelationService`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Z)V` | `com/sun/jmx/defaults/JmxProperties` |
| `addNewMBeanReference(Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/String;)Z` | `com/sun/jmx/defaults/JmxProperties` |
| `addRelation(Ljavax/management/ObjectName;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `addRelationInt(ZLjavax/management/relation/RelationSupport;Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/String;Ljavax/management/relation/RoleList;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `addRelationType(Ljavax/management/relation/RelationType;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `addRelationTypeInt(Ljavax/management/relation/RelationType;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `checkRoleInt(ILjava/lang/String;Ljava/util/List;Ljavax/management/relation/RoleInfo;Z)Ljava/lang/Integer;` | `com/sun/jmx/defaults/JmxProperties` |
| `checkRoleReading(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Integer;` | `com/sun/jmx/defaults/JmxProperties` |
| `checkRoleWriting(Ljavax/management/relation/Role;Ljava/lang/String;Ljava/lang/Boolean;)Ljava/lang/Integer;` | `com/sun/jmx/defaults/JmxProperties` |
| `createRelation(Ljava/lang/String;Ljava/lang/String;Ljavax/management/relation/RoleList;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `createRelationType(Ljava/lang/String;[Ljavax/management/relation/RoleInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `findAssociatedMBeans(Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Map;` | `com/sun/jmx/defaults/JmxProperties` |
| `findReferencingRelations(Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Map;` | `com/sun/jmx/defaults/JmxProperties` |
| `findRelationsOfType(Ljava/lang/String;)Ljava/util/List;` | `com/sun/jmx/defaults/JmxProperties` |
| `getAllRoles(Ljava/lang/String;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `getNotificationInfo()[Ljavax/management/MBeanNotificationInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `getReferencedMBeans(Ljava/lang/String;)Ljava/util/Map;` | `com/sun/jmx/defaults/JmxProperties`, `com/sun/jmx/mbeanserver/Util` |
| `getRelation(Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRelationType(Ljava/lang/String;)Ljavax/management/relation/RelationType;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRelationTypeName(Ljava/lang/String;)Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRole(Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;` | `com/sun/jmx/defaults/JmxProperties`, `com/sun/jmx/mbeanserver/Util` |
| `getRoleCardinality(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Integer;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRoleInfo(Ljava/lang/String;Ljava/lang/String;)Ljavax/management/relation/RoleInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRoleInfos(Ljava/lang/String;)Ljava/util/List;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRoles(Ljava/lang/String;[Ljava/lang/String;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `handleNotification(Ljavax/management/Notification;Ljava/lang/Object;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `handleReferenceUnregistration(Ljava/lang/String;Ljavax/management/ObjectName;Ljava/util/List;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `hasRelation(Ljava/lang/String;)Ljava/lang/Boolean;` | `com/sun/jmx/defaults/JmxProperties` |
| `initializeMissingRoles(ZLjavax/management/relation/RelationSupport;Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/String;Ljava/util/List;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `isRelation(Ljavax/management/ObjectName;)Ljava/lang/String;` | `com/sun/jmx/defaults/JmxProperties` |
| `isRelationMBean(Ljava/lang/String;)Ljavax/management/ObjectName;` | `com/sun/jmx/defaults/JmxProperties` |
| `purgeRelations()V` | `com/sun/jmx/defaults/JmxProperties` |
| `removeMBeanReference(Ljavax/management/ObjectName;Ljava/lang/String;Ljava/lang/String;Z)Z` | `com/sun/jmx/defaults/JmxProperties` |
| `removeRelation(Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `removeRelationType(Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendNotificationInt(ILjava/lang/String;Ljava/lang/String;Ljava/util/List;Ljava/lang/String;Ljava/util/List;Ljava/util/List;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendRelationCreationNotification(Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendRelationRemovalNotification(Ljava/lang/String;Ljava/util/List;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendRoleUpdateNotification(Ljava/lang/String;Ljavax/management/relation/Role;Ljava/util/List;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setRole(Ljava/lang/String;Ljavax/management/relation/Role;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setRoles(Ljava/lang/String;Ljavax/management/relation/RoleList;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `updateRoleMap(Ljava/lang/String;Ljavax/management/relation/Role;Ljava/util/List;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `updateUnregistrationListener(Ljava/util/List;Ljava/util/List;)V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/relation/RelationSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljavax/management/ObjectName;Ljava/lang/String;Ljavax/management/relation/RoleList;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;Ljavax/management/ObjectName;Ljavax/management/MBeanServer;Ljava/lang/String;Ljavax/management/relation/RoleList;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `getAllRoles()Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `getAllRolesInt(ZLjavax/management/relation/RelationService;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `getReferencedMBeans()Ljava/util/Map;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRole(Ljava/lang/String;)Ljava/util/List;` | `com/sun/jmx/defaults/JmxProperties`, `com/sun/jmx/mbeanserver/Util` |
| `getRoleCardinality(Ljava/lang/String;)Ljava/lang/Integer;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRoleInt(Ljava/lang/String;ZLjavax/management/relation/RelationService;Z)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRoles([Ljava/lang/String;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `getRolesInt([Ljava/lang/String;ZLjavax/management/relation/RelationService;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `handleMBeanUnregistration(Ljavax/management/ObjectName;Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `handleMBeanUnregistrationInt(Ljavax/management/ObjectName;Ljava/lang/String;ZLjavax/management/relation/RelationService;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `initMembers(Ljava/lang/String;Ljavax/management/ObjectName;Ljavax/management/MBeanServer;Ljava/lang/String;Ljavax/management/relation/RoleList;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `initRoleMap(Ljavax/management/relation/RoleList;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `retrieveAllRoles()Ljavax/management/relation/RoleList;` | `com/sun/jmx/defaults/JmxProperties` |
| `sendRoleUpdateNotification(Ljavax/management/relation/Role;Ljava/util/List;ZLjavax/management/relation/RelationService;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setRole(Ljavax/management/relation/Role;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `setRoleInt(Ljavax/management/relation/Role;ZLjavax/management/relation/RelationService;Z)Ljava/lang/Object;` | `com/sun/jmx/defaults/JmxProperties` |
| `setRoles(Ljavax/management/relation/RoleList;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `setRolesInt(Ljavax/management/relation/RoleList;ZLjavax/management/relation/RelationService;)Ljavax/management/relation/RoleResult;` | `com/sun/jmx/defaults/JmxProperties` |
| `updateRelationServiceMap(Ljavax/management/relation/Role;Ljava/util/List;ZLjavax/management/relation/RelationService;)V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/relation/RelationTypeSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `<init>(Ljava/lang/String;[Ljavax/management/relation/RoleInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `addRoleInfo(Ljavax/management/relation/RoleInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `getRoleInfo(Ljava/lang/String;)Ljavax/management/relation/RoleInfo;` | `com/sun/jmx/defaults/JmxProperties` |
| `initMembers(Ljava/lang/String;[Ljavax/management/relation/RoleInfo;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/relation/Role`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/relation/RoleList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `asList()Ljava/util/List;` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/relation/RoleUnresolved`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/relation/RoleUnresolvedList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `asList()Ljava/util/List;` | `com/sun/jmx/mbeanserver/Util` |

### `javax/management/remote/JMXConnectorFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getProvider(Ljava/lang/String;Ljava/lang/String;Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;` | `com/sun/jmx/mbeanserver/Util` |
| `newJMXConnector(Ljavax/management/remote/JMXServiceURL;Ljava/util/Map;)Ljavax/management/remote/JMXConnector;` | `com/sun/jmx/remote/util/EnvHelp` |

### `javax/management/remote/JMXConnectorFactory$2$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `loadClass(Ljava/lang/String;Z)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/remote/JMXConnectorFactory$ProviderFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `test(Ljava/util/ServiceLoader$Provider;)Z` | `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/remote/JMXConnectorServerFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newJMXConnectorServer(Ljavax/management/remote/JMXServiceURL;Ljava/util/Map;Ljavax/management/MBeanServer;)Ljavax/management/remote/JMXConnectorServer;` | `com/sun/jmx/remote/util/EnvHelp` |

### `javax/management/remote/JMXServiceURL`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `validateHost(Ljava/lang/String;I)V` | `com/sun/jmx/remote/util/EnvHelp` |

### `javax/management/remote/rmi/RMIConnectionImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/remote/rmi/RMIServerImpl;Ljava/lang/String;Ljava/lang/ClassLoader;Ljavax/security/auth/Subject;Ljava/util/Map;)V` | `com/sun/jmx/remote/security/JMXSubjectDomainCombiner`, `com/sun/jmx/remote/security/SubjectDelegator`, `com/sun/jmx/remote/util/EnvHelp` |
| `addNotificationListener(Ljavax/management/ObjectName;Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;Ljava/rmi/MarshalledObject;Ljavax/security/auth/Subject;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `addNotificationListeners([Ljavax/management/ObjectName;[Ljava/rmi/MarshalledObject;[Ljavax/security/auth/Subject;)[Ljava/lang/Integer;` | `com/sun/jmx/remote/internal/ServerNotifForwarder`, `com/sun/jmx/remote/util/ClassLogger` |
| `close()V` | `com/sun/jmx/remote/internal/ServerCommunicatorAdmin`, `com/sun/jmx/remote/internal/ServerNotifForwarder`, `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;[Ljava/lang/String;Ljavax/security/auth/Subject;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;[Ljava/lang/String;Ljavax/security/auth/Subject;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;Ljavax/management/ObjectName;Ljavax/security/auth/Subject;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;Ljavax/security/auth/Subject;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/util/ClassLogger` |
| `doOperation(I[Ljava/lang/Object;)Ljava/lang/Object;` | `com/sun/jmx/remote/internal/ServerNotifForwarder` |
| `doPrivilegedOperation(I[Ljava/lang/Object;Ljavax/security/auth/Subject;)Ljava/lang/Object;` | `com/sun/jmx/remote/internal/ServerCommunicatorAdmin`, `com/sun/jmx/remote/security/SubjectDelegator` |
| `fetchNotifications(JIJ)Ljavax/management/remote/NotificationResult;` | `com/sun/jmx/remote/internal/ServerCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getAttribute(Ljavax/management/ObjectName;Ljava/lang/String;Ljavax/security/auth/Subject;)Ljava/lang/Object;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getAttributes(Ljavax/management/ObjectName;[Ljava/lang/String;Ljavax/security/auth/Subject;)Ljavax/management/AttributeList;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getDefaultDomain(Ljavax/security/auth/Subject;)Ljava/lang/String;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getDomains(Ljavax/security/auth/Subject;)[Ljava/lang/String;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getMBeanCount(Ljavax/security/auth/Subject;)Ljava/lang/Integer;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getMBeanInfo(Ljavax/management/ObjectName;Ljavax/security/auth/Subject;)Ljavax/management/MBeanInfo;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getObjectInstance(Ljavax/management/ObjectName;Ljavax/security/auth/Subject;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getServerNotifFwd()Lcom/sun/jmx/remote/internal/ServerNotifForwarder;` | `com/sun/jmx/remote/internal/ServerNotifForwarder` |
| `invoke(Ljavax/management/ObjectName;Ljava/lang/String;Ljava/rmi/MarshalledObject;[Ljava/lang/String;Ljavax/security/auth/Subject;)Ljava/lang/Object;` | `com/sun/jmx/remote/util/ClassLogger` |
| `isInstanceOf(Ljavax/management/ObjectName;Ljava/lang/String;Ljavax/security/auth/Subject;)Z` | `com/sun/jmx/remote/util/ClassLogger` |
| `queryMBeans(Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;Ljavax/security/auth/Subject;)Ljava/util/Set;` | `com/sun/jmx/remote/util/ClassLogger` |
| `queryNames(Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;Ljavax/security/auth/Subject;)Ljava/util/Set;` | `com/sun/jmx/remote/util/ClassLogger` |
| `removeNotificationListener(Ljavax/management/ObjectName;Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;Ljava/rmi/MarshalledObject;Ljavax/security/auth/Subject;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `removeNotificationListener(Ljavax/management/ObjectName;Ljavax/management/ObjectName;Ljavax/security/auth/Subject;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `removeNotificationListeners(Ljavax/management/ObjectName;[Ljava/lang/Integer;Ljavax/security/auth/Subject;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `setAttribute(Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;Ljavax/security/auth/Subject;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `setAttributes(Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;Ljavax/security/auth/Subject;)Ljavax/management/AttributeList;` | `com/sun/jmx/remote/util/ClassLogger` |
| `unreferenced()V` | `com/sun/jmx/remote/util/ClassLogger` |
| `unregisterMBean(Ljavax/management/ObjectName;Ljavax/security/auth/Subject;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `unwrap(Ljava/rmi/MarshalledObject;Ljava/lang/ClassLoader;Ljava/lang/Class;Ljavax/security/auth/Subject;)Ljava/lang/Object;` | `com/sun/jmx/remote/security/SubjectDelegator`, `com/sun/jmx/remote/util/ClassLogger` |
| `unwrap(Ljava/rmi/MarshalledObject;Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;Ljava/lang/Class;Ljavax/security/auth/Subject;)Ljava/lang/Object;` | `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/remote/rmi/RMIConnectionImpl$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Lcom/sun/jmx/remote/util/ClassLoaderWithRepository;` | `com/sun/jmx/remote/util/ClassLoaderWithRepository` |

### `javax/management/remote/rmi/RMIConnectionImpl$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljavax/management/remote/NotificationResult;` | `com/sun/jmx/remote/internal/ServerNotifForwarder` |

### `javax/management/remote/rmi/RMIConnectionImpl$7`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/ClassLoader;` | `com/sun/jmx/remote/util/OrderClassLoaders` |

### `javax/management/remote/rmi/RMIConnectionImpl$CombinedClassLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `loadClass(Ljava/lang/String;Z)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/remote/rmi/RMIConnectionImpl$RMIServerCommunicatorAdmin`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/remote/rmi/RMIConnectionImpl;J)V` | `com/sun/jmx/remote/internal/ServerCommunicatorAdmin` |
| `doStop()V` | `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/remote/rmi/RMIConnector`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/remote/rmi/RMIServer;Ljavax/management/remote/JMXServiceURL;Ljava/util/Map;)V` | `com/sun/jmx/remote/util/EnvHelp` |
| `addListenerWithSubject(Ljavax/management/ObjectName;Ljava/rmi/MarshalledObject;Ljavax/security/auth/Subject;Z)Ljava/lang/Integer;` | `com/sun/jmx/remote/util/ClassLogger` |
| `addListenersWithSubjects([Ljavax/management/ObjectName;[Ljava/rmi/MarshalledObject;[Ljavax/security/auth/Subject;Z)[Ljava/lang/Integer;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `checkStub(Ljava/rmi/Remote;Ljava/lang/Class;)V` | `sun/rmi/server/UnicastRef2`, `sun/rmi/transport/LiveRef` |
| `close(Z)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `connect(Ljava/util/Map;)V` | `com/sun/jmx/remote/util/ClassLogger`, `com/sun/jmx/remote/util/EnvHelp` |
| `findRMIServerJNDI(Ljava/lang/String;Ljava/util/Map;)Ljavax/management/remote/rmi/RMIServer;` | `com/sun/jmx/remote/util/EnvHelp` |
| `findRMIServerJRMP(Ljava/lang/String;Ljava/util/Map;)Ljavax/management/remote/rmi/RMIServer;` | `com/sun/jmx/remote/util/EnvHelp` |
| `getConnection(Ljavax/management/remote/rmi/RMIServer;Ljava/lang/Object;Z)Ljavax/management/remote/rmi/RMIConnection;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getConnectionId()Ljava/lang/String;` | `com/sun/jmx/remote/util/ClassLogger` |
| `getMBeanServerConnection(Ljavax/security/auth/Subject;)Ljavax/management/MBeanServerConnection;` | `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/remote/rmi/RMIConnector$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/reflect/Constructor;` | `jdk/internal/module/Modules` |

### `javax/management/remote/rmi/RMIConnector$ObjectInputStreamWithLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `resolveClass(Ljava/io/ObjectStreamClass;)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/management/remote/rmi/RMIConnector$RMIClientCommunicatorAdmin`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/remote/rmi/RMIConnector;J)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin` |
| `checkConnection()V` | `com/sun/jmx/remote/util/ClassLogger` |
| `doStop()V` | `com/sun/jmx/remote/util/ClassLogger` |
| `gotIOException(Ljava/io/IOException;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin` |
| `reconnectNotificationListeners([Lcom/sun/jmx/remote/internal/ClientListenerInfo;)V` | `com/sun/jmx/remote/internal/ClientListenerInfo`, `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/remote/rmi/RMIConnector$RMINotifClient`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/remote/rmi/RMIConnector;Ljava/lang/ClassLoader;Ljava/util/Map;)V` | `com/sun/jmx/remote/internal/ClientNotifForwarder` |
| `addListenerForMBeanRemovedNotif()Ljava/lang/Integer;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin` |
| `fetchNotifs(JIJ)Ljavax/management/remote/NotificationResult;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin` |
| `removeListenerForMBeanRemovedNotif(Ljava/lang/Integer;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin` |

### `javax/management/remote/rmi/RMIConnector$RemoteMBeanServerConnection`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotificationListener(Ljavax/management/ObjectName;Ljavax/management/NotificationListener;Ljavax/management/NotificationFilter;Ljava/lang/Object;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `addNotificationListener(Ljavax/management/ObjectName;Ljavax/management/ObjectName;Ljavax/management/NotificationFilter;Ljava/lang/Object;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;Ljavax/management/ObjectName;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;Ljavax/management/ObjectName;[Ljava/lang/Object;[Ljava/lang/String;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `createMBean(Ljava/lang/String;Ljavax/management/ObjectName;[Ljava/lang/Object;[Ljava/lang/String;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getAttribute(Ljavax/management/ObjectName;Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getAttributes(Ljavax/management/ObjectName;[Ljava/lang/String;)Ljavax/management/AttributeList;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getDefaultDomain()Ljava/lang/String;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getDomains()[Ljava/lang/String;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getMBeanCount()Ljava/lang/Integer;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getMBeanInfo(Ljavax/management/ObjectName;)Ljavax/management/MBeanInfo;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `getObjectInstance(Ljavax/management/ObjectName;)Ljavax/management/ObjectInstance;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `invoke(Ljavax/management/ObjectName;Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/String;)Ljava/lang/Object;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `isInstanceOf(Ljavax/management/ObjectName;Ljava/lang/String;)Z` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `isRegistered(Ljavax/management/ObjectName;)Z` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `queryMBeans(Ljavax/management/ObjectName;Ljavax/management/QueryExp;)Ljava/util/Set;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `queryNames(Ljavax/management/ObjectName;Ljavax/management/QueryExp;)Ljava/util/Set;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `removeNotificationListener(Ljavax/management/ObjectName;Ljavax/management/NotificationListener;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `removeNotificationListener(Ljavax/management/ObjectName;Ljavax/management/NotificationListener;Ljavax/management/NotificationFilter;Ljava/lang/Object;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `removeNotificationListener(Ljavax/management/ObjectName;Ljavax/management/ObjectName;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `removeNotificationListener(Ljavax/management/ObjectName;Ljavax/management/ObjectName;Ljavax/management/NotificationFilter;Ljava/lang/Object;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `setAttribute(Ljavax/management/ObjectName;Ljavax/management/Attribute;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `setAttributes(Ljavax/management/ObjectName;Ljavax/management/AttributeList;)Ljavax/management/AttributeList;` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |
| `unregisterMBean(Ljavax/management/ObjectName;)V` | `com/sun/jmx/remote/internal/ClientCommunicatorAdmin`, `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/remote/rmi/RMIConnectorServer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/management/remote/JMXServiceURL;Ljava/util/Map;Ljavax/management/remote/rmi/RMIServerImpl;Ljavax/management/MBeanServer;)V` | `com/sun/jmx/remote/util/EnvHelp` |
| `getAttributes()Ljava/util/Map;` | `com/sun/jmx/remote/util/EnvHelp` |
| `start()V` | `com/sun/jmx/remote/security/MBeanServerFileAccessController`, `com/sun/jmx/remote/util/ClassLogger`, `com/sun/jmx/remote/util/EnvHelp` |
| `stop()V` | `com/sun/jmx/remote/util/ClassLogger`, `com/sun/jmx/remote/util/EnvHelp` |
| `toJMXConnector(Ljava/util/Map;)Ljavax/management/remote/JMXConnector;` | `com/sun/jmx/remote/util/EnvHelp` |

### `javax/management/remote/rmi/RMIJRMPServerImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `export(Ljava/rmi/Remote;Ljava/io/ObjectInputFilter;)V` | `com/sun/jmx/remote/internal/rmi/RMIExporter`, `com/sun/jmx/remote/util/EnvHelp`, `sun/rmi/server/UnicastServerRef`, `sun/rmi/server/UnicastServerRef2`, +1 |
| `unexport(Ljava/rmi/Remote;Z)V` | `com/sun/jmx/remote/internal/rmi/RMIExporter` |

### `javax/management/remote/rmi/RMIServerImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `clientClosed(Ljavax/management/remote/rmi/RMIConnection;)V` | `com/sun/jmx/remote/util/ClassLogger` |
| `close()V` | `com/sun/jmx/remote/internal/NotificationBuffer`, `com/sun/jmx/remote/util/ClassLogger` |
| `doNewClient(Ljava/lang/Object;)Ljavax/management/remote/rmi/RMIConnection;` | `com/sun/jmx/remote/security/JMXPluggableAuthenticator`, `com/sun/jmx/remote/util/ClassLogger` |
| `getNotifBuffer()Lcom/sun/jmx/remote/internal/NotificationBuffer;` | `com/sun/jmx/remote/internal/ArrayNotificationBuffer` |
| `makeConnectionId(Ljava/lang/String;Ljavax/security/auth/Subject;)Ljava/lang/String;` | `com/sun/jmx/remote/util/ClassLogger` |

### `javax/management/timer/Timer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addNotification(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;Ljava/util/Date;JJZ)Ljava/lang/Integer;` | `com/sun/jmx/defaults/JmxProperties` |
| `preDeregister()V` | `com/sun/jmx/defaults/JmxProperties` |
| `removeAllNotifications()V` | `com/sun/jmx/defaults/JmxProperties` |
| `removeNotification(Ljava/lang/Integer;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendNotification(Ljava/util/Date;Ljavax/management/timer/TimerNotification;)V` | `com/sun/jmx/defaults/JmxProperties` |
| `sendPastNotifications(Ljava/util/Date;Z)V` | `com/sun/jmx/defaults/JmxProperties` |
| `start()V` | `com/sun/jmx/defaults/JmxProperties` |
| `stop()V` | `com/sun/jmx/defaults/JmxProperties` |
| `updateTimerTable(Ljava/lang/Integer;)V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/management/timer/TimerAlarmClock`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `com/sun/jmx/defaults/JmxProperties` |

### `javax/naming/InitialContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `init(Ljava/util/Hashtable;)V` | `com/sun/naming/internal/ResourceManager` |

### `javax/naming/ldap/ControlFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getControlInstance(Ljavax/naming/ldap/Control;Ljavax/naming/Context;Ljava/util/Hashtable;)Ljavax/naming/ldap/Control;` | `com/sun/naming/internal/FactoryEnumeration`, `com/sun/naming/internal/ResourceManager` |

### `javax/naming/ldap/PagedResultsControl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setEncodedValue(I[B)[B` | `com/sun/jndi/ldap/BerEncoder` |

### `javax/naming/ldap/PagedResultsResponseControl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Z[B)V` | `com/sun/jndi/ldap/BerDecoder` |

### `javax/naming/ldap/SortControl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setEncodedValue([Ljavax/naming/ldap/SortKey;)[B` | `com/sun/jndi/ldap/BerEncoder` |

### `javax/naming/ldap/SortResponseControl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Z[B)V` | `com/sun/jndi/ldap/BerDecoder` |
| `getException()Ljavax/naming/NamingException;` | `com/sun/jndi/ldap/LdapCtx` |

### `javax/naming/ldap/StartTlsRequest`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createExtendedResponse(Ljava/lang/String;[BII)Ljavax/naming/ldap/ExtendedResponse;` | `com/sun/naming/internal/VersionHelper` |

### `javax/naming/spi/DirectoryManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getObjectInstance(Ljava/lang/Object;Ljavax/naming/Name;Ljavax/naming/Context;Ljava/util/Hashtable;Ljavax/naming/directory/Attributes;)Ljava/lang/Object;` | `com/sun/naming/internal/NamingManagerHelper` |
| `getStateToBind(Ljava/lang/Object;Ljavax/naming/Name;Ljavax/naming/Context;Ljava/util/Hashtable;Ljavax/naming/directory/Attributes;)Ljavax/naming/spi/DirStateFactory$Result;` | `com/sun/naming/internal/FactoryEnumeration`, `com/sun/naming/internal/ResourceManager` |

### `javax/naming/spi/NamingManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFactory(Ljava/lang/String;)Ljavax/naming/spi/InitialContextFactory;` | `com/sun/naming/internal/VersionHelper` |
| `getInitialContext(Ljava/util/Hashtable;)Ljavax/naming/Context;` | `jdk/internal/loader/AbstractClassLoaderValue$Sub`, `jdk/internal/loader/ClassLoaderValue` |
| `getObjectFactoryBuilder()Ljavax/naming/spi/ObjectFactoryBuilder;` | `com/sun/naming/internal/NamingManagerHelper` |
| `getObjectInstance(Ljava/lang/Object;Ljavax/naming/Name;Ljavax/naming/Context;Ljava/util/Hashtable;)Ljava/lang/Object;` | `com/sun/naming/internal/NamingManagerHelper` |
| `getStateToBind(Ljava/lang/Object;Ljavax/naming/Name;Ljavax/naming/Context;Ljava/util/Hashtable;)Ljava/lang/Object;` | `com/sun/naming/internal/FactoryEnumeration`, `com/sun/naming/internal/ResourceManager` |
| `getURLObject(Ljava/lang/String;Ljava/lang/Object;Ljavax/naming/Name;Ljavax/naming/Context;Ljava/util/Hashtable;)Ljava/lang/Object;` | `com/sun/naming/internal/ResourceManager` |
| `lambda$getInitialContext$1(Ljava/lang/ClassLoader;Ljdk/internal/loader/AbstractClassLoaderValue$Sub;)Ljavax/naming/spi/InitialContextFactory;` | `jdk/internal/loader/AbstractClassLoaderValue$Sub` |
| `setObjectFactoryBuilder(Ljavax/naming/spi/ObjectFactoryBuilder;)V` | `com/sun/naming/internal/NamingManagerHelper` |

### `javax/net/ssl/KeyManagerFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljavax/net/ssl/KeyManagerFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/net/ssl/KeyManagerFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/net/ssl/KeyManagerFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `javax/net/ssl/SSLContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljavax/net/ssl/SSLContext;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/net/ssl/SSLContext;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/net/ssl/SSLContext;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `javax/net/ssl/TrustManagerFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljavax/net/ssl/TrustManagerFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/String;)Ljavax/net/ssl/TrustManagerFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/security/Provider;)Ljavax/net/ssl/TrustManagerFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `javax/print/PrintServiceLookup`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getServicesForContext()Ljavax/print/PrintServiceLookup$Services;` | `sun/awt/AppContext` |

### `javax/print/ServiceUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `printDialog(Ljava/awt/GraphicsConfiguration;II[Ljavax/print/PrintService;Ljavax/print/PrintService;Ljavax/print/DocFlavor;Ljavax/print/attribute/PrintRequestAttributeSet;)Ljavax/print/PrintService;` | `sun/print/ServiceDialog` |

### `javax/print/SimpleDoc`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/Object;Ljavax/print/DocFlavor;Ljavax/print/attribute/DocAttributeSet;)V` | `sun/reflect/misc/ReflectUtil` |

### `javax/print/StreamPrintServiceFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getServices()Ljavax/print/StreamPrintServiceFactory$Services;` | `sun/awt/AppContext` |

### `javax/print/attribute/standard/DialogOwner$Accessor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/print/DialogOwnerAccessor` |

### `javax/security/auth/PrivateCredentialPermission`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljava/lang/String;)V` | `sun/security/util/ResourcesMgr` |
| `init(Ljava/lang/String;)V` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/PrivateCredentialPermission$CredOwner`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toString()Ljava/lang/String;` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/Subject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `collectionNullClean(Ljava/util/Collection;)Ljava/util/LinkedList;` | `sun/security/util/ResourcesMgr` |
| `doAs(Ljavax/security/auth/Subject;Ljava/security/PrivilegedAction;)Ljava/lang/Object;` | `sun/security/util/ResourcesMgr` |
| `doAs(Ljavax/security/auth/Subject;Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;` | `sun/security/util/ResourcesMgr` |
| `doAsPrivileged(Ljavax/security/auth/Subject;Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;` | `sun/security/util/ResourcesMgr` |
| `doAsPrivileged(Ljavax/security/auth/Subject;Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;` | `sun/security/util/ResourcesMgr` |
| `getPrincipals(Ljava/lang/Class;)Ljava/util/Set;` | `sun/security/util/ResourcesMgr` |
| `getPrivateCredentials(Ljava/lang/Class;)Ljava/util/Set;` | `sun/security/util/ResourcesMgr` |
| `getPublicCredentials(Ljava/lang/Class;)Ljava/util/Set;` | `sun/security/util/ResourcesMgr` |
| `getSubject(Ljava/security/AccessControlContext;)Ljavax/security/auth/Subject;` | `sun/security/util/ResourcesMgr` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/security/util/ResourcesMgr` |
| `toString(Z)Ljava/lang/String;` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/Subject$ClassSet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `add(Ljava/lang/Object;)Z` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/Subject$SecureSet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `add(Ljava/lang/Object;)Z` | `sun/security/util/ResourcesMgr` |
| `contains(Ljava/lang/Object;)Z` | `sun/security/util/ResourcesMgr` |
| `remove(Ljava/lang/Object;)Z` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/Subject$SecureSet$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `remove()V` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/SubjectDomainCombiner`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `combine([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)[Ljava/security/ProtectionDomain;` | `sun/security/util/Debug` |
| `printInputDomains([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)V` | `sun/security/util/Debug` |

### `javax/security/auth/SubjectDomainCombiner$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `sun/security/util/Debug` |

### `javax/security/auth/callback/PasswordCallback`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `jdk/internal/ref/CleanerFactory` |
| `setPassword([C)V` | `jdk/internal/ref/CleanerFactory` |

### `javax/security/auth/kerberos/KerberosPrincipal`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;I)V` | `sun/security/krb5/KrbException`, `sun/security/krb5/PrincipalName`, `sun/security/krb5/Realm` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/security/krb5/PrincipalName`, `sun/security/krb5/Realm`, `sun/security/util/DerValue` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `sun/security/krb5/PrincipalName`, `sun/security/krb5/Realm` |

### `javax/security/auth/kerberos/KerberosTicket`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `refresh()V` | `sun/security/krb5/Credentials`, `sun/security/krb5/EncryptionKey`, `sun/security/krb5/PrincipalName` |
| `toString()Ljava/lang/String;` | `sun/security/util/HexDumpEncoder` |

### `javax/security/auth/kerberos/KeyImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/security/auth/kerberos/KerberosPrincipal;[CLjava/lang/String;)V` | `sun/security/krb5/EncryptionKey`, `sun/security/krb5/PrincipalName` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/security/krb5/Asn1Exception`, `sun/security/krb5/EncryptionKey`, `sun/security/util/DerValue` |
| `toString()Ljava/lang/String;` | `sun/security/jgss/krb5/Krb5Util` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `sun/security/krb5/Asn1Exception`, `sun/security/krb5/EncryptionKey` |

### `javax/security/auth/kerberos/KeyTab`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `exists()Z` | `sun/security/krb5/internal/ktab/KeyTab` |
| `getEncryptionKeys(Lsun/security/krb5/PrincipalName;)[Lsun/security/krb5/EncryptionKey;` | `sun/security/krb5/internal/ktab/KeyTab` |
| `getKeys(Ljavax/security/auth/kerberos/KerberosPrincipal;)[Ljavax/security/auth/kerberos/KerberosKey;` | `sun/security/krb5/EncryptionKey`, `sun/security/krb5/PrincipalName`, `sun/security/krb5/internal/ktab/KeyTab` |
| `takeSnapshot()Lsun/security/krb5/internal/ktab/KeyTab;` | `sun/security/krb5/internal/ktab/KeyTab` |

### `javax/security/auth/login/AppConfigurationEntry$LoginModuleControlFlag`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toString()Ljava/lang/String;` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/login/Configuration`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;Ljavax/security/auth/login/Configuration$Parameters;)Ljavax/security/auth/login/Configuration;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljavax/security/auth/login/Configuration$Parameters;Ljava/lang/String;)Ljavax/security/auth/login/Configuration;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljavax/security/auth/login/Configuration$Parameters;Ljava/security/Provider;)Ljavax/security/auth/login/Configuration;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `javax/security/auth/login/LoginContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljavax/security/auth/Subject;)V` | `sun/security/util/ResourcesMgr` |
| `<init>(Ljava/lang/String;Ljavax/security/auth/Subject;Ljavax/security/auth/callback/CallbackHandler;)V` | `sun/security/util/ResourcesMgr` |
| `<init>(Ljava/lang/String;Ljavax/security/auth/callback/CallbackHandler;)V` | `sun/security/util/ResourcesMgr` |
| `init(Ljava/lang/String;)V` | `sun/security/util/ResourcesMgr` |
| `invoke(Ljava/lang/String;)V` | `sun/security/util/Debug`, `sun/security/util/ResourcesMgr` |
| `logout()V` | `sun/security/util/ResourcesMgr` |

### `javax/security/auth/x500/X500Principal`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/io/InputStream;)V` | `sun/security/util/DerValue`, `sun/security/x509/X500Name` |
| `<init>(Ljava/lang/String;Ljava/util/Map;)V` | `sun/security/util/ResourcesMgr`, `sun/security/x509/X500Name` |
| `<init>([B)V` | `sun/security/x509/X500Name` |
| `equals(Ljava/lang/Object;)Z` | `sun/security/x509/X500Name` |
| `getEncoded()[B` | `sun/security/x509/X500Name` |
| `getName(Ljava/lang/String;)Ljava/lang/String;` | `sun/security/x509/X500Name` |
| `getName(Ljava/lang/String;Ljava/util/Map;)Ljava/lang/String;` | `sun/security/util/ResourcesMgr`, `sun/security/x509/X500Name` |
| `hashCode()I` | `sun/security/x509/X500Name` |
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/security/x509/X500Name` |
| `toString()Ljava/lang/String;` | `sun/security/x509/X500Name` |
| `writeObject(Ljava/io/ObjectOutputStream;)V` | `sun/security/x509/X500Name` |

### `javax/smartcardio/TerminalFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;Ljava/lang/Object;)Ljavax/smartcardio/TerminalFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/String;)Ljavax/smartcardio/TerminalFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |
| `getInstance(Ljava/lang/String;Ljava/lang/Object;Ljava/security/Provider;)Ljavax/smartcardio/TerminalFactory;` | `sun/security/jca/GetInstance`, `sun/security/jca/GetInstance$Instance` |

### `javax/sound/midi/MidiSystem`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultDevice(Ljava/lang/Class;)Ljavax/sound/midi/MidiDevice;` | `com/sun/media/sound/JDK13Services` |
| `getProviders(Ljava/lang/Class;)Ljava/util/List;` | `com/sun/media/sound/JDK13Services` |
| `getReceiver()Ljavax/sound/midi/Receiver;` | `com/sun/media/sound/MidiDeviceReceiverEnvelope`, `com/sun/media/sound/ReferenceCountingDevice` |
| `getSequencer(Z)Ljavax/sound/midi/Sequencer;` | `com/sun/media/sound/AutoConnectSequencer`, `com/sun/media/sound/ReferenceCountingDevice` |
| `getTransmitter()Ljavax/sound/midi/Transmitter;` | `com/sun/media/sound/MidiDeviceTransmitterEnvelope`, `com/sun/media/sound/ReferenceCountingDevice` |

### `javax/sound/midi/Sequence`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getMicrosecondLength()J` | `com/sun/media/sound/MidiUtils` |

### `javax/sound/midi/SysexMessage`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setMessage(I[BI)V` | `com/sun/media/sound/MidiUtils` |
| `setMessage([BI)V` | `com/sun/media/sound/MidiUtils` |

### `javax/sound/midi/Track`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `add(Ljavax/sound/midi/MidiEvent;)Z` | `com/sun/media/sound/MidiUtils` |

### `javax/sound/sampled/AudioSystem`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultMixer(Ljava/util/List;Ljavax/sound/sampled/Line$Info;)Ljavax/sound/sampled/Mixer;` | `com/sun/media/sound/JDK13Services` |
| `getProviders(Ljava/lang/Class;)Ljava/util/List;` | `com/sun/media/sound/JDK13Services` |

### `javax/sql/rowset/RowSetProvider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `defaultRowSetFactory()Ljavax/sql/rowset/RowSetFactory;` | `com/sun/rowset/RowSetFactoryImpl` |
| `getFactoryClass(Ljava/lang/String;Ljava/lang/ClassLoader;Z)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |
| `newFactory(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljavax/sql/rowset/RowSetFactory;` | `sun/reflect/misc/ReflectUtil` |

### `javax/sql/rowset/serial/SQLInputImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject()Ljava/lang/Object;` | `sun/reflect/misc/ReflectUtil` |

### `javax/sql/rowset/serial/SerialJavaObject`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFields()[Ljava/lang/reflect/Field;` | `jdk/internal/reflect/Reflection`, `sun/reflect/misc/ReflectUtil` |

### `javax/sql/rowset/spi/SyncFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance(Ljava/lang/String;)Ljavax/sql/rowset/spi/SyncProvider;` | `com/sun/rowset/providers/RIOptimisticProvider`, `sun/reflect/misc/ReflectUtil` |

### `javax/sql/rowset/spi/SyncProviderException`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getSyncResolver()Ljavax/sql/rowset/spi/SyncResolver;` | `com/sun/rowset/internal/SyncResolverImpl` |

### `javax/swing/AbstractAction`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `shouldReconfigure(Ljava/beans/PropertyChangeEvent;)Z` | `sun/security/action/GetPropertyAction` |

### `javax/swing/Autoscroller`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `actionPerformed(Ljava/awt/event/ActionEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |
| `start(Ljavax/swing/JComponent;Ljava/awt/event/MouseEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |

### `javax/swing/BufferStrategyPaintManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `beginPaint()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `copyArea(Ljavax/swing/JComponent;Ljava/awt/Graphics;IIIIIIZ)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `dispose(Ljava/util/List;)V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `endPaint()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `fetchRoot(Ljavax/swing/JComponent;)Ljava/awt/Container;` | `sun/awt/SunToolkit` |
| `flushAccumulatedRegion()Z` | `sun/awt/SubRegionShowable`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getBufferInfo(Ljava/awt/Container;)Ljavax/swing/BufferStrategyPaintManager$BufferInfo;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `paint(Ljavax/swing/JComponent;Ljavax/swing/JComponent;Ljava/awt/Graphics;IIII)Z` | `sun/java2d/SunGraphics2D`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `prepare(Ljavax/swing/JComponent;Ljava/awt/Container;ZIIII)Z` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `show(Ljava/awt/Container;IIII)Z` | `sun/awt/SubRegionShowable` |

### `javax/swing/BufferStrategyPaintManager$BufferInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createBufferStrategy()Ljava/awt/image/BufferStrategy;` | `com/sun/java/swing/SwingUtilities3`, `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `createBufferStrategy(Ljava/awt/Container;Z)Ljava/awt/image/BufferStrategy;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor`, `sun/awt/SunToolkit`, `sun/java2d/pipe/hw/ExtendedBufferCapabilities`, +3 |
| `dispose()V` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getBufferStrategy(Z)Ljava/awt/image/BufferStrategy;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `hasBufferStrategyChanged()Z` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |

### `javax/swing/ColorChooserDialog`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `initColorChooserDialog(Ljava/awt/Component;Ljavax/swing/JColorChooser;Ljava/awt/event/ActionListener;Ljava/awt/event/ActionListener;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/DefaultDesktopManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `dragFrame(Ljavax/swing/JComponent;II)V` | `sun/java2d/SunGraphics2D`, `sun/java2d/SurfaceData` |
| `dragFrameFaster(Ljavax/swing/JComponent;II)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$WindowAccessor`, `sun/awt/SunToolkit` |
| `resizeFrame(Ljavax/swing/JComponent;IIII)V` | `sun/java2d/SunGraphics2D`, `sun/java2d/SurfaceData` |

### `javax/swing/DefaultListCellRenderer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `firePropertyChange(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V` | `sun/swing/SwingUtilities2` |
| `getListCellRendererComponent(Ljavax/swing/JList;Ljava/lang/Object;IZZ)Ljava/awt/Component;` | `sun/swing/DefaultLookup` |
| `getNoFocusBorder()Ljavax/swing/border/Border;` | `sun/swing/DefaultLookup` |

### `javax/swing/GrayFilter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createDisabledImage(Ljava/awt/Image;)Ljava/awt/Image;` | `sun/awt/image/MultiResolutionCachedImage` |

### `javax/swing/ImageIcon`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getTracker()Ljava/awt/MediaTracker;` | `sun/awt/AppContext` |

### `javax/swing/ImageIcon$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/awt/Component;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |

### `javax/swing/JApplet`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/SunToolkit` |

### `javax/swing/JComponent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFontMetrics(Ljava/awt/Font;)Ljava/awt/FontMetrics;` | `sun/swing/SwingUtilities2` |
| `isLightweightComponent(Ljava/awt/Component;)Z` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |
| `repaint(JIIII)V` | `sun/awt/SunToolkit` |
| `revalidate()V` | `sun/awt/SunToolkit` |

### `javax/swing/JDialog`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `dialogInit()V` | `sun/awt/SunToolkit` |

### `javax/swing/JEditorPane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createEditorKitForContentType(Ljava/lang/String;)Ljavax/swing/text/EditorKit;` | `sun/reflect/misc/ReflectUtil` |

### `javax/swing/JFrame`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `frameInit()V` | `sun/awt/SunToolkit` |

### `javax/swing/JInternalFrame`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;ZZZZ)V` | `sun/awt/SunToolkit` |
| `addPropertyChangeListenerIfNecessary()V` | `sun/awt/AppContext` |
| `dispose()V` | `sun/awt/UngrabEvent` |
| `restoreSubcomponentFocus()V` | `sun/swing/SwingUtilities2` |
| `setSelected(Z)V` | `sun/awt/UngrabEvent` |

### `javax/swing/JLayeredPane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `validateOptimizedDrawing()V` | `sun/awt/SunToolkit` |

### `javax/swing/JList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `dropLocationForPoint(Ljava/awt/Point;)Ljavax/swing/JList$DropLocation;` | `sun/swing/SwingUtilities2`, `sun/swing/SwingUtilities2$Section` |
| `getToolTipText(Ljava/awt/event/MouseEvent;)Ljava/lang/String;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |
| `setDropLocation(Ljavax/swing/TransferHandler$DropLocation;Ljava/lang/Object;Z)Ljava/lang/Object;` | `sun/swing/SwingUtilities2` |

### `javax/swing/JMenuBar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `updateUI()V` | `sun/awt/SunToolkit` |

### `javax/swing/JOptionPane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `showInternalInputDialog(Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;ILjavax/swing/Icon;[Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ContainerAccessor` |
| `showInternalOptionDialog(Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;IILjavax/swing/Icon;[Ljava/lang/Object;Ljava/lang/Object;)I` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ContainerAccessor` |

### `javax/swing/JOptionPane$5`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ContainerAccessor` |

### `javax/swing/JPopupMenu`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `canPopupOverlapTaskBar()Z` | `sun/awt/SunToolkit` |
| `setVisible(Z)V` | `sun/awt/SunToolkit` |

### `javax/swing/JSpinner$DateEditor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultPattern(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `javax/swing/JSpinner$NumberEditor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultPattern(Ljava/util/Locale;)Ljava/lang/String;` | `sun/util/locale/provider/LocaleProviderAdapter`, `sun/util/locale/provider/LocaleResources` |

### `javax/swing/JTabbedPane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `fireStateChanged()V` | `sun/swing/SwingUtilities2` |
| `removeTabAt(I)V` | `sun/swing/SwingUtilities2` |
| `setComponentAt(ILjava/awt/Component;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/JTable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `convertColumnIndexToModel(I)I` | `sun/swing/SwingUtilities2` |
| `convertColumnIndexToView(I)I` | `sun/swing/SwingUtilities2` |
| `dropLocationForPoint(Ljava/awt/Point;)Ljavax/swing/JTable$DropLocation;` | `sun/swing/SwingUtilities2`, `sun/swing/SwingUtilities2$Section` |
| `getToolTipText(Ljava/awt/event/MouseEvent;)Ljava/lang/String;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |
| `lambda$print$11(Ljava/awt/print/PrinterJob;Ljavax/print/attribute/PrintRequestAttributeSet;Ljava/lang/Object;Lsun/swing/PrintingStatus;)V` | `sun/swing/PrintingStatus` |
| `print(Ljavax/swing/JTable$PrintMode;Ljava/text/MessageFormat;Ljava/text/MessageFormat;ZLjavax/print/attribute/PrintRequestAttributeSet;ZLjavax/print/PrintService;)Z` | `sun/swing/PrintingStatus` |
| `restoreSortingSelection([IILjavax/swing/JTable$ModelChange;)V` | `sun/swing/SwingUtilities2` |
| `selectAll()V` | `sun/swing/SwingUtilities2` |
| `setDropLocation(Ljavax/swing/TransferHandler$DropLocation;Ljava/lang/Object;Z)Ljava/lang/Object;` | `sun/swing/SwingUtilities2` |

### `javax/swing/JTable$GenericEditor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getTableCellEditorComponent(Ljavax/swing/JTable;Ljava/lang/Object;ZII)Ljava/awt/Component;` | `sun/reflect/misc/ReflectUtil`, `sun/swing/SwingUtilities2` |
| `stopCellEditing()Z` | `sun/swing/SwingUtilities2` |

### `javax/swing/JTable$SortManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `cacheSelection(Ljavax/swing/event/RowSorterEvent;Ljavax/swing/JTable$ModelChange;)V` | `sun/swing/SwingUtilities2` |
| `restoreSelection(Ljavax/swing/JTable$ModelChange;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/JTree`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `dropLocationForPoint(Ljava/awt/Point;)Ljavax/swing/JTree$DropLocation;` | `sun/swing/SwingUtilities2`, `sun/swing/SwingUtilities2$Section` |
| `getToolTipText(Ljava/awt/event/MouseEvent;)Ljava/lang/String;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |
| `removeDescendantSelectedPaths(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/JTree$TreeModelHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `treeNodesRemoved(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeStructureChanged(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/JViewport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `isFPScale()Z` | `sun/swing/SwingUtilities2` |
| `needsRepaintAfterBlit()Z` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |

### `javax/swing/JWindow`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `windowInit()V` | `sun/awt/SunToolkit` |

### `javax/swing/KeyboardManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `registerKeyStroke(Ljavax/swing/KeyStroke;Ljavax/swing/JComponent;)V` | `sun/awt/EmbeddedFrame` |
| `unregisterKeyStroke(Ljavax/swing/KeyStroke;Ljavax/swing/JComponent;)V` | `sun/awt/EmbeddedFrame` |

### `javax/swing/LayoutFocusTraversalPolicy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `accept(Ljava/awt/Component;)Z` | `sun/awt/SunToolkit` |

### `javax/swing/LayoutStyle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInstance()Ljavax/swing/LayoutStyle;` | `sun/awt/AppContext` |
| `setInstance(Ljavax/swing/LayoutStyle;)V` | `sun/awt/AppContext` |

### `javax/swing/LookAndFeel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDisabledIcon(Ljavax/swing/JComponent;Ljavax/swing/Icon;)Ljavax/swing/Icon;` | `sun/swing/ImageIconUIResource` |
| `getLayoutStyle()Ljavax/swing/LayoutStyle;` | `sun/swing/DefaultLayoutStyle` |
| `installProperty(Ljavax/swing/JComponent;Ljava/lang/String;Ljava/lang/Object;)V` | `sun/awt/SunToolkit` |
| `makeIcon(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Object;` | `sun/swing/SwingUtilities2` |

### `javax/swing/MenuSelectionManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `defaultManager()Ljavax/swing/MenuSelectionManager;` | `sun/awt/AppContext`, `sun/swing/SwingUtilities2` |
| `processMouseEvent(Ljava/awt/event/MouseEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |

### `javax/swing/PopupFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPopup(Ljava/awt/Component;Ljava/awt/Component;III)Ljavax/swing/Popup;` | `sun/awt/EmbeddedFrame`, `sun/awt/OSInfo`, `sun/awt/OSInfo$OSType` |

### `javax/swing/RepaintManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `_getOffscreenBuffer(Ljava/awt/Component;II)Ljava/awt/Image;` | `sun/awt/SunToolkit` |
| `addDirtyRegion0(Ljava/awt/Container;IIII)V` | `sun/awt/SunToolkit` |
| `addInvalidComponent(Ljavax/swing/JComponent;)V` | `sun/awt/SunToolkit` |
| `currentManager(Ljava/awt/Component;)Ljavax/swing/RepaintManager;` | `sun/awt/AppContext` |
| `currentManager(Lsun/awt/AppContext;)Ljavax/swing/RepaintManager;` | `sun/awt/AppContext` |
| `getDelegate(Ljava/awt/Component;)Ljavax/swing/RepaintManager;` | `com/sun/java/swing/SwingUtilities3` |
| `getPaintManager()Ljavax/swing/RepaintManager$PaintManager;` | `sun/awt/SunToolkit` |
| `getVolatileOffscreenBuffer(Ljava/awt/Component;II)Ljava/awt/Image;` | `sun/awt/SunToolkit` |
| `notifyRepaintPerformed(Ljavax/swing/JComponent;IIII)V` | `sun/swing/SwingUtilities2$RepaintListener` |
| `paintDirtyRegions(Ljava/util/Map;)V` | `jdk/internal/access/JavaSecurityAccess`, `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |
| `scheduleProcessingRunnable(Lsun/awt/AppContext;)V` | `sun/awt/SunToolkit` |
| `updateWindows(Ljava/util/Map;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$WindowAccessor`, `sun/awt/SunToolkit` |
| `validateInvalidComponents()V` | `jdk/internal/access/JavaSecurityAccess`, `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |

### `javax/swing/RepaintManager$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `jdk/internal/access/JavaSecurityAccess`, `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |

### `javax/swing/RepaintManager$DisplayChangedHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `scheduleDisplayChanges()V` | `sun/awt/AppContext` |

### `javax/swing/RepaintManager$PaintManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getTransform(Ljava/awt/Graphics;)Ljava/awt/geom/AffineTransform;` | `sun/java2d/SunGraphics2D` |
| `isPixelsCopying(Ljavax/swing/JComponent;Ljava/awt/Graphics;)Z` | `sun/swing/SwingUtilities2` |
| `paintDoubleBufferedFPScales(Ljavax/swing/JComponent;Ljava/awt/Image;Ljava/awt/Graphics;IIII)V` | `sun/java2d/pipe/Region` |

### `javax/swing/SortingFocusTraversalPolicy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/util/logging/PlatformLogger` |
| `<init>(Ljava/util/Comparator;)V` | `sun/util/logging/PlatformLogger` |
| `getComponentAfter(Ljava/awt/Container;Ljava/awt/Component;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getComponentBefore(Ljava/awt/Container;Ljava/awt/Component;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getComponentDownCycle(Ljava/awt/Component;I)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getComponentIndex(Ljava/util/List;Ljava/awt/Component;)I` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getFirstComponent(Ljava/awt/Container;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |
| `getLastComponent(Ljava/awt/Container;)Ljava/awt/Component;` | `sun/util/logging/PlatformLogger`, `sun/util/logging/PlatformLogger$Level` |

### `javax/swing/SwingPaintEventDispatcher`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/awt/PaintEventDispatcher` |
| `createPaintEvent(Ljava/awt/Component;IIII)Ljava/awt/event/PaintEvent;` | `sun/awt/PaintEventDispatcher`, `sun/awt/SunToolkit`, `sun/awt/event/IgnorePaintEvent` |
| `queueSurfaceDataReplacing(Ljava/awt/Component;Ljava/lang/Runnable;)Z` | `sun/awt/PaintEventDispatcher`, `sun/awt/SunToolkit` |

### `javax/swing/SwingUtilities`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `appContextGet(Ljava/lang/Object;)Ljava/lang/Object;` | `sun/awt/AppContext` |
| `appContextPut(Ljava/lang/Object;Ljava/lang/Object;)V` | `sun/awt/AppContext` |
| `appContextRemove(Ljava/lang/Object;)V` | `sun/awt/AppContext` |
| `computeStringWidth(Ljava/awt/FontMetrics;Ljava/lang/String;)I` | `sun/swing/SwingUtilities2` |
| `convertMouseEvent(Ljava/awt/Component;Ljava/awt/event/MouseEvent;Ljava/awt/Component;)Ljava/awt/event/MouseEvent;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |
| `getSuppressDropTarget()Z` | `sun/security/action/GetPropertyAction` |
| `layoutCompoundLabelImpl(Ljavax/swing/JComponent;Ljava/awt/FontMetrics;Ljava/lang/String;Ljavax/swing/Icon;IIIILjava/awt/Rectangle;Ljava/awt/Rectangle;Ljava/awt/Rectangle;I)Ljava/lang/String;` | `sun/swing/SwingUtilities2` |
| `loadSystemClass(Ljava/lang/String;)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/swing/SwingWorker`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `doneEDT()V` | `sun/swing/AccumulativeRunnable` |
| `getDoSubmit()Lsun/swing/AccumulativeRunnable;` | `sun/awt/AppContext` |
| `getWorkersExecutorService()Ljava/util/concurrent/ExecutorService;` | `sun/awt/AppContext` |
| `publish([Ljava/lang/Object;)V` | `sun/swing/AccumulativeRunnable` |
| `setProgress(I)V` | `sun/swing/AccumulativeRunnable` |

### `javax/swing/SwingWorker$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/swing/SwingWorker;)V` | `sun/swing/AccumulativeRunnable` |
| `submit()V` | `sun/swing/AccumulativeRunnable` |

### `javax/swing/SwingWorker$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/swing/SwingWorker;)V` | `sun/swing/AccumulativeRunnable` |
| `submit()V` | `sun/swing/AccumulativeRunnable` |

### `javax/swing/SwingWorker$DoSubmitAccumulativeRunnable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/AccumulativeRunnable` |

### `javax/swing/SwingWorker$SwingWorkerPropertyChangeSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `firePropertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/AccumulativeRunnable` |

### `javax/swing/TimerQueue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `sun/awt/AppContext` |
| `startIfNeeded()V` | `sun/awt/AppContext` |

### `javax/swing/TransferHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDropTargetListener()Ljava/awt/dnd/DropTargetListener;` | `sun/awt/AppContext` |
| `importData(Ljavax/swing/JComponent;Ljava/awt/datatransfer/Transferable;)Z` | `sun/reflect/misc/MethodUtil` |

### `javax/swing/TransferHandler$DropHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setComponentDropLocation(Ljavax/swing/TransferHandler$TransferSupport;Z)V` | `sun/awt/SunToolkit`, `sun/swing/SwingAccessor`, `sun/swing/SwingAccessor$JTextComponentAccessor` |

### `javax/swing/TransferHandler$PropertyTransferable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getTransferData(Ljava/awt/datatransfer/DataFlavor;)Ljava/lang/Object;` | `sun/reflect/misc/MethodUtil` |

### `javax/swing/TransferHandler$TransferAction`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |
| `actionPerformed(Ljava/awt/event/ActionEvent;)V` | `jdk/internal/access/JavaSecurityAccess`, `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$AWTEventAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |
| `getClipboard(Ljavax/swing/JComponent;)Ljava/awt/datatransfer/Clipboard;` | `sun/awt/AppContext`, `sun/swing/SwingUtilities2` |

### `javax/swing/TransferHandler$TransferAction$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Void;` | `jdk/internal/access/JavaSecurityAccess` |

### `javax/swing/TransferHandler$TransferSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setDNDVariables(Ljava/awt/Component;Ljava/awt/dnd/DropTargetEvent;)V` | `sun/awt/SunToolkit`, `sun/swing/SwingAccessor`, `sun/swing/SwingAccessor$JTextComponentAccessor` |

### `javax/swing/UIDefaults`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/reflect/misc/MethodUtil` |
| `getUIClass(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;` | `sun/reflect/misc/ReflectUtil` |

### `javax/swing/UIDefaults$ProxyLazyValue$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Object;` | `sun/reflect/misc/MethodUtil`, `sun/reflect/misc/ReflectUtil`, `sun/swing/SwingUtilities2` |

### `javax/swing/UIManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCrossPlatformLookAndFeelClassName()Ljava/lang/String;` | `sun/security/action/GetPropertyAction` |
| `getLAFState()Ljavax/swing/UIManager$LAFState;` | `sun/swing/SwingUtilities2` |
| `getSystemLookAndFeelClassName()Ljava/lang/String;` | `sun/awt/OSInfo`, `sun/awt/OSInfo$OSType`, `sun/awt/SunToolkit`, `sun/security/action/GetPropertyAction` |
| `initialize()V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor`, `sun/awt/PaintEventDispatcher` |
| `initializeDefaultLAF(Ljava/util/Properties;)V` | `sun/awt/AppContext` |
| `setLookAndFeel(Ljavax/swing/LookAndFeel;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/UIManager$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()Ljava/lang/Object;` | `sun/awt/OSInfo`, `sun/awt/OSInfo$OSType` |

### `javax/swing/border/EtchedBorder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintBorder(Ljava/awt/Component;Ljava/awt/Graphics;IIII)V` | `com/sun/java/swing/SwingUtilities3` |

### `javax/swing/border/LineBorder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintBorder(Ljava/awt/Component;Ljava/awt/Graphics;IIII)V` | `com/sun/java/swing/SwingUtilities3` |
| `paintUnscaledBorder(Ljava/awt/Component;Ljava/awt/Graphics;IID)V` | `sun/java2d/pipe/Region` |

### `javax/swing/border/TitledBorder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `installPropertyChangeListeners()V` | `jdk/internal/ref/CleanerFactory` |

### `javax/swing/colorchooser/DefaultPreviewPanel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPreferredSize()Ljava/awt/Dimension;` | `sun/swing/SwingUtilities2` |
| `paintText(Ljava/awt/Graphics;I)I` | `sun/swing/SwingUtilities2` |

### `javax/swing/event/EventListenerList`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `readObject(Ljava/io/ObjectInputStream;)V` | `sun/reflect/misc/ReflectUtil` |

### `javax/swing/filechooser/FileSystemView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `jdk/internal/ref/CleanerFactory` |
| `getChooserComboBoxFiles()[Ljava/io/File;` | `sun/awt/shell/ShellFolder` |
| `getChooserShortcutPanelFiles()[Ljava/io/File;` | `sun/awt/shell/ShellFolder` |
| `getDefaultDirectory()Ljava/io/File;` | `sun/awt/shell/ShellFolder` |
| `getFiles(Ljava/io/File;Z)[Ljava/io/File;` | `sun/awt/shell/ShellFolder` |
| `getLinkLocation(Ljava/io/File;)Ljava/io/File;` | `sun/awt/shell/ShellFolder` |
| `getParentDirectory(Ljava/io/File;)Ljava/io/File;` | `sun/awt/shell/ShellFolder` |
| `getRoots()[Ljava/io/File;` | `sun/awt/shell/ShellFolder` |
| `getShellFolder(Ljava/io/File;)Lsun/awt/shell/ShellFolder;` | `sun/awt/shell/ShellFolder` |
| `getSystemDisplayName(Ljava/io/File;)Ljava/lang/String;` | `sun/awt/shell/ShellFolder` |
| `getSystemIcon(Ljava/io/File;)Ljavax/swing/Icon;` | `sun/awt/shell/ShellFolder` |
| `getSystemIcon(Ljava/io/File;II)Ljavax/swing/Icon;` | `sun/awt/shell/ShellFolder` |
| `isComputerNode(Ljava/io/File;)Z` | `sun/awt/shell/ShellFolder` |
| `isFileSystem(Ljava/io/File;)Z` | `sun/awt/shell/ShellFolder` |
| `isFileSystemRoot(Ljava/io/File;)Z` | `sun/awt/shell/ShellFolder` |
| `isLink(Ljava/io/File;)Z` | `sun/awt/shell/ShellFolder` |

### `javax/swing/filechooser/WindowsFileSystemView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getSystemTypeDescription(Ljava/io/File;)Ljava/lang/String;` | `sun/awt/shell/ShellFolder` |

### `javax/swing/plaf/basic/BasicArrowButton`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintTriangle(Ljava/awt/Graphics;IIIIZ)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicBorders$MenuBarBorder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintBorder(Ljava/awt/Component;Ljava/awt/Graphics;IIII)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicButtonListener`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `focusGained(Ljava/awt/event/FocusEvent;)V` | `sun/swing/DefaultLookup` |
| `focusLost(Ljava/awt/event/FocusEvent;)V` | `sun/swing/DefaultLookup` |
| `getInputMap(ILjavax/swing/JComponent;)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |
| `updateMnemonicBinding(Ljavax/swing/AbstractButton;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicButtonListener$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicButtonUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `paint(Ljava/awt/Graphics;Ljavax/swing/JComponent;)V` | `sun/swing/SwingUtilities2` |
| `paintText(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljava/awt/Rectangle;Ljava/lang/String;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicCheckBoxUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |

### `javax/swing/plaf/basic/BasicColorChooserUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `installUI(Ljavax/swing/JComponent;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicComboBoxEditor`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getItem()Ljava/lang/Object;` | `sun/reflect/misc/MethodUtil` |

### `javax/swing/plaf/basic/BasicComboBoxUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultListCellRenderer()Ljavax/swing/ListCellRenderer;` | `sun/awt/AppContext` |
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `paintCurrentValue(Ljava/awt/Graphics;Ljava/awt/Rectangle;Z)V` | `sun/swing/DefaultLookup` |
| `paintCurrentValueBackground(Ljava/awt/Graphics;Ljava/awt/Rectangle;Z)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicComboBoxUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |
| `actionPerformed(Ljava/awt/event/ActionEvent;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicComboBoxUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicComboPopup`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `convertMouseEvent(Ljava/awt/event/MouseEvent;)Ljava/awt/event/MouseEvent;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |

### `javax/swing/plaf/basic/BasicComboPopup$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `processMouseEvent(Ljava/awt/event/MouseEvent;)V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |

### `javax/swing/plaf/basic/BasicDesktopPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicDesktopPaneUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/UIAction` |
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicDirectoryModel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `sort(Ljava/util/Vector;)V` | `sun/awt/shell/ShellFolder` |

### `javax/swing/plaf/basic/BasicDirectoryModel$FilesLoader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run0()V` | `sun/awt/shell/ShellFolder` |

### `javax/swing/plaf/basic/BasicEditorPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `updateCSS(Ljava/awt/Font;Ljava/awt/Color;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicFileChooserUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `changeDirectory(Ljava/io/File;)V` | `sun/awt/shell/ShellFolder`, `sun/swing/FilePane` |
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `getMnemonic(Ljava/lang/String;Ljava/util/Locale;)I` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicFileChooserUI$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/swing/plaf/basic/BasicFileChooserUI;Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicFileChooserUI$ApproveSelectionAction`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `actionPerformed(Ljava/awt/event/ActionEvent;)V` | `sun/awt/shell/ShellFolder` |

### `javax/swing/plaf/basic/BasicFileChooserUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mouseClicked(Ljava/awt/event/MouseEvent;)V` | `sun/awt/shell/ShellFolder`, `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicGraphicsUtils`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `drawString(Ljavax/swing/JComponent;Ljava/awt/Graphics2D;Ljava/lang/String;FF)V` | `sun/swing/SwingUtilities2` |
| `drawStringUnderlineCharAt(Ljava/awt/Graphics;Ljava/lang/String;III)V` | `sun/swing/SwingUtilities2` |
| `drawStringUnderlineCharAt(Ljavax/swing/JComponent;Ljava/awt/Graphics2D;Ljava/lang/String;IFF)V` | `sun/swing/SwingUtilities2` |
| `getClippedString(Ljavax/swing/JComponent;Ljava/awt/FontMetrics;Ljava/lang/String;I)Ljava/lang/String;` | `sun/swing/SwingUtilities2` |
| `getStringWidth(Ljavax/swing/JComponent;Ljava/awt/FontMetrics;Ljava/lang/String;)F` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicHTML$BasicDocument`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setFontAndColor(Ljava/awt/Font;Ljava/awt/Color;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicHTML$BasicHTMLViewFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `clearAllowHTMLObject()V` | `sun/swing/SwingAccessor` |
| `setAllowHTMLObject()V` | `sun/security/action/GetBooleanAction`, `sun/swing/SwingAccessor` |

### `javax/swing/plaf/basic/BasicInternalFrameTitlePane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getTitle(Ljava/lang/String;Ljava/awt/FontMetrics;I)Ljava/lang/String;` | `sun/swing/SwingUtilities2` |
| `paintComponent(Ljava/awt/Graphics;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicInternalFrameTitlePane$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `minimumLayoutSize(Ljava/awt/Container;)Ljava/awt/Dimension;` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicInternalFrameTitlePane$SystemMenuBar`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paint(Ljava/awt/Graphics;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicInternalFrameUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicInternalFrameUI$1`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicInternalFrameUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `layoutContainer(Ljava/awt/Container;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicLabelUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `installKeyboardActions(Ljavax/swing/JLabel;)V` | `sun/swing/SwingUtilities2` |
| `paint(Ljava/awt/Graphics;Ljavax/swing/JComponent;)V` | `sun/swing/SwingUtilities2` |
| `paintDisabledText(Ljavax/swing/JLabel;Ljava/awt/Graphics;Ljava/lang/String;II)V` | `sun/swing/SwingUtilities2` |
| `paintEnabledText(Ljavax/swing/JLabel;Ljava/awt/Graphics;Ljava/lang/String;II)V` | `sun/swing/SwingUtilities2` |
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicLabelUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |
| `doPress(Ljavax/swing/JLabel;)V` | `sun/swing/SwingUtilities2` |
| `doRelease(Ljavax/swing/JLabel;Z)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicListUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `paintDropLine(Ljava/awt/Graphics;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicListUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |
| `selectAll(Ljavax/swing/JList;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicListUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `adjustSelection(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `dragStarting(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mouseDragged(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mousePressed(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mouseReleased(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicLookAndFeel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFocusAcceleratorKeyMask()I` | `sun/awt/SunToolkit` |
| `initComponentDefaults(Ljavax/swing/UIDefaults;)V` | `sun/swing/SwingUtilities2` |
| `initResourceBundle(Ljavax/swing/UIDefaults;)V` | `sun/swing/SwingAccessor`, `sun/swing/SwingAccessor$UIDefaultsAccessor` |
| `installAWTEventListener()V` | `sun/awt/AppContext` |
| `lambda$initComponentDefaults$35(Ljavax/swing/UIDefaults;)Ljava/lang/Object;` | `sun/swing/icon/SortArrowIcon` |
| `lambda$initComponentDefaults$36(Ljavax/swing/UIDefaults;)Ljava/lang/Object;` | `sun/swing/icon/SortArrowIcon` |
| `uninitialize()V` | `sun/awt/AppContext` |

### `javax/swing/plaf/basic/BasicMenuBarUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicMenuBarUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicMenuItemUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `applyInsets(Ljava/awt/Rectangle;Ljava/awt/Insets;)V` | `com/sun/java/swing/SwingUtilities3` |
| `doNotCloseOnMouseClick()Z` | `sun/swing/SwingUtilities2` |
| `getPreferredMenuItemSize(Ljavax/swing/JComponent;Ljavax/swing/Icon;Ljavax/swing/Icon;I)Ljava/awt/Dimension;` | `sun/swing/MenuItemLayoutHelper`, `sun/swing/MenuItemLayoutHelper$RectSize` |
| `installDefaults()V` | `sun/swing/MenuItemCheckIconFactory`, `sun/swing/MenuItemLayoutHelper` |
| `paintAccText(Ljava/awt/Graphics;Lsun/swing/MenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `com/sun/java/swing/SwingUtilities3` |
| `paintArrowIcon(Ljava/awt/Graphics;Lsun/swing/MenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;Ljava/awt/Color;)V` | `com/sun/java/swing/SwingUtilities3` |
| `paintCheckIcon(Ljava/awt/Graphics;Lsun/swing/MenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;Ljava/awt/Color;Ljava/awt/Color;)V` | `com/sun/java/swing/SwingUtilities3` |
| `paintIcon(Ljava/awt/Graphics;Lsun/swing/MenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;Ljava/awt/Color;)V` | `com/sun/java/swing/SwingUtilities3` |
| `paintMenuItem(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljavax/swing/Icon;Ljavax/swing/Icon;Ljava/awt/Color;Ljava/awt/Color;I)V` | `sun/swing/MenuItemLayoutHelper` |
| `paintText(Ljava/awt/Graphics;Ljavax/swing/JMenuItem;Ljava/awt/Rectangle;Ljava/lang/String;)V` | `sun/swing/SwingUtilities2` |
| `paintText(Ljava/awt/Graphics;Lsun/swing/MenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper`, `sun/swing/MenuItemLayoutHelper$LayoutResult` |
| `uninstallUI(Ljavax/swing/JComponent;)V` | `sun/swing/MenuItemLayoutHelper` |

### `javax/swing/plaf/basic/BasicMenuItemUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicMenuItemUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/MenuItemLayoutHelper`, `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicMenuUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `updateMnemonicBinding()V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicMenuUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;Ljavax/swing/JMenu;Z)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicOptionPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addButtonComponents(Ljava/awt/Container;[Ljava/lang/Object;I)V` | `sun/swing/DefaultLookup` |
| `configureButton(Ljavax/swing/JButton;)V` | `sun/swing/DefaultLookup` |
| `configureMessageLabel(Ljavax/swing/JLabel;)V` | `sun/swing/DefaultLookup` |
| `createButtonArea()Ljava/awt/Container;` | `sun/swing/DefaultLookup` |
| `createMessageArea()Ljava/awt/Container;` | `sun/swing/DefaultLookup` |
| `getButtons()[Ljava/lang/Object;` | `sun/swing/DefaultLookup` |
| `getIconForType(I)Ljavax/swing/Icon;` | `sun/swing/DefaultLookup` |
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicOptionPaneUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicPopupMenuUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `installListeners()V` | `sun/awt/AppContext` |

### `javax/swing/plaf/basic/BasicPopupMenuUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicPopupMenuUI$MenuKeyboardHelper`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `uninstall()V` | `sun/awt/AppContext` |

### `javax/swing/plaf/basic/BasicPopupMenuUI$MouseGrabber`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `grabWindow([Ljavax/swing/MenuElement;)V` | `sun/awt/SunToolkit` |
| `realUngrabWindow()V` | `sun/awt/SunToolkit` |
| `uninstall()V` | `sun/awt/AppContext` |

### `javax/swing/plaf/basic/BasicProgressBarUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPreferredInnerHorizontal()Ljava/awt/Dimension;` | `sun/swing/DefaultLookup` |
| `getPreferredInnerVertical()Ljava/awt/Dimension;` | `sun/swing/DefaultLookup` |
| `getPreferredSize(Ljavax/swing/JComponent;)Ljava/awt/Dimension;` | `sun/swing/SwingUtilities2` |
| `getStringPlacement(Ljava/awt/Graphics;Ljava/lang/String;IIII)Ljava/awt/Point;` | `sun/swing/SwingUtilities2` |
| `initCycleTime()I` | `sun/swing/DefaultLookup` |
| `initRepaintInterval()I` | `sun/swing/DefaultLookup` |
| `paintString(Ljava/awt/Graphics;IIIIIILjava/awt/Insets;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicRadioButtonUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `paint(Ljava/awt/Graphics;Ljavax/swing/JComponent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicRootPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(ILjavax/swing/JComponent;)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `updateDefaultButtonBindings(Ljavax/swing/JRootPane;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicRootPaneUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicScrollBarUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `layoutHScrollbar(Ljavax/swing/JScrollBar;)V` | `sun/swing/DefaultLookup` |
| `layoutVScrollbar(Ljavax/swing/JScrollBar;)V` | `sun/swing/DefaultLookup` |
| `paintThumb(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljava/awt/Rectangle;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicScrollBarUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicScrollBarUI$TrackListener`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `adjustValueIfNecessary(I)I` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicScrollPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicScrollPaneUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicSliderUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(ILjavax/swing/JSlider;)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `getMinimumHorizontalSize()Ljava/awt/Dimension;` | `sun/swing/DefaultLookup` |
| `getMinimumVerticalSize()Ljava/awt/Dimension;` | `sun/swing/DefaultLookup` |
| `getPreferredHorizontalSize()Ljava/awt/Dimension;` | `sun/swing/DefaultLookup` |
| `getPreferredVerticalSize()Ljava/awt/Dimension;` | `sun/swing/DefaultLookup` |
| `paintTicks(Ljava/awt/Graphics;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicSliderUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/UIAction` |
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicSliderUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicSpinnerUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `installListeners()V` | `sun/swing/DefaultLookup` |
| `updateEnabledState(Ljava/awt/Container;Z)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicSpinnerUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `stateChanged(Ljavax/swing/event/ChangeEvent;)V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicSplitPaneDivider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/swing/plaf/basic/BasicSplitPaneUI;)V` | `sun/swing/DefaultLookup` |
| `oneTouchExpandableChanged()V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicSplitPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `startDragging()V` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor` |

### `javax/swing/plaf/basic/BasicSplitPaneUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |
| `toggleFocus(Ljavax/swing/JSplitPane;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTabbedPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addMnemonic(II)V` | `sun/swing/SwingUtilities2` |
| `calculateTabWidth(IILjava/awt/FontMetrics;)I` | `sun/swing/SwingUtilities2` |
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `getTabLabelShiftX(IIZ)I` | `sun/swing/DefaultLookup` |
| `getTabLabelShiftY(IIZ)I` | `sun/swing/DefaultLookup` |
| `navigateSelectedTab(I)V` | `sun/swing/DefaultLookup` |
| `navigateTo(I)V` | `sun/swing/DefaultLookup` |
| `paintTab(Ljava/awt/Graphics;I[Ljava/awt/Rectangle;ILjava/awt/Rectangle;Ljava/awt/Rectangle;)V` | `sun/swing/SwingUtilities2` |
| `paintText(Ljava/awt/Graphics;ILjava/awt/Font;Ljava/awt/FontMetrics;ILjava/lang/String;Ljava/awt/Rectangle;Z)V` | `sun/swing/SwingUtilities2` |
| `requestFocusForVisibleComponent()Z` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTabbedPaneUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicTabbedPaneUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTableHeaderUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `installKeyboardActions()V` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicTableHeaderUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicTableHeaderUI$MouseInputHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mouseDragged(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTableUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `paintGrid(Ljava/awt/Graphics;IIII)V` | `sun/swing/SwingUtilities2` |
| `pointOutsidePrefSize(IILjava/awt/Point;)Z` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTableUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |
| `<init>(Ljava/lang/String;IIZZ)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicTableUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `actionPerformed(Ljava/awt/event/ActionEvent;)V` | `sun/swing/SwingUtilities2` |
| `mouseDragged(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mousePressed(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mousePressedDND(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mouseReleased(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mouseReleasedDND(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `setDispatchComponent(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTextUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createKeymap()Ljavax/swing/text/Keymap;` | `sun/swing/DefaultLookup` |
| `getDragListener()Ljavax/swing/plaf/basic/BasicTextUI$DragListener;` | `sun/awt/AppContext` |
| `getInputMap()Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `installDefaults2()V` | `sun/swing/DefaultLookup` |
| `updateBackground(Ljavax/swing/text/JTextComponent;)V` | `sun/swing/DefaultLookup` |
| `updateFocusAcceleratorBinding(Z)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicToggleButtonUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |

### `javax/swing/plaf/basic/BasicToolBarUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/basic/BasicToolBarUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicToolTipUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPreferredSize(Ljavax/swing/JComponent;)Ljava/awt/Dimension;` | `sun/swing/SwingUtilities2` |
| `paint(Ljava/awt/Graphics;Ljavax/swing/JComponent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicToolTipUI$PropertyChangeHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTransferable`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createInputStream(Ljava/awt/datatransfer/DataFlavor;Ljava/lang/String;)Ljava/io/InputStream;` | `sun/datatransfer/DataFlavorUtil` |

### `javax/swing/plaf/basic/BasicTreeUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `ensureRowsAreVisible(II)V` | `sun/swing/DefaultLookup` |
| `getInputMap(I)Ljavax/swing/InputMap;` | `sun/swing/DefaultLookup` |
| `startEditing(Ljavax/swing/tree/TreePath;Ljava/awt/event/MouseEvent;)Z` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$ComponentAccessor`, `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTreeUI$Actions`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/UIAction` |
| `<init>(Ljava/lang/String;)V` | `sun/swing/UIAction` |

### `javax/swing/plaf/basic/BasicTreeUI$Handler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mouseDragged(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mousePressed(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mousePressedDND(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mouseReleased(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mouseReleasedDND(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeNodesChanged(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeNodesInserted(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeNodesRemoved(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeStructureChanged(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/BasicTreeUI$MouseInputHandler`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/swing/plaf/basic/BasicTreeUI;Ljava/awt/Component;Ljava/awt/Component;Ljava/awt/event/MouseEvent;Ljava/awt/Component;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/basic/DefaultMenuLayout`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `preferredLayoutSize(Ljava/awt/Container;)Ljava/awt/Dimension;` | `sun/swing/MenuItemLayoutHelper` |

### `javax/swing/plaf/basic/DragRecognitionSupport`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDragRecognitionSupport()Ljavax/swing/plaf/basic/DragRecognitionSupport;` | `sun/awt/AppContext` |
| `mapDragOperationFromModifiers(Ljava/awt/event/MouseEvent;Ljavax/swing/TransferHandler;)I` | `sun/awt/dnd/SunDragSourceContextPeer` |

### `javax/swing/plaf/metal/DefaultMetalTheme`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultFontStyle(I)I` | `sun/awt/AppContext`, `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalBorders$AbstractMetalWindowBorder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintBorder(Ljava/awt/Component;Ljava/awt/Graphics;IIII)V` | `com/sun/java/swing/SwingUtilities3` |
| `paintUnscaledBorder(Ljava/awt/Component;Ljava/awt/Graphics;IID)V` | `sun/java2d/pipe/Region` |

### `javax/swing/plaf/metal/MetalBorders$MenuBarBorder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintBorder(Ljava/awt/Component;Ljava/awt/Graphics;IIII)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalBumps`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createBuffer(Ljava/awt/GraphicsConfiguration;Ljava/awt/Color;Ljava/awt/Color;Ljava/awt/Color;)Ljavax/swing/plaf/metal/BumpBuffer;` | `sun/awt/AppContext` |

### `javax/swing/plaf/metal/MetalButtonUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `paintText(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljava/awt/Rectangle;Ljava/lang/String;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalCheckBoxUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |

### `javax/swing/plaf/metal/MetalFileChooserUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createActionMap()Ljavax/swing/ActionMap;` | `sun/swing/FilePane` |
| `createDetailsView(Ljavax/swing/JFileChooser;)Ljavax/swing/JPanel;` | `sun/swing/FilePane` |
| `createList(Ljavax/swing/JFileChooser;)Ljavax/swing/JPanel;` | `sun/swing/FilePane` |
| `ensureFileIsVisible(Ljavax/swing/JFileChooser;Ljava/io/File;)V` | `sun/swing/FilePane` |
| `getMnemonic(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/Integer;` | `sun/swing/SwingUtilities2` |
| `installComponents(Ljavax/swing/JFileChooser;)V` | `sun/swing/FilePane` |
| `rescanCurrentDirectory(Ljavax/swing/JFileChooser;)V` | `sun/swing/FilePane` |
| `uninstallUI(Ljavax/swing/JComponent;)V` | `sun/swing/FilePane` |

### `javax/swing/plaf/metal/MetalFileChooserUI$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/FilePane` |

### `javax/swing/plaf/metal/MetalFileChooserUI$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `focusGained(Ljava/awt/event/FocusEvent;)V` | `sun/swing/FilePane` |

### `javax/swing/plaf/metal/MetalFileChooserUI$DirectoryComboBoxModel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `addItem(Ljava/io/File;)V` | `sun/awt/shell/ShellFolder`, `sun/swing/FilePane` |

### `javax/swing/plaf/metal/MetalFontDesktopProperty`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;I)V` | `sun/swing/plaf/DesktopProperty` |
| `configureValue(Ljava/lang/Object;)Ljava/lang/Object;` | `sun/swing/plaf/DesktopProperty` |

### `javax/swing/plaf/metal/MetalIconFactory$OceanHorizontalSliderThumbIcon`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/CachedPainter` |

### `javax/swing/plaf/metal/MetalIconFactory$OceanVerticalSliderThumbIcon`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/CachedPainter` |

### `javax/swing/plaf/metal/MetalIconFactory$RadioButtonIcon`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintIcon(Ljava/awt/Component;Ljava/awt/Graphics;II)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalIconFactory$RadioButtonMenuItemIcon`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintIcon(Ljava/awt/Component;Ljava/awt/Graphics;II)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalInternalFrameTitlePane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintComponent(Ljava/awt/Graphics;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalInternalFrameTitlePane$MetalTitlePaneLayout`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `minimumLayoutSize(Ljava/awt/Container;)Ljava/awt/Dimension;` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalLabelUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `paintDisabledText(Ljavax/swing/JLabel;Ljava/awt/Graphics;Ljava/lang/String;II)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalLookAndFeel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCurrentTheme()Ljavax/swing/plaf/metal/MetalTheme;` | `sun/awt/AppContext`, `sun/security/action/GetPropertyAction` |
| `initComponentDefaults(Ljavax/swing/UIDefaults;)V` | `sun/swing/SwingUtilities2` |
| `initResourceBundle(Ljavax/swing/UIDefaults;)V` | `sun/swing/SwingAccessor`, `sun/swing/SwingAccessor$UIDefaultsAccessor` |
| `isWindows()Z` | `sun/awt/OSInfo`, `sun/awt/OSInfo$OSType`, `sun/security/action/GetPropertyAction` |
| `setCurrentTheme(Ljavax/swing/plaf/metal/MetalTheme;)V` | `sun/awt/AppContext` |

### `javax/swing/plaf/metal/MetalLookAndFeel$AATextListener`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalLookAndFeel$MetalLayoutStyle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/DefaultLayoutStyle` |
| `getButtonGap(Ljavax/swing/JComponent;Ljavax/swing/JComponent;II)I` | `sun/swing/DefaultLayoutStyle` |
| `getContainerGap(Ljavax/swing/JComponent;ILjava/awt/Container;)I` | `sun/swing/DefaultLayoutStyle` |
| `getPreferredGap(Ljavax/swing/JComponent;Ljavax/swing/JComponent;Ljavax/swing/LayoutStyle$ComponentPlacement;ILjava/awt/Container;)I` | `sun/swing/DefaultLayoutStyle` |

### `javax/swing/plaf/metal/MetalRadioButtonUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `paint(Ljava/awt/Graphics;Ljavax/swing/JComponent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalScrollBarUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `oceanPaintThumb(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljava/awt/Rectangle;)V` | `sun/swing/SwingUtilities2` |
| `paintThumb(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljava/awt/Rectangle;)V` | `sun/swing/SwingUtilities2` |
| `paintTrack(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljava/awt/Rectangle;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalTitlePane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintComponent(Ljava/awt/Graphics;)V` | `sun/swing/SwingUtilities2` |
| `updateSystemIcon()V` | `sun/awt/SunToolkit` |

### `javax/swing/plaf/metal/MetalToggleButtonUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `paintText(Ljava/awt/Graphics;Ljavax/swing/JComponent;Ljava/awt/Rectangle;Ljava/lang/String;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalToolTipUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `calcAccelSpacing(Ljavax/swing/JComponent;Ljava/awt/FontMetrics;Ljava/lang/String;)I` | `sun/swing/SwingUtilities2` |
| `paint(Ljava/awt/Graphics;Ljavax/swing/JComponent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/metal/MetalUtils`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getOceanDisabledButtonIcon(Ljava/awt/Image;)Ljavax/swing/Icon;` | `sun/swing/ImageIconUIResource` |
| `getOceanToolBarIcon(Ljava/awt/Image;)Ljavax/swing/Icon;` | `sun/swing/ImageIconUIResource` |

### `javax/swing/plaf/metal/MetalUtils$GradientPainter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(I)V` | `sun/swing/CachedPainter` |

### `javax/swing/plaf/metal/OceanTheme`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getIconResource(Ljava/lang/String;)Ljava/lang/Object;` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/nimbus/AbstractRegionPainter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getComponentColor(Ljavax/swing/JComponent;Ljava/lang/String;Ljava/awt/Color;FFI)Ljava/awt/Color;` | `sun/reflect/misc/MethodUtil` |

### `javax/swing/plaf/nimbus/Effect`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getArrayCache()Ljavax/swing/plaf/nimbus/Effect$ArrayCache;` | `sun/awt/AppContext` |

### `javax/swing/plaf/nimbus/NimbusDefaults`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/font/FontUtilities`, `sun/swing/plaf/synth/DefaultSynthStyle` |

### `javax/swing/plaf/nimbus/NimbusLookAndFeel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaults()Ljavax/swing/UIDefaults;` | `sun/awt/OSInfo`, `sun/awt/OSInfo$OSType`, `sun/swing/plaf/GTKKeybindings`, `sun/swing/plaf/WindowsKeybindings` |
| `getDisabledIcon(Ljavax/swing/JComponent;Ljavax/swing/Icon;)Ljavax/swing/Icon;` | `sun/swing/ImageIconUIResource` |

### `javax/swing/plaf/synth/DefaultSynthStyleFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/BakedArrayList` |
| `addStyle(Lsun/swing/plaf/synth/DefaultSynthStyle;Ljava/lang/String;I)V` | `sun/swing/plaf/synth/StyleAssociation` |
| `cacheStyle(Ljava/util/List;Ljavax/swing/plaf/synth/SynthStyle;)V` | `sun/swing/BakedArrayList` |
| `getDefaultStyle()Ljavax/swing/plaf/synth/SynthStyle;` | `sun/swing/plaf/synth/DefaultSynthStyle` |
| `getMatchingStyles(Ljava/util/List;Ljavax/swing/JComponent;Ljavax/swing/plaf/synth/Region;)V` | `sun/swing/plaf/synth/StyleAssociation` |
| `getStyle(Ljavax/swing/JComponent;Ljavax/swing/plaf/synth/Region;)Ljavax/swing/plaf/synth/SynthStyle;` | `sun/swing/BakedArrayList` |
| `mergeStyles(Ljava/util/List;)Ljavax/swing/plaf/synth/SynthStyle;` | `sun/swing/plaf/synth/DefaultSynthStyle` |

### `javax/swing/plaf/synth/ImagePainter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPaint9Painter()Lsun/swing/plaf/synth/Paint9Painter;` | `sun/awt/AppContext`, `sun/swing/plaf/synth/Paint9Painter` |
| `paint(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;IIII)V` | `sun/swing/plaf/synth/Paint9Painter`, `sun/swing/plaf/synth/Paint9Painter$PaintType` |

### `javax/swing/plaf/synth/ParsedSynthStyle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/plaf/synth/DefaultSynthStyle` |
| `<init>(Lsun/swing/plaf/synth/DefaultSynthStyle;)V` | `sun/swing/plaf/synth/DefaultSynthStyle` |
| `addTo(Lsun/swing/plaf/synth/DefaultSynthStyle;)Lsun/swing/plaf/synth/DefaultSynthStyle;` | `sun/swing/plaf/synth/DefaultSynthStyle` |
| `toString()Ljava/lang/String;` | `sun/swing/plaf/synth/DefaultSynthStyle` |

### `javax/swing/plaf/synth/ParsedSynthStyle$StateInfo`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/plaf/synth/DefaultSynthStyle$StateInfo` |
| `<init>(Lsun/swing/plaf/synth/DefaultSynthStyle$StateInfo;)V` | `sun/swing/plaf/synth/DefaultSynthStyle$StateInfo` |
| `addTo(Lsun/swing/plaf/synth/DefaultSynthStyle$StateInfo;)Lsun/swing/plaf/synth/DefaultSynthStyle$StateInfo;` | `sun/swing/plaf/synth/DefaultSynthStyle$StateInfo` |
| `toString()Ljava/lang/String;` | `sun/swing/plaf/synth/DefaultSynthStyle$StateInfo` |

### `javax/swing/plaf/synth/Region`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getLowerCaseNameMap()Ljava/util/Map;` | `sun/awt/AppContext` |
| `getUItoRegionMap()Ljava/util/Map;` | `sun/awt/AppContext` |

### `javax/swing/plaf/synth/SynthDefaultLookup`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>()V` | `sun/swing/DefaultLookup` |
| `getDefault(Ljavax/swing/JComponent;Ljavax/swing/plaf/ComponentUI;Ljava/lang/String;)Ljava/lang/Object;` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/synth/SynthGraphicsUtils`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `computeStringWidth(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Font;Ljava/awt/FontMetrics;Ljava/lang/String;)I` | `sun/swing/SwingUtilities2` |
| `getPreferredMenuItemSize(Ljavax/swing/plaf/synth/SynthContext;Ljavax/swing/plaf/synth/SynthContext;Ljavax/swing/JComponent;Ljavax/swing/Icon;Ljavax/swing/Icon;ILjava/lang/String;ZLjava/lang/String;)Ljava/awt/Dimension;` | `sun/swing/MenuItemLayoutHelper`, `sun/swing/MenuItemLayoutHelper$RectSize` |
| `paint(Ljavax/swing/plaf/synth/SynthContext;Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;Ljavax/swing/Icon;Ljavax/swing/Icon;Ljava/lang/String;ILjava/lang/String;)V` | `sun/swing/MenuItemLayoutHelper` |
| `paintAccText(Ljava/awt/Graphics;Ljavax/swing/plaf/synth/SynthMenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper$LayoutResult` |
| `paintArrowIcon(Ljava/awt/Graphics;Ljavax/swing/plaf/synth/SynthMenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper$LayoutResult` |
| `paintCheckIcon(Ljava/awt/Graphics;Ljavax/swing/plaf/synth/SynthMenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper$LayoutResult` |
| `paintIcon(Ljava/awt/Graphics;Ljavax/swing/plaf/synth/SynthMenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper$LayoutResult` |
| `paintText(Ljava/awt/Graphics;Ljavax/swing/plaf/synth/SynthMenuItemLayoutHelper;Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper$LayoutResult` |
| `paintText(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;Ljava/lang/String;III)V` | `sun/swing/SwingUtilities2` |
| `paintText(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;Ljava/lang/String;Ljavax/swing/Icon;IIIIIII)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthInternalFrameTitlePane`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paint(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthLookAndFeel`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createUI(Ljavax/swing/JComponent;)Ljavax/swing/plaf/ComponentUI;` | `sun/swing/plaf/synth/SynthFileChooserUI` |
| `getDefaults()Ljavax/swing/UIDefaults;` | `sun/swing/SwingAccessor`, `sun/swing/SwingAccessor$UIDefaultsAccessor`, `sun/swing/SwingUtilities2` |
| `getSelectedUI()Ljavax/swing/plaf/ComponentUI;` | `sun/awt/AppContext` |
| `getSelectedUIState()I` | `sun/awt/AppContext` |
| `getStyleFactory()Ljavax/swing/plaf/synth/SynthStyleFactory;` | `sun/awt/AppContext` |
| `initialize()V` | `sun/swing/DefaultLookup` |
| `resetSelectedUI()V` | `sun/awt/AppContext` |
| `setSelectedUI(Ljavax/swing/plaf/ComponentUI;ZZZZ)V` | `sun/awt/AppContext` |
| `setStyleFactory(Ljavax/swing/plaf/synth/SynthStyleFactory;)V` | `sun/awt/AppContext` |
| `useLAFConditions()Z` | `sun/awt/SunToolkit`, `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthLookAndFeel$AATextListener`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthMenuItemLayoutHelper`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/swing/plaf/synth/SynthContext;Ljavax/swing/plaf/synth/SynthContext;Ljavax/swing/JMenuItem;Ljavax/swing/Icon;Ljavax/swing/Icon;Ljava/awt/Rectangle;ILjava/lang/String;ZZLjava/lang/String;)V` | `sun/swing/MenuItemLayoutHelper` |
| `calcMaxWidths()V` | `sun/swing/MenuItemLayoutHelper$RectSize` |
| `calcWidthsAndHeights()V` | `sun/swing/MenuItemLayoutHelper`, `sun/swing/MenuItemLayoutHelper$RectSize` |
| `layoutIconAndTextInLabelRect(Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper$LayoutResult` |
| `prepareForLayout(Lsun/swing/MenuItemLayoutHelper$LayoutResult;)V` | `sun/swing/MenuItemLayoutHelper$LayoutResult`, `sun/swing/MenuItemLayoutHelper$RectSize` |

### `javax/swing/plaf/synth/SynthMenuItemUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPreferredMenuItemSize(Ljavax/swing/JComponent;Ljavax/swing/Icon;Ljavax/swing/Icon;I)Ljava/awt/Dimension;` | `sun/swing/MenuItemLayoutHelper` |
| `uninstallUI(Ljavax/swing/JComponent;)V` | `sun/swing/MenuItemLayoutHelper` |

### `javax/swing/plaf/synth/SynthMenuUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPreferredMenuItemSize(Ljavax/swing/JComponent;Ljavax/swing/Icon;Ljavax/swing/Icon;I)Ljava/awt/Dimension;` | `sun/swing/MenuItemLayoutHelper` |
| `uninstallUI(Ljavax/swing/JComponent;)V` | `sun/swing/MenuItemLayoutHelper` |
| `updateStyle(Ljavax/swing/JMenuItem;)V` | `sun/swing/MenuItemLayoutHelper` |

### `javax/swing/plaf/synth/SynthOptionPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getSizeButtonsToSameWidth()Z` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/synth/SynthParser`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `characters([CII)V` | `com/sun/beans/decoder/DocumentHandler` |
| `endDocument()V` | `com/sun/beans/decoder/DocumentHandler` |
| `endElement(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `error(Lorg/xml/sax/SAXParseException;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `fatalError(Lorg/xml/sax/SAXParseException;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `getHandler()Lcom/sun/beans/decoder/DocumentHandler;` | `com/sun/beans/decoder/DocumentHandler` |
| `ignorableWhitespace([CII)V` | `com/sun/beans/decoder/DocumentHandler` |
| `lookup(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;` | `com/sun/beans/decoder/DocumentHandler` |
| `notationDecl(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `processingInstruction(Ljava/lang/String;Ljava/lang/String;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `register(Ljava/lang/String;Ljava/lang/Object;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `resolveEntity(Ljava/lang/String;Ljava/lang/String;)Lorg/xml/sax/InputSource;` | `com/sun/beans/decoder/DocumentHandler` |
| `setDocumentLocator(Lorg/xml/sax/Locator;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `startColor(Lorg/xml/sax/Attributes;)V` | `sun/reflect/misc/ReflectUtil` |
| `startDocument()V` | `com/sun/beans/decoder/DocumentHandler` |
| `startElement(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lorg/xml/sax/Attributes;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `unparsedEntityDecl(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V` | `com/sun/beans/decoder/DocumentHandler` |
| `warning(Lorg/xml/sax/SAXParseException;)V` | `com/sun/beans/decoder/DocumentHandler` |

### `javax/swing/plaf/synth/SynthProgressBarUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPreferredSize(Ljavax/swing/JComponent;)Ljava/awt/Dimension;` | `sun/swing/SwingUtilities2` |
| `paintText(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;Ljava/lang/String;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthSliderUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paint(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthSplitPaneDivider`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `lookupOneTouchSize()I` | `sun/swing/DefaultLookup` |

### `javax/swing/plaf/synth/SynthStyle`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `populateDefaultValues()V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthTabbedPaneUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paintTab(Ljavax/swing/plaf/synth/SynthContext;Ljava/awt/Graphics;I[Ljava/awt/Rectangle;ILjava/awt/Rectangle;Ljava/awt/Rectangle;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/plaf/synth/SynthTableHeaderUI$HeaderRenderer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljavax/swing/plaf/synth/SynthTableHeaderUI;)V` | `sun/swing/table/DefaultTableCellHeaderRenderer` |
| `getTableCellRendererComponent(Ljavax/swing/JTable;Ljava/lang/Object;ZZII)Ljava/awt/Component;` | `sun/swing/table/DefaultTableCellHeaderRenderer` |
| `setBorder(Ljavax/swing/border/Border;)V` | `sun/swing/table/DefaultTableCellHeaderRenderer` |

### `javax/swing/plaf/synth/SynthToolTipUI`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `propertyChange(Ljava/beans/PropertyChangeEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/table/DefaultTableCellRenderer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `firePropertyChange(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V` | `sun/swing/SwingUtilities2` |
| `getNoFocusBorder()Ljavax/swing/border/Border;` | `sun/swing/DefaultLookup` |
| `getTableCellRendererComponent(Ljavax/swing/JTable;Ljava/lang/Object;ZZII)Ljava/awt/Component;` | `sun/swing/DefaultLookup` |

### `javax/swing/table/JTableHeader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createDefaultRenderer()Ljavax/swing/table/TableCellRenderer;` | `sun/swing/table/DefaultTableCellHeaderRenderer` |
| `getToolTipText(Ljava/awt/event/MouseEvent;)Ljava/lang/String;` | `sun/awt/AWTAccessor`, `sun/awt/AWTAccessor$MouseEventAccessor` |

### `javax/swing/text/AbstractDocument`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `calculateBidiLevels(II)[B` | `sun/font/BidiUtils` |
| `handleInsertString(ILjava/lang/String;Ljavax/swing/text/AttributeSet;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/DefaultCaret`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `mouseClicked(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `mousePressed(Ljava/awt/event/MouseEvent;)V` | `sun/swing/SwingUtilities2` |
| `updateSystemSelection()V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/DefaultEditorKit$DefaultKeyTypedAction`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `actionPerformed(Ljava/awt/event/ActionEvent;)V` | `sun/awt/SunToolkit` |

### `javax/swing/text/DefaultFormatter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `stringToValue(Ljava/lang/String;)Ljava/lang/Object;` | `sun/reflect/misc/ReflectUtil`, `sun/swing/SwingUtilities2` |

### `javax/swing/text/FieldView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getPreferredSpan(I)F` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/GlyphView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `paint(Ljava/awt/Graphics;Ljava/awt/Shape;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/JTextComponent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFocusedComponent()Ljavax/swing/text/JTextComponent;` | `sun/awt/AppContext` |
| `getKeymapTable()Ljava/util/HashMap;` | `sun/awt/AppContext` |
| `getPrintable(Ljava/text/MessageFormat;Ljava/text/MessageFormat;)Ljava/awt/print/Printable;` | `sun/swing/text/TextComponentPrintable` |
| `print(Ljava/text/MessageFormat;Ljava/text/MessageFormat;ZLjavax/print/PrintService;Ljavax/print/attribute/PrintRequestAttributeSet;Z)Z` | `sun/swing/PrintingStatus` |
| `removeNotify()V` | `sun/awt/AppContext` |
| `shouldSynthensizeKeyEvents()Z` | `com/sun/beans/util/Cache` |

### `javax/swing/text/JTextComponent$2`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `call()Ljava/lang/Object;` | `sun/swing/PrintingStatus` |

### `javax/swing/text/JTextComponent$3`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `run()V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/JTextComponent$4`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Lcom/sun/beans/util/Cache$Kind;Lcom/sun/beans/util/Cache$Kind;)V` | `com/sun/beans/util/Cache` |

### `javax/swing/text/JTextComponent$MutableCaretEvent`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `focusGained(Ljava/awt/event/FocusEvent;)V` | `sun/awt/AppContext` |

### `javax/swing/text/LayoutQueue`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultQueue()Ljavax/swing/text/LayoutQueue;` | `sun/awt/AppContext` |
| `setDefaultQueue(Ljavax/swing/text/LayoutQueue;)V` | `sun/awt/AppContext` |

### `javax/swing/text/NumberFormatter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `toggleSign(Z)Ljava/lang/Object;` | `sun/reflect/misc/ReflectUtil`, `sun/swing/SwingUtilities2` |

### `javax/swing/text/PasswordView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `drawEchoCharacterImpl(Ljava/awt/Graphics;FFCZ)F` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/StyleContext`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getFont(Ljava/lang/String;II)Ljava/awt/Font;` | `sun/font/FontUtilities` |

### `javax/swing/text/TextLayoutStrategy`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `layoutRow(Ljavax/swing/text/FlowView;II)I` | `sun/font/BidiUtils` |
| `sync(Ljavax/swing/text/FlowView;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/Utilities`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `drawComposedText(Ljavax/swing/text/View;Ljavax/swing/text/AttributeSet;Ljava/awt/Graphics;FFIIZ)F` | `sun/swing/SwingUtilities2` |
| `drawTabbedText(Ljavax/swing/text/View;Ljavax/swing/text/Segment;FFLjava/awt/Graphics;Ljavax/swing/text/TabExpander;I[IZ)F` | `sun/swing/SwingUtilities2` |
| `getTabbedTextOffset(Ljavax/swing/text/View;Ljavax/swing/text/Segment;Ljava/awt/FontMetrics;FFLjavax/swing/text/TabExpander;IZ[IZ)I` | `sun/swing/SwingUtilities2` |
| `getTabbedTextWidth(Ljavax/swing/text/View;Ljavax/swing/text/Segment;Ljava/awt/FontMetrics;FLjavax/swing/text/TabExpander;I[IZ)F` | `sun/swing/SwingUtilities2` |
| `paintComposedText(Ljava/awt/Graphics;Ljava/awt/Rectangle;Ljavax/swing/text/GlyphView;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/html/HTMLDocument$HTMLReader`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `handleText([CI)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/html/HTMLEditorKit`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getStyleSheet()Ljavax/swing/text/html/StyleSheet;` | `sun/awt/AppContext` |
| `setStyleSheet(Ljavax/swing/text/html/StyleSheet;)V` | `sun/awt/AppContext` |

### `javax/swing/text/html/HTMLEditorKit$HTMLFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `create(Ljavax/swing/text/Element;)Ljavax/swing/text/View;` | `sun/swing/SwingAccessor` |

### `javax/swing/text/html/ObjectView`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `createComponent()Ljava/awt/Component;` | `sun/reflect/misc/ReflectUtil` |
| `setParameters(Ljava/awt/Component;Ljavax/swing/text/AttributeSet;)V` | `sun/reflect/misc/MethodUtil` |

### `javax/swing/text/html/StyleSheet$ListPainter`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `drawLetter(Ljava/awt/Graphics;CIIIIFI)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/text/html/parser/DTD`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDtdHash()Ljava/util/Hashtable;` | `sun/awt/AppContext` |

### `javax/swing/text/html/parser/Element`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `<init>(Ljava/lang/String;I)V` | `sun/awt/AppContext` |
| `getMaxIndex()I` | `sun/awt/AppContext` |

### `javax/swing/text/html/parser/ParserDelegator`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getDefaultDTD()Ljavax/swing/text/html/parser/DTD;` | `sun/awt/AppContext` |

### `javax/swing/tree/DefaultTreeCellRenderer`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `firePropertyChange(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V` | `sun/swing/SwingUtilities2` |
| `getDefaultClosedIcon()Ljavax/swing/Icon;` | `sun/swing/DefaultLookup` |
| `getDefaultLeafIcon()Ljavax/swing/Icon;` | `sun/swing/DefaultLookup` |
| `getDefaultOpenIcon()Ljavax/swing/Icon;` | `sun/swing/DefaultLookup` |
| `getTreeCellRendererComponent(Ljavax/swing/JTree;Ljava/lang/Object;ZZZIZ)Ljava/awt/Component;` | `sun/swing/DefaultLookup` |
| `paint(Ljava/awt/Graphics;)V` | `sun/swing/DefaultLookup` |
| `updateUI()V` | `sun/swing/DefaultLookup` |

### `javax/swing/tree/FixedHeightLayoutCache`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `treeNodesChanged(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeNodesInserted(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeNodesRemoved(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeStructureChanged(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/tree/VariableHeightLayoutCache`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `treeNodesChanged(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeNodesInserted(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeNodesRemoved(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |
| `treeStructureChanged(Ljavax/swing/event/TreeModelEvent;)V` | `sun/swing/SwingUtilities2` |

### `javax/swing/undo/UndoManager`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `tryUndoOrRedo(Ljavax/swing/undo/UndoManager$Action;)V` | `sun/swing/text/UndoableEditLockSupport` |

### `javax/xml/catalog/CatalogFeatures`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getSystemProperty(Ljavax/xml/catalog/CatalogFeatures$Feature;Ljava/lang/String;)Z` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/catalog/CatalogImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getParser()Ljavax/xml/parsers/SAXParser;` | `com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl` |

### `javax/xml/catalog/CatalogMessages`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `formatMessage(Ljava/lang/String;[Ljava/lang/Object;Ljava/util/Locale;)Ljava/lang/String;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/catalog/CatalogResolverImpl`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `setEntityResolver(Ljavax/xml/transform/sax/SAXSource;)V` | `com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl` |

### `javax/xml/catalog/Util`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `getCatalogFiles(Ljava/lang/String;)[Ljava/lang/String;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/datatype/DatatypeFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultInstance()Ljavax/xml/datatype/DatatypeFactory;` | `com/sun/org/apache/xerces/internal/jaxp/datatype/DatatypeFactoryImpl` |

### `javax/xml/datatype/FactoryFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `find(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Object;` | `jdk/xml/internal/SecuritySupport` |
| `getProviderClass(Ljava/lang/String;Ljava/lang/ClassLoader;ZZ)Ljava/lang/Class;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/parsers/DocumentBuilderFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultInstance()Ljavax/xml/parsers/DocumentBuilderFactory;` | `com/sun/org/apache/xerces/internal/jaxp/DocumentBuilderFactoryImpl` |
| `newDefaultNSInstance()Ljavax/xml/parsers/DocumentBuilderFactory;` | `com/sun/org/apache/xerces/internal/jaxp/DocumentBuilderFactoryImpl` |

### `javax/xml/parsers/FactoryFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `find(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Object;` | `jdk/xml/internal/SecuritySupport` |
| `getProviderClass(Ljava/lang/String;Ljava/lang/ClassLoader;ZZ)Ljava/lang/Class;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/parsers/SAXParserFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultInstance()Ljavax/xml/parsers/SAXParserFactory;` | `com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl` |
| `newDefaultNSInstance()Ljavax/xml/parsers/SAXParserFactory;` | `com/sun/org/apache/xerces/internal/jaxp/SAXParserFactoryImpl` |

### `javax/xml/stream/FactoryFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `find(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/ClassLoader;Ljava/lang/String;)Ljava/lang/Object;` | `jdk/xml/internal/SecuritySupport` |
| `getProviderClass(Ljava/lang/String;Ljava/lang/ClassLoader;ZZ)Ljava/lang/Class;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/stream/XMLEventFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultFactory()Ljavax/xml/stream/XMLEventFactory;` | `com/sun/xml/internal/stream/events/XMLEventFactoryImpl` |

### `javax/xml/stream/XMLInputFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultFactory()Ljavax/xml/stream/XMLInputFactory;` | `com/sun/xml/internal/stream/XMLInputFactoryImpl` |

### `javax/xml/stream/XMLOutputFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultFactory()Ljavax/xml/stream/XMLOutputFactory;` | `com/sun/xml/internal/stream/XMLOutputFactoryImpl` |

### `javax/xml/transform/FactoryFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `find(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Object;` | `jdk/xml/internal/SecuritySupport` |
| `getProviderClass(Ljava/lang/String;Ljava/lang/ClassLoader;ZZ)Ljava/lang/Class;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/transform/TransformerFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultInstance()Ljavax/xml/transform/TransformerFactory;` | `com/sun/org/apache/xalan/internal/xsltc/trax/TransformerFactoryImpl` |

### `javax/xml/validation/SchemaFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultInstance()Ljavax/xml/validation/SchemaFactory;` | `com/sun/org/apache/xerces/internal/jaxp/validation/XMLSchemaFactory` |
| `newInstance(Ljava/lang/String;)Ljavax/xml/validation/SchemaFactory;` | `jdk/xml/internal/SecuritySupport` |
| `newInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/ClassLoader;)Ljavax/xml/validation/SchemaFactory;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/validation/SchemaFactoryFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `_newFactory(Ljava/lang/String;)Ljavax/xml/validation/SchemaFactory;` | `com/sun/org/apache/xerces/internal/jaxp/validation/XMLSchemaFactory`, `jdk/xml/internal/SecuritySupport` |
| `debugDisplayClassLoader()V` | `jdk/xml/internal/SecuritySupport` |
| `which(Ljava/lang/Class;)Ljava/lang/String;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/xpath/XPathFactory`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `newDefaultInstance()Ljavax/xml/xpath/XPathFactory;` | `com/sun/org/apache/xpath/internal/jaxp/XPathFactoryImpl` |
| `newInstance(Ljava/lang/String;)Ljavax/xml/xpath/XPathFactory;` | `jdk/xml/internal/SecuritySupport` |
| `newInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/ClassLoader;)Ljavax/xml/xpath/XPathFactory;` | `jdk/xml/internal/SecuritySupport` |

### `javax/xml/xpath/XPathFactoryFinder`

| 方法签名 | 调用的内部类 |
|---------|------------|
| `_newFactory(Ljava/lang/String;)Ljavax/xml/xpath/XPathFactory;` | `com/sun/org/apache/xpath/internal/jaxp/XPathFactoryImpl`, `jdk/xml/internal/SecuritySupport` |
| `debugDisplayClassLoader()V` | `jdk/xml/internal/SecuritySupport` |
| `which(Ljava/lang/Class;)Ljava/lang/String;` | `jdk/xml/internal/SecuritySupport` |

## native 方法列表（按公开 API 类）

### `java/awt/AWTEvent`

- `initIDs()V`
- `nativeSetSource(Ljava/awt/peer/ComponentPeer;)V`

### `java/awt/Button`

- `initIDs()V`

### `java/awt/Checkbox`

- `initIDs()V`

### `java/awt/CheckboxMenuItem`

- `initIDs()V`

### `java/awt/Choice`

- `initIDs()V`

### `java/awt/Color`

- `initIDs()V`

### `java/awt/Component`

- `initIDs()V`

### `java/awt/Container`

- `initIDs()V`

### `java/awt/Cursor`

- `finalizeImpl(J)V`
- `initIDs()V`

### `java/awt/Dialog`

- `initIDs()V`

### `java/awt/Dimension`

- `initIDs()V`

### `java/awt/Event`

- `initIDs()V`

### `java/awt/FileDialog`

- `initIDs()V`

### `java/awt/Font`

- `initIDs()V`

### `java/awt/FontMetrics`

- `initIDs()V`

### `java/awt/Frame`

- `initIDs()V`

### `java/awt/Insets`

- `initIDs()V`

### `java/awt/KeyboardFocusManager`

- `initIDs()V`

### `java/awt/Label`

- `initIDs()V`

### `java/awt/Menu`

- `initIDs()V`

### `java/awt/MenuBar`

- `initIDs()V`

### `java/awt/MenuComponent`

- `initIDs()V`

### `java/awt/MenuItem`

- `initIDs()V`

### `java/awt/Rectangle`

- `initIDs()V`

### `java/awt/ScrollPane`

- `initIDs()V`

### `java/awt/ScrollPaneAdjustable`

- `initIDs()V`

### `java/awt/Scrollbar`

- `initIDs()V`

### `java/awt/SplashScreen`

- `_close(J)V`
- `_getBounds(J)Ljava/awt/Rectangle;`
- `_getImageFileName(J)Ljava/lang/String;`
- `_getImageJarName(J)Ljava/lang/String;`
- `_getInstance()J`
- `_getScaleFactor(J)F`
- `_isVisible(J)Z`
- `_setImageData(J[B)Z`
- `_update(J[IIIIII)V`

### `java/awt/TextArea`

- `initIDs()V`

### `java/awt/TextField`

- `initIDs()V`

### `java/awt/Toolkit`

- `initIDs()V`

### `java/awt/TrayIcon`

- `initIDs()V`

### `java/awt/Window`

- `initIDs()V`

### `java/awt/event/InputEvent`

- `initIDs()V`

### `java/awt/event/KeyEvent`

- `initIDs()V`

### `java/awt/event/MouseEvent`

- `initIDs()V`

### `java/awt/image/BufferedImage`

- `initIDs()V`

### `java/awt/image/ColorModel`

- `initIDs()V`

### `java/awt/image/IndexColorModel`

- `initIDs()V`

### `java/awt/image/Kernel`

- `initIDs()V`

### `java/awt/image/Raster`

- `initIDs()V`

### `java/awt/image/SampleModel`

- `initIDs()V`

### `java/awt/image/SinglePixelPackedSampleModel`

- `initIDs()V`

### `java/io/Console`

- `encoding()Ljava/lang/String;`
- `ttyStatus()I`

### `java/io/FileCleanable`

- `cleanupClose0(IJ)V`

### `java/io/FileDescriptor`

- `close0()V`
- `getAppend(I)Z`
- `getHandle(I)J`
- `initIDs()V`
- `sync0()V`

### `java/io/FileInputStream`

- `available0()I`
- `initIDs()V`
- `length0()J`
- `open0(Ljava/lang/String;)V`
- `position0()J`
- `read0()I`
- `readBytes([BII)I`
- `skip0(J)J`

### `java/io/FileOutputStream`

- `initIDs()V`
- `open0(Ljava/lang/String;Z)V`
- `write(IZ)V`
- `writeBytes([BIIZ)V`

### `java/io/ObjectStreamClass`

- `hasStaticInitializer(Ljava/lang/Class;)Z`
- `initNative()V`

### `java/io/RandomAccessFile`

- `getFilePointer()J`
- `initIDs()V`
- `length0()J`
- `open0(Ljava/lang/String;I)V`
- `read0()I`
- `readBytes0([BII)I`
- `seek0(J)V`
- `setLength0(J)V`
- `write0(I)V`
- `writeBytes0([BII)V`

### `java/io/UnixFileSystem`

- `canonicalize0(Ljava/lang/String;)Ljava/lang/String;`
- `checkAccess0(Ljava/io/File;I)Z`
- `createDirectory0(Ljava/io/File;)Z`
- `createFileExclusively0(Ljava/lang/String;)Z`
- `delete0(Ljava/io/File;)Z`
- `getBooleanAttributes0(Ljava/io/File;)I`
- `getLastModifiedTime0(Ljava/io/File;)J`
- `getLength0(Ljava/io/File;)J`
- `getNameMax0(Ljava/lang/String;)J`
- `getSpace0(Ljava/io/File;I)J`
- `initIDs()V`
- `list0(Ljava/io/File;)[Ljava/lang/String;`
- `rename0(Ljava/io/File;Ljava/io/File;)Z`
- `setLastModifiedTime0(Ljava/io/File;J)Z`
- `setPermission0(Ljava/io/File;IZZ)Z`
- `setReadOnly0(Ljava/io/File;)Z`

### `java/lang/Class`

- `desiredAssertionStatus0(Ljava/lang/Class;)Z`
- `forName0(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;`
- `getClassAccessFlagsRaw0()I`
- `getClassFileVersion0()I`
- `getConstantPool()Ljdk/internal/reflect/ConstantPool;`
- `getDeclaredClasses0()[Ljava/lang/Class;`
- `getDeclaredConstructors0(Z)[Ljava/lang/reflect/Constructor;`
- `getDeclaredFields0(Z)[Ljava/lang/reflect/Field;`
- `getDeclaredMethods0(Z)[Ljava/lang/reflect/Method;`
- `getDeclaringClass0()Ljava/lang/Class;`
- `getEnclosingMethod0()[Ljava/lang/Object;`
- `getGenericSignature0()Ljava/lang/String;`
- `getInterfaces0()[Ljava/lang/Class;`
- `getModifiers()I`
- `getNestHost0()Ljava/lang/Class;`
- `getNestMembers0()[Ljava/lang/Class;`
- `getPermittedSubclasses0()[Ljava/lang/Class;`
- `getPrimitiveClass(Ljava/lang/String;)Ljava/lang/Class;`
- `getProtectionDomain0()Ljava/security/ProtectionDomain;`
- `getRawAnnotations()[B`
- `getRawTypeAnnotations()[B`
- `getRecordComponents0()[Ljava/lang/reflect/RecordComponent;`
- `getSigners()[Ljava/lang/Object;`
- `getSimpleBinaryName0()Ljava/lang/String;`
- `getSuperclass()Ljava/lang/Class;`
- `initClassName()Ljava/lang/String;`
- `isArray()Z`
- `isAssignableFrom(Ljava/lang/Class;)Z`
- `isHidden()Z`
- `isInstance(Ljava/lang/Object;)Z`
- `isInterface()Z`
- `isPrimitive()Z`
- `isRecord0()Z`
- `registerNatives()V`
- `setSigners([Ljava/lang/Object;)V`

### `java/lang/ClassLoader`

- `defineClass0(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;`
- `defineClass1(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;`
- `defineClass2(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;`
- `findBootstrapClass(Ljava/lang/String;)Ljava/lang/Class;`
- `findLoadedClass0(Ljava/lang/String;)Ljava/lang/Class;`
- `registerNatives()V`
- `retrieveDirectives()Ljava/lang/AssertionStatusDirectives;`

### `java/lang/Double`

- `doubleToRawLongBits(D)J`
- `longBitsToDouble(J)D`

### `java/lang/Float`

- `floatToRawIntBits(F)I`
- `intBitsToFloat(I)F`

### `java/lang/Module`

- `addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V`
- `addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V`
- `addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V`
- `addReads0(Ljava/lang/Module;Ljava/lang/Module;)V`
- `defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V`

### `java/lang/NullPointerException`

- `getExtendedNPEMessage()Ljava/lang/String;`

### `java/lang/Object`

- `clone()Ljava/lang/Object;`
- `getClass()Ljava/lang/Class;`
- `hashCode()I`
- `notify()V`
- `notifyAll()V`
- `wait0(J)V`

### `java/lang/ProcessEnvironment`

- `environ()[[B`

### `java/lang/ProcessHandleImpl`

- `destroy0(JJZ)Z`
- `getCurrentPid0()J`
- `getProcessPids0(J[J[J[J)I`
- `initNative()V`
- `isAlive0(J)J`
- `parent0(JJ)J`
- `waitForProcessExit0(JZ)I`

### `java/lang/ProcessHandleImpl$Info`

- `info0(J)V`
- `initIDs()V`

### `java/lang/ProcessImpl`

- `forkAndExec(I[B[B[BI[BI[B[IZ)I`
- `init()V`

### `java/lang/Runtime`

- `availableProcessors()I`
- `freeMemory()J`
- `gc()V`
- `maxMemory()J`
- `totalMemory()J`

### `java/lang/SecurityManager`

- `getClassContext()[Ljava/lang/Class;`

### `java/lang/Shutdown`

- `beforeHalt()V`
- `halt0(I)V`

### `java/lang/StackStreamFactory`

- `checkStackWalkModes()Z`

### `java/lang/StackStreamFactory$AbstractStackWalker`

- `callStackWalk(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;`
- `fetchStackFrames(JJII[Ljava/lang/Object;)I`
- `setContinuation(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V`

### `java/lang/StackTraceElement`

- `initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V`
- `initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V`

### `java/lang/String`

- `intern()Ljava/lang/String;`

### `java/lang/StringUTF16`

- `isBigEndian()Z`

### `java/lang/System`

- `arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V`
- `currentTimeMillis()J`
- `identityHashCode(Ljava/lang/Object;)I`
- `mapLibraryName(Ljava/lang/String;)Ljava/lang/String;`
- `nanoTime()J`
- `registerNatives()V`
- `setErr0(Ljava/io/PrintStream;)V`
- `setIn0(Ljava/io/InputStream;)V`
- `setOut0(Ljava/io/PrintStream;)V`

### `java/lang/Thread`

- `clearInterruptEvent()V`
- `currentCarrierThread()Ljava/lang/Thread;`
- `currentThread()Ljava/lang/Thread;`
- `dumpThreads([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;`
- `ensureMaterializedForStackWalk(Ljava/lang/Object;)V`
- `findScopedValueBindings()Ljava/lang/Object;`
- `getNextThreadIdOffset()J`
- `getStackTrace0()Ljava/lang/Object;`
- `getThreads()[Ljava/lang/Thread;`
- `holdsLock(Ljava/lang/Object;)Z`
- `interrupt0()V`
- `registerNatives()V`
- `scopedValueCache()[Ljava/lang/Object;`
- `setCurrentThread(Ljava/lang/Thread;)V`
- `setNativeName(Ljava/lang/String;)V`
- `setPriority0(I)V`
- `setScopedValueCache([Ljava/lang/Object;)V`
- `sleep0(J)V`
- `start0()V`
- `yield0()V`

### `java/lang/Throwable`

- `fillInStackTrace(I)Ljava/lang/Throwable;`

### `java/lang/VirtualThread`

- `notifyJvmtiEnd()V`
- `notifyJvmtiHideFrames(Z)V`
- `notifyJvmtiMount(Z)V`
- `notifyJvmtiStart()V`
- `notifyJvmtiUnmount(Z)V`
- `registerNatives()V`

### `java/lang/invoke/LambdaProxyClassArchive`

- `addToArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V`
- `findFromArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;`

### `java/lang/invoke/MethodHandle`

- `invoke([Ljava/lang/Object;)Ljava/lang/Object;`
- `invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;`
- `invokeExact([Ljava/lang/Object;)Ljava/lang/Object;`
- `linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;`
- `linkToNative([Ljava/lang/Object;)Ljava/lang/Object;`
- `linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;`
- `linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;`
- `linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;`

### `java/lang/invoke/MethodHandleNatives`

- `copyOutBootstrapArguments(Ljava/lang/Class;[III[Ljava/lang/Object;IZLjava/lang/Object;)V`
- `expand(Ljava/lang/invoke/MemberName;)V`
- `getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;`
- `getNamedCon(I[Ljava/lang/Object;)I`
- `init(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V`
- `objectFieldOffset(Ljava/lang/invoke/MemberName;)J`
- `registerNatives()V`
- `resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;`
- `setCallSiteTargetNormal(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V`
- `setCallSiteTargetVolatile(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V`
- `staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;`
- `staticFieldOffset(Ljava/lang/invoke/MemberName;)J`

### `java/lang/invoke/VarHandle`

- `compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;`
- `compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;`
- `compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;`
- `compareAndSet([Ljava/lang/Object;)Z`
- `get([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAcquire([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndSet([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;`
- `getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;`
- `getOpaque([Ljava/lang/Object;)Ljava/lang/Object;`
- `getVolatile([Ljava/lang/Object;)Ljava/lang/Object;`
- `set([Ljava/lang/Object;)V`
- `setOpaque([Ljava/lang/Object;)V`
- `setRelease([Ljava/lang/Object;)V`
- `setVolatile([Ljava/lang/Object;)V`
- `weakCompareAndSet([Ljava/lang/Object;)Z`
- `weakCompareAndSetAcquire([Ljava/lang/Object;)Z`
- `weakCompareAndSetPlain([Ljava/lang/Object;)Z`
- `weakCompareAndSetRelease([Ljava/lang/Object;)Z`

### `java/lang/ref/Finalizer`

- `isFinalizationEnabled()Z`
- `reportComplete(Ljava/lang/Object;)V`

### `java/lang/ref/PhantomReference`

- `refersTo0(Ljava/lang/Object;)Z`

### `java/lang/ref/Reference`

- `clear0()V`
- `getAndClearReferencePendingList()Ljava/lang/ref/Reference;`
- `hasReferencePendingList()Z`
- `refersTo0(Ljava/lang/Object;)Z`
- `waitForReferencePendingList()V`

### `java/lang/reflect/Array`

- `get(Ljava/lang/Object;I)Ljava/lang/Object;`
- `getBoolean(Ljava/lang/Object;I)Z`
- `getByte(Ljava/lang/Object;I)B`
- `getChar(Ljava/lang/Object;I)C`
- `getDouble(Ljava/lang/Object;I)D`
- `getFloat(Ljava/lang/Object;I)F`
- `getInt(Ljava/lang/Object;I)I`
- `getLength(Ljava/lang/Object;)I`
- `getLong(Ljava/lang/Object;I)J`
- `getShort(Ljava/lang/Object;I)S`
- `multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;`
- `newArray(Ljava/lang/Class;I)Ljava/lang/Object;`
- `set(Ljava/lang/Object;ILjava/lang/Object;)V`
- `setBoolean(Ljava/lang/Object;IZ)V`
- `setByte(Ljava/lang/Object;IB)V`
- `setChar(Ljava/lang/Object;IC)V`
- `setDouble(Ljava/lang/Object;ID)V`
- `setFloat(Ljava/lang/Object;IF)V`
- `setInt(Ljava/lang/Object;II)V`
- `setLong(Ljava/lang/Object;IJ)V`
- `setShort(Ljava/lang/Object;IS)V`

### `java/lang/reflect/Executable`

- `getParameters0()[Ljava/lang/reflect/Parameter;`
- `getTypeAnnotationBytes0()[B`

### `java/lang/reflect/Field`

- `getTypeAnnotationBytes0()[B`

### `java/net/Inet4Address`

- `init()V`

### `java/net/Inet4AddressImpl`

- `getHostByAddr([B)Ljava/lang/String;`
- `getLocalHostName()Ljava/lang/String;`
- `isReachable0([BI[BI)Z`
- `lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;`

### `java/net/Inet6Address`

- `init()V`

### `java/net/Inet6AddressImpl`

- `getHostByAddr([B)Ljava/lang/String;`
- `getLocalHostName()Ljava/lang/String;`
- `isReachable0([BII[BII)Z`
- `lookupAllHostAddr(Ljava/lang/String;I)[Ljava/net/InetAddress;`

### `java/net/InetAddress`

- `init()V`
- `isIPv4Available()Z`
- `isIPv6Supported()Z`

### `java/net/NetworkInterface`

- `boundInetAddress0(Ljava/net/InetAddress;)Z`
- `getAll()[Ljava/net/NetworkInterface;`
- `getByIndex0(I)Ljava/net/NetworkInterface;`
- `getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;`
- `getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;`
- `getMTU0(Ljava/lang/String;I)I`
- `getMacAddr0([BLjava/lang/String;I)[B`
- `init()V`
- `isLoopback0(Ljava/lang/String;I)Z`
- `isP2P0(Ljava/lang/String;I)Z`
- `isUp0(Ljava/lang/String;I)Z`
- `supportsMulticast0(Ljava/lang/String;I)Z`

### `java/nio/MappedMemoryUtils`

- `force0(Ljava/io/FileDescriptor;JJ)V`
- `isLoaded0(JJJ)Z`
- `load0(JJ)V`
- `unload0(JJ)V`

### `java/security/AccessController`

- `ensureMaterializedForStackWalk(Ljava/lang/Object;)V`
- `getInheritedAccessControlContext()Ljava/security/AccessControlContext;`
- `getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;`
- `getStackAccessControlContext()Ljava/security/AccessControlContext;`

### `java/util/TimeZone`

- `getSystemGMTOffsetID()Ljava/lang/String;`
- `getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;`

### `java/util/concurrent/atomic/AtomicLong`

- `VMSupportsCS8()Z`

### `java/util/prefs/FileSystemPreferences`

- `chmod(Ljava/lang/String;I)I`
- `lockFile0(Ljava/lang/String;IZ)[I`
- `unlockFile0(I)I`

### `java/util/prefs/MacOSXPreferencesFile`

- `addChildToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z`
- `addKeyToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V`
- `addNode(Ljava/lang/String;Ljava/lang/String;JJ)Z`
- `anyHost()J`
- `anyUser()J`
- `currentHost()J`
- `currentUser()J`
- `getChildrenForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;`
- `getKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;`
- `getKeysForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;`
- `removeChildFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V`
- `removeKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V`
- `removeNode(Ljava/lang/String;Ljava/lang/String;JJ)V`
- `synchronize(Ljava/lang/String;JJ)Z`

### `java/util/zip/Adler32`

- `update(II)I`
- `updateByteBuffer(IJII)I`
- `updateBytes(I[BII)I`

### `java/util/zip/CRC32`

- `update(II)I`
- `updateByteBuffer0(IJII)I`
- `updateBytes0(I[BII)I`

### `java/util/zip/Deflater`

- `deflateBufferBuffer(JJIJIII)J`
- `deflateBufferBytes(JJI[BIIII)J`
- `deflateBytesBuffer(J[BIIJIII)J`
- `deflateBytesBytes(J[BII[BIIII)J`
- `end(J)V`
- `getAdler(J)I`
- `init(IIZ)J`
- `reset(J)V`
- `setDictionary(J[BII)V`
- `setDictionaryBuffer(JJI)V`

### `java/util/zip/Inflater`

- `end(J)V`
- `getAdler(J)I`
- `inflateBufferBuffer(JJIJI)J`
- `inflateBufferBytes(JJI[BII)J`
- `inflateBytesBuffer(J[BIIJI)J`
- `inflateBytesBytes(J[BII[BII)J`
- `init(Z)J`
- `initIDs()V`
- `reset(J)V`
- `setDictionary(J[BII)V`
- `setDictionaryBuffer(JJI)V`

