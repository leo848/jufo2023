use std::{
    error::Error,
    fs::File,
    mem,
};

use clap::ArgMatches;
use itertools::Itertools;
use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};
use shakmaty::{Board, Chess, Move, Position};

use crate::common::*;
const MIN_ELO: u32 = 1500;
const MIN_TIME: u32 = 300;

const ONLY_CHECKMATES: bool = true;
const ONLY_OPENINGS: bool = false;
const ONLY_MIDDLE_GAME: bool = false;
const ONLY_ENDGAME: bool = true;


#[derive(Debug, Clone)]
struct NeuralInputCreator {
    board: Chess,
    moves: Vec<(Chess, Move)>,
    considerable_game: bool,
    move_count: usize,
}

impl NeuralInputCreator {
    fn new() -> Self {
        Self {
            board: Chess::default(),
            moves: Vec::new(),
            move_count: 0,
            considerable_game: true,
        }
    }
}

impl Visitor for NeuralInputCreator {
    type Result = Option<Vec<(Chess, Move)>>;

    fn begin_game(&mut self) {
        self.board = Chess::default();
        self.moves.clear();
        self.move_count = 0;
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
        if ONLY_OPENINGS && self.move_count >= 15 {
            return;
        }

        self.move_count += 1;
        match san_plus.san.to_move(&self.board) {
            Ok(m) => {
                if (!ONLY_MIDDLE_GAME
                    || (self.move_count < 15 || material_count(&self.board.board()) < 28))
                    && (!ONLY_ENDGAME || material_count(&self.board.board()) >= 28)
                {
                    self.moves.push((self.board.clone(), m.clone()));
                }
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

fn material_count(board: &Board) -> usize {
        (board.knights() & board.bishops()).count() * 3
            + board.queens().count() * 9
            + board.rooks().count() * 5
            + board.pawns().count()
}

pub fn main(options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let pgn = File::open(
        options.get_one::<String>("pgn-file").expect("required")
    )?;

    let mut reader = BufferedReader::new(&pgn);
    let mut counter = NeuralInputCreator::new();

    let io_pairs = std::iter::from_fn(|| reader.read_game(&mut counter).ok().flatten())
        .flatten()
        .flatten();

    save_boards(io_pairs)?;

    Ok(())
}
