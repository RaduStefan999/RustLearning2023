use std::{io, fs, time, thread};

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
    fn play(&mut self, slow_down: u64)
    {
        loop 
        {
            self.compute();
            thread::sleep(time::Duration::from_millis(slow_down));
            self.display();
        }
    }

    fn compute(&mut self)
    {
        let mut new_state = LifeGame::make(self.board.len());
        let direction_ox: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
        let direction_oy: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];

        for it in 0..self.board.len()
        {
            for jt in 0..self.board[0].len()
            {
                let mut nr_of_alive_neighbours = 0;

                for dir_idx in 0..direction_ox.len()
                {
                    let next_poz_line = (it as i32) + direction_oy[dir_idx];
                    let next_poz_column = (jt as i32) + direction_ox[dir_idx];

                    if next_poz_line < 0 || next_poz_line >= (self.board.len() as i32)
                    {
                        continue;
                    }
                    if next_poz_column < 0 || next_poz_column >= (self.board.len() as i32)
                    {
                        continue;
                    }

                    if self.board[next_poz_line as usize][next_poz_column as usize] == true
                    {
                        nr_of_alive_neighbours += 1;
                    }

                }

                if self.board[it][jt] == false
                {
                    if nr_of_alive_neighbours == 3
                    {
                        new_state.board[it][jt] = true;
                    }
                }
                else 
                {
                    if nr_of_alive_neighbours == 2 || nr_of_alive_neighbours == 3
                    {
                        new_state.board[it][jt] = true;
                    }    
                }
            }
        }

        self.board = new_state.board;
    }

    fn display(&self)
    {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
        
        for board_line_idx in 0..self.board.len()
        {
            for board_column_idx in 0..self.board[0].len()
            {
                if self.board[board_line_idx][board_column_idx]
                {
                    print!("x");
                }
                else
                {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}


pub fn prob3_start()
{
    let mut life_game_board = LifeGame::make(20);

    //let life_game_board_path = r"D:\personal\RustLabs\RustLearning2023\lab3\res\prob3_file_life_game_1.txt";
    let life_game_board_path = r"D:\personal\RustLabs\RustLearning2023\lab3\res\prob3_file_life_game_2.txt";

    if let Err(error) = life_game_board.load(life_game_board_path)
    {
        println!("Could not load life game board with error {}", error);
    }

    life_game_board.play(300);
}