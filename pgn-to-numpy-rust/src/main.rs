#![warn(clippy::pedantic)]

use std::{
    error::Error,
    fs::File,
    io::{self, stdout, Write},
    path::Path,
    time::Instant,
};

use itertools::Itertools;
use npyz::WriterBuilder;
use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};
use rand::prelude::SliceRandom;
use shakmaty::{Chess, Color, Move, Piece, Position, Square};

const MIN_ELO: u32 = 1500;
const MIN_TIME: u32 = 300;
const ONLY_COUNT: bool = false;

const AMOUNT_OF_BOARDS: usize = 3_000_000;

const NEURAL_INPUT_FILE: &str = "3M_training_input.npy";
const NEURAL_OUTPUT_FILE: &str = "3M_training_output.npy";

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
                println!("Error: {}", e);
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
        self.moves
            .choose_multiple(&mut rand::thread_rng(), 10)
            .cloned()
            .collect_vec()
            .try_into()
            .ok()
    }
}

const INPUT_LENGTH: usize = 1 + (1 + 2 * 6) * 64;
fn chess_to_input(chess: &Chess) -> [bool; INPUT_LENGTH] {
    let mut output = [false; INPUT_LENGTH];

    output[0] = if chess.turn().is_white() { true } else { false };

    let board = chess.board().clone();
    for (index, square) in Square::ALL.into_iter().enumerate() {
        let mut output_index = index * (1 + 2 * 6) + 1;
        output_index += match board.piece_at(square) {
            None => 0,
            Some(Piece { color, role }) => {
                (match color {
                    Color::White => 0usize,
                    Color::Black => 6,
                } + u32::from(role) as usize)
            }
        };
        output[output_index] = true;
    }

    output
}

fn move_to_output(m: &Move) -> u16 {
    let from: u8 = m.from().unwrap().into();
    let to: u8 = m.to().into();
    from as u16 * 64 + to as u16
}

fn main() -> Result<(), Box<dyn Error>> {
    let pgn = File::open("database.pgn")?;

    let mut reader = BufferedReader::new(&pgn);
    let mut counter = NeuralInputCreator::new();

    let io_pairs = std::iter::from_fn(|| reader.read_game(&mut counter).ok().flatten())
        .flatten()
        .flatten();

    if ONLY_COUNT {
        let count = io_pairs.count();
        println!("{} games", count);
    } else {
        save_boards(io_pairs)?;
    }

    Ok(())
}

fn save_boards(io_pairs: impl Iterator<Item = (Chess, Move)>) -> io::Result<()> {
    if Path::new(NEURAL_INPUT_FILE).exists() || Path::new(NEURAL_OUTPUT_FILE).exists() {
        panic!("File already exists!");
    }

    let mut inputs = File::create(NEURAL_INPUT_FILE)?;
    let mut input_writer = {
        npyz::WriteOptions::new()
            .default_dtype()
            .shape(&[AMOUNT_OF_BOARDS as u64, INPUT_LENGTH as u64])
            .writer(&mut inputs)
            .begin_nd()?
    };

    let mut outputs = File::create(NEURAL_OUTPUT_FILE)?;
    let mut output_writer = {
        npyz::WriteOptions::new()
            .default_dtype()
            .shape(&[AMOUNT_OF_BOARDS as u64])
            .writer(&mut outputs)
            .begin_nd()?
    };

    let start_time = Instant::now();

    io_pairs
        .map(|(chess, r#move)| (chess_to_input(&chess), move_to_output(&r#move)))
        .unique_by(|(input, _)| *input)
        .take(AMOUNT_OF_BOARDS)
        .enumerate()
        .inspect(|(i, _)| {
            if *i % 1024 == 0 && *i != 0 {
                let remaining = AMOUNT_OF_BOARDS - i;
                let elapsed = start_time.elapsed();
                let time_per_board = elapsed / *i as u32;
                let eta = remaining as u32 * time_per_board;
                print!(
                    "{} / {} - {elapsed:?} - ETA: {eta:?} - {time_per_board:?} per board\r",
                    i, AMOUNT_OF_BOARDS
                );
                stdout().flush().expect("Couldn't flush stdout");
            }
        })
        .for_each(|(_, (input, output))| {
            input_writer.extend(input).expect("IO error");
            output_writer.push(&output).expect("IO error");
        });

    input_writer.finish()?;

    Ok(())
}
