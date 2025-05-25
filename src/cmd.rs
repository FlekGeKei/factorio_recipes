use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use factorio_recipes::{Ingredient, Instruction, Recipe};

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
enum Cmd {
    /// Import recipes path
    #[command(short_flag = 'm')]
    Import { path: PathBuf },
    /// Get instruction
    #[command(subcommand, short_flag = 'i')]
    Instructions(CmdInstr),
    /// Calculate complex recipes
    #[command(short_flag = 'c')]
    Calculate,
    /// Quit
    #[command(short_flag = 'q')]
    Quit,
}
#[derive(Subcommand, Debug)]
enum CmdInstr {
    Get {
        #[arg(long, short)]
        name: String,
        #[arg(long, short)]
        amount: f64,
    },
    Clear,
    Print,
}

struct State {
    recipes: HashMap<String, Recipe>,
    instructions: Vec<Instruction>,
}

pub fn init_cmd() {
    let mut state = State {
        recipes: HashMap::new(),
        instructions: Vec::new(),
    };
    loop {
        let mut cmd = String::from("> ");
        print!("> ");
        stdout().flush().expect("Can not flush stdout");

        match stdin().read_line(&mut cmd) {
            Ok(o) => o,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        let command = match Cmd::try_parse_from(cmd.split_whitespace()) {
            Ok(o) => o,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match command {
            Cmd::Import { path } => {
                if !path.is_file() {
                    println!("error: {path:?} is not a file");
                    continue;
                }
                if !path.exists() {
                    println!("error: {path:?} does not exist");
                    continue;
                }

                match Recipe::get_recipes(&path) {
                    Ok(o) => state.recipes = o,
                    Err(e) => {
                        println!("{e}");
                        continue;
                    }
                };
            }
            Cmd::Instructions(CmdInstr::Get { name, amount }) => {
                if state.recipes.is_empty() {
                    println!("ERROR: please import recipes first");
                    continue;
                }
                let instr =
                    Instruction::get_instruction(&state.recipes, Ingredient { name, amount });
                instr.print(None);
                state.instructions.push(instr);
            }
            Cmd::Instructions(CmdInstr::Clear) => {
                if state.instructions.is_empty() {
                    println!("INFO: Instructions are already empty");
                    continue;
                }
                state.instructions.clear();
            }
            Cmd::Instructions(CmdInstr::Print) => {
                if state.instructions.is_empty() {
                    println!("INFO: Instructions are empty");
                    continue;
                }
                Instruction::print_vec(&state.instructions, None);
            }
            Cmd::Calculate => todo!(),
            Cmd::Quit => break,
        }
    }
}
