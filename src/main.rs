// Author   :   Mohammed Rehaan.
// Date     :   13/12/2021, Monday.
//
// Language :   Rust v1.56.1.

#![feature(let_else)]

mod parser;

use parser::BfParser;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Please provide an input")]
    NoInputProvided,
    #[error("The file format provided is incorrect. Please provide a \".bf\" format")]
    IncorrectFileFormat,
    #[error("No extension for the given file")]
    NoFileExtenstion,
    #[error("Too many arguments were provided to the interpreter")]
    TooManyArguments,
    #[error("Error parsing the file path to str")]
    ErrWhileReadingFilePath,
    #[error("Runtime Error: {0}")]
    RuntimeError(#[from] std::io::Error),
    #[error("error while parsing int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Out of the tape with index {0}")]
    OutOfTapeIndex(usize),
    #[error("Unknown token {0}")]
    UnknownToken(char),
    #[error("Error in stack module: {0}")]
    StackModuleError(#[from] crate::parser::stack::Error),
}

fn run() -> Result<(), Error> {
    let file = read_arguments()?;
    let mut parser = BfParser::from(file.as_str())?;
    parser.iterate()?;
    Ok(())
}

fn main() {
    match run() {
        Err(err) => {
            print!("Error: {error:?}\nInfo: {error}", error = err)
        },
        _ => {},
    }
    print!("\n");
}

fn read_arguments() -> Result<String, Error> {
        let bfargs: Vec<String> = std::env::args().collect();
        match bfargs.len() {
            1_usize => {
                Err(Error::NoInputProvided)
            },
            2_usize => {
                let file_dir = Path::new(&bfargs[1_usize]);
                match file_dir.extension() {
                    Some(ext) => {
                        if !ext.eq("bf") {
                            return Err(Error::IncorrectFileFormat)
                        }
                    },
                    None => {
                        return Err(Error::NoFileExtenstion)
                    },
                }
                let Some(file_dir_str) = file_dir.to_str() else {
                    return Err(Error::ErrWhileReadingFilePath)
                };
                Ok(std::fs::read_to_string(file_dir_str)?)
            },
            _ => {
                Err(Error::TooManyArguments)
            },
        }

}
