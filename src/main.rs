use std::io;

#[derive(PartialEq, Copy, Clone)]
enum BoardState{
    EMPTY,
    O,
    X
}


// overall board wrapper struct
struct Board{
    board: Vec<BoardState>,
}


// methods for board
impl Board{
    // static function to create a new board
    fn new() -> Board{
        return Board{
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
    }

    // get the turn of the player                
    fn get_player(&self) -> BoardState{
        let mut count_x = 0;
        let mut count_o = 0;

        for elem in self.board.iter() {
            match elem {
                BoardState::X => {
                    count_x += 1;
                },
                BoardState::O => {
                    count_o += 1;
                },
                _ => {}
            }
        }

        if count_o < count_x{
            BoardState::O
        }
        else {
            BoardState::X
        }
    }

    // setter function, i don't know why I created this
    fn set_board_state(&mut self, index:usize, state:BoardState){
        self.board[index] = state;
    }

    // check if the cell is empty ofc
    fn is_empty_cell(&self, index:usize) -> bool {
        match self.board[index] {
            BoardState::EMPTY => true,
            _ => false
        }
    }

    // check if the cell is occupied or not
    fn is_all_occupied(&self) -> bool{
        for elem in self.board.iter(){
            match elem{
                // if empty return false and be done with it
                BoardState::EMPTY => {
                    return false;
                },
                // if else do nothing, could have used if
                _ => {

                }
            }
        }

        return true;
    }

    // check if the game is over or not 
    fn is_game_over(&self) -> BoardState{
        // horizontal check
        for i in 0..3 {
            if self.board[3 * i] == self.board[3 * i + 1] && self.board[3 * i] == self.board[3 * i + 2] && self.board[3 * i] != BoardState::EMPTY{
                return self.board[3 * i];
            }
        }

        // vertical check
        for i in 0..3 {
            if self.board[i] == self.board[i + 3] && self.board[i] == self.board[i + 6] && self.board[i] != BoardState::EMPTY{
                return self.board[i];
            }
        }

        // 1st diagonal check
        if self.board[0] == self.board[4] && self.board[0] == self.board[8] && self.board[8] != BoardState::EMPTY{
            return self.board[4];
        }


        // second diagonal check
        if self.board[2] == self.board[4] && self.board[2] == self.board[6] && self.board[2] != BoardState::EMPTY{
            return self.board[4];
        }

        return BoardState::EMPTY;
    }

    // get all the empty cell in the board
    fn get_emply_states(&self) -> Vec<usize>{
        let mut emply_states = Vec::new();
        
        for (index, elem) in self.board.iter().enumerate() {
            match elem{
                BoardState::EMPTY => {
                    emply_states.push(index);
                }
                _ => {

                }
            }
        }
        
        return emply_states;
    }

    
    // heuristic function, get score of the terminal state
    // since tic tac toe all state can be stored
    fn get_board_score(&self) -> i64{
        let winner = self.is_game_over();
        match winner{
            BoardState::X => 1,
            BoardState::O => -1,
            BoardState::EMPTY => 0,
        }
    }


}

// get maximum value of the board state
// mini-max function
fn get_max_value(board: &mut Board, mut alpha: i64, beta: i64) -> i64{
    let mut new_board = Board::new(); 
    for i in 0..9 {
        new_board.set_board_state(i, board.board[i]);
    }

    // check if the terminal state
    if board.is_all_occupied() || board.is_game_over() != BoardState::EMPTY{
        return board.get_board_score();
    }

    // initial value of vax_value
    let mut max_value: i64 = -100000000;


    // get the possible actions for the current board state
    let possible_actions = board.get_emply_states();

    for action in possible_actions{
        // apply the action
        new_board.set_board_state(action, BoardState::X);
        max_value = std::cmp::max(max_value, get_min_value(&mut new_board, alpha, beta));

        //reverse the state so that it is safe to use for next iteration
        new_board.set_board_state(action, BoardState::EMPTY);

        // alpha-beta pruning
        if max_value >= beta{
            break;
        }

        alpha = std::cmp::max(max_value, alpha);
    }

    return max_value;
}


// get minimum value of the board state
// mini-max function
fn get_min_value(board: &mut Board, alpha: i64, mut beta: i64) -> i64{
    let mut new_board = Board::new(); 
    for i in 0..9 {
        new_board.set_board_state(i, board.board[i]);
    }


    // if it is a terminal state then return the evaluation of the board
    if board.is_all_occupied()  || board.is_game_over() != BoardState::EMPTY{
        return board.get_board_score();
    }

    // initial minimum value
    let mut min_value = 100000000;

    // get all the possible actions in the board
    let possible_actions = board.get_emply_states();

    for action in possible_actions{
        // apply the action
        new_board.set_board_state(action, BoardState::O);
        min_value = std::cmp::min(min_value, get_max_value(&mut new_board, alpha, beta));
        //reverse the state so that it is safe to use for next iteration
        new_board.set_board_state(action, BoardState::EMPTY);

        if min_value <= alpha {
            break;
        }

        beta = std::cmp::min(beta, min_value);
    }

    return min_value;
}



// overall minimax function
fn minimax(board: &mut Board) -> i64{
    let mut new_board = Board::new(); 
    for i in 0..9 {
        new_board.set_board_state(i, board.board[i]);
    }
    let mut selected_action:i64 = 0;

    let player = board.get_player();
    
    match player {
        BoardState::X => {
            let mut max_value: i64 = -100000000;

            let possible_actions = new_board.get_emply_states();

            for action in possible_actions {
                // apply the action
                new_board.set_board_state(action as usize, player);
                let min_value_result = get_min_value(&mut new_board, -100000000, 100000000);

                if min_value_result > max_value {
                    max_value = min_value_result;
                    selected_action = action as i64;
                }

                //reverse the state so that it is safe to use for next iteration
                new_board.set_board_state(action, BoardState::EMPTY);
            }

        },
        BoardState::O => {
            let mut min_value: i64 = 100000000;

            let possible_actions = new_board.get_emply_states();

            for action in possible_actions {
                // apply the action
                new_board.set_board_state(action as usize, player);
                let max_value_result = get_max_value(&mut new_board, -100000000, 100000000);

                if max_value_result < min_value {
                    min_value = max_value_result;
                    selected_action = action as i64;
                }

                //reverse the state so next action could be applied
                new_board.set_board_state(action, BoardState::EMPTY);
            }
        },
        BoardState::EMPTY => {}
    }

    return selected_action;
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
    println!("\n --------------------------------------------------");
}

fn game(board: &mut Board){
    let mut is_x_turn = true;

    let mut winner = BoardState::EMPTY;
    let mut is_game_over = false;
    while !is_game_over{
        // get the character otken and print he required information
        if is_x_turn {
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
                // trim because the string contain white space, and the panic occurs
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
            board.set_board_state(index as usize - 1, BoardState::X);
        }
        else {
            println!("\n O's turn:");
            let index = minimax(board);
            // set the required index
            board.set_board_state(index as usize, BoardState::O);
        };
        

        // show board
        show_board(board);

        // check if the game is over
        winner = board.is_game_over();
        match winner{
            BoardState::EMPTY => {},
            _ => {
                break;
            }
        };
        // if every cell is occupied, game over and draw
        if board.is_all_occupied() {
            is_game_over = true;
            winner = BoardState::EMPTY;
        }

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
    let mut board = Board::new(); 
    game(&mut board);
}
