use std::hash::Hasher;
use std::hash::Hash;
use std::collections::hash_map::DefaultHasher;
use std::{error::Error, fs::File, mem};

use itertools::Itertools;
use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};
use shakmaty::{Chess, Move, Position};

use crate::common::*;
const MIN_ELO: u32 = 1500;
const MIN_TIME: u32 = 300;

const ONLY_CHECKMATES: bool = true;

pub const AMOUNT_OF_BOARDS: usize = 20_000_000;
pub const BOARDS_PER_FILE: usize = 500_000;
pub const AMOUNT_OF_FILES: usize = AMOUNT_OF_BOARDS / BOARDS_PER_FILE;

const NEURAL_INPUT_DIR: &str = "../npy_files/20M_neural_input";
const NEURAL_OUTPUT_DIR: &str = "../npy_files/20M_neural_output";

const PGN_FILE: &str = "database-2016-06.pgn";

#[derive(Debug, Clone)]
struct NeuralInputCreator {
    board: Chess,
    moves: Vec<(Chess, Move)>,
    considerable_game: bool,
}

impl NeuralInputCreator {
    fn new() -> Self {
        Self {
            board: Chess::default(),
            moves: Vec::new(),
            considerable_game: true,
        }
    }
}

impl Visitor for NeuralInputCreator {
    type Result = Option<Vec<(Chess, Move)>>;

    fn begin_game(&mut self) {
        self.board = Chess::default();
        self.moves.clear();
        self.considerable_game = true;
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        let value = value.decode_utf8().expect("invalid utf8");
        if value == "-" || value == "?" {
            return;
        }
        if key == b"TimeControl" {
            let (time, inc) = value
                .split('+')
                .map(|s| s.parse::<u32>().expect("Invalid number"))
                .collect_tuple()
                .expect("Invalid time field");
            let heuristic = time + inc * 30;
            if heuristic < MIN_TIME {
                self.considerable_game = false;
            }
        } else if key.ends_with(b"Elo") {
            let elo = value.parse::<u32>().expect("Invalid number");
            if elo < MIN_ELO {
                self.considerable_game = false;
            }
        } else if key == b"Result" {
            if ONLY_CHECKMATES && value != "1-0" && value != "0-1" {
                self.considerable_game = false;
            }
        }
    }

    fn end_headers(&mut self) -> Skip {
        Skip(!self.considerable_game)
    }

    fn san(&mut self, san_plus: SanPlus) {
        if !self.considerable_game {
            return;
        }
        match san_plus.san.to_move(&self.board) {
            Ok(m) => {
                self.moves.push((self.board.clone(), m.clone()));
                self.board.play_unchecked(&m);
            }
            Err(_) => unreachable!("lichess would not permit this"),
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        if !self.considerable_game {
            return None;
        }
        if ONLY_CHECKMATES && !self.board.is_checkmate() {
            return None;
        }
        Some(mem::take(&mut self.moves))
    }
}

pub fn main(only_count: bool) -> Result<(), Box<dyn Error>> {
    assert!(NEURAL_INPUT_DIR.starts_with(".."), "NEURAL_INPUT_DIR must be relative to the root of the project");
    assert!(NEURAL_OUTPUT_DIR.starts_with(".."), "NEURAL_OUTPUT_DIR must be relative to the root of the project");

    let pgn = File::open(PGN_FILE)?;

    let mut reader = BufferedReader::new(&pgn);
    let mut counter = NeuralInputCreator::new();

    let io_pairs = std::iter::from_fn(|| reader.read_game(&mut counter).ok().flatten())
        .flatten()
        .flatten();

    if only_count {
        let count = io_pairs
            .unique_by(|(i, _)| {
                let mut hasher = DefaultHasher::new();
                i.hash(&mut hasher);
                hasher.finish()
            })
            .count();
        println!("{} boards", count);
    } else {
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
    }

    Ok(())
}
