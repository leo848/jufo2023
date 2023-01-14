use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
};

use csv::{ReaderBuilder, StringRecord};
use itertools::Itertools;
use shakmaty::{fen::Fen, uci::Uci, CastlingMode, Chess, Move, Position};

use crate::{save_boards, SaveConfig};

const CSV_FILE: &str = "puzzles.csv";

const NEURAL_INPUT_DIR: &str = "../npy_files/puzzle_input";
const NEURAL_OUTPUT_DIR: &str = "../npy_files/puzzle_output";

pub const BOARDS_PER_FILE: usize = 1_000_000;
const AMOUNT_OF_FILES: usize = 14;

#[derive(Debug, Clone)]
struct Puzzle {
    fen: Fen,
    moves: Vec<Uci>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(CSV_FILE)?;

    let puzzles = reader.records().flatten().flat_map(parse_record);

    let io_pairs = puzzles_to_boards(puzzles);

    save_boards(
        io_pairs,
        SaveConfig::new(
            NEURAL_INPUT_DIR,
            NEURAL_OUTPUT_DIR,
            BOARDS_PER_FILE,
            AMOUNT_OF_FILES,
            true,
        ),
    )?;

    Ok(())
}

fn parse_record(record: StringRecord) -> Result<Puzzle, Box<dyn Error>> {
    let fen = record[1].parse()?;
    let moves = record[2]
        .split_whitespace()
        .map(str::parse::<Uci>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Puzzle { fen, moves })
}

fn puzzles_to_boards(puzzles: impl Iterator<Item = Puzzle>) -> impl Iterator<Item = (Chess, Move)> {
    puzzles
        .map(|Puzzle { fen, moves }| {
            let mut chess: Chess = fen
                .into_position(CastlingMode::Standard)
                .expect("Invalid FEN");

            moves.into_iter().map(move |m| {
                let Ok(m) = m.to_move(&chess) else {
                    println!("Board: {:?}", chess.board());
                    println!("Invalid move: {}", m);
                    panic!("Invalid move");
                };
                let chess_before = chess.clone();
                chess.play_unchecked(&m);
                (chess_before, m)
            })
        })
        .flatten()
}
