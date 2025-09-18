use std::convert::TryInto;

fn main() {
    println!("Flood fill program!\n");
    let image: Vec<Vec<u32>> = vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]];
    let sr: u32 = 1;
    let sc: u32 = 1;
    let color: u32 = 2;
    let newimage = Solution::flood_fill(image.clone(), sr, sc, color);
    println!("Before flood fill");
    Solution::display_image(&image);
    println!("After flood fill");
    Solution::display_image(&newimage);
    println!("\n================================================\n");
    let image: Vec<Vec<u32>> = vec![[1,1,1,1,1,1,1,1,1,1].to_vec(),
                            [1,0,0,0,0,0,0,0,0,1].to_vec(),
                            [1,0,1,1,1,1,1,1,1,1].to_vec(),
                            [1,0,0,0,0,0,0,0,0,1].to_vec(),
                            [1,1,1,1,1,1,1,1,0,1].to_vec(),
                            [1,0,0,0,0,0,0,0,0,1].to_vec(),
                            [1,0,1,1,1,1,1,1,1,1].to_vec(),
                            [1,0,0,0,0,0,0,0,0,1].to_vec(),
                            [1,1,1,1,1,1,0,1,1,1].to_vec(),
                            [1,0,0,0,0,0,0,0,0,1].to_vec()
                        ];
    let sr: u32 = 1;
    let sc: u32 = 4;
    let color: u32 = 2;
    let newimage = Solution::flood_fill(image.clone(), sr, sc, color);
    println!("Before flood fill");
    Solution::display_image(&image);
    println!("After flood fill");
    Solution::display_image(&newimage);
}

struct PixelLocation{   row_loc:u32, col_loc:u32   }
impl PixelLocation{
   fn get_row_loc(&self)->u32 {   return self.row_loc;   }
   fn get_col_loc(&self)->u32 {   return self.col_loc;   }
}

struct Solution;

impl Solution{
    pub fn display_image(image: &Vec<Vec<u32>>){
        print!("[");
        for (i,row) in image.iter().enumerate(){
            print!("[");
            for (j,element) in row.iter().enumerate(){
                print!("{}", element);
                if j < row.len()-1 {   print!(", ");   }
            }
            if i<image.len()-1 {   print!("],\n ");   }
            else if i==image.len()-1{   print!("]");   }
        }
        print!("]\n");
    }
}


impl Solution {
    pub fn flood_fill(image: Vec<Vec<u32>>, sr: u32, sc: u32, color: u32) -> Vec<Vec<u32>>{
       let mut pixel_list: Vec<PixelLocation> = Vec::new();
       let old_color:u32 = image[sr as usize][sc as usize];
       let num_row: u32 = image.len().try_into().unwrap();
       let num_col: u32 = image[0].len().try_into().unwrap();
       let mut newimage: Vec<Vec<u32>> = Vec::new();
       let mut pixel_processed_grid :Vec<Vec<bool>> = Vec::new();
       for _n in 0..num_row{
          newimage.push(Vec::new());
          pixel_processed_grid.push(Vec::new());
       }
       for row in &mut newimage{
           for _m in 0..num_col{
               row.push(0);
           }
       }
       for row in &mut pixel_processed_grid{
           for _m in 0..num_col{
               row.push(false);
           }
       }
       for(i,row) in image.iter().enumerate(){
           for(j,_element) in row.iter().enumerate(){
               newimage[i][j]=image[i][j];
               if image[i][j]!=old_color {   pixel_processed_grid[i][j]=true;   }
               else{   pixel_processed_grid[i][j]=false;   }
           }
       }
       pixel_list.push(PixelLocation{row_loc:sr, col_loc:sc});
       while(!(pixel_list.is_empty()))
       {
          let process_pixel:PixelLocation=pixel_list.remove(pixel_list.len()-1);
          let sr:u32 = process_pixel.get_row_loc();
          let sc:u32 = process_pixel.get_col_loc();
          newimage[sr as usize][sc as usize]=color;
          pixel_processed_grid[sr as usize][sc as usize]=true;
          if(sr+1<num_row && pixel_processed_grid[(sr+1) as usize][sc as usize]==false)
          {   pixel_list.push(PixelLocation{   row_loc:sr+1,   col_loc:sc,   });   }
          if(sr>=1 && pixel_processed_grid[(sr-1) as usize][sc as usize]==false)
          {   pixel_list.push(PixelLocation{   row_loc:sr-1,   col_loc:sc,   });   }
          if(sc+1<num_col && pixel_processed_grid[sr as usize][(sc+1) as usize]==false)
          {   pixel_list.push(PixelLocation{   row_loc:sr,   col_loc:sc+1,   });   }
          if(sc>=1 && pixel_processed_grid[sr as usize][(sc-1) as usize]==false)
          {   pixel_list.push(PixelLocation{   row_loc:sr,   col_loc:sc-1,   });   }
       }
       return newimage;
    }
}
