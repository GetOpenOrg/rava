#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Locale",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/Cloneable,java/io/Serializable",
    access      = "public final",
    source      = "Locale.java",
))]
pub struct Locale {
    #[cfg_attr(any(), java_field(name = "baseLocale", descriptor = "Lsun/util/locale/BaseLocale;", access = "private"))]
    pub baseLocale: Field<Object>,
    #[cfg_attr(any(), java_field(name = "localeExtensions", descriptor = "Lsun/util/locale/LocaleExtensions;", access = "private"))]
    pub localeExtensions: Field<Object>,
    #[cfg_attr(any(), java_field(name = "hashCodeValue", descriptor = "I", access = "private"))]
    pub hashCodeValue: Field<i32>,
    #[cfg_attr(any(), java_field(name = "languageTag", descriptor = "Ljava/lang/String;", access = "private"))]
    pub languageTag: Field<String>,
}

impl Locale {
    #[cfg_attr(any(), java_method(name = "createConstant", descriptor = "(B)Ljava/util/Locale;", access = "private static"))]
    pub fn createConstant(baseType: i8) -> Result<Object> {
        let mut base: Object = BaseLocale::constantBaseLocales()[baseType as usize].clone();
        /* TODO: aconst_null  */
        let mut _obj0: Locale = Locale::new(Locale::new(), base)?;
        let mut locale: Locale = _obj0;
        let _t1 = Locale::CONSTANT_LOCALES().put(base, locale)?;
        Ok(locale)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V", access = "private"))]
    // java: <init>(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V
    pub fn new__baselo_locale(baseLocale: Object, extensions: Object) -> Result<Self> {
        let this = Self { baseLocale: Field::new(Default::default()), localeExtensions: Field::new(Default::default()), hashCodeValue: Field::new(0), languageTag: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.baseLocale.set(baseLocale);
        this.localeExtensions.set(extensions);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str_str(language: String, country: String, variant: String) -> Result<Self> {
        let this = Self { baseLocale: Field::new(Default::default()), localeExtensions: Field::new(Default::default()), hashCodeValue: Field::new(0), languageTag: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: String = Locale::convertOldISOCodes(language)?;
        let _t1: Object = BaseLocale::getInstance(_t0, String::from(""), country, variant)?;
        this.baseLocale.set(_t1);
        let _t2: Object = Locale::getCompatibilityExtensions(language, String::from(""), country, variant)?;
        this.localeExtensions.set(_t2);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(language: String, country: String) -> Result<Self> {
        let this = Self { baseLocale: Field::new(Default::default()), localeExtensions: Field::new(Default::default()), hashCodeValue: Field::new(0), languageTag: Field::new(String::new()) };
        /* invokespecial Method java/util/Locale.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(language: String) -> Result<Self> {
        let this = Self { baseLocale: Field::new(Default::default()), localeExtensions: Field::new(Default::default()), hashCodeValue: Field::new(0), languageTag: Field::new(String::new()) };
        /* invokespecial Method java/util/Locale.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;", access = "public static"))]
    // java: of(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;
    pub fn of__str_str_str(language: String, country: String, variant: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = Locale::getInstance(todo!("stack underflow"), language, String::from(""), country, variant)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;", access = "public static"))]
    // java: of(Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;
    pub fn of__str_str(language: String, country: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = Locale::getInstance(todo!("stack underflow"), language, String::from(""), country, String::from(""))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/String;)Ljava/util/Locale;", access = "public static"))]
    // java: of(Ljava/lang/String;)Ljava/util/Locale;
    pub fn of__str(language: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = Locale::getInstance(todo!("stack underflow"), language, String::from(""), String::from(""), String::from(""))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getInstance", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;", access = "static"))]
    // java: getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;
    pub fn getInstance__str_str_str(language: String, country: String, variant: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = Locale::getInstance(todo!("stack underflow"), language, String::from(""), country, variant)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getInstance", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;", access = "static"))]
    // java: getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;
    pub fn getInstance__str_str_str_str_locale(language: String, script: String, country: String, variant: String, extensions: Object) -> Result<Object> {
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = Locale::getCompatibilityExtensions(language, script, country, variant)?;
        extensions = _t0;
        let _t1: String = Locale::convertOldISOCodes(language)?;
        let _t2: Object = BaseLocale::getInstance(_t1, script, country, variant)?;
        let mut baseloc: Object = _t2;
        let _t3: Object = Locale::getInstance(baseloc, extensions)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getInstance", descriptor = "(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;", access = "static"))]
    // java: getInstance(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;
    pub fn getInstance__baselo_locale(baseloc: Object, extensions: Object) -> Result<Object> {
        let _t0 = Locale::CONSTANT_LOCALES().get(baseloc)?;
        let mut locale: Object = _t0;
        return Ok(locale);
        let _t1 = Locale$Cache::LOCALECACHE().get(baseloc)?;
        return Ok(_t1);
        locale = Locale_LocaleKey::new(baseloc, extensions)?;
        let _t2 = Locale$Cache::LOCALECACHE().get(locale)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getDefault", descriptor = "()Ljava/util/Locale;", access = "public static"))]
    // java: getDefault()Ljava/util/Locale;
    pub fn getDefault() -> Result<Object> {
        Ok(Locale::defaultLocale())
    }

    #[cfg_attr(any(), java_method(name = "getDefault", descriptor = "(Ljava/util/Locale$Category;)Ljava/util/Locale;", access = "public static"))]
    // java: getDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;
    pub fn getDefault__locale(category: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(category)?;
        let mut loc: Object = Locale::defaultDisplayLocale();
        let _t1: Object = Locale::getDisplayLocale()?;
        loc = _t1;
        return Ok(loc);
        return Err(JvmError::Custom(String::from("athrow")));
        loc = Locale::defaultFormatLocale();
        let _t2: Object = Locale::getFormatLocale()?;
        loc = _t2;
        Ok(loc)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayLocale", descriptor = "()Ljava/util/Locale;", access = "private static"))]
    pub fn getDisplayLocale() -> Result<Object> {
        let mut loc: Object = Locale::defaultDisplayLocale();
        let _t0: Object = Locale::initDefault(Locale$Category::DISPLAY())?;
        Locale::defaultDisplayLocale(_t0);
        loc = _t0;
        Ok(loc)
    }

    #[cfg_attr(any(), java_method(name = "getFormatLocale", descriptor = "()Ljava/util/Locale;", access = "private static"))]
    pub fn getFormatLocale() -> Result<Object> {
        let mut loc: Object = Locale::defaultFormatLocale();
        let _t0: Object = Locale::initDefault(Locale$Category::FORMAT())?;
        Locale::defaultFormatLocale(_t0);
        loc = _t0;
        Ok(loc)
    }

    #[cfg_attr(any(), java_method(name = "initDefault", descriptor = "()Ljava/util/Locale;", access = "private static"))]
    // java: initDefault()Ljava/util/Locale;
    pub fn initDefault() -> Result<Object> {
        let _t0: Object = Locale::getDefaultExtensions(StaticProperty::USER_EXTENSIONS())?;
        /* TODO: aconst_null  */
        let _t1 = StaticProperty::USER_VARIANT().orElse(_t0)?;
        let _t2: Object = Locale::getInstance(todo!("stack underflow"), StaticProperty::USER_LANGUAGE(), StaticProperty::USER_SCRIPT(), StaticProperty::USER_COUNTRY(), _t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "initDefault", descriptor = "(Ljava/util/Locale$Category;)Ljava/util/Locale;", access = "private static"))]
    // java: initDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;
    pub fn initDefault__locale(category: Object) -> Result<Object> {
        let mut locale: Object = Locale::defaultLocale();
        let _t0: Object = Locale::getDefaultExtensions(StaticProperty::USER_EXTENSIONS_FORMAT())?;
        let _t1 = locale.getLocaleExtensions()?;
        let _t2 = _t0.orElse(_t1)?;
        let _t3: Object = Locale::getInstance(StaticProperty::USER_VARIANT_FORMAT(), category, Locale$Category::DISPLAY(), StaticProperty::USER_EXTENSIONS_DISPLAY(), _t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getDefaultExtensions", descriptor = "(Ljava/lang/String;)Ljava/util/Optional;", access = "private static"))]
    pub fn getDefaultExtensions(extensionsProp: String) -> Result<Object> {
        let _t0: bool = LocaleUtils::isEmpty(extensionsProp)?;
        let _t1: Object = Optional::empty()?;
        return Ok(_t1);
        /* TODO: aconst_null  */
        let mut exts: bool = _t0;
        let _t2 = InternalLocaleBuilder::new()?.setExtensions(extensionsProp)?;
        let _t3 = _t2.getLocaleExtensions()?;
        exts = _t3;
        let mut local_2: i32 = todo!("stack underflow");
        let _t4: Object = Optional::ofNullable(exts)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "setDefault", descriptor = "(Ljava/util/Locale;)V", access = "public static"))]
    // java: setDefault(Ljava/util/Locale;)V
    pub fn setDefault__locale(newLocale: Object) -> Result<()> {
        Locale::setDefault(Locale$Category::DISPLAY(), newLocale)?;
        Locale::setDefault(Locale$Category::FORMAT(), newLocale)?;
        Locale::defaultLocale(newLocale);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "setDefault", descriptor = "(Ljava/util/Locale$Category;Ljava/util/Locale;)V", access = "public static"))]
    // java: setDefault(Ljava/util/Locale$Category;Ljava/util/Locale;)V
    pub fn setDefault__locale_locale(category: Object, newLocale: Object) -> Result<()> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(PropertyPermission::new(String::from("user.language"), String::from("write"))?)?;
        let _t1 = category.ordinal()?;
        /* TODO: lookupswitch default:94 0:80 1:87 */
        Locale::defaultDisplayLocale(newLocale);
        Locale::defaultFormatLocale(newLocale);
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public static"))]
    pub fn getAvailableLocales() -> Result<Vec<Object>> {
        let _t0: Vec<Object> = LocaleServiceProviderPool::getAllAvailableLocales()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "availableLocales", descriptor = "()Ljava/util/stream/Stream;", access = "public static"))]
    pub fn availableLocales() -> Result<Object> {
        let _t0: Object = LocaleServiceProviderPool::streamAllAvailableLocales()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getISOCountries", descriptor = "()[Ljava/lang/String;", access = "public static"))]
    // java: getISOCountries()[Ljava/lang/String;
    pub fn getISOCountries() -> Result<Vec<String>> {
        let _t0: Vec<String> = Locale::getISO2Table(String::from("ADANDAEAREAFAFGAGATGAIAIAALALBAMARMAOAGOAQATAARARGASASMATAUTAUAUSAWABWAXALAAZAZEBABIHBBBRBBDBGDBEBELBFBFABGBGRBHBHRBIBDIBJBENBLBLMBMBMUBNBRNBOBOLBQBESBRBRABSBHSBTBTNBVBVTBWBWABYBLRBZBLZCACANCCCCKCDCODCFCAFCGCOGCHCHECICIVCKCOKCLCHLCMCMRCNCHNCOCOLCRCRICUCUBCVCPVCWCUWCXCXRCYCYPCZCZEDEDEUDJDJIDKDNKDMDMADODOMDZDZAECECUEEESTEGEGYEHESHERERIESESPETETHFIFINFJFJIFKFLKFMFSMFOFROFRFRAGAGABGBGBRGDGRDGEGEOGFGUFGGGGYGHGHAGIGIBGLGRLGMGMBGNGINGPGLPGQGNQGRGRCGSSGSGTGTMGUGUMGWGNBGYGUYHKHKGHMHMDHNHNDHRHRVHTHTIHUHUNIDIDNIEIRLILISRIMIMNININDIOIOTIQIRQIRIRNISISLITITAJEJEYJMJAMJOJORJPJPNKEKENKGKGZKHKHMKIKIRKMCOMKNKNAKPPRKKRKORKWKWTKYCYMKZKAZLALAOLBLBNLCLCALILIELKLKALRLBRLSLSOLTLTULULUXLVLVALYLBYMAMARMCMCOMDMDAMEMNEMFMAFMGMDGMHMHLMKMKDMLMLIMMMMRMNMNGMOMACMPMNPMQMTQMRMRTMSMSRMTMLTMUMUSMVMDVMWMWIMXMEXMYMYSMZMOZNANAMNCNCLNENERNFNFKNGNGANINICNLNLDNONORNPNPLNRNRUNUNIUNZNZLOMOMNPAPANPEPERPFPYFPGPNGPHPHLPKPAKPLPOLPMSPMPNPCNPRPRIPSPSEPTPRTPWPLWPYPRYQAQATREREUROROURSSRBRURUSRWRWASASAUSBSLBSCSYCSDSDNSESWESGSGPSHSHNSISVNSJSJMSKSVKSLSLESMSMRSNSENSOSOMSRSURSSSSDSTSTPSVSLVSXSXMSYSYRSZSWZTCTCATDTCDTFATFTGTGOTHTHATJTJKTKTKLTLTLSTMTKMTNTUNTOTONTRTURTTTTOTVTUVTWTWNTZTZAUAUKRUGUGAUMUMIUSUSAUYURYUZUZBVAVATVCVCTVEVENVGVGBVIVIRVNVNMVUVUTWFWLFWSWSMYEYEMYTMYTZAZAFZMZMBZWZWE"))?;
        Locale::isoCountries(_t0);
        let mut _arr1: Vec<Object> = Vec::with_capacity((Locale::isoCountries().len() as i32) as usize);
        let mut result: Vec<Object> = _arr1;
        System::arraycopy(&Locale::isoCountries(), 0i32, &result, 0i32, (Locale::isoCountries().len() as i32))?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "getISOCountries", descriptor = "(Ljava/util/Locale$IsoCountryCode;)Ljava/util/Set;", access = "public static"))]
    // java: getISOCountries(Ljava/util/Locale$IsoCountryCode;)Ljava/util/Set;
    pub fn getISOCountries__locale(type_: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(type_)?;
        let _t1: Object = Locale$IsoCountryCode::retrieveISOCountryCodes(type_)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getISOLanguages", descriptor = "()[Ljava/lang/String;", access = "public static"))]
    pub fn getISOLanguages() -> Result<Vec<String>> {
        let mut languages: Vec<String> = Locale::isoLanguages();
        let _t0: Vec<String> = Locale::getISO2Table(String::from("aaaarababkaeaveafafrakakaamamhanargararaasasmavavaayaymazazebabakbebelbgbulbhbihbibisbmbambnbenbobodbrbrebsboscacatcechechchacocoscrcrecscescuchucvchvcycymdadandedeudvdivdzdzoeeeweelellenengeoepoesspaetesteueusfafasfffulfifinfjfijfofaofrfrafyfrygaglegdglaglglggngrngugujgvglvhahauhehebhihinhohmohrhrvhthathuhunhyhyehzheriainaidindieileigiboiiiiiikipkinindioidoisislititaiuikuiwhebjajpnjiyidjvjavkakatkgkonkikikkjkuakkkazklkalkmkhmknkankokorkrkaukskaskukurkvkomkwcorkykirlalatlbltzlgluglilimlnlinlolaoltlitlulublvlavmgmlgmhmahmimrimkmkdmlmalmnmonmomolmrmarmsmsamtmltmymyananaunbnobndndenenepngndonlnldnnnnononornrnblnvnavnynyaocociojojiomormororiososspapanpipliplpolpspusptporququermrohrnrunroronrurusrwkinsasanscsrdsdsndsesmesgsagsisinskslkslslvsmsmosnsnasosomsqsqisrsrpsssswstsotsusunsvsweswswatatamteteltgtgkththatitirtktuktltgltntsntotontrturtstsotttattwtwitytahuguigukukrururduzuzbvevenvivievovolwawlnwowolxhxhoyiyidyoyorzazhazhzhozuzul"))?;
        languages = _t0;
        Locale::isoLanguages(_t0);
        let mut _arr1: Vec<Object> = Vec::with_capacity((languages.len() as i32) as usize);
        let mut result: Vec<Object> = _arr1;
        System::arraycopy(&languages, 0i32, &result, 0i32, (languages.len() as i32))?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "getISO2Table", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "private static"))]
    pub fn getISO2Table(table: String) -> Result<Vec<String>> {
        let _t0 = table.length()?;
        let mut len: i32 = (_t0/5i32);
        let mut _arr1: Vec<Object> = Vec::with_capacity(len as usize);
        let mut isoTable: Vec<Object> = _arr1;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0 = table.substring(j, (j).wrapping_add(2i32))?;
            isoTable[i as usize] = _t0;
            i = i.wrapping_add(1i32);
            j = j.wrapping_add(5i32);
        }
        Ok(isoTable)
    }

    #[cfg_attr(any(), java_method(name = "getLanguage", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getLanguage(&self) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getLanguage()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getScript", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getScript(&self) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getScript()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getCountry", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getCountry(&self) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getRegion()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getVariant", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getVariant(&self) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getVariant()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "hasExtensions", descriptor = "()Z", access = "public"))]
    pub fn hasExtensions(&self) -> Result<bool> {
        let this = self;
        Ok(!this.localeExtensions.get().is_none())
    }

    #[cfg_attr(any(), java_method(name = "stripExtensions", descriptor = "()Ljava/util/Locale;", access = "public"))]
    pub fn stripExtensions(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.hasExtensions()?;
        /* TODO: aconst_null  */
        let _t1: Object = Locale::getInstance(_t0, this.baseLocale.get())?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getExtension", descriptor = "(C)Ljava/lang/String;", access = "public"))]
    pub fn getExtension(&self, key: u16) -> Result<String> {
        let this = self;
        let _t0: bool = LocaleExtensions::isValidKey(key)?;
        String::new().append(&String::from("Ill-formed extension key:"))?;
        String::new().append(&key)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.hasExtensions()?;
        let _t2: Object = Character::valueOf(key)?;
        let _t3 = this.localeExtensions.get().getExtensionValue(_t2)?;
        /* TODO: aconst_null  */
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getExtensionKeys", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn getExtensionKeys(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.hasExtensions()?;
        let _t1: Object = Collections::emptySet()?;
        return Ok(_t1);
        let _t2 = this.localeExtensions.get().getKeys()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getUnicodeLocaleAttributes", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn getUnicodeLocaleAttributes(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.hasExtensions()?;
        let _t1: Object = Collections::emptySet()?;
        return Ok(_t1);
        let _t2 = this.localeExtensions.get().getUnicodeLocaleAttributes()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getUnicodeLocaleType", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public"))]
    pub fn getUnicodeLocaleType(&self, key: String) -> Result<String> {
        let this = self;
        let _t0: bool = Locale::isUnicodeExtensionKey(key)?;
        String::new().append(&String::from("Ill-formed Unicode locale key:"))?;
        String::new().append(&key)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.hasExtensions()?;
        let _t2 = this.localeExtensions.get().getUnicodeLocaleType(key)?;
        /* TODO: aconst_null  */
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getUnicodeLocaleKeys", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn getUnicodeLocaleKeys(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::emptySet()?;
        return Ok(_t0);
        let _t1 = this.localeExtensions.get().getUnicodeLocaleKeys()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getBaseLocale", descriptor = "()Lsun/util/locale/BaseLocale;"))]
    pub fn getBaseLocale(&self) -> Result<Object> {
        let this = self;
        Ok(this.baseLocale.get())
    }

    #[cfg_attr(any(), java_method(name = "getLocaleExtensions", descriptor = "()Lsun/util/locale/LocaleExtensions;"))]
    pub fn getLocaleExtensions(&self) -> Result<Object> {
        let this = self;
        Ok(this.localeExtensions.get())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public final"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getLanguage()?;
        let _t1 = _t0.isEmpty()?;
        let mut l: i32 = _t1==0i32;
        let _t2 = this.baseLocale.get().getScript()?;
        let _t3 = _t2.isEmpty()?;
        let mut s: i32 = _t3==0i32;
        let _t4 = this.baseLocale.get().getRegion()?;
        let _t5 = _t4.isEmpty()?;
        let mut r: i32 = _t5==0i32;
        let _t6 = this.baseLocale.get().getVariant()?;
        let _t7 = _t6.isEmpty()?;
        let mut v: i32 = _t7==0i32;
        let _t8 = this.localeExtensions.get().getID()?;
        let _t9 = _t8.isEmpty()?;
        let mut e: i32 = _t9==0i32;
        let _t10 = this.baseLocale.get().getLanguage()?;
        let mut result: String = String::new();
        result.append(&95i32)?;
        let _t11 = this.baseLocale.get().getRegion()?;
        result.append(&_t11)?;
        result.append(&95i32)?;
        let _t12 = this.baseLocale.get().getVariant()?;
        result.append(&_t12)?;
        result.append(&String::from("_#"))?;
        let _t13 = this.baseLocale.get().getScript()?;
        result.append(&_t13)?;
        result.append(&95i32)?;
        result.append(&35i32)?;
        let _t14 = this.localeExtensions.get().getID()?;
        result.append(&_t14)?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "toLanguageTag", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toLanguageTag(&self) -> Result<String> {
        let this = self;
        let mut lTag: String = this.languageTag.get();
        return Ok(lTag);
        let _t0: Object = LanguageTag::parseLocale(this.baseLocale.get(), this.localeExtensions.get())?;
        let mut tag: Object = _t0;
        let mut buf: String = String::new();
        let _t1 = tag.getLanguage()?;
        let mut subtag: String = _t1;
        let _t2 = subtag.isEmpty()?;
        let _t3: String = LanguageTag::canonicalizeLanguage(subtag)?;
        buf.append(&_t3)?;
        let _t4 = tag.getScript()?;
        subtag = _t4;
        let _t5 = subtag.isEmpty()?;
        buf.append(&String::from("-"))?;
        let _t6: String = LanguageTag::canonicalizeScript(subtag)?;
        buf.append(&_t6)?;
        let _t7 = tag.getRegion()?;
        subtag = _t7;
        let _t8 = subtag.isEmpty()?;
        buf.append(&String::from("-"))?;
        let _t9: String = LanguageTag::canonicalizeRegion(subtag)?;
        buf.append(&_t9)?;
        let _t10 = tag.getVariants()?;
        let mut subtags: Object = _t10;
        let _t11 = subtags.iterator()?;
        let mut langTag: Object = _t11;
        loop {
            let _t0 = langTag.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = langTag.next()?;
            let mut s: Object = _t0;
            buf.append(&String::from("-"))?;
            buf.append(&s)?;
        }
        let _t12 = tag.getExtensions()?;
        subtags = _t12;
        let _t13 = subtags.iterator()?;
        langTag = _t13;
        loop {
            let _t0 = langTag.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = langTag.next()?;
            s = _t0;
            buf.append(&String::from("-"))?;
            let _t1: String = LanguageTag::canonicalizeExtension(s)?;
            buf.append(&_t1)?;
        }
        let _t14 = tag.getPrivateuse()?;
        subtag = _t14;
        let _t15 = subtag.isEmpty()?;
        let _t16 = buf.length()?;
        buf.append(&String::from("-"))?;
        buf.append(&String::from("x"))?;
        buf.append(&String::from("-"))?;
        buf.append(&subtag)?;
        langTag = buf;
        s = this;
        /* TODO: monitorenter  */
        this.languageTag.set(langTag);
        /* TODO: monitorexit  */
        let mut local_8: Object = s;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(langTag)
    }

    #[cfg_attr(any(), java_method(name = "caseFoldLanguageTag", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    pub fn caseFoldLanguageTag(languageTag: String) -> Result<String> {
        let _t0: String = LanguageTag::caseFoldTag(languageTag)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "forLanguageTag", descriptor = "(Ljava/lang/String;)Ljava/util/Locale;", access = "public static"))]
    pub fn forLanguageTag(languageTag: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let _t0: Object = LanguageTag::parse(todo!("stack underflow"), languageTag)?;
        let mut tag: Object = _t0;
        let mut bldr: InternalLocaleBuilder = InternalLocaleBuilder::new()?;
        let _t1 = bldr.setLanguageTag(tag)?;
        let _t2 = bldr.getBaseLocale()?;
        let mut base: Object = _t2;
        let _t3 = bldr.getLocaleExtensions()?;
        let mut exts: Object = _t3;
        let _t4 = base.getVariant()?;
        let _t5 = _t4.isEmpty()?;
        let _t6 = base.getLanguage()?;
        let _t7 = base.getScript()?;
        let _t8 = base.getRegion()?;
        let _t9 = base.getVariant()?;
        let _t10: Object = Locale::getCompatibilityExtensions(_t6, _t7, _t8, _t9)?;
        exts = _t10;
        let _t11: Object = Locale::getInstance(base, exts)?;
        Ok(_t11)
    }

    #[cfg_attr(any(), java_method(name = "getISO3Language", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getISO3Language(&self) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getLanguage()?;
        let mut lang: String = _t0;
        let _t1 = lang.length()?;
        return Ok(lang);
        let _t2: String = Locale::getISO3Code(lang, String::from("aaaarababkaeaveafafrakakaamamhanargararaasasmavavaayaymazazebabakbebelbgbulbhbihbibisbmbambnbenbobodbrbrebsboscacatcechechchacocoscrcrecscescuchucvchvcycymdadandedeudvdivdzdzoeeeweelellenengeoepoesspaetesteueusfafasfffulfifinfjfijfofaofrfrafyfrygaglegdglaglglggngrngugujgvglvhahauhehebhihinhohmohrhrvhthathuhunhyhyehzheriainaidindieileigiboiiiiiikipkinindioidoisislititaiuikuiwhebjajpnjiyidjvjavkakatkgkonkikikkjkuakkkazklkalkmkhmknkankokorkrkaukskaskukurkvkomkwcorkykirlalatlbltzlgluglilimlnlinlolaoltlitlulublvlavmgmlgmhmahmimrimkmkdmlmalmnmonmomolmrmarmsmsamtmltmymyananaunbnobndndenenepngndonlnldnnnnononornrnblnvnavnynyaocociojojiomormororiososspapanpipliplpolpspusptporququermrohrnrunroronrurusrwkinsasanscsrdsdsndsesmesgsagsisinskslkslslvsmsmosnsnasosomsqsqisrsrpsssswstsotsusunsvsweswswatatamteteltgtgkththatitirtktuktltgltntsntotontrturtstsotttattwtwitytahuguigukukrururduzuzbvevenvivievovolwawlnwowolxhxhoyiyidyoyorzazhazhzhozuzul"))?;
        let mut language3: String = _t2;
        String::new().append(&String::from("Couldn't find 3-letter language code for"))?;
        String::new().append(&lang)?;
        String::new().append(&String::from("FormatData_"))?;
        let _t3 = this.toString()?;
        String::new().append(&_t3)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(language3)
    }

    #[cfg_attr(any(), java_method(name = "getISO3Country", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getISO3Country(&self) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getRegion()?;
        let _t1: String = Locale::getISO3Code(_t0, String::from("ADANDAEAREAFAFGAGATGAIAIAALALBAMARMAOAGOAQATAARARGASASMATAUTAUAUSAWABWAXALAAZAZEBABIHBBBRBBDBGDBEBELBFBFABGBGRBHBHRBIBDIBJBENBLBLMBMBMUBNBRNBOBOLBQBESBRBRABSBHSBTBTNBVBVTBWBWABYBLRBZBLZCACANCCCCKCDCODCFCAFCGCOGCHCHECICIVCKCOKCLCHLCMCMRCNCHNCOCOLCRCRICUCUBCVCPVCWCUWCXCXRCYCYPCZCZEDEDEUDJDJIDKDNKDMDMADODOMDZDZAECECUEEESTEGEGYEHESHERERIESESPETETHFIFINFJFJIFKFLKFMFSMFOFROFRFRAGAGABGBGBRGDGRDGEGEOGFGUFGGGGYGHGHAGIGIBGLGRLGMGMBGNGINGPGLPGQGNQGRGRCGSSGSGTGTMGUGUMGWGNBGYGUYHKHKGHMHMDHNHNDHRHRVHTHTIHUHUNIDIDNIEIRLILISRIMIMNININDIOIOTIQIRQIRIRNISISLITITAJEJEYJMJAMJOJORJPJPNKEKENKGKGZKHKHMKIKIRKMCOMKNKNAKPPRKKRKORKWKWTKYCYMKZKAZLALAOLBLBNLCLCALILIELKLKALRLBRLSLSOLTLTULULUXLVLVALYLBYMAMARMCMCOMDMDAMEMNEMFMAFMGMDGMHMHLMKMKDMLMLIMMMMRMNMNGMOMACMPMNPMQMTQMRMRTMSMSRMTMLTMUMUSMVMDVMWMWIMXMEXMYMYSMZMOZNANAMNCNCLNENERNFNFKNGNGANINICNLNLDNONORNPNPLNRNRUNUNIUNZNZLOMOMNPAPANPEPERPFPYFPGPNGPHPHLPKPAKPLPOLPMSPMPNPCNPRPRIPSPSEPTPRTPWPLWPYPRYQAQATREREUROROURSSRBRURUSRWRWASASAUSBSLBSCSYCSDSDNSESWESGSGPSHSHNSISVNSJSJMSKSVKSLSLESMSMRSNSENSOSOMSRSURSSSSDSTSTPSVSLVSXSXMSYSYRSZSWZTCTCATDTCDTFATFTGTGOTHTHATJTJKTKTKLTLTLSTMTKMTNTUNTOTONTRTURTTTTOTVTUVTWTWNTZTZAUAUKRUGUGAUMUMIUSUSAUYURYUZUZBVAVATVCVCTVEVENVGVGBVIVIRVNVNMVUVUTWFWLFWSWSMYEYEMYTMYTZAZAFZMZMBZWZWE"))?;
        let mut country3: String = _t1;
        String::new().append(&String::from("Couldn't find 3-letter country code for"))?;
        let _t2 = this.baseLocale.get().getRegion()?;
        String::new().append(&_t2)?;
        String::new().append(&String::from("FormatData_"))?;
        let _t3 = this.toString()?;
        String::new().append(&_t3)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(country3)
    }

    #[cfg_attr(any(), java_method(name = "getISO3Code", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "private static"))]
    pub fn getISO3Code(iso2Code: String, table: String) -> Result<String> {
        let _t0 = iso2Code.length()?;
        let mut codeLength: i32 = _t0;
        return Ok(String::from(""));
        let _t1 = table.length()?;
        let mut tableLength: i32 = _t1;
        let mut index: i32 = tableLength;
        let _t2 = iso2Code.charAt(0i32)?;
        let mut c1: i32 = _t2;
        let _t3 = iso2Code.charAt(1i32)?;
        let mut c2: i32 = _t3;
        index = 0i32;
        loop {
            if index >= tableLength { break; }
            let _t0 = table.charAt(index)?;
            let _t1 = table.charAt((index).wrapping_add(1i32))?;
            index = index.wrapping_add(5i32);
        }
        let _t4 = table.substring((index).wrapping_add(2i32), (index).wrapping_add(5i32))?;
        /* TODO: aconst_null  */
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayLanguage", descriptor = "()Ljava/lang/String;", access = "public final"))]
    // java: getDisplayLanguage()Ljava/lang/String;
    pub fn getDisplayLanguage(&self) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault(Locale$Category::DISPLAY())?;
        let _t1 = this.getDisplayLanguage(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayLanguage", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public"))]
    // java: getDisplayLanguage(Ljava/util/Locale;)Ljava/lang/String;
    pub fn getDisplayLanguage__locale(&self, inLocale: Object) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getLanguage()?;
        /* TODO: aconst_null  */
        let _t1 = todo!("stack underflow").getDisplayString(this, _t0, inLocale, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayScript", descriptor = "()Ljava/lang/String;", access = "public"))]
    // java: getDisplayScript()Ljava/lang/String;
    pub fn getDisplayScript(&self) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault(Locale$Category::DISPLAY())?;
        let _t1 = this.getDisplayScript(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayScript", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public"))]
    // java: getDisplayScript(Ljava/util/Locale;)Ljava/lang/String;
    pub fn getDisplayScript__locale(&self, inLocale: Object) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getScript()?;
        /* TODO: aconst_null  */
        let _t1 = todo!("stack underflow").getDisplayString(this, _t0, inLocale, 3i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayCountry", descriptor = "()Ljava/lang/String;", access = "public final"))]
    // java: getDisplayCountry()Ljava/lang/String;
    pub fn getDisplayCountry(&self) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault(Locale$Category::DISPLAY())?;
        let _t1 = this.getDisplayCountry(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayCountry", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public"))]
    // java: getDisplayCountry(Ljava/util/Locale;)Ljava/lang/String;
    pub fn getDisplayCountry__locale(&self, inLocale: Object) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getRegion()?;
        /* TODO: aconst_null  */
        let _t1 = todo!("stack underflow").getDisplayString(this, _t0, inLocale, 1i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayString", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;I)Ljava/lang/String;", access = "private"))]
    pub fn getDisplayString(&self, code: String, cat: String, inLocale: Object, type_: i32) -> Result<String> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(inLocale)?;
        let _t1: Object = Objects::requireNonNull(code)?;
        let _t2 = code.isEmpty()?;
        return Ok(String::from(""));
        let _t3: Object = LocaleServiceProviderPool::getPool(481i32)?;
        let mut pool: Object = _t3;
        String::new().append(&String::from("%%"))?;
        String::new().append(&code)?;
        let mut rbKey: String = code;
        let mut _arr4: Vec<Object> = Vec::with_capacity(3i32 as usize);
        _arr4[0i32 as usize] = type_;
        _arr4[1i32 as usize] = code;
        _arr4[2i32 as usize] = cat;
        let _t5 = pool.getLocalizedObject(Locale$LocaleNameGetter::INSTANCE(), inLocale, rbKey, _arr4)?;
        let mut result: Object = _t5;
        Ok(code)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayVariant", descriptor = "()Ljava/lang/String;", access = "public final"))]
    // java: getDisplayVariant()Ljava/lang/String;
    pub fn getDisplayVariant(&self) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault(Locale$Category::DISPLAY())?;
        let _t1 = this.getDisplayVariant(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayVariant", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public"))]
    // java: getDisplayVariant(Ljava/util/Locale;)Ljava/lang/String;
    pub fn getDisplayVariant__locale(&self, inLocale: Object) -> Result<String> {
        let this = self;
        let _t0 = this.baseLocale.get().getVariant()?;
        let _t1 = _t0.isEmpty()?;
        return Ok(String::from(""));
        let _t2: Object = LocaleProviderAdapter::getResourceBundleBased()?;
        let _t3 = _t2.getLocaleResources(inLocale)?;
        let mut lr: Object = _t3;
        let _t4 = this.getDisplayVariantArray(inLocale)?;
        let mut names: Vec<String> = _t4;
        let _t5 = lr.getLocaleName(String::from("ListCompositionPattern"))?;
        let _t6: String = Locale::formatList(&names, _t5)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayName", descriptor = "()Ljava/lang/String;", access = "public final"))]
    // java: getDisplayName()Ljava/lang/String;
    pub fn getDisplayName(&self) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault(Locale$Category::DISPLAY())?;
        let _t1 = this.getDisplayName(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayName", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public"))]
    // java: getDisplayName(Ljava/util/Locale;)Ljava/lang/String;
    pub fn getDisplayName__locale(&self, inLocale: Object) -> Result<String> {
        let this = self;
        let _t0: Object = LocaleProviderAdapter::getResourceBundleBased()?;
        let _t1 = _t0.getLocaleResources(inLocale)?;
        let mut lr: Object = _t1;
        let _t2 = this.getDisplayLanguage(inLocale)?;
        let mut languageName: String = _t2;
        let _t3 = this.getDisplayScript(inLocale)?;
        let mut scriptName: String = _t3;
        let _t4 = this.getDisplayCountry(inLocale)?;
        let mut countryName: String = _t4;
        let _t5 = this.getDisplayVariantArray(inLocale)?;
        let mut variantNames: Vec<String> = _t5;
        let _t6 = lr.getLocaleName(String::from("DisplayNamePattern"))?;
        let mut displayNamePattern: String = _t6;
        let _t7 = lr.getLocaleName(String::from("ListCompositionPattern"))?;
        let mut listCompositionPattern: String = _t7;
        let _t8 = languageName.isEmpty()?;
        let _t9 = scriptName.isEmpty()?;
        let _t10 = countryName.isEmpty()?;
        return Ok(String::from(""));
        let _t11: String = Locale::formatList(&variantNames, listCompositionPattern)?;
        return Ok(_t11);
        let mut names: ArrayList<_> = ArrayList::<_>::new()?;
        let _t12 = languageName.isEmpty()?;
        let _t13 = names.add(languageName)?;
        let _t14 = scriptName.isEmpty()?;
        let _t15 = names.add(scriptName)?;
        let _t16 = countryName.isEmpty()?;
        let _t17 = names.add(countryName)?;
        let _t18: Object = Arrays::asList(&variantNames)?;
        let _t19 = names.addAll(_t18)?;
        let _t20 = this.localeExtensions.get().getUnicodeLocaleAttributes()?;
        let _t21 = _t20.stream()?;
        /* TODO: invokedynamic 561 */
        let _t22 = this.map(inLocale)?;
        let _t23: Object = Objects::requireNonNull(names)?;
        /* TODO: invokedynamic 571 */
        _t22.forEach(names)?;
        let _t24 = this.localeExtensions.get().getUnicodeLocaleKeys()?;
        let _t25 = _t24.stream()?;
        /* TODO: invokedynamic 579 */
        let _t26 = lr.map(inLocale)?;
        let _t27: Object = Objects::requireNonNull(names)?;
        /* TODO: invokedynamic 571 */
        _t26.forEach(names)?;
        let _t28 = names.get(0i32)?;
        let mut mainName: Object = _t28;
        let _t29 = names.size()?;
        let mut numNames: i32 = _t29;
        let _t30 = names.subList(1i32, numNames)?;
        let mut _arr31: Vec<Object> = Vec::with_capacity((numNames).wrapping_sub(1i32) as usize);
        let _t32 = _t30.toArray(_arr31)?;
        let mut _arr33: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let mut qualifierNames: Vec<Object> = _arr33;
        let mut _arr34: Vec<Object> = Vec::with_capacity(3i32 as usize);
        _arr34[0i32 as usize] = (qualifierNames.len() as i32)==0i32;
        _arr34[1i32 as usize] = mainName;
        let _t35: String = Locale::formatList(&qualifierNames, listCompositionPattern)?;
        /* TODO: aconst_null  */
        2i32[(qualifierNames.len() as i32) as usize] = _t35;
        let mut displayNames: Vec<Object> = _arr34;
        let _t36 = MessageFormat::new(displayNamePattern)?.format(displayNames)?;
        return Ok(_t36);
        let mut result: String = String::new();
        result.append(&displayNames[1i32 as usize].clone())?;
        result.append(&String::from("("))?;
        result.append(&displayNames[2i32 as usize].clone())?;
        result.append(&41i32)?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn clone(&self) -> Result<Object> {
        let this = self;
        let mut that: java/util/Locale = this;
        return Ok(that);
        that = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut hc: i32 = this.hashCodeValue.get();
        let _t0 = this.baseLocale.get().hashCode()?;
        hc = _t0;
        let _t1 = this.localeExtensions.get().hashCode()?;
        hc = (hc^_t1);
        this.hashCodeValue.set(hc);
        Ok(hc)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let mut otherBase: Object = obj.baseLocale.get();
        let _t0 = this.baseLocale.get().equals(otherBase)?;
        return Ok(0i32);
        return Ok(obj.localeExtensions.get().is_none());
        let _t1 = this.localeExtensions.get().equals(obj.localeExtensions.get())?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayVariantArray", descriptor = "(Ljava/util/Locale;)[Ljava/lang/String;", access = "private"))]
    pub fn getDisplayVariantArray(&self, inLocale: Object) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.baseLocale.get().getVariant()?;
        let mut tokenizer: StringTokenizer = StringTokenizer::new(_t0, String::from("_"))?;
        let _t1 = tokenizer.countTokens()?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(_t1 as usize);
        let mut names: Vec<Object> = _arr2;
        let mut i: i32 = 0i32;
        loop {
            if i >= (names.len() as i32) { break; }
            let _t0 = tokenizer.nextToken()?;
            /* TODO: aconst_null  */
            let _t1 = i.getDisplayString(this, _t0, inLocale, 2i32)?;
            todo!("stack underflow")[names as usize] = _t1;
            i = i.wrapping_add(1i32);
        }
        Ok(names)
    }

    #[cfg_attr(any(), java_method(name = "getDisplayKeyTypeExtensionString", descriptor = "(Ljava/lang/String;Lsun/util/locale/provider/LocaleResources;Ljava/util/Locale;)Ljava/lang/String;", access = "private"))]
    pub fn getDisplayKeyTypeExtensionString(&self, key: String, lr: Object, inLocale: Object) -> Result<String> {
        let this = self;
        let _t0 = this.localeExtensions.get().getUnicodeLocaleType(key)?;
        let mut type_: String = _t0;
        let _t1 = this.getDisplayString(type_, key, inLocale, 5i32)?;
        let mut ret: String = _t1;
        let _t2 = ret.equals(type_)?;
        let mut displayType: String = type_;
        let mut local_7: String = key;
        let mut local_8: i32 = -1i32;
        let _t3 = local_7.hashCode()?;
        /* TODO: lookupswitch default:132 3186:84 3637:101 3718:118 */
        let _t4 = local_7.equals(String::from("cu"))?;
        local_8 = 0i32;
        let _t5 = local_7.equals(String::from("rg"))?;
        local_8 = 1i32;
        let _t6 = local_7.equals(String::from("tz"))?;
        local_8 = 2i32;
        /* TODO: tableswitch default:239 low:0 high:2 */
        let _t7 = type_.toLowerCase(Locale::ROOT())?;
        let _t8 = lr.getCurrencyName(_t7)?;
        displayType = _t8;
        let _t9 = type_.matches(String::from("^[a-zA-Z]{2}[zZ]{4}$"))?;
        let _t10 = type_.substring(0i32, 2i32)?;
        let _t11 = _t10.toUpperCase(Locale::ROOT())?;
        let _t12 = lr.getLocaleName(_t11)?;
        displayType = _t12;
        let _t13: Object = TimeZoneNameUtility::convertLDMLShortID(type_)?;
        /* TODO: invokedynamic 669 */
        let _t14 = _t13.map(inLocale)?;
        let _t15 = _t14.orElse(type_)?;
        displayType = _t15;
        let _t16 = lr.getLocaleName(String::from("ListKeyTypePattern"))?;
        let mut _arr17: Vec<Object> = Vec::with_capacity(2i32 as usize);
        /* TODO: aconst_null  */
        let _t18 = 0i32.getDisplayString(this, key, inLocale, 4i32)?;
        _arr17[_arr17 as usize] = _t18;
        let _t19: Object = Optional::ofNullable(displayType)?;
        let _t20 = _t19.orElse(type_)?;
        _t16[1i32 as usize] = _t20;
        let _t21: String = MessageFormat::format(_t9, _t16)?;
        ret = _t21;
        Ok(ret)
    }

    #[cfg_attr(any(), java_method(name = "formatList", descriptor = "([Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "private static"))]
    pub fn formatList(stringList: &[String], pattern: String) -> Result<String> {
        let _t0: String = String::join(String::from(","), &stringList)?;
        return Ok(_t0);
        /* TODO: lookupswitch default:51 0:40 1:45 */
        let _t1: Object = Arrays::stream(&stringList)?;
        /* TODO: invokedynamic 689 */
        let _t2 = _t1.reduce(String::from(""), pattern)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "isUnicodeExtensionKey", descriptor = "(Ljava/lang/String;)Z", access = "private static"))]
    pub fn isUnicodeExtensionKey(s: String) -> Result<bool> {
        let _t0 = s.length()?;
        let _t1: bool = LocaleUtils::isAlphaNumericString(s)?;
        Ok(_t1!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, out: Object) -> Result<()> {
        let this = self;
        let _t0 = out.putFields()?;
        let mut fields: Object = _t0;
        let _t1 = this.baseLocale.get().getLanguage()?;
        fields.put(String::from("language"), _t1)?;
        let _t2 = this.baseLocale.get().getScript()?;
        fields.put(String::from("script"), _t2)?;
        let _t3 = this.baseLocale.get().getRegion()?;
        fields.put(String::from("country"), _t3)?;
        let _t4 = this.baseLocale.get().getVariant()?;
        fields.put(String::from("variant"), _t4)?;
        let _t5 = this.localeExtensions.get().getID()?;
        this.localeExtensions.get().put(String::from(""), _t5)?;
        fields.put(String::from("hashcode"), -1i32)?;
        out.writeFields()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, in_: Object) -> Result<()> {
        let this = self;
        let _t0 = in_.readFields()?;
        let mut fields: Object = _t0;
        let _t1 = fields.get(String::from("language"), String::from(""))?;
        let mut language: Object = _t1;
        let _t2 = fields.get(String::from("script"), String::from(""))?;
        let mut script: Object = _t2;
        let _t3 = fields.get(String::from("country"), String::from(""))?;
        let mut country: Object = _t3;
        let _t4 = fields.get(String::from("variant"), String::from(""))?;
        let mut variant: Object = _t4;
        let _t5 = fields.get(String::from("extensions"), String::from(""))?;
        let mut extStr: Object = _t5;
        let _t6: String = Locale::convertOldISOCodes(language)?;
        let _t7: Object = BaseLocale::getInstance(_t6, script, country, variant)?;
        this.baseLocale.set(_t7);
        let _t8 = extStr.isEmpty()?;
        let mut bldr: InternalLocaleBuilder = InternalLocaleBuilder::new()?;
        let _t9 = bldr.setExtensions(extStr)?;
        let _t10 = bldr.getLocaleExtensions()?;
        this.localeExtensions.set(_t10);
        bldr = _t8;
        let _t11 = bldr.getMessage()?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        todo!("stack underflow").localeExtensions.set(this);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readResolve", descriptor = "()Ljava/lang/Object;", access = "private"))]
    pub fn readResolve(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.baseLocale.get().getLanguage()?;
        let _t1 = this.baseLocale.get().getScript()?;
        let _t2 = this.baseLocale.get().getRegion()?;
        let _t3 = this.baseLocale.get().getVariant()?;
        let _t4: Object = Locale::getInstance(_t0, _t1, _t2, _t3, this.localeExtensions.get())?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "convertOldISOCodes", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private static"))]
    pub fn convertOldISOCodes(language: String) -> Result<String> {
        let _t0: String = LocaleUtils::toLowerString(language)?;
        let _t1 = _t0.intern()?;
        let _t2: String = BaseLocale::convertOldISOCodes(_t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getCompatibilityExtensions", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/LocaleExtensions;", access = "private static"))]
    pub fn getCompatibilityExtensions(language: String, script: String, country: String, variant: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let mut extensions: i32 = todo!("stack underflow");
        let _t0: bool = LocaleUtils::caseIgnoreMatch(language, String::from("ja"))?;
        let _t1 = script.isEmpty()?;
        let _t2: bool = LocaleUtils::caseIgnoreMatch(country, String::from("jp"))?;
        let _t3 = String::from("JP").equals(variant)?;
        extensions = LocaleExtensions::CALENDAR_JAPANESE();
        let _t4: bool = LocaleUtils::caseIgnoreMatch(language, String::from("th"))?;
        let _t5 = script.isEmpty()?;
        let _t6: bool = LocaleUtils::caseIgnoreMatch(country, String::from("th"))?;
        let _t7 = String::from("TH").equals(variant)?;
        extensions = LocaleExtensions::NUMBER_THAI();
        Ok(extensions)
    }

    #[cfg_attr(any(), java_method(name = "filter", descriptor = "(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;", access = "public static"))]
    // java: filter(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;
    pub fn filter__list_coll_locale(priorityList: Object, locales: Object, mode: Object) -> Result<Object> {
        let _t0: Object = LocaleMatcher::filter(priorityList, locales, mode)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "filter", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;", access = "public static"))]
    // java: filter(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;
    pub fn filter__list_coll(priorityList: Object, locales: Object) -> Result<Object> {
        let _t0: Object = Locale::filter(priorityList, locales, Locale$FilteringMode::AUTOSELECT_FILTERING())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "filterTags", descriptor = "(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;", access = "public static"))]
    // java: filterTags(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;
    pub fn filterTags__list_coll_locale(priorityList: Object, tags: Object, mode: Object) -> Result<Object> {
        let _t0: Object = LocaleMatcher::filterTags(priorityList, tags, mode)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "filterTags", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;", access = "public static"))]
    // java: filterTags(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;
    pub fn filterTags__list_coll(priorityList: Object, tags: Object) -> Result<Object> {
        let _t0: Object = Locale::filterTags(priorityList, tags, Locale$FilteringMode::AUTOSELECT_FILTERING())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "lookup", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/util/Locale;", access = "public static"))]
    pub fn lookup(priorityList: Object, locales: Object) -> Result<Object> {
        let _t0: Object = LocaleMatcher::lookup(priorityList, locales)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "lookupTag", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/lang/String;", access = "public static"))]
    pub fn lookupTag(priorityList: Object, tags: Object) -> Result<String> {
        let _t0: String = LocaleMatcher::lookupTag(priorityList, tags)?;
        Ok(_t0)
    }
}
