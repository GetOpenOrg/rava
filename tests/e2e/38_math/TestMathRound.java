public class TestMathRound {
    public static void main(String[] args) {
        System.out.println("round=" + Math.round(2.5) + "," + Math.round(-2.5));
        System.out.println("floor=" + Math.floor(2.7) + "," + Math.floor(-2.7));
        System.out.println("ceil=" + Math.ceil(2.3) + "," + Math.ceil(-2.3));
        System.out.println("rint=" + Math.rint(2.5) + "," + Math.rint(3.5));
        System.out.println("signum=" + Math.signum(-3.0) + "," + Math.signum(0.0));
        System.out.println("ieee=" + Math.IEEEremainder(10, 3));
        System.out.println("abs=" + Math.abs(-7));
        System.out.println("max=" + Math.max(3, 8));
        System.out.println("min=" + Math.min(3, 8));
        System.out.println("nanMin=" + Math.min(Double.NaN, 5.0));
        System.out.println("nanMax=" + Math.max(Double.NaN, 5.0));
        System.out.println("trunc=" + (int) 2.9);
    }
}
