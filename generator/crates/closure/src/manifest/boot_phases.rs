//! seeds.toml `[[boot_init.phases]]`：VM 引导阶段（HotSpot `call_initPhase2` / `call_initPhase3`）。
//!
//! 每个阶段是 VM 以常量实参调用的静态方法；`anchors` 是该阶段写入、此后由 Java 代码读取的 VM 状态字段
//! （`类.字段:描述符`）。闭包内任一锚点字段有读取点时阶段作根（入口形参取 `args` 常量），并由生成项目在
//! main 之前按序调用；否则阶段不入链、不执行。锚点判定在不动点内单调（一旦作根不撤回）。

/// 一个引导阶段
#[derive(Debug, Clone, PartialEq)]
pub struct BootPhase {
    /// `类.方法:描述符`（静态方法）
    pub call: String,
    /// 实参常量（布尔按 0 / 1），个数与描述符形参一致
    pub args: Vec<i32>,
    /// 锚点字段（声明类, 字段名, 描述符）
    pub anchors: Vec<(String, String, String)>,
}

/// 形参描述符中的各形参（只接受可由整数常量表示的基本类型）
fn int_params(desc: &str) -> Option<Vec<u8>> {
    let inner = desc.strip_prefix('(')?.split_once(')')?.0;
    let bytes = inner.as_bytes();
    bytes.iter().all(|c| b"ZBCSI".contains(c)).then(|| bytes.to_vec())
}

pub fn parse(seeds: &toml::Table) -> Result<Vec<BootPhase>, String> {
    let Some(arr) = seeds.get("boot_init").and_then(|s| s.get("phases")) else { return Ok(Vec::new()) };
    let arr = arr.as_array().ok_or("seeds.toml [[boot_init.phases]]：须为表数组")?;
    let mut out = Vec::new();
    for p in arr {
        let call = p.get("call").and_then(|v| v.as_str()).ok_or("seeds.toml [[boot_init.phases]]：缺 call")?;
        let err = |what: &str| format!("seeds.toml [[boot_init.phases]] {call}：{what}");
        let desc = call.split_once(':').filter(|(m, _)| m.contains('.')).map(|(_, d)| d).ok_or_else(|| err("call 须写成 类.方法:描述符"))?;
        let params = int_params(desc).ok_or_else(|| err("形参只能是 Z / B / C / S / I（实参为清单常量）"))?;
        let mut args = Vec::new();
        for a in p.get("args").and_then(|v| v.as_array()).into_iter().flatten() {
            args.push(match a {
                toml::Value::Boolean(b) => *b as i32,
                toml::Value::Integer(i) => i32::try_from(*i).map_err(|_| err("整数实参越界"))?,
                _ => return Err(err("args 只能是布尔 / 整数")),
            });
        }
        if args.len() != params.len() {
            return Err(err("args 个数与描述符形参不一致"));
        }
        let mut anchors = Vec::new();
        for a in p.get("anchors").and_then(|v| v.as_array()).into_iter().flatten() {
            let s = a.as_str().ok_or_else(|| err("anchors 须为字符串"))?;
            let (owner_name, fdesc) = s.split_once(':').ok_or_else(|| err("锚点须写成 类.字段:描述符"))?;
            let (owner, name) = owner_name.rsplit_once('.').ok_or_else(|| err("锚点须写成 类.字段:描述符"))?;
            if fdesc.starts_with('(') {
                return Err(err("锚点须是字段"));
            }
            anchors.push((owner.to_string(), name.to_string(), fdesc.to_string()));
        }
        out.push(BootPhase { call: call.to_string(), args, anchors });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(s: &str) -> Result<Vec<BootPhase>, String> {
        parse(&s.parse::<toml::Table>().unwrap())
    }

    #[test]
    fn phases_parse_args_and_anchors() {
        let p = load(
            "[boot_init]\nclasses = []\n[[boot_init.phases]]\ncall = \"p/S.phase:(ZZ)I\"\nargs = [false, true]\nanchors = [\"p/S.layer:Lp/L;\"]\n",
        )
        .unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].args, vec![0, 1]);
        assert_eq!(p[0].anchors, vec![("p/S".into(), "layer".into(), "Lp/L;".into())]);
    }

    #[test]
    fn phases_reject_arity_and_reference_params() {
        assert!(load("[[boot_init.phases]]\ncall = \"p/S.phase:(ZZ)I\"\nargs = [false]\n").is_err());
        assert!(load("[[boot_init.phases]]\ncall = \"p/S.phase:(Lp/X;)V\"\nargs = [0]\n").is_err());
        assert!(load("[[boot_init.phases]]\ncall = \"p/S.phase:()V\"\nanchors = [\"p/S.m:()V\"]\n").is_err());
    }
}
