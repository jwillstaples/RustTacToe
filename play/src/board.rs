use std::io; 

struct GameState {

    board: [[[bool; 2]; 3]; 3],
    mover: bool,  // false for 0 row and true for 1 row

}

impl GameState { 

    fn losing_position(&self) -> bool { 
        // returns true if position is won

        let mover_index = if self.mover {0} else {1};
        
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

    fn full_board(&self) -> bool { 
        return ! self.legal_moves().iter().any(|&x|x)
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

    fn legal_moves(&self) -> [bool; 9] { 

        let mut legals = [true; 9];

        for mv in 0..9 { 
            if self.board[mv/3][mv%3][0] || self.board[mv/3][mv%3][1] { 
                legals[mv] = false;
            }
        }
        return legals;
    }

    fn minimax(&self) -> (usize, i8) { 

        if self.losing_position() { 
            return (0, -1); 
        } else if ! self.legal_moves().iter().any(|&x|x) { 
            return (0, 0);
        }

        let mut child_scores = [0; 9];
        let legals = self.legal_moves();

        for mv in 0..9 {  
            if legals[mv] { 
                let (_, score) = self.move_from_int(mv).minimax();
                child_scores[mv] = -score; 
            } else { 
                child_scores[mv] = -2; 
            }
        }
        return child_scores.into_iter().enumerate().max_by_key(|(_, num)| *num).unwrap()
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

    let mut game_state = starting_state(); 

    loop { 

        game_state.print_board();

        let mut mv = 0; 
        let mut eval = 0; 

        if game_state.mover {
            (mv, eval) = game_state.minimax(); 
        } else {
            println!("Accepting move: ");
            let mut input = String::new(); 
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            mv = input.parse().unwrap(); 
        }

        game_state = game_state.move_from_int(mv);

        if game_state.losing_position() || game_state.full_board(){ 
            game_state.print_board();
            break
        }

    }
}