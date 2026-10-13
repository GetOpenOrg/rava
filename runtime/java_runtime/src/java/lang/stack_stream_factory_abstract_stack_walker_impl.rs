//! `java.lang.StackStreamFactory$AbstractStackWalker` 的 native 层（类 ③：VM 栈遍历的落地语义）。
//!
//! 对标 HotSpot `StackWalk::walk` / `StackWalk::fetchNextBatch` / `StackWalk::fill_in_frames`
//!（stackwalk.cpp，JDK 21）：
//! - `callStackWalk`：在本线程锚定一条帧流（帧源 `crate::vm_stack`，于 callStackWalk 帧处取快照，
//!   与 HotSpot 锚定在该帧同口径），跳过 StackWalker 实现帧（持有类为 StackWalker、AbstractStackWalker
//!   或其直接子类），再跳 `skipframes` 帧，填首批后上调 `doStackWalk(anchor, skipframes, batchSize,
//!   startIndex, endIndex)`，返回其结果；返回（含异常完成）后锚点失效；
//! - JDK 25（`cfg(jdk_ge_25)`）：mode 改为 int、fetchStackFrames 多 lastBatchFrameCount 形参；帧为
//!   ClassFrameInfo / StackFrameInfo（classOrMemberName + flags），见各填帧函数；
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

use crate::java::lang::InternalError__Methods as _;
use crate::java::lang::NullPointerException__Methods as _;
use crate::java::lang::StackStreamFactory_AbstractStackWalker__Methods as _;
use crate::java::lang::UnsupportedOperationException__Methods as _;
use crate::prelude::*;
use super::stack_stream_factory_abstract_stack_walker::StackStreamFactory_AbstractStackWalker;
use super::Class;
use crate::vm_stack::JavaFrame;
use crate::jdk::internal::vm::{Continuation, ContinuationScope};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

// 模式位（JDK 21 `FILL_CLASS_REFS_ONLY` 与 JDK 25 `CLASS_INFO_ONLY` 同值 0x2；GET_CALLER_CLASS 只在 JDK 21
// 存在，JDK 25 的 @CallerSensitive 判定移到 Java 侧 CallerClassFinder，模式里不再出现该位）
const FILL_CLASS_REFS_ONLY: i64 = 0x2;
const GET_CALLER_CLASS: i64 = 0x4;
const SHOW_HIDDEN_FRAMES: i64 = 0x20;
const FILL_LIVE_STACK_FRAMES: i64 = 0x100;

const STACK_WALKER: &str = "java/lang/StackWalker";
const ABSTRACT_STACK_WALKER: &str = "java/lang/StackStreamFactory$AbstractStackWalker";

/// 锚定的帧流随执行流存放（执行上下文块，a3-T3）：遍历回调期间虚拟线程可能换载体。
use crate::vm_stack::AnchoredWalk as Anchored;
use super::__StackStreamFactory_AbstractStackWalkerHw;

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

/// StackFrameInfo 填写（HotSpot `java_lang_StackFrameInfo::set_method_and_bci`，JDK 21）：方法身份写入
/// `memberName` 的 clazz / name / type / flags，bci 写入 StackFrameInfo。
#[cfg(not(jdk_ge_25))]
fn _fill_frame_info(info: Object, frame: &JavaFrame) {
    let Some(member) = info.0.__unsafe_ref_get("memberName") else {
        panic!("stub: java/lang/StackFrameInfo.memberName 无按名协议");
    };
    let clazz = Object::from(Class::for_class(frame.class));
    _ensure(member.0.__unsafe_ref_set("clazz", clazz), "clazz");
    _ensure(member.0.__unsafe_ref_set("name", Object::from(String::from(frame.method.name))), "name");
    _ensure(member.0.__unsafe_ref_set("type_", Object::from(String::from(frame.method.descriptor))), "type");
    _ensure(member.0.__unsafe_int_set("flags", frame.member_name_flags()), "flags");
    _ensure(info.0.__unsafe_int_set("bci", frame.bci), "bci");
}

/// ClassFrameInfo.flags 的成员位并入（构造器已写入的 RETAIN_CLASS_REF 等高位保留）
#[cfg(jdk_ge_25)]
fn _or_flags(info: &Object, frame: &JavaFrame) {
    let old = info.0.__unsafe_int_cell("flags").map_or(0, |c| c.get());
    _ensure(info.0.__unsafe_int_set("flags", old | frame.member_name_flags()), "flags");
}

/// JDK 25 仅类模式（HotSpot `java_lang_ClassFrameInfo::init_class`）：帧缓冲元素为 ClassFrameInfo，
/// classOrMemberName 写声明类，flags 并入方法的成员位
#[cfg(jdk_ge_25)]
fn _fill_class_only<T>(frames: &JArray<T>, index: i32, frame: &JavaFrame) -> Result<()>
where
    T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
{
    let info: Object = frames.get(index)?.into();
    let clazz = Object::from(Class::for_class(frame.class));
    _ensure(info.0.__unsafe_ref_set("classOrMemberName", clazz), "classOrMemberName");
    _or_flags(&info, frame);
    Ok(())
}

/// JDK 25 StackFrameInfo 填写（HotSpot `java_lang_StackFrameInfo::set_method_and_bci`）：classOrMemberName
/// 写 ResolvedMethodName（声明类在其 vmholder，JLIA.getDeclaringClass 经它取类），flags 并入成员位，写 bci。
/// HotSpot 的 name / type 由 `expandStackFrameInfo` 惰性补齐；本模型帧不带方法句柄，在此一次填齐
///（type 为描述符串，getMethodType 原生支持该形态），expandStackFrameInfo 不会被调用
#[cfg(jdk_ge_25)]
fn _fill_frame_info(info: Object, frame: &JavaFrame) {
    let mut method = crate::java::lang::invoke::ResolvedMethodName::default();
    method._init_not_null();
    let method = Object::from(method);
    _ensure(method.0.__unsafe_ref_set("vmholder", Object::from(Class::for_class(frame.class))), "vmholder");
    _ensure(info.0.__unsafe_ref_set("classOrMemberName", method), "classOrMemberName");
    _ensure(info.0.__unsafe_ref_set("name", Object::from(String::from(frame.method.name))), "name");
    _ensure(info.0.__unsafe_ref_set("type_", Object::from(String::from(frame.method.descriptor))), "type");
    _or_flags(&info, frame);
    _ensure(info.0.__unsafe_int_set("bci", frame.bci), "bci");
}

impl<R, T> __StackStreamFactory_AbstractStackWalkerHw for StackStreamFactory_AbstractStackWalker<R, T> where R: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe, T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe {




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
                // JDK 21：仅类模式的帧缓冲元素是 Class 本身。数组写入留在 native 体内：闭包分析器按 native
                // 形参建模手写体对实参数组的写入（移入自由函数则其元素值池断开，Class 元素被判恒 null）
                #[cfg(not(jdk_ge_25))]
                frames.set(index, T::from(Object::from(Class::for_class(frame.class))))?;
                #[cfg(jdk_ge_25)]
                _fill_class_only(frames, index, &frame)?;
            }
            decoded += 1;
        }
        Ok(end)
    }

    /// native `callStackWalk(long mode, int skipframes, ContinuationScope, Continuation, int batchSize,
    /// int startIndex, T[] frames)`（JDK 21）。续体参数：本模型无虚拟线程续体帧（VirtualThread 为 VM 边界类），
    /// 帧流即当前载体线程栈。
    ///
    /// 两个版本的 native 体各自直接调 `__fill_in_frames` 并直接上调 `self.doStackWalk`：闭包手写扫描器只在
    /// 本 fn 体内认 `self` 上的回调（经辅助 fn 并入的调用不再视为 `self` 接收者），上调移进 `&self` 辅助后
    /// doStackWalk 的接收者与本 native 的 this 断开，JDK 21 仅类模式帧缓冲的 Class 元素被判恒 null
    ///（failure_patterns `hw-upcall-self-lost-in-helper`）
    #[cfg(not(jdk_ge_25))]
    #[jvm_native]
    #[cfg_attr(any(), rava_hw_pub)]
    fn callStackWalk(&self, mode: i64, skipframes: i32, _cont_scope: ContinuationScope, _continuation: Continuation,
                                batch_size: i32, start_index: i32, frames: JArray<T>) -> Result<R> {
        let limit = start_index + batch_size;
        if frames.is_jvm_null() {
            return Err(_npe("frames_array is null"));
        }
        if frames.len()? < limit {
            return Err(JvmError::illegal_argument("not enough space in buffers"));
        }
        let mut stream = Self::__anchor_stream(skipframes);
        let end_index = Self::__fill_in_frames(mode, &mut stream, limit - start_index, start_index, &frames)?;
        let anchor = Self::__register_anchor(stream, start_index, end_index)?;
        let result = self.doStackWalk(anchor, skipframes, batch_size, start_index, end_index);
        crate::exec_context::state().walk_anchors.borrow_mut().remove(&anchor);
        Ok(R::from(result?))
    }

    /// native `callStackWalk(int mode, int skipframes, ContinuationScope, Continuation, int bufferSize,
    /// int startIndex, T[] frames)`（JDK 25：mode 改为 int；bufferSize 为缓冲区总长（含保留首元素），
    /// 填帧上界是 bufferSize 而非 startIndex + batchSize，HotSpot `StackWalk::walk` 同口径）
    #[cfg(jdk_ge_25)]
    #[jvm_native]
    #[cfg_attr(any(), rava_hw_pub)]
    fn callStackWalk(&self, mode: i32, skipframes: i32, _cont_scope: ContinuationScope, _continuation: Continuation,
                                batch_size: i32, start_index: i32, frames: JArray<T>) -> Result<R> {
        let mode = i64::from(mode);
        let limit = batch_size;
        if frames.is_jvm_null() {
            return Err(_npe("frames_array is null"));
        }
        if frames.len()? < limit {
            return Err(JvmError::illegal_argument("not enough space in buffers"));
        }
        let mut stream = Self::__anchor_stream(skipframes);
        let end_index = Self::__fill_in_frames(mode, &mut stream, limit - start_index, start_index, &frames)?;
        let anchor = Self::__register_anchor(stream, start_index, end_index)?;
        let result = self.doStackWalk(anchor, skipframes, batch_size, start_index, end_index);
        crate::exec_context::state().walk_anchors.borrow_mut().remove(&anchor);
        Ok(R::from(result?))
    }

    /// 本线程栈快照为帧流：跳过 StackWalker 实现帧，再跳 `skipframes` 帧
    fn __anchor_stream(skipframes: i32) -> Anchored {
        let all = crate::vm_stack::capture_java_frames();
        let mut cursor = all.iter().position(|f| !_is_walker_impl(f.class)).unwrap_or(all.len());
        cursor = (cursor + skipframes.max(0) as usize).min(all.len());
        Anchored { frames: all, cursor }
    }

    /// 首批已填：登记锚点并返回之；调用方上调 `doStackWalk`，返回（含异常完成）后移除锚点
    fn __register_anchor(stream: Anchored, start_index: i32, end_index: i32) -> Result<i64> {
        if end_index - start_index < 1 {
            return Err(_internal_error("stack walk: decode failed"));
        }
        let st = crate::exec_context::state();
        let anchor = {
            let a = st.next_walk_anchor.get().max(1);
            st.next_walk_anchor.set(if a + 1 == -1 || a + 1 == 0 { 1 } else { a + 1 });
            a
        };
        crate::exec_context::state().walk_anchors.borrow_mut().insert(anchor, stream);
        Ok(anchor)
    }

    /// native `fetchStackFrames(long mode, long anchor, int batchSize, int startIndex, T[] frames)`（JDK 21）：
    /// 按锚点续取下一批，返回 endIndex（流已到底 → startIndex）
    #[cfg(not(jdk_ge_25))]
    #[jvm_native]
    #[cfg_attr(any(), rava_hw_pub)]
    fn fetchStackFrames_l_l_i_i_arr_obj(&self, mode: i64, anchor: i64, batch_size: i32, start_index: i32,
                                            frames: JArray<T>) -> Result<i32> {
        let limit = start_index + batch_size;
        let Some(mut stream) = crate::exec_context::state().walk_anchors.borrow_mut().remove(&anchor) else {
            return Err(_internal_error("doStackWalk: corrupted buffers on stack"));
        };
        let result = (|| {
            if frames.is_jvm_null() {
                return Err(_npe("frames_array is null"));
            }
            if frames.len()? < limit {
                return Err(JvmError::illegal_argument("not enough space in buffers"));
            }
            let had_more = stream.cursor < stream.frames.len();
            let end_index = Self::__fill_in_frames(mode, &mut stream, limit - start_index, start_index, &frames)?;
            if had_more && end_index == start_index {
                return Err(_internal_error("doStackWalk: decode frames failed"));
            }
            Ok(end_index)
        })();
        crate::exec_context::state().walk_anchors.borrow_mut().insert(anchor, stream);
        result
    }

    /// native `fetchStackFrames(int mode, long anchor, int lastBatchFrameCount, int bufferSize, int startIndex,
    /// T[] frames)`（JDK 25）。lastBatchFrameCount 供 HotSpot 把栈帧流推过上一批末帧；本模型的流位置在填帧时
    /// 已停在下一个待检视帧，无需再推；bufferSize 口径同 callStackWalk
    #[cfg(jdk_ge_25)]
    #[jvm_native]
    #[cfg_attr(any(), rava_hw_pub)]
    fn fetchStackFrames_i_l_i_i_i_arr_obj(&self, mode: i32, anchor: i64, _last_batch_frame_count: i32, batch_size: i32,
                                              start_index: i32, frames: JArray<T>) -> Result<i32> {
        let mode = i64::from(mode);
        let limit = batch_size;
        let Some(mut stream) = crate::exec_context::state().walk_anchors.borrow_mut().remove(&anchor) else {
            return Err(_internal_error("doStackWalk: corrupted buffers on stack"));
        };
        let result = (|| {
            if frames.is_jvm_null() {
                return Err(_npe("frames_array is null"));
            }
            if frames.len()? < limit {
                return Err(JvmError::illegal_argument("not enough space in buffers"));
            }
            let had_more = stream.cursor < stream.frames.len();
            let end_index = Self::__fill_in_frames(mode, &mut stream, limit - start_index, start_index, &frames)?;
            if had_more && end_index == start_index {
                return Err(_internal_error("doStackWalk: decode frames failed"));
            }
            Ok(end_index)
        })();
        crate::exec_context::state().walk_anchors.borrow_mut().insert(anchor, stream);
        result
    }
}
