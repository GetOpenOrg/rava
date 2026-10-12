package com.example.cglib;

import java.lang.reflect.Method;

import org.springframework.cglib.proxy.Callback;
import org.springframework.cglib.proxy.CallbackFilter;
import org.springframework.cglib.proxy.Enhancer;
import org.springframework.cglib.proxy.MethodInterceptor;
import org.springframework.cglib.proxy.MethodProxy;
import org.springframework.cglib.proxy.NoOp;

/**
 * 特色1：CallbackFilter——根据方法特征路由到不同的拦截器。
 * get 开头的方法走拦截器 0（缓存），其余方法走 NoOp（直接放行）。
 */
public class CallbackFilterDemo {

    static class SkuService {
        public String getSku(String code) {
            System.out.println("查询商品: " + code);
            return "SKU_INFO_" + code;
        }

        public void refreshCache() {
            System.out.println("刷新缓存");
        }
    }

    public static void main(String[] args) {
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(SkuService.class);

        Callback[] callbacks = new Callback[]{
                (MethodInterceptor) (obj, method, mArgs, proxy) -> {
                    System.out.println("【缓存拦截器】" + method.getName());
                    return proxy.invokeSuper(obj, mArgs);
                },
                NoOp.INSTANCE // index 1：不增强，直接调用父类
        };
        enhancer.setCallbacks(callbacks);
        enhancer.setCallbackFilter(new CallbackFilter() {
            @Override
            public int accept(Method method) {
                // 返回值即 callbacks 数组下标
                return method.getName().startsWith("get") ? 0 : 1;
            }
        });

        SkuService proxy = (SkuService) enhancer.create();
        proxy.getSku("A001");   // 走拦截器 0
        proxy.refreshCache();   // 走 NoOp
    }
}
