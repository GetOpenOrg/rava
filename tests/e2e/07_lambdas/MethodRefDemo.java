import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class MethodRefDemo {
    static class Employee {
        private String name;
        private int salary;

        Employee(String name, int salary) {
            this.name = name;
            this.salary = salary;
        }

        String getName() { return name; }
        int getSalary() { return salary; }

        @Override
        public String toString() { return name + ":" + salary; }
    }

    public static void main(String[] args) {
        List<Employee> employees = new ArrayList<>();
        employees.add(new Employee("Charlie", 50000));
        employees.add(new Employee("Alice", 60000));
        employees.add(new Employee("Bob", 55000));

        // Unbound instance method reference as Function<Employee, String>
        List<String> names = employees.stream()
                .map(Employee::getName)
                .sorted()
                .collect(Collectors.toList());

        for (String name : names) {
            System.out.println(name);
        }

        System.out.println("---");

        // Lambda (existing behavior, should still work)
        List<String> lambdaNames = employees.stream()
                .map(e -> e.getName())
                .sorted()
                .collect(Collectors.toList());

        for (String name : lambdaNames) {
            System.out.println(name);
        }
    }
}
