import java.util.Arrays;

public class TestComparable {

    static class Student implements Comparable<Student> {
        String name;
        double gpa;

        Student(String name, double gpa) {
            this.name = name;
            this.gpa = gpa;
        }

        @Override
        public int compareTo(Student other) {
            // sort by gpa descending, then name ascending
            int cmp = Double.compare(other.gpa, this.gpa);
            if (cmp != 0) return cmp;
            return this.name.compareTo(other.name);
        }

        @Override
        public String toString() { return name + ":" + gpa; }
    }

    static class Version implements Comparable<Version> {
        int major, minor, patch;

        Version(int major, int minor, int patch) {
            this.major = major;
            this.minor = minor;
            this.patch = patch;
        }

        @Override
        public int compareTo(Version other) {
            if (major != other.major) return Integer.compare(major, other.major);
            if (minor != other.minor) return Integer.compare(minor, other.minor);
            return Integer.compare(patch, other.patch);
        }

        @Override
        public String toString() { return major + "." + minor + "." + patch; }
    }

    public static void main(String[] args) {
        Student[] students = {
            new Student("Charlie", 3.5),
            new Student("Alice", 3.9),
            new Student("Bob", 3.9),
            new Student("Dave", 3.2)
        };
        Arrays.sort(students);
        for (Student s : students) {
            System.out.println(s);
        }
        // Alice:3.9, Bob:3.9, Charlie:3.5, Dave:3.2

        Version v1 = new Version(1, 2, 3);
        Version v2 = new Version(1, 2, 4);
        Version v3 = new Version(2, 0, 0);
        Version v4 = new Version(1, 2, 3);

        System.out.println(v1.compareTo(v2) < 0);  // true
        System.out.println(v2.compareTo(v3) < 0);  // true
        System.out.println(v1.compareTo(v4) == 0); // true
        System.out.println(v3.compareTo(v1) > 0);  // true

        Version[] versions = { v3, v1, v2 };
        Arrays.sort(versions);
        for (Version v : versions) {
            System.out.println(v);
        }
        // 1.2.3, 1.2.4, 2.0.0
    }
}
