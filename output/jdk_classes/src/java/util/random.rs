#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Random",
    super_class = "java/lang/Object",
    interfaces  = "java/util/random/RandomGenerator,java/io/Serializable",
    access      = "public",
    source      = "Random.java",
))]
pub struct Random {
    #[cfg_attr(any(), java_field(name = "seed", descriptor = "Ljava/util/concurrent/atomic/AtomicLong;", access = "private final"))]
    pub seed: Field<Object>,
    #[cfg_attr(any(), java_field(name = "nextNextGaussian", descriptor = "D", access = "private"))]
    pub nextNextGaussian: Field<f64>,
    #[cfg_attr(any(), java_field(name = "haveNextNextGaussian", descriptor = "Z", access = "private"))]
    pub haveNextNextGaussian: Field<bool>,
}

impl Random {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { seed: Field::new(Default::default()), nextNextGaussian: Field::new(0.0), haveNextNextGaussian: Field::new(false) };
        let _t0: i64 = Random::seedUniquifier()?;
        let _t1: i64 = System::nanoTime()?;
        /* TODO: lxor  */
        /* invokespecial Method java/util/Random.<init>:(J)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/Void;)V
    // java: <init>(Ljava/lang/Void;)V
    pub fn new__void(unused: Object) -> Result<Self> {
        let this = Self { seed: Field::new(Default::default()), nextNextGaussian: Field::new(0.0), haveNextNextGaussian: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.haveNextNextGaussian.set(0i32);
        /* TODO: aconst_null  */
        todo!("stack underflow").seed.set(this);
        Ok(this)
    }

    // java: seedUniquifier()J
    pub fn seedUniquifier() -> Result<i64> {
        let _t0 = Random::seedUniquifier().get()?;
        let mut current: i64 = _t0;
        let mut next: i64 = (current).wrapping_mul(1181783497276652981i64);
        let _t1 = Random::seedUniquifier().compareAndSet(current, next)?;
        Ok(next)
    }

    // java: <init>(J)V
    // java: <init>(J)V
    pub fn new__l(seed: i64) -> Result<Self> {
        let this = Self { seed: Field::new(Default::default()), nextNextGaussian: Field::new(0.0), haveNextNextGaussian: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.haveNextNextGaussian.set(0i32);
        let _t0 = this.getClass()?;
        let _t1: i64 = Random::initialScramble(seed)?;
        this.seed.set(AtomicLong::new(_t1)?);
        this.seed.set(AtomicLong::new()?);
        this.setSeed(seed)?;
        Ok(this)
    }

    // java: initialScramble(J)J
    pub fn initialScramble(seed: i64) -> Result<i64> {
        /* TODO: lxor  */
        /* TODO: land  */
        Ok(281474976710655i64)
    }

    // java: from(Ljava/util/random/RandomGenerator;)Ljava/util/Random;
    pub fn from(generator: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(generator)?;
        let mut rand: Object = generator;
        return Ok(rand);
        Ok(Random_RandomWrapper::new(generator)?)
    }

    // java: setSeed(J)V
    pub fn setSeed(&self, seed: i64) -> Result<()> {
        let this = self;
        let _t0: i64 = Random::initialScramble(seed)?;
        this.seed.get().set(_t0)?;
        this.haveNextNextGaussian.set(0i32);
        Ok(())
    }

    // java: next(I)I
    pub fn next(&self, bits: i32) -> Result<i32> {
        let this = self;
        let mut seed: Object = this.seed.get();
        let _t0 = seed.get()?;
        let mut oldseed: i64 = _t0;
        /* TODO: land  */
        let mut nextseed: i64 = 281474976710655i64;
        let _t1 = seed.compareAndSet(oldseed, nextseed)?;
        /* TODO: lushr  */
        Ok(((48i32).wrapping_sub(bits) as i32))
    }

    // java: nextBytes([B)V
    pub fn nextBytes(&self, bytes: Vec<i8>) -> Result<()> {
        let this = self;
        let mut i: i32 = 0i32;
        let mut len: i32 = (bytes.len() as i32);
        loop {
            if i >= len { break; }
            let _t0 = this.nextInt()?;
            let mut rnd: i32 = _t0;
            let _t1: i32 = ((len).wrapping_sub(i)).min(4i32);
            let mut n: i32 = _t1;
            n = n.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
            /* TODO: i2b  */
            bytes[i as usize] = rnd;
            rnd = (rnd>>((8i32&0x1f)));
        }
        Ok(())
    }

    // java: nextInt()I
    // java: nextInt()I
    pub fn nextInt(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.next(32i32)?;
        Ok(_t0)
    }

    // java: nextInt(I)I
    // java: nextInt(I)I
    pub fn nextInt__i(&self, bound: i32) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.next(31i32)?;
        let mut r: i32 = _t0;
        let mut m: i32 = (bound).wrapping_sub(1i32);
        /* TODO: lshr  */
        r = (31i32 as i32);
        let mut u: i32 = r;
        loop {
            r = (u%bound);
            if ((u).wrapping_sub((u%bound))).wrapping_add(m)>=0i32 { break; }
            let _t0 = this.next(31i32)?;
            u = _t0;
        }
        Ok(r)
    }

    // java: nextLong()J
    pub fn nextLong(&self) -> Result<i64> {
        let this = self;
        let _t0 = this.next(32i32)?;
        /* TODO: lshl  */
        let _t1 = this.next(32i32)?;
        Ok((32i32).wrapping_add((_t1 as i64)))
    }

    // java: nextBoolean()Z
    pub fn nextBoolean(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.next(1i32)?;
        Ok(_t0!=0i32)
    }

    // java: nextFloat()F
    pub fn nextFloat(&self) -> Result<f32> {
        let this = self;
        let _t0 = this.next(24i32)?;
        Ok(((_t0 as f32)*5.960464477539063e-08f32))
    }

    // java: nextDouble()D
    pub fn nextDouble(&self) -> Result<f64> {
        let this = self;
        let _t0 = this.next(26i32)?;
        /* TODO: lshl  */
        let _t1 = this.next(27i32)?;
        /* TODO: l2d  */
        Ok(((27i32).wrapping_add((_t1 as i64))*1.1102230246251565e-16f64))
    }

    // java: nextGaussian()D
    pub fn nextGaussian(&self) -> Result<f64> {
        let this = self;
        this.haveNextNextGaussian.set(0i32);
        return Ok(this.nextNextGaussian.get());
        let _t0 = this.nextDouble()?;
        let mut v1: f64 = ((2.0f64*_t0)-1f64);
        let _t1 = this.nextDouble()?;
        let mut v2: f64 = ((2.0f64*_t1)-1f64);
        let mut s: f64 = ((v1*v1)+(v2*v2));
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        let _t2: f64 = StrictMath::log(s)?;
        let _t3: f64 = StrictMath::sqrt(((-2.0f64*_t2)/s))?;
        let mut multiplier: f64 = _t3;
        this.nextNextGaussian.set((v2*multiplier));
        this.haveNextNextGaussian.set(1i32);
        Ok((v1*multiplier))
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let mut fields: Object = _t0;
        let _t1 = fields.get(String::from("seed"), 18446744073709551615i64)?;
        let mut seedVal: i64 = _t1;
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        this.resetSeed(seedVal)?;
        let _t2 = fields.get(String::from("nextNextGaussian"), 0f64)?;
        this.nextNextGaussian.set(_t2);
        let _t3 = fields.get(String::from("haveNextNextGaussian"), 0i32)?;
        this.haveNextNextGaussian.set(_t3);
        Ok(())
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.putFields()?;
        let mut fields: Object = _t0;
        let _t1 = this.seed.get().get()?;
        fields.put(String::from("seed"), _t1)?;
        fields.put(String::from("nextNextGaussian"), this.nextNextGaussian.get())?;
        fields.put(String::from("haveNextNextGaussian"), this.haveNextNextGaussian.get())?;
        s.writeFields()?;
        Ok(())
    }

    // java: resetSeed(J)V
    pub fn resetSeed(&self, seedVal: i64) -> Result<()> {
        let this = self;
        Random::unsafe_().putReferenceVolatile(this, Random::seedOffset(), AtomicLong::new(seedVal)?)?;
        Ok(())
    }

    // java: ints(J)Ljava/util/stream/IntStream;
    // java: ints(J)Ljava/util/stream/IntStream;
    pub fn ints__l(&self, streamSize: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::ints(this, streamSize)?;
        Ok(_t0)
    }

    // java: ints()Ljava/util/stream/IntStream;
    // java: ints()Ljava/util/stream/IntStream;
    pub fn ints(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::ints(this)?;
        Ok(_t0)
    }

    // java: ints(JII)Ljava/util/stream/IntStream;
    // java: ints(JII)Ljava/util/stream/IntStream;
    pub fn ints__l_i_i(&self, streamSize: i64, arg_1: i32, randomNumberOrigin: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::ints(this, streamSize, randomNumberOrigin, local_4)?;
        Ok(_t0)
    }

    // java: ints(II)Ljava/util/stream/IntStream;
    // java: ints(II)Ljava/util/stream/IntStream;
    pub fn ints__i_i(&self, randomNumberOrigin: i32, randomNumberBound: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::ints(this, randomNumberOrigin, randomNumberBound)?;
        Ok(_t0)
    }

    // java: longs(J)Ljava/util/stream/LongStream;
    // java: longs(J)Ljava/util/stream/LongStream;
    pub fn longs__l(&self, streamSize: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::longs(this, streamSize)?;
        Ok(_t0)
    }

    // java: longs()Ljava/util/stream/LongStream;
    // java: longs()Ljava/util/stream/LongStream;
    pub fn longs(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::longs(this)?;
        Ok(_t0)
    }

    // java: longs(JJJ)Ljava/util/stream/LongStream;
    // java: longs(JJJ)Ljava/util/stream/LongStream;
    pub fn longs__l_l_l(&self, streamSize: i64, arg_1: i64, randomNumberOrigin: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::longs(this, streamSize, randomNumberOrigin, local_5)?;
        Ok(_t0)
    }

    // java: longs(JJ)Ljava/util/stream/LongStream;
    // java: longs(JJ)Ljava/util/stream/LongStream;
    pub fn longs__l_l(&self, randomNumberOrigin: i64, arg_1: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::longs(this, randomNumberOrigin, local_3)?;
        Ok(_t0)
    }

    // java: doubles(J)Ljava/util/stream/DoubleStream;
    // java: doubles(J)Ljava/util/stream/DoubleStream;
    pub fn doubles__l(&self, streamSize: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::doubles(this, streamSize)?;
        Ok(_t0)
    }

    // java: doubles()Ljava/util/stream/DoubleStream;
    // java: doubles()Ljava/util/stream/DoubleStream;
    pub fn doubles(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::doubles(this)?;
        Ok(_t0)
    }

    // java: doubles(JDD)Ljava/util/stream/DoubleStream;
    // java: doubles(JDD)Ljava/util/stream/DoubleStream;
    pub fn doubles__l_d_d(&self, streamSize: i64, arg_1: f64, randomNumberOrigin: f64) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::doubles(this, streamSize, randomNumberOrigin, local_5)?;
        Ok(_t0)
    }

    // java: doubles(DD)Ljava/util/stream/DoubleStream;
    // java: doubles(DD)Ljava/util/stream/DoubleStream;
    pub fn doubles__d_d(&self, randomNumberOrigin: f64, arg_1: f64) -> Result<Object> {
        let this = self;
        let _t0: Object = RandomSupport_AbstractSpliteratorGenerator::doubles(this, randomNumberOrigin, local_3)?;
        Ok(_t0)
    }
}
