use std::error::Error;

use clap::ArgMatches;
use csv::{ReaderBuilder, StringRecord};
use shakmaty::{fen::Fen, uci::Uci, CastlingMode, Chess, Move, Position};

use crate::save_boards;

const CSV_FILE: &str = "puzzles.csv";

#[derive(Debug, Clone)]
struct Puzzle {
    fen: Fen,
    moves: Vec<Uci>,
}

pub fn main(_options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(CSV_FILE)?;

    let puzzles = reader.records().flatten().flat_map(parse_record);

    let io_pairs = puzzles_to_boards(puzzles);

    save_boards( io_pairs)?;

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
