import java.beans.PropertyEditorManager;
import java.beans.PropertyEditorSupport;

/**
 * java.beans PropertyEditor：内建编辑器（int/String）、自定义注册、
 * setAsText/getAsText 与变更监听（spring-beans 属性编辑器族的 JDK 地基）。
 */
public class TestBeansPropertyEditor {

    public static class Level {
        final int v;

        Level(int v) {
            this.v = v;
        }

        @Override
        public String toString() {
            return "Level(" + v + ")";
        }
    }

    public static class LevelEditor extends PropertyEditorSupport {
        @Override
        public void setAsText(String text) {
            setValue(new Level(Integer.parseInt(text.trim())));
        }

        @Override
        public String getAsText() {
            return String.valueOf(((Level) getValue()).v);
        }
    }

    public static void main(String[] args) {
        // 内建编辑器
        java.beans.PropertyEditor intEd = PropertyEditorManager.findEditor(int.class);
        intEd.setAsText("42");
        System.out.println("int=" + intEd.getValue());
        System.out.println("int-back=" + intEd.getAsText());
        // 边界：内建 IntegerEditor 不 trim，前导空格直接 NumberFormatException
        java.beans.PropertyEditor intEd2 = PropertyEditorManager.findEditor(int.class);
        try {
            intEd2.setAsText(" 42 ");
        } catch (NumberFormatException e) {
            System.out.println("int-notrim-ex=" + e.getClass().getSimpleName());
        }

        java.beans.PropertyEditor strEd = PropertyEditorManager.findEditor(String.class);
        strEd.setAsText("plain");
        System.out.println("str=" + strEd.getValue());

        // 无内建 → null
        System.out.println("none=" + (PropertyEditorManager.findEditor(Level.class) == null));

        // 注册自定义 → 命中
        PropertyEditorManager.registerEditor(Level.class, LevelEditor.class);
        java.beans.PropertyEditor lvEd = PropertyEditorManager.findEditor(Level.class);
        lvEd.setAsText("7");
        System.out.println("custom=" + lvEd.getValue());
        System.out.println("custom-back=" + lvEd.getAsText());

        // 变更监听
        final int[] fired = { 0 };
        lvEd.addPropertyChangeListener(evt -> fired[0]++);
        lvEd.setAsText("9");
        System.out.println("fired=" + (fired[0] > 0) + " value=" + ((Level) lvEd.getValue()).v);

        // 非法文本 → NumberFormatException（不吞异常）
        try {
            lvEd.setAsText("not-a-number");
        } catch (NumberFormatException e) {
            System.out.println("bad-text-ex=" + e.getClass().getSimpleName());
        }
    }
}
