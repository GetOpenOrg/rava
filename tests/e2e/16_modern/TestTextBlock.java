public class TestTextBlock {
    public static void main(String[] args) {
        String json = """
                {
                    "name": "Alice",
                    "age": 30
                }
                """;
        System.out.println(json.trim());

        String html = """
                <html>
                  <body>Hello</body>
                </html>""";
        System.out.println(html);

        // Text block preserves indentation relative to closing delimiter
        String sql = """
                SELECT *
                FROM users
                WHERE age > 18
                """;
        System.out.println(sql.trim());
    }
}
