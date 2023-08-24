use std::{io, fs};

struct LifeGame
{
    board: Vec<Vec<bool>>
}

impl LifeGame {
    fn make(size: usize) -> LifeGame
    {
        return LifeGame{board: vec![vec![false; size]; size]};
    }

    fn load(&mut self, file_path: &str) -> Result<(), io::Error>
    {
        let board_state_string = fs::read_to_string(file_path)?;

        for (line_idx, line_content) in board_state_string.lines().enumerate()
        {
            for (gliph_idx, gliph) in line_content.chars().enumerate()
            {
                if gliph != ' '
                {
                    self.board[line_idx][gliph_idx] = true;
                }
            }
        }

        return Ok(());
    }
}

impl LifeGame {
    fn play(self, slow_down: u32)
    {

    }

    fn compute(&mut self)
    {
        let new_state = LifeGame::make(self.board.len());
        let direction_ox: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
        let direction_oy: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];

        for it in 0..self.board.len()
        {
            for jt in 0..self.board[0].len()
            {
                for dir_idx in 0..direction_ox.len()
                {
                    let next_poz_line = std::cmp::min(self.board.len() - 1, std::cmp::max(0, it + direction_oy[dir_idx])); 
                    let next_poz_column = std::cmp::min(self.board.len() - 1, std::cmp::max(0, jt + direction_ox[dir_idx])); 
                
                    
                }
            }
        }
    }

    fn display(self)
    {

    }
}


pub fn prob3_start()
{
    let mut life_game_board = LifeGame::make(10);

    let life_game_board_path = r"D:\personal\RustLabs\RustLearning2023\lab3\res\prob3_file_life_game_1.txt";
    //let life_game_board_path = r"D:\personal\RustLabs\RustLearning2023\lab3\res\prob3_file_life_game_2.txt";

    if let Err(error) = life_game_board.load(life_game_board_path)
    {
        println!("Could not load life game board with error {}", error);
    }

    life_game_board.play(10);
}