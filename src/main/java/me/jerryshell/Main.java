package me.jerryshell;

import javax.imageio.ImageIO;
import java.awt.image.BufferedImage;
import java.io.FileInputStream;
import java.io.IOException;

public class Main {
    public static void main(String[] args) throws IOException {
        if (args.length != 2) {
            System.out.println("java -jar similar-image.jar img1.jpg img2.jpg");
            return;
        }

        String image1Path = args[0];
        BufferedImage image1 = ImageIO.read(new FileInputStream(image1Path));
        int[] hash1 = Util.hashImage(image1);

        String image2Path = args[1];
        BufferedImage image2 = ImageIO.read(new FileInputStream(image2Path));
        int[] hash2 = Util.hashImage(image2);

        int hamming = Util.hamming(hash1, hash2);
        double result = (1 - hamming / 64.0) * 100;
        System.out.println(result);
    }
}
