// 无特权采样剖析器（R1）：服务器 perf_event_paranoid=4 时 perf 不可用，以 LD_PRELOAD 注入。
// ITIMER_PROF 周期（RUNPROF_US，缺省 1000 µs）触发 SIGPROF，handler 记录被中断 PC 与
// backtrace()（.eh_frame 展开）前若干帧。样本数达到 RUNPROF_MAX（缺省 25000）或进程退出时，
// 以异步信号安全的 write() 把 /proc/self/maps 与样本写入 RUNPROF_OUT（缺省 runprof.raw）：
// `maps` 段原样，其后每行一个样本（十六进制地址，空格分隔）。由 report.py 符号化汇总。
#define _GNU_SOURCE
#include <execinfo.h>
#include <fcntl.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>
#include <ucontext.h>
#include <unistd.h>

#define DEPTH 24
#define MAXS 100000
static void *buf[MAXS][DEPTH];
static unsigned char nfr[MAXS];
static int ns = 0;
static int maxs = 25000;
static int dumped = 0;
static char outpath[512] = "runprof.raw";

static void put(int fd, const char *s, size_t n) {
    while (n > 0) {
        ssize_t w = write(fd, s, n);
        if (w <= 0) return;
        s += w; n -= (size_t)w;
    }
}

static void dump(void) {
    if (__atomic_load_n(&ns, __ATOMIC_RELAXED) == 0) return; // 无样本的进程（如外层 timeout）不覆盖产物
    if (__atomic_exchange_n(&dumped, 1, __ATOMIC_ACQ_REL)) return;
    struct itimerval z; memset(&z, 0, sizeof z);
    setitimer(ITIMER_PROF, &z, NULL);
    int fd = open(outpath, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd < 0) return;
    put(fd, "maps\n", 5);
    int mf = open("/proc/self/maps", O_RDONLY);
    if (mf >= 0) {
        char b[4096];
        ssize_t r;
        while ((r = read(mf, b, sizeof b)) > 0) put(fd, b, (size_t)r);
        close(mf);
    }
    put(fd, "samples\n", 8);
    int total = ns < maxs ? ns : maxs;
    char line[DEPTH * 20 + 2];
    for (int i = 0; i < total; i++) {
        int n = nfr[i], p = 0;
        if (n == 0) continue;
        for (int j = 0; j < n; j++) {
            unsigned long a = (unsigned long)buf[i][j];
            char tmp[17]; int t = 0;
            do { int d = a & 15; tmp[t++] = d < 10 ? '0' + d : 'a' + d - 10; a >>= 4; } while (a);
            while (t) line[p++] = tmp[--t];
            line[p++] = ' ';
        }
        line[p++] = '\n';
        put(fd, line, (size_t)p);
    }
    close(fd);
    put(2, "[runprof] dumped\n", 17);
}

static void on_prof(int sig, siginfo_t *si, void *uc_) {
    (void)sig; (void)si;
    int i = __atomic_fetch_add(&ns, 1, __ATOMIC_RELAXED);
    if (i >= maxs) {
        if (i == maxs) dump();
        return;
    }
    ucontext_t *uc = (ucontext_t *)uc_;
    void *frames[DEPTH + 3];
    int n = backtrace(frames, DEPTH + 3);
    // frames[0..2]：handler、信号跳板、被中断函数（与 RIP 重复）；以被中断 PC 作首帧
    buf[i][0] = (void *)uc->uc_mcontext.gregs[REG_RIP];
    int k = 1;
    for (int j = 3; j < n && k < DEPTH; j++) buf[i][k++] = frames[j];
    nfr[i] = (unsigned char)k;
}

static void on_term(int sig) {
    dump();
    signal(sig, SIG_DFL);
    raise(sig);
}

__attribute__((constructor)) static void init(void) {
    void *warm[4];
    backtrace(warm, 4); // 预加载 libgcc 展开器，避免在 handler 内首次 dlopen
    const char *out = getenv("RUNPROF_OUT");
    if (out) { strncpy(outpath, out, sizeof outpath - 1); }
    const char *mx = getenv("RUNPROF_MAX");
    if (mx) { maxs = atoi(mx); if (maxs > MAXS) maxs = MAXS; }
    struct sigaction sa; memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = on_prof;
    sa.sa_flags = SA_SIGINFO | SA_RESTART;
    sigaction(SIGPROF, &sa, NULL);
    const char *us = getenv("RUNPROF_US");
    long p = us ? atol(us) : 1000;
    struct itimerval it;
    it.it_interval.tv_sec = 0; it.it_interval.tv_usec = p;
    it.it_value = it.it_interval;
    setitimer(ITIMER_PROF, &it, NULL);
    atexit(dump);
    signal(SIGTERM, on_term); // timeout 到期：先落盘再按原信号退出
    put(2, "[runprof] armed\n", 16);
}
