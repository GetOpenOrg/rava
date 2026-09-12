#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Formatter$FormatSpecifier",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Formatter$FormatString",
    access      = "",
    source      = "Formatter.java",
))]
pub struct Formatter_FormatSpecifier {
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "flags", descriptor = "I"))]
    pub flags: Field<i32>,
    #[cfg_attr(any(), java_field(name = "width", descriptor = "I"))]
    pub width: Field<i32>,
    #[cfg_attr(any(), java_field(name = "precision", descriptor = "I"))]
    pub precision: Field<i32>,
    #[cfg_attr(any(), java_field(name = "dt", descriptor = "Z"))]
    pub dt: Field<bool>,
    #[cfg_attr(any(), java_field(name = "c", descriptor = "C"))]
    pub c: Field<u16>,
}

impl Formatter_FormatSpecifier {
    #[cfg_attr(any(), java_method(name = "index", descriptor = "(Ljava/lang/String;II)V", access = "private"))]
    // java: index(Ljava/lang/String;II)V
    pub fn index__str_i_i(&self, s: String, start: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Integer::parseInt(s, start, (end).wrapping_sub(1i32), 10i32)?;
        this.index.set(_t0);
        return Err(JvmError::Custom(String::from("athrow")));
        let mut x: i32 = this.index.get();
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "index", descriptor = "()I", access = "public"))]
    // java: index()I
    pub fn index(&self) -> Result<i32> {
        let this = self;
        Ok(this.index.get())
    }

    #[cfg_attr(any(), java_method(name = "flags", descriptor = "(Ljava/lang/String;II)V", access = "private"))]
    pub fn flags(&self, s: String, start: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Formatter$Flags::parse(s, start, end)?;
        this.flags.set(_t0);
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 256i32)?;
        this.index.set(-1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "width", descriptor = "(Ljava/lang/String;II)V", access = "private"))]
    pub fn width(&self, s: String, start: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Integer::parseInt(s, start, end, 10i32)?;
        this.width.set(_t0);
        return Err(JvmError::Custom(String::from("athrow")));
        let mut x: i32 = this.width.get();
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "precision", descriptor = "(Ljava/lang/String;II)V", access = "private"))]
    pub fn precision(&self, s: String, start: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Integer::parseInt(s, (start).wrapping_add(1i32), end, 10i32)?;
        this.precision.set(_t0);
        return Err(JvmError::Custom(String::from("athrow")));
        let mut x: i32 = this.precision.get();
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "conversion", descriptor = "(C)V", access = "private"))]
    pub fn conversion(&self, conv: u16) -> Result<()> {
        let this = self;
        this.c.set(conv);
        let _t0: bool = Formatter$Conversion::isValid(this.c.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1: bool = Character::isUpperCase(this.c.get())?;
        let _t2: i32 = Formatter$Flags::add(this.flags.get(), 2i32)?;
        this.flags.set(_t2);
        let _t3: u16 = Character::toLowerCase(this.c.get())?;
        this.c.set(_t3);
        let _t4: bool = Formatter$Conversion::isText(this.c.get())?;
        this.index.set(-2i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(C)V"))]
    // java: <init>(C)V
    pub fn new__c(conv: u16) -> Result<Self> {
        let this = Self { index: Field::new(0), flags: Field::new(0), width: Field::new(0), precision: Field::new(0), dt: Field::new(false), c: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.index.set(0i32);
        this.flags.set(0i32);
        this.width.set(-1i32);
        this.precision.set(-1i32);
        this.dt.set(0i32);
        this.c.set(conv);
        let _t0: bool = Character::isUpperCase(conv)?;
        this.flags.set(2i32);
        let _t1: u16 = Character::toLowerCase(conv)?;
        this.c.set(_t1);
        let _t2: bool = Formatter$Conversion::isText(conv)?;
        this.index.set(-2i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/util/regex/Matcher;)V"))]
    // java: <init>(Ljava/lang/String;Ljava/util/regex/Matcher;)V
    pub fn new__str_matche(s: String, m: Object) -> Result<Self> {
        let this = Self { index: Field::new(0), flags: Field::new(0), width: Field::new(0), precision: Field::new(0), dt: Field::new(false), c: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.index.set(0i32);
        this.flags.set(0i32);
        this.width.set(-1i32);
        this.precision.set(-1i32);
        this.dt.set(0i32);
        let _t0 = m.start(1i32)?;
        let _t1 = m.end(1i32)?;
        this.index(s, _t0, _t1)?;
        let _t2 = m.start(2i32)?;
        let _t3 = m.end(2i32)?;
        this.flags(s, _t2, _t3)?;
        let _t4 = m.start(3i32)?;
        let _t5 = m.end(3i32)?;
        this.width(s, _t4, _t5)?;
        let _t6 = m.start(4i32)?;
        let _t7 = m.end(4i32)?;
        this.precision(s, _t6, _t7)?;
        let _t8 = m.start(5i32)?;
        let mut tTStart: i32 = _t8;
        this.dt.set(1i32);
        let _t9 = s.charAt(tTStart)?;
        let _t10: i32 = Formatter$Flags::add(this.flags.get(), 2i32)?;
        this.flags.set(_t10);
        let _t11 = m.start(6i32)?;
        let _t12 = s.charAt(_t11)?;
        this.conversion(_t12)?;
        this.checkDateTime()?;
        let _t13: bool = Formatter$Conversion::isGeneral(this.c.get())?;
        this.checkGeneral()?;
        let _t14: bool = Formatter$Conversion::isCharacter(this.c.get())?;
        this.checkCharacter()?;
        let _t15: bool = Formatter$Conversion::isInteger(this.c.get())?;
        this.checkInteger()?;
        let _t16: bool = Formatter$Conversion::isFloat(this.c.get())?;
        this.checkFloat()?;
        let _t17: bool = Formatter$Conversion::isText(this.c.get())?;
        this.checkText()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "public"))]
    // java: print(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V
    pub fn print__format_obj_locale(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        this.printDateTime(fmt, arg, l)?;
        return Ok(());
        /* TODO: lookupswitch default:219 37:208 97:142 98:162 99:152 100:132 101:142 102:142 103:142 104:182 110:192 111:132 115:172 120:132 */
        this.printInteger(fmt, arg, l)?;
        this.printFloat(fmt, arg, l)?;
        this.printCharacter(fmt, arg, l)?;
        this.printBoolean(fmt, arg, l)?;
        this.printString(fmt, arg, l)?;
        this.printHashCode(fmt, arg, l)?;
        let _t0: String = System::lineSeparator()?;
        let _t1 = fmt.a.get().append(_t0)?;
        this.print(fmt, String::from("%"), l)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printInteger", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private"))]
    pub fn printInteger(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        this.print(fmt, String::from("null"), l)?;
        this.print(fmt, arg, l)?;
        this.print(fmt, arg, l)?;
        this.print(fmt, arg, l)?;
        this.print(fmt, arg, l)?;
        this.print(fmt, arg, l)?;
        this.failConversion(this.c.get(), arg)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printFloat", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private"))]
    pub fn printFloat(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        this.print(fmt, String::from("null"), l)?;
        this.print(fmt, arg, l)?;
        this.print(fmt, arg, l)?;
        this.print(fmt, arg, l)?;
        this.failConversion(this.c.get(), arg)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printDateTime", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private"))]
    pub fn printDateTime(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        this.print(fmt, String::from("null"), l)?;
        return Ok(());
        /* TODO: aconst_null  */
        let mut cal: Object = arg;
        let _t0: Object = Calendar::getInstance(l)?;
        cal = _t0;
        cal.setTimeInMillis(arg)?;
        let _t1: Object = Calendar::getInstance(l)?;
        cal = _t1;
        cal.setTime(arg)?;
        let _t2 = arg.clone()?;
        cal = _t2;
        cal.setLenient(1i32)?;
        this.print(fmt, arg, this.c.get(), l)?;
        return Ok(());
        this.failConversion(this.c.get(), arg)?;
        this.print(fmt, cal, this.c.get(), l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printCharacter", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private"))]
    pub fn printCharacter(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        this.print(fmt, String::from("null"), l)?;
        return Ok(());
        /* TODO: aconst_null  */
        let mut s: Object = arg;
        let _t0 = arg.toString()?;
        s = _t0;
        let mut i: i32 = arg;
        let _t1: bool = Character::isValidCodePoint(i)?;
        let _t2: Vec<u16> = Character::toChars(i)?;
        s = String::new(_t2)?;
        return Err(JvmError::Custom(String::from("athrow")));
        i = arg;
        let _t3: bool = Character::isValidCodePoint(i)?;
        let _t4: Vec<u16> = Character::toChars(i)?;
        s = String::new(_t4)?;
        return Err(JvmError::Custom(String::from("athrow")));
        i = arg;
        let _t5: bool = Character::isValidCodePoint(i)?;
        let _t6: Vec<u16> = Character::toChars(i)?;
        s = String::new(_t6)?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.failConversion(this.c.get(), arg)?;
        this.print(fmt, s, l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printString", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private"))]
    pub fn printString(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        let _t0 = fmt.locale()?;
        let _t1 = fmt.out()?;
        fmt = Formatter::new(_t1, l)?;
        arg.formatTo(fmt, this.flags.get(), this.width.get(), this.precision.get())?;
        let _t2: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        this.failMismatch(4i32, 115i32)?;
        this.print(fmt, String::from("null"), l)?;
        let _t3 = arg.toString()?;
        this.print(fmt, _t3, l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printBoolean", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private"))]
    pub fn printBoolean(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        let _t0 = arg.toString()?;
        let _t1: String = Boolean::toString(1i32)?;
        let mut s: String = _t1;
        let _t2: String = Boolean::toString(0i32)?;
        s = _t2;
        this.print(fmt, s, l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printHashCode", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private"))]
    pub fn printHashCode(&self, fmt: Object, arg: Object, l: Object) -> Result<()> {
        let this = self;
        let _t0 = arg.hashCode()?;
        let _t1: String = Integer::toHexString(_t0)?;
        let mut s: String = _t1;
        this.print(fmt, s, l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/String;Ljava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/lang/String;Ljava/util/Locale;)V
    pub fn print__format_str_locale(&self, fmt: Object, s: String, l: Object) -> Result<()> {
        let this = self;
        let _t0 = s.length()?;
        let _t1 = s.substring(0i32, this.precision.get())?;
        s = _t1;
        let _t2: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        let _t3 = this.toUpperCaseWithLocale(s, l)?;
        s = _t3;
        this.appendJustified(fmt.a.get(), s)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "toUpperCaseWithLocale", descriptor = "(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/String;", access = "private"))]
    pub fn toUpperCaseWithLocale(&self, s: String, l: Object) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t1: Object = Objects::requireNonNullElse(l, _t0)?;
        let _t2 = s.toUpperCase(_t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "appendJustified", descriptor = "(Ljava/lang/Appendable;Ljava/lang/CharSequence;)V", access = "private"))]
    pub fn appendJustified(&self, a: Object, cs: Object) -> Result<()> {
        let this = self;
        let _t0 = a.append(cs)?;
        return Ok(());
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 1i32)?;
        let mut padRight: i32 = _t1;
        let _t2 = cs.length()?;
        let mut sp: i32 = (this.width.get()).wrapping_sub(_t2);
        let _t3 = a.append(cs)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= sp { break; }
            let _t0 = a.append(32i32)?;
            i = i.wrapping_add(1i32);
        }
        let _t4 = a.append(cs)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut sb: String = String::new();
        let _t0: i32 = Formatter$Flags::remove(this.flags.get(), 2i32)?;
        let _t1: String = Formatter$Flags::toString(_t0)?;
        sb.append(&_t1)?;
        sb.append(&this.index.get())?;
        sb.append(&36i32)?;
        sb.append(&this.width.get())?;
        sb.append(&46i32)?;
        sb.append(&this.precision.get())?;
        let _t2: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        84i32.append(&116i32)?;
        let _t3: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        let _t4: u16 = Character::toUpperCase(this.c.get())?;
        _t4.append(&this.c.get())?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "checkGeneral", descriptor = "()V", access = "private"))]
    pub fn checkGeneral(&self) -> Result<()> {
        let this = self;
        let _t0: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        this.failMismatch(4i32, this.c.get())?;
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 1i32)?;
        let _t2 = this.toString()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkBadFlags(248i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkDateTime", descriptor = "()V", access = "private"))]
    pub fn checkDateTime(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: bool = Formatter$DateTime::isValid(this.c.get())?;
        String::new().append(&String::from("t"))?;
        String::new().append(&this.c.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkBadFlags(252i32)?;
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 1i32)?;
        let _t2 = this.toString()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkCharacter", descriptor = "()V", access = "private"))]
    pub fn checkCharacter(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkBadFlags(252i32)?;
        let _t0: bool = Formatter$Flags::contains(this.flags.get(), 1i32)?;
        let _t1 = this.toString()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkInteger", descriptor = "()V", access = "private"))]
    pub fn checkInteger(&self) -> Result<()> {
        let this = self;
        this.checkNumeric()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkBadFlags(4i32)?;
        this.checkBadFlags(64i32)?;
        this.checkBadFlags(64i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkBadFlags", descriptor = "(I)V", access = "private"))]
    pub fn checkBadFlags(&self, badFlags: i32) -> Result<()> {
        let this = self;
        this.failMismatch((this.flags.get()&badFlags), this.c.get())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkFloat", descriptor = "()V", access = "private"))]
    pub fn checkFloat(&self) -> Result<()> {
        let this = self;
        this.checkNumeric()?;
        this.checkBadFlags(192i32)?;
        this.checkBadFlags(64i32)?;
        this.checkBadFlags(4i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkNumeric", descriptor = "()V", access = "private"))]
    pub fn checkNumeric(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: bool = Formatter$Flags::containsAny(this.flags.get(), 33i32)?;
        let _t1 = this.toString()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: bool = Formatter$Flags::contains(this.flags.get(), 24i32)?;
        let _t3: bool = Formatter$Flags::contains(this.flags.get(), 33i32)?;
        let _t4: String = Formatter$Flags::toString(this.flags.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkText", descriptor = "()V", access = "private"))]
    pub fn checkText(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lookupswitch default:155 37:52 110:113 */
        let _t0: String = Formatter$Flags::toString(this.flags.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 1i32)?;
        let _t2 = this.toString()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3: String = Formatter$Flags::toString(this.flags.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;BLjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;BLjava/util/Locale;)V
    pub fn print__format_b_locale(&self, fmt: Object, value: i8, l: Object) -> Result<()> {
        let this = self;
        let mut v: i64 = (value as i64);
        v = (v).wrapping_add(256i64);
        this.print(fmt, v, l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;SLjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;SLjava/util/Locale;)V
    pub fn print__format_s_locale(&self, fmt: Object, value: i16, l: Object) -> Result<()> {
        let this = self;
        let mut v: i64 = (value as i64);
        v = (v).wrapping_add(65536i64);
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.print(fmt, v, l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;ILjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;ILjava/util/Locale;)V
    pub fn print__format_i_locale(&self, fmt: Object, value: i32, l: Object) -> Result<()> {
        let this = self;
        let mut v: i64 = (value as i64);
        v = (v).wrapping_add(4294967296i64);
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.print(fmt, v, l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;JLjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;JLjava/util/Locale;)V
    pub fn print__format_l_locale(&self, fmt: Object, value: i64, arg_2: Object) -> Result<()> {
        let this = self;
        let mut sb: String = String::new();
        /* TODO: lcmp  */
        let mut neg: i32 = 0i64<0i32;
        let _t0: String = Long::toString(value, 10i32)?;
        let mut valueStr: String = _t0;
        let _t1 = this.leadingSign(sb, neg)?;
        let _t2 = this.adjustWidth(this.width.get(), this.flags.get(), neg)?;
        let _t3 = this.localizedMagnitude(fmt, sb, valueStr, neg!=0i32, this.flags.get(), _t2, local_4)?;
        let _t4 = this.trailingSign(sb, neg)?;
        this.checkBadFlags(152i32)?;
        let _t5: String = Long::toOctalString(value)?;
        neg = _t5;
        let _t6: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        let _t7 = neg.length()?;
        let _t8 = neg.length()?;
        valueStr = _t8;
        let _t9: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        sb.append(&48i32)?;
        let _t10: bool = Formatter$Flags::contains(this.flags.get(), 32i32)?;
        this.trailingZeros(sb, (this.width.get()).wrapping_sub(valueStr))?;
        sb.append(&neg)?;
        this.checkBadFlags(152i32)?;
        let _t11: String = Long::toHexString(value)?;
        neg = _t11;
        let _t12: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        let _t13 = neg.length()?;
        let _t14 = neg.length()?;
        valueStr = _t14;
        let _t15: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        let _t16: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        String::from("0X").append(&String::from("0x"))?;
        let _ = String::from("0X");
        let _t17: bool = Formatter$Flags::contains(this.flags.get(), 32i32)?;
        this.trailingZeros(sb, (this.width.get()).wrapping_sub(valueStr))?;
        let _t18: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        let _t19 = this.toUpperCaseWithLocale(neg, local_4)?;
        neg = _t19;
        sb.append(&neg)?;
        this.appendJustified(fmt.a.get(), sb)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "leadingSign", descriptor = "(Ljava/lang/StringBuilder;Z)Ljava/lang/StringBuilder;", access = "private"))]
    pub fn leadingSign(&self, sb: Object, neg: bool) -> Result<Object> {
        let this = self;
        let _t0: bool = Formatter$Flags::contains(this.flags.get(), 8i32)?;
        sb.append(&43i32)?;
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 16i32)?;
        sb.append(&32i32)?;
        let _t2: bool = Formatter$Flags::contains(this.flags.get(), 128i32)?;
        sb.append(&40i32)?;
        sb.append(&45i32)?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "trailingSign", descriptor = "(Ljava/lang/StringBuilder;Z)Ljava/lang/StringBuilder;", access = "private"))]
    pub fn trailingSign(&self, sb: Object, neg: bool) -> Result<Object> {
        let this = self;
        let _t0: bool = Formatter$Flags::contains(this.flags.get(), 128i32)?;
        sb.append(&41i32)?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/math/BigInteger;Ljava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/math/BigInteger;Ljava/util/Locale;)V
    pub fn print__format_bigint_locale(&self, fmt: Object, value: Object, l: Object) -> Result<()> {
        let this = self;
        let mut sb: String = String::new();
        let _t0 = value.signum()?;
        let mut neg: i32 = _t0 == -1i32;
        let _t1 = value.abs()?;
        let mut v: Object = _t1;
        let _t2 = this.leadingSign(sb, neg)?;
        let _t3 = v.toString()?;
        let _t4 = this.adjustWidth(this.width.get(), this.flags.get(), neg)?;
        let _t5 = this.localizedMagnitude(fmt, sb, _t3, 0i32, this.flags.get(), _t4, l)?;
        let _t6 = v.toString(8i32)?;
        let mut s: String = _t6;
        let _t7 = s.length()?;
        let _t8 = sb.length()?;
        let mut len: i32 = (_t7).wrapping_add(_t8);
        let _t9: bool = Formatter$Flags::contains(this.flags.get(), 128i32)?;
        len = len.wrapping_add(1i32);
        let _t10: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        len = len.wrapping_add(1i32);
        sb.append(&48i32)?;
        let _t11: bool = Formatter$Flags::contains(this.flags.get(), 32i32)?;
        this.trailingZeros(sb, (this.width.get()).wrapping_sub(len))?;
        sb.append(&s)?;
        let _t12 = v.toString(16i32)?;
        s = _t12;
        let _t13 = s.length()?;
        let _t14 = sb.length()?;
        len = (_t13).wrapping_add(_t14);
        let _t15: bool = Formatter$Flags::contains(this.flags.get(), 128i32)?;
        len = len.wrapping_add(1i32);
        let _t16: bool = Formatter$Flags::contains(this.flags.get(), 4i32)?;
        len = len.wrapping_add(2i32);
        let _t17: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        String::from("0X").append(&String::from("0x"))?;
        let _ = String::from("0X");
        let _t18: bool = Formatter$Flags::contains(this.flags.get(), 32i32)?;
        this.trailingZeros(sb, (this.width.get()).wrapping_sub(len))?;
        let _t19: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        let _t20 = this.toUpperCaseWithLocale(s, l)?;
        s = _t20;
        sb.append(&s)?;
        let _t21 = value.signum()?;
        let _t22 = this.trailingSign(sb, _t21 == -1i32)?;
        this.appendJustified(fmt.a.get(), sb)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;FLjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;FLjava/util/Locale;)V
    pub fn print__format_f_locale(&self, fmt: Object, value: f32, l: Object) -> Result<()> {
        let this = self;
        this.print(fmt, (value as f64), l)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;DLjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;DLjava/util/Locale;)V
    pub fn print__format_d_locale(&self, fmt: Object, value: f64, arg_2: Object) -> Result<()> {
        let this = self;
        let mut sb: String = String::new();
        let _t0: i32 = Double::compare(value, 0f64)?;
        let mut neg: i32 = _t0 == -1i32;
        let _t1: bool = Double::isNaN(value)?;
        let _t2: f64 = (value).abs();
        let mut v: f64 = _t2;
        let _t3 = this.leadingSign(sb, neg)?;
        let _t4: bool = Double::isInfinite(v)?;
        this.print(fmt, sb, v, local_4, this.flags.get(), this.c.get(), this.precision.get(), neg)?;
        let _t5: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        String::from("INFINITY").append(&String::from("Infinity"))?;
        let _ = String::from("INFINITY");
        let _t6 = this.trailingSign(sb, neg)?;
        let _t7: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        String::from("NAN").append(&String::from("NaN"))?;
        let _ = String::from("NAN");
        this.appendJustified(fmt.a.get(), sb)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;DLjava/util/Locale;ICIZ)V", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;DLjava/util/Locale;ICIZ)V
    pub fn print__format_sb_d_locale_i_c_i_z(&self, fmt: Object, sb: Object, value: f64, arg_3: Object, l: i32, flags: u16, c: i32, precision: bool) -> Result<()> {
        let this = self;
        let mut prec: i32 = precision;
        let _t0: Object = FormattedFPDecimal::valueOf(value, prec, 101i32)?;
        let mut fd: Object = _t0;
        let _t1 = fd.getMantissa()?;
        String::new().append(&_t1)?;
        let mut mant: String = String::new();
        this.addZeros(mant, prec)?;
        let _t2: bool = Formatter$Flags::contains(flags, 4i32)?;
        mant.append(&46i32)?;
        /* TODO: dcmpl  */
        let mut _arr3: Vec<u16> = vec![0u16; 3i32 as usize];
        _arr3[0i32 as usize] = 43i32;
        _arr3[1i32 as usize] = 48i32;
        _arr3[2i32 as usize] = 48i32;
        let _t4 = fd.getExponent()?;
        let mut exp: Vec<u16> = _t4;
        let mut newW: i32 = this.width.get();
        let _t5 = this.adjustWidth(((this.width.get()).wrapping_sub((exp.len() as i32))).wrapping_sub(1i32), flags, local_9)?;
        newW = _t5;
        let _t6 = this.localizedMagnitude(fmt, sb, mant, 0i32, flags, newW, l)?;
        let _t7: bool = Formatter$Flags::contains(flags, 2i32)?;
        69i32.append(&101i32)?;
        let mut sign: i32 = exp[0i32 as usize];
        return Err(JvmError::Custom(String::from("athrow")));
        sb.append(&sign)?;
        this.localizedMagnitudeExp(fmt, sb, exp, 1i32, l)?;
        prec = precision;
        let _t8: Object = FormattedFPDecimal::valueOf(value, prec, 102i32)?;
        fd = _t8;
        let _t9 = fd.getMantissa()?;
        String::new().append(&_t9)?;
        mant = String::new();
        this.addZeros(mant, prec)?;
        let _t10: bool = Formatter$Flags::contains(flags, 4i32)?;
        mant.append(&46i32)?;
        exp = this.width.get();
        let _t11 = this.adjustWidth(this.width.get(), flags, local_9)?;
        exp = _t11;
        let _t12 = this.localizedMagnitude(fmt, sb, mant, 0i32, flags, exp, l)?;
        prec = precision;
        prec = 6i32;
        prec = 1i32;
        mant = String::new();
        /* TODO: dcmpl  */
        /* TODO: aconst_null  */
        fd = 0f64;
        mant.append(&48i32)?;
        exp = 0i32;
        let _t13: Object = FormattedFPDecimal::valueOf(value, prec, 103i32)?;
        newW = _t13;
        let _t14 = newW.getExponent()?;
        fd = _t14;
        let _t15 = newW.getMantissa()?;
        mant.append(&_t15)?;
        let _t16 = newW.getExponentRounded()?;
        exp = _t16;
        prec = prec.wrapping_sub(1i32);
        prec = (prec).wrapping_sub((exp).wrapping_add(1i32));
        this.addZeros(mant, prec)?;
        let _t17: bool = Formatter$Flags::contains(flags, 4i32)?;
        mant.append(&46i32)?;
        newW = this.width.get();
        let _t18 = this.adjustWidth(((this.width.get()).wrapping_sub((fd.len() as i32))).wrapping_sub(1i32), flags, local_9)?;
        newW = _t18;
        let _t19 = this.adjustWidth(this.width.get(), flags, local_9)?;
        newW = _t19;
        let _t20 = this.localizedMagnitude(fmt, sb, mant, 0i32, flags, newW, l)?;
        let _t21: bool = Formatter$Flags::contains(flags, 2i32)?;
        69i32.append(&101i32)?;
        sign = fd[0i32 as usize];
        return Err(JvmError::Custom(String::from("athrow")));
        sb.append(&sign)?;
        this.localizedMagnitudeExp(fmt, sb, fd, 1i32, l)?;
        prec = precision;
        prec = 0i32;
        prec = 1i32;
        let _t22 = this.hexDouble(value, prec)?;
        fd = _t22;
        mant = String::new();
        let _t23: bool = Formatter$Flags::contains(flags, 2i32)?;
        exp = _t23;
        String::from("0X").append(&String::from("0x"))?;
        let _ = String::from("0X");
        let _t24: bool = Formatter$Flags::contains(flags, 32i32)?;
        newW = 2i32;
        let _t25: bool = Formatter$Flags::contains(flags, 16i32)?;
        let _t26: bool = Formatter$Flags::contains(flags, 8i32)?;
        newW = 3i32;
        let _t27 = fd.length()?;
        this.trailingZeros(sb, ((this.width.get()).wrapping_sub(_t27)).wrapping_sub(newW))?;
        let _t28 = fd.indexOf(112i32)?;
        newW = _t28;
        let _t29 = fd.substring(0i32, newW)?;
        sign = _t29;
        let _t30 = sign.toUpperCase(Locale::ROOT())?;
        sign = _t30;
        mant.append(&sign)?;
        mant.append(&fd)?;
        this.addZeros(mant, prec)?;
        sb.append(&mant)?;
        80i32.append(&112i32)?;
        let _t31 = fd.length()?;
        sb.append(&fd)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addZeros", descriptor = "(Ljava/lang/StringBuilder;I)V", access = "private"))]
    pub fn addZeros(&self, sb: Object, prec: i32) -> Result<()> {
        let this = self;
        let _t0 = sb.length()?;
        let mut len: i32 = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0 = sb.charAt(i)?;
            i = i.wrapping_add(1i32);
        }
        let mut needDot: i32 = 0i32;
        needDot = 1i32;
        let mut outPrec: i32 = ((len).wrapping_sub(i)).wrapping_sub(needDot==0i32);
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(());
        sb.append(&46i32)?;
        this.trailingZeros(sb, (prec).wrapping_sub(outPrec))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "hexDouble", descriptor = "(DI)Ljava/lang/String;", access = "private"))]
    pub fn hexDouble(&self, d: f64, arg_1: i32) -> Result<String> {
        let this = self;
        let _t0: bool = Double::isFinite(d)?;
        /* TODO: dcmpl  */
        let _t1: String = Double::toHexString(d)?;
        let _t2 = _t1.substring(2i32)?;
        return Ok(_t2);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3: i32 = (d).abs();
        let mut exponent: i32 = _t3;
        let mut subnormal: i32 = exponent == -1023i32;
        d = (d*Formatter$FormatSpecifier::SCALEUP());
        let _t4: i32 = (d).abs();
        exponent = _t4;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut precision: i32 = (1i32).wrapping_add((local_3).wrapping_mul(4i32));
        let mut shiftDistance: i32 = (53i32).wrapping_sub(precision);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5: i64 = Double::doubleToLongBits(d)?;
        let mut doppel: i64 = _t5;
        /* TODO: land  */
        /* TODO: lshr  */
        let mut newSignif: i64 = shiftDistance;
        /* TODO: lshl  */
        /* TODO: lxor  */
        /* TODO: land  */
        let mut roundingBits: i64 = 18446744073709551615i64;
        /* TODO: land  */
        /* TODO: lcmp  */
        let mut leastZero: i32 = 0i64==0i32;
        /* TODO: lshl  */
        /* TODO: land  */
        /* TODO: lcmp  */
        let mut round: i32 = 0i64!=0i32;
        /* TODO: lshl  */
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: lcmp  */
        let mut sticky: i32 = 0i64!=0i32;
        newSignif = (newSignif).wrapping_add(1i64);
        /* TODO: land  */
        let mut signBit: i64 = 9223372036854775808i64;
        /* TODO: lshl  */
        /* TODO: lor  */
        newSignif = shiftDistance;
        let _t6: f64 = Double::longBitsToDouble(newSignif)?;
        let mut result: f64 = _t6;
        let _t7: bool = Double::isInfinite(result)?;
        return Ok(String::from("1.0p1024"));
        let _t8: String = Double::toHexString(result)?;
        let _t9 = _t8.substring(2i32)?;
        let mut res: String = _t9;
        return Ok(res);
        let _t10 = res.indexOf(112i32)?;
        let mut idx: i32 = _t10;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        return Ok(Formatter$FormatSpecifier::$assertionsDisabled());
        let _t11 = res.substring((idx).wrapping_add(1i32))?;
        let mut exp: String = _t11;
        let _t12: i32 = Integer::parseInt(exp)?;
        let mut iexp: i32 = (_t12).wrapping_sub(54i32);
        let _t13 = res.substring(0i32, idx)?;
        String::new().append(&_t13)?;
        String::new().append(&String::from("p"))?;
        let _t14: String = Integer::toString(iexp)?;
        String::new().append(&_t14)?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/math/BigDecimal;Ljava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/math/BigDecimal;Ljava/util/Locale;)V
    pub fn print__format_bigdec_locale(&self, fmt: Object, value: Object, l: Object) -> Result<()> {
        let this = self;
        this.failConversion(this.c.get(), value)?;
        let mut sb: String = String::new();
        let _t0 = value.signum()?;
        let mut neg: i32 = _t0 == -1i32;
        let _t1 = value.abs()?;
        let mut v: Object = _t1;
        let _t2 = this.leadingSign(sb, neg)?;
        this.print(fmt, sb, v, l, this.flags.get(), this.c.get(), this.precision.get(), neg)?;
        let _t3 = this.trailingSign(sb, neg)?;
        this.appendJustified(fmt.a.get(), sb)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/math/BigDecimal;Ljava/util/Locale;ICIZ)V", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/math/BigDecimal;Ljava/util/Locale;ICIZ)V
    pub fn print__format_sb_bigdec_locale_i_c_i_z(&self, fmt: Object, sb: Object, value: Object, l: Object, flags: i32, c: u16, precision: i32, neg: bool) -> Result<()> {
        let this = self;
        let mut prec: i32 = precision;
        let _t0 = value.scale()?;
        let mut scale: i32 = _t0;
        let _t1 = value.precision()?;
        let mut origPrec: i32 = _t1;
        let mut nzeros: i32 = 0i32;
        let mut compPrec: i32 = origPrec;
        nzeros = (prec).wrapping_sub((origPrec).wrapping_sub(1i32));
        compPrec = (prec).wrapping_add(1i32);
        let mut mc: MathContext = MathContext::new(compPrec)?;
        let _t2 = value.unscaledValue()?;
        let mut v: BigDecimal = BigDecimal::new(_t2, scale, mc)?;
        let _t3 = v.unscaledValue()?;
        let _t4 = v.scale()?;
        let mut bdl: Formatter_FormatSpecifier_BigDecimalLayout = Formatter_FormatSpecifier_BigDecimalLayout::new(this, _t3, _t4, Formatter$BigDecimalLayoutForm::SCIENTIFIC())?;
        let _t5 = bdl.mantissa()?;
        let mut mant: Object = _t5;
        let _t6 = bdl.hasDot()?;
        let _t7: bool = Formatter$Flags::contains(flags, 4i32)?;
        mant.append(&46i32)?;
        this.trailingZeros(mant, nzeros)?;
        let _t8 = bdl.exponent()?;
        let mut exp: Object = _t8;
        let mut newW: i32 = this.width.get();
        let _t9 = exp.length()?;
        let _t10 = this.adjustWidth(((this.width.get()).wrapping_sub(_t9)).wrapping_sub(1i32), flags, neg)?;
        newW = _t10;
        let _t11 = this.localizedMagnitude(fmt, sb, mant, 0i32, flags, newW, l)?;
        let _t12: bool = Formatter$Flags::contains(flags, 2i32)?;
        69i32.append(&101i32)?;
        let _t13: i32 = Formatter$Flags::remove(flags, 64i32)?;
        let mut adaptedFlags: i32 = _t13;
        let _t14 = exp.charAt(0i32)?;
        let mut sign: i32 = _t14;
        return Err(JvmError::Custom(String::from("athrow")));
        sb.append(&sign)?;
        /* TODO: aconst_null  */
        let _t15 = sb.localizedMagnitude(this, fmt, exp, 1i32, adaptedFlags, -1i32, l)?;
        45i32.append(&_t15)?;
        prec = precision;
        let _t16 = value.scale()?;
        scale = _t16;
        let _t17 = value.precision()?;
        origPrec = _t17;
        let _t18 = value.setScale(prec, RoundingMode::HALF_UP())?;
        value = _t18;
        origPrec = (origPrec).wrapping_sub((scale).wrapping_sub(prec));
        let _t19 = value.unscaledValue()?;
        value = BigDecimal::new(_t19, scale, MathContext::new(origPrec)?)?;
        let _t20 = value.unscaledValue()?;
        let _t21 = value.scale()?;
        origPrec = Formatter_FormatSpecifier_BigDecimalLayout::new(this, _t20, _t21, Formatter$BigDecimalLayoutForm::DECIMAL_FLOAT())?;
        let _t22 = origPrec.mantissa()?;
        nzeros = _t22;
        let _t23 = origPrec.scale()?;
        let _t24 = origPrec.scale()?;
        compPrec = 0i32;
        let _t25 = origPrec.scale()?;
        let _t26: bool = Formatter$Flags::contains(flags, 4i32)?;
        nzeros.append(&46i32)?;
        this.trailingZeros(nzeros, compPrec)?;
        let _t27 = this.adjustWidth(this.width.get(), flags, neg)?;
        let _t28 = this.localizedMagnitude(fmt, sb, nzeros, 0i32, flags, _t27, l)?;
        prec = precision;
        prec = 6i32;
        prec = 1i32;
        let _t29 = value.round(MathContext::new(prec)?)?;
        value = _t29;
        let _t30 = value.equals(BigDecimal::ZERO())?;
        let _t31: Object = BigDecimal::valueOf(1i64, 4i32)?;
        let _t32 = value.compareTo(_t31)?;
        let _t33: Object = BigDecimal::valueOf(1i64, (prec).wrapping_neg())?;
        let _t34 = value.compareTo(_t33)?;
        let _t35 = value.scale()?;
        let _t36 = value.unscaledValue()?;
        let _t37 = _t36.toString()?;
        let _t38 = _t37.length()?;
        scale = ((_t35).wrapping_neg()).wrapping_add((_t38).wrapping_sub(1i32));
        prec = ((prec).wrapping_sub(scale)).wrapping_sub(1i32);
        this.print(fmt, sb, value, l, flags, 102i32, prec, neg)?;
        this.print(fmt, sb, value, l, flags, 101i32, (prec).wrapping_sub(1i32), neg)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "adjustWidth", descriptor = "(IIZ)I", access = "private"))]
    pub fn adjustWidth(&self, width: i32, flags: i32, neg: bool) -> Result<i32> {
        let this = self;
        let mut newW: i32 = width;
        let _t0: bool = Formatter$Flags::contains(flags, 128i32)?;
        newW = newW.wrapping_sub(1i32);
        Ok(newW)
    }

    #[cfg_attr(any(), java_method(name = "trailingZeros", descriptor = "(Ljava/lang/StringBuilder;I)V", access = "private"))]
    pub fn trailingZeros(&self, sb: Object, nzeros: i32) -> Result<()> {
        let this = self;
        let mut i: i32 = 0i32;
        loop {
            if i >= nzeros { break; }
            sb.append(&48i32)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/util/Calendar;CLjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/util/Calendar;CLjava/util/Locale;)V
    pub fn print__format_calend_c_locale(&self, fmt: Object, t: Object, c: u16, l: Object) -> Result<()> {
        let this = self;
        let mut sb: String = String::new();
        let _t0 = this.print(fmt, sb, t, c, l)?;
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        let _t2 = this.toUpperCaseWithLocale(sb, l)?;
        this.appendJustified(fmt.a.get(), _t2)?;
        this.appendJustified(fmt.a.get(), sb)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/util/Calendar;CLjava/util/Locale;)Ljava/lang/Appendable;", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/util/Calendar;CLjava/util/Locale;)Ljava/lang/Appendable;
    pub fn print__format_sb_calend_c_locale(&self, fmt: Object, sb: Object, t: Object, c: u16, l: Object) -> Result<Object> {
        let this = self;
        sb = String::new();
        /* TODO: tableswitch default:1489 low:65 high:122 */
        let _t0 = t.get(11i32)?;
        let mut i: i32 = _t0;
        i = (i%12i32);
        let mut flags: i32 = 0i32;
        /* TODO: aconst_null  */
        let _t1 = sb.localizedMagnitude(this, fmt, (i as i64), flags, 2i32, l)?;
        32i32.append(&_t1)?;
        let _t2 = t.get(12i32)?;
        i = _t2;
        /* TODO: aconst_null  */
        let _t3 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        73i32.append(&_t3)?;
        let _t4 = t.get(14i32)?;
        i = (_t4).wrapping_mul(686i32);
        /* TODO: aconst_null  */
        let _t5 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 9i32, l)?;
        c.append(&_t5)?;
        let _t6 = t.get(14i32)?;
        i = _t6;
        /* TODO: aconst_null  */
        let _t7 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 3i32, l)?;
        72i32.append(&_t7)?;
        let _t8 = t.getTimeInMillis()?;
        i = _t8;
        /* TODO: aconst_null  */
        let _t9 = sb.localizedMagnitude(this, fmt, i, 0i32, this.width.get(), l)?;
        c.append(&_t9)?;
        let mut _arr10: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr10[0i32 as usize] = String::from("AM");
        _arr10[1i32 as usize] = String::from("PM");
        i = _arr10;
        let _t11: Object = DateFormatSymbols::getInstance(l)?;
        flags = _t11;
        let _t12 = flags.getAmPmStrings()?;
        i = _t12;
        let _t13 = t.get(9i32)?;
        flags = i[_t13 as usize].clone();
        let _t14: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t15: Object = Objects::requireNonNullElse(l, _t14)?;
        let _t16 = flags.toLowerCase(_t15)?;
        sb.append(&_t16)?;
        let _t17 = t.getTimeInMillis()?;
        i = (_t17/1000i64);
        /* TODO: aconst_null  */
        let _t18 = sb.localizedMagnitude(this, fmt, i, 0i32, this.width.get(), l)?;
        Locale::US().append(&_t18)?;
        let _ = Locale::US();
        let _t19 = t.get(13i32)?;
        i = _t19;
        /* TODO: aconst_null  */
        let _t20 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        l.append(&_t20)?;
        let _t21 = t.get(15i32)?;
        let _t22 = t.get(16i32)?;
        i = (_t21).wrapping_add(_t22);
        flags = i<0i32;
        45i32.append(&43i32)?;
        i = (i).wrapping_neg();
        let mut min: i32 = (i/707i32);
        let mut offset: i32 = (((min/60i32)).wrapping_mul(100i32)).wrapping_add((min%60i32));
        /* TODO: aconst_null  */
        let _t23 = sb.localizedMagnitude(this, fmt, (offset as i64), 32i32, 4i32, l)?;
        flags.append(&_t23)?;
        let _t24 = t.getTimeZone()?;
        i = _t24;
        let _t25 = t.get(16i32)?;
        let _t26: Object = Objects::requireNonNullElse(l, Locale::US())?;
        let _t27 = i.getDisplayName(_t25!=0i32, 0i32, _t26)?;
        sb.append(&_t27)?;
        let _t28 = t.get(7i32)?;
        i = _t28;
        let _t29: Object = Objects::requireNonNullElse(l, Locale::US())?;
        flags = _t29;
        let _t30: Object = DateFormatSymbols::getInstance(flags)?;
        min = _t30;
        let _t31 = min.getWeekdays()?;
        sb.append(&_t31[i as usize].clone())?;
        let _t32 = min.getShortWeekdays()?;
        sb.append(&_t32[i as usize].clone())?;
        let _t33 = t.get(2i32)?;
        i = _t33;
        let _t34: Object = Objects::requireNonNullElse(l, Locale::US())?;
        flags = _t34;
        let _t35: Object = DateFormatSymbols::getInstance(flags)?;
        min = _t35;
        let _t36 = min.getMonths()?;
        sb.append(&_t36[i as usize].clone())?;
        let _t37 = min.getShortMonths()?;
        sb.append(&_t37[i as usize].clone())?;
        let _t38 = t.get(1i32)?;
        i = _t38;
        flags = 2i32;
        /* TODO: lookupswitch default:967 67:944 89:964 121:954 */
        i = (i/100i32);
        i = (i%100i32);
        flags = 4i32;
        /* TODO: aconst_null  */
        let _t39 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, flags, l)?;
        c.append(&_t39)?;
        let _t40 = t.get(5i32)?;
        i = _t40;
        flags = 0i32;
        /* TODO: aconst_null  */
        let _t41 = sb.localizedMagnitude(this, fmt, (i as i64), flags, 2i32, l)?;
        32i32.append(&_t41)?;
        let _t42 = t.get(6i32)?;
        i = _t42;
        /* TODO: aconst_null  */
        let _t43 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 3i32, l)?;
        100i32.append(&_t43)?;
        let _t44 = t.get(2i32)?;
        i = (_t44).wrapping_add(1i32);
        /* TODO: aconst_null  */
        let _t45 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        c.append(&_t45)?;
        i = 58i32;
        let _t46 = this.print(fmt, sb, t, 72i32, l)?;
        let _t47 = _t46.append(i)?;
        let _t48 = this.print(fmt, sb, t, 77i32, l)?;
        sb.append(&i)?;
        let _t49 = this.print(fmt, sb, t, 83i32, l)?;
        i = 58i32;
        let _t50 = this.print(fmt, sb, t, 73i32, l)?;
        let _t51 = _t50.append(i)?;
        let _t52 = this.print(fmt, sb, t, 77i32, l)?;
        let _t53 = _t52.append(i)?;
        let _t54 = this.print(fmt, sb, t, 83i32, l)?;
        let _t55 = _t54.append(32i32)?;
        flags = String::new();
        let _t56 = this.print(fmt, flags, t, 112i32, l)?;
        let _t57 = this.toUpperCaseWithLocale(flags, l)?;
        sb.append(&_t57)?;
        i = 32i32;
        let _t58 = this.print(fmt, sb, t, 97i32, l)?;
        let _t59 = _t58.append(i)?;
        let _t60 = this.print(fmt, sb, t, 98i32, l)?;
        let _t61 = _t60.append(i)?;
        let _t62 = this.print(fmt, sb, t, 100i32, l)?;
        let _t63 = _t62.append(i)?;
        let _t64 = this.print(fmt, sb, t, 84i32, l)?;
        let _t65 = _t64.append(i)?;
        let _t66 = this.print(fmt, sb, t, 90i32, l)?;
        let _t67 = _t66.append(i)?;
        let _t68 = this.print(fmt, sb, t, 89i32, l)?;
        i = 47i32;
        let _t69 = this.print(fmt, sb, t, 109i32, l)?;
        let _t70 = _t69.append(i)?;
        let _t71 = this.print(fmt, sb, t, 100i32, l)?;
        let _t72 = _t71.append(i)?;
        let _t73 = this.print(fmt, sb, t, 121i32, l)?;
        i = 45i32;
        let _t74 = this.print(fmt, sb, t, 89i32, l)?;
        let _t75 = _t74.append(i)?;
        let _t76 = this.print(fmt, sb, t, 109i32, l)?;
        let _t77 = _t76.append(i)?;
        let _t78 = this.print(fmt, sb, t, 100i32, l)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)V", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)V
    pub fn print__format_tempor_c_locale(&self, fmt: Object, t: Object, c: u16, l: Object) -> Result<()> {
        let this = self;
        let mut sb: String = String::new();
        let _t0 = this.print(fmt, sb, t, c, l)?;
        let _t1: bool = Formatter$Flags::contains(this.flags.get(), 2i32)?;
        let _t2 = this.toUpperCaseWithLocale(sb, l)?;
        this.appendJustified(fmt.a.get(), _t2)?;
        this.appendJustified(fmt.a.get(), sb)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)Ljava/lang/Appendable;", access = "private"))]
    // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)Ljava/lang/Appendable;
    pub fn print__format_sb_tempor_c_locale(&self, fmt: Object, sb: Object, t: Object, c: u16, l: Object) -> Result<Object> {
        let this = self;
        sb = String::new();
        /* TODO: tableswitch default:1674 low:65 high:122 */
        let _t0 = t.get(ChronoField::HOUR_OF_DAY())?;
        let mut i: i32 = _t0;
        /* TODO: aconst_null  */
        let _t1 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        c.append(&_t1)?;
        let _t2 = t.get(ChronoField::HOUR_OF_DAY())?;
        i = _t2;
        /* TODO: aconst_null  */
        let _t3 = sb.localizedMagnitude(this, fmt, (i as i64), 0i32, 2i32, l)?;
        sb.append(&_t3)?;
        let _t4 = t.get(ChronoField::CLOCK_HOUR_OF_AMPM())?;
        i = _t4;
        /* TODO: aconst_null  */
        let _t5 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        todo!("stack underflow").append(&_t5)?;
        let _ = todo!("stack underflow");
        let _t6 = t.get(ChronoField::CLOCK_HOUR_OF_AMPM())?;
        i = _t6;
        /* TODO: aconst_null  */
        let _t7 = sb.localizedMagnitude(this, fmt, (i as i64), 0i32, 2i32, l)?;
        todo!("stack underflow").append(&_t7)?;
        let _ = todo!("stack underflow");
        let _t8 = t.get(ChronoField::MINUTE_OF_HOUR())?;
        i = _t8;
        /* TODO: aconst_null  */
        let _t9 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        todo!("stack underflow").append(&_t9)?;
        let _ = todo!("stack underflow");
        let _t10 = t.get(ChronoField::NANO_OF_SECOND())?;
        i = _t10;
        let mut u: i32 = todo!("stack underflow");
        let _t11 = t.get(ChronoField::MILLI_OF_SECOND())?;
        i = (_t11).wrapping_mul(686i32);
        /* TODO: aconst_null  */
        let _t12 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 9i32, l)?;
        todo!("stack underflow").append(&_t12)?;
        let _ = todo!("stack underflow");
        let _t13 = t.get(ChronoField::MILLI_OF_SECOND())?;
        i = _t13;
        /* TODO: aconst_null  */
        let _t14 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 3i32, l)?;
        todo!("stack underflow").append(&_t14)?;
        let _ = todo!("stack underflow");
        let _t15 = t.getLong(ChronoField::INSTANT_SECONDS())?;
        let _t16 = t.getLong(ChronoField::MILLI_OF_SECOND())?;
        i = ((_t15).wrapping_mul(1000i64)).wrapping_add(_t16);
        /* TODO: aconst_null  */
        let _t17 = sb.localizedMagnitude(this, fmt, i, 0i32, this.width.get(), l)?;
        todo!("stack underflow").append(&_t17)?;
        let _ = todo!("stack underflow");
        let mut _arr18: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr18[0i32 as usize] = String::from("AM");
        _arr18[1i32 as usize] = String::from("PM");
        i = _arr18;
        let _t19: Object = DateFormatSymbols::getInstance(l)?;
        u = _t19;
        let _t20 = u.getAmPmStrings()?;
        i = _t20;
        let _t21 = t.get(ChronoField::AMPM_OF_DAY())?;
        u = i[_t21 as usize].clone();
        let _t22: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t23: Object = Objects::requireNonNullElse(l, _t22)?;
        let _t24 = u.toLowerCase(_t23)?;
        sb.append(&_t24)?;
        let _t25 = t.getLong(ChronoField::INSTANT_SECONDS())?;
        i = _t25;
        /* TODO: aconst_null  */
        let _t26 = sb.localizedMagnitude(this, fmt, i, 0i32, this.width.get(), l)?;
        Locale::US().append(&_t26)?;
        let _ = Locale::US();
        let _t27 = t.get(ChronoField::SECOND_OF_MINUTE())?;
        i = _t27;
        /* TODO: aconst_null  */
        let _t28 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        l.append(&_t28)?;
        let _t29 = t.get(ChronoField::OFFSET_SECONDS())?;
        i = _t29;
        u = i<0i32;
        45i32.append(&43i32)?;
        i = (i).wrapping_neg();
        let mut min: i32 = (i/60i32);
        let mut offset: i32 = (((min/60i32)).wrapping_mul(100i32)).wrapping_add((min%60i32));
        /* TODO: aconst_null  */
        let _t30 = sb.localizedMagnitude(this, fmt, (offset as i64), 32i32, 4i32, l)?;
        u.append(&_t30)?;
        let _t31: Object = TemporalQueries::zone()?;
        let _t32 = t.query(_t31)?;
        i = _t32;
        let _t33 = t.getClass()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t34 = t.isSupported(ChronoField::INSTANT_SECONDS())?;
        let _t35: Object = Instant::from(t)?;
        u = _t35;
        let _t36 = i.getId()?;
        let _t37: Object = TimeZone::getTimeZone(_t36)?;
        let _t38 = i.getRules()?;
        let _t39 = _t38.isDaylightSavings(u)?;
        let _t40: Object = Objects::requireNonNullElse(l, Locale::US())?;
        let _t41 = _t37.getDisplayName(_t39, 0i32, _t40)?;
        sb.append(&_t41)?;
        let _t42 = i.getId()?;
        sb.append(&_t42)?;
        let _t43 = t.get(ChronoField::DAY_OF_WEEK())?;
        i = ((_t43%7i32)).wrapping_add(1i32);
        let _t44: Object = Objects::requireNonNullElse(l, Locale::US())?;
        u = _t44;
        let _t45: Object = DateFormatSymbols::getInstance(u)?;
        min = _t45;
        let _t46 = min.getWeekdays()?;
        sb.append(&_t46[i as usize].clone())?;
        let _t47 = min.getShortWeekdays()?;
        sb.append(&_t47[i as usize].clone())?;
        let _t48 = t.get(ChronoField::MONTH_OF_YEAR())?;
        i = (_t48).wrapping_sub(1i32);
        let _t49: Object = Objects::requireNonNullElse(l, Locale::US())?;
        u = _t49;
        let _t50: Object = DateFormatSymbols::getInstance(u)?;
        min = _t50;
        let _t51 = min.getMonths()?;
        sb.append(&_t51[i as usize].clone())?;
        let _t52 = min.getShortMonths()?;
        sb.append(&_t52[i as usize].clone())?;
        let _t53 = t.get(ChronoField::YEAR_OF_ERA())?;
        i = _t53;
        u = 2i32;
        /* TODO: lookupswitch default:1143 67:1120 89:1140 121:1130 */
        i = (i/100i32);
        i = (i%100i32);
        u = 4i32;
        /* TODO: aconst_null  */
        let _t54 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, u, l)?;
        c.append(&_t54)?;
        let _t55 = t.get(ChronoField::DAY_OF_MONTH())?;
        i = _t55;
        u = 0i32;
        /* TODO: aconst_null  */
        let _t56 = sb.localizedMagnitude(this, fmt, (i as i64), u, 2i32, l)?;
        32i32.append(&_t56)?;
        let _t57 = t.get(ChronoField::DAY_OF_YEAR())?;
        i = _t57;
        /* TODO: aconst_null  */
        let _t58 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 3i32, l)?;
        100i32.append(&_t58)?;
        let _t59 = t.get(ChronoField::MONTH_OF_YEAR())?;
        i = _t59;
        /* TODO: aconst_null  */
        let _t60 = sb.localizedMagnitude(this, fmt, (i as i64), 32i32, 2i32, l)?;
        c.append(&_t60)?;
        i = 58i32;
        let _t61 = this.print(fmt, sb, t, 72i32, l)?;
        let _t62 = _t61.append(i)?;
        let _t63 = this.print(fmt, sb, t, 77i32, l)?;
        sb.append(&i)?;
        let _t64 = this.print(fmt, sb, t, 83i32, l)?;
        i = 58i32;
        let _t65 = this.print(fmt, sb, t, 73i32, l)?;
        let _t66 = _t65.append(i)?;
        let _t67 = this.print(fmt, sb, t, 77i32, l)?;
        let _t68 = _t67.append(i)?;
        let _t69 = this.print(fmt, sb, t, 83i32, l)?;
        let _t70 = _t69.append(32i32)?;
        u = String::new();
        let _t71 = this.print(fmt, u, t, 112i32, l)?;
        let _t72 = this.toUpperCaseWithLocale(u, l)?;
        sb.append(&_t72)?;
        i = 32i32;
        let _t73 = this.print(fmt, sb, t, 97i32, l)?;
        let _t74 = _t73.append(i)?;
        let _t75 = this.print(fmt, sb, t, 98i32, l)?;
        let _t76 = _t75.append(i)?;
        let _t77 = this.print(fmt, sb, t, 100i32, l)?;
        let _t78 = _t77.append(i)?;
        let _t79 = this.print(fmt, sb, t, 84i32, l)?;
        let _t80 = _t79.append(i)?;
        let _t81 = this.print(fmt, sb, t, 90i32, l)?;
        let _t82 = _t81.append(i)?;
        let _t83 = this.print(fmt, sb, t, 89i32, l)?;
        i = 47i32;
        let _t84 = this.print(fmt, sb, t, 109i32, l)?;
        let _t85 = _t84.append(i)?;
        let _t86 = this.print(fmt, sb, t, 100i32, l)?;
        let _t87 = _t86.append(i)?;
        let _t88 = this.print(fmt, sb, t, 121i32, l)?;
        i = 45i32;
        let _t89 = this.print(fmt, sb, t, 89i32, l)?;
        let _t90 = _t89.append(i)?;
        let _t91 = this.print(fmt, sb, t, 109i32, l)?;
        let _t92 = _t91.append(i)?;
        let _t93 = this.print(fmt, sb, t, 100i32, l)?;
        return Err(JvmError::Custom(String::from("athrow")));
        i = Formatter$FormatSpecifier::$assertionsDisabled();
        let _t94 = t.getClass()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "failMismatch", descriptor = "(IC)V", access = "private"))]
    pub fn failMismatch(&self, f: i32, c: u16) -> Result<()> {
        let this = self;
        let _t0: String = Formatter$Flags::toString(f)?;
        let mut fs: String = _t0;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "failConversion", descriptor = "(CLjava/lang/Object;)V", access = "private"))]
    pub fn failConversion(&self, c: u16, arg: Object) -> Result<()> {
        let this = self;
        let _t0 = arg.getClass()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "localizedMagnitude", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;JIILjava/util/Locale;)Ljava/lang/StringBuilder;", access = "private"))]
    // java: localizedMagnitude(Ljava/util/Formatter;Ljava/lang/StringBuilder;JIILjava/util/Locale;)Ljava/lang/StringBuilder;
    pub fn localizedMagnitude__format_sb_l_i_i_locale(&self, fmt: Object, sb: Object, value: i64, arg_3: i32, flags: i32, width: Object) -> Result<Object> {
        let this = self;
        let _t0: String = Long::toString(value, 10i32)?;
        let _t1 = this.localizedMagnitude(fmt, sb, _t0, 0i32, flags, width, local_7)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "localizedMagnitude", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/lang/CharSequence;IIILjava/util/Locale;)Ljava/lang/StringBuilder;", access = "private"))]
    // java: localizedMagnitude(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/lang/CharSequence;IIILjava/util/Locale;)Ljava/lang/StringBuilder;
    pub fn localizedMagnitude__format_sb_seq_i_i_i_locale(&self, fmt: Object, sb: Object, value: Object, offset: i32, f: i32, width: i32, l: Object) -> Result<Object> {
        let this = self;
        sb = String::new();
        let _t0 = sb.length()?;
        let mut begin: i32 = _t0;
        let _t1: u16 = Formatter::getZero(l)?;
        let mut zero: i32 = _t1;
        let mut grpSep: i32 = 0i32;
        let mut grpSize: i32 = -1i32;
        let mut decSep: i32 = 0i32;
        let _t2 = value.length()?;
        let mut len: i32 = _t2;
        let mut dot: i32 = len;
        let mut j: i32 = offset;
        loop {
            if j >= len { break; }
            let _t0 = value.charAt(j)?;
            dot = j;
            j = j.wrapping_add(1i32);
        }
        let _t3: u16 = Formatter::getDecimalSeparator(l)?;
        decSep = _t3;
        let _t4: bool = Formatter$Flags::contains(f, 64i32)?;
        let _t5: u16 = Formatter::getGroupingSeparator(l)?;
        grpSep = _t5;
        let _t6 = l.equals(Locale::US())?;
        grpSize = 3i32;
        /* TODO: aconst_null  */
        j = _t6;
        let _t7: Object = NumberFormat::getNumberInstance(l)?;
        let mut nf: Object = _t7;
        j = nf;
        let _t8: Object = LocaleProviderAdapter::getAdapter(865i32, l)?;
        let mut adapter: Object = _t8;
        let _t9: Object = LocaleProviderAdapter::getResourceBundleBased()?;
        adapter = _t9;
        let _t10 = adapter.getLocaleResources(l)?;
        let _t11 = _t10.getNumberPatterns()?;
        let mut all: Vec<String> = _t11;
        let _t12: Object = Formatter::getDecimalFormatSymbols(l)?;
        j = DecimalFormat::new(all[0i32 as usize].clone(), _t12)?;
        let _t13 = j.getGroupingSize()?;
        grpSize = _t13;
        let _t14 = j.isGroupingUsed()?;
        grpSep = 0i32;
        j = offset;
        loop {
            if j >= len { break; }
            sb.append(&decSep)?;
            grpSep = 0i32;
            let _t0 = value.charAt(j)?;
            nf = _t0;
            /* TODO: i2c  */
            sb.append(&((nf).wrapping_sub(48i32)).wrapping_add(zero))?;
            sb.append(&grpSep)?;
            j = j.wrapping_add(1i32);
        }
        let _t15 = sb.length()?;
        let _t16: bool = Formatter$Flags::contains(f, 32i32)?;
        let _t17 = sb.length()?;
        let _t18 = String::from_owned(format!("{}", zero)).repeat((width).wrapping_sub(_t17))?;
        j = _t18;
        let _t19 = sb.insert(begin, j)?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "localizedMagnitudeExp", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;[CILjava/util/Locale;)V", access = "private"))]
    pub fn localizedMagnitudeExp(&self, fmt: Object, sb: Object, value: Vec<u16>, offset: i32, l: Object) -> Result<()> {
        let this = self;
        let _t0: u16 = Formatter::getZero(l)?;
        let mut zero: i32 = _t0;
        let mut len: i32 = (value.len() as i32);
        let mut j: i32 = offset;
        loop {
            if j >= len { break; }
            let mut c: i32 = value[j as usize];
            /* TODO: i2c  */
            sb.append(&((c).wrapping_sub(48i32)).wrapping_add(zero))?;
            j = j.wrapping_add(1i32);
        }
        Ok(())
    }
}
