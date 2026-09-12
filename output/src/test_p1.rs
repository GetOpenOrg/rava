#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

// @java_class(name="TestP1", super="java/lang/Object", access="public", source="TestP1.java")
pub struct TestP1 {
// @java_field(name="x", descriptor="I", access="public")
    pub x: Field<i32>,
// @java_field(name="y", descriptor="I", access="public")
    pub y: Field<i32>,
}

impl TestP1 {
    // @java_method(name="<init>", descriptor="(II)V", access="public")
    pub fn new(x: i32, y: i32) -> Result<Self> {
        let this = Self { x: Field::new(0), y: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.x.set(x);
        this.y.set(y);
        Ok(this)
    }

    // @java_method(name="sum", descriptor="()I", access="public")
    pub fn sum(&self) -> Result<i32> {
        let this = self;
        Ok((this.x.get()).wrapping_add(this.y.get()))
    }

    // @java_method(name="setX", descriptor="(I)V", access="public")
    pub fn setX(&self, val: i32) -> Result<()> {
        let this = self;
        this.x.set(val);
        Ok(())
    }

    // @java_method(name="main", descriptor="([Ljava/lang/String;)V", access="public static")
    pub fn main() -> Result<()> {
        let p: TestP1 = TestP1::new(3i32, 4i32)?;
        let _t0: i32 = p.sum()?;
        System::out().println(_t0)?;
        p.setX(10i32)?;
        let _t1: i32 = p.sum()?;
        System::out().println(_t1)?;
        System::out().println(p.x.get())?;
        Ok(())
    }
}
