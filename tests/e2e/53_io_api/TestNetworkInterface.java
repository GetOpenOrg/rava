import java.net.*;
import java.util.*;

/** NetworkInterface native（getAll / getByName0 / isLoopback0 / getMacAddr0 / getMTU0 …）：
 *  只断言与主机无关的性质——loopback 接口恒在。 */
public class TestNetworkInterface {
    public static void main(String[] args) throws Exception {
        List<NetworkInterface> all = Collections.list(NetworkInterface.getNetworkInterfaces());
        System.out.println("any interfaces: " + !all.isEmpty());
        NetworkInterface lo = all.stream().filter(n -> {
            try { return n.isLoopback(); } catch (SocketException e) { return false; }
        }).findFirst().orElse(null);
        System.out.println("loopback found: " + (lo != null));
        System.out.println("loopback up: " + lo.isUp());
        System.out.println("loopback index positive: " + (lo.getIndex() > 0));
        System.out.println("loopback mac: " + lo.getHardwareAddress());
        System.out.println("loopback mtu positive: " + (lo.getMTU() > 0));
        System.out.println("loopback displayName==name: " + lo.getName().equals(lo.getDisplayName()));
        boolean has127 = Collections.list(lo.getInetAddresses()).stream()
            .anyMatch(a -> a.getHostAddress().equals("127.0.0.1"));
        System.out.println("loopback has 127.0.0.1: " + has127);
        boolean prefix8 = lo.getInterfaceAddresses().stream()
            .anyMatch(ia -> ia.getAddress() instanceof Inet4Address && ia.getNetworkPrefixLength() == 8);
        System.out.println("loopback /8 binding: " + prefix8);
        NetworkInterface byName = NetworkInterface.getByName(lo.getName());
        System.out.println("byName equals: " + lo.equals(byName));
        NetworkInterface byIndex = NetworkInterface.getByIndex(lo.getIndex());
        System.out.println("byIndex equals: " + lo.equals(byIndex));
        System.out.println("byName missing: " + NetworkInterface.getByName("no-such-if0"));
        System.out.println("virtual: " + lo.isVirtual() + " parent: " + lo.getParent());
        System.out.println("subinterfaces: " + Collections.list(lo.getSubInterfaces()).size());
        java.security.SecureRandom sr = java.security.SecureRandom.getInstance("SHA1PRNG");
        byte[] b = new byte[8];
        sr.nextBytes(b);
        System.out.println("sha1prng seeded: ok");
    }
}
