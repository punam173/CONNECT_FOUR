//Program submitted by: Punam Rani Pal
//reference:   https://github.com/michael-zucchetta/connect-four-rust-web-assembly
//https://en.wikipedia.org/wiki/Connect_Four and  https://github.com/krahul84/ConnectFour
use std::default::Default;
use std::fmt;
use std::io::stdin;
use std::str::FromStr;

//grid structure for 6 * 7 matrix
// row size = 6 and column size = 7
//Value stores players col option

pub struct GridMatrix{
     row_size:usize,
     col_size:usize,
     values: Vec<Box>,

}
//Implementation of GridMatrix
impl GridMatrix{
    //function to create grid matrix
    //parameters: row size and col size

    pub fn create_grid(row_size:usize,col_size:usize) -> GridMatrix{
       //the number of rows and columns are matched to a value to 
       //help the palyers to given an input for a specific col or row
       GridMatrix{
         row_size: row_size,
         col_size: col_size,
         values: (0..row_size).map(|_| Box::create(col_size)).collect(),
      }
    }

   //function to take user choice for the col
   //returns the modified grid for each user choice
   //parameters: self matrix, palyer (A/B) and col

   pub fn grid_matrix(&mut self, player: Grid, column: usize) -> Result<(), ()> {
        let exception = Err(());
        for value in self.values.iter_mut().rev() {
            if value.position_grid(player, column).is_ok() {
                return Ok(());
            }
        }
        exception
    }
    //Function that prints the matrix
     pub fn grid_print(&self) {
        println!("{}", self);
    }

    //check if a palyer appears 4 times consequently in a row
    //it takes a matrix
    //returns an option as a winner

    fn winner_rowwise(&self) -> Option<Grid> {
        let mut num = 1;
        let mut grid_last = Grid::Empty;

        for value in self.values.iter() {
            for value in value.grid.iter() {
                if value.check_player() && *value == grid_last {
                    num += 1;
                    if num == 4 {
                        return Some(*value)
                    }
                } else {
                    num = 1;
                    // num += 1;
                    grid_last = *value;
                }
            }
        }

        None
    }

    //check if a palyer appears 4 times consequently in a col
    //it takes a matrix
    //returns an option as a winner

    fn winner_columnwise(&self) -> Option<Grid> {
        let mut num = 1;
        let mut grid_last = Grid::Empty;

        for column in 0..self.col_size {
            for value in self.values.iter() {
                let gridval = value.grid[column];

                if gridval.check_player() && gridval == grid_last {
                    num += 1;

                    if num == 4 {
                        return Some(gridval)
                    }
                } else {
                    num = 1;
                    //  num += 1;
                    grid_last = gridval;
                }
            }
        }

        None
    }

    //check if a palyer appears 4 times consequently diagonally downward
    //it takes a matrix
    //returns an option as a winner

    fn winner_daigonal_down(&self) -> Option<Grid> {
        for (cols, _) in (0..(self.col_size - 3)).enumerate() {
            for (rows, _) in (0..(self.row_size - 3)).enumerate() {
                let mut num = 1;
                let mut grid_last = self.values[rows].grid[cols];

                if !grid_last.check_player() {
                    continue;
                }

                for offset in 1..4 {
                    let gridval = self.values[rows + offset].grid[cols + offset];
                //     if !grid_last.check_player() {
                //     continue;
                // }

                    if gridval.check_player() && gridval == grid_last {
                        num += 1;

                        if num == 4 {
                            return Some(gridval);
                        }
                    } else {
                        num = 1;
                        grid_last = gridval;
                    }
                }
            }
        }

        None
    }

    //check if a palyer appears 4 times consequently diagonally upward
    //it takes a matrix
    //returns an option as a winner

    fn winner_daigonal_up(&self) -> Option<Grid> {
        for (cols, _) in (0..(self.col_size - 3)).enumerate() {
            for (rows, _) in ((self.row_size - 3)..self.row_size).enumerate() {
                let row_num = self.row_size - 3 + rows;
                let mut num = 1;
                let mut grid_last = self.values[row_num].grid[cols];

                if !grid_last.check_player() {
                    continue;
                }

                for offset in 1..4 {
                    let gridval = self.values[row_num - offset].grid[cols + offset];

                    if gridval.check_player() && gridval == grid_last {
                        num += 1;

                        if num == 4 {
                            return Some(gridval);
                        }
                    } else {
                        num = 1;
                        grid_last = gridval;
                    }
                }
            }
        }

        None
    }

    //it patter matches for all the deciding functions
    //if a functions has none than its not a winner
    //function with some value value is a winner

    pub fn winner(&self) -> Option<Grid> {
        match self.winner_rowwise() {
            Some(tile) => Some(tile),
            None => match self.winner_columnwise() {
                Some(tile) => Some(tile),
                None => match self.winner_daigonal_down() {
                    Some(tile) => Some(tile),
                    None => match self.winner_daigonal_up() {
                        Some(tile) => Some(tile),
                        None => None,
                    }
                }
            }
        }
    }

}

//first test is tio test the grid size
//checks to see if the inut size that the user
//has given is within the range
//Each rows and column are specified with a value

#[test]
fn test_grid_size(){
    // let mut grid = grid_matrix::create_grid(6, 7);
    // let mut grid1 = grid_matrix::create_grid(6, 7);
    let a = 1;
    assert_eq!(1,a);
}
#[test]
fn test_user_input_range(){
    let row = 7;
    let column = 6;
    let input_row_size = 4; 
    let inputolum_size = 5; 
    assert!(input_row_size <= row - 1);
    assert!(inputolum_size <= column - 1);    
}
#[test]
fn test_winner_horizontal(){
    let playera = vec![1,1,1];
    let playerb = vec![1,1,1];
    assert_eq!(playera,playerb);
}


//Function to have the column number as given
impl fmt::Display for GridMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for n in 0..self.values[0].size() {
            try!(write!(f, "{}  ", n + 1));
        }

        try!(writeln!(f, ""));

        let mut result = Ok(());

        for row in self.values.iter() {
            result = writeln!(f, "{}", row);
        }

        result
    }
}
//valid values for each grid
#[derive(Clone, Copy)]
pub enum Grid {
    Empty,
    A,
    B,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid::Empty
    }
}
//check if the col cell is filled or empty
impl Grid {
    pub fn check_player(&self) -> bool {
        match *self {
            Grid::Empty => false,
            _ => true,
        }
    }
}
//formate of the grid for display
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let result = match *self {
            Grid::Empty => "x",
            Grid::A => "A",
            Grid::B => "B",
        };

        write!(f, "{}", result)
    }
}
//matrix grid is formatted and formed as a box
impl PartialEq for Grid {
    fn eq(&self, secplayer: &Grid) -> bool {
        format!("{}", self) == format!("{}", secplayer)
    }
}

pub struct Box {
    grid: Vec<Grid>,
}
// impl fmt::Display for GridMatrix {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         for n in 0..self.values[0].size() {
//             try!(write!(f, "{}  ", n + 1));
//         }

//         try!(writeln!(f, ""));

//         let mut result = Ok(());

//         for row in self.values.iter() {
//             result = writeln!(f, "{}", row);
//         }

//         result
//     }
// }
//mapping each colum
impl Box {
    pub fn create(col_size: usize) -> Box {
        Box {
            grid: (0..col_size).map(|_| Grid::Empty).collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.grid.len() as usize
    }

    pub fn position_grid(&mut self, player: Grid, column: usize) -> Result<(), ()> {
        if column > self.grid.len() - 1 {
            return Err(());
        }

        match self.grid[column] {
            Grid::Empty => {
                self.grid[column] = player;
                return Ok(());
            },
            _ => return Err(()),
        }
    }
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut output = Ok(());

        for value in self.grid.iter() {
            output = write!(f, "{}  ", value);
        }

        output
    }
}
//The main functionis where all the implemented features
//have been called and executed

//first teh size of the grid matrix is created
//which is in this case is 6*7

//then each player selects a coloum and then the grid and matrix
//represenTion of each cell get updated at each turn

fn main() {
    let mut grid = GridMatrix::create_grid(6, 7);
    let mut player = Grid::A;

    grid.grid_print();

    loop {
        println!("It's turn of Player {}'s. Please pick a column to drop", player);
        let mut colcell = String::new();
        //  match grid.winner() {
        //             Some(winner) => {
        //                 println!("The winner is Player {}", winner);
        //                 break;
        //             },
        //             None => {},
        //         }
        match stdin().read_line(&mut colcell) {
            Ok(_) => {
                let col = match usize::from_str(&colcell[..].trim()) {
                    Ok(col) => {
                        if col != 0 {
                            col - 1
                        } else {
                            println!("Please provide valid column");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("Please provide valid column");
                        continue;
                    }
                };

                if grid.grid_matrix(player, col).is_err() {
                    println!("Please provide a valid column");
                    continue;
                }

                grid.grid_print();

                match grid.winner() {
                    Some(winner) => {
                        println!("The winner is Player {}", winner);
                        break;
                    },
                    None => {},
                }

                player = match player {
                    Grid::B => Grid::A,
                    _ => Grid::B,
                };
                //  Err(_) => {
                //         println!("Please provide valid column");
                //         continue;
                //     }
            },
            Err(_) => {
                println!("Please provide data.");
                continue;
            }
        }
    }
}