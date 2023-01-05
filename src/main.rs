mod game;
mod win_check;

use game::*;
use std::time::*;
use win_check::*;

fn main() -> Result<(), &'static str> {
    let mut board = Board::init();
    loop {
        loop {
            let hash = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("ain't no way")
                .as_millis()
                % Board::MAX_BOARD_HASH as u128;
            board = Board::parse(hash as u16, FieldOcc::X)?;
            if is_winning(&board) {
                break;
            }
        }
        println!("{board}");
        println!(
            "Is {}",
            if is_winning(&board) {
                "a win!"
            } else {
                "not a win!"
            }
        );
        std::io::stdin().read_line(&mut String::with_capacity(0));
    }
    Ok(())
}
