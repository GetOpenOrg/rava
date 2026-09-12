#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FileNotFoundException",
    super_class = "java/io/IOException",
    interfaces  = "",
    access      = "public",
    source      = "FileNotFoundException.java",
))]
pub struct FileNotFoundException;

impl FileNotFoundException {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/IOException.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/IOException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(path: String, reason: String) -> Result<Self> {
        let this = Self {};
        String::new().append(&path)?;
        String::new().append(&String::from("("))?;
        String::new().append(&reason)?;
        String::new().append(&String::from(")"))?;
        String::from("").append(&String::new())?;
        /* invokespecial Method java/io/IOException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }
}
