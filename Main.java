import java.util.Arrays;

public class Main
{
    public static void main(String[] args)
    {
        int[][] image = new int[][]{{1,1,1},{1,1,0},{1,0,1}};
        int sr=1;
        int sc=1;
        int color=2;
        int[][] newimage = new Solution().floodFill(image, sr, sc, color);
        System.out.println("Before flood fill");
        System.out.println(Arrays.deepToString(image));
        System.out.println("\n------------------------------------------------\n");
        System.out.println("After flood fill");
        System.out.println(Arrays.deepToString(newimage));
        System.out.println("\n================================================\n");
        image = new int[][]{{1,1,1,1,1,1,1,1,1,1},
                            {1,0,0,0,0,0,0,0,0,1},
                            {1,0,1,1,1,1,1,1,1,1},
                            {1,0,0,0,0,0,0,0,0,1},
                            {1,1,1,1,1,1,1,1,0,1},
                            {1,0,0,0,0,0,0,0,0,1},
                            {1,0,1,1,1,1,1,1,1,1},
                            {1,0,0,0,0,0,0,0,0,1},
                            {1,1,1,1,1,1,0,1,1,1},
                            {1,0,0,0,0,0,0,0,0,1}
                        };
        sr=1;
        sc=1;
        color=2;
        newimage = new Solution().floodFill(image, sr, sc, color);
        System.out.println("Before flood fill");
        System.out.println(Arrays.deepToString(image));
        System.out.println("\n------------------------------------------------\n");
        System.out.println("After flood fill");
        System.out.println(Arrays.deepToString(newimage));
        System.out.println("\n================================================\n");
    }
}
