public class AbstractInherit {
    abstract static class Vehicle {
        abstract String fuel();
        abstract int maxSpeed();
    }

    abstract static class LandVehicle extends Vehicle {
        abstract int wheels();
    }

    static class Car extends LandVehicle {
        String fuel()    { return "gasoline"; }
        int maxSpeed()   { return 200; }
        int wheels()     { return 4; }
    }

    static class Bicycle extends LandVehicle {
        String fuel()    { return "none"; }
        int maxSpeed()   { return 40; }
        int wheels()     { return 2; }
    }

    public static void main(String[] args) {
        LandVehicle car  = new Car();
        LandVehicle bike = new Bicycle();
        System.out.println(car.fuel());
        System.out.println(car.maxSpeed());
        System.out.println(car.wheels());
        System.out.println(bike.fuel());
        System.out.println(bike.maxSpeed());
        System.out.println(bike.wheels());
    }
}
