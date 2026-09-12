#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/channels/spi/SelectorProvider",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "SelectorProvider.java",
))]
pub struct SelectorProvider;

impl SelectorProvider {
    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "()Ljava/lang/Void;", access = "private static"))]
    pub fn checkPermission() -> Result<Object> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("selectorProvider"))?)?;
        /* TODO: aconst_null  */
        Ok(sm)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Void;)V", access = "private"))]
    // java: <init>(Ljava/lang/Void;)V
    pub fn new__void(ignore: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        let _t0: Object = SelectorProvider::checkPermission()?;
        /* invokespecial Method java/nio/channels/spi/SelectorProvider.<init>:(Ljava/lang/Void;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "provider", descriptor = "()Ljava/nio/channels/spi/SelectorProvider;", access = "public static"))]
    pub fn provider() -> Result<Object> {
        Ok(SelectorProvider$Holder::INSTANCE())
    }

    #[cfg_attr(any(), java_native(name = "openDatagramChannel", descriptor = "()Ljava/nio/channels/DatagramChannel;", access = "public abstract"))]
    pub fn openDatagramChannel(&self) -> Result<Object> {
        todo!("abstract java/nio/channels/spi/SelectorProvider.openDatagramChannel")
    }

    #[cfg_attr(any(), java_native(name = "openDatagramChannel", descriptor = "(Ljava/net/ProtocolFamily;)Ljava/nio/channels/DatagramChannel;", access = "public abstract"))]
    pub fn openDatagramChannel__protoc(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/nio/channels/spi/SelectorProvider.openDatagramChannel")
    }

    #[cfg_attr(any(), java_native(name = "openPipe", descriptor = "()Ljava/nio/channels/Pipe;", access = "public abstract"))]
    pub fn openPipe(&self) -> Result<Object> {
        todo!("abstract java/nio/channels/spi/SelectorProvider.openPipe")
    }

    #[cfg_attr(any(), java_native(name = "openSelector", descriptor = "()Ljava/nio/channels/spi/AbstractSelector;", access = "public abstract"))]
    pub fn openSelector(&self) -> Result<Object> {
        todo!("abstract java/nio/channels/spi/SelectorProvider.openSelector")
    }

    #[cfg_attr(any(), java_native(name = "openServerSocketChannel", descriptor = "()Ljava/nio/channels/ServerSocketChannel;", access = "public abstract"))]
    pub fn openServerSocketChannel(&self) -> Result<Object> {
        todo!("abstract java/nio/channels/spi/SelectorProvider.openServerSocketChannel")
    }

    #[cfg_attr(any(), java_native(name = "openSocketChannel", descriptor = "()Ljava/nio/channels/SocketChannel;", access = "public abstract"))]
    pub fn openSocketChannel(&self) -> Result<Object> {
        todo!("abstract java/nio/channels/spi/SelectorProvider.openSocketChannel")
    }

    #[cfg_attr(any(), java_method(name = "inheritedChannel", descriptor = "()Ljava/nio/channels/Channel;", access = "public"))]
    pub fn inheritedChannel(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "openSocketChannel", descriptor = "(Ljava/net/ProtocolFamily;)Ljava/nio/channels/SocketChannel;", access = "public"))]
    // java: openSocketChannel(Ljava/net/ProtocolFamily;)Ljava/nio/channels/SocketChannel;
    pub fn openSocketChannel__protoc(&self, family: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(family)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "openServerSocketChannel", descriptor = "(Ljava/net/ProtocolFamily;)Ljava/nio/channels/ServerSocketChannel;", access = "public"))]
    // java: openServerSocketChannel(Ljava/net/ProtocolFamily;)Ljava/nio/channels/ServerSocketChannel;
    pub fn openServerSocketChannel__protoc(&self, family: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(family)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }
}
