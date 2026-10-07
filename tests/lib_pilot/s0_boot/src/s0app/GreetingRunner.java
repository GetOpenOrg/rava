package s0app;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;

@Component
public class GreetingRunner implements CommandLineRunner {
    private static final Logger log = LoggerFactory.getLogger(GreetingRunner.class);

    private final GreetingService service;
    private final String appName;

    public GreetingRunner(GreetingService service, @Value("${spring.application.name}") String appName) {
        this.service = service;
        this.appName = appName;
    }

    @Override
    public void run(String... args) {
        log.info("runner start: app={} args={}", appName, args.length);
        for (int i = 0; i < service.repeat(); i++) {
            for (String g : service.greetings()) {
                System.out.println(g);
            }
        }
        log.info("runner done");
    }
}
