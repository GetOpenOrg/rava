import javax.script.ScriptEngineManager;

/**
 * java.scripting：无引擎路径（jmod 覆盖计划 B 档空提供者组；
 * Nashorn 移除后无 js 引擎，工厂列表为空——ServiceLoader 空结果语义）。
 */
public class TestScriptEngineNone {

    public static void main(String[] args) {
        ScriptEngineManager m = new ScriptEngineManager();
        System.out.println("js-null=" + (m.getEngineByName("js") == null));
        System.out.println("factories=" + m.getEngineFactories().size());
        System.out.println("by-ext-null=" + (m.getEngineByExtension("js") == null));
        System.out.println("by-mime-null=" + (m.getEngineByMimeType("application/javascript") == null));
    }
}
