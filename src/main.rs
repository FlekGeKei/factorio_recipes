mod core;

use core::Ingredient;
use core::Instruction;
use core::Recipe;

use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
            Commands::Cli { recipes, request } => Cli::run(recipes, request),
        }
    }
}

impl Cli {
    fn run(recipes_path: Option<PathBuf>, request_path: Option<PathBuf>) -> Vec<Instruction> {
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

        instructions
    }
}

fn main() {
    let cli = Cli::parse();

    let instructions = Commands::march(cli.command);

    dbg!(instructions);
}
