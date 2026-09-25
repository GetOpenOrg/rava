import java.util.OptionalInt;
import java.util.OptionalLong;
import java.util.OptionalDouble;

public class JavaBaseOptionalPrimitiveTest {
    public static void main(String[] args) {
        // OptionalInt
        OptionalInt optInt = OptionalInt.of(42);
        System.out.println("optInt isPresent: " + optInt.isPresent());
        System.out.println("optInt getAsInt: " + optInt.getAsInt());
        System.out.println("optInt orElse: " + optInt.orElse(0));

        OptionalInt emptyInt = OptionalInt.empty();
        System.out.println("emptyInt isPresent: " + emptyInt.isPresent());
        System.out.println("emptyInt orElse: " + emptyInt.orElse(-1));

        // OptionalLong
        OptionalLong optLong = OptionalLong.of(123456789L);
        System.out.println("optLong isPresent: " + optLong.isPresent());
        System.out.println("optLong getAsLong: " + optLong.getAsLong());
        System.out.println("optLong orElse: " + optLong.orElse(0L));

        OptionalLong emptyLong = OptionalLong.empty();
        System.out.println("emptyLong isPresent: " + emptyLong.isPresent());
        System.out.println("emptyLong orElse: " + emptyLong.orElse(-1L));

        // OptionalDouble
        OptionalDouble optDbl = OptionalDouble.of(3.14);
        System.out.println("optDbl isPresent: " + optDbl.isPresent());
        System.out.println("optDbl getAsDouble: " + optDbl.getAsDouble());
        System.out.println("optDbl orElse: " + optDbl.orElse(0.0));

        OptionalDouble emptyDbl = OptionalDouble.empty();
        System.out.println("emptyDbl isPresent: " + emptyDbl.isPresent());
        System.out.println("emptyDbl orElse: " + emptyDbl.orElse(-1.0));

        // toString
        System.out.println("optInt toString: " + optInt.toString());
        System.out.println("emptyInt toString: " + emptyInt.toString());
        System.out.println("optLong toString: " + optLong.toString());
        System.out.println("optDbl toString: " + optDbl.toString());

        // equals
        System.out.println("optInt equals of(42): " + optInt.equals(OptionalInt.of(42)));
        System.out.println("optInt equals of(0): " + optInt.equals(OptionalInt.of(0)));
        System.out.println("optInt equals empty: " + optInt.equals(OptionalInt.empty()));
        System.out.println("emptyInt equals empty: " + emptyInt.equals(OptionalInt.empty()));
    }
}
