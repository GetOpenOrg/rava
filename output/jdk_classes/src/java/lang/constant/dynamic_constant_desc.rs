#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/constant/DynamicConstantDesc",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/constant/ConstantDesc",
    access      = "public abstract",
    source      = "DynamicConstantDesc.java",
))]
pub struct DynamicConstantDesc<T> {
    #[cfg_attr(any(), java_field(name = "bootstrapMethod", descriptor = "Ljava/lang/constant/DirectMethodHandleDesc;", access = "private final"))]
    pub bootstrapMethod: Field<Object>,
    #[cfg_attr(any(), java_field(name = "bootstrapArgs", descriptor = "[Ljava/lang/constant/ConstantDesc;", access = "private final"))]
    pub bootstrapArgs: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "constantName", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub constantName: Field<String>,
    #[cfg_attr(any(), java_field(name = "constantType", descriptor = "Ljava/lang/constant/ClassDesc;", access = "private final"))]
    pub constantType: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> DynamicConstantDesc<T> {
    // java: <init>(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)V
    pub fn new(bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: Vec<Object>) -> Result<Self> {
        let this = Self { bootstrapMethod: Field::new(Default::default()), bootstrapArgs: Field::new(Default::default()), constantName: Field::new(String::new()), constantType: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(bootstrapMethod)?;
        this.bootstrapMethod.set(_t0);
        let _t1: Object = Objects::requireNonNull__obj(constantName)?;
        let _t2: String = ConstantUtils::validateMemberName(_t1, 1i32)?;
        this.constantName.set(_t2);
        let _t3: Object = Objects::requireNonNull__obj(constantType)?;
        this.constantType.set(_t3);
        let _t4: Object = Objects::requireNonNull__obj(&bootstrapArgs)?;
        let _t5 = _t4.clone()?;
        this.bootstrapArgs.set(_t5);
        let _t6 = constantName.length()?;
        String::new().append(&String::from("Illegal invocation name:"))?;
        String::new().append(&constantName)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this)
    }

    // java: ofCanonical(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn ofCanonical(bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: &[Object]) -> Result<Object> {
        let _t0: Object = DynamicConstantDesc::ofNamed(bootstrapMethod, constantName, constantType, &bootstrapArgs)?;
        let _t1 = _t0.tryCanonicalize()?;
        Ok(_t1)
    }

    // java: ofNamed(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn ofNamed(bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: &[Object]) -> Result<Object> {
        Ok(DynamicConstantDesc_AnonymousDynamicConstantDesc::new(bootstrapMethod, constantName, constantType, bootstrapArgs)?)
    }

    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;
    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn of__direct_arr_con(bootstrapMethod: Object, bootstrapArgs: &[Object]) -> Result<Object> {
        let _t0 = bootstrapMethod.invocationType()?;
        let _t1 = _t0.returnType()?;
        let _t2: Object = DynamicConstantDesc::ofNamed(bootstrapMethod, String::from("_"), _t1, &bootstrapArgs)?;
        Ok(_t2)
    }

    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;)Ljava/lang/constant/DynamicConstantDesc;
    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn of__direct(bootstrapMethod: Object) -> Result<Object> {
        let _t0: Object = DynamicConstantDesc::of__direct_arr_con(bootstrapMethod, &ConstantUtils::EMPTY_CONSTANTDESC())?;
        Ok(_t0)
    }

    // java: constantName()Ljava/lang/String;
    pub fn constantName(&self) -> Result<String> {
        let this = self;
        Ok(this.constantName.get())
    }

    // java: constantType()Ljava/lang/constant/ClassDesc;
    pub fn constantType(&self) -> Result<Object> {
        let this = self;
        Ok(this.constantType.get())
    }

    // java: bootstrapMethod()Ljava/lang/constant/DirectMethodHandleDesc;
    pub fn bootstrapMethod(&self) -> Result<Object> {
        let this = self;
        Ok(this.bootstrapMethod.get())
    }

    // java: bootstrapArgs()[Ljava/lang/constant/ConstantDesc;
    pub fn bootstrapArgs(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.bootstrapArgs.get().clone()?;
        Ok(_t0)
    }

    // java: bootstrapArgsList()Ljava/util/List;
    pub fn bootstrapArgsList(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = List::of__arr_obj(&this.bootstrapArgs.get())?;
        Ok(_t0)
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Object;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<T> {
        let this = self;
        let _t0 = this.bootstrapMethod.get().resolveConstantDesc(lookup)?;
        let mut bsm: Object = _t0;
        let _t1 = bsm.type()?;
        let _t2 = _t1.parameterCount()?;
        let _t3 = bsm.type()?;
        let _t4 = _t3.parameterType(0i32)?;
        let _t5 = 124i32.isAssignableFrom(_t4)?;
        String::new().append(&String::from("Invalid bootstrap method declared for resolving a dynamic constant:"))?;
        String::new().append(&this.bootstrapMethod.get())?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut _arr6: Vec<Object> = Vec::with_capacity((3i32).wrapping_add((this.bootstrapArgs.get().len() as i32)) as usize);
        let mut bsmArgs: Vec<Object> = _arr6;
        bsmArgs[0i32 as usize] = lookup;
        bsmArgs[1i32 as usize] = this.constantName.get();
        let _t7 = this.constantType.get().resolveConstantDesc(lookup)?;
        bsmArgs[2i32 as usize] = _t7;
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.bootstrapArgs.get().len() as i32) { break; }
            let _t0 = this.bootstrapArgs.get()[i as usize].clone().resolveConstantDesc(lookup)?;
            bsmArgs[(3i32).wrapping_add(i) as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        let _t8 = bsm.invokeWithArguments(bsmArgs)?;
        return Ok(_t8);
        bsm = _t5;
        return Err(JvmError::Custom("athrow".to_owned()));
        bsm = 2i32;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: tryCanonicalize()Ljava/lang/constant/ConstantDesc;
    pub fn tryCanonicalize(&self) -> Result<Object> {
        let this = self;
        let _t0 = DynamicConstantDesc_CanonicalMapHolder::CANONICAL_MAP().get(this.bootstrapMethod.get())?;
        let mut f: Object = _t0;
        let _t1 = f.apply(this)?;
        return Ok(_t1);
        let mut t: Object = f;
        return Ok(this);
        Ok(this)
    }

    // java: canonicalizeNull(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeNull(desc: Object) -> Result<Object> {
        return Ok(desc);
        Ok(ConstantDescs::NULL())
    }

    // java: canonicalizeEnum(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeEnum(desc: Object) -> Result<Object> {
        return Ok(desc);
        let _t0: Object = Enum_EnumDesc::of(desc.constantType.get(), desc.constantName.get())?;
        Ok(_t0)
    }

    // java: canonicalizePrimitiveClass(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizePrimitiveClass(desc: Object) -> Result<Object> {
        let _t0 = desc.constantType()?;
        let _t1 = _t0.equals(ConstantDescs::CD_Class())?;
        return Ok(desc);
        let _t2: Object = ClassDesc::ofDescriptor(desc.constantName.get())?;
        Ok(_t2)
    }

    // java: canonicalizeStaticFieldVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeStaticFieldVarHandle(desc: Object) -> Result<Object> {
        let _t0 = desc.constantType()?;
        let _t1 = _t0.equals(ConstantDescs::CD_VarHandle())?;
        return Ok(desc);
        let _t2: Object = VarHandle_VarHandleDesc::ofStaticField(desc.bootstrapArgs.get()[0i32 as usize].clone(), desc.constantName.get(), desc.bootstrapArgs.get()[1i32 as usize].clone())?;
        Ok(_t2)
    }

    // java: canonicalizeFieldVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeFieldVarHandle(desc: Object) -> Result<Object> {
        let _t0 = desc.constantType()?;
        let _t1 = _t0.equals(ConstantDescs::CD_VarHandle())?;
        return Ok(desc);
        let _t2: Object = VarHandle_VarHandleDesc::ofField(desc.bootstrapArgs.get()[0i32 as usize].clone(), desc.constantName.get(), desc.bootstrapArgs.get()[1i32 as usize].clone())?;
        Ok(_t2)
    }

    // java: canonicalizeArrayVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeArrayVarHandle(desc: Object) -> Result<Object> {
        let _t0 = desc.constantType()?;
        let _t1 = _t0.equals(ConstantDescs::CD_VarHandle())?;
        return Ok(desc);
        let _t2: Object = VarHandle_VarHandleDesc::ofArray(desc.bootstrapArgs.get()[0i32 as usize].clone())?;
        Ok(_t2)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut desc: Object = o;
        let _t0: bool = Objects::equals(this.bootstrapMethod.get(), desc.bootstrapMethod.get())?;
        let _t1: bool = Arrays::equals__arr_obj_arr_obj(&this.bootstrapArgs.get(), &desc.bootstrapArgs.get())?;
        let _t2: bool = Objects::equals(this.constantName.get(), desc.constantName.get())?;
        let _t3: bool = Objects::equals(this.constantType.get(), desc.constantType.get())?;
        Ok(_t3!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(3i32 as usize);
        _arr0[0i32 as usize] = this.bootstrapMethod.get();
        _arr0[1i32 as usize] = this.constantName.get();
        _arr0[2i32 as usize] = this.constantType.get();
        let _t1: i32 = Objects::hash(&_arr0)?;
        let mut result: i32 = _t1;
        let _t2: i32 = Arrays::hashCode__arr_obj(&this.bootstrapArgs.get())?;
        result = ((31i32).wrapping_mul(result)).wrapping_add(_t2);
        Ok(result)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(5i32 as usize);
        let _t1 = this.bootstrapMethod.get().owner()?;
        let _t2 = _t1.displayName()?;
        _arr0[0i32 as usize] = _t2;
        let _t3 = this.bootstrapMethod.get().methodName()?;
        _arr0[1i32 as usize] = _t3;
        let _t4 = this.constantName.get().equals(String::from("_"))?;
        String::new().append(&this.constantName.get())?;
        String::new().append(&String::from("/"))?;
        _t4[String::from("") as usize] = String::new();
        let _t5: Object = Stream::of(&this.bootstrapArgs.get())?;
        /* TODO: invokedynamic 253 */
        let _t6 = 3i32.map(_t5)?;
        let _t7: Object = Collectors::joining(String::from(","))?;
        let _t8 = _t6.collect(_t7)?;
        2i32[2i32 as usize] = _t8;
        let _t9 = this.constantType.get().displayName()?;
        _arr0[4i32 as usize] = _t9;
        let _t10: String = String::format(&_arr0, &_arr0)?;
        Ok(_t10)
    }
}
