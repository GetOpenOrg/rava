import java.io.*;

public class SerializableDemo {
    static class Point implements Serializable {
        private static final long serialVersionUID = 1L;
        int x, y;
        Point(int x, int y) { this.x = x; this.y = y; }
        public String toString() { return "(" + x + "," + y + ")"; }
    }

    static class Named implements Serializable {
        private static final long serialVersionUID = 2L;
        String name;
        int value;
        Named(String name, int value) { this.name = name; this.value = value; }
        public String toString() { return name + "=" + value; }
    }

    public static void main(String[] args) throws Exception {
        // Serialize Point to byte array and deserialize back
        Point p = new Point(3, 7);
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        ObjectOutputStream oos = new ObjectOutputStream(baos);
        oos.writeObject(p);
        oos.flush();

        byte[] bytes = baos.toByteArray();
        System.out.println(bytes.length > 0);

        ObjectInputStream ois = new ObjectInputStream(new ByteArrayInputStream(bytes));
        Point p2 = (Point) ois.readObject();
        System.out.println(p2.x);
        System.out.println(p2.y);
        System.out.println(p2.toString());

        // Serialize Named
        Named n = new Named("count", 42);
        ByteArrayOutputStream baos2 = new ByteArrayOutputStream();
        ObjectOutputStream oos2 = new ObjectOutputStream(baos2);
        oos2.writeObject(n);
        oos2.flush();

        ObjectInputStream ois2 = new ObjectInputStream(new ByteArrayInputStream(baos2.toByteArray()));
        Named n2 = (Named) ois2.readObject();
        System.out.println(n2.toString());

        // Serialize integer
        ByteArrayOutputStream baos3 = new ByteArrayOutputStream();
        ObjectOutputStream oos3 = new ObjectOutputStream(baos3);
        oos3.writeInt(12345);
        oos3.flush();

        ObjectInputStream ois3 = new ObjectInputStream(new ByteArrayInputStream(baos3.toByteArray()));
        System.out.println(ois3.readInt());

        System.out.println("done");
    }
}
