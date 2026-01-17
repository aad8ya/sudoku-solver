mod board;
use board::SudokuBoard;

fn main() {
    let puzzle = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

    match SudokuBoard::new(puzzle) {
        Some(board) => {
            println!("Board initialized successfully");
            let candidates = board.get_candidates(0, 2);
            println!("Candidates for cell (0, 2) in bitmask: {:b}", candidates);

            for val in 1..=9 {
                if candidates & (1 << val) != 0 {
                    println!("{}", val);
                }
            }
        }
        None => {
            println!("Failed to initialize board");
        }
    }
}
