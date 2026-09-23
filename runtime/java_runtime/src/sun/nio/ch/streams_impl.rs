//! `sun/nio/ch/Streams` 手写伴生：POSIX 原生族档 A（切入序 6）。
//!
//! `of(ReadableByteChannel)` / `of(WritableByteChannel)`：JDK 对 SocketChannel
//! 的特化分支（SocketInputStream/SocketOutputStream）不在档 A 语料面，通道
//! 形态恒为 FileChannelImpl——统一走 ChannelInputStream/ChannelOutputStream
//! （档 A 落差记录：socket 通道族留档 B）。

use crate::prelude::*;
use super::streams::implref::Streams;
use crate::java::io::InputStream;
use crate::java::io::OutputStream;

impl super::streams::implref::Streams {
    /// `of(ReadableByteChannel)`：读通道 → InputStream。
    #[jvm_boundary]
    pub fn of_readablebytechannel(ch: Object) -> Result<InputStream> {
        let stream = super::super::ch::channel_input_stream::ChannelInputStream::new(ch)?;
        Ok(stream.into())
    }

    /// `of(WritableByteChannel)`：写通道 → OutputStream。
    #[jvm_boundary]
    pub fn of_writablebytechannel(ch: Object) -> Result<OutputStream> {
        let stream = super::super::ch::channel_output_stream::ChannelOutputStream::new(ch)?;
        Ok(stream.into())
    }
}
