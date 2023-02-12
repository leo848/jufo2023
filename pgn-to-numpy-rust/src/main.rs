use std::error::Error;

use derive_more::IsVariant;

// mod intersperse;
mod common;
mod csv_to_numpy;
mod pgn_to_numpy;
mod pgn_to_numpy_eval;

pub use common::*;

const ACTION: Action = Action::PgnToNpy;

#[derive(Debug, Clone, Copy, IsVariant)]
#[allow(dead_code)]
enum Action {
    OnlyCount,
    PgnToNpy,
    PgnToEvalNpy,
    CsvToNpy,
    IntersperseNpys,
}

fn main() -> Result<(), Box<dyn Error>> {
    match ACTION {
        Action::OnlyCount => pgn_to_numpy::main(true)?,
        Action::PgnToNpy => pgn_to_numpy::main(false)?,
        Action::PgnToEvalNpy => pgn_to_numpy_eval::main()?,
        Action::CsvToNpy => csv_to_numpy::main()?,
        Action::IntersperseNpys => {
            todo!();
            //intersperse::intersperse_files()?,
        }
    }

    Ok(())
}
