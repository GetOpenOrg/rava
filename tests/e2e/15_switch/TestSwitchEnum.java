public class TestSwitchEnum {
    enum Season { SPRING, SUMMER, FALL, WINTER }

    static String describe(Season s) {
        return switch (s) {
            case SPRING -> "warm and rainy";
            case SUMMER -> "hot and sunny";
            case FALL -> "cool and windy";
            case WINTER -> "cold and snowy";
        };
    }

    public static void main(String[] args) {
        for (Season s : Season.values()) {
            System.out.println(s + ": " + describe(s));
        }
    }
}
