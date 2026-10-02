// 记录型 --flows 查询（@grow / @trace / @edge）夹具：值经形参直通返回、写入字段
public class FlowProbe {
    static class Box {
        Object v;
    }

    static Object pass(Object o) {
        return o;
    }

    public static void main(String[] args) {
        Box b = new Box();
        b.v = pass(new StringBuilder("x"));
        System.out.println(b.v);
    }
}
