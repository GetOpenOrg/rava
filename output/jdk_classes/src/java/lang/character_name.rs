#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/CharacterName",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "CharacterName.java",
))]
pub struct CharacterName {
    #[cfg_attr(any(), java_field(name = "strPool", descriptor = "[B", access = "private final"))]
    pub strPool: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "lookup", descriptor = "[I", access = "private final"))]
    pub lookup: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "bkIndices", descriptor = "[I", access = "private final"))]
    pub bkIndices: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "cpEntries", descriptor = "[I", access = "private final"))]
    pub cpEntries: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "hsIndices", descriptor = "[I", access = "private final"))]
    pub hsIndices: Field<Vec<i32>>,
}

impl CharacterName {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { strPool: Field::new(Default::default()), lookup: Field::new(Default::default()), bkIndices: Field::new(Default::default()), cpEntries: Field::new(Default::default()), hsIndices: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = AccessController::doPrivileged(CharacterName_1::new(this)?)?;
        let mut dis: DataInputStream = DataInputStream::new(InflaterInputStream::new(_t0)?)?;
        let _t1 = dis.readInt()?;
        let mut total: i32 = _t1;
        let _t2 = dis.readInt()?;
        let mut bkNum: i32 = _t2;
        let _t3 = dis.readInt()?;
        let mut cpNum: i32 = _t3;
        let _t4 = dis.readInt()?;
        let mut cpEnd: i32 = _t4;
        let mut _arr5: Vec<i8> = vec![0i8; cpEnd as usize];
        let mut ba: Vec<i8> = _arr5;
        let mut _arr6: Vec<i32> = vec![0i32; (bkNum).wrapping_mul(256i32) as usize];
        this.lookup.set(_arr6);
        let mut _arr7: Vec<i32> = vec![0i32; 4352i32 as usize];
        this.bkIndices.set(_arr7);
        let mut _arr8: Vec<i8> = vec![0i8; (total).wrapping_sub(cpEnd) as usize];
        this.strPool.set(_arr8);
        let mut _arr9: Vec<i32> = vec![0i32; (cpNum).wrapping_mul(3i32) as usize];
        this.cpEntries.set(_arr9);
        let mut _arr10: Vec<i32> = vec![0i32; ((cpNum/2i32)|1i32) as usize];
        this.hsIndices.set(_arr10);
        Arrays::fill__arr_i_i(&this.bkIndices.get(), -1i32)?;
        Arrays::fill__arr_i_i(&this.hsIndices.get(), -1i32)?;
        dis.readFully(ba)?;
        dis.readFully(this.strPool.get())?;
        let mut nameOff: i32 = 0i32;
        let mut cpOff: i32 = 0i32;
        let mut cp: i32 = 0i32;
        let mut bk: i32 = -1i32;
        let mut prevBk: i32 = -1i32;
        let mut idx: i32 = 0i32;
        cpOff = cpOff.wrapping_add(1i32);
        let mut len: i32 = (ba[cpOff as usize]&255i32);
        cpOff = cpOff.wrapping_add(1i32);
        len = (ba[cpOff as usize]&255i32);
        cpOff = cpOff.wrapping_add(1i32);
        cpOff = cpOff.wrapping_add(1i32);
        cpOff = cpOff.wrapping_add(1i32);
        cp = ((((ba[cpOff as usize]&255i32)<<(16i32&0x1f))|((ba[cpOff as usize]&255i32)<<(8i32&0x1f)))|(ba[cpOff as usize]&255i32));
        cp = cp.wrapping_add(1i32);
        let mut hi: i32 = (cp>>((8i32&0x1f)));
        bk = bk.wrapping_add(1i32);
        this.bkIndices.get()[hi as usize] = bk;
        prevBk = hi;
        this.lookup.get()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize] = ((nameOff<<(8i32&0x1f))|len);
        let _t11: i32 = CharacterName::hashN(&this.strPool.get(), nameOff, len)?;
        let mut hash: i32 = _t11;
        let mut hsh: i32 = ((hash&2147483647i32)%(this.hsIndices.get().len() as i32));
        let mut next: i32 = this.hsIndices.get()[hsh as usize];
        this.hsIndices.get()[hsh as usize] = idx;
        let _t12 = this.addCp(idx, hash, next, cp)?;
        idx = _t12;
        nameOff = (nameOff).wrapping_add(len);
        dis.close()?;
        total = cpEnd;
        dis.close()?;
        bkNum = cpOff;
        total.addSuppressed(bkNum)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        dis = hi;
        let _t13 = dis.getMessage()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this)
    }

    // java: hashN([BII)I
    pub fn hashN(a: &[i8], off: i32, len: i32) -> Result<i32> {
        let _t0: i32 = ArraysSupport::vectorizedHashCode(&a, off, len, 1i32, 8i32)?;
        Ok(_t0)
    }

    // java: addCp(IIII)I
    pub fn addCp(&self, idx: i32, hash: i32, next: i32, cp: i32) -> Result<i32> {
        let this = self;
        idx = idx.wrapping_add(1i32);
        this.cpEntries.get()[idx as usize] = hash;
        idx = idx.wrapping_add(1i32);
        this.cpEntries.get()[idx as usize] = next;
        idx = idx.wrapping_add(1i32);
        this.cpEntries.get()[idx as usize] = cp;
        Ok(idx)
    }

    // java: getCpHash(I)I
    pub fn getCpHash(&self, idx: i32) -> Result<i32> {
        let this = self;
        Ok(this.cpEntries.get()[idx as usize])
    }

    // java: getCpNext(I)I
    pub fn getCpNext(&self, idx: i32) -> Result<i32> {
        let this = self;
        Ok(this.cpEntries.get()[(idx).wrapping_add(1i32) as usize])
    }

    // java: getCp(I)I
    pub fn getCp(&self, idx: i32) -> Result<i32> {
        let this = self;
        Ok(this.cpEntries.get()[(idx).wrapping_add(2i32) as usize])
    }

    // java: getInstance()Ljava/lang/CharacterName;
    pub fn getInstance() -> Result<Object> {
        let mut ref_: Object = CharacterName::refCharName();
        let _t0 = ref_.get()?;
        let mut cname: Object = _t0;
        cname = CharacterName::new()?;
        CharacterName::refCharName(SoftReference::new(cname)?);
        Ok(cname)
    }

    // java: getName(I)Ljava/lang/String;
    pub fn getName(&self, cp: i32) -> Result<String> {
        let this = self;
        let mut bk: i32 = this.bkIndices.get()[(cp>>((8i32&0x1f))) as usize];
        let mut off: i32 = this.lookup.get()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize];
        /* TODO: aconst_null  */
        return Ok(this.lookup.get()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize]);
        let mut result: String = String::new(this.strPool.get(), 0i32, ((off as u32>>(8i32&0x1f)) as i32), (off&255i32))?;
        Ok(result)
    }

    // java: getCodePoint(Ljava/lang/String;)I
    pub fn getCodePoint(&self, name: String) -> Result<i32> {
        let this = self;
        let _t0 = name.getBytes(ISO_8859_1::INSTANCE())?;
        let mut bname: Vec<i8> = _t0;
        let _t1: i32 = CharacterName::hashN(&bname, 0i32, (bname.len() as i32))?;
        let mut hsh: i32 = _t1;
        let mut idx: i32 = this.hsIndices.get()[((hsh&2147483647i32)%(this.hsIndices.get().len() as i32)) as usize];
        loop {
            if idx == -1i32 { break; }
            let _t0 = this.getCpHash(idx)?;
            let _t1 = this.getCp(idx)?;
            let mut cp: i32 = _t1;
            let mut bk: i32 = this.bkIndices.get()[(cp>>((8i32&0x1f))) as usize];
            let mut off: i32 = this.lookup.get()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize];
            let mut len: i32 = (off&255i32);
            off = ((off as u32>>(8i32&0x1f)) as i32);
            let mut i: i32 = 0i32;
            off = off.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
            return Ok(cp);
            let _t2 = this.getCpNext(idx)?;
            idx = _t2;
        }
        Ok(-1i32)
    }
}
