#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/System$Logger",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "System.java",
))]
pub struct System_Logger;

impl System_Logger {
    // java: getName()Ljava/lang/String;
    pub fn getName(&self) -> Result<String> {
        todo!("abstract java/lang/System$Logger.getName")
    }

    // java: isLoggable(Ljava/lang/System$Logger$Level;)Z
    pub fn isLoggable(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/lang/System$Logger.isLoggable")
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/String;)V
    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/String;)V
    pub fn log__system_str(&self, level: Object, msg: String) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        todo!("stack underflow").log(todo!("stack underflow"), this, level, msg)?;
        Ok(())
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/util/function/Supplier;)V
    // java: log(Ljava/lang/System$Logger$Level;Ljava/util/function/Supplier;)V
    pub fn log__system_suppli(&self, level: Object, msgSupplier: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(msgSupplier)?;
        let _t1: Object = Objects::requireNonNull(level)?;
        let _t2 = this.isLoggable(_t1)?;
        /* TODO: aconst_null  */
        let _t3 = msgSupplier.get()?;
        /* TODO: aconst_null  */
        todo!("stack underflow").log(_t2, this, level, _t3)?;
        Ok(())
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/Object;)V
    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/Object;)V
    pub fn log__system_obj(&self, level: Object, obj: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(obj)?;
        let _t1: Object = Objects::requireNonNull(level)?;
        let _t2 = this.isLoggable(_t1)?;
        /* TODO: aconst_null  */
        let _t3 = obj.toString()?;
        /* TODO: aconst_null  */
        todo!("stack underflow").log(_t2, this, level, _t3)?;
        Ok(())
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/String;Ljava/lang/Throwable;)V
    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn log__system_str_throwa(&self, level: Object, msg: String, thrown: Object) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").log(this, level, msg, thrown)?;
        Ok(())
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/util/function/Supplier;Ljava/lang/Throwable;)V
    // java: log(Ljava/lang/System$Logger$Level;Ljava/util/function/Supplier;Ljava/lang/Throwable;)V
    pub fn log__system_suppli_throwa(&self, level: Object, msgSupplier: Object, thrown: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(msgSupplier)?;
        let _t1: Object = Objects::requireNonNull(level)?;
        let _t2 = this.isLoggable(_t1)?;
        /* TODO: aconst_null  */
        let _t3 = msgSupplier.get()?;
        _t2.log(this, level, _t3, thrown)?;
        Ok(())
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/String;[Ljava/lang/Object;)V
    // java: log(Ljava/lang/System$Logger$Level;Ljava/lang/String;[Ljava/lang/Object;)V
    pub fn log__system_str_arr_obj(&self, level: Object, format: String, params: Vec<Object>) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").log(this, level, format, params)?;
        Ok(())
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/util/ResourceBundle;Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn log__system_resour_str_throwa(&self, arg0: Object, arg1: Object, arg2: String, arg3: Object) -> Result<()> {
        todo!("abstract java/lang/System$Logger.log")
    }

    // java: log(Ljava/lang/System$Logger$Level;Ljava/util/ResourceBundle;Ljava/lang/String;[Ljava/lang/Object;)V
    pub fn log__system_resour_str_arr_obj(&self, arg0: Object, arg1: Object, arg2: String, arg3: Vec<Object>) -> Result<()> {
        todo!("abstract java/lang/System$Logger.log")
    }
}
