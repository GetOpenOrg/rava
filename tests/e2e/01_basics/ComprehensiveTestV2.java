import java.util.*;
import java.util.stream.*;
import java.util.function.*;

/**
 * ComprehensiveTestV2 — JNC 综合测试（当前可运行版）。
 * 场景：学生成绩管理系统，覆盖 20+ 语言特性。
 *
 * 与 V1 的差异（V1 含有当前 JNC 尚未修复的 bug）：
 *   V1 额外测试：
 *     - PriorityQueue<Student>（Comparator + addAll + poll → cast）
 *     - TreeMap<String, Student>.forEach（lambda 内 Object 值调用用户类方法）
 *     - InvalidScoreException.getBadScore()（自定义异常方法 dispatch）
 *     - String[][] 二维数组 + Stream.mapToDouble + summaryStatistics
 *     - new TreeMap<>(map) / new ArrayList<>(collection) 拷贝构造器
 *
 * 涵盖特性：
 *   OOP（继承/接口 default/泛型/枚举/record）、集合（ArrayList/HashMap/
 *   TreeMap/LinkedHashMap/HashSet/PriorityQueue<Integer>）、Stream/Lambda
 *   （filter/map/sorted/collect/Comparator.comparingDouble）、Optional、
 *   StringBuilder、StringJoiner、BitSet、Math、switch 表达式、
 *   try-with-resources、static inner class、anonymous Comparator、
 *   varargs、Comparable、Function/Predicate、Collectors.joining/
 *   partitioningBy、TreeMap.forEach（String 值）
 */
public class ComprehensiveTestV2 {

    // ── Enum ──────────────────────────────────────────────────────────────
    enum Grade {
        A_PLUS("A+", 100, 97),
        A("A",       97,  93),
        A_MINUS("A-", 93, 90),
        B_PLUS("B+",  90,  87),
        B("B",        87,  83),
        B_MINUS("B-", 83,  80),
        C_PLUS("C+",  80,  77),
        C("C",        77,  73),
        C_MINUS("C-", 73,  70),
        D("D",        70,  60),
        F("F",        60,   0);

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

    // ── Record ─────────────────────────────────────────────────────────────
    record CourseScore(String course, double score) implements Comparable<CourseScore> {
        @Override
        public int compareTo(CourseScore other) {
            return Double.compare(other.score, this.score); // descending
        }
    }

    // ── Interface with default method ──────────────────────────────────────
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

    // ── Abstract base class ────────────────────────────────────────────────
    static abstract class Person {
        protected final String name;
        protected final int id;

        Person(String name, int id) {
            this.name = name;
            this.id   = id;
        }

        public String getName() { return name; }
        public int    getId()   { return id; }
    }

    // ── Concrete class ─────────────────────────────────────────────────────
    static class Student extends Person implements Rankable, Comparable<Student> {
        private final Map<String, Double> scores;
        private final List<String> activities;

        Student(String name, int id) {
            super(name, id);
            this.scores     = new LinkedHashMap<>();
            this.activities = new ArrayList<>();
        }

        public void addScore(String course, double score) { scores.put(course, score); }
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
    }

    // ── Generic container ──────────────────────────────────────────────────
    static class Pair<A, B> {
        final A first;
        final B second;
        Pair(A first, B second) { this.first = first; this.second = second; }
        @Override public String toString() { return "(" + first + ", " + second + ")"; }
    }

    // ── AutoCloseable ──────────────────────────────────────────────────────
    static class ReportWriter implements AutoCloseable {
        private final StringBuilder buf = new StringBuilder();
        public void writeLine(String line) { buf.append(line).append("\n"); }
        public String contents() { return buf.toString(); }
        @Override public void close() { /* flush */ }
    }

    // ── Custom exception ───────────────────────────────────────────────────
    static class InvalidScoreException extends RuntimeException {
        InvalidScoreException(double score) {
            super("Invalid score: " + score + " (must be 0-100)");
        }
    }

    // ── Varargs helpers ────────────────────────────────────────────────────
    static double sum(double... values) {
        double total = 0;
        for (double v : values) total += v;
        return total;
    }

    static double validated(double score) {
        if (score < 0 || score > 100) throw new InvalidScoreException(score);
        return score;
    }

    // ══════════════════════════════════════════════════════════════════════
    // Main
    // ══════════════════════════════════════════════════════════════════════
    public static void main(String[] args) {

        // ── 1. Build roster ────────────────────────────────────────────────
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

        // ── 2. Comparable + Collections.sort ──────────────────────────────
        Collections.sort(roster); // descending by average

        System.out.println("=== Student Rankings ===");
        for (int i = 0; i < roster.size(); i++) {
            Student s = roster.get(i);
            System.out.printf("#%d  %-8s  avg=%.1f  GPA=%.2f  %s  [%s]%n",
                i + 1, s.getName(), s.average(),
                s.getGpa(), Grade.of(s.average()), s.rankLabel());
        }

        // ── 3. Course statistics (Optional.isPresent/get) ─────────────────
        System.out.println("\n=== Course Statistics ===");
        List<String> courses = new ArrayList<>();
        courses.add("CS"); courses.add("English"); courses.add("Math"); courses.add("Physics");

        for (String course : courses) {
            double cMin = Double.MAX_VALUE, cMax = Double.MIN_VALUE, cSum = 0;
            int cnt = 0;
            for (Student s : roster) {
                Optional<Double> sc = s.getScore(course);
                if (sc.isPresent()) {
                    double v = sc.get();
                    if (v < cMin) cMin = v;
                    if (v > cMax) cMax = v;
                    cSum += v;
                    cnt++;
                }
            }
            System.out.printf("%-8s  avg=%.1f  min=%.0f  max=%.0f%n",
                course, cSum / cnt, cMin, cMax);
        }

        // ── 4. Switch expression + TreeMap grouping ───────────────────────
        System.out.println("\n=== Grade Distribution ===");
        TreeMap<String, Integer> gradeDist = new TreeMap<>();
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
        // Use entrySet() iteration (not forEach) — avoid TreeMap<K,V>.forEach
        // with lambda calling user-class methods (V1 bug #BUG-2)
        for (Map.Entry<String, Integer> e : gradeDist.entrySet()) {
            System.out.println(e.getKey() + ": " + e.getValue());
        }

        // ── 5. Top scorer per course (TreeMap<String, String>) ───────────
        System.out.println("\n=== Top Scorer per Course ===");
        // Store formatted string "Name(Score)" — avoids TreeMap<String,Student>.forEach
        // with Student method calls in lambda (V1 bug #BUG-3)
        TreeMap<String, String> topPerCourse = new TreeMap<>();
        for (String course : courses) {
            String best = "";
            double bestScore = -1;
            for (Student s : roster) {
                Optional<Double> sc = s.getScore(course);
                if (sc.isPresent() && sc.get() > bestScore) {
                    bestScore = sc.get();
                    best = s.getName() + "(" + sc.get().intValue() + ")";
                }
            }
            topPerCourse.put(course, best);
        }
        topPerCourse.forEach((course, info) -> System.out.printf("%-8s  %s%n", course, info));

        // ── 6. PriorityQueue<Integer> (simple demo) ───────────────────────
        // PriorityQueue<Student> with addAll/poll→cast needs fixes (V1 bug #BUG-1, #BUG-4)
        System.out.println("\n=== PriorityQueue (min-heap) ===");
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        pq.add(85); pq.add(92); pq.add(78); pq.add(99); pq.add(68);
        StringBuilder pqOut = new StringBuilder("[");
        while (!pq.isEmpty()) {
            if (pqOut.length() > 1) pqOut.append(", ");
            pqOut.append(pq.poll());
        }
        pqOut.append("]");
        System.out.println("Sorted: " + pqOut);

        // ── 7. StringJoiner ────────────────────────────────────────────────
        System.out.println("\n=== Activity Summary ===");
        StringJoiner sj = new StringJoiner(", ", "[", "]");
        for (Student s : roster) {
            for (String act : s.getActivities()) {
                sj.add(s.getName() + ":" + act);
            }
        }
        System.out.println(sj);

        // ── 8. Try-with-resources + StringBuilder ─────────────────────────
        System.out.println("\n=== Report (try-with-resources) ===");
        try (ReportWriter w = new ReportWriter()) {
            for (Student s : roster) {
                StringBuilder sb = new StringBuilder();
                sb.append(s.getName())
                  .append(" | avg=").append(String.format("%.1f", s.average()))
                  .append(" | best: ");
                List<CourseScore> sorted = s.sortedScores();
                if (!sorted.isEmpty()) {
                    sb.append(sorted.get(0).course())
                      .append("(").append((int) sorted.get(0).score()).append(")");
                }
                w.writeLine(sb.toString());
            }
            System.out.print(w.contents());
        }

        // ── 9. Exception handling (getMessage only) ────────────────────────
        // V1 additionally tests getBadScore() — needs exception method dispatch fix
        // (V1 bug #BUG-5)
        System.out.println("=== Exception Handling ===");
        try {
            validated(105.0);
        } catch (InvalidScoreException e) {
            System.out.println("Caught: " + e.getMessage());
        }

        // ── 10. Generic Pair + Optional chain ─────────────────────────────
        System.out.println("\n=== Pair & Optional ===");
        Pair<String, Double> topPair = new Pair<>(roster.get(0).getName(), roster.get(0).average());
        Pair<String, Double> lowPair = new Pair<>(
            roster.get(roster.size() - 1).getName(),
            roster.get(roster.size() - 1).average());
        System.out.println("Highest: (" + topPair.first + ", " + topPair.second + ")");
        System.out.println("Lowest:  (" + lowPair.first + ", " + lowPair.second + ")");

        Optional<Student> topOpt = roster.stream()
                .max(Comparator.comparingDouble(Student::average));
        topOpt.ifPresent(s -> System.out.println("Optional top: " + s.getName()));

        // ── 11. Varargs + Math ────────────────────────────────────────────
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

        // ── 13. Anonymous Comparator ──────────────────────────────────────
        System.out.println("\n=== Anonymous Comparator (by name) ===");
        List<Student> byName = new ArrayList<>();
        for (Student s : roster) byName.add(s);
        byName.sort(new Comparator<Student>() {
            @Override
            public int compare(Student a, Student b) {
                return a.getName().compareTo(b.getName());
            }
        });
        StringJoiner nameList = new StringJoiner(", ");
        for (Student s : byName) nameList.add(s.getName());
        System.out.println(nameList);

        // ── 14. Stream + Lambda + Functional interfaces ───────────────────
        System.out.println("\n=== Stream + Functional Pipeline ===");
        Function<Student, String> toSummary = s -> s.getName() + "=" + Grade.of(s.average());
        Predicate<Student> honorRoll = s -> s.getGpa() >= 3.7;

        String honorNames = roster.stream()
                .filter(honorRoll)
                .map(toSummary)
                .collect(Collectors.joining(", "));
        System.out.println("Honor roll: " + honorNames);

        // ── 15. TreeMap.forEach (String → String) ─────────────────────────
        System.out.println("\n=== TreeMap by ID ===");
        TreeMap<Integer, String> idMap = new TreeMap<>();
        for (Student s : roster) {
            idMap.put(s.getId(), s.getName() + "/" + Grade.of(s.average()));
        }
        idMap.forEach((id, info) -> System.out.println("ID " + id + ": " + info));

        // ── 16. Collectors.partitioningBy ─────────────────────────────────
        System.out.println("\n=== Pass/Fail Partition ===");
        Map<Boolean, List<Student>> partition = roster.stream()
                .collect(Collectors.partitioningBy(s -> s.average() >= 70));
        System.out.println("Pass: " + partition.get(true).stream()
                .map(Student::getName).collect(Collectors.joining(", ")));
        System.out.println("Fail: " + partition.get(false).stream()
                .map(Student::getName).collect(Collectors.joining(", ")));

        // ── 17. Summary ───────────────────────────────────────────────────
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
