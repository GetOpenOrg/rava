package s0app;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class AppConfig {
    @Bean
    public GreetingFormatter greetingFormatter(@Value("${s0.greeting.prefix}") String prefix) {
        return new GreetingFormatter(prefix);
    }
}
