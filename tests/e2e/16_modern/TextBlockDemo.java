public class TextBlockDemo {
    public static void main(String[] args) {
        // Text block is a Java 15+ feature; javac normalizes indentation at compile time.
        // At the bytecode level this is just a regular String constant -- ruva needs no special handling.
        String json = """
                {
                    "name": "Alice",
                    "age": 30
                }
                """;
        System.out.println(json.strip());

        String html = """
                <html>
                    <body>Hello</body>
                </html>
                """;
        System.out.println(html.strip().length());

        // Text block with embedded newline sequences
        String msg = """
                line1
                line2
                line3
                """;
        System.out.println(msg.strip());
    }
}
