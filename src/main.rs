mod cli;
mod core;

use crate::core::get_instruction;
use crate::core::get_order;
use crate::core::get_recipes;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let recipes = get_recipes(Path::new("./recipes.json")).expect("normal json");

    let instruction = get_instruction(
        &recipes,
        get_order(Path::new("./request.json")).expect("normal json"),
    );
    dbg!(instruction);
}
