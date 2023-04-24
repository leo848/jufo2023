use std::{error::Error, process::exit};

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use lazy_static::lazy_static;

// mod intersperse;
mod common;
mod csv_to_numpy;
mod get_database;
mod pgn_to_numpy;
mod pgn_to_numpy_eval;

pub use common::*;

lazy_static! {
    pub static ref ARGS: ArgMatches = cli().get_matches();
}

fn cli() -> Command {
    let pgn_arg = Arg::new("pgn-file")
        .long("pgn-file")
        .short('f')
        .required(true)
        .default_value("database-2017-01.pgn");
    Command::new("rust-neural-chess")
        .author("Leo Blume")
        .about("Tools to create neural training data for the board game chess.")
        .arg(
            Arg::new("dry_run")
                .long("dry-run")
                .short('n')
                .help("Only count the number of games that would be converted to .npy files")
                // .default_value(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output directory prefix"),
        )
        .arg(
            Arg::new("total")
                .long("total")
                .short('t')
                .value_parser(value_parser!(usize))
                .help("Total amount of boards to convert"),
        )
        .arg(
            // Boards per file, as a usize, default 500_000
            Arg::new("boards_per_file")
                .long("boards-per-file")
                .short('b')
                .value_parser(value_parser!(usize))
                .default_value("500000")
                .help("Boards per file"),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("pgn-to-npy")
                .about("Convert a PGN database to training data")
                .arg(pgn_arg.clone()),
        )
        .subcommand(
            Command::new("pgn-to-eval")
                .about("Convert a PGN database to evaluation data")
                .arg(pgn_arg.clone()),
        )
        .subcommand(
            Command::new("csv-to-npy")
                .about("Convert a CSV database to training data")
                .arg(
                    Arg::new("csv-file")
                        .long("csv-file")
                        .short('f')
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get-database")
                .about("Download a Lichess database from the internet")
                .arg(
                    Arg::new("date")
                        .required(true)
                        .help("Date of the database to download"),
                ),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    match ARGS.subcommand() {
        Some(("pgn-to-npy", matches)) => {
            pgn_to_numpy::main(matches)?;
        }
        Some(("pgn-to-eval", matches)) => {
            pgn_to_numpy_eval::main(matches)?;
        }
        Some(("csv-to-npy", matches)) => {
            csv_to_numpy::main(matches)?;
        }
        Some(("get-database", matches)) => {
            get_database::main(matches)?;
        }
        Some(_) => {
            eprintln!("Unknown subcommand");
            exit(1);
        }
        None => unreachable!(),
    }

    Ok(())
}
