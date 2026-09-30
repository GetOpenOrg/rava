/*
 * 类加载轨迹 JVMTI agent（C5 动态对照，scripts/dyn_compare.py 编译并驱动；只用于验证，不进入转译产物）。
 *
 * 用原生 agent 而不是 java.lang.instrument：后者的装载本身（sun.instrument 的 lambda、反射调
 * premain）会在 main 之前加载整套 MethodHandle / LambdaMetafactory 基础设施，被测程序随后对这些
 * 类的加载就没有调用栈可归因。JVMTI 的 ClassLoad 回调不执行任何 Java 代码，不改变加载序列。
 *
 * 用法：java -agentpath:<lib>=<输出文件> …
 * 输出为行式文本（字段以单个空格分隔）：
 *   L <类> <线程名>              一个类被加载；其后紧跟调用栈帧，栈顶在前
 *   F <类> <方法> <描述符> <bci>  调用栈帧（类名为 binary name，`/` 分隔；隐藏类形如 `a/B$$Lambda.0x…`）
 * 调用栈包含全部帧（隐藏帧、LambdaForm 编译体、反射帧），过滤规则在 dyn_compare.py 侧。
 */
#include <jvmti.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_FRAMES 512

static FILE *out;
static jrawMonitorID lock;

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

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
    jvmtiEnv *jvmti = NULL;
    jvmtiEventCallbacks cb;
    (void)reserved;

    if (options == NULL || options[0] == '\0') {
        fprintf(stderr, "load_trace: 需要输出文件路径（-agentpath:<lib>=<file>）\n");
        return JNI_ERR;
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
    if ((*jvmti)->SetEventCallbacks(jvmti, &cb, (jint)sizeof(cb)) != JVMTI_ERROR_NONE) return JNI_ERR;
    if ((*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_LOAD, NULL)
            != JVMTI_ERROR_NONE) {
        return JNI_ERR;
    }
    return JNI_OK;
}
