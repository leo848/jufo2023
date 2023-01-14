use std::{error::Error, fs::File};

use itertools::Itertools;
use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};
use rand::prelude::SliceRandom;
use shakmaty::{Chess, Move, Position};

use crate::common::*;
const MIN_ELO: u32 = 1500;
const MIN_TIME: u32 = 300;

pub const AMOUNT_OF_BOARDS: usize = 20_000_000;
pub const BOARDS_PER_FILE: usize = 1_000_000;
pub const AMOUNT_OF_FILES: usize = AMOUNT_OF_BOARDS / BOARDS_PER_FILE;

const NEURAL_INPUT_DIR: &str = "/../npy_files/20M_neural_input";
const NEURAL_OUTPUT_DIR: &str = "/../npy_files/20M_neural_output";

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
    type Result = Option<[(Chess, Move); 10]>;

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
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        if !self.considerable_game {
            return None;
        }
        let mut rng = rand::thread_rng();
        self.moves
            .choose_multiple(&mut rng, 10)
            .cloned()
            .collect_vec()
            .try_into()
            .ok()
    }
}

pub fn main(only_count: bool) -> Result<(), Box<dyn Error>> {
    let pgn = File::open(PGN_FILE)?;

    let mut reader = BufferedReader::new(&pgn);
    let mut counter = NeuralInputCreator::new();

    let io_pairs = std::iter::from_fn(|| reader.read_game(&mut counter).ok().flatten())
        .flatten()
        .flatten();

    if only_count {
        let count = io_pairs.count();
        println!("{} games", count);
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
