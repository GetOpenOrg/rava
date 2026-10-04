// 无特权采样剖析器（R1）：服务器 perf_event_paranoid=4 时 perf 不可用，以 LD_PRELOAD 注入。
// ITIMER_PROF 周期（RUNPROF_US，缺省 1000 µs）触发 SIGPROF，handler 记录被中断 PC 与
// backtrace()（.eh_frame 展开）前若干帧；SIGTERM（timeout 结束）或进程正常退出时把样本写入
// RUNPROF_OUT（缺省 runprof.raw）：首行为主程序映射基址，其后每行一个样本（十六进制地址，空格分隔）。
// 由 scripts/runprof/report.py 符号化汇总。
#define _GNU_SOURCE
#include <execinfo.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>
#include <ucontext.h>
#include <unistd.h>

#define DEPTH 24
#define MAXS 200000
static void *buf[MAXS][DEPTH];
static unsigned char nfr[MAXS];
static int ns = 0;
static volatile int dumped = 0;

static void on_prof(int sig, siginfo_t *si, void *uc_) {
    (void)sig; (void)si;
    int i = __atomic_fetch_add(&ns, 1, __ATOMIC_RELAXED);
    if (i >= MAXS) { ns = MAXS; return; }
    ucontext_t *uc = (ucontext_t *)uc_;
    void *frames[DEPTH + 3];
    int n = backtrace(frames, DEPTH + 3);
    // frames[0..1] 为 handler 与信号跳板；以被中断 PC 作首帧
    buf[i][0] = (void *)uc->uc_mcontext.gregs[REG_RIP];
    int k = 1;
    for (int j = 3; j < n && k < DEPTH; j++) buf[i][k++] = frames[j];
    nfr[i] = (unsigned char)k;
}

static void dump(void) {
    if (dumped) return;
    dumped = 1;
    struct itimerval z; memset(&z, 0, sizeof z);
    setitimer(ITIMER_PROF, &z, NULL);
    const char *out = getenv("RUNPROF_OUT");
    FILE *f = fopen(out ? out : "runprof.raw", "w");
    if (!f) return;
    // 主程序基址：/proc/self/maps 首个映射（PIE 可执行文件）
    FILE *m = fopen("/proc/self/maps", "r");
    char line[512], exe[512] = "";
    if (m && fgets(line, sizeof line, m)) {
        unsigned long lo = strtoul(line, NULL, 16);
        char *path = strchr(line, '/');
        if (path) { strncpy(exe, path, sizeof exe - 1); }
        fprintf(f, "base %lx %s", lo, path ? path : "\n");
    }
    // 其余映射（共享库）：符号化时归入库名
    while (m && fgets(line, sizeof line, m)) {
        char *path = strchr(line, '/');
        if (strstr(line, " r-xp ") && !(path && exe[0] && strcmp(path, exe) == 0)) fprintf(f, "map %s", line);
    }
    if (m) fclose(m);
    int total = ns < MAXS ? ns : MAXS;
    for (int i = 0; i < total; i++) {
        if (nfr[i] == 0) continue;
        for (int j = 0; j < nfr[i]; j++) fprintf(f, "%lx ", (unsigned long)buf[i][j]);
        fputc('\n', f);
    }
    fclose(f);
}

static void on_term(int sig) {
    dump();
    signal(sig, SIG_DFL);
    raise(sig);
}

__attribute__((constructor)) static void init(void) {
    void *warm[4];
    backtrace(warm, 4); // 预加载 libgcc 展开器，避免在 handler 内首次 dlopen
    struct sigaction sa; memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = on_prof;
    sa.sa_flags = SA_SIGINFO | SA_RESTART;
    sigaction(SIGPROF, &sa, NULL);
    signal(SIGTERM, on_term);
    signal(SIGINT, on_term);
    const char *us = getenv("RUNPROF_US");
    long p = us ? atol(us) : 1000;
    struct itimerval it;
    it.it_interval.tv_sec = 0; it.it_interval.tv_usec = p;
    it.it_value = it.it_interval;
    setitimer(ITIMER_PROF, &it, NULL);
    atexit(dump);
}
