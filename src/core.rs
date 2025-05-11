use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Recipe {
    name: String,
    kind: RecipeKind,
    ingredients: Option<Vec<Ingredient>>,
    craft_time: f64,
    craft_amount: f64,
}
impl Recipe {
    pub fn get_recipes(path: &Path) -> Result<HashMap<String, Self>, std::io::Error> {
        let mut hash_map: HashMap<String, Recipe> = Default::default();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let json_recep: Vec<Recipe> = serde_json::from_reader(reader)?;
        for rec in json_recep {
            hash_map.insert(rec.name.clone(), rec);
        }

        Ok(hash_map)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Ingredient {
    pub name: String,
    pub amount: f64,
}

impl Ingredient {
    pub fn get_request(path: &Path) -> Result<Vec<Self>, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let json_request: Vec<Ingredient> = serde_json::from_reader(reader)?;

        Ok(json_request)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Instruction {
    pub ingredient: Ingredient,
    pub sub_instruction: Option<Vec<Instruction>>,
}

impl Instruction {
    pub fn get_instruction(hash_map: &HashMap<String, Recipe>, order: Ingredient) -> Self {
        let sub_instructions: Option<Vec<Instruction>>;

        match hash_map.get(&order.name) {
            Some(order_recipe) => {
                let order_production_per_second =
                    order.amount / order_recipe.craft_amount / order_recipe.craft_time;

                match &order_recipe.ingredients {
                    Some(order_recipe_ingrs) => {
                        let mut tmp_instructions: Vec<Instruction> = Vec::new();

                        for order_recipe_ingr_to_order in order_recipe_ingrs {
                            let neded_amount =
                                order_recipe_ingr_to_order.amount * order_production_per_second;

                            let instruction_out = Instruction::get_instruction(
                                hash_map,
                                Ingredient {
                                    name: order_recipe_ingr_to_order.name.clone(),
                                    amount: neded_amount,
                                },
                            );
                            tmp_instructions.push(instruction_out);
                        }

                        sub_instructions = Some(tmp_instructions);
                    }
                    None => sub_instructions = None,
                }
            }
            None => {
                panic!(
                    "ERROR: trying to found nonexisting \"{}\" in {hash_map:?}",
                    &order.name
                );
            }
        }

        Instruction {
            ingredient: Ingredient {
                name: order.name.clone(),
                amount: order.amount,
            },
            sub_instruction: sub_instructions,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RecipeKind {
    Complex(String),
    Simple,
}
