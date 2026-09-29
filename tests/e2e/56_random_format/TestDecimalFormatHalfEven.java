import java.text.DecimalFormat;

// DecimalFormat 默认 HALF_EVEN：5 尾判定依赖 double 的精确值是否恰为该十进制数字。
// 360.0 / 6400.0 = 0.0562499999999999944…（非精确）→ 0.0563；0.125（精确中点）→ 0.12。
public class TestDecimalFormatHalfEven {
    public static void main(String[] args) {
        DecimalFormat f4 = new DecimalFormat("###0.0000");
        DecimalFormat f2 = new DecimalFormat("###0.00");
        double[] four = {360.0 / 6400.0, -360.0 / 6400.0, 0.00015, 0.00025, 1.00005, 2.34565};
        for (double v : four) {
            System.out.println(v + " -> " + f4.format(v));
        }
        double[] two = {0.125, 0.375, 2.675, 1.005, 0.115, 1.115, 10.245, 0.5 / 8};
        for (double v : two) {
            System.out.println(v + " -> " + f2.format(v));
        }
    }
}
