#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/VirtualThread",
    super_class = "java/lang/BaseVirtualThread",
    interfaces  = "",
    access      = "final",
    source      = "VirtualThread.java",
))]
pub struct VirtualThread {
    #[cfg_attr(any(), java_field(name = "scheduler", descriptor = "Ljava/util/concurrent/Executor;", access = "private final"))]
    pub scheduler: Field<Object>,
    #[cfg_attr(any(), java_field(name = "cont", descriptor = "Ljdk/internal/vm/Continuation;", access = "private final"))]
    pub cont: Field<Object>,
    #[cfg_attr(any(), java_field(name = "runContinuation", descriptor = "Ljava/lang/Runnable;", access = "private final"))]
    pub runContinuation: Field<Object>,
    #[cfg_attr(any(), java_field(name = "state", descriptor = "I", access = "private"))]
    pub state: Field<i32>,
    #[cfg_attr(any(), java_field(name = "parkPermit", descriptor = "Z", access = "private"))]
    pub parkPermit: Field<bool>,
    #[cfg_attr(any(), java_field(name = "carrierThread", descriptor = "Ljava/lang/Thread;", access = "private"))]
    pub carrierThread: Field<Object>,
    #[cfg_attr(any(), java_field(name = "termination", descriptor = "Ljava/util/concurrent/CountDownLatch;", access = "private"))]
    pub termination: Field<Object>,
}

impl VirtualThread {
    // java: continuationScope()Ljdk/internal/vm/ContinuationScope;
    pub fn continuationScope() -> Result<Object> {
        Ok(VirtualThread::VTHREAD_SCOPE())
    }

    // java: <init>(Ljava/util/concurrent/Executor;Ljava/lang/String;ILjava/lang/Runnable;)V
    pub fn new(scheduler: Object, name: String, characteristics: i32, task: Object) -> Result<Self> {
        let this = Self { scheduler: Field::new(Default::default()), cont: Field::new(Default::default()), runContinuation: Field::new(Default::default()), state: Field::new(0), parkPermit: Field::new(false), carrierThread: Field::new(Default::default()), termination: Field::new(Default::default()) };
        /* invokespecial Method java/lang/BaseVirtualThread.<init>:(Ljava/lang/String;IZ)V */
        let _t0: Object = Objects::requireNonNull(task)?;
        let _t1: Object = Thread::currentThread()?;
        let mut parent: Object = _t1;
        let mut vparent: Object = parent;
        scheduler = vparent.scheduler.get();
        scheduler = VirtualThread::DEFAULT_SCHEDULER();
        this.scheduler.set(scheduler);
        this.cont.set(VirtualThread_VThreadContinuation::new(this, task)?);
        /* TODO: invokedynamic 42 */
        this.runContinuation.set(this);
        Ok(this)
    }

    // java: runContinuation()V
    pub fn runContinuation(&self) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let _t1 = _t0.isVirtual()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = this.state()?;
        let mut initialState: i32 = _t2;
        let _t3 = this.compareAndSetState(initialState, 2i32)?;
        return Ok(());
        this.setParkPermit(0i32)?;
        return Ok(());
        this.mount()?;
        this.cont.get().run()?;
        this.unmount()?;
        let _t4 = this.cont.get().isDone()?;
        this.afterDone()?;
        this.afterYield()?;
        let mut local_2: bool = _t4;
        this.unmount()?;
        let _t5 = this.cont.get().isDone()?;
        this.afterDone()?;
        this.afterYield()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: submitRunContinuation()V
    pub fn submitRunContinuation(&self) -> Result<()> {
        let this = self;
        this.scheduler.get().execute(this.runContinuation.get())?;
        let mut ree: i32 = todo!("stack underflow");
        this.submitFailed(ree)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: lazySubmitRunContinuation(Ljava/util/concurrent/ForkJoinPool;)V
    pub fn lazySubmitRunContinuation(&self, pool: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ForkJoinTask::adapt(this.runContinuation.get())?;
        let _t1 = pool.lazySubmit(_t0)?;
        let mut ree: i32 = todo!("stack underflow");
        this.submitFailed(ree)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: externalSubmitRunContinuation(Ljava/util/concurrent/ForkJoinPool;)V
    pub fn externalSubmitRunContinuation(&self, pool: Object) -> Result<()> {
        let this = self;
        let _t0: Object = ForkJoinTask::adapt(this.runContinuation.get())?;
        let _t1 = pool.externalSubmit(_t0)?;
        let mut ree: i32 = todo!("stack underflow");
        this.submitFailed(ree)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: submitFailed(Ljava/util/concurrent/RejectedExecutionException;)V
    pub fn submitFailed(&self, ree: Object) -> Result<()> {
        let this = self;
        let mut event: VirtualThreadSubmitFailedEvent = VirtualThreadSubmitFailedEvent::new()?;
        let _t0 = event.isEnabled()?;
        let _t1 = this.threadId()?;
        event.javaThreadId.set(_t1);
        let _t2 = ree.getMessage()?;
        event.exceptionMessage.set(_t2);
        event.commit()?;
        Ok(())
    }

    // java: run(Ljava/lang/Runnable;)V
    // java: run(Ljava/lang/Runnable;)V
    pub fn run__runnab(&self, task: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.notifyJvmtiStart()?;
        let _t1: bool = VirtualThreadStartEvent::isTurnedOn()?;
        let mut event: VirtualThreadStartEvent = VirtualThreadStartEvent::new()?;
        let _t2 = this.threadId()?;
        event.javaThreadId.set(_t2);
        event.commit()?;
        let _t3: Object = Thread::scopedValueBindings()?;
        event = _t3;
        this.runWith(event, task)?;
        StackableScope::popAll()?;
        let _t4: bool = VirtualThreadEndEvent::isTurnedOn()?;
        let mut event: VirtualThreadEndEvent = VirtualThreadEndEvent::new()?;
        let _t5 = this.threadId()?;
        event.javaThreadId.set(_t5);
        event.commit()?;
        this.notifyJvmtiEnd()?;
        let mut local_4: bool = _t4;
        this.notifyJvmtiEnd()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        event = _t1;
        this.dispatchUncaughtException(event)?;
        StackableScope::popAll()?;
        let _t6: bool = VirtualThreadEndEvent::isTurnedOn()?;
        event = VirtualThreadEndEvent::new()?;
        let _t7 = this.threadId()?;
        event.javaThreadId.set(_t7);
        event.commit()?;
        this.notifyJvmtiEnd()?;
        let mut local_5: bool = _t6;
        this.notifyJvmtiEnd()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut local_6: i32 = 2i32;
        StackableScope::popAll()?;
        let _t8: bool = VirtualThreadEndEvent::isTurnedOn()?;
        let mut event: VirtualThreadEndEvent = VirtualThreadEndEvent::new()?;
        let _t9 = this.threadId()?;
        event.javaThreadId.set(_t9);
        event.commit()?;
        this.notifyJvmtiEnd()?;
        let mut local_8: bool = _t8;
        this.notifyJvmtiEnd()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: mount()V
    pub fn mount(&self) -> Result<()> {
        let this = self;
        this.notifyJvmtiMount(1i32)?;
        let _t0: Object = Thread::currentCarrierThread()?;
        let mut carrier: Object = _t0;
        this.setCarrierThread(carrier)?;
        carrier.setInterrupt()?;
        let _t1 = carrier.isInterrupted()?;
        let mut local_2: Object = this.interruptLock.get();
        /* TODO: monitorenter  */
        carrier.clearInterrupt()?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        carrier.setCurrentThread(this)?;
        Ok(())
    }

    // java: unmount()V
    pub fn unmount(&self) -> Result<()> {
        let this = self;
        let mut carrier: Object = this.carrierThread.get();
        carrier.setCurrentThread(carrier)?;
        let mut local_2: Object = this.interruptLock.get();
        /* TODO: monitorenter  */
        /* TODO: aconst_null  */
        this.interruptLock.get().setCarrierThread(this)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        carrier.clearInterrupt()?;
        this.notifyJvmtiUnmount(0i32)?;
        Ok(())
    }

    // java: switchToCarrierThread()V
    pub fn switchToCarrierThread(&self) -> Result<()> {
        let this = self;
        this.notifyJvmtiHideFrames(1i32)?;
        let mut carrier: Object = this.carrierThread.get();
        let _t0: Object = Thread::currentThread()?;
        let _t1: Object = Thread::currentCarrierThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        carrier.setCurrentThread(carrier)?;
        Ok(())
    }

    // java: switchToVirtualThread(Ljava/lang/VirtualThread;)V
    pub fn switchToVirtualThread(&self, vthread: Object) -> Result<()> {
        let this = self;
        let mut carrier: Object = vthread.carrierThread.get();
        let _t0: Object = Thread::currentCarrierThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        carrier.setCurrentThread(vthread)?;
        this.notifyJvmtiHideFrames(0i32)?;
        Ok(())
    }

    // java: executeOnCarrierThread(Ljava/util/concurrent/Callable;)Ljava/lang/Object;
    pub fn executeOnCarrierThread(&self, task: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.switchToCarrierThread()?;
        let _t1 = task.call()?;
        let mut local_2: Object = _t1;
        this.switchToVirtualThread(this)?;
        return Ok(local_2);
        let mut local_3: VirtualThread = this;
        this.switchToVirtualThread(this)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: yieldContinuation()Z
    pub fn yieldContinuation(&self) -> Result<bool> {
        let this = self;
        this.notifyJvmtiUnmount(1i32)?;
        let _t0: bool = Continuation::yield(VirtualThread::VTHREAD_SCOPE())?;
        let mut local_1: i32 = _t0;
        this.notifyJvmtiMount(0i32)?;
        return Ok(local_1);
        let mut local_2: i32 = todo!("stack underflow");
        this.notifyJvmtiMount(0i32)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: afterYield()V
    pub fn afterYield(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.state()?;
        let mut s: i32 = _t0;
        let mut newState: i32 = 7i32;
        this.setState(newState)?;
        let _t1 = this.compareAndSetState(newState, 9i32)?;
        let _t2: Object = VirtualThread::currentThread()?;
        let mut local_4: Object = _t2;
        let mut ct: Object = local_4;
        let _t3 = ct.getPool()?;
        this.lazySubmitRunContinuation(_t3)?;
        this.submitRunContinuation()?;
        return Ok(());
        this.setState(11i32)?;
        let _t4: Object = VirtualThread::currentThread()?;
        ct = _t4;
        newState = ct;
        let _t5 = newState.getQueuedTaskCount()?;
        let _t6 = newState.getPool()?;
        this.externalSubmitRunContinuation(_t6)?;
        this.submitRunContinuation()?;
        return Ok(());
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: afterDone()V
    // java: afterDone()V
    pub fn afterDone(&self) -> Result<()> {
        let this = self;
        this.afterDone(1i32)?;
        Ok(())
    }

    // java: afterDone(Z)V
    // java: afterDone(Z)V
    pub fn afterDone__z(&self, notifyContainer: bool) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(99i32)?;
        let mut termination: Object = this.termination.get();
        let _t0 = termination.getCount()?;
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        termination.countDown()?;
        let _t1 = this.threadContainer()?;
        _t1.onExit(this)?;
        this.clearReferences()?;
        Ok(())
    }

    // java: start(Ljdk/internal/vm/ThreadContainer;)V
    // java: start(Ljdk/internal/vm/ThreadContainer;)V
    pub fn start__thread(&self, container: Object) -> Result<()> {
        let this = self;
        let _t0 = this.compareAndSetState(0i32, 1i32)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.threadContainer()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setThreadContainer(container)?;
        let mut addedToContainer: i32 = 0i32;
        let mut started: i32 = 0i32;
        container.onStart(this)?;
        addedToContainer = 1i32;
        this.inheritScopedValueBindings(container)?;
        this.submitRunContinuation()?;
        started = 1i32;
        this.afterDone(addedToContainer)?;
        let mut local_4: i32 = started;
        this.afterDone(addedToContainer)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: start()V
    // java: start()V
    pub fn start(&self) -> Result<()> {
        let this = self;
        let _t0: Object = ThreadContainers::root()?;
        this.start(_t0)?;
        Ok(())
    }

    // java: run()V
    // java: run()V
    pub fn run(&self) -> Result<()> {
        let this = self;
        Ok(())
    }

    // java: park()V
    pub fn park(&self) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.getAndSetParkPermit(0i32)?;
        return Ok(());
        let mut yielded: i32 = 0i32;
        this.setState(3i32)?;
        let _t2 = this.yieldContinuation()?;
        yielded = _t2;
        let _t3: Object = Thread::currentThread()?;
        let _t4 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t5 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(2i32)?;
        let mut local_2: i32 = 3i32;
        let _t6: Object = Thread::currentThread()?;
        let _t7 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t8 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(2i32)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.parkOnCarrierThread(0i32, 0i64)?;
        Ok(())
    }

    // java: parkNanos(J)V
    pub fn parkNanos(&self, nanos: i64) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.getAndSetParkPermit(0i32)?;
        return Ok(());
        /* TODO: lcmp  */
        let _t2: i64 = System::nanoTime()?;
        let mut startTime: i64 = _t2;
        let mut yielded: i32 = 0i32;
        let _t3 = this.scheduleUnpark(nanos)?;
        let mut unparker: Object = _t3;
        this.setState(6i32)?;
        let _t4 = this.yieldContinuation()?;
        yielded = _t4;
        let _t5: Object = Thread::currentThread()?;
        let _t6 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t7 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(2i32)?;
        this.cancel(unparker)?;
        let mut remainingNanos: i32 = 6i32;
        let _t8: Object = Thread::currentThread()?;
        let _t9 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t10 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(2i32)?;
        this.cancel(unparker)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t11: i64 = System::nanoTime()?;
        remainingNanos = (nanos).wrapping_sub((_t11).wrapping_sub(startTime));
        this.parkOnCarrierThread(1i32, remainingNanos)?;
        Ok(())
    }

    // java: parkOnCarrierThread(ZJ)V
    pub fn parkOnCarrierThread(&self, timed: bool, nanos: i64) -> Result<()> {
        let this = self;
        let _t0 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut event: VirtualThreadPinnedEvent = VirtualThreadPinnedEvent::new()?;
        event.begin()?;
        let mut e: i32 = 2i32;
        /* TODO: aconst_null  */
        event = _t0;
        8i32.setState(5i32)?;
        VirtualThread::U().park(0i32, 0i64)?;
        /* TODO: lcmp  */
        VirtualThread::U().park(0i32, nanos)?;
        this.setState(2i32)?;
        let mut local_6: i64 = 0i64;
        this.setState(2i32)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setParkPermit(0i32)?;
        event.commit()?;
        e = event;
        Ok(())
    }

    // java: scheduleUnpark(J)Ljava/util/concurrent/Future;
    pub fn scheduleUnpark(&self, nanos: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.switchToCarrierThread()?;
        /* TODO: invokedynamic 370 */
        let _t1 = VirtualThread::UNPARKER().schedule(this, nanos, TimeUnit::NANOSECONDS())?;
        let mut local_3: Object = _t1;
        this.switchToVirtualThread(this)?;
        return Ok(local_3);
        let mut local_4: VirtualThread = this;
        this.switchToVirtualThread(this)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: cancel(Ljava/util/concurrent/Future;)V
    pub fn cancel(&self, future: Object) -> Result<()> {
        let this = self;
        let _t0 = future.isDone()?;
        this.switchToCarrierThread()?;
        let _t1 = future.cancel(0i32)?;
        this.switchToVirtualThread(this)?;
        let mut local_2: bool = _t0;
        this.switchToVirtualThread(this)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: unpark()V
    pub fn unpark(&self) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let mut currentThread: Object = _t0;
        let _t1 = this.getAndSetParkPermit(1i32)?;
        let _t2 = this.state()?;
        let mut s: i32 = _t2;
        let mut parked: i32 = s == 7i32;
        let _t3 = this.compareAndSetState(s, 9i32)?;
        let mut vthread: Object = currentThread;
        vthread.switchToCarrierThread()?;
        this.submitRunContinuation()?;
        this.switchToVirtualThread(vthread)?;
        let mut carrier: bool = true;
        this.switchToVirtualThread(vthread)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.submitRunContinuation()?;
        let _t4 = this.carrierThreadAccessLock()?;
        vthread = _t4;
        /* TODO: monitorenter  */
        carrier = this.carrierThread.get();
        let _t5 = this.state()?;
        s = _t5;
        VirtualThread::U().unpark(carrier)?;
        /* TODO: monitorexit  */
        let mut local_6: Object = vthread;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: tryYield()V
    pub fn tryYield(&self) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(10i32)?;
        let mut yielded: i32 = 0i32;
        let _t1 = this.yieldContinuation()?;
        yielded = _t1;
        let _t2: Object = Thread::currentThread()?;
        let _t3 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t4 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(2i32)?;
        let mut local_2: i32 = 10i32;
        let _t5: Object = Thread::currentThread()?;
        let _t6 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t7 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(2i32)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: sleepNanos(J)V
    pub fn sleepNanos(&self, nanos: i64) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.getAndClearInterrupt()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: lcmp  */
        this.tryYield()?;
        let mut remainingNanos: i64 = nanos;
        let _t2: i64 = System::nanoTime()?;
        let mut startNanos: i64 = _t2;
        loop {
            /* TODO: lcmp  */
            if 0i64<=0i32 { break; }
            this.parkNanos(remainingNanos)?;
            let _t0 = this.getAndClearInterrupt()?;
            return Err(JvmError::Custom("athrow".to_owned()));
            let _t1: i64 = System::nanoTime()?;
            remainingNanos = (nanos).wrapping_sub((_t1).wrapping_sub(startNanos));
        }
        this.setParkPermit(1i32)?;
        let mut local_7: i64 = 0i64;
        this.setParkPermit(1i32)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: joinNanos(J)Z
    pub fn joinNanos(&self, nanos: i64) -> Result<bool> {
        let this = self;
        let _t0 = this.state()?;
        return Ok(1i32);
        let _t1 = this.getTermination()?;
        let mut termination: Object = _t1;
        let _t2 = this.state()?;
        return Ok(1i32);
        /* TODO: lcmp  */
        termination.await()?;
        let _t3 = termination.await(nanos, TimeUnit::NANOSECONDS())?;
        let mut terminated: i32 = _t3;
        return Ok(0i32);
        let _t4 = this.state()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(1i32)
    }

    // java: interrupt()V
    pub fn interrupt(&self) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        this.checkAccess()?;
        let mut local_1: Object = this.interruptLock.get();
        /* TODO: monitorenter  */
        this.interrupted.set(1i32);
        let mut b: Object = this.nioBlocker.get();
        b.interrupt(this)?;
        let mut carrier: Object = this.carrierThread.get();
        carrier.setInterrupt()?;
        /* TODO: monitorexit  */
        let mut local_4: Object = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        this.interrupted.set(1i32);
        this.carrierThread.get().setInterrupt()?;
        this.unpark()?;
        Ok(())
    }

    // java: isInterrupted()Z
    pub fn isInterrupted(&self) -> Result<bool> {
        let this = self;
        Ok(this.interrupted.get())
    }

    // java: getAndClearInterrupt()Z
    pub fn getAndClearInterrupt(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut oldValue: i32 = this.interrupted.get();
        let mut local_2: Object = this.interruptLock.get();
        /* TODO: monitorenter  */
        this.interrupted.set(0i32);
        this.carrierThread.get().clearInterrupt()?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(oldValue)
    }

    // java: threadState()Ljava/lang/Thread$State;
    pub fn threadState(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.state()?;
        let mut s: i32 = _t0;
        /* TODO: lookupswitch default:202 0:124 1:128 2:147 3:186 4:190 5:190 6:186 7:194 8:194 9:143 10:186 11:143 99:198 */
        return Ok(Thread_State::NEW());
        let _t1 = this.threadContainer()?;
        return Ok(Thread_State::NEW());
        return Ok(Thread_State::RUNNABLE());
        return Ok(Thread_State::RUNNABLE());
        let _t2 = this.carrierThreadAccessLock()?;
        let mut local_2: Object = _t2;
        /* TODO: monitorenter  */
        let mut carrierThread: Object = this.carrierThread.get();
        let _t3 = carrierThread.threadState()?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        /* TODO: monitorexit  */
        let mut local_4: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(Thread_State::RUNNABLE());
        return Ok(Thread_State::RUNNABLE());
        return Ok(Thread_State::WAITING());
        return Ok(Thread_State::TIMED_WAITING());
        return Ok(Thread_State::TERMINATED());
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: alive()Z
    pub fn alive(&self) -> Result<bool> {
        let this = self;
        let mut s: i32 = this.state.get();
        Ok(s != 99i32)
    }

    // java: isTerminated()Z
    pub fn isTerminated(&self) -> Result<bool> {
        let this = self;
        Ok(this.state.get() == 99i32)
    }

    // java: asyncGetStackTrace()[Ljava/lang/StackTraceElement;
    pub fn asyncGetStackTrace(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = BaseVirtualThread::asyncGetStackTrace()?;
        let _t1 = this.tryGetStackTrace()?;
        let mut stackTrace: Vec<Object> = _t1;
        Thread::yield()?;
        Ok(stackTrace)
    }

    // java: tryGetStackTrace()[Ljava/lang/StackTraceElement;
    pub fn tryGetStackTrace(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.state()?;
        let mut initialState: i32 = (_t0&-257i32);
        /* TODO: lookupswitch default:139 0:124 1:124 2:129 3:137 4:131 5:129 6:137 7:131 8:129 9:134 10:137 11:134 99:124 */
        let mut _arr1: Vec<Object> = Vec::with_capacity(0i32 as usize);
        return Ok(_arr1);
        /* TODO: aconst_null  */
        return Ok(initialState);
        /* TODO: aconst_null  */
        return Ok(todo!("stack underflow"));
        String::new().append(&String::from(""))?;
        String::new().append(&initialState)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut suspendedState: i32 = (initialState|256i32);
        let _t2 = this.compareAndSetState(initialState, suspendedState)?;
        /* TODO: aconst_null  */
        return Ok(_t2);
        let _t3 = this.cont.get().getStackTrace()?;
        let mut stack: Vec<Object> = _t3;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(initialState)?;
        let mut resubmit: i32 = suspendedState;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.setState(initialState)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: tableswitch default:329 low:4 high:11 */
        let _t4 = this.compareAndSetState(initialState, 9i32)?;
        resubmit = _t4!=0i32;
        this.submitRunContinuation()?;
        Ok(stack)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut sb: String = String::new();
        let _t0 = this.threadId()?;
        sb.append(&_t0)?;
        let _t1 = this.getName()?;
        let mut name: String = _t1;
        let _t2 = name.isEmpty()?;
        sb.append(&String::from(","))?;
        sb.append(&name)?;
        sb.append(&String::from("]/"))?;
        let mut carrier: Object = this.carrierThread.get();
        let _t3 = this.carrierThreadAccessLock()?;
        let mut stateAsString: Object = _t3;
        /* TODO: monitorenter  */
        carrier = this.carrierThread.get();
        let _t4 = carrier.threadState()?;
        let _t5 = _t4.toString()?;
        let mut stateAsString: String = _t5;
        let _t6 = stateAsString.toLowerCase(Locale::ROOT())?;
        sb.append(&_t6)?;
        sb.append(&64i32)?;
        let _t7 = carrier.getName()?;
        sb.append(&_t7)?;
        /* TODO: monitorexit  */
        let mut local_6: Object = stateAsString;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t8 = this.threadState()?;
        let _t9 = _t8.toString()?;
        stateAsString = _t9;
        let _t10 = stateAsString.toLowerCase(Locale::ROOT())?;
        sb.append(&_t10)?;
        Ok(sb)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.threadId()?;
        Ok((_t0 as i32))
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        Ok(/* if_acmpne */ true)
    }

    // java: getTermination()Ljava/util/concurrent/CountDownLatch;
    pub fn getTermination(&self) -> Result<Object> {
        let this = self;
        let mut termination: Object = this.termination.get();
        termination = CountDownLatch::new(1i32)?;
        /* TODO: aconst_null  */
        let _t0 = termination.compareAndSetReference(VirtualThread::U(), this, VirtualThread::TERMINATION(), termination)?;
        termination = this.termination.get();
        Ok(termination)
    }

    // java: carrierThreadAccessLock()Ljava/lang/Object;
    pub fn carrierThreadAccessLock(&self) -> Result<Object> {
        let this = self;
        Ok(this.interruptLock.get())
    }

    // java: state()I
    pub fn state(&self) -> Result<i32> {
        let this = self;
        Ok(this.state.get())
    }

    // java: setState(I)V
    pub fn setState(&self, newValue: i32) -> Result<()> {
        let this = self;
        this.state.set(newValue);
        Ok(())
    }

    // java: compareAndSetState(II)Z
    pub fn compareAndSetState(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
        let this = self;
        let _t0 = VirtualThread::U().compareAndSetInt(this, VirtualThread::STATE(), expectedValue, newValue)?;
        Ok(_t0)
    }

    // java: setParkPermit(Z)V
    pub fn setParkPermit(&self, newValue: bool) -> Result<()> {
        let this = self;
        this.parkPermit.set(newValue);
        Ok(())
    }

    // java: getAndSetParkPermit(Z)Z
    pub fn getAndSetParkPermit(&self, newValue: bool) -> Result<bool> {
        let this = self;
        let _t0 = VirtualThread::U().getAndSetBoolean(this, VirtualThread::PARK_PERMIT(), newValue)?;
        return Ok(_t0);
        Ok(newValue)
    }

    // java: setCarrierThread(Ljava/lang/Thread;)V
    pub fn setCarrierThread(&self, carrier: Object) -> Result<()> {
        let this = self;
        this.carrierThread.set(carrier);
        Ok(())
    }

    // java: notifyJvmtiStart()V
    pub fn notifyJvmtiStart(&self) -> Result<()> {
        todo!("native java/lang/VirtualThread.notifyJvmtiStart")
    }

    // java: notifyJvmtiEnd()V
    pub fn notifyJvmtiEnd(&self) -> Result<()> {
        todo!("native java/lang/VirtualThread.notifyJvmtiEnd")
    }

    // java: notifyJvmtiMount(Z)V
    pub fn notifyJvmtiMount(&self, arg0: bool) -> Result<()> {
        todo!("native java/lang/VirtualThread.notifyJvmtiMount")
    }

    // java: notifyJvmtiUnmount(Z)V
    pub fn notifyJvmtiUnmount(&self, arg0: bool) -> Result<()> {
        todo!("native java/lang/VirtualThread.notifyJvmtiUnmount")
    }

    // java: notifyJvmtiHideFrames(Z)V
    pub fn notifyJvmtiHideFrames(&self, arg0: bool) -> Result<()> {
        todo!("native java/lang/VirtualThread.notifyJvmtiHideFrames")
    }

    // java: registerNatives()V
    pub fn registerNatives() -> Result<()> {
        todo!("native java/lang/VirtualThread.registerNatives")
    }

    // java: createDefaultScheduler()Ljava/util/concurrent/ForkJoinPool;
    pub fn createDefaultScheduler() -> Result<Object> {
        /* TODO: invokedynamic 544 */
        let mut factory: i32 = todo!("stack underflow");
        /* TODO: invokedynamic 548 */
        let mut pa: i32 = factory;
        let _t0: Object = AccessController::doPrivileged(pa)?;
        Ok(_t0)
    }

    // java: createDelayedTaskScheduler()Ljava/util/concurrent/ScheduledExecutorService;
    pub fn createDelayedTaskScheduler() -> Result<Object> {
        let _t0: String = GetPropertyAction::privilegedGetProperty(String::from("jdk.unparker.maxPoolSize"))?;
        let mut propValue: String = _t0;
        let _t1: i32 = Integer::parseInt(propValue)?;
        let mut poolSize: i32 = _t1;
        poolSize = 1i32;
        /* TODO: invokedynamic 571 */
        let _t2: Object = Executors::newScheduledThreadPool(propValue, poolSize)?;
        let mut stpe: Object = _t2;
        stpe.setRemoveOnCancelPolicy(1i32)?;
        Ok(stpe)
    }

    // java: tracePinningMode()I
    pub fn tracePinningMode() -> Result<i32> {
        let _t0: String = GetPropertyAction::privilegedGetProperty(String::from("jdk.tracePinnedThreads"))?;
        let mut propValue: String = _t0;
        let _t1 = propValue.length()?;
        let _t2 = String::from("full").equalsIgnoreCase(propValue)?;
        return Ok(1i32);
        let _t3 = String::from("short").equalsIgnoreCase(propValue)?;
        return Ok(2i32);
        Ok(0i32)
    }
}
