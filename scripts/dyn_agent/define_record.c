/*
 * 预定义类训练运行的记录 JVMTI agent（docs/plans/2026-10-10-xsltc-translet.md §5 第 1 步；由 rava 驱动：
 * 语料构建 `rava build --train-predefined` 按需、生产构建 `rava trace` 显式。只在参考 JDK 上运行，不进入转译产物）。
 *
 * 用法：java -agentpath:<lib>=<输出目录>[,exclude=<文件>] …
 * 每个由非引导加载器定义的类（ClassFileLoadHook，排除重定义）把定义时的类文件原样写成
 * `<输出目录>/<序号>.class`（序号按定义先后，从 0 起）。隐藏类不触发 ClassFileLoadHook（JVMTI 规范），不记录。
 * 类路径 / 模块里的类同样经此回调（内建加载器从类路径定义类），它们与 rava 的类路径逐字节相同，由 rava 侧剔除；
 * 格式不合法的字节（定义随后失败）也会触发回调，同样由 rava 侧剔除（解析失败即丢弃）。
 *
 * exclude 文件：每行一个成员 `<类>.<方法>:<描述符>`（类为 `/` 分隔的 binary name），即清单里 VM 已另行承载的
 * 运行期类定义点（vm_intrinsics.toml `[[intrinsic]] kind = "class_definition"`，如 Proxy.newProxyInstance）。
 * 定义时调用栈上出现其中任一成员的类不记录：这些类在 rava 里由 VM 支持类承载，不需要预定义。
 * 回调不执行任何 Java 代码，不改变被测程序的加载序列。
 */
#include <jvmti.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_FRAMES 256
#define MAX_EXCLUDES 256

static char out_dir[4096];
static char *excludes[MAX_EXCLUDES];
static int n_excludes;
static jrawMonitorID lock;
static long seq;

/* 读 exclude 文件：每行一个成员，空行忽略 */
static void load_excludes(const char *path) {
    FILE *f = fopen(path, "r");
    char line[2048];
    if (f == NULL) {
        fprintf(stderr, "[define_record] 无法读取 exclude 文件 %s\n", path);
        return;
    }
    while (n_excludes < MAX_EXCLUDES && fgets(line, sizeof line, f)) {
        size_t n = strlen(line);
        while (n > 0 && (line[n - 1] == '\n' || line[n - 1] == '\r' || line[n - 1] == ' ')) line[--n] = 0;
        if (n > 0) excludes[n_excludes++] = strdup(line);
    }
    fclose(f);
}

/* 帧的方法是否为 exclude 成员 */
static int excluded_frame(jvmtiEnv *jvmti, jmethodID m) {
    jclass decl;
    char *csig = NULL, *mname = NULL, *msig = NULL;
    char key[4096];
    int hit = 0, i;
    size_t cl;
    if ((*jvmti)->GetMethodDeclaringClass(jvmti, m, &decl) != JVMTI_ERROR_NONE) return 0;
    if ((*jvmti)->GetClassSignature(jvmti, decl, &csig, NULL) != JVMTI_ERROR_NONE) return 0;
    if ((*jvmti)->GetMethodName(jvmti, m, &mname, &msig, NULL) == JVMTI_ERROR_NONE) {
        cl = strlen(csig);
        /* 类签名 `Lpkg/Name;` → `pkg/Name` */
        if (cl >= 2 && csig[0] == 'L' && csig[cl - 1] == ';') {
            snprintf(key, sizeof key, "%.*s.%s:%s", (int)(cl - 2), csig + 1, mname, msig);
            for (i = 0; i < n_excludes && !hit; i++) hit = strcmp(key, excludes[i]) == 0;
        }
    }
    if (csig) (*jvmti)->Deallocate(jvmti, (unsigned char *)csig);
    if (mname) (*jvmti)->Deallocate(jvmti, (unsigned char *)mname);
    if (msig) (*jvmti)->Deallocate(jvmti, (unsigned char *)msig);
    return hit;
}

static int defined_by_excluded(jvmtiEnv *jvmti, jthread thread) {
    jvmtiFrameInfo frames[MAX_FRAMES];
    jint count = 0, i;
    if (n_excludes == 0) return 0;
    if ((*jvmti)->GetStackTrace(jvmti, thread, 0, MAX_FRAMES, frames, &count) != JVMTI_ERROR_NONE) return 0;
    for (i = 0; i < count; i++) {
        if (excluded_frame(jvmti, frames[i].method)) return 1;
    }
    return 0;
}

static void JNICALL on_hook(jvmtiEnv *jvmti, JNIEnv *jni, jclass redefined, jobject loader, const char *name,
                            jobject pd, jint len, const unsigned char *data, jint *new_len, unsigned char **new_data) {
    char path[4200];
    long n;
    FILE *f;
    (void)jni; (void)name; (void)pd; (void)new_len; (void)new_data;
    if (redefined != NULL || loader == NULL) return;
    if (defined_by_excluded(jvmti, NULL)) return;
    (*jvmti)->RawMonitorEnter(jvmti, lock);
    n = seq++;
    (*jvmti)->RawMonitorExit(jvmti, lock);
    snprintf(path, sizeof path, "%s/%ld.class", out_dir, n);
    f = fopen(path, "wb");
    if (f == NULL) {
        fprintf(stderr, "[define_record] 无法写入 %s\n", path);
        return;
    }
    fwrite(data, 1, (size_t)len, f);
    fclose(f);
}

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
    jvmtiEnv *jvmti;
    jvmtiEventCallbacks cb;
    char *comma;
    (void)reserved;
    if (options == NULL || options[0] == 0) {
        fprintf(stderr, "[define_record] 用法：-agentpath:<lib>=<输出目录>[,exclude=<文件>]\n");
        return 1;
    }
    snprintf(out_dir, sizeof out_dir, "%s", options);
    comma = strchr(out_dir, ',');
    if (comma != NULL) {
        *comma = 0;
        if (strncmp(comma + 1, "exclude=", 8) == 0) load_excludes(comma + 9);
    }
    if ((*vm)->GetEnv(vm, (void **)&jvmti, JVMTI_VERSION_1_2) != JNI_OK) return 1;
    (*jvmti)->CreateRawMonitor(jvmti, "define_record", &lock);
    memset(&cb, 0, sizeof cb);
    cb.ClassFileLoadHook = &on_hook;
    if ((*jvmti)->SetEventCallbacks(jvmti, &cb, sizeof cb) != JVMTI_ERROR_NONE) return 1;
    if ((*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_FILE_LOAD_HOOK, NULL) != JVMTI_ERROR_NONE)
        return 1;
    return 0;
}
