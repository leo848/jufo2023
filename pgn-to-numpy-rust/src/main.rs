use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    fs::{create_dir, File},
    hash::{Hash, Hasher},
    io::{self, stderr, Write},
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
const ONLY_COUNT: bool = true;

const AMOUNT_OF_BOARDS: usize = 20_000_000;
const BOARDS_PER_FILE: usize = 1_000_000;
const AMOUNT_OF_FILES: usize = AMOUNT_OF_BOARDS / BOARDS_PER_FILE;

const NEURAL_INPUT_DIR: &str = "/tmp/20M_neural_input";
const NEURAL_OUTPUT_DIR: &str = "/tmp/20M_neural_output";

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

    output[0] = chess.turn().is_white();

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
    let (from, to) = match m {
        Move::Normal { from, to, .. } | Move::EnPassant { from, to, .. } => (*from, *to),
        Move::Castle { king, rook } => {
            let (king, king_to) = if king > rook {
                (
                    king,
                    Square::from_coords(king.file().offset(-2).expect("Invalid rank"), king.rank()),
                )
            } else {
                (
                    king,
                    Square::from_coords(king.file().offset(2).expect("Invalid rank"), king.rank()),
                )
            };
            (*king, king_to)
        }
        Move::Put { .. } => unreachable!(),
    };
    let from: u8 = from.into();
    let to: u8 = to.into();
    from as u16 * 64 + to as u16
}

fn main() -> Result<(), Box<dyn Error>> {
    let pgn = File::open(PGN_FILE)?;

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

fn debug(start_time: Instant, count: usize) {
    if count % 1024 == 0 && count != 0 {
        let remaining = AMOUNT_OF_BOARDS - count;
        let elapsed = start_time.elapsed();
        let time_per_board = elapsed / count as u32;

        let raw_eta = remaining as u32 * time_per_board;
        // Format the eta as a HH:MM:SS string
        let eta = format!(
            "{}:{:02}:{:02}",
            raw_eta.as_secs() / 3600,
            (raw_eta.as_secs() % 3600) / 60,
            raw_eta.as_secs() % 60
        );
        eprint!(
            "{count} / {AMOUNT_OF_BOARDS} ({:.3}%) - {board_time:.4}ms per board - ETA: {eta}\r",
            count as f32 * 100. / AMOUNT_OF_BOARDS as f32,
            board_time=time_per_board.as_secs_f32() * 1000.0
        );
        stderr().flush().expect("Couldn't flush stdout");
    }
}

fn save_boards(io_pairs: impl Iterator<Item = (Chess, Move)>) -> io::Result<()> {
    if Path::new(NEURAL_INPUT_DIR).exists() || Path::new(NEURAL_INPUT_DIR).exists() {
        assert!(NEURAL_INPUT_DIR != NEURAL_OUTPUT_DIR);
        panic!("Directory already exists!");
    }

    for dir in &[NEURAL_INPUT_DIR, NEURAL_OUTPUT_DIR] {
        create_dir(dir)?;
    }

    let io_pairs_chunked = io_pairs
            .map(|(chess, r#move)| (chess_to_input(&chess), move_to_output(&r#move)))
            .unique_by(|(input, _)| {
                let mut hasher = DefaultHasher::new();
                input.hash(&mut hasher);
                hasher.finish()
            })
            .chunks(BOARDS_PER_FILE);

    let start_time = Instant::now();

    for (chunk_index, chunk) in io_pairs_chunked.into_iter().take(AMOUNT_OF_FILES).enumerate() {
        let mut inputs = File::create(format!("{NEURAL_INPUT_DIR}/{chunk_index}.npy"))?;
        let mut input_writer = {
            npyz::WriteOptions::new()
                .default_dtype()
                .shape(&[BOARDS_PER_FILE as u64, INPUT_LENGTH as u64])
                .writer(&mut inputs)
                .begin_nd()?
        };

        let mut outputs = File::create(format!("{NEURAL_OUTPUT_DIR}/{chunk_index}.npy"))?;
        let mut output_writer = {
            npyz::WriteOptions::new()
                .default_dtype()
                .shape(&[BOARDS_PER_FILE as u64])
                .writer(&mut outputs)
                .begin_nd()?
        };

        chunk
            .enumerate()
            .for_each(|(board_index, (input, output))| {
                // i is the index in the chunk, not the total index
                // so we need to add the chunk index to it
                debug(start_time, chunk_index * BOARDS_PER_FILE + board_index);
                input_writer.extend(input).expect("IO error");
                output_writer.push(&output).expect("IO error");
            });

        input_writer.finish()?;
        output_writer.finish()?;
    }

    Ok(())
}
