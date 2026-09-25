import java.util.*;
import java.util.stream.*;
import java.util.function.*;

/**
 * ComprehensiveTest — 覆盖 JNC 所有主要语言特性的综合测试。
 * 场景：学生成绩管理系统（录入、统计、排名、分析）
 *
 * 涵盖特性：
 *   OOP（继承/接口/泛型/枚举/record）、集合（ArrayList/HashMap/TreeMap/
 *   PriorityQueue/LinkedHashMap）、Stream/Lambda、异常、StringBuilder、
 *   Math、switch 表达式、try-with-resources、static inner class、
 *   anonymous class、varargs、boxing/unboxing、Comparable/Comparator、
 *   Optional、BitSet、StringJoiner
 */
public class ComprehensiveTest {

    // ── Enum ──────────────────────────────────────────────────────────────
    enum Grade {
        A_PLUS("A+", 100, 97),
        A("A",    97,  93),
        A_MINUS("A-", 93, 90),
        B_PLUS("B+", 90, 87),
        B("B",    87,  83),
        B_MINUS("B-", 83, 80),
        C_PLUS("C+", 80, 77),
        C("C",    77,  73),
        C_MINUS("C-", 73, 70),
        D("D",    70,  60),
        F("F",    60,   0);

        private final String label;
        private final int maxScore;
        private final int minScore;

        Grade(String label, int max, int min) {
            this.label    = label;
            this.maxScore = max;
            this.minScore = min;
        }

        public static Grade of(double score) {
            int s = (int) score;
            for (Grade g : values()) {
                if (s >= g.minScore && s < g.maxScore) return g;
            }
            return F;
        }

        @Override public String toString() { return label; }
    }

    // ── Record ────────────────────────────────────────────────────────────
    record CourseScore(String course, double score) implements Comparable<CourseScore> {
        @Override
        public int compareTo(CourseScore other) {
            return Double.compare(other.score, this.score); // descending
        }
    }

    // ── Interface with default method ─────────────────────────────────────
    interface Rankable {
        double getGpa();
        default String rankLabel() {
            double gpa = getGpa();
            if (gpa >= 3.7) return "Honors";
            if (gpa >= 3.0) return "Good Standing";
            if (gpa >= 2.0) return "Satisfactory";
            return "Probation";
        }
    }

    // ── Abstract base class ───────────────────────────────────────────────
    static abstract class Person {
        protected final String name;
        protected final int id;

        Person(String name, int id) {
            this.name = name;
            this.id   = id;
        }

        public String getName() { return name; }
        public int    getId()   { return id; }

        @Override
        public String toString() { return name + "#" + id; }
    }

    // ── Concrete class with generics ──────────────────────────────────────
    static class Student extends Person implements Rankable, Comparable<Student> {
        private final Map<String, Double> scores;
        private final List<String> activities;

        Student(String name, int id) {
            super(name, id);
            this.scores     = new LinkedHashMap<>();
            this.activities = new ArrayList<>();
        }

        public void addScore(String course, double score) {
            scores.put(course, score);
        }

        public void addActivity(String act) { activities.add(act); }

        public Optional<Double> getScore(String course) {
            return Optional.ofNullable(scores.get(course));
        }

        public double average() {
            if (scores.isEmpty()) return 0.0;
            double sum = 0;
            for (double v : scores.values()) sum += v;
            return sum / scores.size();
        }

        @Override
        public double getGpa() {
            double avg = average();
            if (avg >= 90) return 4.0;
            if (avg >= 80) return 3.0 + (avg - 80) / 10.0;
            if (avg >= 70) return 2.0 + (avg - 70) / 10.0;
            if (avg >= 60) return 1.0 + (avg - 60) / 10.0;
            return 0.0;
        }

        public List<CourseScore> sortedScores() {
            List<CourseScore> list = new ArrayList<>();
            for (Map.Entry<String, Double> e : scores.entrySet()) {
                list.add(new CourseScore(e.getKey(), e.getValue()));
            }
            Collections.sort(list);
            return list;
        }

        public Map<String, Double> getScores()  { return scores; }
        public List<String> getActivities()     { return activities; }

        @Override
        public int compareTo(Student other) {
            return Double.compare(other.average(), this.average()); // descending
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof Student)) return false;
            return id == ((Student) o).id;
        }

        @Override
        public int hashCode() { return Integer.hashCode(id); }
    }

    // ── Generic container ─────────────────────────────────────────────────
    static class Pair<A, B> {
        final A first;
        final B second;
        Pair(A first, B second) { this.first = first; this.second = second; }
        @Override public String toString() { return "(" + first + ", " + second + ")"; }
    }

    // ── AutoCloseable for try-with-resources ──────────────────────────────
    static class ReportWriter implements AutoCloseable {
        private final StringBuilder buf = new StringBuilder();
        private boolean closed = false;

        public void writeLine(String line) {
            if (closed) throw new IllegalStateException("writer is closed");
            buf.append(line).append("\n");
        }

        public String contents() { return buf.toString(); }

        @Override
        public void close() { closed = true; }
    }

    // ── Custom exception ──────────────────────────────────────────────────
    static class InvalidScoreException extends RuntimeException {
        private final double badScore;
        InvalidScoreException(double score) {
            super("Invalid score: " + score + " (must be 0-100)");
            this.badScore = score;
        }
        public double getBadScore() { return badScore; }
    }

    // ── Utility: varargs sum ──────────────────────────────────────────────
    static double sum(double... values) {
        double total = 0;
        for (double v : values) total += v;
        return total;
    }

    // ── Utility: validate score ───────────────────────────────────────────
    static double validated(double score) {
        if (score < 0 || score > 100) throw new InvalidScoreException(score);
        return score;
    }

    // ══════════════════════════════════════════════════════════════════════
    // Main
    // ══════════════════════════════════════════════════════════════════════

    public static void main(String[] args) {

        // ── 1. Build student roster (avoid String[][] — direct construction) ──
        List<Student> roster = new ArrayList<>();

        Student alice = new Student("Alice", 101);
        alice.addScore("Math", validated(95)); alice.addScore("Physics", validated(88));
        alice.addScore("CS", validated(92));   alice.addScore("English", validated(85));
        alice.addActivity("Chess Club"); alice.addActivity("Math Olympiad");

        Student bob = new Student("Bob", 102);
        bob.addScore("Math", validated(72)); bob.addScore("Physics", validated(78));
        bob.addScore("CS", validated(85));   bob.addScore("English", validated(80));

        Student charlie = new Student("Charlie", 103);
        charlie.addScore("Math", validated(88)); charlie.addScore("Physics", validated(91));
        charlie.addScore("CS", validated(79));   charlie.addScore("English", validated(94));
        charlie.addActivity("Drama Club");

        Student diana = new Student("Diana", 104);
        diana.addScore("Math", validated(65)); diana.addScore("Physics", validated(70));
        diana.addScore("CS", validated(68));   diana.addScore("English", validated(72));

        Student eve = new Student("Eve", 105);
        eve.addScore("Math", validated(98)); eve.addScore("Physics", validated(96));
        eve.addScore("CS", validated(99));   eve.addScore("English", validated(97));
        eve.addActivity("Research Lab"); eve.addActivity("Science Bowl");
        eve.addActivity("Robotics");

        roster.add(alice); roster.add(bob); roster.add(charlie);
        roster.add(diana); roster.add(eve);

        // ── 2. Ranking ─────────────────────────────────────────────────────
        Collections.sort(roster); // by average descending

        System.out.println("=== Student Rankings ===");
        for (int i = 0; i < roster.size(); i++) {
            Student s = roster.get(i);
            System.out.printf("#%d  %-8s  avg=%.1f  GPA=%.2f  %s  [%s]%n",
                i + 1, s.getName(), s.average(),
                s.getGpa(), Grade.of(s.average()), s.rankLabel());
        }

        // ── 3. Course statistics (manual min/max/avg) ─────────────────────
        System.out.println("\n=== Course Statistics ===");

        List<String> courses = new ArrayList<>();
        courses.add("CS"); courses.add("English"); courses.add("Math"); courses.add("Physics");

        for (String course : courses) {
            double courseMin = Double.MAX_VALUE, courseMax = Double.MIN_VALUE, courseSum = 0;
            int cnt = 0;
            for (Student s : roster) {
                Optional<Double> sc = s.getScore(course);
                if (sc.isPresent()) {
                    double v = sc.get();
                    if (v < courseMin) courseMin = v;
                    if (v > courseMax) courseMax = v;
                    courseSum += v;
                    cnt++;
                }
            }
            System.out.printf("%-8s  avg=%.1f  min=%.0f  max=%.0f%n",
                course, courseSum / cnt, courseMin, courseMax);
        }

        // ── 4. Switch expression + grouping ──────────────────────────────
        System.out.println("\n=== Grade Distribution ===");

        Map<String, Integer> gradeDist = new TreeMap<>();
        for (Student s : roster) {
            Grade g = Grade.of(s.average());
            String range = switch (g) {
                case A_PLUS, A, A_MINUS -> "A range";
                case B_PLUS, B, B_MINUS -> "B range";
                case C_PLUS, C, C_MINUS -> "C range";
                default -> "Below C";
            };
            gradeDist.put(range, gradeDist.getOrDefault(range, 0) + 1);
        }
        gradeDist.forEach((range, count) -> System.out.println(range + ": " + count));

        // ── 5. Top scorer per course ──────────────────────────────────────
        System.out.println("\n=== Top Scorer per Course ===");

        TreeMap<String, Student> topPerCourse = new TreeMap<>();
        for (String course : courses) {
            Student best = null;
            double bestScore = -1;
            for (Student s : roster) {
                Optional<Double> sc = s.getScore(course);
                if (sc.isPresent() && sc.get() > bestScore) {
                    bestScore = sc.get();
                    best = s;
                }
            }
            if (best != null) topPerCourse.put(course, best);
        }
        topPerCourse.forEach((course, s) ->
            System.out.printf("%-8s  %s (%.0f)%n",
                course, s.getName(), s.getScore(course).get()));

        // ── 6. PriorityQueue (min-heap by GPA) ────────────────────────────
        System.out.println("\n=== Bottom 2 by GPA (intervention list) ===");
        PriorityQueue<Student> pq = new PriorityQueue<>(
                Comparator.comparingDouble(Student::getGpa));
        pq.addAll(roster);

        for (int i = 0; i < 2 && !pq.isEmpty(); i++) {
            Student s = pq.poll();
            System.out.printf("  %s  GPA=%.2f  (%s)%n",
                s.getName(), s.getGpa(), s.rankLabel());
        }

        // ── 7. StringBuilder + StringJoiner ───────────────────────────────
        System.out.println("\n=== Activity Summary ===");

        StringJoiner sj = new StringJoiner(", ", "[", "]");
        for (Student s : roster) {
            for (String act : s.getActivities()) {
                sj.add(s.getName() + ":" + act);
            }
        }
        System.out.println(sj);

        // ── 8. Try-with-resources ──────────────────────────────────────────
        System.out.println("\n=== Report (try-with-resources) ===");

        try (ReportWriter w = new ReportWriter()) {
            for (Student s : roster) {
                StringBuilder sb = new StringBuilder();
                sb.append(s.getName())
                  .append(" | avg=")
                  .append(String.format("%.1f", s.average()))
                  .append(" | best: ");

                List<CourseScore> sorted = s.sortedScores();
                if (!sorted.isEmpty()) {
                    sb.append(sorted.get(0).course())
                      .append("(")
                      .append((int) sorted.get(0).score())
                      .append(")");
                }
                w.writeLine(sb.toString());
            }
            System.out.print(w.contents());
        }

        // ── 9. Exception handling ──────────────────────────────────────────
        System.out.println("=== Exception Handling ===");
        try {
            validated(105.0);
        } catch (InvalidScoreException e) {
            System.out.println("Caught: " + e.getMessage());
            System.out.println("Bad score was: " + (int) e.getBadScore());
        }

        // ── 10. Generics, Pair, Optional chain ────────────────────────────
        System.out.println("\n=== Pair & Optional ===");

        Optional<Pair<Student, Student>> highLow = Optional.of(roster)
                .filter(r -> r.size() >= 2)
                .map(r -> new Pair<>(r.get(0), r.get(r.size() - 1)));

        highLow.ifPresent(p ->
            System.out.println("Highest: " + p.first.getName()
                + " (" + String.format("%.1f", p.first.average()) + ")"
                + "  Lowest: " + p.second.getName()
                + " (" + String.format("%.1f", p.second.average()) + ")"));

        // ── 11. varargs + Math ────────────────────────────────────────────
        System.out.println("\n=== Varargs & Math ===");
        double total = sum(95, 88, 92, 85);
        System.out.println("Alice total: " + (int) total);
        System.out.println("Alice avg (math): " + (int) Math.round(total / 4));
        System.out.println("sqrt(2): " + String.format("%.4f", Math.sqrt(2)));
        System.out.println("log10(1000): " + (int) Math.log10(1000));

        // ── 12. BitSet ────────────────────────────────────────────────────
        System.out.println("\n=== BitSet (passing students) ===");
        BitSet passing = new BitSet(200);
        for (Student s : roster) {
            if (s.average() >= 70) passing.set(s.getId());
        }
        int passingCount = 0;
        for (int i = 0; i < 200; i++) {
            if (passing.get(i)) passingCount++;
        }
        System.out.println("Students passing (avg>=70): " + passingCount);

        // ── 13. Anonymous class (Comparator) ──────────────────────────────
        System.out.println("\n=== Anonymous Comparator (by name) ===");

        List<Student> byName = new ArrayList<>();
        byName.addAll(roster);
        byName.sort(new Comparator<Student>() {
            @Override
            public int compare(Student a, Student b) {
                return a.getName().compareTo(b.getName());
            }
        });
        StringJoiner nameList = new StringJoiner(", ");
        for (Student s : byName) nameList.add(s.getName());
        System.out.println(nameList);

        // ── 14. Stream + Functional pipeline ──────────────────────────────
        System.out.println("\n=== Stream + Functional Pipeline ===");

        Function<Student, String> toSummary = s -> s.getName() + "=" + Grade.of(s.average());
        Predicate<Student> honorRoll = s -> s.getGpa() >= 3.7;

        String honorNames = roster.stream()
                .filter(honorRoll)
                .map(toSummary)
                .collect(Collectors.joining(", "));
        System.out.println("Honor roll: " + honorNames);

        // ── 15. TreeMap (sorted by student ID) ───────────────────────────
        System.out.println("\n=== TreeMap by ID ===");
        TreeMap<Integer, String> idMap = new TreeMap<>();
        for (Student s : roster) {
            idMap.put(s.getId(), s.getName() + "/" + Grade.of(s.average()));
        }
        idMap.forEach((id, info) -> System.out.println("ID " + id + ": " + info));

        // ── 16. Stream partitioningBy ──────────────────────────────────────
        System.out.println("\n=== Pass/Fail Partition ===");
        Map<Boolean, List<Student>> partition = roster.stream()
                .collect(Collectors.partitioningBy(s -> s.average() >= 70));
        System.out.println("Pass: " + partition.get(true).stream()
                .map(Student::getName).collect(Collectors.joining(", ")));
        System.out.println("Fail: " + partition.get(false).stream()
                .map(Student::getName).collect(Collectors.joining(", ")));

        // ── 17. Summary statistics ────────────────────────────────────────
        System.out.println("\n=== Summary Statistics ===");
        int totalStudents = roster.size();
        double classAvgSum = 0;
        for (Student s : roster) classAvgSum += s.average();
        final double classAvg = classAvgSum / totalStudents;

        long aboveAvg = roster.stream()
                .filter(s -> s.average() > classAvg)
                .count();

        System.out.println("Total students: " + totalStudents);
        System.out.println("Above class avg: " + aboveAvg);
        System.out.printf("Class average: %.2f%n", classAvg);
        System.out.println("Done.");
    }
}
