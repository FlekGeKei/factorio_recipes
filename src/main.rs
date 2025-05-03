use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map, HashMap},
    env,
    fs::File,
    io::BufReader,
    path::Path,
    vec,
};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Recipe {
    name: String,
    kind: RecipeKind,
    ingredients: Option<Vec<Ingredient>>, // id and amount
    craft_time: f64,
    craft_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Ingredient {
    name: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Instruction {
    ingredient: Ingredient,
    sub_instruction: Option<Vec<Instruction>>,
}

#[derive(Debug, Deserialize, Serialize)]
enum RecipeKind {
    Complex(String),
    Simple,
}

fn get_instruction(
    hash_map: &HashMap<String, Recipe>,
    ingr_orders: Vec<Ingredient>,
) -> Vec<Instruction> {
    let mut got_instruction: Vec<Instruction> = Vec::new();

    for ingr in &ingr_orders {
        let sub_instructions: Option<Vec<Instruction>>;
        match hash_map.get(&ingr.name) {
            Some(ingr_recipe) => {
                let ingr_production_per_second =
                    ingr.amount / ingr_recipe.craft_amount / ingr_recipe.craft_time;
                match &ingr_recipe.ingredients {
                    Some(ingr_recipe_ingrs) => {
                        let mut tmp_instructions: Vec<Instruction> = Vec::new();
                        for ingr_recipe_ingr_to_order in ingr_recipe_ingrs {
                            let neded_amount =
                                ingr_recipe_ingr_to_order.amount * ingr_production_per_second;
                            let instruction_out = get_instruction(
                                hash_map,
                                vec![Ingredient {
                                    name: ingr_recipe_ingr_to_order.name.clone(),
                                    amount: neded_amount,
                                }],
                            );
                            for instruct in instruction_out {
                                tmp_instructions.push(instruct);
                            }
                        }
                        sub_instructions = tmp_instructions.into();
                    }
                    None => sub_instructions = None,
                }
            }
            None => {
                panic!(
                    "ERROR: trying to found nonexisting \"{}\" in {hash_map:?}",
                    ingr.name
                );
            }
        }
        got_instruction.push(Instruction {
            ingredient: Ingredient {
                name: ingr.name.clone(),
                amount: ingr.amount,
            },
            sub_instruction: sub_instructions,
        });
    }

    got_instruction
}

fn recipe_add(path: &Path) -> std::io::Result<HashMap<String, Recipe>> {
    let mut hash_map: HashMap<String, Recipe> = Default::default();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json_recep: Vec<Recipe> = serde_json::from_reader(reader)?;
    for rec in json_recep {
        hash_map.insert(rec.name.clone(), rec);
    }
    Ok(hash_map)
}

fn get_order(path: &Path) -> std::io::Result<Vec<Ingredient>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json_orders: Vec<Ingredient> = serde_json::from_reader(reader)?;
    Ok(json_orders)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let recipes = recipe_add(Path::new("./recipes.json")).expect("normal json");

    let instruction = get_instruction(
        &recipes,
        get_order(Path::new("./request.json")).expect("normal json"),
    );
    dbg!(instruction);
    /*
    let mut refineries: f64 = 1.0;
    loop {
        let h_produced: f64 = refineries * HEAVY_OIL;
        let l_produced: f64 = refineries * LIGHT_OIL;
        let p_produced: f64 = refineries * PETROLEUM_GAS;

        if h_produced >= h_consumed && l_produced >= l_consumed && p_produced >= p_consumed {
            dbg!(&h_produced);
            dbg!(&l_produced);
            dbg!(&p_consumed);

            println!("required refinerys: {}", refineries);
            break;
        } else {
            refineries += 1.0;
        }
    }
    */
}
