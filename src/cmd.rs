use std::io;
use std::io::{stdin, Write};
use std::path::PathBuf;

use clap::Parser;
use factorio_recipes::Instruction;

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
enum Command {
    /// Import recipes path
    Import {
        recipes_path: PathBuf,
    },
    Exit,
}

struct State {
    recipes_path: PathBuf,
    instructions: Vec<Instruction>,
}

pub fn init_cmd() {
    let mut state = State {
        recipes_path: PathBuf::new(),
        instructions: Vec::new(),
    };
    loop {
        let mut cmd = String::from("> ");
        print!("> ");
        io::stdout().flush().expect("wtf");

        match stdin().read_line(&mut cmd) {
            Ok(o) => o,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        let command = match Command::try_parse_from(cmd.split_whitespace()) {
            Ok(o) => o,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        match command {
            Command::Import { recipes_path } => {
                if !recipes_path.is_file() {
                    println!("error: {recipes_path:?} is not a file");
                    continue;
                }
                if !recipes_path.exists() {
                    println!("error: {recipes_path:?} does not exist");
                    continue;
                }
                state.recipes_path = recipes_path;
            }
            Command::Exit => break,
        }
    }
}
