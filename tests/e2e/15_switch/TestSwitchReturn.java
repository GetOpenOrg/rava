public class TestSwitchReturn {

    static int gradePoint(int score) {
        switch (score / 10) {
            case 10:
            case 9:
                return 4;
            case 8:
                return 3;
            case 7:
                return 2;
            case 6:
                return 1;
            default:
                return 0;
        }
    }

    static String describe(int n) {
        switch (n) {
            case 1:
                return "one";
            case 2:
                if (n > 1) {
                    return "two-plus";
                }
                return "two";
            case 3:
                for (int i = 0; i < 1; i++) {
                    return "three-loop-return";
                }
            default:
                System.out.println("default path");
                return "other";
        }
    }

    static boolean earlyExit(char c) {
        switch (c) {
            case 'q':
                return true;
            case 'x':
                return true;
            default:
                break;
        }
        System.out.println("not quit");
        return false;
    }

    public static void main(String[] args) {
        for (int score : new int[]{95, 85, 75, 65, 40}) {
            System.out.println(score + " -> " + gradePoint(score));
        }

        System.out.println(describe(1));
        System.out.println(describe(2));
        System.out.println(describe(3));
        System.out.println(describe(9));

        System.out.println("q=" + earlyExit('q'));
        System.out.println("a=" + earlyExit('a'));

        // switch 表达式 + yield（Java 14+）
        int day = 3;
        String label = switch (day) {
            case 1, 7 -> "weekend";
            case 2, 3, 4, 5, 6 -> {
                String prefix = "weekday-";
                yield prefix + day;
            }
            default -> throw new IllegalStateException("bad day");
        };
        System.out.println("label=" + label);

        int value = 5;
        int computed = switch (value) {
            case 1 -> 10;
            case 2 -> 20;
            case 3 -> 30;
            default -> {
                int fallback = value * 100;
                yield fallback;
            }
        };
        System.out.println("computed=" + computed);

        // switch 表达式在返回语句中
        System.out.println("mapped=" + mapSize("large"));

        // 每个 case 都必须产出值：箭头形式无需 break
        String kind = switch (2) {
            case 1 -> "one";
            case 2 -> "two";
            default -> "many";
        };
        System.out.println("kind=" + kind);

        // switch 表达式的穷尽性由 default 保证
        try {
            System.out.println(switch (99) {
                case 1 -> "one";
                default -> throw new IllegalArgumentException("unsupported");
            });
        } catch (IllegalArgumentException e) {
            System.out.println("thrown -> " + e.getMessage());
        }

        System.out.println("done");
    }

    static int mapSize(String s) {
        return switch (s) {
            case "small" -> 1;
            case "medium" -> 5;
            case "large" -> 10;
            default -> -1;
        };
    }
}
