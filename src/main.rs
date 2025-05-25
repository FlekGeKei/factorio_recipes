mod cmd;

use cmd::init_cmd;
use factorio_recipes::Ingredient;
use factorio_recipes::Instruction;
use factorio_recipes::Recipe;

use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// help to do
    Cmd,
    /// help to do
    Cli {
        /// Help to do
        #[arg(long, default_value = "./recipes.json")]
        recipes: PathBuf,

        /// Help to do
        #[arg(long, default_value = "./request.json")]
        request: PathBuf,
    },
}

impl Commands {
    fn c_match(self) {
        match self {
            Commands::Cmd => init_cmd(),
            Commands::Cli { recipes, request } => {
                let c = crate::Commands::cli_run(recipes, request);
                Instruction::print_vec(&c, None);
            }
        }
    }
}

impl crate::Commands {
    fn cli_run(recipes_path: PathBuf, request_path: PathBuf) -> Vec<Instruction> {
        let recipes = Recipe::get_recipes(&recipes_path).expect("normal json");

        let mut instructions: Vec<Instruction> = Vec::new();
        for order in Ingredient::get_request(&request_path).expect("normal json") {
            instructions.push(Instruction::get_instruction(&recipes, order));
        }

        dbg!(&instructions[0].get_complex(&recipes));

        instructions
    }
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(command) => command.c_match(),
        None => {
            let c = crate::Commands::cli_run(
                PathBuf::from("./recipes.json"),
                PathBuf::from("./request.json"),
            );
            Instruction::print_vec(&c, None);
        }
    }
}
