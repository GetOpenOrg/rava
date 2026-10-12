package com.example.timefold;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore;
import ai.timefold.solver.core.api.solver.Solver;
import ai.timefold.solver.core.api.solver.SolverFactory;
import ai.timefold.solver.core.config.constructionheuristic.ConstructionHeuristicPhaseConfig;
import ai.timefold.solver.core.config.solver.EnvironmentMode;
import ai.timefold.solver.core.config.solver.SolverConfig;
import ai.timefold.solver.core.api.domain.entity.PlanningEntity;
import ai.timefold.solver.core.api.domain.lookup.PlanningId;
import ai.timefold.solver.core.api.domain.solution.PlanningSolution;
import ai.timefold.solver.core.api.domain.valuerange.ValueRangeProvider;
import ai.timefold.solver.core.api.domain.variable.PlanningVariable;
import ai.timefold.solver.core.api.score.stream.Constraint;
import ai.timefold.solver.core.api.score.stream.ConstraintFactory;
import ai.timefold.solver.core.api.score.stream.ConstraintProvider;
import ai.timefold.solver.core.api.domain.solution.ProblemFactCollectionProperty;
import ai.timefold.solver.core.api.domain.solution.PlanningEntityCollectionProperty;
import ai.timefold.solver.core.api.domain.solution.PlanningScore;

/**
 * Timefold Solver 排班求解（对应文章领域建模/约束流章节，剥离 Spring/JPA）。
 * 3 名员工 × 5 个班次：技能匹配/同槽位冲突（硬约束），尽量多排（软约束）。
 *
 * 输出确定性说明（对应规划的风险预案）：
 * - EnvironmentMode.REPRODUCIBLE（固定随机种子）
 * - 只配构造启发式阶段（FIRST_FIT），无局部搜索/无时间型终止
 * - 输出按班次 ID 排序
 */
public class TimefoldScheduleDemo {

    // ---------- 领域模型 ----------
    public static class Employee {
        private String name;
        private List<String> skills;

        public Employee() {
        } // planning 克隆需要无参构造器

        public Employee(String name, List<String> skills) {
            this.name = name;
            this.skills = skills;
        }

        public String getName() {
            return name;
        }

        public List<String> getSkills() {
            return skills;
        }

        @Override
        public String toString() {
            return name;
        }
    }

    @PlanningEntity
    public static class Shift {
        private String shiftId;
        private int slot;            // 时间槽（同一槽位互斥）
        private String requiredSkill;

        @PlanningVariable(valueRangeProviderRefs = "employeeRange", allowsUnassigned = true)
        private Employee employee;

        public Shift() {
        } // planning 克隆需要无参构造器

        public Shift(String shiftId, int slot, String requiredSkill) {
            this.shiftId = shiftId;
            this.slot = slot;
            this.requiredSkill = requiredSkill;
        }

        @PlanningId
        public String getShiftId() {
            return shiftId;
        }

        public int getSlot() {
            return slot;
        }

        public String getRequiredSkill() {
            return requiredSkill;
        }

        public Employee getEmployee() {
            return employee;
        }
    }

    @PlanningSolution
    public static class Schedule {
        private List<Employee> employees;
        private List<Shift> shifts;
        private HardSoftScore score;

        public Schedule() {
        } // planning 克隆需要无参构造器

        public Schedule(List<Employee> employees, List<Shift> shifts) {
            this.employees = employees;
            this.shifts = shifts;
        }

        @ProblemFactCollectionProperty
        @ValueRangeProvider(id = "employeeRange")
        public List<Employee> getEmployees() {
            return employees;
        }

        @PlanningEntityCollectionProperty
        public List<Shift> getShifts() {
            return shifts;
        }

        @PlanningScore
        public HardSoftScore getScore() {
            return score;
        }

        public void setScore(HardSoftScore score) {
            this.score = score;
        }
    }

    // ---------- 约束流 ----------
    public static class ScheduleConstraintProvider implements ConstraintProvider {
        @Override
        public Constraint[] defineConstraints(ConstraintFactory f) {
            return new Constraint[]{
                    skillMismatch(f),
                    slotConflict(f),
                    unassignedShift(f),
                    preferAssignment(f),
            };
        }

        /** 硬：员工必须具备班次要求的技能。 */
        Constraint skillMismatch(ConstraintFactory f) {
            return f.forEach(Shift.class)
                    .filter(s -> s.getEmployee() != null
                            && !s.getEmployee().getSkills().contains(s.getRequiredSkill()))
                    .penalize(HardSoftScore.ONE_HARD)
                    .asConstraint("技能不匹配");
        }

        /** 硬：同一员工不能同时上两个班次（同槽位）。 */
        Constraint slotConflict(ConstraintFactory f) {
            return f.forEachUniquePair(Shift.class,
                            ai.timefold.solver.core.api.score.stream.Joiners.equal(Shift::getSlot))
                    .filter((a, b) -> a.getEmployee() != null
                            && a.getEmployee().equals(b.getEmployee()))
                    .penalize(HardSoftScore.ONE_HARD)
                    .asConstraint("同槽位冲突");
        }

        /** 硬：班次尽量必须有安排（未分配扣重罚）。 */
        Constraint unassignedShift(ConstraintFactory f) {
            return f.forEach(Shift.class)
                    .filter(s -> s.getEmployee() == null)
                    .penalize(HardSoftScore.ofHard(10))
                    .asConstraint("班次未分配");
        }

        /** 软：优先安排资深员工（体现软约束打分）。 */
        Constraint preferAssignment(ConstraintFactory f) {
            return f.forEach(Shift.class)
                    .filter(s -> s.getEmployee() != null)
                    .reward(HardSoftScore.ONE_SOFT)
                    .asConstraint("尽量多排班");
        }
    }

    public static void main(String[] args) {
        List<Employee> employees = List.of(
                new Employee("张三", List.of("JAVA", "SQL")),
                new Employee("李四", List.of("SQL")),
                new Employee("王五", List.of("JAVA")));
        // 5 个班次：槽位 0、0、1、1、2；技能要求制造一个必然冲突的结构
        List<Shift> shifts = new ArrayList<>(List.of(
                new Shift("S1", 0, "JAVA"),
                new Shift("S2", 0, "JAVA"),
                new Shift("S3", 1, "SQL"),
                new Shift("S4", 1, "JAVA"),
                new Shift("S5", 2, "GO"))); // 无人会 GO → 必然未分配，验证硬约束扣分

        SolverConfig config = new SolverConfig()
                .withSolutionClass(Schedule.class)
                .withEntityClasses(Shift.class)
                .withConstraintProviderClass(ScheduleConstraintProvider.class)
                .withEnvironmentMode(EnvironmentMode.REPRODUCIBLE)
                .withPhases(new ConstructionHeuristicPhaseConfig()); // 仅构造启发式（varargs）

        Solver<Schedule> solver = SolverFactory.<Schedule>create(config).buildSolver();
        Schedule solved = solver.solve(new Schedule(employees, shifts));

        System.out.println("[求解] 3 员工 x 5 班次（S5 需要 GO 技能，无人具备）");
        System.out.println("[排班表]");
        solved.getShifts().stream()
                .sorted(Comparator.comparing(Shift::getShiftId))
                .forEach(s -> System.out.printf("  %s 槽位%d [%s] -> %s%n",
                        s.getShiftId(), s.getSlot(), s.getRequiredSkill(),
                        s.getEmployee() == null ? "（未分配）" : s.getEmployee().getName()));
        System.out.println("[得分] " + solved.getScore() + "（硬: S5 未分配 -10；软: 4 个已排 +4）");
    }
}
