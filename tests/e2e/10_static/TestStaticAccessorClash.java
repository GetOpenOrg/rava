// 边界：static 字段访问器与方法 / 其它 static 字段的访问器同名
// （`tab` 的 setter 与字段 `set_tab` 的 getter 同名；字段 `mode` 与方法 `mode()` 同名；
// 字段 `flag` 的 setter 与方法 `set_flag` 同名）
public class TestStaticAccessorClash {
    static int tab = 1;
    static int set_tab = 2;
    static int mode = 3;
    static int flag = 4;

    static int mode() {
        return mode * 10;
    }

    static void set_flag(int v) {
        flag = v * 10;
    }

    public static void main(String[] args) {
        System.out.println(tab + " " + set_tab + " " + mode + " " + flag);
        tab = 11;
        set_tab = 22;
        mode = 33;
        set_flag(5);
        System.out.println(tab + " " + set_tab + " " + mode + " " + mode() + " " + flag);
        set_tab += tab;
        System.out.println(set_tab);
    }
}
