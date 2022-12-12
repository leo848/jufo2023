use std::{path::Path, fs::File, error::Error};
use npyz::{NpyFile, WriteOptions, WriterBuilder};

use crate::BOARDS_PER_FILE;

use crate::INPUT_LENGTH;
use crate::INTERSPERSE_CHUNK_SIZE;

const READ_INPUT_DIR: &str = "../npy_files/20M_neural_input";
const READ_OUTPUT_DIR: &str = "../npy_files/20M_neural_output";
const WRITE_INPUT_DIR: &str = "../npy_files/20M_neural_input_interspersed";
const WRITE_OUTPUT_DIR: &str = "../npy_files/20M_neural_output_interspersed";

const READER_AMOUNT: usize = 16;

pub fn intersperse_files() -> Result<(), Box<dyn Error>> {

    if Path::new(WRITE_INPUT_DIR).exists() || Path::new(WRITE_OUTPUT_DIR).exists() {
        panic!("Directory already exists!");
    }


    // Create READER_AMOUNT numpy readers. This is one for each file in the input directory.
    let mut input_readers = Vec::new();
    let mut output_readers = Vec::new();

    for i in 0..READER_AMOUNT {
        let read_input_path = Path::new(READ_INPUT_DIR).join(format!("{i}.npy"));
        let input_reader = NpyFile::new(File::open(read_input_path)?)?;

        let read_output_path = Path::new(READ_OUTPUT_DIR).join(format!("{i}.npy"));
        let output_reader = NpyFile::new(File::open(read_output_path)?)?;

        input_readers.push(input_reader);
        output_readers.push(output_reader);
    }

    for i in 0..READER_AMOUNT {
        let mut inputs = File::create(Path::new(WRITE_INPUT_DIR).join(format!("{i}.npy")))?;
        let input_writer = npyz::WriteOptions::new()
            .default_dtype()
            .shape(&[BOARDS_PER_FILE as u64, INPUT_LENGTH as u64])
            .writer(&mut inputs)
            .begin_nd()?;


    }


    Ok(())
}
