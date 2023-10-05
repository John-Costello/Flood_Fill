#include <iostream>
#include <vector>

using namespace std;

class PixelLocation {
   private:
   int rowLoc;
   int colLoc;
   public:
   PixelLocation(int rowLoc, int colLoc){   this->rowLoc=rowLoc;  this->colLoc=colLoc;   }
   int getRowLoc(){   return rowLoc;   }
   int getColLoc(){   return colLoc;   }
};

class Solution {
public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int color){
        vector<PixelLocation> pixellist;
        int oldColor = image[sr][sc];
        int numRow = image.size();
        int numCol = image[0].size();
        vector<vector<int>> newimage(numRow, vector<int>(numCol));
        vector<vector<bool>> pixelProcessedGrid(numRow, vector<bool>(numCol));
        for(int row=0; row<numRow; row++)
           for(int col=0; col<numCol; col++)
           {
              newimage[row][col]=image[row][col];
              pixelProcessedGrid[row][col]=(image[row][col]!=oldColor)?true:false;
           }
        pixellist.push_back(PixelLocation(sr, sc));
        while(!(pixellist.empty()))        
        {
            sr=pixellist.at(pixellist.size()-1).getRowLoc();
            sc=pixellist.at(pixellist.size()-1).getColLoc();
            pixellist.pop_back();
            newimage[sr][sc]=color; pixelProcessedGrid[sr][sc]=true;
            if(sr+1<numRow && pixelProcessedGrid[sr+1][sc]==false)pixellist.push_back(PixelLocation(sr+1, sc));
            if(sr-1>=0 && pixelProcessedGrid[sr-1][sc]==false)pixellist.push_back(PixelLocation(sr-1, sc));
            if(sc+1<numCol && pixelProcessedGrid[sr][sc+1]==false)pixellist.push_back(PixelLocation(sr, sc+1));
            if(sc-1>=0 && pixelProcessedGrid[sr][sc-1]==false)pixellist.push_back(PixelLocation(sr, sc-1));
        }
        return newimage;
    }
};

int main()
{
    vector<vector<int>> image
    {
        {1,1,1},
        {1,1,0},
        {1,0,1}
    };
    vector<vector<int>> newimage;
    int sr=1;
    int sc=1;
    int color=2;
    Solution solution;
    newimage = solution.floodFill(image, sr, sc, color);
    cout << "Before flood fill"  << endl;
    for(int i=0; i<image.size();i++)
    {
        for(int j=0; j<image[i].size(); j++)
        {
            cout<< image[i][j] << "  ";
        }
        cout <<endl;
    }
    cout << "\n--------------------------------\n" << endl;
    cout << "After flood fill"  << endl;
    for(int i=0; i<newimage.size();i++)
    {
        for(int j=0; j<newimage[i].size(); j++)
        {
            cout<< newimage[i][j] << "  ";
        }
        cout <<endl;
    }
    cout << "\n================================\n" << endl;
    vector<vector<int>> image2
    {
        {1,1,1,1,1,1,1,1,1,1},
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
    cout << "Before flood fill"  << endl;
    for(int i=0; i<image2.size();i++)
    {
        for(int j=0; j<image2[i].size(); j++)
        {
            cout<< image2[i][j] << "  ";
        }
        cout <<endl;
    }
    cout << "\n--------------------------------\n" << endl;
    newimage = solution.floodFill(image2, sr, sc, color);
    cout << "After flood fill"  << endl;
    for(int i=0; i<newimage.size();i++)
    {
        for(int j=0; j<newimage[i].size(); j++)
        {
            cout<< newimage[i][j] << "  ";
        }
        cout <<endl;
    }
    cout << "\n================================\n" << endl;
    
    return(0);
}