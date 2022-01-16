use std::io;

#[derive(PartialEq)]
enum BoardState{
    EMPTY,
    O,
    X
}

struct Board{
    board: Vec<BoardState>
}


impl Board{
    fn init(&mut self) {
        for i in 0..9 {
            self.board[i] = BoardState::EMPTY;
        }
    }


    fn set_board_state(&mut self, index:usize, state:&BoardState){
        // need to do this because of token is not a reference it it borrowed
        // and the variable which is used to call this function can not be used later in the program
        self.board[index] = match state{
            BoardState::EMPTY => BoardState::EMPTY,
            BoardState::O => BoardState::O,
            BoardState::X => BoardState::X
        }
    }

    fn is_empty_cell(&self, index:usize) -> bool {
        match self.board[index] {
            BoardState::EMPTY => true,
            _ => false
        }
    }

    fn is_all_occupied(&self) -> bool{
        for elem in self.board.iter(){
            match elem{
                BoardState::EMPTY => {
                    return false;
                },
                _ => {

                }
            }
        }

        return true;
    }

    // check if the game is over or not 
    fn is_game_over(&self) -> bool {
        // horizzontal check
        for i in 0..3 {
            if self.board[3 * i] == self.board[3 * i + 1] && self.board[3 * i] == self.board[3 * i + 2] && self.board[3 * i] != BoardState::EMPTY{
                return true;
            }
        }

        // vertical check
        for i in 0..3 {
            if self.board[i] == self.board[i + 3] && self.board[i] == self.board[i + 6] && self.board[i] != BoardState::EMPTY{
                return true;
            }
        }

        // 1st diagonal check
        if self.board[0] == self.board[4] && self.board[0] == self.board[8] && self.board[8] != BoardState::EMPTY{
            return true;
        }


        // second diagonal check
        if self.board[2] == self.board[4] && self.board[2] == self.board[6] && self.board[2] != BoardState::EMPTY{
            return true;
        }

        return false;
    }
}



fn show_board(board: &Board) {
    for (i, elem) in board.board.iter().enumerate() {
        let value =  match elem{
            BoardState::EMPTY => (i + 1).to_string(),
            BoardState::O => String::from("O"),
            BoardState::X => String::from("X")
        };
        if (i + 1) % 3 == 0{
            println!("{}", value);
        } 
        else{
            print!("{} | ", value);
        }
    }
    println!("");
}

fn game(board: &mut Board){
    let mut is_x_turn = true;

    let mut winner = BoardState::EMPTY;
    let mut is_game_over = false;
    while !is_game_over{
        // get the character otken and print he required information
        let token = if is_x_turn {
            BoardState::X
        }
        else {
            BoardState::O
        };
        
        let mut invalid_input = true;
        let mut index:usize = 0;

        while invalid_input{
            if is_x_turn {
                println!("X TURN");
                BoardState::X
            }
            else {
                println!("O turn");
                BoardState::O
            };


            // get input
            let mut inp = String::new();
            println!("Enter a place to input: ");
            io::stdin().read_line(&mut inp).expect("Failed to readline");

            //parse the input to get the index
            index = inp.trim().parse().unwrap();
            
            // check if the index is valid
            invalid_input = if index > 0 && index <= 9{
                // check if the cell is empty or not
                if !board.is_empty_cell(index as usize - 1){
                    println!("Cell already occupied");
                    invalid_input
                }
                else {
                    // if the cell is not empty it it is an invalid input
                    false
                }
            } 
            else {
                println!("Invalid Input");
                invalid_input
            }
        }

        // set the required index
        board.set_board_state(index as usize - 1, &token);

        // show board
        show_board(board);

        // check if the game is over
        winner =if board.is_game_over() {
            is_game_over = true;
            token
        }
        else if board.is_all_occupied() {
            is_game_over = true;
            BoardState::EMPTY
        }
        else {
            BoardState::EMPTY
        };

        // alter the turn
        is_x_turn = !is_x_turn;
    }

    match winner{
        BoardState::EMPTY => {
            println!("Game Draw");
        }
        BoardState::X => {
            println!("game winner: X");
        },
        BoardState::O => {
            println!("game winner: O");
        }
    }
}


fn main() {
    let mut board = Board{
        board: vec![
            BoardState::EMPTY,
            BoardState::EMPTY,
            BoardState::EMPTY,
            BoardState::EMPTY,
            BoardState::EMPTY,
            BoardState::EMPTY,
            BoardState::EMPTY,
            BoardState::EMPTY,
            BoardState::EMPTY,
        ],
    };

    board.init();
    game(&mut board);
}