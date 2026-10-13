
import java.util.ServiceLoader;

import org.slf4j.spi.SLF4JServiceProvider;

/**
 * slf4j-api pilot A3：SPI 绑定探测路径（C-SPI 静态目录的实证面）。
 * 通过 ServiceLoader 枚举 classpath 上的 SLF4JServiceProvider，
 * 打印提供者与版本（当前项目配 logback，结果确定）。
 */
public class Slf4jSpiMain {

    public static void main(String[] args) {
        var providers = new java.util.ArrayList<String>();
        for (SLF4JServiceProvider p : ServiceLoader.load(SLF4JServiceProvider.class)) {
            providers.add(p.getClass().getName() + " v" + p.getRequestedApiVersion());
        }
        java.util.Collections.sort(providers);
        System.out.println("providers=" + providers.size());
        for (String p : providers) {
            System.out.println("  " + p);
        }

        // LoggerFactory 初始化后仍能正常拿 Logger（绑定协商成功）
        String name = org.slf4j.LoggerFactory.getLogger(Slf4jSpiMain.class).getName();
        System.out.println("PASS logger-initialized " + name.equals(Slf4jSpiMain.class.getName()));
        System.out.println("DONE Slf4jSpiMain");
    }
}
