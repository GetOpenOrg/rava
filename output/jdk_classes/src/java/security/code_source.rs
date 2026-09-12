#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/security/CodeSource",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "public",
    source      = "CodeSource.java",
))]
pub struct CodeSource {
    #[cfg_attr(any(), java_field(name = "location", descriptor = "Ljava/net/URL;", access = "private final"))]
    pub location: Field<Object>,
    #[cfg_attr(any(), java_field(name = "signers", descriptor = "[Ljava/security/CodeSigner;", access = "private"))]
    pub signers: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "certs", descriptor = "[Ljava/security/cert/Certificate;", access = "private"))]
    pub certs: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "sp", descriptor = "Ljava/net/SocketPermission;", access = "private"))]
    pub sp: Field<Object>,
    #[cfg_attr(any(), java_field(name = "factory", descriptor = "Ljava/security/cert/CertificateFactory;", access = "private"))]
    pub factory: Field<Object>,
    #[cfg_attr(any(), java_field(name = "locationNoFragString", descriptor = "Ljava/lang/String;", access = "private"))]
    pub locationNoFragString: Field<String>,
}

impl CodeSource {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/net/URL;[Ljava/security/cert/Certificate;)V", access = "public"))]
    // java: <init>(Ljava/net/URL;[Ljava/security/cert/Certificate;)V
    pub fn new__url_arr_cer(url: Object, certs: Vec<Object>) -> Result<Self> {
        let this = Self { location: Field::new(Default::default()), signers: Field::new(Default::default()), certs: Field::new(Default::default()), sp: Field::new(Default::default()), factory: Field::new(Default::default()), locationNoFragString: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").signers.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").certs.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").factory.set(this);
        this.location.set(url);
        let _t0: String = URLUtil::urlNoFragString(url)?;
        this.locationNoFragString.set(_t0);
        let _t1 = certs.clone()?;
        this.certs.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/net/URL;[Ljava/security/CodeSigner;)V", access = "public"))]
    // java: <init>(Ljava/net/URL;[Ljava/security/CodeSigner;)V
    pub fn new__url_arr_cod(url: Object, signers: Vec<Object>) -> Result<Self> {
        let this = Self { location: Field::new(Default::default()), signers: Field::new(Default::default()), certs: Field::new(Default::default()), sp: Field::new(Default::default()), factory: Field::new(Default::default()), locationNoFragString: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").signers.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").certs.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").factory.set(this);
        this.location.set(url);
        let _t0: String = URLUtil::urlNoFragString(url)?;
        this.locationNoFragString.set(_t0);
        let _t1 = signers.clone()?;
        this.signers.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.location.get().hashCode()?;
        return Ok(_t0);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut other: Object = obj;
        let _t0: bool = Objects::equals(this.location.get(), other.location.get())?;
        let _t1 = this.matchCerts(other, 1i32)?;
        Ok(_t1!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "getLocation", descriptor = "()Ljava/net/URL;", access = "public final"))]
    pub fn getLocation(&self) -> Result<Object> {
        let this = self;
        Ok(this.location.get())
    }

    #[cfg_attr(any(), java_method(name = "getLocationNoFragString", descriptor = "()Ljava/lang/String;"))]
    pub fn getLocationNoFragString(&self) -> Result<String> {
        let this = self;
        Ok(this.locationNoFragString.get())
    }

    #[cfg_attr(any(), java_method(name = "getCertificates", descriptor = "()[Ljava/security/cert/Certificate;", access = "public final"))]
    pub fn getCertificates(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.certs.get().clone()?;
        return Ok(_t0);
        let mut certChains: ArrayList<_> = ArrayList::<_>::new()?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.signers.get().len() as i32) { break; }
            let _t0 = this.signers.get()[i as usize].clone().getSignerCertPath()?;
            let _t1 = _t0.getCertificates()?;
            let _t2 = certChains.addAll(_t1)?;
            i = i.wrapping_add(1i32);
        }
        let mut _arr1: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t2 = certChains.toArray(_arr1)?;
        this.certs.set(_t2);
        let _t3 = this.certs.get().clone()?;
        return Ok(_t3);
        /* TODO: aconst_null  */
        Ok(this.signers.get())
    }

    #[cfg_attr(any(), java_method(name = "getCodeSigners", descriptor = "()[Ljava/security/CodeSigner;", access = "public final"))]
    pub fn getCodeSigners(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.signers.get().clone()?;
        return Ok(_t0);
        let _t1 = this.convertCertArrayToSignerArray(this.certs.get())?;
        this.signers.set(_t1);
        let _t2 = this.signers.get().clone()?;
        return Ok(_t2);
        /* TODO: aconst_null  */
        Ok(this.certs.get())
    }

    #[cfg_attr(any(), java_method(name = "implies", descriptor = "(Ljava/security/CodeSource;)Z", access = "public"))]
    pub fn implies(&self, codesource: Object) -> Result<bool> {
        let this = self;
        return Ok(0i32);
        let _t0 = this.matchCerts(codesource, 0i32)?;
        let _t1 = this.matchLocation(codesource)?;
        Ok(_t1!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "matchCerts", descriptor = "(Ljava/security/CodeSource;Z)Z"))]
    pub fn matchCerts(&self, that: Object, strict: bool) -> Result<bool> {
        let this = self;
        return Ok(that.signers.get().is_none());
        return Ok(1i32);
        return Ok(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.signers.get().len() as i32) { break; }
            let mut match_: i32 = 0i32;
            let mut j: i32 = 0i32;
            let _t0 = this.signers.get()[i as usize].clone().equals(that.signers.get()[j as usize].clone())?;
            match_ = 1i32;
            j = j.wrapping_add(1i32);
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        return Ok(1i32);
        return Ok(0i32);
        i = 0i32;
        loop {
            if i >= (this.certs.get().len() as i32) { break; }
            match_ = 0i32;
            j = 0i32;
            let _t0 = this.certs.get()[i as usize].clone().equals(that.certs.get()[j as usize].clone())?;
            match_ = 1i32;
            j = j.wrapping_add(1i32);
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        return Ok(1i32);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "matchLocation", descriptor = "(Ljava/security/CodeSource;)Z", access = "private"))]
    pub fn matchLocation(&self, that: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let _t0 = this.location.get().equals(that.location.get())?;
        return Ok(1i32);
        let _t1 = this.location.get().getProtocol()?;
        let _t2 = that.location.get().getProtocol()?;
        let _t3 = _t1.equalsIgnoreCase(_t2)?;
        return Ok(0i32);
        let _t4 = this.location.get().getPort()?;
        let mut thisPort: i32 = _t4;
        let _t5 = that.location.get().getPort()?;
        let mut thatPort: i32 = _t5;
        let _t6 = that.location.get().getDefaultPort()?;
        let mut port: i32 = _t6;
        return Ok(0i32);
        let _t7 = this.location.get().getFile()?;
        let _t8 = _t7.endsWith(String::from("/-"))?;
        let _t9 = this.location.get().getFile()?;
        let _t10 = this.location.get().getFile()?;
        let _t11 = _t10.length()?;
        let _t12 = _t9.substring(0i32, (_t11).wrapping_sub(1i32))?;
        thatPort = _t12;
        let _t13 = that.location.get().getFile()?;
        let _t14 = _t13.startsWith(thatPort)?;
        return Ok(0i32);
        let _t15 = this.location.get().getFile()?;
        let _t16 = _t15.endsWith(String::from("/*"))?;
        let _t17 = that.location.get().getFile()?;
        let _t18 = _t17.lastIndexOf(47i32)?;
        thatPort = _t18;
        return Ok(0i32);
        let _t19 = this.location.get().getFile()?;
        let _t20 = this.location.get().getFile()?;
        let _t21 = _t20.length()?;
        let _t22 = _t19.substring(0i32, (_t21).wrapping_sub(1i32))?;
        port = _t22;
        let _t23 = that.location.get().getFile()?;
        let _t24 = _t23.substring(0i32, (thatPort).wrapping_add(1i32))?;
        let mut thatPath: String = _t24;
        let _t25 = thatPath.equals(port)?;
        return Ok(0i32);
        let _t26 = that.location.get().getFile()?;
        let _t27 = this.location.get().getFile()?;
        let _t28 = _t26.equals(_t27)?;
        let _t29 = that.location.get().getFile()?;
        let _t30 = this.location.get().getFile()?;
        String::new().append(&_t30)?;
        String::new().append(&String::from("/"))?;
        let _t31 = _t29.equals(String::new())?;
        return Ok(0i32);
        let _t32 = this.location.get().getRef()?;
        let _t33 = this.location.get().getRef()?;
        let _t34 = that.location.get().getRef()?;
        let _t35 = _t33.equals(_t34)?;
        return Ok(0i32);
        let _t36 = this.location.get().getHost()?;
        thatPort = _t36;
        let _t37 = that.location.get().getHost()?;
        port = _t37;
        let _t38 = String::from("").equals(thatPort)?;
        let _t39 = String::from("localhost").equals(thatPort)?;
        let _t40 = String::from("").equals(port)?;
        let _t41 = String::from("localhost").equals(port)?;
        let _t42 = thatPort.equals(port)?;
        return Ok(0i32);
        this.sp.set(SocketPermission::new(thatPort, String::from("resolve"))?);
        that.sp.set(SocketPermission::new(port, String::from("resolve"))?);
        let _t43 = this.sp.get().implies(that.sp.get())?;
        return Ok(_t43);
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut sb: String = String::new();
        sb.append(&String::from("("))?;
        sb.append(&this.location.get())?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.certs.get().len() as i32) { break; }
            String::new().append(&String::from(""))?;
            String::new().append(&this.certs.get()[i as usize].clone())?;
            sb.append(&String::new())?;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (this.signers.get().len() as i32) { break; }
            String::new().append(&String::from(""))?;
            String::new().append(&this.signers.get()[i as usize].clone())?;
            sb.append(&String::new())?;
            i = i.wrapping_add(1i32);
        }
        sb.append(&String::from("<no signer certificates>"))?;
        sb.append(&String::from(")"))?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, oos: Object) -> Result<()> {
        let this = self;
        oos.defaultWriteObject()?;
        oos.writeInt(0i32)?;
        oos.writeInt((this.certs.get().len() as i32))?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.certs.get().len() as i32) { break; }
            let mut cert: Object = this.certs.get()[i as usize].clone();
            let _t0 = cert.getType()?;
            oos.writeUTF(_t0)?;
            let _t1 = cert.getEncoded()?;
            let mut encoded: Vec<i8> = _t1;
            oos.writeInt((encoded.len() as i32))?;
            oos.write(encoded)?;
            encoded = todo!("stack underflow");
            let _t2 = encoded.getMessage()?;
            return Err(JvmError::Custom(String::from("athrow")));
            i = i.wrapping_add(1i32);
        }
        oos.writeObject(this.signers.get())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, ois: Object) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        let mut cfs: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        let mut certList: i32 = todo!("stack underflow");
        ois.defaultReadObject()?;
        let _t0 = ois.readInt()?;
        let mut size: i32 = _t0;
        cfs = Hashtable::new(3i32)?;
        let _t1: i32 = (size).min(20i32);
        certList = ArrayList::<_>::new()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = ois.readUTF()?;
            let mut certType: String = _t0;
            let _t1 = cfs.containsKey(certType)?;
            let _t2 = cfs.get(certType)?;
            let mut cf: Object = _t2;
            let _t3: Object = CertificateFactory::getInstance(certType)?;
            cf = _t3;
            let mut ce: bool = _t1;
            String::new().append(&String::from("Certificate factory for"))?;
            String::new().append(&certType)?;
            String::new().append(&String::from("not found"))?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t4 = cfs.put(certType, cf)?;
            let _t5 = ois.readInt()?;
            let _t6: Vec<i8> = IOUtils::readExactlyNBytes(ois, _t5)?;
            ce = _t6;
            let mut bais: ByteArrayInputStream = ByteArrayInputStream::new(ce)?;
            let _t7 = cf.generateCertificate(bais)?;
            let _t8 = certList.add(_t7)?;
            let mut ce: i32 = todo!("stack underflow");
            let _t9 = ce.getMessage()?;
            return Err(JvmError::Custom(String::from("athrow")));
            bais.close()?;
            i = i.wrapping_add(1i32);
        }
        let mut _arr2: Vec<Object> = Vec::with_capacity(size as usize);
        let _t3 = certList.toArray(_arr2)?;
        this.certs.set(_t3);
        let _t4 = ois.readObject()?;
        let _t5 = _t4.clone()?;
        this.signers.set(_t5);
        i = certList;
        let _t6: String = URLUtil::urlNoFragString(this.location.get())?;
        this.locationNoFragString.set(_t6);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "convertCertArrayToSignerArray", descriptor = "([Ljava/security/cert/Certificate;)[Ljava/security/CodeSigner;", access = "private"))]
    pub fn convertCertArrayToSignerArray(&self, certs: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        /* TODO: aconst_null  */
        return Ok(certs);
        let _t0: Object = CertificateFactory::getInstance(String::from("X.509"))?;
        this.factory.set(_t0);
        let mut i: i32 = 0i32;
        let mut signers: ArrayList<_> = ArrayList::<_>::new()?;
        loop {
            if i >= (certs.len() as i32) { break; }
            let mut certChain: ArrayList<_> = ArrayList::<_>::new()?;
            i = i.wrapping_add(1i32);
            let _t0 = certChain.add(certs[i as usize].clone())?;
            let mut j: i32 = i;
            let _t1 = certs[j as usize].clone().getBasicConstraints()?;
            let _t2 = certChain.add(certs[j as usize].clone())?;
            j = j.wrapping_add(1i32);
            i = j;
            let _t3 = this.factory.get().generateCertPath(certChain)?;
            let mut certPath: Object = _t3;
            /* TODO: aconst_null  */
            let mut _obj4: CodeSigner = CodeSigner::new(CodeSigner::new(), certPath)?;
            let _t5 = signers.add(_obj4)?;
        }
        let _t1 = signers.isEmpty()?;
        /* TODO: aconst_null  */
        return Ok(_t1);
        let mut _arr2: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t3 = signers.toArray(_arr2)?;
        return Ok(_t3);
        i = this.factory.get();
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }
}
