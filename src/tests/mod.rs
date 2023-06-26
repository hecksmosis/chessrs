use super::*;

#[test]
fn pawn_move() {
    let mut game = Game::default();

    assert_eq!(game.turn, 0);
    assert_eq!(game.make_move(Input::build("e4").unwrap()).is_ok(), true);
    assert_eq!(game.turn, 1);
    let mut e4_board = create_board();
    e4_board[4][4] = Piece::new(0b1, (4, 3));
    e4_board[6][4] = Piece::empty((4, 1).into());

    assert_eq!(game.get_board(), &e4_board);
}

#[test]
fn king_pawn_game() -> Result<(), String> {
    let mut game = Game::default();

    assert_eq!(game.turn, 0);
    assert_eq!(game.make_move(Input::build("e4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("e5")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("nf3")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("nc6")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("bc4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("bc5")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("c3")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("nf6")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("d4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("pxd4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("pxd4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("bb4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("bd2")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("nxe4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("bxb4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("nxb4")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("d5")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("c5")?).is_ok(), true);
    assert_eq!(game.make_move(Input::build("pxc6")?).is_ok(), true);
    assert_eq!(game.turn, 1);
    let mut final_board = create_board();
    final_board[5][5] = Piece::new(0b11, (5, 2));
    final_board[4][1] = Piece::new(0b1011, (1, 3));
    final_board[4][2] = Piece::new(0b100, (2, 3));
    final_board[4][4] = Piece::new(0b1011, (4, 3));
    final_board[2][2] = Piece::new(0b1, (2, 5));
    final_board[6][4] = Piece::empty((4, 1).into());
    final_board[6][3] = Piece::empty((3, 1).into());
    final_board[6][2] = Piece::empty((2, 1).into());
    final_board[7][2] = Piece::empty((2, 0).into());
    final_board[7][5] = Piece::empty((5, 0).into());
    final_board[7][6] = Piece::empty((6, 0).into());
    final_board[1][2] = Piece::empty((2, 6).into());
    final_board[1][4] = Piece::empty((4, 6).into());
    final_board[0][1] = Piece::empty((1, 7).into());
    final_board[0][5] = Piece::empty((5, 7).into());
    final_board[0][6] = Piece::empty((6, 7).into());

    assert_eq!(game.get_board(), &final_board);
    Ok(())
}

#[test]
fn handle_input_test() {
    let mut game = Game::default();

    assert_eq!(
        handle_input(&mut game, &mut "w7".to_string())
            .unwrap_err()
            .to_string(),
        "Invalid x coordinate"
    );
}
