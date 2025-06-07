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

pub fn optimize_complex(complex: Vec<Ingredient>) {
    dbg!(&complex);
    let mut compl_rec: Vec<(String, f64)> = vec![];
    for ingr in complex {
        let compl_name = &ingr.name[ingr.name.find('^').expect("use normal shit") + 1..];
        let elem = compl_rec.iter_mut().find(|x| x.0 == compl_name);
        match elem {
            None => compl_rec.push((compl_name.to_string(), ingr.amount)),
            Some(i) => {
                if i.1 < ingr.amount {
                    i.1 = ingr.amount
                }
            }
        }
    }
    dbg!(compl_rec);
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Instruction {
    pub ingredient: Ingredient,
    pub sub_instruction: Option<Vec<Instruction>>,
}

impl Instruction {
    pub fn get_instruction(hash_map: &HashMap<String, Recipe>, request: Ingredient) -> Self {
        let sub_instructions: Option<Vec<Instruction>>;

        match hash_map.get(&request.name) {
            Some(request_recipe) => {
                let request_production_per_second =
                    request.amount / request_recipe.craft_amount / request_recipe.craft_time;

                match &request_recipe.ingredients {
                    Some(request_recipe_ingrs) => {
                        let mut tmp_instructions: Vec<Instruction> = Vec::new();

                        for request_recipe_ingr_to_request in request_recipe_ingrs {
                            let neded_amount = request_recipe_ingr_to_request.amount
                                * request_production_per_second;

                            let instruction_out = Instruction::get_instruction(
                                hash_map,
                                Ingredient {
                                    name: request_recipe_ingr_to_request.name.clone(),
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
                    &request.name
                );
            }
        }

        Instruction {
            ingredient: Ingredient {
                name: request.name.clone(),
                amount: request.amount,
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

        Instruction::print_vec(self.sub_instruction.as_ref().unwrap(), Some(prefix));
    }

    pub fn print_vec(instr: &[Instruction], pref: Option<String>) {
        let mut prefix = String::new();
        if pref.is_some() {
            prefix = pref.unwrap() + "   ";
        } else {
            println!("Instructions");
        }

        let mut i = 0;
        while i < instr.len() - 1 {
            instr[i].print(Some(prefix.clone() + LVAR));
            i += 1
        }
        instr[i].print(Some(prefix + LUAR));
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

                    'a: for ingr in ingrs.unwrap_or_default() {
                        if vec_ingr.is_empty() {
                            vec_ingr.push(ingr);
                            continue;
                        }

                        for elem in vec_ingr.iter_mut() {
                            if elem.name == ingr.name {
                                elem.amount += ingr.amount;
                                break 'a;
                            }
                        }
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
