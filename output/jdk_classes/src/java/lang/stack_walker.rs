#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StackWalker",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "StackWalker.java",
))]
pub struct StackWalker {
    #[cfg_attr(any(), java_field(name = "continuation", descriptor = "Ljdk/internal/vm/Continuation;", access = "private final"))]
    pub continuation: Field<Object>,
    #[cfg_attr(any(), java_field(name = "contScope", descriptor = "Ljdk/internal/vm/ContinuationScope;", access = "private final"))]
    pub contScope: Field<Object>,
    #[cfg_attr(any(), java_field(name = "options", descriptor = "Ljava/util/Set;", access = "private final"))]
    pub options: Field<Object>,
    #[cfg_attr(any(), java_field(name = "extendedOption", descriptor = "Ljava/lang/StackWalker$ExtendedOption;", access = "private final"))]
    pub extendedOption: Field<Object>,
    #[cfg_attr(any(), java_field(name = "estimateDepth", descriptor = "I", access = "private final"))]
    pub estimateDepth: Field<i32>,
    #[cfg_attr(any(), java_field(name = "retainClassRef", descriptor = "Z", access = "final"))]
    pub retainClassRef: Field<bool>,
}

impl StackWalker {
    // java: getInstance()Ljava/lang/StackWalker;
    // java: getInstance()Ljava/lang/StackWalker;
    pub fn getInstance() -> Result<Object> {
        Ok(StackWalker::DEFAULT_WALKER())
    }

    // java: getInstance(Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    // java: getInstance(Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    pub fn getInstance__contin(contScope: Object) -> Result<Object> {
        let _t0: Object = EnumSet::noneOf(7i32)?;
        let _t1: Object = StackWalker::getInstance(_t0, contScope)?;
        Ok(_t1)
    }

    // java: getInstance(Ljava/lang/StackWalker$Option;)Ljava/lang/StackWalker;
    // java: getInstance(Ljava/lang/StackWalker$Option;)Ljava/lang/StackWalker;
    pub fn getInstance__stackw(option: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(option)?;
        let _t1: Object = EnumSet::of(_t0)?;
        let _t2: Object = StackWalker::getInstance(_t1)?;
        Ok(_t2)
    }

    // java: getInstance(Ljava/lang/StackWalker$Option;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    // java: getInstance(Ljava/lang/StackWalker$Option;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    pub fn getInstance__stackw_contin(option: Object, contScope: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(option)?;
        let _t1: Object = EnumSet::of(_t0)?;
        let _t2: Object = StackWalker::getInstance(_t1, contScope)?;
        Ok(_t2)
    }

    // java: getInstance(Ljava/util/Set;)Ljava/lang/StackWalker;
    // java: getInstance(Ljava/util/Set;)Ljava/lang/StackWalker;
    pub fn getInstance__set(options: Object) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = StackWalker::getInstance(todo!("stack underflow"), options)?;
        Ok(_t0)
    }

    // java: getInstance(Ljava/util/Set;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    // java: getInstance(Ljava/util/Set;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    pub fn getInstance__set_contin(options: Object, contScope: Object) -> Result<Object> {
        let _t0 = options.isEmpty()?;
        return Ok(StackWalker::DEFAULT_WALKER());
        let _t1: Object = StackWalker::toEnumSet(options)?;
        let mut optionSet: Object = _t1;
        StackWalker::checkPermission(optionSet)?;
        Ok(StackWalker::new(optionSet, contScope)?)
    }

    // java: getInstance(Ljava/util/Set;I)Ljava/lang/StackWalker;
    // java: getInstance(Ljava/util/Set;I)Ljava/lang/StackWalker;
    pub fn getInstance__set_i(options: Object, estimateDepth: i32) -> Result<Object> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: Object = StackWalker::toEnumSet(options)?;
        let mut optionSet: Object = _t0;
        StackWalker::checkPermission(optionSet)?;
        Ok(StackWalker::new(optionSet, estimateDepth)?)
    }

    // java: <init>(Ljava/util/EnumSet;)V
    // java: <init>(Ljava/util/EnumSet;)V
    pub fn new__enumse(options: Object) -> Result<Self> {
        let this = Self { continuation: Field::new(Default::default()), contScope: Field::new(Default::default()), options: Field::new(Default::default()), extendedOption: Field::new(Default::default()), estimateDepth: Field::new(0), retainClassRef: Field::new(false) };
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/StackWalker.<init>:(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/EnumSet;Ljdk/internal/vm/ContinuationScope;)V
    // java: <init>(Ljava/util/EnumSet;Ljdk/internal/vm/ContinuationScope;)V
    pub fn new__enumse_contin(options: Object, contScope: Object) -> Result<Self> {
        let this = Self { continuation: Field::new(Default::default()), contScope: Field::new(Default::default()), options: Field::new(Default::default()), extendedOption: Field::new(Default::default()), estimateDepth: Field::new(0), retainClassRef: Field::new(false) };
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/StackWalker.<init>:(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/EnumSet;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V
    // java: <init>(Ljava/util/EnumSet;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V
    pub fn new__enumse_contin_contin(options: Object, contScope: Object, continuation: Object) -> Result<Self> {
        let this = Self { continuation: Field::new(Default::default()), contScope: Field::new(Default::default()), options: Field::new(Default::default()), extendedOption: Field::new(Default::default()), estimateDepth: Field::new(0), retainClassRef: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/StackWalker.<init>:(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/EnumSet;I)V
    // java: <init>(Ljava/util/EnumSet;I)V
    pub fn new__enumse_i(options: Object, estimateDepth: i32) -> Result<Self> {
        let this = Self { continuation: Field::new(Default::default()), contScope: Field::new(Default::default()), options: Field::new(Default::default()), extendedOption: Field::new(Default::default()), estimateDepth: Field::new(0), retainClassRef: Field::new(false) };
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/StackWalker.<init>:(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/EnumSet;ILjdk/internal/vm/ContinuationScope;)V
    // java: <init>(Ljava/util/EnumSet;ILjdk/internal/vm/ContinuationScope;)V
    pub fn new__enumse_i_contin(options: Object, estimateDepth: i32, contScope: Object) -> Result<Self> {
        let this = Self { continuation: Field::new(Default::default()), contScope: Field::new(Default::default()), options: Field::new(Default::default()), extendedOption: Field::new(Default::default()), estimateDepth: Field::new(0), retainClassRef: Field::new(false) };
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/StackWalker.<init>:(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/EnumSet;ILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V
    // java: <init>(Ljava/util/EnumSet;ILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V
    pub fn new__enumse_i_contin_contin(options: Object, estimateDepth: i32, contScope: Object, continuation: Object) -> Result<Self> {
        let this = Self { continuation: Field::new(Default::default()), contScope: Field::new(Default::default()), options: Field::new(Default::default()), extendedOption: Field::new(Default::default()), estimateDepth: Field::new(0), retainClassRef: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/StackWalker.<init>:(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V */
        Ok(this)
    }

    // java: <init>(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V
    // java: <init>(Ljava/util/EnumSet;ILjava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)V
    pub fn new__enumse_i_stackw_contin_contin(options: Object, estimateDepth: i32, extendedOption: Object, contScope: Object, continuation: Object) -> Result<Self> {
        let this = Self { continuation: Field::new(Default::default()), contScope: Field::new(Default::default()), options: Field::new(Default::default()), extendedOption: Field::new(Default::default()), estimateDepth: Field::new(0), retainClassRef: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.options.set(options);
        this.estimateDepth.set(estimateDepth);
        this.extendedOption.set(extendedOption);
        let _t0 = this.hasOption(StackWalker_Option::RETAIN_CLASS_REFERENCE())?;
        this.retainClassRef.set(_t0);
        this.contScope.set(contScope);
        this.continuation.set(continuation);
        Ok(this)
    }

    // java: checkPermission(Ljava/util/Set;)V
    pub fn checkPermission(options: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull(options)?;
        let _t1: Object = System::getSecurityManager()?;
        let mut sm: Object = _t1;
        let _t2 = options.contains(StackWalker_Option::RETAIN_CLASS_REFERENCE())?;
        sm.checkPermission(RuntimePermission::new(String::from("getStackWalkerWithClassReference"))?)?;
        Ok(())
    }

    // java: toEnumSet(Ljava/util/Set;)Ljava/util/EnumSet;
    pub fn toEnumSet(options: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(options)?;
        let _t1 = options.isEmpty()?;
        return Ok(StackWalker::DEFAULT_EMPTY_OPTION());
        let _t2: Object = EnumSet::copyOf(options)?;
        Ok(_t2)
    }

    // java: walk(Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn walk(&self, function: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(function)?;
        let _t1: Object = StackStreamFactory::makeStackTraverser(this, function)?;
        let _t2 = _t1.walk()?;
        Ok(_t2)
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        /* TODO: invokedynamic 140 */
        let _t1: Object = StackStreamFactory::makeStackTraverser(this, action)?;
        let _t2 = _t1.walk()?;
        Ok(())
    }

    // java: getCallerClass()Ljava/lang/Class;
    pub fn getCallerClass(&self) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: Object = StackStreamFactory::makeCallerFinder(this)?;
        let _t1 = _t0.findCaller()?;
        Ok(_t1)
    }

    // java: newInstance(Ljava/util/Set;Ljava/lang/StackWalker$ExtendedOption;)Ljava/lang/StackWalker;
    // java: newInstance(Ljava/util/Set;Ljava/lang/StackWalker$ExtendedOption;)Ljava/lang/StackWalker;
    pub fn newInstance__set_stackw(options: Object, extendedOption: Object) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = StackWalker::newInstance(todo!("stack underflow"), options, extendedOption)?;
        Ok(_t0)
    }

    // java: newInstance(Ljava/util/Set;Ljava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    // java: newInstance(Ljava/util/Set;Ljava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;)Ljava/lang/StackWalker;
    pub fn newInstance__set_stackw_contin(options: Object, extendedOption: Object, contScope: Object) -> Result<Object> {
        let _t0: Object = StackWalker::toEnumSet(options)?;
        let mut optionSet: Object = _t0;
        StackWalker::checkPermission(optionSet)?;
        /* TODO: aconst_null  */
        let mut _obj1: StackWalker = StackWalker::new(StackWalker::new(), optionSet, 0i32, extendedOption, contScope)?;
        Ok(_obj1)
    }

    // java: newInstance(Ljava/util/Set;Ljava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)Ljava/lang/StackWalker;
    // java: newInstance(Ljava/util/Set;Ljava/lang/StackWalker$ExtendedOption;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)Ljava/lang/StackWalker;
    pub fn newInstance__set_stackw_contin_contin(options: Object, extendedOption: Object, contScope: Object, continuation: Object) -> Result<Object> {
        let _t0: Object = StackWalker::toEnumSet(options)?;
        let mut optionSet: Object = _t0;
        StackWalker::checkPermission(optionSet)?;
        Ok(StackWalker::new(optionSet, 0i32, extendedOption, contScope, continuation)?)
    }

    // java: estimateDepth()I
    pub fn estimateDepth(&self) -> Result<i32> {
        let this = self;
        Ok(this.estimateDepth.get())
    }

    // java: hasOption(Ljava/lang/StackWalker$Option;)Z
    pub fn hasOption(&self, option: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.options.get().contains(option)?;
        Ok(_t0)
    }

    // java: hasLocalsOperandsOption()Z
    pub fn hasLocalsOperandsOption(&self) -> Result<bool> {
        let this = self;
        Ok(/* if_acmpne */ true)
    }

    // java: getContScope()Ljdk/internal/vm/ContinuationScope;
    pub fn getContScope(&self) -> Result<Object> {
        let this = self;
        Ok(this.contScope.get())
    }

    // java: getContinuation()Ljdk/internal/vm/Continuation;
    pub fn getContinuation(&self) -> Result<Object> {
        let this = self;
        Ok(this.continuation.get())
    }
}
