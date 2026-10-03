import java.awt.geom.AffineTransform;
import java.awt.geom.Point2D;
import java.util.Locale;

/**
 * java.awt.geom：AffineTransform 与 Point2D 变换（jmod 覆盖计划第 4 步；
 * 纯计算几何，不触碰 AWT Toolkit/native——闭包拉进 Toolkit 即为精度缺陷）。
 */
public class TestGeomAffineTransform {

    static String fmt(Point2D p) {
        return String.format(Locale.ROOT, "%.3f", p.getX()) + ","
                + String.format(Locale.ROOT, "%.3f", p.getY());
    }

    public static void main(String[] args) throws Exception {
        AffineTransform t = new AffineTransform(2, 0, 0, 3, 10, 20);
        Point2D dst = t.transform(new Point2D.Double(1, 2), null);
        System.out.println("scale-translate=" + fmt(dst));

        Point2D back = t.createInverse().transform(dst, null);
        System.out.println("inverse=" + fmt(back));

        AffineTransform rot = AffineTransform.getRotateInstance(Math.PI / 2);
        System.out.println("rot90=" + fmt(rot.transform(new Point2D.Double(1, 0), null)));
        System.out.println("rot180=" + fmt(AffineTransform.getQuadrantRotateInstance(2)
                .transform(new Point2D.Double(1, 0), null)));

        AffineTransform composed = new AffineTransform();
        composed.translate(10, 20);
        composed.scale(2, 3);
        Point2D chained = composed.transform(new Point2D.Double(1, 2), null);
        System.out.println("compose=" + fmt(chained) + " same-as-single="
                + t.transform(new Point2D.Double(1, 2), null).equals(chained));

        System.out.println("identity=" + (new AffineTransform().getType() == AffineTransform.TYPE_IDENTITY));
        System.out.println("type-after-scale=" + ((AffineTransform.getScaleInstance(2, 3).getType()
                & AffineTransform.TYPE_UNIFORM_SCALE) != 0));
    }
}
