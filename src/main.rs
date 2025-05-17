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
        #[arg(long)]
        recipes: Option<PathBuf>,

        /// Help to do
        #[arg(long)]
        request: Option<PathBuf>,
    },
}

impl Commands {
    fn march(commands: Commands) -> Vec<Instruction> {
        match commands {
            Commands::Cmd => panic!("CMD command is WIP"),
            Commands::Cli { recipes, request } => crate::Commands::cli_run(recipes, request),
        }
    }
}

impl crate::Commands {
    fn cli_run(recipes_path: Option<PathBuf>, request_path: Option<PathBuf>) -> Vec<Instruction> {
        let recipes_path = match recipes_path {
            Some(path) => path,
            None => PathBuf::from("./recipes.json"),
        };
        let request_path = match request_path {
            Some(path) => path,
            None => PathBuf::from("./request.json"),
        };

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

    let instructions = match args.command {
        Some(command) => Commands::march(command),
        None => Commands::cli_run(None, None),
    };

    Instruction::print_vec(&instructions, None);
}
