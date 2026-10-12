package com.example.instrument;

/** 模拟昂贵的查询服务：每次调用都“查数据库”。 */
public class DataService {

    public String getData(int id) {
        System.out.println("查询数据库，耗时操作...");
        try {
            Thread.sleep(50); // 模拟慢查询
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
        return "data_" + id;
    }
}
