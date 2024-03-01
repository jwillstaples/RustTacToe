struct GameState {

    board: [[[bool; 2]; 3]; 3],
    mover: bool,  // false for 0 row and true for 1 row

}

impl GameState { 

    fn winning_position(&self) -> bool { 
        // returns true if position is won

        let mover_index = if self.mover {1} else {0};
        
        for i in vec![0, 1, 2] {
            // check horizontal
            if self.board[0][i][mover_index] && self.board[1][i][mover_index] && self.board[2][i][mover_index] {
                return true;
            };
            // check vertical
            if self.board[i][0][mover_index] && self.board[i][1][mover_index] && self.board[i][2][mover_index] {
                return true;
            }
        }

        // check diagonal 
        if self.board[0][0][mover_index] && self.board[1][1][mover_index] && self.board[2][2][mover_index] { 
            return true; 
        }
        if self.board[0][2][mover_index] && self.board[1][1][mover_index] && self.board[2][0][mover_index] { 
            return true;
        }

        return false;
    }

    fn print_board(&self) {
        println!("------");
        for row in vec![0, 1, 2]{
            for col in vec![0, 1, 2]{
                print!("|");
                if self.board[row][col][0]{ print!("x")}
                else if self.board[row][col][1]{ print!("o")}
                else {print!(" ")}
            }
            println!("|");
            println!("------");
        }
    }

    fn move_from_int(&self, mv: usize) -> GameState {
        let row = mv / 3;
        let col = mv % 3; 
        let mut new_board = self.board.clone();

        if self.mover {
            new_board[row][col][1] = true;
        }
        else { 
            new_board[row][col][0] = true; 
        };
        let new_game_state = GameState {
            board: new_board,
            mover: ! self.mover,
        };
        return new_game_state; 
    }

}

fn starting_state() -> GameState { 

    let game_state = GameState { 
        board: [[[false; 2]; 3]; 3],
        mover: true,
    };
    return game_state
}

fn main(){ 

    let game_state = starting_state(); 
    game_state.print_board(); 

}