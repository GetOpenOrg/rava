#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Optional",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Optional.java",
))]
pub struct Optional<T> {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub value: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Optional<T> {
    // java: empty()Ljava/util/Optional;
    pub fn empty() -> Result<Object> {
        let mut t: Object = Optional::EMPTY();
        Ok(t)
    }

    // java: <init>(Ljava/lang/Object;)V
    pub fn new(value: T) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    // java: of(Ljava/lang/Object;)Ljava/util/Optional;
    pub fn of(value: T) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(value)?;
        Ok(Optional::new(_t0)?)
    }

    // java: ofNullable(Ljava/lang/Object;)Ljava/util/Optional;
    pub fn ofNullable(value: T) -> Result<Object> {
        Ok(Optional::new(value)?)
    }

    // java: get()Ljava/lang/Object;
    pub fn get(&self) -> Result<T> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this.value.get())
    }

    // java: isPresent()Z
    pub fn isPresent(&self) -> Result<bool> {
        let this = self;
        Ok(!this.value.get().is_none())
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(this.value.get().is_none())
    }

    // java: ifPresent(Ljava/util/function/Consumer;)V
    pub fn ifPresent(&self, action: Object) -> Result<()> {
        let this = self;
        action.accept(this.value.get())?;
        Ok(())
    }

    // java: ifPresentOrElse(Ljava/util/function/Consumer;Ljava/lang/Runnable;)V
    pub fn ifPresentOrElse(&self, action: Object, emptyAction: Object) -> Result<()> {
        let this = self;
        action.accept(this.value.get())?;
        emptyAction.run()?;
        Ok(())
    }

    // java: filter(Ljava/util/function/Predicate;)Ljava/util/Optional;
    pub fn filter(&self, predicate: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(predicate)?;
        let _t1 = this.isEmpty()?;
        return Ok(this);
        let _t2 = predicate.test(this.value.get())?;
        let _t3: Object = Optional::empty()?;
        Ok(_t3)
    }

    // java: map(Ljava/util/function/Function;)Ljava/util/Optional;
    pub fn map(&self, mapper: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(mapper)?;
        let _t1 = this.isEmpty()?;
        let _t2: Object = Optional::empty()?;
        return Ok(_t2);
        let _t3 = mapper.apply(this.value.get())?;
        let _t4: Object = Optional::ofNullable(_t3)?;
        Ok(_t4)
    }

    // java: flatMap(Ljava/util/function/Function;)Ljava/util/Optional;
    pub fn flatMap(&self, mapper: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(mapper)?;
        let _t1 = this.isEmpty()?;
        let _t2: Object = Optional::empty()?;
        return Ok(_t2);
        let _t3 = mapper.apply(this.value.get())?;
        let mut r: Object = _t3;
        let _t4: Object = Objects::requireNonNull__obj(r)?;
        Ok(_t4)
    }

    // java: or(Ljava/util/function/Supplier;)Ljava/util/Optional;
    pub fn or(&self, supplier: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(supplier)?;
        let _t1 = this.isPresent()?;
        return Ok(this);
        let _t2 = supplier.get()?;
        let mut r: Object = _t2;
        let _t3: Object = Objects::requireNonNull__obj(r)?;
        Ok(_t3)
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isEmpty()?;
        let _t1: Object = Stream::empty()?;
        return Ok(_t1);
        let _t2: Object = Stream::of(this.value.get())?;
        Ok(_t2)
    }

    // java: orElse(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn orElse(&self, other: T) -> Result<T> {
        let this = self;
        Ok(other)
    }

    // java: orElseGet(Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn orElseGet(&self, supplier: Object) -> Result<T> {
        let this = self;
        let _t0 = supplier.get()?;
        Ok(_t0)
    }

    // java: orElseThrow()Ljava/lang/Object;
    // java: orElseThrow()Ljava/lang/Object;
    pub fn orElseThrow(&self) -> Result<T> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this.value.get())
    }

    // java: orElseThrow(Ljava/util/function/Supplier;)Ljava/lang/Object;
    // java: orElseThrow(Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn orElseThrow__suppli(&self, exceptionSupplier: Object) -> Result<T> {
        let this = self;
        return Ok(this.value.get());
        let _t0 = exceptionSupplier.get()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut other: Object = obj;
        let _t0: bool = Objects::equals(this.value.get(), other.value.get())?;
        Ok(_t0!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Objects::hashCode(this.value.get())?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Optional["))?;
        String::new().append(&this.value.get())?;
        String::new().append(&String::from("]"))?;
        Ok(String::from("Optional.empty"))
    }
}
