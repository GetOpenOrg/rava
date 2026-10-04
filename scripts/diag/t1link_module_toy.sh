#!/usr/bin/env bash
# T1 第 2 步诊断：按模块分 dylib 的最小可行性（两模块 a ← b，各含 decl / body 与 export_name 环）。
# 用法：t1link_module_toy.sh <out_dir> [single]
#   缺省：每模块一个 dylib（liba 含 a_decl + a_body；libb 含 b_decl + b_body，依赖 liba）
#   single：一个 dylib 含全部模块 crate（对照）
# 验证：rustc 是否接受「上游 dylib 已静态含 a_decl、下游 rlib 依赖 a_decl」；static / thread_local /
#       OnceLock 跨 dylib 唯一；模块登记由 bin 按拓扑序调用；panic=unwind + prefer-dynamic。
set -euo pipefail
out=$1; mode=${2:-multi}
rm -rf "$out"; mkdir -p "$out/src"; cd "$out"
LIBDIR=$(rustc --print target-libdir)
R="rustc --edition=2021 -C panic=unwind -C prefer-dynamic -L dependency=$out"
cat > src/a_decl.rs <<'R'
use std::sync::{Mutex, OnceLock, atomic::{AtomicI32, Ordering}};
pub static COUNTER: AtomicI32 = AtomicI32::new(0);
thread_local! { pub static TL: std::cell::Cell<i32> = std::cell::Cell::new(0); }
pub static INIT: OnceLock<String> = OnceLock::new();
// 模块登记表（java_base 中的 meta / 反射分派 / 类初始化钩子的同构物）：下游模块的行由 bin 按拓扑序登记
pub static REGISTRY: Mutex<Vec<(&'static str, fn() -> i32)>> = Mutex::new(Vec::new());
pub fn register(rows: &'static [(&'static str, fn() -> i32)]) { REGISTRY.lock().unwrap().extend_from_slice(rows); }
pub fn call(name: &str) -> i32 { let f = REGISTRY.lock().unwrap().iter().find(|r| r.0 == name).unwrap().1; f() }
pub struct A;
impl A {
    #[inline]
    pub fn hello(&self) -> i32 {
        extern "Rust" { #[link_name = "__rava_a_A__hello"] fn h() -> i32; }
        unsafe { h() }
    }
}
pub fn tl_bump() -> i32 { TL.with(|c| { c.set(c.get() + 1); c.get() }) }
pub fn bump() -> i32 { COUNTER.fetch_add(1, Ordering::SeqCst) + 1 }
R
cat > src/a_body.rs <<'R'
#[export_name = "__rava_a_A__hello"]
pub fn hello() -> i32 { a_decl::INIT.get_or_init(|| "a-init".to_string()); a_decl::bump() }
pub static ROWS: [(&str, fn() -> i32); 1] = [("a.A.hello", hello)];
R
cat > src/b_decl.rs <<'R'
pub struct B;
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
pub fn go() -> i32 { a_decl::A.hello() + 100 + a_decl::tl_bump() * 0 }
pub static ROWS: [(&str, fn() -> i32); 1] = [("b.B.go", go)];
R
$R --crate-type rlib --emit=link,metadata --crate-name a_decl src/a_decl.rs
$R --crate-type rlib --emit=link,metadata --crate-name a_body src/a_body.rs --extern a_decl=liba_decl.rlib
$R --crate-type rlib --emit=link,metadata --crate-name b_decl src/b_decl.rs --extern a_decl=liba_decl.rlib
$R --crate-type rlib --emit=link,metadata --crate-name b_body src/b_body.rs --extern a_decl=liba_decl.rlib --extern b_decl=libb_decl.rlib
inst() { echo "-C link-arg=-Wl,-install_name,@rpath/lib$1.dylib -C link-arg=-Wl,-rpath,@loader_path"; }
if [ "$mode" = single ]; then
  printf 'pub extern crate a_decl;\npub extern crate a_body;\npub extern crate b_decl;\npub extern crate b_body;\n' > src/profile.rs
  $R --crate-type dylib --crate-name profile src/profile.rs $(inst profile) \
     --extern a_decl=liba_decl.rlib --extern a_body=liba_body.rlib --extern b_decl=libb_decl.rlib --extern b_body=libb_body.rlib
  EXT="--extern profile=libprofile.dylib"; USE="use profile as _;"
else
  printf 'pub extern crate a_decl;\npub extern crate a_body;\n' > src/a.rs
  $R --crate-type dylib --crate-name a src/a.rs $(inst a) --extern a_decl=liba_decl.rlib --extern a_body=liba_body.rlib
  printf 'pub extern crate b_decl;\npub extern crate b_body;\nextern crate a;\n' > src/b.rs
  $R --crate-type dylib --crate-name b src/b.rs $(inst b) --extern a=liba.dylib \
     --extern a_decl=liba_decl.rlib --extern b_decl=libb_decl.rlib --extern b_body=libb_body.rlib
  EXT="--extern a=liba.dylib --extern b=libb.dylib"; USE="use a as _; use b as _;"
fi
cat > src/main.rs <<R
$USE
fn main() {
    a_decl::register(&a_body::ROWS);   // 模块登记：bin 按模块拓扑序（a → b）
    a_decl::register(&b_body::ROWS);
    let x = a_decl::A.hello();
    let y = b_decl::B.go();
    let z = a_decl::call("b.B.go");
    println!("x={x} y={y} z={z} counter={} init={:?} tl={}", a_decl::COUNTER.load(std::sync::atomic::Ordering::SeqCst), a_decl::INIT.get(), a_decl::tl_bump());
}
R
$R --crate-type bin --crate-name main src/main.rs $EXT --extern a_decl=liba_decl.rmeta --extern b_decl=libb_decl.rmeta \
   --extern a_body=liba_body.rmeta --extern b_body=libb_body.rmeta \
   -C link-arg=-Wl,-rpath,$out -C link-arg=-Wl,-rpath,$LIBDIR 2>&1 | tail -20 || true
ls *.dylib main 2>/dev/null
for f in *.dylib main; do echo "== $f"; otool -L $f | tail -n +2; done
echo "== 未定义 __rava_ 符号："; for f in *.dylib; do echo "$f $(nm -u $f | grep -c __rava_ || true)"; done
./main
