import java.lang.module.Configuration;
import java.lang.module.ModuleDescriptor;
import java.lang.module.ModuleFinder;
import java.lang.module.ModuleReader;
import java.lang.module.ModuleReference;
import java.net.URI;
import java.util.List;
import java.util.Optional;
import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * 运行期定义模块层：Module 构造经 defineModule0 向 VM 登记，Controller 的 addReads / addExports /
 * addOpens 经 addReads0 / addExports0 同步 VM 侧可读性与导出表。模块内不含类（只登记包名）。
 */
public class TestModuleLayerDefine {
    static ModuleReference ref(ModuleDescriptor d) {
        return new ModuleReference(d, URI.create("mem:/" + d.name())) {
            @Override
            public ModuleReader open() {
                throw new UnsupportedOperationException("no content");
            }
        };
    }

    static ModuleFinder finder(ModuleDescriptor... ds) {
        return new ModuleFinder() {
            @Override
            public Optional<ModuleReference> find(String name) {
                for (ModuleDescriptor d : ds) {
                    if (d.name().equals(name)) return Optional.of(ref(d));
                }
                return Optional.empty();
            }

            @Override
            public Set<ModuleReference> findAll() {
                return Stream.of(ds).map(TestModuleLayerDefine::ref).collect(Collectors.toSet());
            }
        };
    }

    public static void main(String[] args) {
        ModuleDescriptor a = ModuleDescriptor.newModule("demo.a")
                .packages(Set.of("demo.a.api", "demo.a.impl"))
                .exports("demo.a.api")
                .build();
        ModuleDescriptor b = ModuleDescriptor.newModule("demo.b")
                .packages(Set.of("demo.b"))
                .requires("demo.a")
                .build();
        ModuleDescriptor c = ModuleDescriptor.newOpenModule("demo.c")
                .packages(Set.of("demo.c"))
                .build();

        Configuration cf = ModuleLayer.boot().configuration()
                .resolve(finder(a, b, c), ModuleFinder.of(), Set.of("demo.b", "demo.c"));
        ModuleLayer.Controller ctl = ModuleLayer.defineModulesWithOneLoader(
                cf, List.of(ModuleLayer.boot()), ClassLoader.getSystemClassLoader());
        ModuleLayer layer = ctl.layer();

        Module ma = layer.findModule("demo.a").orElseThrow();
        Module mb = layer.findModule("demo.b").orElseThrow();
        Module mc = layer.findModule("demo.c").orElseThrow();
        System.out.println("modules = " + new TreeSet<>(layer.modules().stream().map(Module::getName).toList()));
        System.out.println("same loader = " + (ma.getClassLoader() == mb.getClassLoader()));
        System.out.println("a packages = " + new TreeSet<>(ma.getPackages()));
        System.out.println("b reads a = " + mb.canRead(ma) + ", a reads b = " + ma.canRead(mb));
        System.out.println("a exports api to b = " + ma.isExported("demo.a.api", mb));
        System.out.println("a exports impl to b = " + ma.isExported("demo.a.impl", mb));
        System.out.println("c open = " + mc.getDescriptor().isOpen() + ", c opens to b = " + mc.isOpen("demo.c", mb));

        ctl.addReads(ma, mb);
        System.out.println("after addReads: a reads b = " + ma.canRead(mb));
        ctl.addExports(ma, "demo.a.impl", mb);
        System.out.println("after addExports: impl to b = " + ma.isExported("demo.a.impl", mb)
                + ", impl to c = " + ma.isExported("demo.a.impl", mc));
        ctl.addOpens(ma, "demo.a.api", mc);
        System.out.println("after addOpens: api open to c = " + ma.isOpen("demo.a.api", mc)
                + ", api open to b = " + ma.isOpen("demo.a.api", mb));
        try {
            ctl.addExports(ma, "demo.zzz", mb);
        } catch (IllegalArgumentException e) {
            System.out.println("foreign package: " + e.getMessage());
        }
        try {
            ctl.addReads(ma, null);
        } catch (NullPointerException e) {
            System.out.println("null target: NPE");
        }
        System.out.println("layer parents = " + layer.parents().size() + ", boot has a = "
                + ModuleLayer.boot().findModule("demo.a").isPresent());
    }
}
