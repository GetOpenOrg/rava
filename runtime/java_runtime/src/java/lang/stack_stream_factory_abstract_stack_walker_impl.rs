//! `java.lang.StackStreamFactory$AbstractStackWalker` 的 native 层（类 ③：VM 栈遍历的落地语义）。
//!
//! 对标 HotSpot `StackWalk::walk` / `StackWalk::fetchNextBatch` / `StackWalk::fill_in_frames`
//!（stackwalk.cpp，JDK 21）：
//! - `callStackWalk`：在本线程锚定一条帧流（帧源 `crate::vm_stack`，于 callStackWalk 帧处取快照，
//!   与 HotSpot 锚定在该帧同口径），跳过 StackWalker 实现帧（持有类为 StackWalker、AbstractStackWalker
//!   或其直接子类），再跳 `skipframes` 帧，填首批后上调 `doStackWalk(anchor, skipframes, batchSize,
//!   startIndex, endIndex)`，返回其结果；返回（含异常完成）后锚点失效；
//! - `fetchStackFrames`：按锚点续取下一批，返回 endIndex；锚点不在本线程的活动流中 → InternalError；
//! - 填帧：未带 SHOW_HIDDEN_FRAMES 或 GET_CALLER_CLASS 模式时跳过 `@Hidden` 方法；FILL_CLASS_REFS_ONLY
//!   只填 Class，否则填 StackFrameInfo（VM 写入其 memberName 的 clazz / name / type / flags 与 bci）。
//!
//! 本模型的 StackFrameInfo 填写：name 与 type（方法描述符串）一并填入——HotSpot 只填 clazz / flags /
//! vmtarget，name / type 由 `MethodHandleNatives.expand` 经 vmtarget 惰性补齐；本模型 MemberName 不带
//! vmtarget，故在此处一次填齐（`MemberName.getMethodType` / `getMethodDescriptor` 对描述符串形态原生
//! 支持）。bci：原生二进制运行期无字节码，帧不携带字节码下标，记 0（StackFrameInfo 的「VM 初始化为
//! >= 0」约定）；行号同理不可得（见 stack_trace_element_impl.rs）。
//! FILL_LIVE_STACK_FRAMES（内部 API LiveStackFrame 的局部变量 / 操作数栈 / 监视器快照）依赖解释器帧
//! 布局，原生帧无此数据，抛 UnsupportedOperationException。
//! StackFrameInfo / MemberName 字段经按名协议写入：两类只在 StackFrameTraverser 路径存在，本文件不以
//! 类型名引用，免得 CallerClassFinder 一路（getCallerClass）把它们拉进生成范围。

use crate::prelude::*;
use super::stack_stream_factory_abstract_stack_walker::StackStreamFactory_AbstractStackWalker;
use super::Class;
use crate::vm_stack::JavaFrame;
use crate::jdk::internal::vm::{Continuation, ContinuationScope};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

const FILL_CLASS_REFS_ONLY: i64 = 0x2;
const GET_CALLER_CLASS: i64 = 0x4;
const SHOW_HIDDEN_FRAMES: i64 = 0x20;
const FILL_LIVE_STACK_FRAMES: i64 = 0x100;

const STACK_WALKER: &str = "java/lang/StackWalker";
const ABSTRACT_STACK_WALKER: &str = "java/lang/StackStreamFactory$AbstractStackWalker";

/// 一条锚定的帧流：快照与下一个待检视帧的下标。
struct Anchored {
    frames: Vec<JavaFrame>,
    cursor: usize,
}

thread_local! {
    static ANCHORS: RefCell<HashMap<i64, Anchored>> = RefCell::new(HashMap::new());
    static NEXT_ANCHOR: Cell<i64> = const { Cell::new(1) };
}

fn _internal_error(msg: &str) -> JvmError {
    match crate::java::lang::InternalError::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

fn _unsupported(msg: &str) -> JvmError {
    match crate::java::lang::UnsupportedOperationException::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

fn _npe(msg: &str) -> JvmError {
    match crate::java::lang::NullPointerException::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// StackWalker 实现帧：持有类为 StackWalker、AbstractStackWalker 或其直接子类。
fn _is_walker_impl(class: &str) -> bool {
    class == STACK_WALKER || class == ABSTRACT_STACK_WALKER
        || crate::vm_stack::direct_super(class) == Some(ABSTRACT_STACK_WALKER)
}

/// 按名写入的命中检查；字段缺失是协议断裂（生成类恒应答其字段）。字段名须在调用点以字面量
/// 给出：闭包分析器据此得知该名字段被手写层写入，其读取不按值集折叠。
fn _ensure(written: bool, field: &str) {
    if !written {
        panic!("stub: java/lang/StackStreamFactory$AbstractStackWalker 填帧：{} 字段无按名协议", field);
    }
}

/// StackFrameInfo 填写（HotSpot `java_lang_StackFrameInfo::set_method_and_bci`）。
fn _fill_frame_info(info: Object, frame: &JavaFrame) {
    let Some(member) = info.0.__unsafe_ref_get("memberName") else {
        panic!("stub: java/lang/StackFrameInfo.memberName 无按名协议");
    };
    let clazz = Object::from(Class::for_class(String::from(frame.class)));
    _ensure(member.0.__unsafe_ref_set("clazz", clazz), "clazz");
    _ensure(member.0.__unsafe_ref_set("name", Object::from(String::from(frame.method.name))), "name");
    _ensure(member.0.__unsafe_ref_set("type_", Object::from(String::from(frame.method.descriptor))), "type");
    _ensure(member.0.__unsafe_int_set("flags", frame.member_name_flags()), "flags");
    _ensure(info.0.__unsafe_int_set("bci", frame.bci), "bci");
}

impl<R, T> StackStreamFactory_AbstractStackWalker<R, T>
where
    R: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
    T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
{
    /// HotSpot `StackWalk::fill_in_frames`：自流的当前位置起填至多 `max` 帧到 `frames[start..]`，返回
    /// endIndex。流位置停在下一个待检视帧。
    fn __fill_in_frames(mode: i64, stream: &mut Anchored, max: i32, start: i32, frames: &JArray<T>) -> Result<i32> {
        if mode & FILL_LIVE_STACK_FRAMES != 0 {
            return Err(_unsupported("live stack frames are not available: no interpreter frame layout in a native image"));
        }
        let skip_hidden = mode & SHOW_HIDDEN_FRAMES == 0 || mode & GET_CALLER_CLASS != 0;
        let mut end = start;
        let mut decoded = 0;
        while stream.cursor < stream.frames.len() && decoded < max {
            let frame = stream.frames[stream.cursor].clone();
            stream.cursor += 1;
            if skip_hidden && frame.is_hidden() {
                continue;
            }
            let index = end;
            end += 1;
            if mode & FILL_CLASS_REFS_ONLY == 0 {
                _fill_frame_info(Into::<Object>::into(frames.get(index)?), &frame);
            } else {
                if mode & GET_CALLER_CLASS != 0 && index == start && frame.is_caller_sensitive() {
                    return Err(_unsupported(&format!(
                        "StackWalker::getCallerClass called from @CallerSensitive '{}' method",
                        frame.external_name())));
                }
                let class = Class::for_class(String::from(frame.class));
                frames.set(index, T::from(Object::from(class)))?;
            }
            decoded += 1;
        }
        Ok(end)
    }

    /// native `callStackWalk(long mode, int skipframes, ContinuationScope, Continuation, int batchSize,
    /// int startIndex, T[] frames)`。续体参数：本模型无虚拟线程续体帧（VirtualThread 为 VM 边界类），
    /// 帧流即当前载体线程栈。
    #[jvm_native]
    pub fn callStackWalk(&self, mode: i64, skipframes: i32, _cont_scope: ContinuationScope, _continuation: Continuation,
                                batch_size: i32, start_index: i32, frames: JArray<T>) -> Result<R> {
        if frames.is_jvm_null() {
            return Err(_npe("frames_array is null"));
        }
        if frames.len()? < start_index + batch_size {
            return Err(JvmError::illegal_argument("not enough space in buffers"));
        }
        let all = crate::vm_stack::capture_java_frames();
        let mut cursor = all.iter().position(|f| !_is_walker_impl(f.class)).unwrap_or(all.len());
        cursor = (cursor + skipframes.max(0) as usize).min(all.len());
        let mut stream = Anchored { frames: all, cursor };
        let end_index = Self::__fill_in_frames(mode, &mut stream, batch_size, start_index, &frames)?;
        if end_index - start_index < 1 {
            return Err(_internal_error("stack walk: decode failed"));
        }
        let anchor = NEXT_ANCHOR.with(|n| {
            let a = n.get();
            n.set(if a + 1 == -1 || a + 1 == 0 { 1 } else { a + 1 });
            a
        });
        ANCHORS.with(|t| t.borrow_mut().insert(anchor, stream));
        let result = self.doStackWalk(anchor, skipframes, batch_size, start_index, end_index);
        ANCHORS.with(|t| t.borrow_mut().remove(&anchor));
        Ok(R::from(result?))
    }

    /// native `fetchStackFrames(long mode, long anchor, int batchSize, int startIndex, T[] frames)`：
    /// 续取下一批，返回 endIndex（流已到底 → startIndex）。
    #[jvm_native]
    pub fn fetchStackFrames_l_l_i_i_arr_obj(&self, mode: i64, anchor: i64, batch_size: i32, start_index: i32,
                                            frames: JArray<T>) -> Result<i32> {
        let Some(mut stream) = ANCHORS.with(|t| t.borrow_mut().remove(&anchor)) else {
            return Err(_internal_error("doStackWalk: corrupted buffers on stack"));
        };
        let result = (|| {
            if frames.is_jvm_null() {
                return Err(_npe("frames_array is null"));
            }
            if frames.len()? < start_index + batch_size {
                return Err(JvmError::illegal_argument("not enough space in buffers"));
            }
            let had_more = stream.cursor < stream.frames.len();
            let end_index = Self::__fill_in_frames(mode, &mut stream, batch_size, start_index, &frames)?;
            if had_more && end_index == start_index {
                return Err(_internal_error("doStackWalk: decode frames failed"));
            }
            Ok(end_index)
        })();
        ANCHORS.with(|t| t.borrow_mut().insert(anchor, stream));
        result
    }
}
