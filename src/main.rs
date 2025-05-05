mod cli;
mod core;

use crate::core::get_instruction;
use crate::core::get_order;
use crate::core::get_recipes;
use core::Instruction;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let recipes = get_recipes(Path::new("./recipes.json")).expect("normal json");

    let mut instructions: Vec<Instruction> = Vec::new();
    for order in get_order(Path::new("./request.json")).expect("normal json") {
        instructions.push(get_instruction(&recipes, order));
    }
    dbg!(instructions);
}
