# 调用链追踪：HelloWorld

生成时间：2026-09-14

## 摘要

| | 数量 |
|---|---:|
| 调用链涉及类（有方法调用） | 577 |
| 仅引用类（new/field，无方法调用） | 58 |
| 合计 | 635 |
| 可达方法数 | 3124 |

## 调用链方法（按类分组）

### `java/io/BufferedWriter`

- `ensureOpen()V`
- `flushBuffer()V`
- `implFlushBuffer()V`
- `newLine()V`
- `write(Ljava/lang/String;)V`

### `java/io/DataInputStream`

- `<init>(Ljava/io/InputStream;)V`
- `close()V`
- `readFully([B)V`
- `readFully([BII)V`
- `readInt()I`

### `java/io/EOFException`

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

### `java/io/UncheckedIOException`

- `<init>(Ljava/lang/String;Ljava/io/IOException;)V`

### `java/io/Writer`

- `write([CII)V`

### `java/lang/AbstractMethodError`

- `<init>(Ljava/lang/String;)V`

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
- `insert(IC)Ljava/lang/AbstractStringBuilder;`
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

- `<init>(Ljava/lang/String;)V`

### `java/lang/ArrayIndexOutOfBoundsException`

- `<init>(I)V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/AssertionError`

- `<init>()V`
- `<init>(I)V`
- `<init>(J)V`
- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/String;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/Boolean`

- `booleanValue()Z`
- `valueOf(Z)Ljava/lang/Boolean;`

### `java/lang/Byte`

- `byteValue()B`
- `intValue()I`
- `valueOf(B)Ljava/lang/Byte;`

### `java/lang/CharSequence`

- `charAt(I)C`
- `length()I`
- `subSequence(II)Ljava/lang/CharSequence;`
- `toString()Ljava/lang/String;`

### `java/lang/Character`

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
- `isExtendedPictographic(I)Z`
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

- `forName(Ljava/lang/String;)Ljava/lang/Character$UnicodeBlock;`
- `of(I)Ljava/lang/Character$UnicodeBlock;`
- `toString()Ljava/lang/String;`

### `java/lang/Character$UnicodeScript`

- `forName(Ljava/lang/String;)Ljava/lang/Character$UnicodeScript;`
- `valueOf(Ljava/lang/String;)Ljava/lang/Character$UnicodeScript;`

### `java/lang/CharacterData`

- `digit(II)I`
- `getType(I)I`
- `isExtendedPictographic(I)Z`
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

- `<init>(Ljava/lang/CharacterName;)V`

### `java/lang/Class`

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
- `isSynthetic()Z`
- `isTopLevelClass()Z`
- `isUnnamedClass()Z`
- `methodToString(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/String;`
- `newReflectionData(Ljava/lang/ref/SoftReference;I)Ljava/lang/Class$ReflectionData;`
- `privateGetDeclaredConstructors(Z)[Ljava/lang/reflect/Constructor;`
- `privateGetDeclaredMethods(Z)[Ljava/lang/reflect/Method;`
- `reflectionData()Ljava/lang/Class$ReflectionData;`
- `searchMethods([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;`

### `java/lang/Class$3`

- `<init>(Ljava/lang/Class;Ljava/lang/reflect/Method;)V`

### `java/lang/Class$Atomic`

- `casReflectionData(Ljava/lang/Class;Ljava/lang/ref/SoftReference;Ljava/lang/ref/SoftReference;)Z`

### `java/lang/Class$EnclosingMethodInfo`

- `<init>([Ljava/lang/Object;)V`
- `getEnclosingClass()Ljava/lang/Class;`
- `validate([Ljava/lang/Object;)V`

### `java/lang/Class$ReflectionData`

- `<init>(I)V`

### `java/lang/ClassCastException`

- `<init>(Ljava/lang/String;)V`

### `java/lang/ClassLoader`

- `checkClassLoaderPermission(Ljava/lang/ClassLoader;Ljava/lang/Class;)V`
- `desiredAssertionStatus(Ljava/lang/String;)Z`
- `getBuiltinAppClassLoader()Ljava/lang/ClassLoader;`
- `getBuiltinPlatformClassLoader()Ljava/lang/ClassLoader;`
- `getClassLoader(Ljava/lang/Class;)Ljava/lang/ClassLoader;`
- `getParent()Ljava/lang/ClassLoader;`
- `getPlatformClassLoader()Ljava/lang/ClassLoader;`
- `getSystemClassLoader()Ljava/lang/ClassLoader;`
- `isAncestor(Ljava/lang/ClassLoader;)Z`
- `needsClassLoaderPermissionCheck(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z`

### `java/lang/ClassValue`

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
- `removeStaleEntries()V`
- `removeStaleEntries([Ljava/lang/ClassValue$Entry;II)V`
- `sizeCache(I)V`
- `startEntry(Ljava/lang/ClassValue;)Ljava/lang/ClassValue$Entry;`

### `java/lang/ClassValue$Entry`

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

- `classValue()Ljava/lang/ClassValue;`
- `isLive()Z`
- `promise()Ljava/lang/ClassValue$Entry;`

### `java/lang/Comparable`

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

- `getCondition()I`
- `getLanguage()Ljava/lang/String;`
- `getLowerCase()[C`
- `getUpperCase()[C`

### `java/lang/Double`

- `<init>(D)V`
- `doubleToRawLongBits(D)J`
- `doubleValue()D`
- `longBitsToDouble(J)D`
- `valueOf(D)Ljava/lang/Double;`

### `java/lang/Enum`

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

- `<init>(F)V`
- `floatToRawIntBits(F)I`
- `floatValue()F`
- `intBitsToFloat(I)F`
- `isNaN(F)Z`
- `valueOf(F)Ljava/lang/Float;`

### `java/lang/IllegalAccessException`

- `<init>(Ljava/lang/String;)V`

### `java/lang/IllegalArgumentException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/IllegalCallerException`

- `<init>(Ljava/lang/String;)V`

### `java/lang/IllegalStateException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/IncompatibleClassChangeError`

- `<init>(Ljava/lang/String;)V`

### `java/lang/IndexOutOfBoundsException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/Integer`

- `<init>(I)V`
- `formatUnsignedInt(II[BI)V`
- `formatUnsignedIntUTF16(II[BI)V`
- `getChars(II[B)I`
- `intValue()I`
- `numberOfLeadingZeros(I)I`
- `numberOfTrailingZeros(I)I`
- `parseInt(Ljava/lang/CharSequence;III)I`
- `parseInt(Ljava/lang/String;)I`
- `parseInt(Ljava/lang/String;I)I`
- `stringSize(I)I`
- `toHexString(I)Ljava/lang/String;`
- `toString()Ljava/lang/String;`
- `toString(I)Ljava/lang/String;`
- `toUnsignedString0(II)Ljava/lang/String;`
- `valueOf(I)Ljava/lang/Integer;`

### `java/lang/InternalError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/LinkageError`

- `<init>(Ljava/lang/String;)V`

### `java/lang/Long`

- `<init>(J)V`
- `formatUnsignedLong0(JI[BII)V`
- `formatUnsignedLong0UTF16(JI[BII)V`
- `getChars(JI[B)I`
- `longValue()J`
- `numberOfLeadingZeros(J)I`
- `numberOfTrailingZeros(J)I`
- `stringSize(J)I`
- `toHexString(J)Ljava/lang/String;`
- `toString(J)Ljava/lang/String;`
- `toUnsignedString0(JI)Ljava/lang/String;`
- `valueOf(J)Ljava/lang/Long;`

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
- `multiplyHigh(JJ)J`

### `java/lang/Module`

- `allows(Ljava/util/Set;Ljava/lang/Module;)Z`
- `canRead(Ljava/lang/Module;)Z`
- `implIsExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z`
- `isExported(Ljava/lang/String;)Z`
- `isExported(Ljava/lang/String;Ljava/lang/Module;)Z`
- `isNamed()Z`
- `isReflectivelyExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z`
- `isStaticallyExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z`

### `java/lang/NegativeArraySizeException`

- `<init>()V`

### `java/lang/NoSuchFieldException`

- `<init>(Ljava/lang/String;)V`

### `java/lang/NoSuchMethodException`

- `<init>(Ljava/lang/String;)V`
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

- `<init>(Ljava/lang/String;)V`

### `java/lang/PublicMethods$Key`

- `matches(Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Z`

### `java/lang/PublicMethods$MethodList`

- `<init>(Ljava/lang/reflect/Method;)V`
- `filter([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;`
- `getMostSpecific()Ljava/lang/reflect/Method;`
- `merge(Ljava/lang/PublicMethods$MethodList;Ljava/lang/PublicMethods$MethodList;)Ljava/lang/PublicMethods$MethodList;`
- `merge(Ljava/lang/PublicMethods$MethodList;Ljava/lang/reflect/Method;)Ljava/lang/PublicMethods$MethodList;`

### `java/lang/Record`

- `<init>()V`

### `java/lang/ReflectiveOperationException`

- `<init>(Ljava/lang/String;)V`
- `initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;`

### `java/lang/RuntimeException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/RuntimePermission`

- `<init>(Ljava/lang/String;)V`

### `java/lang/SecurityException`

- `<init>(Ljava/lang/String;)V`

### `java/lang/SecurityManager`

- `checkAccess(Ljava/lang/Thread;)V`
- `checkPackageAccess(Ljava/lang/String;)V`
- `checkPermission(Ljava/security/Permission;)V`
- `checkPropertiesAccess()V`
- `getPackages(Ljava/lang/String;)[Ljava/lang/String;`

### `java/lang/SecurityManager$1`

- `<init>(Ljava/lang/SecurityManager;)V`

### `java/lang/Short`

- `<init>(S)V`
- `intValue()I`
- `shortValue()S`
- `valueOf(S)Ljava/lang/Short;`

### `java/lang/StackStreamFactory`

- `makeStackTraverser(Ljava/lang/StackWalker;Ljava/util/function/Function;)Ljava/lang/StackStreamFactory$StackFrameTraverser;`

### `java/lang/StackStreamFactory$AbstractStackWalker`

- `<init>(Ljava/lang/StackWalker;I)V`
- `<init>(Ljava/lang/StackWalker;II)V`
- `toStackWalkMode(Ljava/lang/StackWalker;I)I`

### `java/lang/StackStreamFactory$LiveStackInfoTraverser`

- `<init>(Ljava/lang/StackWalker;Ljava/util/function/Function;)V`

### `java/lang/StackStreamFactory$StackFrameTraverser`

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

- `<init>(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/StringBuilder;)V`
- `<init>([BB)V`
- `<init>([BIII)V`
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
- `rangeCheck([CII)Ljava/lang/Void;`
- `repeat(I)Ljava/lang/String;`
- `repeatCopyRest([BIII)V`
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
- `toCharArray()[C`
- `toLowerCase(Ljava/util/Locale;)Ljava/lang/String;`
- `toUpperCase(Ljava/util/Locale;)Ljava/lang/String;`
- `trim()Ljava/lang/String;`
- `value()[B`
- `valueOf(C)Ljava/lang/String;`
- `valueOf(I)Ljava/lang/String;`
- `valueOf(J)Ljava/lang/String;`
- `valueOf(Ljava/lang/Object;)Ljava/lang/String;`
- `valueOf([C)Ljava/lang/String;`

### `java/lang/StringBuffer`

- `append([C)Ljava/lang/StringBuffer;`

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
- `append([C)Ljava/lang/StringBuilder;`
- `appendCodePoint(I)Ljava/lang/StringBuilder;`
- `delete(II)Ljava/lang/StringBuilder;`
- `insert(IC)Ljava/lang/StringBuilder;`
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
- `getProperties()Ljava/util/Properties;`
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

### `java/lang/TypeNotPresentException`

- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/lang/UnsupportedOperationException`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`

### `java/lang/VirtualMachineError`

- `<init>()V`
- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`
- `<init>(Ljava/lang/Throwable;)V`

### `java/lang/VirtualThread`

- `continuationScope()Ljdk/internal/vm/ContinuationScope;`
- `notifyJvmtiMount(Z)V`
- `notifyJvmtiUnmount(Z)V`
- `setState(I)V`
- `state()I`
- `tryYield()V`
- `yieldContinuation()Z`

### `java/lang/WeakPairMap`

- `containsKeyPair(Ljava/lang/Object;Ljava/lang/Object;)Z`
- `expungeStaleAssociations()V`
- `get(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/lang/WeakPairMap$Pair`

- `lookup(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/WeakPairMap$Pair;`

### `java/lang/WeakPairMap$Pair$Lookup`

- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/lang/WeakPairMap$WeakRefPeer`

- `weakPair()Ljava/lang/WeakPairMap$Pair$Weak;`

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

- `extendWith(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `factory()Ljava/lang/invoke/MethodHandle;`
- `fieldCount()I`
- `getterFunction(I)Ljava/lang/invoke/LambdaForm$NamedFunction;`
- `getterFunctions()Ljava/util/List;`
- `key()Ljava/lang/Object;`

### `java/lang/invoke/BoundMethodHandle$Species_L`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/Object;)V`
- `make(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`

### `java/lang/invoke/DelegatingMethodHandle`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;)V`
- `chooseDelegatingForm(Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/LambdaForm;`
- `makeReinvokerForm(Ljava/lang/invoke/MethodHandle;ILjava/lang/Object;Ljava/lang/invoke/LambdaForm$NamedFunction;)Ljava/lang/invoke/LambdaForm;`
- `makeReinvokerForm(Ljava/lang/invoke/MethodHandle;ILjava/lang/Object;ZLjava/lang/invoke/LambdaForm$NamedFunction;Ljava/lang/invoke/LambdaForm$NamedFunction;)Ljava/lang/invoke/LambdaForm;`
- `whichKind(I)Ljava/lang/invoke/LambdaForm$Kind;`

### `java/lang/invoke/DirectMethodHandle`

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

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZI)V`

### `java/lang/invoke/DirectMethodHandle$Constructor`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/invoke/MemberName;Ljava/lang/Class;)V`

### `java/lang/invoke/DirectMethodHandle$Interface`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/Class;)V`

### `java/lang/invoke/DirectMethodHandle$Special`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/Class;)V`

### `java/lang/invoke/DirectMethodHandle$StaticAccessor`

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/MemberName;ZLjava/lang/Object;J)V`

### `java/lang/invoke/InvokerBytecodeGenerator`

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

- `<init>(Ljava/lang/Exception;)V`
- `printStackTrace(Ljava/io/PrintStream;)V`

### `java/lang/invoke/InvokerBytecodeGenerator$ClassData`

- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)V`

### `java/lang/invoke/Invokers`

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
- `formParametersMatch(Ljava/lang/invoke/LambdaForm;Ljava/lang/invoke/LambdaForm$BasicType;[I)Z`
- `getInCache(Ljava/lang/invoke/LambdaFormEditor$TransformKey;)Ljava/lang/invoke/LambdaForm;`
- `lambdaFormEditor(Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/LambdaFormEditor;`
- `makeArgumentCombinationForm(ILjava/lang/invoke/MethodType;ZZ)Ljava/lang/invoke/LambdaForm;`
- `makeRepeatedFilterForm(Ljava/lang/invoke/MethodType;[I)Ljava/lang/invoke/LambdaForm;`
- `newSpeciesData(Ljava/lang/invoke/LambdaForm$BasicType;)Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `oldSpeciesData()Ljava/lang/invoke/BoundMethodHandle$SpeciesData;`
- `putInCache(Ljava/lang/invoke/LambdaFormEditor$TransformKey;Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/LambdaForm;`
- `spreadArgumentsForm(ILjava/lang/Class;I)Ljava/lang/invoke/LambdaForm;`

### `java/lang/invoke/LambdaFormEditor$1`

- `<init>(Ljava/lang/invoke/LambdaFormEditor;)V`

### `java/lang/invoke/LambdaFormEditor$Transform`

- `<init>(J[BLjava/lang/invoke/LambdaForm;)V`
- `equals(Ljava/lang/invoke/LambdaFormEditor$Transform;)Z`
- `equals(Ljava/lang/invoke/LambdaFormEditor$TransformKey;)Z`
- `get()Ljava/lang/Object;`

### `java/lang/invoke/LambdaFormEditor$TransformKey`

- `<init>(J)V`
- `<init>([B)V`
- `bval(I)B`
- `equals(Ljava/lang/invoke/LambdaFormEditor$Transform;)Z`
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
- `packedBytes(II)J`
- `packedBytes(III)J`
- `packedBytes(IIII)J`
- `packedBytes([B)J`
- `withResult(Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/LambdaFormEditor$Transform;`

### `java/lang/invoke/MemberName`

- `<init>(Ljava/lang/Class;)V`
- `<init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;B)V`
- `<init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;B)V`
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

- `<init>(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)V`
- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)V`

### `java/lang/invoke/MethodHandleImpl$BindCaller`

- `bindCaller(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `bindCallerWithInjectedInvoker(Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `prepareForInvoker(Ljava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `restoreToType(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandleImpl$BindCaller$InjectedInvokerHolder`

- `invoker()Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandleImpl$Intrinsic`

- `ordinal()I`

### `java/lang/invoke/MethodHandleImpl$IntrinsicMethodHandle`

- `<init>(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandleImpl$Intrinsic;)V`
- `<init>(Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandleImpl$Intrinsic;Ljava/lang/Object;)V`

### `java/lang/invoke/MethodHandleImpl$WrappedMember`

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
- `exactInvoker(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `identity(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `insertArgumentPrimitive(Ljava/lang/invoke/BoundMethodHandle;ILjava/lang/Class;Ljava/lang/Object;)Ljava/lang/invoke/BoundMethodHandle;`
- `insertArguments(Ljava/lang/invoke/MethodHandle;I[Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;`
- `insertArgumentsChecks(Ljava/lang/invoke/MethodHandle;II)[Ljava/lang/Class;`
- `invoker(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `lookup()Ljava/lang/invoke/MethodHandles$Lookup;`
- `makeIdentity(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`
- `publicLookup()Ljava/lang/invoke/MethodHandles$Lookup;`
- `setCachedMethodHandle([Ljava/lang/invoke/MethodHandle;ILjava/lang/invoke/MethodHandle;)Ljava/lang/invoke/MethodHandle;`
- `varHandleInvoker(Ljava/lang/invoke/VarHandle$AccessMode;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandles$Lookup`

- `<init>(Ljava/lang/Class;)V`
- `<init>(Ljava/lang/Class;Ljava/lang/Class;I)V`
- `accessFailedMessage(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)Ljava/lang/String;`
- `checkAccess(BLjava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `checkMethod(BLjava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `checkMethodName(BLjava/lang/String;)V`
- `checkSecurityManager(Ljava/lang/Class;)V`
- `checkSecurityManager(Ljava/lang/Class;Ljava/lang/invoke/MemberName;)V`
- `checkSymbolicClass(Ljava/lang/Class;)V`
- `ensureInitialized(Ljava/lang/Class;)Ljava/lang/Class;`
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
- `makeAccessException(Ljava/lang/Class;)Ljava/lang/IllegalAccessException;`
- `makeHiddenClassDefiner(Ljava/lang/String;[BLjava/util/Set;Ljdk/internal/util/ClassFileDumper;)Ljava/lang/invoke/MethodHandles$Lookup$ClassDefiner;`
- `makeHiddenClassDefiner(Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;Ljava/util/Set;ZLjdk/internal/util/ClassFileDumper;)Ljava/lang/invoke/MethodHandles$Lookup$ClassDefiner;`
- `maybeBindCaller(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/invoke/MethodHandle;`
- `previousLookupClass()Ljava/lang/Class;`
- `resolveOrFail(BLjava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MemberName;`
- `resolveOrNull(BLjava/lang/invoke/MemberName;)Ljava/lang/invoke/MemberName;`
- `restrictProtectedReceiver(Ljava/lang/invoke/MemberName;)Z`
- `restrictReceiver(Ljava/lang/invoke/MemberName;Ljava/lang/invoke/DirectMethodHandle;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;`

### `java/lang/invoke/MethodHandles$Lookup$ClassDefiner`

- `<init>(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;ILjdk/internal/util/ClassFileDumper;)V`
- `defineClass(ZLjava/lang/Object;)Ljava/lang/Class;`
- `internalName()Ljava/lang/String;`
- `isNestmate()Z`

### `java/lang/invoke/MethodHandles$Lookup$ClassFile`

- `<init>(Ljava/lang/String;I[B)V`
- `newInstanceNoCheck(Ljava/lang/String;[B)Ljava/lang/invoke/MethodHandles$Lookup$ClassFile;`

### `java/lang/invoke/MethodHandles$Lookup$ClassOption`

- `optionsToFlag(Ljava/util/Set;)I`

### `java/lang/invoke/MethodType`

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

- `<init>(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)V`
- `make(Ljava/lang/invoke/MethodType;Ljava/lang/invoke/LambdaForm;)Ljava/lang/invoke/BoundMethodHandle;`

### `java/lang/invoke/VarHandle`

- `get([BI)I`

### `java/lang/invoke/VarHandle$AccessDescriptor`

- `<init>(Ljava/lang/invoke/MethodType;II)V`

### `java/lang/invoke/VarHandle$AccessMode`

- `methodName()Ljava/lang/String;`
- `ordinal()I`
- `valueFromMethodName(Ljava/lang/String;)Ljava/lang/invoke/VarHandle$AccessMode;`

### `java/lang/invoke/VarHandle$AccessType`

- `ordinal()I`

### `java/lang/invoke/WrongMethodTypeException`

- `<init>(Ljava/lang/String;)V`

### `java/lang/module/ModuleDescriptor`

- `isAutomatic()Z`
- `isOpen()Z`
- `packages()Ljava/util/Set;`

### `java/lang/ref/Cleaner`

- `register(Ljava/lang/Object;Ljava/lang/Runnable;)Ljava/lang/ref/Cleaner$Cleanable;`

### `java/lang/ref/PhantomReference`

- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`

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

- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`
- `get()Ljava/lang/Object;`

### `java/lang/ref/WeakReference`

- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`

### `java/lang/reflect/Array`

- `newArray(Ljava/lang/Class;I)Ljava/lang/Object;`
- `newInstance(Ljava/lang/Class;I)Ljava/lang/Object;`

### `java/lang/reflect/Constructor`

- `acquireConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V`
- `getConstructorAccessor()Ljdk/internal/reflect/ConstructorAccessor;`
- `getDeclaringClass()Ljava/lang/Class;`
- `getModifiers()I`
- `getParameterCount()I`
- `getParameterTypes()[Ljava/lang/Class;`
- `newInstance([Ljava/lang/Object;)Ljava/lang/Object;`
- `newInstanceWithCaller([Ljava/lang/Object;ZLjava/lang/Class;)Ljava/lang/Object;`
- `setConstructorAccessor(Ljdk/internal/reflect/ConstructorAccessor;)V`

### `java/lang/reflect/Executable`

- `getModifiers()I`
- `getParameterCount()I`
- `isVarArgs()Z`

### `java/lang/reflect/Member`

- `getClass()Ljava/lang/Class;`
- `getName()Ljava/lang/String;`

### `java/lang/reflect/Method`

- `acquireMethodAccessor()Ljdk/internal/reflect/MethodAccessor;`
- `checkAccess(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V`
- `getDeclaringClass()Ljava/lang/Class;`
- `getMethodAccessor()Ljdk/internal/reflect/MethodAccessor;`
- `getModifiers()I`
- `getName()Ljava/lang/String;`
- `getParameterCount()I`
- `getParameterTypes()[Ljava/lang/Class;`
- `getReturnType()Ljava/lang/Class;`
- `invoke(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;`
- `isAnnotationPresent(Ljava/lang/Class;)Z`
- `isCallerSensitive()Z`
- `isDefault()Z`
- `isVarArgs()Z`
- `setMethodAccessor(Ljdk/internal/reflect/MethodAccessor;)V`
- `sharedToString(IZ[Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/String;`
- `toString()Ljava/lang/String;`

### `java/lang/reflect/Modifier`

- `isAbstract(I)Z`
- `isFinal(I)Z`
- `isNative(I)Z`
- `isPrivate(I)Z`
- `isProtected(I)Z`
- `isPublic(I)Z`
- `isStatic(I)Z`
- `isVolatile(I)Z`
- `methodModifiers()I`
- `toString(I)Ljava/lang/String;`

### `java/lang/reflect/ParameterizedType`

- `getActualTypeArguments()[Ljava/lang/reflect/Type;`
- `getRawType()Ljava/lang/reflect/Type;`

### `java/lang/reflect/Proxy`

- `isProxyClass(Ljava/lang/Class;)Z`

### `java/lang/reflect/Proxy$ProxyBuilder`

- `isProxyClass(Ljava/lang/Class;)Z`

### `java/net/URL`

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
- `asLongBuffer()Ljava/nio/LongBuffer;`
- `base()Ljava/lang/Object;`
- `clear()Ljava/nio/ByteBuffer;`
- `flip()Ljava/nio/ByteBuffer;`
- `isReadOnly()Z`
- `limit()I`
- `order(Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;`
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

- `<init>([BIILjava/lang/foreign/MemorySegment;)V`

### `java/nio/HeapCharBuffer`

- `<init>([CIILjava/lang/foreign/MemorySegment;)V`

### `java/nio/LongBuffer`

- `get(I)J`
- `put(IJ)Ljava/nio/LongBuffer;`

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

- `<init>(Ljava/lang/Exception;)V`

### `java/nio/charset/CoderResult`

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

- `<init>(I)V`

### `java/nio/charset/UnmappableCharacterException`

- `<init>(I)V`

### `java/nio/file/FileSystem`

- `getPath(Ljava/lang/String;[Ljava/lang/String;)Ljava/nio/file/Path;`

### `java/nio/file/FileSystems`

- `getDefault()Ljava/nio/file/FileSystem;`

### `java/nio/file/Path`

- `of(Ljava/lang/String;[Ljava/lang/String;)Ljava/nio/file/Path;`

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

- `<init>(Ljava/security/AccessControlContext;Lsun/security/util/Debug;Ljava/security/ProtectionDomain;)V`

### `java/security/AccessControlException`

- `<init>(Ljava/lang/String;Ljava/security/Permission;)V`

### `java/security/AccessController`

- `checkPermission(Ljava/security/Permission;)V`
- `doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;`
- `ensureMaterializedForStackWalk(Ljava/lang/Object;)V`
- `executePrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;`
- `getInheritedAccessControlContext()Ljava/security/AccessControlContext;`
- `getStackAccessControlContext()Ljava/security/AccessControlContext;`
- `isPrivileged()Z`

### `java/security/BasicPermission`

- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `init(Ljava/lang/String;)V`

### `java/security/CodeSource`

- `getLocation()Ljava/net/URL;`

### `java/security/DomainCombiner`

- `combine([Ljava/security/ProtectionDomain;[Ljava/security/ProtectionDomain;)[Ljava/security/ProtectionDomain;`

### `java/security/Permission`

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

- `<init>(Ljava/lang/String;)V`

### `java/security/Policy$3`

- `<init>(Ljava/security/Policy;)V`

### `java/security/Policy$PolicyInfo`

- `<init>(Ljava/security/Policy;Z)V`

### `java/security/PrivilegedAction`

- `run()Ljava/lang/Object;`

### `java/security/ProtectionDomain`

- `getCodeSource()Ljava/security/CodeSource;`
- `getPermissions()Ljava/security/PermissionCollection;`
- `implies(Ljava/security/Permission;)Z`
- `impliesWithAltFilePerm(Ljava/security/Permission;)Z`

### `java/security/UnresolvedPermission`

- `resolve(Ljava/security/Permission;[Ljava/security/cert/Certificate;)Ljava/security/Permission;`

### `java/security/UnresolvedPermissionCollection`

- `getUnresolvedPermissions(Ljava/security/Permission;)Ljava/util/List;`

### `java/security/cert/Certificate`

- `equals(Ljava/lang/Object;)Z`
- `getEncoded()[B`

### `java/text/BreakIterator`

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

- `<init>(Ljava/util/Locale;Ljava/text/BreakIterator;)V`
- `createBreakInstance()Ljava/text/BreakIterator;`
- `getLocale()Ljava/util/Locale;`

### `java/text/Normalizer`

- `normalize(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;)Ljava/lang/String;`

### `java/text/Normalizer$Form`

- `ordinal()I`

### `java/text/StringCharacterIterator`

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

- `<init>(Ljava/lang/String;)V`
- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/time/DayOfWeek`

- `getValue()I`
- `ordinal()I`

### `java/time/Instant`

- `<init>(JI)V`
- `create(JI)Ljava/time/Instant;`
- `from(Ljava/time/temporal/TemporalAccessor;)Ljava/time/Instant;`
- `getEpochSecond()J`
- `getNano()I`
- `now()Ljava/time/Instant;`
- `ofEpochSecond(JJ)Ljava/time/Instant;`

### `java/time/LocalDate`

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

- `<init>(Ljava/time/LocalDate;Ljava/time/LocalTime;)V`
- `getNano()I`
- `of(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;`
- `ofEpochSecond(JILjava/time/ZoneOffset;)Ljava/time/LocalDateTime;`
- `plusSeconds(J)Ljava/time/LocalDateTime;`
- `plusWithOverflow(Ljava/time/LocalDate;JJJJI)Ljava/time/LocalDateTime;`
- `toEpochSecond(Ljava/time/ZoneOffset;)J`
- `with(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;`

### `java/time/LocalTime`

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

- `getRules()Ljava/time/zone/ZoneRules;`
- `normalized()Ljava/time/ZoneId;`

### `java/time/ZoneOffset`

- `equals(Ljava/lang/Object;)Z`
- `getTotalSeconds()I`

### `java/time/chrono/ChronoLocalDate`

- `atTime(Ljava/time/LocalTime;)Ljava/time/chrono/ChronoLocalDateTime;`

### `java/time/chrono/ChronoLocalDateTimeImpl`

- `<init>(Ljava/time/chrono/ChronoLocalDate;Ljava/time/LocalTime;)V`
- `of(Ljava/time/chrono/ChronoLocalDate;Ljava/time/LocalTime;)Ljava/time/chrono/ChronoLocalDateTimeImpl;`

### `java/time/chrono/ChronoZonedDateTimeImpl`

- `<init>(Ljava/time/chrono/ChronoLocalDateTimeImpl;Ljava/time/ZoneOffset;Ljava/time/ZoneId;)V`
- `ofInstant(Ljava/time/chrono/Chronology;Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/chrono/ChronoZonedDateTimeImpl;`

### `java/time/chrono/Chronology`

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

- `<init>(Ljava/time/temporal/TemporalAccessor;Ljava/time/format/DateTimeFormatter;)V`
- `adjust(Ljava/time/temporal/TemporalAccessor;Ljava/time/format/DateTimeFormatter;)Ljava/time/temporal/TemporalAccessor;`
- `endOptional()V`
- `startOptional()V`

### `java/time/format/DateTimePrintContext$1`

- `<init>(Ljava/time/chrono/ChronoLocalDate;Ljava/time/temporal/TemporalAccessor;Ljava/time/chrono/Chronology;Ljava/time/ZoneId;)V`

### `java/time/temporal/ChronoField`

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

- `<init>(Ljava/lang/String;)V`

### `java/time/temporal/ValueRange`

- `checkValidValue(JLjava/time/temporal/TemporalField;)J`
- `genInvalidFieldMessage(Ljava/time/temporal/TemporalField;J)Ljava/lang/String;`
- `getMaximum()J`
- `getMinimum()J`
- `isIntValue()Z`
- `isValidValue(J)Z`

### `java/time/zone/ZoneOffsetTransition`

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
- `clear()V`
- `grow()[Ljava/lang/Object;`
- `grow(I)[Ljava/lang/Object;`
- `iterator()Ljava/util/Iterator;`

### `java/util/ArrayList$Itr`

- `<init>(Ljava/util/ArrayList;)V`

### `java/util/Arrays`

- `asList([Ljava/lang/Object;)Ljava/util/List;`
- `binarySearch([JJ)I`
- `binarySearch0([JIIJ)I`
- `checkLength(II)V`
- `copyOf([BI)[B`
- `copyOf([II)[I`
- `copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;`
- `copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;`
- `copyOfRange([BII)[B`
- `copyOfRange([Ljava/lang/Object;II)[Ljava/lang/Object;`
- `copyOfRange([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;`
- `copyOfRangeByte([BII)[B`
- `equals([B[B)Z`
- `equals([Ljava/lang/Object;[Ljava/lang/Object;)Z`
- `fill([BB)V`
- `fill([BIIB)V`
- `fill([II)V`
- `fill([Ljava/lang/Object;IILjava/lang/Object;)V`
- `fill([Ljava/lang/Object;Ljava/lang/Object;)V`
- `rangeCheck(III)V`
- `spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;`
- `stream([Ljava/lang/Object;)Ljava/util/stream/Stream;`
- `stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;`

### `java/util/Arrays$ArrayList`

- `<init>([Ljava/lang/Object;)V`

### `java/util/Collection`

- `getClass()Ljava/lang/Class;`
- `toArray()[Ljava/lang/Object;`

### `java/util/Collections`

- `emptyMap()Ljava/util/Map;`
- `emptySet()Ljava/util/Set;`
- `nCopies(ILjava/lang/Object;)Ljava/util/List;`
- `synchronizedMap(Ljava/util/Map;)Ljava/util/Map;`

### `java/util/Collections$CopiesList`

- `<init>(ILjava/lang/Object;)V`

### `java/util/Collections$SynchronizedMap`

- `<init>(Ljava/util/Map;)V`

### `java/util/Comparator`

- `compare(Ljava/lang/Object;Ljava/lang/Object;)I`

### `java/util/DuplicateFormatFlagsException`

- `<init>(Ljava/lang/String;)V`

### `java/util/Enumeration`

- `hasMoreElements()Z`
- `nextElement()Ljava/lang/Object;`

### `java/util/FormatFlagsConversionMismatchException`

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

- `index()I`
- `print(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V`
- `toString()Ljava/lang/String;`

### `java/util/FormatterClosedException`

- `<init>()V`

### `java/util/HashMap`

- `<init>(I)V`
- `<init>(IF)V`
- `afterNodeAccess(Ljava/util/HashMap$Node;)V`
- `afterNodeInsertion(Z)V`
- `calculateHashMapCapacity(I)I`
- `comparableClassFor(Ljava/lang/Object;)Ljava/lang/Class;`
- `compareComparables(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)I`
- `entrySet()Ljava/util/Set;`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getNode(Ljava/lang/Object;)Ljava/util/HashMap$Node;`
- `getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `hash(Ljava/lang/Object;)I`
- `keySet()Ljava/util/Set;`
- `newHashMap(I)Ljava/util/HashMap;`
- `newNode(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;`
- `newTreeNode(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `putVal(ILjava/lang/Object;Ljava/lang/Object;ZZ)Ljava/lang/Object;`
- `replacementNode(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$Node;`
- `replacementTreeNode(Ljava/util/HashMap$Node;Ljava/util/HashMap$Node;)Ljava/util/HashMap$TreeNode;`
- `resize()[Ljava/util/HashMap$Node;`
- `tableSizeFor(I)I`
- `treeifyBin([Ljava/util/HashMap$Node;I)V`

### `java/util/HashMap$EntrySet`

- `<init>(Ljava/util/HashMap;)V`

### `java/util/HashMap$KeySet`

- `<init>(Ljava/util/HashMap;)V`

### `java/util/HashMap$Node`

- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V`

### `java/util/HashMap$TreeNode`

- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V`
- `balanceInsertion(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;`
- `checkInvariants(Ljava/util/HashMap$TreeNode;)Z`
- `find(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/HashMap$TreeNode;`
- `getTreeNode(ILjava/lang/Object;)Ljava/util/HashMap$TreeNode;`
- `moveRootToFront([Ljava/util/HashMap$Node;Ljava/util/HashMap$TreeNode;)V`
- `putTreeVal(Ljava/util/HashMap;[Ljava/util/HashMap$Node;ILjava/lang/Object;Ljava/lang/Object;)Ljava/util/HashMap$TreeNode;`
- `root()Ljava/util/HashMap$TreeNode;`
- `rotateLeft(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;`
- `rotateRight(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;`
- `split(Ljava/util/HashMap;[Ljava/util/HashMap$Node;II)V`
- `tieBreakOrder(Ljava/lang/Object;Ljava/lang/Object;)I`
- `treeify([Ljava/util/HashMap$Node;)V`
- `untreeify(Ljava/util/HashMap;)Ljava/util/HashMap$Node;`

### `java/util/HashSet`

- `<init>(I)V`
- `<init>(IFZ)V`
- `iterator()Ljava/util/Iterator;`
- `newHashSet(I)Ljava/util/HashSet;`

### `java/util/HexFormat`

- `toHexDigits(Ljava/lang/Appendable;B)Ljava/lang/Appendable;`
- `toHighHexDigit(I)C`
- `toLowHexDigit(I)C`

### `java/util/IllegalFormatArgumentIndexException`

- `<init>(I)V`

### `java/util/IllegalFormatException`

- `<init>()V`

### `java/util/IllegalFormatFlagsException`

- `<init>(Ljava/lang/String;)V`

### `java/util/IllegalFormatPrecisionException`

- `<init>(I)V`

### `java/util/IllegalFormatWidthException`

- `<init>(I)V`

### `java/util/ImmutableCollections`

- `listFromArray([Ljava/lang/Object;)Ljava/util/List;`
- `listFromTrustedArray([Ljava/lang/Object;)Ljava/util/List;`

### `java/util/ImmutableCollections$AbstractImmutableCollection`

- `<init>()V`

### `java/util/ImmutableCollections$AbstractImmutableList`

- `<init>()V`

### `java/util/ImmutableCollections$AbstractImmutableMap`

- `<init>()V`

### `java/util/ImmutableCollections$List12`

- `<init>(Ljava/lang/Object;)V`
- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/ImmutableCollections$ListN`

- `<init>([Ljava/lang/Object;Z)V`

### `java/util/ImmutableCollections$Map1`

- `<init>(Ljava/lang/Object;Ljava/lang/Object;)V`

### `java/util/ImmutableCollections$MapN`

- `<init>([Ljava/lang/Object;)V`
- `probe(Ljava/lang/Object;)I`

### `java/util/Iterator`

- `hasNext()Z`
- `next()Ljava/lang/Object;`

### `java/util/LinkedHashMap`

- `<init>(IF)V`

### `java/util/LinkedHashMap$Entry`

- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V`

### `java/util/LinkedHashSet`

- `<init>()V`

### `java/util/List`

- `add(Ljava/lang/Object;)Z`
- `clear()V`
- `contains(Ljava/lang/Object;)Z`
- `equals(Ljava/lang/Object;)Z`
- `get(I)Ljava/lang/Object;`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;`
- `of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;`
- `of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;`
- `of([Ljava/lang/Object;)Ljava/util/List;`
- `remove(I)Ljava/lang/Object;`
- `size()I`
- `subList(II)Ljava/util/List;`
- `toArray([Ljava/lang/Object;)[Ljava/lang/Object;`

### `java/util/Locale`

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

- `<init>(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V`

### `java/util/Map`

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
- `remove(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/Map$Entry`

- `getKey()Ljava/lang/Object;`
- `getValue()Ljava/lang/Object;`

### `java/util/MissingFormatArgumentException`

- `<init>(Ljava/lang/String;)V`

### `java/util/MissingFormatWidthException`

- `<init>(Ljava/lang/String;)V`

### `java/util/NoSuchElementException`

- `<init>()V`

### `java/util/Objects`

- `checkFromIndexSize(III)I`
- `checkFromToIndex(III)I`
- `equals(Ljava/lang/Object;Ljava/lang/Object;)Z`
- `hashCode(Ljava/lang/Object;)I`
- `requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;`
- `requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;`
- `requireNonNullElse(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/Optional`

- `<init>(Ljava/lang/Object;)V`
- `empty()Ljava/util/Optional;`
- `ofNullable(Ljava/lang/Object;)Ljava/util/Optional;`
- `orElse(Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/Properties`

- `getProperty(Ljava/lang/String;)Ljava/lang/String;`

### `java/util/PropertyPermission`

- `<init>(Ljava/lang/String;Ljava/lang/String;)V`
- `getMask(Ljava/lang/String;)I`
- `getName()Ljava/lang/String;`
- `init(I)V`

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

- `<init>(Ljava/lang/String;Ljava/lang/Throwable;)V`

### `java/util/Set`

- `add(Ljava/lang/Object;)Z`
- `clear()V`
- `contains(Ljava/lang/Object;)Z`
- `forEach(Ljava/util/function/Consumer;)V`
- `isEmpty()Z`
- `iterator()Ljava/util/Iterator;`
- `of()Ljava/util/Set;`
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

- `<init>([Ljava/lang/Object;III)V`

### `java/util/StringJoiner`

- `<init>(Ljava/lang/CharSequence;)V`
- `<init>(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)V`
- `add(Ljava/lang/CharSequence;)Ljava/util/StringJoiner;`
- `checkAddLength(II)I`
- `toString()Ljava/lang/String;`

### `java/util/StringTokenizer`

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
- `<init>(Ljava/util/Comparator;)V`
- `addEntry(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;Z)V`
- `addEntryToEmptyMap(Ljava/lang/Object;Ljava/lang/Object;)V`
- `colorOf(Ljava/util/TreeMap$Entry;)Z`
- `compare(Ljava/lang/Object;Ljava/lang/Object;)I`
- `entrySet()Ljava/util/Set;`
- `fixAfterInsertion(Ljava/util/TreeMap$Entry;)V`
- `leftOf(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;`
- `parentOf(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
- `put(Ljava/lang/Object;Ljava/lang/Object;Z)Ljava/lang/Object;`
- `rightOf(Ljava/util/TreeMap$Entry;)Ljava/util/TreeMap$Entry;`
- `rotateLeft(Ljava/util/TreeMap$Entry;)V`
- `rotateRight(Ljava/util/TreeMap$Entry;)V`
- `setColor(Ljava/util/TreeMap$Entry;Z)V`

### `java/util/TreeMap$Entry`

- `<init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;)V`

### `java/util/TreeMap$EntrySet`

- `<init>(Ljava/util/TreeMap;)V`

### `java/util/TreeSet`

- `<init>()V`
- `<init>(Ljava/util/NavigableMap;)V`

### `java/util/UnknownFormatConversionException`

- `<init>(Ljava/lang/String;)V`

### `java/util/UnknownFormatFlagsException`

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
- `containsKey(Ljava/lang/Object;)Z`
- `fullAddCount(JZ)V`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `helpTransfer([Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)[Ljava/util/concurrent/ConcurrentHashMap$Node;`
- `initTable()[Ljava/util/concurrent/ConcurrentHashMap$Node;`
- `put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`
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

### `java/util/concurrent/ConcurrentHashMap$CounterCell`

- `<init>(J)V`

### `java/util/concurrent/ConcurrentHashMap$ForwardingNode`

- `<init>([Ljava/util/concurrent/ConcurrentHashMap$Node;)V`

### `java/util/concurrent/ConcurrentHashMap$Node`

- `<init>(ILjava/lang/Object;Ljava/lang/Object;)V`
- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/concurrent/ConcurrentHashMap$Node;)V`
- `find(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;`

### `java/util/concurrent/ConcurrentHashMap$TreeBin`

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

- `<init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)V`
- `findTreeNode(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;`

### `java/util/concurrent/ConcurrentMap`

- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/concurrent/ThreadLocalRandom`

- `advanceProbe(I)I`
- `getProbe()I`
- `localInit()V`

### `java/util/concurrent/atomic/AtomicInteger`

- `addAndGet(I)I`
- `incrementAndGet()I`

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

### `java/util/function/BiFunction`

- `apply(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`

### `java/util/function/Function`

- `apply(Ljava/lang/Object;)Ljava/lang/Object;`

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

- `clear()V`

### `java/util/regex/Matcher`

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

- `<init>(I)V`

### `java/util/regex/Pattern$Begin`

- `<init>()V`

### `java/util/regex/Pattern$Behind`

- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$BehindS`

- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$BitClass`

- `<init>()V`
- `add(II)Ljava/util/regex/Pattern$BitClass;`

### `java/util/regex/Pattern$BmpCharPredicate`

- `negate()Ljava/util/regex/Pattern$CharPredicate;`

### `java/util/regex/Pattern$BmpCharProperty`

- `<init>(Ljava/util/regex/Pattern$BmpCharPredicate;)V`

### `java/util/regex/Pattern$BmpCharPropertyGreedy`

- `<init>(Ljava/util/regex/Pattern$BmpCharProperty;I)V`

### `java/util/regex/Pattern$BnM`

- `<init>([I[I[ILjava/util/regex/Pattern$Node;)V`
- `optimize(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;`

### `java/util/regex/Pattern$BnMS`

- `<init>([I[I[ILjava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$Bound`

- `<init>(IZ)V`

### `java/util/regex/Pattern$Branch`

- `<init>(Ljava/util/regex/Pattern$Node;Ljava/util/regex/Pattern$Node;Ljava/util/regex/Pattern$Node;)V`
- `add(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$BranchConn`

- `<init>()V`

### `java/util/regex/Pattern$CIBackRef`

- `<init>(IZ)V`

### `java/util/regex/Pattern$Caret`

- `<init>()V`

### `java/util/regex/Pattern$CharPredicate`

- `and(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;`
- `negate()Ljava/util/regex/Pattern$CharPredicate;`
- `union(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;`
- `union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;`

### `java/util/regex/Pattern$CharProperty`

- `<init>(Ljava/util/regex/Pattern$CharPredicate;)V`

### `java/util/regex/Pattern$CharPropertyGreedy`

- `<init>(Ljava/util/regex/Pattern$CharProperty;I)V`

### `java/util/regex/Pattern$Curly`

- `<init>(Ljava/util/regex/Pattern$Node;IILjava/util/regex/Pattern$Qtype;)V`

### `java/util/regex/Pattern$Dollar`

- `<init>(Z)V`

### `java/util/regex/Pattern$End`

- `<init>()V`

### `java/util/regex/Pattern$GraphemeBound`

- `<init>()V`

### `java/util/regex/Pattern$GroupCurly`

- `<init>(Ljava/util/regex/Pattern$Node;IILjava/util/regex/Pattern$Qtype;IIZ)V`

### `java/util/regex/Pattern$GroupHead`

- `<init>(I)V`

### `java/util/regex/Pattern$GroupTail`

- `<init>(II)V`

### `java/util/regex/Pattern$LastMatch`

- `<init>()V`

### `java/util/regex/Pattern$LazyLoop`

- `<init>(II)V`

### `java/util/regex/Pattern$LineEnding`

- `<init>()V`

### `java/util/regex/Pattern$Loop`

- `<init>(II)V`

### `java/util/regex/Pattern$NFCCharProperty`

- `<init>(Ljava/util/regex/Pattern$CharPredicate;)V`

### `java/util/regex/Pattern$Neg`

- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$Node`

- `<init>()V`
- `match(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z`
- `study(Ljava/util/regex/Pattern$TreeInfo;)Z`

### `java/util/regex/Pattern$NotBehind`

- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$NotBehindS`

- `<init>(Ljava/util/regex/Pattern$Node;II)V`

### `java/util/regex/Pattern$Pos`

- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$Prolog`

- `<init>(Ljava/util/regex/Pattern$Loop;)V`

### `java/util/regex/Pattern$Ques`

- `<init>(Ljava/util/regex/Pattern$Node;Ljava/util/regex/Pattern$Qtype;)V`

### `java/util/regex/Pattern$Slice`

- `<init>([I)V`

### `java/util/regex/Pattern$SliceI`

- `<init>([I)V`

### `java/util/regex/Pattern$SliceIS`

- `<init>([I)V`

### `java/util/regex/Pattern$SliceNode`

- `<init>([I)V`

### `java/util/regex/Pattern$SliceS`

- `<init>([I)V`

### `java/util/regex/Pattern$SliceU`

- `<init>([I)V`

### `java/util/regex/Pattern$SliceUS`

- `<init>([I)V`

### `java/util/regex/Pattern$Start`

- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$StartS`

- `<init>(Ljava/util/regex/Pattern$Node;)V`

### `java/util/regex/Pattern$TreeInfo`

- `<init>()V`
- `reset()V`

### `java/util/regex/Pattern$UnixCaret`

- `<init>()V`

### `java/util/regex/Pattern$UnixDollar`

- `<init>(Z)V`

### `java/util/regex/Pattern$XGrapheme`

- `<init>()V`

### `java/util/regex/PatternSyntaxException`

- `<init>(Ljava/lang/String;Ljava/lang/String;I)V`

### `java/util/spi/LocaleServiceProvider`

- `getAvailableLocales()[Ljava/util/Locale;`
- `isSupportedLocale(Ljava/util/Locale;)Z`

### `java/util/stream/AbstractPipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`

### `java/util/stream/Collectors`

- `joining(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;`

### `java/util/stream/Collectors$CollectorImpl`

- `<init>(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V`

### `java/util/stream/PipelineHelper`

- `<init>()V`

### `java/util/stream/ReferencePipeline`

- `<init>(Ljava/util/Spliterator;IZ)V`

### `java/util/stream/ReferencePipeline$Head`

- `<init>(Ljava/util/Spliterator;IZ)V`

### `java/util/stream/Stream`

- `collect(Ljava/util/stream/Collector;)Ljava/lang/Object;`
- `filter(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;`
- `map(Ljava/util/function/Function;)Ljava/util/stream/Stream;`
- `of([Ljava/lang/Object;)Ljava/util/stream/Stream;`
- `toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;`

### `java/util/stream/StreamOpFlag`

- `fromCharacteristics(Ljava/util/Spliterator;)I`

### `java/util/stream/StreamSupport`

- `stream(Ljava/util/Spliterator;Z)Ljava/util/stream/Stream;`

### `java/util/zip/Inflater`

- `<init>()V`
- `<init>(Z)V`
- `init(Z)J`

### `java/util/zip/Inflater$InflaterZStreamRef`

- `<init>(Ljava/util/zip/Inflater;J)V`

### `java/util/zip/InflaterInputStream`

- `<init>(Ljava/io/InputStream;)V`
- `<init>(Ljava/io/InputStream;Ljava/util/zip/Inflater;)V`
- `<init>(Ljava/io/InputStream;Ljava/util/zip/Inflater;I)V`

### `jdk/internal/access/JavaIOFilePermissionAccess`

- `newPermUsingAltPath(Ljava/io/FilePermission;)Ljava/io/FilePermission;`

### `jdk/internal/access/JavaLangAccess`

- `currentCarrierThread()Ljava/lang/Thread;`
- `defineClass(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BLjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;`
- `getContinuation(Ljava/lang/Thread;)Ljdk/internal/vm/Continuation;`
- `getUTF16Char([BI)C`
- `join(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;`
- `parkVirtualThread()V`
- `protectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;`

### `jdk/internal/access/JavaLangInvokeAccess`

- `findStatic(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `findVirtual(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;`
- `unreflectConstructor(Ljava/lang/reflect/Constructor;)Ljava/lang/invoke/MethodHandle;`

### `jdk/internal/access/JavaLangReflectAccess`

- `copyConstructor(Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;`
- `copyMethod(Ljava/lang/reflect/Method;)Ljava/lang/reflect/Method;`
- `getExecutableSharedParameterTypes(Ljava/lang/reflect/Executable;)[Ljava/lang/Class;`
- `getRoot(Ljava/lang/reflect/AccessibleObject;)Ljava/lang/reflect/AccessibleObject;`

### `jdk/internal/access/JavaSecurityAccess`

- `getProtectionDomainCache()Ljdk/internal/access/JavaSecurityAccess$ProtectionDomainCache;`

### `jdk/internal/access/SharedSecrets`

- `ensureClassInitialized(Ljava/lang/Class;)V`
- `getJavaIOFilePermissionAccess()Ljdk/internal/access/JavaIOFilePermissionAccess;`
- `getJavaLangAccess()Ljdk/internal/access/JavaLangAccess;`
- `getJavaSecurityAccess()Ljdk/internal/access/JavaSecurityAccess;`

### `jdk/internal/foreign/MemorySessionImpl`

- `checkValidStateRaw()V`

### `jdk/internal/icu/impl/Norm2AllModes`

- `getInstanceFromSingleton(Ljdk/internal/icu/impl/Norm2AllModes$Norm2AllModesSingleton;)Ljdk/internal/icu/impl/Norm2AllModes;`
- `getNFCInstance()Ljdk/internal/icu/impl/Norm2AllModes;`

### `jdk/internal/icu/lang/UCharacter`

- `getCombiningClass(I)I`

### `jdk/internal/icu/text/Normalizer2`

- `getCombiningClass(I)I`
- `getNFDInstance()Ljdk/internal/icu/text/Normalizer2;`
- `normalize(Ljava/lang/CharSequence;)Ljava/lang/String;`
- `normalize(Ljava/lang/CharSequence;Ljava/lang/StringBuilder;)Ljava/lang/StringBuilder;`
- `normalizeSecondAndAppend(Ljava/lang/StringBuilder;Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;`
- `spanQuickCheckYes(Ljava/lang/CharSequence;)I`

### `jdk/internal/icu/text/NormalizerBase`

- `normalize(Ljava/lang/String;Ljava/text/Normalizer$Form;)Ljava/lang/String;`
- `normalize(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;I)Ljava/lang/String;`
- `toMode(Ljava/text/Normalizer$Form;)Ljdk/internal/icu/text/NormalizerBase$Mode;`

### `jdk/internal/icu/text/NormalizerBase$Mode`

- `getNormalizer2(I)Ljdk/internal/icu/text/Normalizer2;`

### `jdk/internal/loader/AbstractClassLoaderValue$Sub`

- `get(Ljava/lang/ClassLoader;)Ljava/lang/Object;`

### `jdk/internal/loader/ClassLoaderValue`

- `sub(Ljava/lang/Object;)Ljdk/internal/loader/AbstractClassLoaderValue$Sub;`

### `jdk/internal/loader/ClassLoaders`

- `appClassLoader()Ljava/lang/ClassLoader;`
- `platformClassLoader()Ljava/lang/ClassLoader;`

### `jdk/internal/math/FloatToDecimal`

- `<init>()V`
- `append(I)V`
- `append8Digits(I)V`
- `appendDecimalTo(FLjava/lang/Appendable;)Ljava/lang/Appendable;`
- `appendDigit(I)V`
- `appendTo(FLjava/lang/Appendable;)Ljava/lang/Appendable;`
- `exponent(I)V`
- `removeTrailingZeroes()V`
- `rop(JJ)I`
- `toChars(II)I`
- `toChars1(III)I`
- `toChars2(III)I`
- `toChars3(III)I`
- `toDecimal(F)I`
- `toDecimal(III)I`
- `y(I)I`

### `jdk/internal/math/MathUtils`

- `flog10pow2(I)I`
- `flog10threeQuartersPow2(I)I`
- `flog2pow10(I)I`
- `g1(I)J`
- `pow10(I)J`

### `jdk/internal/misc/CDS`

- `isDumpingClassList()Z`
- `logLambdaFormInvoker(Ljava/lang/String;)V`
- `traceLambdaFormInvoker(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`

### `jdk/internal/misc/InternalLock`

- `lock()V`
- `unlock()V`

### `jdk/internal/misc/PreviewFeatures`

- `isEnabled()Z`

### `jdk/internal/misc/ScopedMemoryAccess`

- `copyMemory(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJ)V`
- `copyMemoryInternal(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/foreign/MemorySessionImpl;Ljava/lang/Object;JLjava/lang/Object;JJ)V`

### `jdk/internal/misc/ScopedMemoryAccess$ScopedAccessError`

- `newRuntimeException()Ljava/lang/RuntimeException;`

### `jdk/internal/misc/Unsafe`

- `allocateUninitializedArray(Ljava/lang/Class;I)Ljava/lang/Object;`
- `allocateUninitializedArray0(Ljava/lang/Class;I)Ljava/lang/Object;`
- `checkNativeAddress(J)V`
- `checkOffset(Ljava/lang/Object;J)V`
- `checkPointer(Ljava/lang/Object;J)V`
- `checkPrimitiveArray(Ljava/lang/Class;)V`
- `checkPrimitivePointer(Ljava/lang/Object;J)V`
- `checkSize(J)V`
- `compareAndSetInt(Ljava/lang/Object;JII)Z`
- `compareAndSetLong(Ljava/lang/Object;JJJ)Z`
- `compareAndSetReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z`
- `copyMemory(Ljava/lang/Object;JLjava/lang/Object;JJ)V`
- `copyMemory0(Ljava/lang/Object;JLjava/lang/Object;JJ)V`
- `copyMemoryChecks(Ljava/lang/Object;JLjava/lang/Object;JJ)V`
- `ensureClassInitialized(Ljava/lang/Class;)V`
- `ensureClassInitialized0(Ljava/lang/Class;)V`
- `fullFence()V`
- `getAndAddInt(Ljava/lang/Object;JI)I`
- `getAndAddLong(Ljava/lang/Object;JJ)J`
- `getByte(Ljava/lang/Object;J)B`
- `getInt(Ljava/lang/Object;J)I`
- `getIntUnaligned(Ljava/lang/Object;J)I`
- `getIntVolatile(Ljava/lang/Object;J)I`
- `getLong(Ljava/lang/Object;J)J`
- `getLongUnaligned(Ljava/lang/Object;J)J`
- `getLongVolatile(Ljava/lang/Object;J)J`
- `getReferenceAcquire(Ljava/lang/Object;J)Ljava/lang/Object;`
- `getReferenceVolatile(Ljava/lang/Object;J)Ljava/lang/Object;`
- `getShort(Ljava/lang/Object;J)S`
- `getUnsafe()Ljdk/internal/misc/Unsafe;`
- `invalidInput()Ljava/lang/RuntimeException;`
- `is32BitClean(J)Z`
- `makeInt(BBBB)I`
- `makeInt(SS)I`
- `makeLong(BBBBBBBB)J`
- `makeLong(II)J`
- `makeLong(SSSS)J`
- `park(ZJ)V`
- `pickPos(II)I`
- `putInt(Ljava/lang/Object;JI)V`
- `putLong(Ljava/lang/Object;JJ)V`
- `putLongVolatile(Ljava/lang/Object;JJ)V`
- `putReferenceOpaque(Ljava/lang/Object;JLjava/lang/Object;)V`
- `putReferenceRelease(Ljava/lang/Object;JLjava/lang/Object;)V`
- `putReferenceVolatile(Ljava/lang/Object;JLjava/lang/Object;)V`
- `shouldBeInitialized(Ljava/lang/Class;)Z`
- `shouldBeInitialized0(Ljava/lang/Class;)Z`
- `storeFence()V`
- `toUnsignedInt(B)I`
- `toUnsignedInt(S)I`
- `toUnsignedLong(B)J`
- `toUnsignedLong(I)J`
- `toUnsignedLong(S)J`
- `weakCompareAndSetInt(Ljava/lang/Object;JII)Z`
- `weakCompareAndSetLong(Ljava/lang/Object;JJJ)Z`

### `jdk/internal/misc/VM`

- `addFinalRefCount(I)V`
- `getNanoTimeAdjustment(J)J`
- `initLevel()I`
- `isBooted()Z`
- `isJavaLangInvokeInited()Z`
- `isModuleSystemInited()Z`
- `isSystemDomainLoader(Ljava/lang/ClassLoader;)Z`
- `toThreadState(I)Ljava/lang/Thread$State;`

### `jdk/internal/misc/VirtualThreads`

- `park()V`

### `jdk/internal/org/objectweb/asm/AnnotationVisitor`

- `visit(Ljava/lang/String;Ljava/lang/Object;)V`
- `visitAnnotation(Ljava/lang/String;Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitArray(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitEnd()V`
- `visitEnum(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`

### `jdk/internal/org/objectweb/asm/AnnotationWriter`

- `computeAnnotationsSize(Ljava/lang/String;)I`
- `computeAnnotationsSize(Ljdk/internal/org/objectweb/asm/AnnotationWriter;Ljdk/internal/org/objectweb/asm/AnnotationWriter;Ljdk/internal/org/objectweb/asm/AnnotationWriter;Ljdk/internal/org/objectweb/asm/AnnotationWriter;)I`
- `computeParameterAnnotationsSize(Ljava/lang/String;[Ljdk/internal/org/objectweb/asm/AnnotationWriter;I)I`
- `putAnnotations(ILjdk/internal/org/objectweb/asm/ByteVector;)V`
- `putAnnotations(Ljdk/internal/org/objectweb/asm/SymbolTable;Ljdk/internal/org/objectweb/asm/AnnotationWriter;Ljdk/internal/org/objectweb/asm/AnnotationWriter;Ljdk/internal/org/objectweb/asm/AnnotationWriter;Ljdk/internal/org/objectweb/asm/AnnotationWriter;Ljdk/internal/org/objectweb/asm/ByteVector;)V`
- `putParameterAnnotations(I[Ljdk/internal/org/objectweb/asm/AnnotationWriter;ILjdk/internal/org/objectweb/asm/ByteVector;)V`
- `visitEnd()V`

### `jdk/internal/org/objectweb/asm/Attribute`

- `<init>(Ljava/lang/String;)V`
- `computeAttributesSize(Ljdk/internal/org/objectweb/asm/SymbolTable;)I`
- `computeAttributesSize(Ljdk/internal/org/objectweb/asm/SymbolTable;II)I`
- `computeAttributesSize(Ljdk/internal/org/objectweb/asm/SymbolTable;[BIII)I`
- `getAttributeCount()I`
- `putAttributes(Ljdk/internal/org/objectweb/asm/SymbolTable;IILjdk/internal/org/objectweb/asm/ByteVector;)V`
- `putAttributes(Ljdk/internal/org/objectweb/asm/SymbolTable;Ljdk/internal/org/objectweb/asm/ByteVector;)V`
- `putAttributes(Ljdk/internal/org/objectweb/asm/SymbolTable;[BIIILjdk/internal/org/objectweb/asm/ByteVector;)V`
- `read(Ljdk/internal/org/objectweb/asm/ClassReader;II[CI[Ljdk/internal/org/objectweb/asm/Label;)Ljdk/internal/org/objectweb/asm/Attribute;`
- `write(Ljdk/internal/org/objectweb/asm/ClassWriter;[BIII)Ljdk/internal/org/objectweb/asm/ByteVector;`

### `jdk/internal/org/objectweb/asm/Attribute$Set`

- `<init>()V`
- `add(Ljdk/internal/org/objectweb/asm/Attribute;)V`
- `addAttributes(Ljdk/internal/org/objectweb/asm/Attribute;)V`
- `contains(Ljdk/internal/org/objectweb/asm/Attribute;)Z`
- `toArray()[Ljdk/internal/org/objectweb/asm/Attribute;`

### `jdk/internal/org/objectweb/asm/ByteVector`

- `<init>()V`
- `<init>(I)V`
- `<init>([B)V`
- `encodeUtf8(Ljava/lang/String;II)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `enlarge(I)V`
- `put112(III)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `put12(II)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `put122(III)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `putByte(I)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `putByteArray([BII)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `putInt(I)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `putLong(J)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `putShort(I)Ljdk/internal/org/objectweb/asm/ByteVector;`
- `putUTF8(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/ByteVector;`

### `jdk/internal/org/objectweb/asm/ClassReader`

- `<init>([BIZ)V`
- `accept(Ljdk/internal/org/objectweb/asm/ClassVisitor;[Ljdk/internal/org/objectweb/asm/Attribute;I)V`
- `computeImplicitFrame(Ljdk/internal/org/objectweb/asm/Context;)V`
- `createDebugLabel(I[Ljdk/internal/org/objectweb/asm/Label;)V`
- `createLabel(I[Ljdk/internal/org/objectweb/asm/Label;)Ljdk/internal/org/objectweb/asm/Label;`
- `getFirstAttributeOffset()I`
- `getItem(I)I`
- `getItemCount()I`
- `getMaxStringLength()I`
- `getTypeAnnotationBytecodeOffset([II)I`
- `readAttribute([Ljdk/internal/org/objectweb/asm/Attribute;Ljava/lang/String;II[CI[Ljdk/internal/org/objectweb/asm/Label;)Ljdk/internal/org/objectweb/asm/Attribute;`
- `readBootstrapMethodsAttribute(I)[I`
- `readByte(I)I`
- `readClass(I[C)Ljava/lang/String;`
- `readCode(Ljdk/internal/org/objectweb/asm/MethodVisitor;Ljdk/internal/org/objectweb/asm/Context;I)V`
- `readConst(I[C)Ljava/lang/Object;`
- `readConstantDynamic(I[C)Ljdk/internal/org/objectweb/asm/ConstantDynamic;`
- `readElementValue(Ljdk/internal/org/objectweb/asm/AnnotationVisitor;ILjava/lang/String;[C)I`
- `readElementValues(Ljdk/internal/org/objectweb/asm/AnnotationVisitor;IZ[C)I`
- `readField(Ljdk/internal/org/objectweb/asm/ClassVisitor;Ljdk/internal/org/objectweb/asm/Context;I)I`
- `readInt(I)I`
- `readLabel(I[Ljdk/internal/org/objectweb/asm/Label;)Ljdk/internal/org/objectweb/asm/Label;`
- `readLong(I)J`
- `readMethod(Ljdk/internal/org/objectweb/asm/ClassVisitor;Ljdk/internal/org/objectweb/asm/Context;I)I`
- `readModule(I[C)Ljava/lang/String;`
- `readModuleAttributes(Ljdk/internal/org/objectweb/asm/ClassVisitor;Ljdk/internal/org/objectweb/asm/Context;IILjava/lang/String;)V`
- `readPackage(I[C)Ljava/lang/String;`
- `readParameterAnnotations(Ljdk/internal/org/objectweb/asm/MethodVisitor;Ljdk/internal/org/objectweb/asm/Context;IZ)V`
- `readRecordComponent(Ljdk/internal/org/objectweb/asm/ClassVisitor;Ljdk/internal/org/objectweb/asm/Context;I)I`
- `readShort(I)S`
- `readStackMapFrame(IZZLjdk/internal/org/objectweb/asm/Context;)I`
- `readStringish(I[C)Ljava/lang/String;`
- `readTypeAnnotationTarget(Ljdk/internal/org/objectweb/asm/Context;I)I`
- `readTypeAnnotations(Ljdk/internal/org/objectweb/asm/MethodVisitor;Ljdk/internal/org/objectweb/asm/Context;IZ)[I`
- `readUTF8(I[C)Ljava/lang/String;`
- `readUnsignedShort(I)I`
- `readUtf(II[C)Ljava/lang/String;`
- `readUtf(I[C)Ljava/lang/String;`
- `readVerificationTypeInfo(I[Ljava/lang/Object;I[C[Ljdk/internal/org/objectweb/asm/Label;)I`

### `jdk/internal/org/objectweb/asm/ClassTooLargeException`

- `<init>(Ljava/lang/String;I)V`

### `jdk/internal/org/objectweb/asm/ClassVisitor`

- `<init>(I)V`
- `<init>(ILjdk/internal/org/objectweb/asm/ClassVisitor;)V`
- `visit(IILjava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V`
- `visitAnnotation(Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitAttribute(Ljdk/internal/org/objectweb/asm/Attribute;)V`
- `visitEnd()V`
- `visitField(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)Ljdk/internal/org/objectweb/asm/FieldVisitor;`
- `visitInnerClass(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V`
- `visitMethod(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/MethodVisitor;`
- `visitModule(Ljava/lang/String;ILjava/lang/String;)Ljdk/internal/org/objectweb/asm/ModuleVisitor;`
- `visitNestHost(Ljava/lang/String;)V`
- `visitNestMember(Ljava/lang/String;)V`
- `visitOuterClass(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `visitPermittedSubclass(Ljava/lang/String;)V`
- `visitRecordComponent(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/RecordComponentVisitor;`
- `visitSource(Ljava/lang/String;Ljava/lang/String;)V`
- `visitTypeAnnotation(ILjdk/internal/org/objectweb/asm/TypePath;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`

### `jdk/internal/org/objectweb/asm/ClassWriter`

- `<init>(I)V`
- `<init>(Ljdk/internal/org/objectweb/asm/ClassReader;I)V`
- `getAttributePrototypes()[Ljdk/internal/org/objectweb/asm/Attribute;`
- `replaceAsmInstructions([BZ)[B`
- `toByteArray()[B`
- `visit(IILjava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V`
- `visitField(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)Ljdk/internal/org/objectweb/asm/FieldVisitor;`
- `visitMethod(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/MethodVisitor;`
- `visitSource(Ljava/lang/String;Ljava/lang/String;)V`

### `jdk/internal/org/objectweb/asm/ConstantDynamic`

- `<init>(Ljava/lang/String;Ljava/lang/String;Ljdk/internal/org/objectweb/asm/Handle;[Ljava/lang/Object;)V`
- `getBootstrapMethod()Ljdk/internal/org/objectweb/asm/Handle;`
- `getBootstrapMethodArgumentsUnsafe()[Ljava/lang/Object;`
- `getDescriptor()Ljava/lang/String;`
- `getName()Ljava/lang/String;`

### `jdk/internal/org/objectweb/asm/Context`

- `<init>()V`

### `jdk/internal/org/objectweb/asm/Edge`

- `<init>(ILjdk/internal/org/objectweb/asm/Label;Ljdk/internal/org/objectweb/asm/Edge;)V`

### `jdk/internal/org/objectweb/asm/FieldVisitor`

- `<init>(I)V`
- `<init>(ILjdk/internal/org/objectweb/asm/FieldVisitor;)V`
- `visitAnnotation(Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitAttribute(Ljdk/internal/org/objectweb/asm/Attribute;)V`
- `visitEnd()V`
- `visitTypeAnnotation(ILjdk/internal/org/objectweb/asm/TypePath;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`

### `jdk/internal/org/objectweb/asm/FieldWriter`

- `<init>(Ljdk/internal/org/objectweb/asm/SymbolTable;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)V`
- `collectAttributePrototypes(Ljdk/internal/org/objectweb/asm/Attribute$Set;)V`
- `computeFieldInfoSize()I`
- `putFieldInfo(Ljdk/internal/org/objectweb/asm/ByteVector;)V`

### `jdk/internal/org/objectweb/asm/Frame`

- `<init>(Ljdk/internal/org/objectweb/asm/Label;)V`

### `jdk/internal/org/objectweb/asm/Handle`

- `<init>(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V`
- `getDesc()Ljava/lang/String;`
- `getName()Ljava/lang/String;`
- `getOwner()Ljava/lang/String;`
- `getTag()I`
- `hashCode()I`
- `isInterface()Z`

### `jdk/internal/org/objectweb/asm/Handler`

- `getExceptionTableLength(Ljdk/internal/org/objectweb/asm/Handler;)I`
- `getExceptionTableSize(Ljdk/internal/org/objectweb/asm/Handler;)I`
- `putExceptionTable(Ljdk/internal/org/objectweb/asm/Handler;Ljdk/internal/org/objectweb/asm/ByteVector;)V`

### `jdk/internal/org/objectweb/asm/Label`

- `<init>()V`
- `accept(Ljdk/internal/org/objectweb/asm/MethodVisitor;Z)V`
- `addLineNumber(I)V`
- `resolve([BI)Z`

### `jdk/internal/org/objectweb/asm/MethodTooLargeException`

- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V`

### `jdk/internal/org/objectweb/asm/MethodVisitor`

- `<init>(I)V`
- `<init>(ILjdk/internal/org/objectweb/asm/MethodVisitor;)V`
- `visitAnnotableParameterCount(IZ)V`
- `visitAnnotation(Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitAnnotationDefault()Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitAttribute(Ljdk/internal/org/objectweb/asm/Attribute;)V`
- `visitCode()V`
- `visitEnd()V`
- `visitFieldInsn(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `visitFrame(II[Ljava/lang/Object;I[Ljava/lang/Object;)V`
- `visitIincInsn(II)V`
- `visitInsn(I)V`
- `visitInsnAnnotation(ILjdk/internal/org/objectweb/asm/TypePath;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitIntInsn(II)V`
- `visitInvokeDynamicInsn(Ljava/lang/String;Ljava/lang/String;Ljdk/internal/org/objectweb/asm/Handle;[Ljava/lang/Object;)V`
- `visitJumpInsn(ILjdk/internal/org/objectweb/asm/Label;)V`
- `visitLabel(Ljdk/internal/org/objectweb/asm/Label;)V`
- `visitLdcInsn(Ljava/lang/Object;)V`
- `visitLineNumber(ILjdk/internal/org/objectweb/asm/Label;)V`
- `visitLocalVariable(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljdk/internal/org/objectweb/asm/Label;Ljdk/internal/org/objectweb/asm/Label;I)V`
- `visitLocalVariableAnnotation(ILjdk/internal/org/objectweb/asm/TypePath;[Ljdk/internal/org/objectweb/asm/Label;[Ljdk/internal/org/objectweb/asm/Label;[ILjava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitLookupSwitchInsn(Ljdk/internal/org/objectweb/asm/Label;[I[Ljdk/internal/org/objectweb/asm/Label;)V`
- `visitMaxs(II)V`
- `visitMethodInsn(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `visitMethodInsn(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V`
- `visitMultiANewArrayInsn(Ljava/lang/String;I)V`
- `visitParameter(Ljava/lang/String;I)V`
- `visitParameterAnnotation(ILjava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitTableSwitchInsn(IILjdk/internal/org/objectweb/asm/Label;[Ljdk/internal/org/objectweb/asm/Label;)V`
- `visitTryCatchAnnotation(ILjdk/internal/org/objectweb/asm/TypePath;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitTryCatchBlock(Ljdk/internal/org/objectweb/asm/Label;Ljdk/internal/org/objectweb/asm/Label;Ljdk/internal/org/objectweb/asm/Label;Ljava/lang/String;)V`
- `visitTypeAnnotation(ILjdk/internal/org/objectweb/asm/TypePath;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitTypeInsn(ILjava/lang/String;)V`
- `visitVarInsn(II)V`

### `jdk/internal/org/objectweb/asm/MethodWriter`

- `<init>(Ljdk/internal/org/objectweb/asm/SymbolTable;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)V`
- `addSuccessorToCurrentBasicBlock(ILjdk/internal/org/objectweb/asm/Label;)V`
- `canCopyMethodAttributes(Ljdk/internal/org/objectweb/asm/ClassReader;ZZIII)Z`
- `collectAttributePrototypes(Ljdk/internal/org/objectweb/asm/Attribute$Set;)V`
- `computeMethodInfoSize()I`
- `hasAsmInstructions()Z`
- `hasFrames()Z`
- `putMethodInfo(Ljdk/internal/org/objectweb/asm/ByteVector;)V`
- `setMethodAttributesSource(II)V`
- `visitLabel(Ljdk/internal/org/objectweb/asm/Label;)V`

### `jdk/internal/org/objectweb/asm/ModuleVisitor`

- `visitEnd()V`
- `visitExport(Ljava/lang/String;I[Ljava/lang/String;)V`
- `visitMainClass(Ljava/lang/String;)V`
- `visitOpen(Ljava/lang/String;I[Ljava/lang/String;)V`
- `visitPackage(Ljava/lang/String;)V`
- `visitProvide(Ljava/lang/String;[Ljava/lang/String;)V`
- `visitRequire(Ljava/lang/String;ILjava/lang/String;)V`
- `visitUse(Ljava/lang/String;)V`

### `jdk/internal/org/objectweb/asm/ModuleWriter`

- `computeAttributesSize()I`
- `getAttributeCount()I`
- `putAttributes(Ljdk/internal/org/objectweb/asm/ByteVector;)V`

### `jdk/internal/org/objectweb/asm/RecordComponentVisitor`

- `visitAnnotation(Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`
- `visitAttribute(Ljdk/internal/org/objectweb/asm/Attribute;)V`
- `visitEnd()V`
- `visitTypeAnnotation(ILjdk/internal/org/objectweb/asm/TypePath;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/AnnotationVisitor;`

### `jdk/internal/org/objectweb/asm/RecordComponentWriter`

- `collectAttributePrototypes(Ljdk/internal/org/objectweb/asm/Attribute$Set;)V`
- `computeRecordComponentInfoSize()I`
- `putRecordComponentInfo(Ljdk/internal/org/objectweb/asm/ByteVector;)V`

### `jdk/internal/org/objectweb/asm/Symbol`

- `<init>(IILjava/lang/String;Ljava/lang/String;Ljava/lang/String;J)V`

### `jdk/internal/org/objectweb/asm/SymbolTable`

- `<init>(Ljdk/internal/org/objectweb/asm/ClassWriter;)V`
- `<init>(Ljdk/internal/org/objectweb/asm/ClassWriter;Ljdk/internal/org/objectweb/asm/ClassReader;)V`
- `add(Ljdk/internal/org/objectweb/asm/SymbolTable$Entry;)V`
- `addBootstrapMethod(III)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addBootstrapMethod(Ljdk/internal/org/objectweb/asm/Handle;[Ljava/lang/Object;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstant(Ljava/lang/Object;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantClass(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantDouble(D)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantDynamic(Ljava/lang/String;Ljava/lang/String;Ljdk/internal/org/objectweb/asm/Handle;[Ljava/lang/Object;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantDynamicOrInvokeDynamicReference(IILjava/lang/String;Ljava/lang/String;I)V`
- `addConstantDynamicOrInvokeDynamicReference(ILjava/lang/String;Ljava/lang/String;I)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantFieldref(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantFloat(F)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantInteger(I)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantIntegerOrFloat(II)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantIntegerOrFloat(III)V`
- `addConstantLong(J)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantLongOrDouble(IIJ)V`
- `addConstantLongOrDouble(IJ)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantMemberReference(IILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `addConstantMemberReference(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/SymbolTable$Entry;`
- `addConstantMethodHandle(IILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)V`
- `addConstantMethodHandle(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantMethodType(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantMethodref(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantNameAndType(ILjava/lang/String;Ljava/lang/String;)V`
- `addConstantNameAndType(Ljava/lang/String;Ljava/lang/String;)I`
- `addConstantString(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `addConstantUtf8(ILjava/lang/String;)V`
- `addConstantUtf8(Ljava/lang/String;)I`
- `addConstantUtf8Reference(IILjava/lang/String;)V`
- `addConstantUtf8Reference(ILjava/lang/String;)Ljdk/internal/org/objectweb/asm/Symbol;`
- `computeBootstrapMethodsSize()I`
- `copyBootstrapMethods(Ljdk/internal/org/objectweb/asm/ClassReader;[C)V`
- `get(I)Ljdk/internal/org/objectweb/asm/SymbolTable$Entry;`
- `getClassName()Ljava/lang/String;`
- `getConstantPoolCount()I`
- `getConstantPoolLength()I`
- `getMajorVersion()I`
- `getSource()Ljdk/internal/org/objectweb/asm/ClassReader;`
- `hash(II)I`
- `hash(IJ)I`
- `hash(ILjava/lang/String;)I`
- `hash(ILjava/lang/String;Ljava/lang/String;)I`
- `hash(ILjava/lang/String;Ljava/lang/String;I)I`
- `hash(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)I`
- `hash(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;I)I`
- `put(Ljdk/internal/org/objectweb/asm/SymbolTable$Entry;)Ljdk/internal/org/objectweb/asm/SymbolTable$Entry;`
- `putBootstrapMethods(Ljdk/internal/org/objectweb/asm/ByteVector;)V`
- `putConstantPool(Ljdk/internal/org/objectweb/asm/ByteVector;)V`
- `setMajorVersionAndClassName(ILjava/lang/String;)I`

### `jdk/internal/org/objectweb/asm/SymbolTable$Entry`

- `<init>(IIJI)V`
- `<init>(IILjava/lang/String;I)V`
- `<init>(IILjava/lang/String;Ljava/lang/String;I)V`
- `<init>(IILjava/lang/String;Ljava/lang/String;Ljava/lang/String;JI)V`

### `jdk/internal/org/objectweb/asm/Type`

- `<init>(ILjava/lang/String;II)V`
- `getArgumentsAndReturnSizes(Ljava/lang/String;)I`
- `getDescriptor()Ljava/lang/String;`
- `getInternalName()Ljava/lang/String;`
- `getMethodType(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/Type;`
- `getObjectType(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/Type;`
- `getSort()I`
- `getType(Ljava/lang/String;)Ljdk/internal/org/objectweb/asm/Type;`
- `getTypeInternal(Ljava/lang/String;II)Ljdk/internal/org/objectweb/asm/Type;`

### `jdk/internal/org/objectweb/asm/TypePath`

- `<init>([BI)V`

### `jdk/internal/perf/Perf`

- `createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;`

### `jdk/internal/perf/PerfCounter`

- `<init>(Ljava/lang/String;I)V`
- `add(J)V`
- `get()J`
- `increment()V`
- `newPerfCounter(Ljava/lang/String;)Ljdk/internal/perf/PerfCounter;`

### `jdk/internal/ref/CleanerFactory`

- `cleaner()Ljava/lang/ref/Cleaner;`

### `jdk/internal/ref/CleanerImpl`

- `getCleanerImpl(Ljava/lang/ref/Cleaner;)Ljdk/internal/ref/CleanerImpl;`

### `jdk/internal/ref/CleanerImpl$PhantomCleanableRef`

- `<init>(Ljava/lang/Object;Ljava/lang/ref/Cleaner;Ljava/lang/Runnable;)V`

### `jdk/internal/ref/PhantomCleanable`

- `<init>(Ljava/lang/Object;Ljava/lang/ref/Cleaner;)V`
- `insert()V`

### `jdk/internal/reflect/AccessorGenerator`

- `<init>()V`

### `jdk/internal/reflect/BootstrapConstructorAccessorImpl`

- `<init>(Ljava/lang/reflect/Constructor;)V`

### `jdk/internal/reflect/ByteVector`

- `add(B)V`
- `getData()[B`
- `getLength()I`
- `put(IB)V`
- `trim()V`

### `jdk/internal/reflect/ByteVectorFactory`

- `create()Ljdk/internal/reflect/ByteVector;`

### `jdk/internal/reflect/ByteVectorImpl`

- `<init>()V`
- `<init>(I)V`

### `jdk/internal/reflect/ClassFileAssembler`

- `<init>()V`
- `<init>(Ljdk/internal/reflect/ByteVector;)V`
- `cpi()S`
- `decStack()V`
- `emitByte(B)V`
- `emitConstantPoolClass(S)V`
- `emitConstantPoolInterfaceMethodref(SS)V`
- `emitConstantPoolMethodref(SS)V`
- `emitConstantPoolNameAndType(SS)V`
- `emitConstantPoolUTF8(Ljava/lang/String;)V`
- `emitInt(I)V`
- `emitMagicAndVersion()V`
- `emitShort(S)V`
- `emitShort(SS)V`
- `getLength()S`
- `getMaxLocals()S`
- `getStack()I`
- `incStack()V`
- `opc_aaload()V`
- `opc_aconst_null()V`
- `opc_aload_1()V`
- `opc_aload_2()V`
- `opc_aload_3()V`
- `opc_areturn()V`
- `opc_arraylength()V`
- `opc_astore_2()V`
- `opc_astore_3()V`
- `opc_athrow()V`
- `opc_checkcast(S)V`
- `opc_dup()V`
- `opc_dup_x1()V`
- `opc_goto(Ljdk/internal/reflect/Label;)V`
- `opc_if_icmpeq(Ljdk/internal/reflect/Label;)V`
- `opc_ifeq(Ljdk/internal/reflect/Label;)V`
- `opc_ifnonnull(Ljdk/internal/reflect/Label;)V`
- `opc_ifnull(Ljdk/internal/reflect/Label;)V`
- `opc_instanceof(S)V`
- `opc_invokeinterface(SIBI)V`
- `opc_invokespecial(SII)V`
- `opc_invokestatic(SII)V`
- `opc_invokevirtual(SII)V`
- `opc_new(S)V`
- `opc_sipush(S)V`
- `opc_swap()V`
- `setMaxLocals(I)V`
- `setStack(I)V`

### `jdk/internal/reflect/ConstructorAccessor`

- `newInstance([Ljava/lang/Object;)Ljava/lang/Object;`

### `jdk/internal/reflect/ConstructorAccessorImpl`

- `<init>()V`

### `jdk/internal/reflect/DelegatingConstructorAccessorImpl`

- `<init>(Ljdk/internal/reflect/ConstructorAccessorImpl;)V`

### `jdk/internal/reflect/DelegatingMethodAccessorImpl`

- `<init>(Ljdk/internal/reflect/MethodAccessorImpl;)V`

### `jdk/internal/reflect/DirectConstructorHandleAccessor`

- `<init>(Ljava/lang/reflect/Constructor;Ljava/lang/invoke/MethodHandle;)V`
- `constructorAccessor(Ljava/lang/reflect/Constructor;Ljava/lang/invoke/MethodHandle;)Ljdk/internal/reflect/ConstructorAccessorImpl;`
- `nativeAccessor(Ljava/lang/reflect/Constructor;)Ljdk/internal/reflect/ConstructorAccessorImpl;`

### `jdk/internal/reflect/DirectConstructorHandleAccessor$NativeAccessor`

- `<init>(Ljava/lang/reflect/Constructor;)V`

### `jdk/internal/reflect/DirectMethodHandleAccessor`

- `<init>(Ljava/lang/reflect/Method;Ljava/lang/invoke/MethodHandle;Z)V`
- `callerSensitiveAdapter(Ljava/lang/reflect/Method;Ljava/lang/invoke/MethodHandle;)Ljdk/internal/reflect/MethodAccessorImpl;`
- `findCSMethodAdapter(Ljava/lang/reflect/Method;)Ljava/lang/reflect/Method;`
- `methodAccessor(Ljava/lang/reflect/Method;Ljava/lang/invoke/MethodHandle;)Ljdk/internal/reflect/MethodAccessorImpl;`
- `nativeAccessor(Ljava/lang/reflect/Method;Z)Ljdk/internal/reflect/MethodAccessorImpl;`

### `jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor`

- `<init>(Ljava/lang/reflect/Method;)V`
- `<init>(Ljava/lang/reflect/Method;Ljava/lang/reflect/Method;)V`

### `jdk/internal/reflect/InstantiationExceptionConstructorAccessorImpl`

- `<init>(Ljava/lang/String;)V`

### `jdk/internal/reflect/Label`

- `<init>()V`
- `add(Ljdk/internal/reflect/ClassFileAssembler;SSI)V`
- `bind()V`

### `jdk/internal/reflect/Label$PatchInfo`

- `<init>(Ljdk/internal/reflect/ClassFileAssembler;SSI)V`

### `jdk/internal/reflect/MagicAccessorImpl`

- `<init>()V`

### `jdk/internal/reflect/MethodAccessor`

- `invoke(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;`
- `invoke(Ljava/lang/Object;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;`

### `jdk/internal/reflect/MethodAccessorGenerator`

- `<init>()V`
- `add(SS)S`
- `boxingMethodForPrimitiveType(Ljava/lang/Class;)S`
- `buildInternalSignature()Ljava/lang/String;`
- `canWidenTo(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `emitBoxingContantPoolEntries()V`
- `emitCommonConstantPoolEntries()V`
- `emitConstructor()V`
- `emitInvoke()V`
- `emitMethod(SILjdk/internal/reflect/ClassFileAssembler;Ljdk/internal/reflect/ClassFileAssembler;[S)V`
- `emitWideningBytecodeForPrimitiveConversion(Ljdk/internal/reflect/ClassFileAssembler;Ljava/lang/Class;Ljava/lang/Class;)V`
- `generate(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;IZZLjava/lang/Class;)Ljdk/internal/reflect/MagicAccessorImpl;`
- `generateConstructor(Ljava/lang/Class;[Ljava/lang/Class;I)Ljdk/internal/reflect/ConstructorAccessor;`
- `generateMethod(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;I)Ljdk/internal/reflect/MethodAccessor;`
- `generateName(ZZ)Ljava/lang/String;`
- `getClassName(Ljava/lang/Class;Z)Ljava/lang/String;`
- `indexForPrimitiveType(Ljava/lang/Class;)S`
- `isInterface()Z`
- `isPrimitive(Ljava/lang/Class;)Z`
- `isStatic()Z`
- `numNonPrimitiveParameterTypes()I`
- `sub(SS)S`
- `typeSizeInStackSlots(Ljava/lang/Class;)I`
- `unboxingMethodForPrimitiveType(Ljava/lang/Class;)S`
- `usesPrimitiveTypes()Z`

### `jdk/internal/reflect/MethodAccessorGenerator$1`

- `<init>(Ljdk/internal/reflect/MethodAccessorGenerator;Ljava/lang/String;[BLjava/lang/Class;)V`

### `jdk/internal/reflect/MethodAccessorImpl`

- `<init>()V`

### `jdk/internal/reflect/MethodHandleAccessorFactory`

- `ensureClassInitialized(Ljava/lang/Class;)V`
- `findCallerSensitiveAdapter(Ljava/lang/reflect/Method;)Ljava/lang/invoke/MethodHandle;`
- `getDirectMethod(Ljava/lang/reflect/Method;Z)Ljava/lang/invoke/MethodHandle;`
- `isSignaturePolymorphicMethod(Ljava/lang/reflect/Method;)Z`
- `makeSpecializedTarget(Ljava/lang/invoke/MethodHandle;ZZ)Ljava/lang/invoke/MethodHandle;`
- `makeTarget(Ljava/lang/invoke/MethodHandle;ZZ)Ljava/lang/invoke/MethodHandle;`
- `newConstructorAccessor(Ljava/lang/reflect/Constructor;)Ljdk/internal/reflect/ConstructorAccessorImpl;`
- `newMethodAccessor(Ljava/lang/reflect/Method;Z)Ljdk/internal/reflect/MethodAccessorImpl;`
- `slotCount(Ljava/lang/reflect/Executable;)I`
- `specializedMethodType(ZZI)Ljava/lang/invoke/MethodType;`
- `specializedMethodTypeForConstructor(I)Ljava/lang/invoke/MethodType;`
- `useNativeAccessor(Ljava/lang/reflect/Executable;)Z`

### `jdk/internal/reflect/NativeConstructorAccessorImpl`

- `<init>(Ljava/lang/reflect/Constructor;)V`
- `getParent()Ljdk/internal/reflect/DelegatingConstructorAccessorImpl;`

### `jdk/internal/reflect/NativeMethodAccessorImpl`

- `<init>(Ljava/lang/reflect/Method;)V`
- `getParent()Ljdk/internal/reflect/DelegatingMethodAccessorImpl;`

### `jdk/internal/reflect/Reflection`

- `areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z`
- `filter([Ljava/lang/reflect/Member;Ljava/util/Set;)[Ljava/lang/reflect/Member;`
- `filterMethods(Ljava/lang/Class;[Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;`
- `getCallerClass()Ljava/lang/Class;`
- `getClassAccessFlags(Ljava/lang/Class;)I`
- `isCallerSensitive(Ljava/lang/reflect/Method;)Z`
- `isSubclassOf(Ljava/lang/Class;Ljava/lang/Class;)Z`

### `jdk/internal/reflect/ReflectionFactory`

- `config()Ljdk/internal/reflect/ReflectionFactory$Config;`
- `copyConstructor(Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;`
- `copyMethod(Ljava/lang/reflect/Method;)Ljava/lang/reflect/Method;`
- `generateMethodAccessor(Ljava/lang/reflect/Method;)Ljdk/internal/reflect/MethodAccessorImpl;`
- `getExecutableSharedParameterTypes(Ljava/lang/reflect/Executable;)[Ljava/lang/Class;`
- `loadConfig()Ljdk/internal/reflect/ReflectionFactory$Config;`
- `newConstructorAccessor(Ljava/lang/reflect/Constructor;)Ljdk/internal/reflect/ConstructorAccessor;`
- `newMethodAccessor(Ljava/lang/reflect/Method;Z)Ljdk/internal/reflect/MethodAccessor;`
- `noInflation()Z`
- `useMethodHandleAccessor()Z`
- `useNativeAccessorOnly()Z`

### `jdk/internal/reflect/ReflectionFactory$Config`

- `<init>(ZIIZZ)V`

### `jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction`

- `<init>()V`

### `jdk/internal/reflect/UTF8`

- `encode(Ljava/lang/String;)[B`
- `utf8Length(Ljava/lang/String;)I`

### `jdk/internal/util/ArraysSupport`

- `hashCode(I[BII)I`
- `hashCode(I[CII)I`
- `hashCode(I[III)I`
- `hashCode(I[SII)I`
- `hugeLength(II)I`
- `mismatch([BI[BII)I`
- `mismatch([B[BI)I`
- `newLength(III)I`
- `signedHashCode(I[BII)I`
- `utf16hashCode(I[BII)I`
- `vectorizedHashCode(Ljava/lang/Object;IIII)I`
- `vectorizedMismatch(Ljava/lang/Object;JLjava/lang/Object;JII)I`

### `jdk/internal/util/ByteArray`

- `getInt([BI)I`

### `jdk/internal/util/ClassFileDumper`

- `dumpClass(Ljava/lang/String;Ljava/lang/Class;[B)V`
- `dumpFailedClass(Ljava/lang/String;[B)V`
- `encodeForFilename(Ljava/lang/String;)Ljava/lang/String;`
- `isEnabled()Z`
- `pathname(Ljava/lang/String;)Ljava/nio/file/Path;`
- `write(Ljava/nio/file/Path;[B)V`

### `jdk/internal/util/ClassFileDumper$1`

- `<init>(Ljdk/internal/util/ClassFileDumper;Ljava/nio/file/Path;[B)V`

### `jdk/internal/util/Preconditions`

- `checkFromIndexSize(IIILjava/util/function/BiFunction;)I`
- `checkFromToIndex(IIILjava/util/function/BiFunction;)I`
- `checkIndex(IILjava/util/function/BiFunction;)I`
- `outOfBounds(Ljava/util/function/BiFunction;Ljava/lang/String;[Ljava/lang/Number;)Ljava/lang/RuntimeException;`
- `outOfBoundsCheckFromIndexSize(Ljava/util/function/BiFunction;III)Ljava/lang/RuntimeException;`
- `outOfBoundsCheckFromToIndex(Ljava/util/function/BiFunction;III)Ljava/lang/RuntimeException;`
- `outOfBoundsCheckIndex(Ljava/util/function/BiFunction;II)Ljava/lang/RuntimeException;`
- `outOfBoundsMessage(Ljava/lang/String;Ljava/util/List;)Ljava/lang/String;`

### `jdk/internal/util/ReferenceKey`

- `get()Ljava/lang/Object;`
- `unused()V`

### `jdk/internal/util/ReferencedKeyMap`

- `entryKey(Ljava/lang/Object;)Ljdk/internal/util/ReferenceKey;`
- `existingKey(Ljdk/internal/util/ReferencedKeyMap;Ljava/lang/Object;)Ljava/lang/Object;`
- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `getNoCheckStale(Ljava/lang/Object;)Ljava/lang/Object;`
- `intern(Ljdk/internal/util/ReferencedKeyMap;Ljava/lang/Object;)Ljava/lang/Object;`
- `internKey(Ljdk/internal/util/ReferencedKeyMap;Ljava/lang/Object;)Ljava/lang/Object;`
- `lookupKey(Ljava/lang/Object;)Ljdk/internal/util/ReferenceKey;`
- `removeStaleReferences()V`

### `jdk/internal/util/ReferencedKeySet`

- `get(Ljava/lang/Object;)Ljava/lang/Object;`
- `intern(Ljava/lang/Object;)Ljava/lang/Object;`

### `jdk/internal/util/SoftReferenceKey`

- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`

### `jdk/internal/util/StrongReferenceKey`

- `<init>(Ljava/lang/Object;)V`

### `jdk/internal/util/WeakReferenceKey`

- `<init>(Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V`

### `jdk/internal/util/random/RandomSupport`

- `mixMurmur64(J)J`

### `jdk/internal/util/regex/Grapheme`

- `getType(I)I`
- `isExcludedSpacingMark(I)Z`
- `nextBoundary(Ljava/lang/CharSequence;II)I`

### `jdk/internal/vm/Continuation`

- `currentCarrierThread()Ljava/lang/Thread;`
- `doYield()I`
- `onContinue()V`
- `onPinned(Ljdk/internal/vm/Continuation$Pinned;)V`
- `onPinned0(I)V`
- `pinnedReason(I)Ljdk/internal/vm/Continuation$Pinned;`
- `yield(Ljdk/internal/vm/ContinuationScope;)Z`
- `yield0(Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)Z`

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

### `sun/nio/ch/Interruptible`

- `interrupt(Ljava/lang/Thread;)V`

### `sun/nio/cs/ArrayEncoder`

- `encodeFromLatin1([BII[B)I`
- `encodeFromUTF16([BII[B)I`
- `isASCIICompatible()Z`

### `sun/nio/cs/StreamEncoder`

- `flushBuffer()V`
- `implFlushBuffer()V`
- `isOpen()Z`
- `lockedFlushBuffer()V`
- `writeBytes()V`

### `sun/nio/fs/DefaultFileSystemProvider`

- `theFileSystem()Ljava/nio/file/FileSystem;`

### `sun/nio/fs/MacOSXFileSystemProvider`

- `theFileSystem()Lsun/nio/fs/UnixFileSystem;`

### `sun/reflect/generics/factory/CoreReflectionFactory`

- `<init>(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)V`
- `make(Ljava/lang/reflect/GenericDeclaration;Lsun/reflect/generics/scope/Scope;)Lsun/reflect/generics/factory/CoreReflectionFactory;`

### `sun/reflect/generics/repository/AbstractRepository`

- `<init>(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V`
- `parse(Ljava/lang/String;)Lsun/reflect/generics/tree/Tree;`

### `sun/reflect/generics/repository/ClassRepository`

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

- `<init>(Ljava/lang/Class;)V`
- `make(Ljava/lang/Class;)Lsun/reflect/generics/scope/ClassScope;`

### `sun/reflect/generics/tree/ClassSignature`

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

- `privilegedGetProperties()Ljava/util/Properties;`

### `sun/security/action/GetPropertyAction$1`

- `<init>()V`

### `sun/security/provider/PolicyFile`

- `<init>()V`
- `init(Ljava/net/URL;)V`
- `initPolicyFile(Ljava/lang/String;Ljava/lang/String;Lsun/security/provider/PolicyFile$PolicyInfo;)Z`
- `initPolicyFile(Lsun/security/provider/PolicyFile$PolicyInfo;Ljava/net/URL;)V`
- `initStaticPolicy(Lsun/security/provider/PolicyFile$PolicyInfo;)V`

### `sun/security/provider/PolicyFile$1`

- `<init>(Lsun/security/provider/PolicyFile;)V`

### `sun/security/provider/PolicyFile$2`

- `<init>(Lsun/security/provider/PolicyFile;Lsun/security/provider/PolicyFile$PolicyInfo;)V`

### `sun/security/provider/PolicyFile$3`

- `<init>(Lsun/security/provider/PolicyFile;Ljava/net/URL;Lsun/security/provider/PolicyFile$PolicyInfo;)V`

### `sun/security/provider/PolicyFile$4`

- `<init>(Lsun/security/provider/PolicyFile;Ljava/lang/String;Lsun/security/provider/PolicyFile$PolicyInfo;Ljava/lang/String;)V`

### `sun/security/provider/PolicyFile$5`

- `<init>(Lsun/security/provider/PolicyFile;Lsun/security/provider/PolicyFile$PolicyInfo;)V`

### `sun/security/provider/PolicyFile$PolicyInfo`

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

- `getEncodedInternal()[B`
- `getEncodedInternal(Ljava/security/cert/Certificate;)[B`

### `sun/text/Normalizer`

- `getCombiningClass(I)I`

### `sun/util/locale/BaseLocale`

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

- `<init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V`
- `hashCode(Lsun/util/locale/BaseLocale;)I`

### `sun/util/locale/Extension`

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

- `<init>(C)V`
- `<init>(Ljava/lang/String;)V`
- `value()C`

### `sun/util/locale/InternalLocaleBuilder$CaseInsensitiveString`

- `<init>(Ljava/lang/String;)V`
- `value()Ljava/lang/String;`

### `sun/util/locale/LanguageTag`

- `isExtensionSingleton(Ljava/lang/String;)Z`
- `isExtensionSubtag(Ljava/lang/String;)Z`
- `isPrivateusePrefix(Ljava/lang/String;)Z`
- `isPrivateusePrefixChar(C)Z`
- `isPrivateuseSubtag(Ljava/lang/String;)Z`

### `sun/util/locale/LocaleExtensions`

- `<init>(Ljava/util/Map;Ljava/util/Set;Ljava/util/Map;)V`
- `equals(Ljava/lang/Object;)Z`
- `hashCode()I`
- `isEmpty()Z`
- `toID(Ljava/util/SortedMap;)Ljava/lang/String;`

### `sun/util/locale/LocaleSyntaxException`

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

- `<init>(Ljava/util/SortedSet;Ljava/util/SortedMap;)V`
- `isAttribute(Ljava/lang/String;)Z`
- `isKey(Ljava/lang/String;)Z`
- `isSingletonChar(C)Z`
- `setValue(Ljava/lang/String;)V`

### `sun/util/locale/provider/LocaleProviderAdapter`

- `findAdapter(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;`
- `forJRE()Lsun/util/locale/provider/LocaleProviderAdapter;`
- `forType(Lsun/util/locale/provider/LocaleProviderAdapter$Type;)Lsun/util/locale/provider/LocaleProviderAdapter;`
- `getAdapter(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;`
- `getAdapterPreference()Ljava/util/List;`
- `getBreakIteratorProvider()Ljava/text/spi/BreakIteratorProvider;`
- `getLocaleServiceProvider(Ljava/lang/Class;)Ljava/util/spi/LocaleServiceProvider;`

### `sun/util/locale/provider/LocaleProviderAdapter$Type`

- `getAdapterClassName()Ljava/lang/String;`
- `ordinal()I`

## 仅引用类（new/field，无方法调用）

- `java/io/FilePermission`
- `java/io/Serializable`
- `java/lang/BaseVirtualThread`
- `java/lang/Byte$ByteCache`
- `java/lang/Character$CharacterCache`
- `java/lang/CharacterData00`
- `java/lang/CharacterData01`
- `java/lang/CharacterData02`
- `java/lang/CharacterData03`
- `java/lang/CharacterData0E`
- `java/lang/CharacterDataPrivateUse`
- `java/lang/CharacterDataUndefined`
- `java/lang/Integer$IntegerCache`
- `java/lang/Long$LongCache`
- `java/lang/Module$ReflectionData`
- `java/lang/NoSuchFieldError`
- `java/lang/NoSuchMethodError`
- `java/lang/Short$ShortCache`
- `java/lang/StackWalker$ExtendedOption`
- `java/lang/StackWalker$Option`
- `java/lang/Thread$Constants`
- `java/lang/Thread$FieldHolder`
- `java/lang/Thread$State`
- `java/lang/Void`
- `java/lang/constant/Constable`
- `java/lang/invoke/ClassSpecializer$SpeciesData`
- `java/lang/invoke/DirectMethodHandle$2`
- `java/lang/invoke/InvokerBytecodeGenerator$1`
- `java/lang/invoke/MethodHandleImpl$Makers`
- `java/lang/invoke/MethodHandles$1`
- `java/lang/ref/FinalReference`
- `java/lang/reflect/Type`
- `java/nio/charset/CoderResult$Cache`
- `java/nio/charset/CodingErrorAction`
- `java/nio/file/FileSystems$DefaultFileSystemHolder`
- `java/security/AllPermission`
- `java/util/Locale$Category`
- `java/util/ResourceBundle$SingleFormatControl`
- `java/util/concurrent/ConcurrentHashMap$ReservationNode`
- `java/util/regex/Pattern$First`
- `java/util/regex/Pattern$LookBehindEndNode`
- `java/util/regex/Pattern$Qtype`
- `java/util/zip/ZipUtils`
- `jdk/internal/access/JavaSecurityAccess$ProtectionDomainCache`
- `jdk/internal/icu/impl/Norm2AllModes$NFCSingleton`
- `jdk/internal/icu/impl/Norm2AllModes$Norm2AllModesSingleton`
- `jdk/internal/icu/text/NormalizerBase$1`
- `jdk/internal/misc/UnsafeConstants`
- `jdk/internal/org/objectweb/asm/Opcodes`
- `jdk/internal/reflect/MethodHandleAccessorFactory$LazyStaticHolder`
- `jdk/internal/util/StaticProperty`
- `jdk/internal/vm/Continuation$Pinned`
- `sun/invoke/util/ValueConversions$1`
- `sun/nio/cs/ISO_8859_1`
- `sun/nio/cs/US_ASCII`
- `sun/nio/cs/UTF_8`
- `sun/security/util/Debug$FormatHolder`
- `sun/security/util/SecurityConstants`
