package s0app;

import java.util.List;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

@Component
public class GreetingService {
    private final GreetingFormatter formatter;
    private final List<String> names;
    private final int repeat;

    public GreetingService(GreetingFormatter formatter,
                           @Value("${s0.names}") List<String> names,
                           @Value("${s0.repeat:1}") int repeat) {
        this.formatter = formatter;
        this.names = names;
        this.repeat = repeat;
    }

    public List<String> greetings() {
        return names.stream().map(formatter::format).toList();
    }

    public int repeat() {
        return repeat;
    }
}
