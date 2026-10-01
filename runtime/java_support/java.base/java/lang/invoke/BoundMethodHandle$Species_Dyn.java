/*
 * VM 支持类（rava）：非预生成 key 的 BoundMethodHandle 物种通用载体。
 *
 * JDK 的 ClassSpecializer 对 jlink 未预生成的物种 key 用 ASM 现场生成专用类
 * （字段 arg<T><i> + make 工厂 + copyWith*）。原生二进制不能在运行期定义类，
 * 全部动态 key 共用本类作为 speciesCode：绑定参数按序存于 args（基本类型装箱），
 * 物种身份存于实例字段 sd。make 工厂 / arg getter 的按 key 形态由 VM 按描述符 /
 * 字段名路由到本类（runtime/java_runtime/src/species_dyn.rs）。
 * 方案：docs/plans/2026-09-27-bmh-dynamic-species.md
 *
 * 编译：转译时以当前 JDK 的 javac --patch-module java.base 编入 java.lang.invoke 包，
 * 经常规字节码翻译进入类宇宙（generator/crates/resolve/src/image.rs VM 支持类目录）。
 */
package java.lang.invoke;

import java.lang.invoke.LambdaForm.BasicType;
import java.util.Arrays;

final class BoundMethodHandle$Species_Dyn extends BoundMethodHandle {
    /** linkCodeToSpeciesData 的静态写入落点（全部动态 key 共用；实例物种见 sd）。 */
    static BoundMethodHandle.SpeciesData BMH_SPECIES;

    final BoundMethodHandle.SpeciesData sd;
    final Object[] args;

    private BoundMethodHandle$Species_Dyn(MethodType mt, LambdaForm lf,
                                          BoundMethodHandle.SpeciesData sd, Object[] args) {
        super(mt, lf);
        this.sd = sd;
        this.args = args;
    }

    /** 规范工厂：按 key 形态的 make(MethodType, LambdaForm, T0..Tn-1) 由 VM 转调至此。 */
    static BoundMethodHandle make(MethodType mt, LambdaForm lf,
                                  BoundMethodHandle.SpeciesData sd, Object[] args) {
        return new BoundMethodHandle$Species_Dyn(mt, lf, sd, args);
    }

    @Override
    final BoundMethodHandle.SpeciesData speciesData() {
        return sd;
    }

    @Override
    final BoundMethodHandle copyWith(MethodType mt, LambdaForm lf) {
        return make(mt, lf, sd, args);
    }

    private BoundMethodHandle extend(MethodType mt, LambdaForm lf, BasicType t, Object narg) {
        Object[] nargs = Arrays.copyOf(args, args.length + 1);
        nargs[args.length] = narg;
        return make(mt, lf, sd.extendWith(t), nargs);
    }

    @Override
    final BoundMethodHandle copyWithExtendL(MethodType mt, LambdaForm lf, Object narg) {
        return extend(mt, lf, BasicType.L_TYPE, narg);
    }

    @Override
    final BoundMethodHandle copyWithExtendI(MethodType mt, LambdaForm lf, int narg) {
        return extend(mt, lf, BasicType.I_TYPE, Integer.valueOf(narg));
    }

    @Override
    final BoundMethodHandle copyWithExtendJ(MethodType mt, LambdaForm lf, long narg) {
        return extend(mt, lf, BasicType.J_TYPE, Long.valueOf(narg));
    }

    @Override
    final BoundMethodHandle copyWithExtendF(MethodType mt, LambdaForm lf, float narg) {
        return extend(mt, lf, BasicType.F_TYPE, Float.valueOf(narg));
    }

    @Override
    final BoundMethodHandle copyWithExtendD(MethodType mt, LambdaForm lf, double narg) {
        return extend(mt, lf, BasicType.D_TYPE, Double.valueOf(narg));
    }
}
