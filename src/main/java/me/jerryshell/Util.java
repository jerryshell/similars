package me.jerryshell;

import java.awt.*;
import java.awt.image.BufferedImage;
import java.awt.image.Raster;
import java.util.Arrays;

public class Util {
    // 缩放图片
    public static BufferedImage resizeImage(BufferedImage originalImage, int targetWidth, int targetHeight, int imageType) {
        BufferedImage resizeImage = new BufferedImage(targetWidth, targetHeight, imageType);
        Graphics2D g2d = resizeImage.createGraphics();
        g2d.drawImage(originalImage, 0, 0, targetWidth, targetHeight, null);
        g2d.dispose();
        return resizeImage;
    }

    // 获得图片 hash
    public static int[] hashImage(BufferedImage image) {
        // 缩放图片，并将图片改成灰度图
        int targetWidth = 8;
        int targetHeight = 8;
        BufferedImage resizeImage = resizeImage(image, targetWidth, targetHeight, BufferedImage.TYPE_BYTE_GRAY);

        // 取出所有像素
        int[] pixelArray = new int[targetWidth * targetHeight];
        Raster resizeImageData = resizeImage.getData();
        resizeImageData.getPixels(0, 0, targetWidth, targetHeight, pixelArray);

        // 计算灰度平均值
        int graySum = Arrays.stream(pixelArray).sum();
        int grayAvg = graySum / (targetWidth * targetHeight);

        // 根据灰度值平均值计算 hash
        return Arrays.stream(pixelArray).map(i -> i < grayAvg ? 0 : 1).toArray();
    }

    // 汉明距离
    public static int hamming(int[] hash1, int[] hash2) {
        int result = 0;
        for (int i = 0; i < hash1.length; i++) {
            if (hash1[i] != hash2[i]) {
                result += 1;
            }
        }
        return result;
    }
}
