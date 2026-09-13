// StringBuilder 内部字符串内容通过注入字段 _sb 存储（Rc<RefCell<String>> 支持共享可变）。
/// @field _sb: JField<Rc<RefCell<std::string::String>>>

/// @synthetic
pub fn new() -> Result<StringBuilder> {
    Ok(StringBuilder::default())
}

/// java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;
pub fn append__str(_this: &StringBuilder, s: String) -> Result<Object> {
    let content = format!("{}", s);
    _this._sb.get().borrow_mut().push_str(&content);
    Ok(Object::default())
}

/// java/lang/StringBuilder.append:(I)Ljava/lang/StringBuilder;
pub fn append__i(_this: &StringBuilder, v: i32) -> Result<Object> {
    _this._sb.get().borrow_mut().push_str(&v.to_string());
    Ok(Object::default())
}

/// java/lang/StringBuilder.append:(J)Ljava/lang/StringBuilder;
pub fn append__l(_this: &StringBuilder, v: i64) -> Result<Object> {
    _this._sb.get().borrow_mut().push_str(&v.to_string());
    Ok(Object::default())
}

/// java/lang/StringBuilder.append:(D)Ljava/lang/StringBuilder;
pub fn append__d(_this: &StringBuilder, v: f64) -> Result<Object> {
    _this._sb.get().borrow_mut().push_str(&v.to_string());
    Ok(Object::default())
}

/// java/lang/StringBuilder.append:(F)Ljava/lang/StringBuilder;
pub fn append__f(_this: &StringBuilder, v: f32) -> Result<Object> {
    _this._sb.get().borrow_mut().push_str(&v.to_string());
    Ok(Object::default())
}

/// java/lang/StringBuilder.append:(Z)Ljava/lang/StringBuilder;
pub fn append__z(_this: &StringBuilder, v: bool) -> Result<Object> {
    _this._sb.get().borrow_mut().push_str(&v.to_string());
    Ok(Object::default())
}

/// java/lang/StringBuilder.append:(C)Ljava/lang/StringBuilder;
pub fn append__c(_this: &StringBuilder, v: u16) -> Result<Object> {
    if let Some(c) = char::from_u32(v as u32) {
        _this._sb.get().borrow_mut().push(c);
    }
    Ok(Object::default())
}

/// java/lang/StringBuilder.append:(Ljava/lang/Object;)Ljava/lang/StringBuilder;
pub fn append__obj(_this: &StringBuilder, obj: Object) -> Result<Object> {
    _this._sb.get().borrow_mut().push_str("Object");
    Ok(Object::default())
}

/// java/lang/StringBuilder.toString:()Ljava/lang/String;
pub fn toString(_this: &StringBuilder) -> Result<String> {
    let s = _this._sb.get().borrow().clone();
    Ok(String::from(s.as_str()))
}

/// java/lang/StringBuilder.length:()I
pub fn length(_this: &StringBuilder) -> Result<i32> {
    Ok(_this._sb.get().borrow().len() as i32)
}
