// Java 16: sealed interface + permits + final/non-sealed 子类
public class SealedTypes {
    public static void main(String[] args) {
        PlayStationButton btn = new Circle();
        btn.press();
        SealedTypes.printName();
    }

    public static void printName() {
        System.out.println("My name is Neo");
    }
}

sealed interface PlayStationButton permits Circle, Square {
    void press();
}

final class Circle implements PlayStationButton {
    public void press() {
        System.out.println("Button was pressed");
    }
}

final class Square implements PlayStationButton {
    public void press() {
        System.out.println("Button was pressed");
    }
}
