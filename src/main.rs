mod game;
mod board;

fn main() {

    let mut player1: game::Player = game::Player::new(board::Color::BLACK);
    let mut player2: game::Player = game::Player::new(board::Color::WHITE);

    let mut game_board: board::Board = board::Board::new(9);
    game_board.board_state[0][0] = 1;
    game_board.board_state[0][1] = 2;
    println!("{:?}", game_board.board_state);

    println!("P1 Stones Taken: {}",player1.add_stone_taken(1));
    println!("P1 Stones Taken: {}",player1.add_stone_taken(3));
    println!("P2 Stones Taken: {}",player2.add_stone_taken(5));


}
