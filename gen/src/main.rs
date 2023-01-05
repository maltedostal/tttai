use tttai::game::*;

/// Generates all winning tic tac toe combinations This algorithm yields every possible winning
/// combination though not all yielded board may be reached by legal play.
fn main() -> Result<(), &'static str> {
    let mut winning_hashes = Vec::new();
    for hash in 0..19682 {
        let board = Board::parse(hash, FieldOcc::Empty)?;
        let validation = valid(&board);
        if validation.0 == true && validation.1 == true {
            winning_hashes.push(board.hash());
        }
    }
    for h in &winning_hashes {
        println!("{}", h);
    }
    Ok(())
}

/// Checks if the given board is in a position reachable by playing according to the rules, i. e.
/// `xs - os` is either 0 or 1 with `xs` being the number of Xs on the board and `os` being the
/// number of Os on the board and not both players are winning at the same time.
/// The first bool in the returned tuple indicates if the board is valid while the second indicates
/// if it is a winning position or not
fn valid(board: &Board) -> (bool, bool) {
    let mut xs = 0;
    let mut os = 0;
    for field in &board.fields {
        match field {
            FieldOcc::X => { xs += 1 }
            FieldOcc::O => { os += 1 }
            _ => {}
        }
    }
    if xs != os && xs - os != 1 {
        return (false, false);
    }
    match (is_winning(board, FieldOcc::X), is_winning(board, FieldOcc::O)) {
        (true, true) => (false, false),
        (x, o) if x != o => (true, true),
        _ => (true, false),
    }
}

fn is_winning(board: &Board, color: FieldOcc) -> bool {
    match board.fields {
        // first row
        [a, b, c, ..] if a == color && a == b && b == c => true,
        // second row
        [_, _, _, a, b, c, ..] if a == color && a == b && b == c => true,
        // thrird row
        [_, _, _, _, _, _, a, b, c] if a == color && a == b && b == c => true,
        // first column
        [a, _, _, b, _, _, c, ..] if a == color && a == b && b == c => true,
        // second column
        [_, a, _, _, b, _, _, c, ..] if a == color && a == b && b == c => true,
        // third column
        [_, _, a, _, _, b, _, _, c] if a == color && a == b && b == c => true,
        // first diagonal, starting top left
        [a, _, _, _, b, _, _, _, c] if a == color && a == b && b == c => true,
        // second diagonal, starting top right
        [_, _, a,  _, b, _, c, ..] if a == color && a == b && b == c => true,
        _ => false,
    }
}

