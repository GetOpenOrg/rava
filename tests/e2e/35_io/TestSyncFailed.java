// FileDescriptor.sync 失败抛 SyncFailedException("sync failed")
import java.io.*;

public class TestSyncFailed {
    public static void main(String[] args) throws Exception {
        try {
            new FileDescriptor().sync();
            System.out.println("no exception");
        } catch (SyncFailedException e) {
            System.out.println("SyncFailedException: " + e.getMessage());
        }
        File f = File.createTempFile("rava-sync", ".tmp");
        try (FileOutputStream out = new FileOutputStream(f)) {
            out.write(42);
            out.getFD().sync();
            System.out.println("synced: " + out.getFD().valid());
        } finally {
            f.delete();
        }
    }
}
