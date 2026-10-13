
import org.apache.commons.io.FilenameUtils;
import org.apache.commons.io.IOCase;

/**
 * commons-io pilot A17：FilenameUtils 路径语义矩阵（W1 #5）。
 */
public class IoFilenameMain {

    static void eq(String name, String expected, String actual) {
        boolean ok = java.util.Objects.equals(expected, actual);
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        eq("normalize", "a/b/c.txt", FilenameUtils.normalize("a/b/../b/c.txt"));
        eq("normalize-dotslash", "a/c", FilenameUtils.normalize("a/./c"));
        eq("normalize-parent-root", null, FilenameUtils.normalize("a/../../c")); // 越过根 → null
        eq("normalize-double-slash", "a/b", FilenameUtils.normalize("a//b"));
        eq("normalize-no-end-sep", "a/b", FilenameUtils.normalizeNoEndSeparator("a/b/"));

        eq("concat", "a/b/c.txt", FilenameUtils.concat("a/b", "c.txt"));
        eq("concat-absolute", "/x/y", FilenameUtils.concat("a/b", "/x/y"));

        eq("extension", "gz", FilenameUtils.getExtension("dir/file.tar.gz")); // 最后一个点之后
        eq("base-name", "file.tar", FilenameUtils.getBaseName("dir/file.tar.gz"));
        eq("name", "file.tar.gz", FilenameUtils.getName("dir/file.tar.gz"));
        eq("prefix-unix", "/", FilenameUtils.getPrefix("/data/file"));
        eq("prefix-win-drive", "C:\\", FilenameUtils.getPrefix("C:\\data\\file"));

        eq("path-unix", "a/b/", FilenameUtils.getPath("a/b/c.txt"));
        eq("path-noend-unix", "a/b", FilenameUtils.getPathNoEndSeparator("a/b/c.txt"));
        eq("full-path-unix", "/a/b/", FilenameUtils.getFullPath("/a/b/c.txt")); // 含结尾分隔符

        eq("separators-to-unix", "a/b/c", FilenameUtils.separatorsToUnix("a\\b/c"));
        eq("separators-to-system", FilenameUtils.normalize("a/b/c"),
                FilenameUtils.separatorsToSystem("a/b/c"));

        eq("directory-contains-rel-false", "false", String.valueOf(
                FilenameUtils.directoryContains("a/b", "a/b/c.txt"))); // 相对路径返回 false（javadoc 规定）

        eq("wildcard-match", "true", String.valueOf(
                FilenameUtils.wildcardMatch("data.txt", "*.txt")));
        eq("wildcard-case-insensitive-default", "true", String.valueOf(
                FilenameUtils.wildcardMatch("DATA.txt", "*.txt"))); // 默认不区分大小写
        eq("wildcard-case-sensitive", "false", String.valueOf(
                FilenameUtils.wildcardMatch("DATA.txt", "*.TXT", IOCase.SENSITIVE)));

        eq("equals-normalized", "true", String.valueOf(FilenameUtils.equals(
                "a/b/../b/c.txt", "a/b/c.txt", true, IOCase.SENSITIVE)));

        eq("is-extension", "true", String.valueOf(
                FilenameUtils.isExtension("a.json", "json")));
        eq("index-of-last-separator", "3", String.valueOf(
                FilenameUtils.indexOfLastSeparator("a/b/c.txt")));
        System.out.println("DONE IoFilenameMain");
    }
}
