use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    mem,
};

use fs_err::File;
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, combinator::opt, number::complete::float};
use pgn_reader::{BufferedReader, Skip, Visitor};
use shakmaty::{Chess, Position};

use crate::{eval_to_output, save_boards_outputs, SaveConfig};

const PGN_FILE: &str = "database-2016-06.pgn";

const NEURAL_INPUT_DIR: &str = "../npy_files/20M_evaluations_input";
const NEURAL_OUTPUT_DIR: &str = "../npy_files/20M_evaluations_output";

pub const AMOUNT_OF_BOARDS: usize = 20_000_000;
pub const BOARDS_PER_FILE: usize = 500_000;
pub const AMOUNT_OF_FILES: usize = AMOUNT_OF_BOARDS / BOARDS_PER_FILE;

struct NeuralInputCreator {
    board: Chess,
    evaluations: Vec<(Chess, f32)>,
    has_evaluations: bool,
}

impl Default for NeuralInputCreator {
    fn default() -> Self {
        Self {
            board: Chess::default(),
            evaluations: Vec::default(),
            has_evaluations: true,
        }
    }
}

impl Visitor for NeuralInputCreator {
    type Result = Vec<(Chess, f32)>;

    fn begin_game(&mut self) {
        self.board = Chess::default();
        self.evaluations.clear();
        self.has_evaluations = true;
    }

    fn san(&mut self, san_plus: pgn_reader::SanPlus) {
        self.board
            .play_unchecked(&san_plus.san.to_move(&self.board).expect("invalid move"));

        // self.board
        // .play_unchecked(&san_plus.san.to_move(&self.board).expect("invalid move"));
        // dbg!(&self.board.board());
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn comment(&mut self, comment: pgn_reader::RawComment<'_>) {
        if !self.has_evaluations {
            return;
        }
        // The comment is in the form "[%eval -0.01] [%clk 0:00:30]". We want to extract the eval
        // as a float.
        let comment = comment.as_bytes();
        let Some(first_bracket) = comment.iter().position(|&b| b == b'[') else {
            self.has_evaluations = false;
            return;
        };

        let parse_result = parse_eval_comment(&comment[first_bracket..]).expect("invalid eval");
        let (_, eval) = parse_result;
        self.evaluations.push((self.board.clone(), eval));
    }

    fn end_game(&mut self) -> Self::Result {
        mem::take(&mut self.evaluations)
    }
}

fn parse_eval_comment(input: &[u8]) -> nom::IResult<&[u8], f32> {
    let (input, _) = tag(b"[%eval ")(input)?;
    alt((float, parse_checkmate))(input)
}

fn parse_checkmate(input: &[u8]) -> nom::IResult<&[u8], f32> {
    let (input, _) = tag(b"#")(input)?;
    // If the next character is a minus, return -inf.
    // If it is a number (not a plus), return inf.
    let (input, sign) = opt(tag(b"-"))(input)?;

    if sign.is_some() {
        Ok((input, f32::NEG_INFINITY))
    } else {
        Ok((input, f32::INFINITY))
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let pgn = File::open(PGN_FILE)?;

    let mut reader = BufferedReader::new(&pgn);
    let mut counter = NeuralInputCreator::default();

    let io_pairs = std::iter::from_fn(|| reader.read_game(&mut counter).ok().flatten())
        .flatten()
        .map(|(board, eval)| (board, eval_to_output(eval)));

    // panic!(
    //     "{} boards",
    //     io_pairs
    //         .unique_by(|(i, _)| {
    //             let mut hasher = DefaultHasher::new();
    //             i.hash(&mut hasher);
    //             hasher.finish()
    //         })
    //         .count()
    // );

    save_boards_outputs(
        io_pairs,
        SaveConfig::new(
            NEURAL_INPUT_DIR,
            NEURAL_OUTPUT_DIR,
            BOARDS_PER_FILE,
            AMOUNT_OF_FILES,
            false,
        ),
    )?;

    Ok(())
}
