use super::*;
use std::sync::Arc;

/// 8 字节对齐、按字计的测试缓冲区（满足地址前提）。
struct Buf(Vec<AtomicU64>);

impl Buf {
    fn new(words: usize) -> Arc<Buf> {
        Arc::new(Buf((0..words).map(|_| AtomicU64::new(0)).collect()))
    }
    fn addr(&self) -> usize {
        self.0.as_ptr() as usize
    }
    fn len(&self) -> usize {
        self.0.len() * WORD
    }
    fn bytes(&self) -> Vec<u8> {
        let mut v = vec![0u8; self.len()];
        unsafe { read(self.addr(), &mut v) };
        v
    }
}

const ITERS: u32 = if cfg!(miri) { 40 } else { 200_000 };

#[test]
fn negative_length_rejected() {
    assert_eq!(length(-1), Err(Error::NegativeLength(-1)));
    assert_eq!(length(i64::MIN), Err(Error::NegativeLength(i64::MIN)));
    assert_eq!(length(0), Ok(0));
    assert_eq!(length(17), Ok(17));
}

#[test]
fn aligned_update_all_widths() {
    let b = Buf::new(4);
    let base = b.addr();
    for (off, width) in [(0, 8), (8, 4), (12, 4), (16, 2), (22, 2), (29, 1)] {
        let a = base + off;
        let want = 0x0102_0304_0506_0708u64 & mask(width);
        let old = unsafe { update(a, width, &mut |c| (c == 0).then_some(want)) }.unwrap();
        assert_eq!(old, 0, "首次 CAS 见证旧值 0（off={off} width={width}）");
        let fail = unsafe { update(a, width, &mut |c| (c == 0).then_some(1)) }.unwrap();
        assert_eq!(fail, want, "期望值不符：不写，返回当前值");
        assert_eq!(unsafe { load(a, width) }, want);
    }
}

#[test]
fn misaligned_update_rejected() {
    let b = Buf::new(2);
    let base = b.addr();
    for (off, width) in [(1, 2), (2, 4), (6, 4), (4, 8), (7, 8), (1, 8)] {
        let r = unsafe { update(base + off, width, &mut |_| Some(u64::MAX)) };
        assert_eq!(r, Err(Error::Misaligned { addr: base + off, width }));
    }
    assert!(b.bytes().iter().all(|&x| x == 0), "被拒的 CAS 不得写入");
}

#[test]
fn plain_access_roundtrip_any_alignment() {
    let b = Buf::new(4);
    let base = b.addr();
    for off in 0..16 {
        for width in [1, 2, 4, 8] {
            let v = 0xA1B2_C3D4_E5F6_0718u64 & mask(width);
            unsafe { store(base + off, width, v) };
            assert_eq!(unsafe { load(base + off, width) }, v, "off={off} width={width}");
        }
    }
}

#[test]
fn bulk_read_write_fill_copy() {
    let b = Buf::new(8);
    let base = b.addr();
    let src: Vec<u8> = (0..37u8).collect();
    unsafe { write(base + 3, &src) };
    let mut back = vec![0u8; src.len()];
    unsafe { read(base + 3, &mut back) };
    assert_eq!(back, src);
    let all = b.bytes();
    assert!(all[..3].iter().all(|&x| x == 0) && all[40..].iter().all(|&x| x == 0), "区间外字节不变");

    unsafe { fill(base + 5, 20, 0xEE) };
    let all = b.bytes();
    assert!(all[5..25].iter().all(|&x| x == 0xEE));
    assert_eq!(all[4], 1, "填充起点前一字节不变");
    assert_eq!(all[25], 22, "填充终点后一字节不变");
}

/// 起点偏移 0..8 × 长度 0..40 全组合：读 / 写 / 填充 / 同偏移拷贝与逐字节参照一致，区间外字节不变。
#[test]
fn bulk_every_offset_and_length() {
    for off in 0..8usize {
        for n in 0..40usize {
            let b = Buf::new(8);
            let base = b.addr();
            let init: Vec<u8> = (0..b.len()).map(|i| (i * 7 + 1) as u8).collect();
            unsafe { write(base, &init) };
            let src: Vec<u8> = (0..n).map(|i| (200 - i) as u8).collect();
            unsafe { write(base + off, &src) };
            let mut want = init.clone();
            want[off..off + n].copy_from_slice(&src);
            assert_eq!(b.bytes(), want, "write off={off} n={n}");
            let mut back = vec![0u8; n];
            unsafe { read(base + off, &mut back) };
            assert_eq!(back, src, "read off={off} n={n}");
            unsafe { fill(base + off, n, 0x5A) };
            want[off..off + n].fill(0x5A);
            assert_eq!(b.bytes(), want, "fill off={off} n={n}");
            unsafe { copy(base + 64 - 16 + off % 8 - 8, base + off, n.min(8)) };
            let from = 64 - 16 + off % 8 - 8;
            want.copy_within(from..from + n.min(8), off);
            assert_eq!(b.bytes(), want, "copy off={off} n={n}");
        }
    }
}

#[test]
fn copy_overlapping_matches_memmove() {
    for (from, to, n) in [(0usize, 7usize, 1500usize), (7, 0, 1500), (3, 3, 100), (0, 600, 512), (1, 2, 1023), (0, 8, 1500), (8, 0, 1500), (3, 11, 1001), (11, 3, 1001), (2, 10, 5), (13, 5, 3), (5, 13, 16), (6, 14, 2)] {
        let b = Buf::new(400);
        let base = b.addr();
        let init: Vec<u8> = (0..b.len()).map(|i| (i * 31 % 251) as u8).collect();
        unsafe { write(base, &init) };
        let mut want = init.clone();
        want.copy_within(from..from + n, to);
        unsafe { copy(base + from, base + to, n) };
        assert_eq!(b.bytes(), want, "from={from} to={to} n={n}");
    }
}

/// 同一字上：一线程以 4 字节原子加计数（getAndAddInt 形态），另一线程反复普通写 / 填充 / 拷贝相邻
/// 字节。字粒度合并不得丢失计数（旧实现的 memcpy + 非原子读-改-写会丢更新，且属数据竞争）。
#[test]
fn concurrent_cas_and_bulk_writes_lose_nothing() {
    let b = Buf::new(4);
    let counter = b.addr() + 4;
    let writer = {
        let b = Arc::clone(&b);
        std::thread::spawn(move || {
            let base = b.addr();
            for i in 0..ITERS {
                unsafe {
                    write(base, &[i as u8; 4]);
                    store(base + 1, 2, i as u64);
                    fill(base + 8, 20, i as u8);
                    copy(base + 8, base, 4);
                }
            }
        })
    };
    for _ in 0..ITERS {
        unsafe { update(counter, 4, &mut |c| Some(c.wrapping_add(1))) }.unwrap();
    }
    writer.join().unwrap();
    assert_eq!(unsafe { load(counter, 4) }, ITERS as u64);
}

/// 两线程在同一字的两半各自做 4 字节 CAS 自增：互不干扰。
#[test]
fn concurrent_cas_on_word_halves() {
    let b = Buf::new(1);
    let other = {
        let b = Arc::clone(&b);
        std::thread::spawn(move || {
            for _ in 0..ITERS {
                unsafe { update(b.addr(), 4, &mut |c| Some(c + 1)) }.unwrap();
            }
        })
    };
    for _ in 0..ITERS {
        unsafe { update(b.addr() + 4, 4, &mut |c| Some(c + 1)) }.unwrap();
    }
    other.join().unwrap();
    assert_eq!(unsafe { load(b.addr(), 4) }, ITERS as u64);
    assert_eq!(unsafe { load(b.addr() + 4, 4) }, ITERS as u64);
}
