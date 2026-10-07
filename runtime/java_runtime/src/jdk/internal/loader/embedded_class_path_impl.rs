use crate::prelude::*;
use super::embedded_class_path::EmbeddedClassPath;

// jdk.internal.loader.EmbeddedClassPath 伴生（VM 支持类，runtime/java_support）：构建期嵌入资源的读取 native
//（用户决策 (c)，计划 docs/plans/2026-10-01-c1d-closure-bloat.md §30.15）。
//
// 视图与系统类加载器所见一致：先模块资源（JDK 侧，`meta::module_resource`：档案侧编译期嵌入表与用户代码
// 指名的用户侧表，含 `<类名>.class` 形态的 JDK 类文件），后类路径资源（发射层写入用户侧元数据 `meta::class_path_resources`：
// 按名有序，同名按类路径序）。
impl EmbeddedClassPath {
    /// native `count(String)`：名为 name 的嵌入资源份数。name 为 null → NPE。
    #[jvm_native]
    pub fn count(name: String) -> Result<i32> {
        let (module, rows) = same_name(&name)?;
        Ok((module.is_some() as usize + rows.len()) as i32)
    }

    /// native `bytes(String, int)`：名为 name 的第 index 份资源的字节；越界 → null。
    #[jvm_native]
    pub fn bytes(name: String, index: i32) -> Result<JArray<i8>> {
        let (module, rows) = same_name(&name)?;
        let all = module.into_iter().chain(rows.iter().map(|(_, b)| *b));
        let Some(bytes) = usize::try_from(index).ok().and_then(|i| all.clone().nth(i)) else {
            return Ok(JArray::default());
        };
        Ok(JArray::from(bytes.iter().map(|b| *b as i8).collect::<Vec<i8>>()))
    }
}

/// 同名资源：模块资源（至多一份）与类路径行（类路径序）
fn same_name(name: &String) -> Result<(Option<&'static [u8]>, &'static [(&'static str, &'static [u8])])> {
    if name.is_jvm_null() {
        return Err(JvmError::null_pointer());
    }
    let key = format!("{}", name);
    let module = crate::meta::module_resource(&key);
    let table = crate::meta::class_path_resources();
    let lo = table.partition_point(|(n, _)| *n < key.as_str());
    let hi = lo + table[lo..].partition_point(|(n, _)| *n == key.as_str());
    Ok((module, &table[lo..hi]))
}
