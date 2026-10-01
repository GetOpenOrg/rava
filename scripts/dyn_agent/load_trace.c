/*
 * 类加载轨迹 JVMTI agent（C5 动态对照，scripts/dyn_compare.py 编译并驱动；只用于验证，不进入转译产物）。
 *
 * 用原生 agent 而不是 java.lang.instrument：后者的装载本身（sun.instrument 的 lambda、反射调
 * premain）会在 main 之前加载整套 MethodHandle / LambdaMetafactory 基础设施，被测程序随后对这些
 * 类的加载就没有调用栈可归因。JVMTI 的 ClassLoad 回调不执行任何 Java 代码，不改变加载序列。
 *
 * 用法：java -agentpath:<lib>=<输出文件>[,methods=<主类>] …
 * 输出为行式文本（字段以单个空格分隔）：
 *   L <类> <线程名>              一个类被加载；其后紧跟调用栈帧，栈顶在前
 *   F <类> <方法> <描述符> <bci>  调用栈帧（类名为 binary name，`/` 分隔；隐藏类形如 `a/B$$Lambda.0x…`）
 *   M <类> <方法> <描述符> <调用方类> <调用方方法> <调用方描述符> <bci>
 *                                程序期（主类 main 首次进入之后）首次进入的方法及该次进入的调用方帧
 *                                （无调用方帧时三段为 `-`，bci 为 -1）；只在 `methods=` 模式下输出。
 * 调用栈包含全部帧（隐藏帧、LambdaForm 编译体、反射帧），过滤规则在 dyn_compare.py 侧。
 * `methods=` 模式开启 MethodEntry 事件（解释执行，慢一个量级；只用于验证），同样不执行 Java 代码。
 */
#include <jvmti.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MAX_FRAMES 512

static FILE *out;
static jrawMonitorID lock;

static char *binary_name(char *sig);

/* ── methods= 模式：按 jmethodID 去重的首次进入记录 ── */
static char main_class[1024];
static int program_phase;          /* 0 = main 之前（只登记，不输出）；1 = 程序期 */
static uintptr_t *seen;            /* 开放寻址集合；0 = 空槽 */
static size_t seen_cap, seen_len;

static size_t slot_of(uintptr_t k, size_t cap) {
    return (size_t)((k >> 3) * 0x9E3779B97F4A7C15ull) & (cap - 1);
}

/* 插入；已存在返回 0 */
static int seen_insert(uintptr_t k) {
    size_t i;
    if (seen_len * 2 >= seen_cap) {
        size_t ncap = seen_cap ? seen_cap * 2 : 1 << 14, j;
        uintptr_t *n = calloc(ncap, sizeof(uintptr_t));
        if (n == NULL) return 0;
        for (j = 0; j < seen_cap; j++) {
            if (seen[j]) {
                i = slot_of(seen[j], ncap);
                while (n[i]) i = (i + 1) & (ncap - 1);
                n[i] = seen[j];
            }
        }
        free(seen);
        seen = n;
        seen_cap = ncap;
    }
    i = slot_of(k, seen_cap);
    while (seen[i]) {
        if (seen[i] == k) return 0;
        i = (i + 1) & (seen_cap - 1);
    }
    seen[i] = k;
    seen_len++;
    return 1;
}

/* 方法 → (类, 名, 描述符)；成功返回 1，调用方负责 Deallocate */
static int method_names(jvmtiEnv *jvmti, jmethodID m, char **csig, char **mname, char **msig) {
    jclass decl;
    *csig = *mname = *msig = NULL;
    if ((*jvmti)->GetMethodDeclaringClass(jvmti, m, &decl) != JVMTI_ERROR_NONE) return 0;
    if ((*jvmti)->GetClassSignature(jvmti, decl, csig, NULL) != JVMTI_ERROR_NONE) return 0;
    if ((*jvmti)->GetMethodName(jvmti, m, mname, msig, NULL) != JVMTI_ERROR_NONE) return 0;
    return binary_name(*csig) != NULL;
}

static void release_names(jvmtiEnv *jvmti, char *csig, char *mname, char *msig) {
    if (csig) (*jvmti)->Deallocate(jvmti, (unsigned char *)csig);
    if (mname) (*jvmti)->Deallocate(jvmti, (unsigned char *)mname);
    if (msig) (*jvmti)->Deallocate(jvmti, (unsigned char *)msig);
}

/* "Ljava/lang/Foo;" → "java/lang/Foo"（原地截取）；数组返回 NULL */
static char *binary_name(char *sig) {
    size_t n;
    if (sig == NULL || sig[0] != 'L') return NULL;
    n = strlen(sig);
    if (n >= 2 && sig[n - 1] == ';') sig[n - 1] = '\0';
    return sig + 1;
}

static void sanitize(char *s) {
    for (; s && *s; s++) {
        if (*s == ' ' || *s == '\n' || *s == '\r' || *s == '\t') *s = '_';
    }
}

static void JNICALL on_class_load(jvmtiEnv *jvmti, JNIEnv *jni, jthread thread, jclass klass) {
    char *sig = NULL, *name;
    jvmtiThreadInfo info;
    jvmtiFrameInfo frames[MAX_FRAMES];
    jint count = 0, i;
    (void)jni;

    if ((*jvmti)->GetClassSignature(jvmti, klass, &sig, NULL) != JVMTI_ERROR_NONE) return;
    name = binary_name(sig);
    if (name == NULL) {
        (*jvmti)->Deallocate(jvmti, (unsigned char *)sig);
        return;
    }
    memset(&info, 0, sizeof(info));
    if (thread == NULL || (*jvmti)->GetThreadInfo(jvmti, thread, &info) != JVMTI_ERROR_NONE) {
        info.name = NULL;
    }
    if (thread != NULL &&
        (*jvmti)->GetStackTrace(jvmti, thread, 0, MAX_FRAMES, frames, &count) != JVMTI_ERROR_NONE) {
        count = 0;
    }

    (*jvmti)->RawMonitorEnter(jvmti, lock);
    sanitize(info.name);
    fprintf(out, "L %s %s\n", name, info.name ? info.name : "?");
    for (i = 0; i < count; i++) {
        jclass decl;
        char *csig = NULL, *mname = NULL, *msig = NULL, *cname;
        if ((*jvmti)->GetMethodDeclaringClass(jvmti, frames[i].method, &decl) != JVMTI_ERROR_NONE) continue;
        if ((*jvmti)->GetClassSignature(jvmti, decl, &csig, NULL) == JVMTI_ERROR_NONE &&
            (*jvmti)->GetMethodName(jvmti, frames[i].method, &mname, &msig, NULL) == JVMTI_ERROR_NONE) {
            cname = binary_name(csig);
            if (cname != NULL) {
                fprintf(out, "F %s %s %s %lld\n", cname, mname, msig, (long long)frames[i].location);
            }
        }
        if (csig) (*jvmti)->Deallocate(jvmti, (unsigned char *)csig);
        if (mname) (*jvmti)->Deallocate(jvmti, (unsigned char *)mname);
        if (msig) (*jvmti)->Deallocate(jvmti, (unsigned char *)msig);
    }
    fflush(out);
    (*jvmti)->RawMonitorExit(jvmti, lock);

    if (info.name) (*jvmti)->Deallocate(jvmti, (unsigned char *)info.name);
    (*jvmti)->Deallocate(jvmti, (unsigned char *)sig);
}

static void JNICALL on_method_entry(jvmtiEnv *jvmti, JNIEnv *jni, jthread thread, jmethodID method) {
    char *csig, *mname, *msig;
    (void)jni;
    (*jvmti)->RawMonitorEnter(jvmti, lock);
    if (!seen_insert((uintptr_t)method)) {
        (*jvmti)->RawMonitorExit(jvmti, lock);
        return;
    }
    if (method_names(jvmti, method, &csig, &mname, &msig)) {
        char *cname = csig + 1;   /* binary_name 已原地截去 `;` */
        if (!program_phase && strcmp(cname, main_class) == 0 && strcmp(mname, "main") == 0) {
            /* 程序期开始：清空去重集，此后首次进入的方法全部输出 */
            memset(seen, 0, seen_cap * sizeof(uintptr_t));
            seen_len = 0;
            seen_insert((uintptr_t)method);
            program_phase = 1;
        }
        if (program_phase) {
            jmethodID cm = NULL;
            jlocation loc = -1;
            char *ccsig = NULL, *cmname = NULL, *cmsig = NULL;
            if (thread != NULL && (*jvmti)->GetFrameLocation(jvmti, thread, 1, &cm, &loc) == JVMTI_ERROR_NONE &&
                cm != NULL && method_names(jvmti, cm, &ccsig, &cmname, &cmsig)) {
                fprintf(out, "M %s %s %s %s %s %s %lld\n", cname, mname, msig, ccsig + 1, cmname, cmsig, (long long)loc);
            } else {
                fprintf(out, "M %s %s %s - - - -1\n", cname, mname, msig);
            }
            release_names(jvmti, ccsig, cmname, cmsig);
        }
    }
    release_names(jvmti, csig, mname, msig);
    (*jvmti)->RawMonitorExit(jvmti, lock);
}

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
    jvmtiEnv *jvmti = NULL;
    jvmtiEventCallbacks cb;
    (void)reserved;

    if (options == NULL || options[0] == '\0') {
        fprintf(stderr, "load_trace: 需要输出文件路径（-agentpath:<lib>=<file>）\n");
        return JNI_ERR;
    }
    {
        char *m = strstr(options, ",methods=");
        if (m != NULL) {
            *m = '\0';
            strncpy(main_class, m + strlen(",methods="), sizeof(main_class) - 1);
        }
    }
    out = fopen(options, "w");
    if (out == NULL) {
        fprintf(stderr, "load_trace: 无法写入 %s\n", options);
        return JNI_ERR;
    }
    if ((*vm)->GetEnv(vm, (void **)&jvmti, JVMTI_VERSION_1_2) != JNI_OK || jvmti == NULL) {
        return JNI_ERR;
    }
    if ((*jvmti)->CreateRawMonitor(jvmti, "load_trace", &lock) != JVMTI_ERROR_NONE) return JNI_ERR;
    memset(&cb, 0, sizeof(cb));
    cb.ClassLoad = &on_class_load;
    if (main_class[0]) {
        jvmtiCapabilities caps;
        memset(&caps, 0, sizeof(caps));
        caps.can_generate_method_entry_events = 1;
        if ((*jvmti)->AddCapabilities(jvmti, &caps) != JVMTI_ERROR_NONE) return JNI_ERR;
        cb.MethodEntry = &on_method_entry;
    }
    if ((*jvmti)->SetEventCallbacks(jvmti, &cb, (jint)sizeof(cb)) != JVMTI_ERROR_NONE) return JNI_ERR;
    if ((*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_LOAD, NULL)
            != JVMTI_ERROR_NONE) {
        return JNI_ERR;
    }
    if (main_class[0] &&
        (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_METHOD_ENTRY, NULL) != JVMTI_ERROR_NONE) {
        return JNI_ERR;
    }
    return JNI_OK;
}
