// 进程内 CPU 采样器（a3-T6 剖析用；服务器 perf_event_paranoid=4 时 perf 不可用的替代）。
// LD_PRELOAD 注入：ITIMER_PROF 定时（按进程 CPU 时间计）发 SIGPROF，处理器在被打断的线程上 backtrace()，
// 样本（线程号 + 返回地址）写入预分配缓冲；进程 exit 时连同 /proc/self/maps 落盘，由 sampler_report.py 符号化。
// 环境：SAMPLER_OUT=<输出前缀>（必需），SAMPLER_HZ（缺省 499）。
// 编译：cc -O2 -shared -fPIC -o sampler.so sampler.c
#define _GNU_SOURCE
#include <execinfo.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <sys/time.h>
#include <unistd.h>

#define DEPTH 64
#define MAX_SAMPLES (1 << 20)

struct sample {
    int tid;
    int n;
    void *pc[DEPTH];
};

static struct sample *buf;
static atomic_long next;
static const char *out;

static void on_prof(int sig, siginfo_t *si, void *uc) {
    (void)sig; (void)si; (void)uc;
    long i = atomic_fetch_add(&next, 1);
    if (i >= MAX_SAMPLES) return;
    struct sample *s = &buf[i];
    s->tid = (int)syscall(SYS_gettid);
    s->n = backtrace(s->pc, DEPTH);
}

static void dump(void) {
    if (!buf) return;
    struct itimerval off = {0};
    setitimer(ITIMER_PROF, &off, NULL);
    char path[4096];
    snprintf(path, sizeof path, "%s.%d.samples", out, getpid());
    FILE *f = fopen(path, "w");
    if (!f) return;
    long n = atomic_load(&next);
    if (n > MAX_SAMPLES) n = MAX_SAMPLES;
    fprintf(f, "pid %d samples %ld\n", getpid(), n);
    for (long i = 0; i < n; i++) {
        struct sample *s = &buf[i];
        fprintf(f, "%d", s->tid);
        for (int k = 0; k < s->n; k++) fprintf(f, " %lx", (unsigned long)s->pc[k]);
        fputc('\n', f);
    }
    fclose(f);
    snprintf(path, sizeof path, "%s.%d.maps", out, getpid());
    FILE *m = fopen("/proc/self/maps", "r"), *g = fopen(path, "w");
    if (m && g) {
        char line[4096];
        while (fgets(line, sizeof line, m)) fputs(line, g);
    }
    if (m) fclose(m);
    if (g) fclose(g);
}

__attribute__((constructor)) static void init(void) {
    out = getenv("SAMPLER_OUT");
    if (!out) return;
    int hz = getenv("SAMPLER_HZ") ? atoi(getenv("SAMPLER_HZ")) : 499;
    buf = mmap(NULL, sizeof(struct sample) * (size_t)MAX_SAMPLES, PROT_READ | PROT_WRITE,
               MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0);
    if (buf == MAP_FAILED) { buf = NULL; return; }
    void *warm[4];
    backtrace(warm, 4);  // 预载 libgcc_s，处理器内不再 dlopen
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = on_prof;
    sa.sa_flags = SA_SIGINFO | SA_RESTART;
    sigemptyset(&sa.sa_mask);
    sigaction(SIGPROF, &sa, NULL);
    struct itimerval it;
    it.it_interval.tv_sec = 0;
    it.it_interval.tv_usec = 1000000 / hz;
    it.it_value = it.it_interval;
    setitimer(ITIMER_PROF, &it, NULL);
    atexit(dump);
}
