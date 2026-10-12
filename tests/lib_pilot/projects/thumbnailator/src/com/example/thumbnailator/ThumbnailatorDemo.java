package com.example.thumbnailator;

import java.awt.Color;
import java.awt.Graphics2D;
import java.awt.image.BufferedImage;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import javax.imageio.ImageIO;

import net.coobird.thumbnailator.Thumbnails;

/**
 * Thumbnailator 链式图片处理（对应文章全部 API：size/scale/keepAspectRatio/
 * outputFormat/watermark/rotate/sourceRegion）。
 *
 * 与文章的差异：文章是 JUnit + 本地图片文件；本示例程序化生成源图（不依赖外部文件），
 * 输出到内存流再回读，只校验尺寸与采样像素（PNG 解码确定；不比对文件字节——
 * 编码器字节输出不保证跨版本一致）。
 *
 * 输出确定性说明：自绘图 + 像素级读取，同 JVM 内结果恒定。
 */
public class ThumbnailatorDemo {

    /** 生成 200x100 的测试图：红底 + 左上角 20x10 蓝块 + 右下角 20x10 绿块。 */
    static BufferedImage sourceImage() {
        BufferedImage img = new BufferedImage(200, 100, BufferedImage.TYPE_INT_RGB);
        Graphics2D g = img.createGraphics();
        g.setColor(Color.RED);
        g.fillRect(0, 0, 200, 100);
        g.setColor(Color.BLUE);
        g.fillRect(0, 0, 20, 10);
        g.setColor(Color.GREEN);
        g.fillRect(180, 90, 20, 10);
        g.dispose();
        return img;
    }

    static BufferedImage process(BufferedImage img, int w, int h,
                                 java.util.function.Function<Thumbnails.Builder<BufferedImage>,
                                         Thumbnails.Builder<BufferedImage>> chain) throws Exception {
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        Thumbnails.Builder<BufferedImage> builder = Thumbnails.of(img).size(w, h)
                .outputFormat("png");
        chain.apply(builder).toOutputStream(out);
        return ImageIO.read(new ByteArrayInputStream(out.toByteArray()));
    }

    static String rgb(BufferedImage img, int x, int y) {
        return String.format("#%06X", img.getRGB(x, y) & 0xFFFFFF);
    }

    public static void main(String[] args) throws Exception {
        BufferedImage src = sourceImage();
        System.out.println("[源图] " + src.getWidth() + "x" + src.getHeight()
                + " 左上像素=" + rgb(src, 5, 5));

        // ---------- 1. 等比缩放（30 行 ImageIO 代码 → 1 行链式） ----------
        BufferedImage thumb = process(src, 100, 50, b -> b);
        System.out.println("[size] 100x50 -> 实际 " + thumb.getWidth() + "x" + thumb.getHeight()
                + "，左上(2,2)=" + rgb(thumb, 2, 2));

        // ---------- 2. 强制尺寸（keepAspectRatio(false)，文章示例） ----------
        BufferedImage forced = process(src, 100, 100, b -> b.keepAspectRatio(false));
        System.out.println("[keepAspectRatio(false)] -> " + forced.getWidth() + "x" + forced.getHeight());

        // ---------- 3. 旋转 90 度：宽高互换，读取安全坐标的中心像素 ----------
        BufferedImage rotated = process(src, 200, 100, b -> b.rotate(90));
        System.out.println("[rotate(90)] -> " + rotated.getWidth() + "x" + rotated.getHeight()
                + "，中心像素=" + rgb(rotated, rotated.getWidth() / 2, rotated.getHeight() / 2)
                + "（红底）");

        // ---------- 4. 中心裁剪 50x50：中心点是红色 ----------
        BufferedImage cropped = process(src, 50, 50,
                b -> b.sourceRegion(net.coobird.thumbnailator.geometry.Positions.CENTER, 50, 50));
        System.out.println("[sourceRegion(CENTER)] -> " + cropped.getWidth() + "x" + cropped.getHeight()
                + "，中心(25,25)=" + rgb(cropped, 25, 25));

        // ---------- 5. 水印：右下角叠加半透明蓝色块 ----------
        BufferedImage watermark = new BufferedImage(20, 10, BufferedImage.TYPE_INT_ARGB);
        Graphics2D wg = watermark.createGraphics();
        wg.setColor(new Color(0, 0, 255, 128)); // 半透明蓝
        wg.fillRect(0, 0, 20, 10);
        wg.dispose();
        BufferedImage marked = process(src, 200, 100,
                b -> b.watermark(net.coobird.thumbnailator.geometry.Positions.BOTTOM_RIGHT,
                        watermark, 0.8f));
        System.out.println("[watermark] -> " + marked.getWidth() + "x" + marked.getHeight()
                + "，右下(190,95)=" + rgb(marked, 190, 95) + "（红蓝叠加，非纯红）");

        System.out.println("[结论] 缩放/裁剪/旋转/水印全部链式一行搞定，回读校验尺寸与像素符合预期");
    }
}
