use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{self, Write},
    path::Path,
    time::Instant,
};

use fs_err::{self as fs, File};
use itertools::Itertools;
use npyz::WriterBuilder;
use shakmaty::{Chess, Color, Move, Piece, Position, Square};

use crate::ARGS;

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

pub fn eval_to_output(eval: f32) -> f32 {
    // Calculate the sigmoid of the evaluation
    1.0 / (1.0 + (-eval).exp())
}

pub fn save_boards(
    io_pairs: impl Iterator<Item = (Chess, Move)>,
) -> io::Result<()> {
    save_boards_outputs(
        io_pairs
            .map(|(input, output)| (input, move_to_output(&output))),
        )
}

pub fn save_boards_outputs<T>(
    io_pairs: impl Iterator<Item = (Chess, T)>,
) -> io::Result<()>
where
    T: npyz::Serialize + npyz::AutoSerialize,
{
    if ARGS.get_flag("dry-run") {
        let count = io_pairs.count();
        println!("{} games would be converted to .npy files", count);
        return Ok(());
    }

    let neural_dir_prefix = ARGS.get_one::<String>("output").expect("No output directory specified");
    let neural_input_dir = &Path::new("../npy_files").join(neural_dir_prefix.clone() + "_input");
    let neural_output_dir = &Path::new("../npy_files").join(neural_dir_prefix.clone() + "_output");

    let total_data = *ARGS.get_one::<usize>("total").expect("No total data specified");
    let boards_per_file = *ARGS.get_one::<usize>("boards_per_file").expect("No boards per file specified");
    let amount_of_files = total_data / boards_per_file;

    assert!(total_data % boards_per_file == 0);


    if neural_input_dir.try_exists()? || neural_output_dir.try_exists()? {
        panic!("Directory already exists!");
    }

    for dir in [neural_input_dir, neural_output_dir] {
        fs::create_dir(dir)?;
    }

    let io_pairs_chunked = io_pairs
        .map(|(chess, output)| (chess_to_input(&chess), output))
        .unique_by(|(input, _)| {
            let mut hasher = DefaultHasher::new();
            input.hash(&mut hasher);
            hasher.finish()
        })
        .chunks(boards_per_file);

    let start_time = Instant::now();

    for (chunk_index, chunk) in io_pairs_chunked
        .into_iter()
        .take(amount_of_files)
        .enumerate()
    {
        let mut inputs = File::create(neural_input_dir.join(format!("{chunk_index}.npy")))?;
        let mut input_writer = {
            npyz::WriteOptions::new()
                .default_dtype()
                .shape(&[boards_per_file as u64, INPUT_LENGTH as u64])
                .writer(&mut inputs)
                .begin_nd()?
        };

        let mut outputs =
            File::create(neural_output_dir.join(format!("{chunk_index}.npy")))?;
        let mut output_writer = {
            npyz::WriteOptions::new()
                .default_dtype()
                .shape(&[boards_per_file as u64])
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
                    chunk_index * boards_per_file + board_index,
                    total_data,
                );
                input_writer.extend(input).expect("IO error");
                output_writer.push(&output).expect("IO error");
            });

        input_writer.finish()?;
        output_writer.finish()?;
    }

    Ok(())
}

fn debug(start_time: Instant, count: usize, total_data: usize) {
    if count % 1024 == 0 && count != 0 {
        let amount_of_boards = total_data;
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
            board_amount = total_data,
        );
        io::stderr().flush().expect("Couldn't flush stdout");
    }
}
