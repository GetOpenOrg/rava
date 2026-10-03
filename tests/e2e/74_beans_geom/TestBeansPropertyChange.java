import java.beans.PropertyChangeEvent;
import java.beans.PropertyChangeListener;
import java.beans.PropertyChangeSupport;

/**
 * java.beans：PropertyChangeSupport 监听器模型（jmod 覆盖计划第 4 步；
 * 通用/具名监听、相等值抑制、手工事件、hasListeners）。
 */
public class TestBeansPropertyChange {

    static int fired;

    public static void main(String[] args) {
        Object bean = new Object();
        PropertyChangeSupport pcs = new PropertyChangeSupport(bean);

        PropertyChangeListener all = evt -> {
            fired++;
            System.out.println("evt " + evt.getPropertyName() + " " + evt.getOldValue()
                    + "->" + evt.getNewValue());
        };
        pcs.addPropertyChangeListener(all);

        pcs.firePropertyChange("level", 1, 2);
        pcs.firePropertyChange("level", 2, 2);   // 新旧相等 → 抑制
        System.out.println("fired=" + fired);

        // 具名监听：只收自己的属性
        pcs.addPropertyChangeListener("only", evt ->
                System.out.println("named " + evt.getNewValue()));
        pcs.firePropertyChange("only", null, "x");
        pcs.firePropertyChange("other", null, "y");
        System.out.println("has-only=" + pcs.hasListeners("only"));
        System.out.println("has-other=" + pcs.hasListeners("other"));

        // 手工构造事件直接发（不做相等抑制）
        pcs.firePropertyChange(new PropertyChangeEvent(bean, "manual", "a", "b"));

        pcs.removePropertyChangeListener(all);
        pcs.firePropertyChange("level", 3, 4);
        System.out.println("after-remove=" + fired);
    }
}
