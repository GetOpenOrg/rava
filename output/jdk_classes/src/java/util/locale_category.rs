#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Locale$Category",
    super_class = "java/lang/Enum",
    interfaces  = "",
    access      = "public final",
    source      = "Locale.java",
))]
pub struct Locale_Category {
    #[cfg_attr(any(), java_field(name = "languageKey", descriptor = "Ljava/lang/String;", access = "final"))]
    pub languageKey: Field<String>,
    #[cfg_attr(any(), java_field(name = "scriptKey", descriptor = "Ljava/lang/String;", access = "final"))]
    pub scriptKey: Field<String>,
    #[cfg_attr(any(), java_field(name = "countryKey", descriptor = "Ljava/lang/String;", access = "final"))]
    pub countryKey: Field<String>,
    #[cfg_attr(any(), java_field(name = "variantKey", descriptor = "Ljava/lang/String;", access = "final"))]
    pub variantKey: Field<String>,
    #[cfg_attr(any(), java_field(name = "extensionsKey", descriptor = "Ljava/lang/String;", access = "final"))]
    pub extensionsKey: Field<String>,
}

impl Locale_Category {
    // java: values()[Ljava/util/Locale$Category;
    pub fn values() -> Result<Vec<Object>> {
        let _t0 = Locale_Category::_VALUES().clone()?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/util/Locale$Category;
    pub fn valueOf(name: String) -> Result<Object> {
        let _t0: Object = Enum::valueOf(1i32, name)?;
        Ok(_t0)
    }

    // java: <init>(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
    pub fn new(arg_0: String, arg_1: i32, languageKey: String, scriptKey: String, countryKey: String, variantKey: String, extensionsKey: String) -> Result<Self> {
        let this = Self { languageKey: Field::new(String::new()), scriptKey: Field::new(String::new()), countryKey: Field::new(String::new()), variantKey: Field::new(String::new()), extensionsKey: Field::new(String::new()) };
        /* invokespecial Method java/lang/Enum.<init>:(Ljava/lang/String;I)V */
        this.languageKey.set(languageKey);
        this.scriptKey.set(scriptKey);
        this.countryKey.set(countryKey);
        this.variantKey.set(variantKey);
        this.extensionsKey.set(extensionsKey);
        Ok(this)
    }
}
