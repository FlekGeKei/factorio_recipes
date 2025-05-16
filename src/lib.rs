use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const LV: &str = "│";
const LVAR: &str = "├";
const LUAR: &str = "└";
const LH: &str = "─";

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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
                    "ERROR: trying to found non-existing \"{}\" in {hash_map:?}",
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

    pub fn print(&self, refr: Option<String>) {
        let prefix = refr.unwrap_or_default();
        let cont = self.sub_instruction.is_some();

        let name_prf = LVAR.to_owned() + " Name....";
        let amount_prf = if !cont { LUAR } else { LVAR }.to_owned() + " Amount..";

        println!("{}Instruction", prefix);

        let last_cahr = prefix.chars().next_back().unwrap_or_default().to_string();
        let prefix = match last_cahr.as_ref() {
            LUAR => prefix[..prefix.len() - last_cahr.len()].to_string() + " ",
            LVAR => prefix[..prefix.len() - last_cahr.len()].to_string() + LV,
            _ => prefix,
        };

        println!("{}{}{}", prefix, name_prf, self.ingredient.name);
        println!("{}{}{}", prefix, amount_prf, self.ingredient.amount);

        if !cont {
            return;
        }

        println!("{}{}{} Sub instruction", prefix, LUAR, LH);

        let pre_refr = prefix + "   ";

        let mut i = 0;
        let ingr = self.sub_instruction.as_ref().unwrap();
        while i < ingr.len() - 1 {
            ingr[i].print(Some(pre_refr.to_string() + LVAR));
            i += 1
        }
        ingr[i].print(Some(pre_refr + LUAR));
    }

    pub fn get_complex(&self, hash_map: &HashMap<String, Recipe>) -> Option<Vec<Ingredient>> {
        let recepy = hash_map.get(&self.ingredient.name).unwrap();
        match &recepy.kind {
            RecipeKind::Complex(kind) => Some(vec![Ingredient {
                name: self.ingredient.name.clone() + "^" + kind,
                amount: self.ingredient.amount,
            }]),
            RecipeKind::Simple => {
                self.sub_instruction.as_ref()?;

                let mut vec_ingr: Vec<Ingredient> = Vec::new();
                for instr in self.sub_instruction.as_ref().unwrap() {
                    let ingrs = instr.get_complex(hash_map);

                    if ingrs.is_none() {
                        continue;
                    }

                    for ingr in ingrs.unwrap_or_default() {
                        vec_ingr.push(ingr);
                    }
                }
                Some(vec_ingr)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum RecipeKind {
    Complex(String),
    Simple,
}
