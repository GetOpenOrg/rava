#!/usr/bin/env bash
# T1 第 2 步诊断：按模块命名的终态形态——模块 crate（dylib / rlib）以模块名命名并 `pub use` 本模块声明层，
# 下游模块与 bin 只经模块 crate 引用（a::A），模块内声明层 / 实现层为内部 crate（a_decl / a_body）。
# 每模块导出 `__rava_register_module()`，bin 按模块拓扑序调用（取代 main.rs 中逐类登记行）。
# 用法：t1link_module_toy_reexport.sh <out_dir> [dylib|rlib]
set -euo pipefail
out=$1; kind=${2:-dylib}
rm -rf "$out"; mkdir -p "$out/src"; cd "$out"
LIBDIR=$(rustc --print target-libdir)
if [ "$kind" = dylib ]; then
  R="rustc --edition=2021 -C panic=unwind -C prefer-dynamic -L dependency=$out"; EXTN=dylib
else
  R="rustc --edition=2021 -C panic=abort -L dependency=$out"; EXTN=rlib
fi
RL="$R --crate-type rlib --emit=link,metadata"
cat > src/a_decl.rs <<'R'
use std::sync::{Mutex, OnceLock, atomic::{AtomicI32, Ordering}};
pub static COUNTER: AtomicI32 = AtomicI32::new(0);
pub static INIT: OnceLock<String> = OnceLock::new();
pub static REGISTRY: Mutex<Vec<(&'static str, fn() -> i32)>> = Mutex::new(Vec::new());
pub fn register(rows: &'static [(&'static str, fn() -> i32)]) { REGISTRY.lock().unwrap().extend_from_slice(rows); }
pub fn call(name: &str) -> i32 { let f = REGISTRY.lock().unwrap().iter().find(|r| r.0 == name).unwrap().1; f() }
pub fn bump() -> i32 { COUNTER.fetch_add(1, Ordering::SeqCst) + 1 }
pub trait ObjectVTable { fn hash_code(&self) -> i32; }
pub struct A;
impl ObjectVTable for A { fn hash_code(&self) -> i32 { 7 } }
impl A {
    #[inline]
    pub fn hello(&self) -> i32 {
        extern "Rust" { #[link_name = "__rava_a_A__hello"] fn h() -> i32; }
        unsafe { h() }
    }
}
R
cat > src/a_body.rs <<'R'
#[export_name = "__rava_a_A__hello"]
pub fn hello() -> i32 { a_decl::INIT.get_or_init(|| "a-init".to_string()); a_decl::bump() }
pub static ROWS: [(&str, fn() -> i32); 1] = [("a.A.hello", hello)];
R
cat > src/a.rs <<'R'
pub use a_decl::*;
pub fn __rava_register_module() { a_decl::register(&a_body::ROWS); }
R
cat > src/b_decl.rs <<'R'
pub struct B;
impl a::ObjectVTable for B { fn hash_code(&self) -> i32 { 8 } }
impl B {
    #[inline]
    pub fn go(&self) -> i32 {
        extern "Rust" { #[link_name = "__rava_b_B__go"] fn g() -> i32; }
        unsafe { g() }
    }
}
R
cat > src/b_body.rs <<'R'
#[export_name = "__rava_b_B__go"]
pub fn go() -> i32 { a::A.hello() + 100 }
pub static ROWS: [(&str, fn() -> i32); 1] = [("b.B.go", go)];
R
cat > src/b.rs <<'R'
pub use b_decl::*;
pub fn __rava_register_module() { a::register(&b_body::ROWS); }
R
inst() { [ "$kind" = dylib ] && echo "-C link-arg=-Wl,-install_name,@rpath/lib$1.dylib -C link-arg=-Wl,-rpath,@loader_path" || true; }
$RL --crate-name a_decl src/a_decl.rs
$RL --crate-name a_body src/a_body.rs --extern a_decl=liba_decl.rlib
$R --crate-type $EXTN --emit=link,metadata --crate-name a src/a.rs $(inst a) --extern a_decl=liba_decl.rlib --extern a_body=liba_body.rlib
$RL --crate-name b_decl src/b_decl.rs --extern a=liba.$EXTN
$RL --crate-name b_body src/b_body.rs --extern a=liba.$EXTN --extern b_decl=libb_decl.rlib
$R --crate-type $EXTN --emit=link,metadata --crate-name b src/b.rs $(inst b) --extern a=liba.$EXTN --extern b_decl=libb_decl.rlib --extern b_body=libb_body.rlib
cat > src/main.rs <<'R'
fn main() {
    a::__rava_register_module();      // 按模块拓扑序：a → b
    b::__rava_register_module();
    let objs: Vec<std::rc::Rc<dyn a::ObjectVTable>> = vec![std::rc::Rc::new(a::A), std::rc::Rc::new(b::B)];
    let h: i32 = objs.iter().map(|o| o.hash_code()).sum();
    println!("x={} y={} z={} counter={} init={:?} hash={h}", a::A.hello(), b::B.go(), a::call("b.B.go"),
             a::COUNTER.load(std::sync::atomic::Ordering::SeqCst), a::INIT.get());
}
R
RP=""; [ "$kind" = dylib ] && RP="-C link-arg=-Wl,-rpath,$out -C link-arg=-Wl,-rpath,$LIBDIR"
$R --crate-type bin --crate-name main src/main.rs --extern a=liba.$EXTN --extern b=libb.$EXTN $RP
[ "$kind" = dylib ] && for f in liba.dylib libb.dylib main; do echo "== $f"; otool -L $f | tail -n +2 | grep -v libSystem; done
./main
