#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Thread$State",
    super_class       = "java/lang/Enum",
    interfaces        = "",
    access            = "public",
    modifiers         = "final enum",
    generic_signature = "Ljava/lang/Enum<Ljava/lang/Thread$State;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = true,
    is_deprecated     = false,
    source            = "Thread.java",
    inner_classes     = "java/lang/Thread$State:java/lang/Thread:State:16409",
    all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/Thread$State;java/lang/constant/Constable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Thread_State {
    pub _super: Enum<Object>,
}

impl Thread_State {
    pub fn as_enum_(&self) -> &Enum<Object> { &self._super }
    pub fn into_enum_(self) -> Enum<Object> { self._super }
}

impl From<Thread_State> for Enum<Object> {
    fn from(v: Thread_State) -> Enum<Object> { v._super }
}

impl Thread_State {
    #[cfg_attr(any(), java_field(name = "NEW", descriptor = "Ljava/lang/Thread$State;", access = "public", modifiers = "static final", is_static = true))]
    // static field: NEW:Ljava/lang/Thread$State;
    pub fn NEW() -> Thread_State {
        panic!("stub: java/lang/Thread$State.NEW:Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_field(name = "RUNNABLE", descriptor = "Ljava/lang/Thread$State;", access = "public", modifiers = "static final", is_static = true))]
    // static field: RUNNABLE:Ljava/lang/Thread$State;
    pub fn RUNNABLE() -> Thread_State {
        panic!("stub: java/lang/Thread$State.RUNNABLE:Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_field(name = "BLOCKED", descriptor = "Ljava/lang/Thread$State;", access = "public", modifiers = "static final", is_static = true))]
    // static field: BLOCKED:Ljava/lang/Thread$State;
    pub fn BLOCKED() -> Thread_State {
        panic!("stub: java/lang/Thread$State.BLOCKED:Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_field(name = "WAITING", descriptor = "Ljava/lang/Thread$State;", access = "public", modifiers = "static final", is_static = true))]
    // static field: WAITING:Ljava/lang/Thread$State;
    pub fn WAITING() -> Thread_State {
        panic!("stub: java/lang/Thread$State.WAITING:Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_field(name = "TIMED_WAITING", descriptor = "Ljava/lang/Thread$State;", access = "public", modifiers = "static final", is_static = true))]
    // static field: TIMED_WAITING:Ljava/lang/Thread$State;
    pub fn TIMED_WAITING() -> Thread_State {
        panic!("stub: java/lang/Thread$State.TIMED_WAITING:Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_field(name = "TERMINATED", descriptor = "Ljava/lang/Thread$State;", access = "public", modifiers = "static final", is_static = true))]
    // static field: TERMINATED:Ljava/lang/Thread$State;
    pub fn TERMINATED() -> Thread_State {
        panic!("stub: java/lang/Thread$State.TERMINATED:Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/lang/Thread$State;", access = "private", modifiers = "static final synthetic", is_static = true))]
    // static field: $VALUES:[Ljava/lang/Thread$State;
    pub fn _VALUES() -> Rc<RefCell<Vec<Thread_State>>> {
        panic!("stub: java/lang/Thread$State.$VALUES:[Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_method(name = "values", descriptor = "()[Ljava/lang/Thread$State;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn values() -> Result<Rc<RefCell<Vec<Thread_State>>>> {
        panic!("stub: java/lang/Thread$State.values:()[Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Thread$State;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768"))]
    pub fn valueOf(name: String) -> Result<Thread_State> {
        panic!("stub: java/lang/Thread$State.valueOf:(Ljava/lang/String;)Ljava/lang/Thread$State;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096"))]
    pub fn new(arg0: String, arg1: i32) -> Result<Self> {
        panic!("stub: java/lang/Thread$State.<init>:(Ljava/lang/String;I)V")
    }
}
