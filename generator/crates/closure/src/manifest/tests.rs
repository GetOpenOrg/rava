//! 清单解析单元测试。

use super::*;

fn with_vm(vm: &str) -> Result<Manifest, String> {
    // 测试并行运行：目录名按进程内序号区分（按清单长度区分时，等长清单的测试互相覆盖）
    static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("rava-manifest-{}-{seq}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("vm_intrinsics.toml"), vm).unwrap();
    let r = Manifest::load(&dir);
    std::fs::remove_dir_all(&dir).ok();
    r
}

#[test]
fn array_returns_parse() {
    let m = with_vm("[facts.array_returns]\n\"a/B.f:()[Ljava/lang/Object;\" = { elements = [\"a/C\", \"a/D\"] }\n").unwrap();
    assert_eq!(m.array_return("a/B.f:()[Ljava/lang/Object;"), Some(&["a/C".to_string(), "a/D".to_string()][..]));
    assert_eq!(m.array_return("a/B.g:()[Ljava/lang/Object;"), None);
}

#[test]
fn serializable_markers_parse() {
    let m = with_vm("[facts.field_writes]\nserializable_markers = [\"a/Ser\"]\n").unwrap();
    assert_eq!(m.serializable_markers(), &["a/Ser".to_string()][..]);
    assert!(with_vm("").unwrap().serializable_markers().is_empty());
}

#[test]
fn array_returns_reject_non_array() {
    assert!(with_vm("[facts.array_returns]\n\"a/B.f:()Ljava/lang/Object;\" = { elements = [\"a/C\"] }\n").is_err());
    assert!(with_vm("[facts.array_returns]\n\"a/B.f:()[Ljava/lang/Object;\" = { elements = [] }\n").is_err());
}

#[test]
fn indy_object_methods_refines_native_and_boxing() {
    let m = with_vm("[indy]\nnative = [\"a/B.boot\", \"a/C.boot\"]\nobject_methods = [\"a/B.boot\"]\nconcat_stringify = \"a/S.v:(La/O;)La/S;\"\ncomponent_hash = \"a/U.h:(La/O;)I\"\ncomponent_equals = \"a/U.e:(La/O;La/O;)Z\"\n[boxing]\nI = \"a/BoxI\"\n").unwrap();
    assert_eq!(m.indy_kind("a/B.boot"), Some(IndyKind::ObjectMethods));
    assert_eq!(m.indy_kind("a/C.boot"), Some(IndyKind::Native));
    assert_eq!(m.boxed_class(b'I'), Some("a/BoxI"));
    assert_eq!(m.unboxed_prim("a/BoxI"), Some(b'I'));
    assert_eq!(m.boxed_class(b'J'), None);
}

#[test]
fn field_name_resolvers_and_class_initializers_parse() {
    let m = with_vm(
        "[facts.field_writes]\nenumerators = []\n[facts.field_writes.name_resolvers]\n\"a/B.f:(Ljava/lang/Class;Ljava/lang/String;)J\" = { class = 0, name = 1 }\n[facts.reflect]\nclass_initializers = [\"a/U.init:(Ljava/lang/Class;)V\"]\nhandle_owner_initializers = [\"a/D.check:(La/M;)Z\"]\nreflect_owner_initializers = [\"a/F.acc:(La/M;)V\"]\n",
    )
    .unwrap();
    assert_eq!(
        m.field_name_resolver("a/B.f:(Ljava/lang/Class;Ljava/lang/String;)J"),
        Some(NameResolver { class: Some(0), name: 1, handle: false, offset: false })
    );
    assert!(m.is_class_initializer("a/U.init:(Ljava/lang/Class;)V"));
    assert!(!m.is_class_initializer("a/U.other:(Ljava/lang/Class;)V"));
    assert_eq!(m.member_owner_route("a/D.check:(La/M;)Z"), Some(LinkRoute::Handle));
    assert_eq!(m.member_owner_route("a/F.acc:(La/M;)V"), Some(LinkRoute::Reflect));
    assert_eq!(m.member_owner_route("a/U.init:(Ljava/lang/Class;)V"), None);
}

/// 字段句柄类型：枚举返回数组的分量类型 ∪ handle = true 的按名入口返回类型（非句柄入口不计）
#[test]
fn field_handle_types_from_enumerators_and_handle_resolvers() {
    let m = with_vm(
        "[facts.field_writes]\nenumerators = [\"a/C.all:()[La/H;\"]\n[facts.field_writes.name_resolvers]\n\"a/C.one:(Ljava/lang/String;)La/H;\" = { class = \"receiver\", name = 0, handle = true }\n\"a/C.alt:(Ljava/lang/String;)La/K;\" = { class = \"receiver\", name = 0, handle = true }\n\"a/U.off:(Ljava/lang/Class;Ljava/lang/String;)J\" = { class = 0, name = 1, offset = true }\n",
    )
    .unwrap();
    assert_eq!(m.field_handle_types(), vec!["a/H".to_string(), "a/K".to_string()]);
    assert!(with_vm("").unwrap().field_handle_types().is_empty());
}

/// 仓库清单：按名取字段的入口与按镜像初始化入口都已登记（写入来源审计的闭合项）
#[test]
fn repo_manifest_declares_write_sources() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime");
    let m = Manifest::load(&dir).unwrap();
    let r = m.field_name_resolver("java/lang/Class.getDeclaredField:(Ljava/lang/String;)Ljava/lang/reflect/Field;").unwrap();
    assert!(r.handle && r.class.is_none());
    assert!(m.field_name_resolver("jdk/internal/misc/Unsafe.objectFieldOffset:(Ljava/lang/Class;Ljava/lang/String;)J").is_some_and(|r| r.offset));
    // 按偏移读写的 Unsafe 引用操作登记偏移形参（符号偏移收窄到所指字段）
    let cas = "jdk/internal/misc/Unsafe.compareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z";
    assert_eq!(m.array_writes(cas).and_then(|w| w.offset), Some(1));
    assert_eq!(m.memory_read_offset("jdk/internal/misc/Unsafe.getReferenceVolatile:(Ljava/lang/Object;J)Ljava/lang/Object;"), Some(1));
    assert!(m.is_class_initializer("jdk/internal/misc/Unsafe.ensureClassInitialized:(Ljava/lang/Class;)V"));
    assert_eq!(
        m.member_owner_route("java/lang/invoke/DirectMethodHandle.checkInitialized:(Ljava/lang/invoke/MemberName;)Z"),
        Some(LinkRoute::Handle)
    );
    assert_eq!(
        m.member_owner_route("jdk/internal/reflect/MethodHandleAccessorFactory.ensureClassInitialized:(Ljava/lang/Class;)V"),
        Some(LinkRoute::Reflect)
    );
}

#[test]
fn string_ops_parse() {
    let m = with_vm("[facts.string_ops]\n\"a/S.eic:(La/S;)Z\" = \"equals_ignore_case\"\n\"a/S.len:()I\" = \"length\"\n").unwrap();
    assert_eq!(m.string_op("a/S.eic:(La/S;)Z"), Some(StrOp::EqualsIgnoreCase));
    assert_eq!(m.string_op("a/S.len:()I"), Some(StrOp::Length));
    assert_eq!(m.string_op("a/S.x:()I"), None);
    assert!(with_vm("[facts.string_ops]\n\"a/S.f:()I\" = \"upper\"\n").is_err());
}

#[test]
fn handle_interpreters_parse() {
    let m = with_vm("[facts.handle_interpreters]\nmembers = [\"a/H.run:([La/O;)La/O;\"]\n").unwrap();
    let key = |n: &str| classfile::constant::MemberRef { owner: "a/H".into(), name: n.into(), desc: "([La/O;)La/O;".into() };
    assert!(m.is_handle_interpreter(&key("run")));
    assert!(!m.is_handle_interpreter(&key("other")));
}

#[test]
fn static_offset_getters_parse() {
    let m = with_vm("[facts.field_writes]\nstatic_offset_getters = [\"a/U.sfo:(La/F;)J\"]\n").unwrap();
    let key = |name: &str, desc: &str| classfile::constant::MemberRef { owner: "a/U".into(), name: name.into(), desc: desc.into() };
    assert!(m.is_static_offset_getter(&key("sfo", "(La/F;)J")));
    assert!(!m.is_static_offset_getter(&key("sfo", "(La/G;)J")));
}

#[test]
fn serial_enumerators_parse() {
    let m = with_vm("[facts.field_writes]\nserial_enumerators = [\"a/S.f:(Ljava/lang/Class;)J\"]\n").unwrap();
    assert!(m.is_serial_enumerator("a/S.f:(Ljava/lang/Class;)J"));
    assert!(!m.is_serial_enumerator("a/S.g:()V"));
}

#[test]
fn instance_field_users_parse() {
    let m = with_vm("[facts.field_writes]\ninstance_field_users = [\"a/S.f:(Ljava/lang/Class;)V\"]\n").unwrap();
    assert!(m.is_instance_field_user("a/S.f:(Ljava/lang/Class;)V"));
    assert!(!m.is_instance_field_user("a/S.g:()V"));
}

#[test]
fn serial_allocators_parse() {
    let m = with_vm("[facts.reflect.serial_allocators]\n\"a/G.gen:(Ljava/lang/Class;)La/A;\" = 0\n").unwrap();
    assert_eq!(m.serial_allocator("a/G.gen:(Ljava/lang/Class;)La/A;"), Some(0));
    assert_eq!(m.serial_allocator("a/G.other:()V"), None);
    assert!(with_vm("[facts.reflect.serial_allocators]\n\"a/G.gen:(Ljava/lang/Class;)La/A;\" = -1\n").is_err());
}
