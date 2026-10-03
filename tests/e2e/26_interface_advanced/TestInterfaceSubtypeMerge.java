import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

// 边界：同一局部在两个分支分别赋具体类（ArrayList）与接口（List）值，合并类型取接口
// （同 jline LineReaderImpl.redisplay 的 newLines / rightPromptLines）
public class TestInterfaceSubtypeMerge {
    static List<String> split(String s) {
        return new ArrayList<>(Arrays.asList(s.split(",")));
    }

    static String layout(String text, String right, int columns) {
        List<String> newLines;
        if (columns <= 0) {
            newLines = new ArrayList<>();
            newLines.add(text);
        } else {
            newLines = split(text);
        }
        List<String> rightLines;
        if (right.length() == 0 || columns <= 0) {
            rightLines = new ArrayList<>();
        } else {
            rightLines = split(right);
        }
        while (newLines.size() < rightLines.size()) {
            newLines.add("");
        }
        for (int i = 0; i < rightLines.size(); i++) {
            String line = rightLines.get(i);
            newLines.set(i, newLines.get(i) + "|" + line);
        }
        return String.join("/", newLines);
    }

    public static void main(String[] args) {
        System.out.println(layout("a,b", "x,y,z", 10));
        System.out.println(layout("a,b", "", 10));
        System.out.println(layout("a,b", "x", 0));
    }
}
