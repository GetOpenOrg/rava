use crate::prelude::*;
use super::embedded_class_path::EmbeddedClassPath;

// jdk.internal.loader.EmbeddedClassPath 伴生（VM 支持类，runtime/java_support）：构建期嵌入资源的读取 native
//（用户决策 (c)，计划 docs/plans/2026-10-01-c1d-closure-bloat.md §30.15）。
//
// 视图：类路径资源（发射层写入用户侧元数据 `meta::class_path_resources`：按名有序，同名按类路径序）。
// 模块资源不在此：它们在本程序 jimage 里，经翻译的 BuiltinClassLoader → SystemModuleReader 读取（boot-image §5.7）。
impl EmbeddedClassPath {
    /// native `count(String)`：名为 name 的嵌入资源份数。name 为 null → NPE。
    #[jvm_native]
    pub fn count(name: String) -> Result<i32> {
        Ok(same_name(&name)?.len() as i32)
    }

    /// native `bytes(String, int)`：名为 name 的第 index 份资源的字节；越界 → null。
    #[jvm_native]
    pub fn bytes(name: String, index: i32) -> Result<JArray<i8>> {
        let rows = same_name(&name)?;
        let Some((_, bytes)) = usize::try_from(index).ok().and_then(|i| rows.get(i)) else {
            return Ok(JArray::default());
        };
        Ok(JArray::from(bytes.iter().map(|b| *b as i8).collect::<Vec<i8>>()))
    }
}

/// 同名类路径资源行（类路径序）
fn same_name(name: &String) -> Result<&'static [(&'static str, &'static [u8])]> {
    if name.is_jvm_null() {
        return Err(JvmError::null_pointer());
    }
    let key = format!("{}", name);
    let table = crate::meta::class_path_resources();
    let lo = table.partition_point(|(n, _)| *n < key.as_str());
    let hi = lo + table[lo..].partition_point(|(n, _)| *n == key.as_str());
    Ok(&table[lo..hi])
}
