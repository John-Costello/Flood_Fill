import java.util.List;
import java.util.ArrayList;

class Solution
{
    public int[][] floodFill(int[][] image, int sr, int sc, int color)
    {
        class PixelLocation
        {
            private int rowLoc;
            private int colLoc;
            public PixelLocation(int rowLoc, int colLoc)
            {
                this.rowLoc = rowLoc;
                this.colLoc = colLoc;
            }
            public int getRowLoc(){return rowLoc;}
            public int getColLoc(){return colLoc;}
        }
        List<PixelLocation> pixellist = new ArrayList<>();
        PixelLocation processPixel;
        int oldColor=image[sr][sc];
        int numRow=image.length;
        int numCol=image[0].length;
        int[][] newimage = new int[numRow][numCol];
        boolean[][] pixelProcessedGrid = new boolean[numRow][numCol];
        for(int row=0; row<numRow; row++)
            for(int col=0; col<numCol; col++)
            {
                newimage[row][col]=image[row][col];
                pixelProcessedGrid[row][col]=(image[row][col]!=oldColor)?true:false;
            }
              
        pixellist.add(new PixelLocation(sr,sc));
        while(!(pixellist.isEmpty()))
        {
            processPixel=pixellist.remove(pixellist.size()-1);
            sr=processPixel.getRowLoc();
            sc=processPixel.getColLoc();
            newimage[sr][sc]=color; pixelProcessedGrid[sr][sc]=true;
            if(sr+1<numRow && pixelProcessedGrid[sr+1][sc]==false)pixellist.add(new PixelLocation(sr+1,sc));
            if(sr-1>=0 && pixelProcessedGrid[sr-1][sc]==false)pixellist.add(new PixelLocation(sr-1,sc));
            if(sc+1<numCol && pixelProcessedGrid[sr][sc+1]==false)pixellist.add(new PixelLocation(sr,sc+1));
            if(sc-1>=0 && pixelProcessedGrid[sr][sc-1]==false)pixellist.add(new PixelLocation(sr,sc-1));
        }
        return newimage;
    }
}
