use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{self, Write},
    path::Path,
    time::Instant,
};

use fs_err::{self as fs, File };

use itertools::Itertools;
use npyz::WriterBuilder;
use shakmaty::{Chess, Color, Move, Piece, Position, Square};

pub const INPUT_LENGTH: usize = 1 + (1 + 2 * 6) * 64;

pub fn chess_to_input(chess: &Chess) -> [bool; INPUT_LENGTH] {
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

pub fn move_to_output(m: &Move) -> u16 {
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

#[derive(Debug, Clone, Copy)]
pub struct SaveConfig<'a> {
    neural_input_dir: &'a Path,
    neural_output_dir: &'a Path,
    boards_per_file: usize,
    amount_of_files: usize,
    unique: bool,
}

impl<'a> SaveConfig<'a> {
    pub fn new<P>(
        neural_input_dir: &'a P,
        neural_output_dir: &'a P,
        boards_per_file: usize,
        amount_of_files: usize,
        unique: bool,
    ) -> Self
    where
        P: AsRef<Path> + 'a + ?Sized,
    {
        Self {
            neural_input_dir: neural_input_dir.as_ref(),
            neural_output_dir: neural_output_dir.as_ref(),
            boards_per_file,
            amount_of_files,
            unique,
        }
    }
}

pub fn save_boards(
    io_pairs: impl Iterator<Item = (Chess, Move)>,
    config: SaveConfig<'_>,
) -> io::Result<()> {
    assert!(config.neural_input_dir != config.neural_output_dir);
    if config.neural_input_dir.exists() || config.neural_output_dir.exists() {
        panic!("Directory already exists!");
    }

    for dir in [config.neural_input_dir, config.neural_output_dir] {
        fs::create_dir(dir)?;
    }

    let io_pairs_chunked = io_pairs
        .map(|(chess, r#move)| (chess_to_input(&chess), move_to_output(&r#move)))
        .unique_by(|(input, _)| {
            let mut hasher = DefaultHasher::new();
            input.hash(&mut hasher);
            hasher.finish()
        })
        .chunks(config.boards_per_file);

    let start_time = Instant::now();

    for (chunk_index, chunk) in io_pairs_chunked
        .into_iter()
        .take(config.amount_of_files)
        .enumerate()
    {
        let mut inputs = File::create(config.neural_input_dir.join(format!("{chunk_index}.npy")))?;
        let mut input_writer = {
            npyz::WriteOptions::new()
                .default_dtype()
                .shape(&[config.boards_per_file as u64, INPUT_LENGTH as u64])
                .writer(&mut inputs)
                .begin_nd()?
        };

        let mut outputs =
            File::create(config.neural_output_dir.join(format!("{chunk_index}.npy")))?;
        let mut output_writer = {
            npyz::WriteOptions::new()
                .default_dtype()
                .shape(&[config.boards_per_file as u64])
                .writer(&mut outputs)
                .begin_nd()?
        };

        chunk
            .enumerate()
            .for_each(|(board_index, (input, output))| {
                // i is the index in the chunk, not the total index
                // so we need to add the chunk index to it
                debug(
                    start_time,
                    chunk_index * config.boards_per_file + board_index,
                    &config,
                );
                input_writer.extend(input).expect("IO error");
                output_writer.push(&output).expect("IO error");
            });

        input_writer.finish()?;
        output_writer.finish()?;
    }

    Ok(())
}

fn debug(start_time: Instant, count: usize, config: &SaveConfig<'_>) {
    if count % 1024 == 0 && count != 0 {
        let amount_of_boards = config.boards_per_file * config.amount_of_files;
        let remaining = amount_of_boards - count;
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
            "{count} / {board_amount} ({:.3}%) - {board_time:.4}ms per board - ETA: {eta}\r",
            count as f32 * 100. / amount_of_boards as f32,
            board_time = time_per_board.as_secs_f32() * 1000.0,
            board_amount = config.amount_of_files * config.boards_per_file
        );
        io::stderr().flush().expect("Couldn't flush stdout");
    }
}
