#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/File",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable,java/lang/Comparable",
    access      = "public",
    source      = "File.java",
))]
pub struct File {
    #[cfg_attr(any(), java_field(name = "path", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub path: Field<String>,
    #[cfg_attr(any(), java_field(name = "status", descriptor = "Ljava/io/File$PathStatus;", access = "private"))]
    pub status: Field<Object>,
    #[cfg_attr(any(), java_field(name = "prefixLength", descriptor = "I", access = "private final"))]
    pub prefixLength: Field<i32>,
    #[cfg_attr(any(), java_field(name = "filePath", descriptor = "Ljava/nio/file/Path;", access = "private"))]
    pub filePath: Field<Object>,
}

impl File {
    // java: isInvalid()Z
    pub fn isInvalid(&self) -> Result<bool> {
        let this = self;
        let mut s: Object = this.status.get();
        let _t0 = File::FS().isInvalid(this)?;
        s = File_PathStatus::CHECKED();
        this.status.set(s);
        Ok(/* if_acmpne */ true)
    }

    // java: getPrefixLength()I
    pub fn getPrefixLength(&self) -> Result<i32> {
        let this = self;
        Ok(this.prefixLength.get())
    }

    // java: <init>(Ljava/lang/String;I)V
    // java: <init>(Ljava/lang/String;I)V
    pub fn new__str_i(pathname: String, prefixLength: i32) -> Result<Self> {
        let this = Self { path: Field::new(String::new()), status: Field::new(Default::default()), prefixLength: Field::new(0), filePath: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").status.set(this);
        this.path.set(pathname);
        this.prefixLength.set(prefixLength);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/io/File;)V
    // java: <init>(Ljava/lang/String;Ljava/io/File;)V
    pub fn new__str_file(child: String, parent: Object) -> Result<Self> {
        let this = Self { path: Field::new(String::new()), status: Field::new(Default::default()), prefixLength: Field::new(0), filePath: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").status.set(this);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = parent.path.get().isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = File::FS().resolve(parent.path.get(), child)?;
        this.path.set(_t1);
        this.prefixLength.set(parent.prefixLength.get());
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(pathname: String) -> Result<Self> {
        let this = Self { path: Field::new(String::new()), status: Field::new(Default::default()), prefixLength: Field::new(0), filePath: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").status.set(this);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = File::FS().normalize(pathname)?;
        this.path.set(_t0);
        let _t1 = File::FS().prefixLength(this.path.get())?;
        this.prefixLength.set(_t1);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(parent: String, child: String) -> Result<Self> {
        let this = Self { path: Field::new(String::new()), status: Field::new(Default::default()), prefixLength: Field::new(0), filePath: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").status.set(this);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = parent.isEmpty()?;
        let _t1 = File::FS().getDefaultParent()?;
        let _t2 = File::FS().normalize(child)?;
        let _t3 = File::FS().resolve(_t1, _t2)?;
        this.path.set(_t3);
        let _t4 = File::FS().normalize(parent)?;
        let _t5 = File::FS().normalize(child)?;
        let _t6 = File::FS().resolve(_t4, _t5)?;
        this.path.set(_t6);
        let _t7 = File::FS().normalize(child)?;
        this.path.set(_t7);
        let _t8 = File::FS().prefixLength(this.path.get())?;
        this.prefixLength.set(_t8);
        Ok(this)
    }

    // java: <init>(Ljava/io/File;Ljava/lang/String;)V
    // java: <init>(Ljava/io/File;Ljava/lang/String;)V
    pub fn new__file_str(parent: Object, child: String) -> Result<Self> {
        let this = Self { path: Field::new(String::new()), status: Field::new(Default::default()), prefixLength: Field::new(0), filePath: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").status.set(this);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = parent.path.get().isEmpty()?;
        let _t1 = File::FS().getDefaultParent()?;
        let _t2 = File::FS().normalize(child)?;
        let _t3 = File::FS().resolve(_t1, _t2)?;
        this.path.set(_t3);
        let _t4 = File::FS().normalize(child)?;
        let _t5 = File::FS().resolve(parent.path.get(), _t4)?;
        this.path.set(_t5);
        let _t6 = File::FS().normalize(child)?;
        this.path.set(_t6);
        let _t7 = File::FS().prefixLength(this.path.get())?;
        this.prefixLength.set(_t7);
        Ok(this)
    }

    // java: <init>(Ljava/net/URI;)V
    // java: <init>(Ljava/net/URI;)V
    pub fn new__uri(uri: Object) -> Result<Self> {
        let this = Self { path: Field::new(String::new()), status: Field::new(Default::default()), prefixLength: Field::new(0), filePath: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").status.set(this);
        let _t0 = uri.isAbsolute()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = uri.isOpaque()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = uri.getScheme()?;
        let mut scheme: String = _t2;
        let _t3 = scheme.equalsIgnoreCase(String::from("file"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t4 = uri.getRawAuthority()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t5 = uri.getRawFragment()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t6 = uri.getRawQuery()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t7 = uri.getPath()?;
        let mut p: String = _t7;
        let _t8 = p.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t9 = File::FS().fromURIPath(p)?;
        p = _t9;
        let _t10 = p.replace(47i32, File::separatorChar())?;
        p = _t10;
        let _t11 = File::FS().normalize(p)?;
        this.path.set(_t11);
        let _t12 = File::FS().prefixLength(this.path.get())?;
        this.prefixLength.set(_t12);
        Ok(this)
    }

    // java: getName()Ljava/lang/String;
    pub fn getName(&self) -> Result<String> {
        let this = self;
        let _t0 = this.path.get().lastIndexOf(File::separatorChar())?;
        let mut index: i32 = _t0;
        let _t1 = this.path.get().substring(this.prefixLength.get())?;
        return Ok(_t1);
        let _t2 = this.path.get().substring((index).wrapping_add(1i32))?;
        Ok(_t2)
    }

    // java: getParent()Ljava/lang/String;
    pub fn getParent(&self) -> Result<String> {
        let this = self;
        let _t0 = this.path.get().lastIndexOf(File::separatorChar())?;
        let mut index: i32 = _t0;
        let _t1 = this.path.get().length()?;
        let _t2 = this.path.get().substring(0i32, this.prefixLength.get())?;
        return Ok(_t2);
        /* TODO: aconst_null  */
        return Ok(this.prefixLength.get());
        let _t3 = this.path.get().substring(0i32, index)?;
        Ok(_t3)
    }

    // java: getParentFile()Ljava/io/File;
    pub fn getParentFile(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getParent()?;
        let mut p: String = _t0;
        /* TODO: aconst_null  */
        return Ok(p);
        let _t1 = this.getClass()?;
        let _t2 = File::FS().normalize(p)?;
        p = _t2;
        Ok(File::new(p, this.prefixLength.get())?)
    }

    // java: getPath()Ljava/lang/String;
    pub fn getPath(&self) -> Result<String> {
        let this = self;
        Ok(this.path.get())
    }

    // java: isAbsolute()Z
    pub fn isAbsolute(&self) -> Result<bool> {
        let this = self;
        let _t0 = File::FS().isAbsolute(this)?;
        Ok(_t0)
    }

    // java: getAbsolutePath()Ljava/lang/String;
    pub fn getAbsolutePath(&self) -> Result<String> {
        let this = self;
        let _t0 = File::FS().resolve(this)?;
        Ok(_t0)
    }

    // java: getAbsoluteFile()Ljava/io/File;
    pub fn getAbsoluteFile(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getAbsolutePath()?;
        let mut absPath: String = _t0;
        let _t1 = this.getClass()?;
        let _t2 = File::FS().normalize(absPath)?;
        absPath = _t2;
        let _t3 = File::FS().prefixLength(absPath)?;
        Ok(File::new(absPath, _t3)?)
    }

    // java: getCanonicalPath()Ljava/lang/String;
    pub fn getCanonicalPath(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isInvalid()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = File::FS().resolve(this)?;
        let _t2 = File::FS().canonicalize(_t1)?;
        Ok(_t2)
    }

    // java: getCanonicalFile()Ljava/io/File;
    pub fn getCanonicalFile(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getCanonicalPath()?;
        let mut canonPath: String = _t0;
        let _t1 = this.getClass()?;
        let _t2 = File::FS().normalize(canonPath)?;
        canonPath = _t2;
        let _t3 = File::FS().prefixLength(canonPath)?;
        Ok(File::new(canonPath, _t3)?)
    }

    // java: slashify(Ljava/lang/String;Z)Ljava/lang/String;
    pub fn slashify(path: String, isDirectory: bool) -> Result<String> {
        let mut p: String = path;
        let _t0 = p.replace(File::separatorChar(), 47i32)?;
        p = _t0;
        let _t1 = p.startsWith(String::from("/"))?;
        String::new().append(&String::from("/"))?;
        String::new().append(&p)?;
        p = String::new();
        let _t2 = p.endsWith(String::from("/"))?;
        String::new().append(&p)?;
        String::new().append(&String::from("/"))?;
        p = String::new();
        Ok(p)
    }

    // java: toURL()Ljava/net/URL;
    pub fn toURL(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isInvalid()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.getAbsolutePath()?;
        let _t2 = this.isDirectory()?;
        let _t3: String = File::slashify(_t1, _t2)?;
        let mut result: URL = URL::new(String::from("file"), String::from(""), _t3)?;
        Ok(result)
    }

    // java: toURI()Ljava/net/URI;
    pub fn toURI(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getAbsoluteFile()?;
        let mut f: Object = _t0;
        let _t1 = f.getPath()?;
        let _t2 = f.isDirectory()?;
        let _t3: String = File::slashify(_t1, _t2)?;
        let mut sp: String = _t3;
        let _t4 = sp.startsWith(String::from("//"))?;
        String::new().append(&String::from("//"))?;
        String::new().append(&sp)?;
        sp = String::new();
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/net/URI.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V */
        return Ok(todo!("stack underflow"));
        f = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: canRead()Z
    pub fn canRead(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().checkAccess(this, 4i32)?;
        Ok(_t2)
    }

    // java: canWrite()Z
    pub fn canWrite(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().checkAccess(this, 2i32)?;
        Ok(_t2)
    }

    // java: exists()Z
    pub fn exists(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().hasBooleanAttributes(this, 1i32)?;
        Ok(_t2)
    }

    // java: isDirectory()Z
    pub fn isDirectory(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().hasBooleanAttributes(this, 4i32)?;
        Ok(_t2)
    }

    // java: isFile()Z
    pub fn isFile(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().hasBooleanAttributes(this, 2i32)?;
        Ok(_t2)
    }

    // java: isHidden()Z
    pub fn isHidden(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().hasBooleanAttributes(this, 8i32)?;
        Ok(_t2)
    }

    // java: lastModified()J
    pub fn lastModified(&self) -> Result<i64> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i64);
        let _t2 = File::FS().getLastModifiedTime(this)?;
        Ok(_t2)
    }

    // java: length()J
    pub fn length(&self) -> Result<i64> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i64);
        let _t2 = File::FS().getLength(this)?;
        Ok(_t2)
    }

    // java: createNewFile()Z
    pub fn createNewFile(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = File::FS().createFileExclusively(this.path.get())?;
        Ok(_t2)
    }

    // java: delete()Z
    pub fn delete(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkDelete(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().delete(this)?;
        Ok(_t2)
    }

    // java: deleteOnExit()V
    pub fn deleteOnExit(&self) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkDelete(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(());
        DeleteOnExitHook::add(this.path.get())?;
        Ok(())
    }

    // java: list()[Ljava/lang/String;
    // java: list()[Ljava/lang/String;
    pub fn list(&self) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.normalizedList()?;
        Ok(_t0)
    }

    // java: normalizedList()[Ljava/lang/String;
    pub fn normalizedList(&self) -> Result<Vec<String>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        /* TODO: aconst_null  */
        return Ok(_t1);
        let _t2 = File::FS().list(this)?;
        let mut s: Vec<String> = _t2;
        let _t3 = this.getClass()?;
        let mut _arr4: Vec<Object> = Vec::with_capacity((s.len() as i32) as usize);
        let mut normalized: Vec<Object> = _arr4;
        let mut i: i32 = 0i32;
        loop {
            if i >= (s.len() as i32) { break; }
            let _t0 = File::FS().normalize(s[i as usize].clone())?;
            normalized[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        s = normalized;
        Ok(s)
    }

    // java: list(Ljava/io/FilenameFilter;)[Ljava/lang/String;
    // java: list(Ljava/io/FilenameFilter;)[Ljava/lang/String;
    pub fn list__filena(&self, filter: Object) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.normalizedList()?;
        let mut names: Vec<String> = _t0;
        return Ok(names);
        let mut v: ArrayList<_> = ArrayList::<_>::new()?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (names.len() as i32) { break; }
            let _t0 = filter.accept(this, names[i as usize].clone())?;
            let _t1 = v.add(names[i as usize].clone())?;
            i = i.wrapping_add(1i32);
        }
        let _t1 = v.size()?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(_t1 as usize);
        let _t3 = v.toArray(_arr2)?;
        Ok(_t3)
    }

    // java: listFiles()[Ljava/io/File;
    // java: listFiles()[Ljava/io/File;
    pub fn listFiles(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.normalizedList()?;
        let mut ss: Vec<String> = _t0;
        /* TODO: aconst_null  */
        return Ok(ss);
        let mut n: i32 = (ss.len() as i32);
        let mut _arr1: Vec<Object> = Vec::with_capacity(n as usize);
        let mut fs: Vec<Object> = _arr1;
        let mut i: i32 = 0i32;
        loop {
            if i >= n { break; }
            fs[i as usize] = File::new(ss[i as usize].clone(), this)?;
            i = i.wrapping_add(1i32);
        }
        Ok(fs)
    }

    // java: listFiles(Ljava/io/FilenameFilter;)[Ljava/io/File;
    // java: listFiles(Ljava/io/FilenameFilter;)[Ljava/io/File;
    pub fn listFiles__filena(&self, filter: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.normalizedList()?;
        let mut ss: Vec<String> = _t0;
        /* TODO: aconst_null  */
        return Ok(ss);
        let mut files: ArrayList<_> = ArrayList::<_>::new()?;
        let mut local_4: Vec<String> = ss;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut s: String = local_4[local_6 as usize].clone();
            let _t0 = filter.accept(this, s)?;
            let _t1 = files.add(File::new(s, this)?)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t1 = files.size()?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(_t1 as usize);
        let _t3 = files.toArray(_arr2)?;
        Ok(_t3)
    }

    // java: listFiles(Ljava/io/FileFilter;)[Ljava/io/File;
    // java: listFiles(Ljava/io/FileFilter;)[Ljava/io/File;
    pub fn listFiles__filefi(&self, filter: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.normalizedList()?;
        let mut ss: Vec<String> = _t0;
        /* TODO: aconst_null  */
        return Ok(ss);
        let mut files: ArrayList<_> = ArrayList::<_>::new()?;
        let mut local_4: Vec<String> = ss;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut s: String = local_4[local_6 as usize].clone();
            let mut f: File = File::new(s, this)?;
            let _t0 = filter.accept(f)?;
            let _t1 = files.add(f)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t1 = files.size()?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(_t1 as usize);
        let _t3 = files.toArray(_arr2)?;
        Ok(_t3)
    }

    // java: mkdir()Z
    pub fn mkdir(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().createDirectory(this)?;
        Ok(_t2)
    }

    // java: mkdirs()Z
    pub fn mkdirs(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.exists()?;
        return Ok(0i32);
        let _t1 = this.mkdir()?;
        return Ok(1i32);
        /* TODO: aconst_null  */
        let mut canonFile: bool = _t1;
        let _t2 = this.getCanonicalFile()?;
        canonFile = _t2;
        let mut e: bool = _t0;
        return Ok(0i32);
        let _t3 = canonFile.getParentFile()?;
        e = _t3;
        let _t4 = e.mkdirs()?;
        let _t5 = e.exists()?;
        let _t6 = canonFile.mkdir()?;
        Ok(_t6!=0i32)
    }

    // java: renameTo(Ljava/io/File;)Z
    pub fn renameTo(&self, dest: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        security.checkWrite(dest.path.get())?;
        let _t1 = this.isInvalid()?;
        let _t2 = dest.isInvalid()?;
        return Ok(0i32);
        let _t3 = File::FS().rename(this, dest)?;
        Ok(_t3)
    }

    // java: setLastModified(J)Z
    pub fn setLastModified(&self, time: i64) -> Result<bool> {
        let this = self;
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().setLastModifiedTime(this, time)?;
        Ok(_t2)
    }

    // java: setReadOnly()Z
    pub fn setReadOnly(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().setReadOnly(this)?;
        Ok(_t2)
    }

    // java: setWritable(ZZ)Z
    // java: setWritable(ZZ)Z
    pub fn setWritable__z_z(&self, writable: bool, ownerOnly: bool) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().setPermission(this, 2i32, writable, ownerOnly)?;
        Ok(_t2)
    }

    // java: setWritable(Z)Z
    // java: setWritable(Z)Z
    pub fn setWritable__z(&self, writable: bool) -> Result<bool> {
        let this = self;
        let _t0 = this.setWritable(writable, 1i32)?;
        Ok(_t0)
    }

    // java: setReadable(ZZ)Z
    // java: setReadable(ZZ)Z
    pub fn setReadable__z_z(&self, readable: bool, ownerOnly: bool) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().setPermission(this, 4i32, readable, ownerOnly)?;
        Ok(_t2)
    }

    // java: setReadable(Z)Z
    // java: setReadable(Z)Z
    pub fn setReadable__z(&self, readable: bool) -> Result<bool> {
        let this = self;
        let _t0 = this.setReadable(readable, 1i32)?;
        Ok(_t0)
    }

    // java: setExecutable(ZZ)Z
    // java: setExecutable(ZZ)Z
    pub fn setExecutable__z_z(&self, executable: bool, ownerOnly: bool) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkWrite(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().setPermission(this, 1i32, executable, ownerOnly)?;
        Ok(_t2)
    }

    // java: setExecutable(Z)Z
    // java: setExecutable(Z)Z
    pub fn setExecutable__z(&self, executable: bool) -> Result<bool> {
        let this = self;
        let _t0 = this.setExecutable(executable, 1i32)?;
        Ok(_t0)
    }

    // java: canExecute()Z
    pub fn canExecute(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkExec(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i32);
        let _t2 = File::FS().checkAccess(this, 1i32)?;
        Ok(_t2)
    }

    // java: listRoots()[Ljava/io/File;
    pub fn listRoots() -> Result<Vec<Object>> {
        let _t0 = File::FS().listRoots()?;
        Ok(_t0)
    }

    // java: getTotalSpace()J
    pub fn getTotalSpace(&self) -> Result<i64> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("getFileSystemAttributes"))?)?;
        sm.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i64);
        let _t2 = File::FS().getSpace(this, 0i32)?;
        let mut space: i64 = _t2;
        /* TODO: lcmp  */
        Ok(9223372036854775807i64)
    }

    // java: getFreeSpace()J
    pub fn getFreeSpace(&self) -> Result<i64> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("getFileSystemAttributes"))?)?;
        sm.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i64);
        let _t2 = File::FS().getSpace(this, 1i32)?;
        let mut space: i64 = _t2;
        /* TODO: lcmp  */
        Ok(9223372036854775807i64)
    }

    // java: getUsableSpace()J
    pub fn getUsableSpace(&self) -> Result<i64> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("getFileSystemAttributes"))?)?;
        sm.checkRead(this.path.get())?;
        let _t1 = this.isInvalid()?;
        return Ok(0i64);
        let _t2 = File::FS().getSpace(this, 2i32)?;
        let mut space: i64 = _t2;
        /* TODO: lcmp  */
        Ok(9223372036854775807i64)
    }

    // java: createTempFile(Ljava/lang/String;Ljava/lang/String;Ljava/io/File;)Ljava/io/File;
    // java: createTempFile(Ljava/lang/String;Ljava/lang/String;Ljava/io/File;)Ljava/io/File;
    pub fn createTempFile__str_str_file(prefix: String, suffix: String, directory: Object) -> Result<Object> {
        let _t0 = prefix.length()?;
        String::new().append(&String::from("Prefix string \""))?;
        String::new().append(&prefix)?;
        String::new().append(&String::from("\" too short: length must be at least 3"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        suffix = String::from(".tmp");
        let _t1: Object = File_TempDirectory::location()?;
        let mut tmpdir: Object = _t1;
        let _t2: Object = System::getSecurityManager()?;
        let mut sm: Object = _t2;
        let _t3: Object = File_TempDirectory::generateFile(prefix, suffix, tmpdir)?;
        let mut f: Object = _t3;
        let _t4 = f.getPath()?;
        sm.checkWrite(_t4)?;
        let mut se: Object = sm;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t5 = File::FS().hasBooleanAttributes(f, 1i32)?;
        let _t6 = f.getPath()?;
        let _t7 = File::FS().createFileExclusively(_t6)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(f)
    }

    // java: createTempFile(Ljava/lang/String;Ljava/lang/String;)Ljava/io/File;
    // java: createTempFile(Ljava/lang/String;Ljava/lang/String;)Ljava/io/File;
    pub fn createTempFile__str_str(prefix: String, suffix: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = File::createTempFile(todo!("stack underflow"), prefix, suffix)?;
        Ok(_t0)
    }

    // java: compareTo(Ljava/io/File;)I
    pub fn compareTo(&self, pathname: Object) -> Result<i32> {
        let this = self;
        let _t0 = File::FS().compare(this, pathname)?;
        Ok(_t0)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        let mut file: Object = obj;
        let _t0 = this.compareTo(file)?;
        return Ok(_t0==0i32);
        Ok(0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = File::FS().hashCode(this)?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.getPath()?;
        Ok(_t0)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultWriteObject()?;
        s.writeChar(File::separatorChar())?;
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let mut fields: Object = _t0;
        /* TODO: aconst_null  */
        let _t1 = todo!("stack underflow").get(fields, String::from("path"))?;
        let mut pathField: Object = _t1;
        let _t2 = s.readChar()?;
        let mut sep: i32 = _t2;
        let _t3 = pathField.replace(sep, File::separatorChar())?;
        pathField = _t3;
        let _t4 = File::FS().normalize(pathField)?;
        let mut path: String = _t4;
        File::UNSAFE().putReference(this, File::PATH_OFFSET(), path)?;
        let _t5 = File::FS().prefixLength(path)?;
        File::UNSAFE().putIntVolatile(this, File::PREFIX_LENGTH_OFFSET(), _t5)?;
        Ok(())
    }

    // java: toPath()Ljava/nio/file/Path;
    pub fn toPath(&self) -> Result<Object> {
        let this = self;
        let mut result: Object = this.filePath.get();
        let mut local_2: File = this;
        /* TODO: monitorenter  */
        result = this.filePath.get();
        let _t0: Object = FileSystems::getDefault()?;
        let mut _arr1: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t2 = _t0.getPath(this.path.get(), _arr1)?;
        result = _t2;
        this.filePath.set(result);
        /* TODO: monitorexit  */
        let mut local_3: File = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(result)
    }
}
