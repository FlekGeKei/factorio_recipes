mod core;

use core::Ingredient;
use core::Instruction;
use core::Recipe;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Help to do
    #[arg(long)]
    recipes: Option<PathBuf>,

    /// Help to do
    #[arg(long)]
    request: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// help to do
    Cmd,
    /// help to do
    Cli {
        /// Help to do
        #[arg(long)]
        recipes: PathBuf,

        /// Help to do
        #[arg(long)]
        request: PathBuf,
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
    fn run(recipes_path: PathBuf, request_path: PathBuf) -> Vec<Instruction> {
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

    let recipes_path = match cli.recipes {
        Some(path) => path,
        None => PathBuf::from("./recipes.json"),
    };

    let request_path = match cli.request {
        Some(path) => path,
        None => PathBuf::from("./request.json"),
    };

    let instructions = match cli.command {
        Some(commands) => Commands::march(commands),
        None => Cli::run(recipes_path, request_path),
    };

    dbg!(instructions);
}
