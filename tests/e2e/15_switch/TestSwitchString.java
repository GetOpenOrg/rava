public class TestSwitchString {
    static String classify(String s) {
        switch (s) {
            case "red":
            case "blue":
            case "green":
                return "primary-ish";
            case "yellow":
                return "bright";
            default:
                return "other";
        }
    }

    public static void main(String[] args) {
        System.out.println(classify("red"));
        System.out.println(classify("blue"));
        System.out.println(classify("yellow"));
        System.out.println(classify("purple"));
        System.out.println(classify("green"));
    }
}
