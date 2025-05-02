use serde::{Deserialize, Serialize};
use serde_json::*;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::BufReader,
    path::{self, Path},
    vec,
};

const HEAVY_OIL: f64 = 25.0;
const LIGHT_OIL: f64 = 45.0;
const PETROLEUM_GAS: f64 = 55.0;

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
        let mut sub_instructions: Vec<Instruction> = Vec::new();
        match hash_map.get(&ingr.name) {
            Some(ingr_recipe) => {
                let ingr_production_per_second =
                    ingr.amount / ingr_recipe.craft_amount / ingr_recipe.craft_time;
                match &ingr_recipe.ingredients {
                    Some(ingr_recipe_ingrs) => {
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

                            sub_instructions.push(Instruction {
                                ingredient: Ingredient {
                                    name: String::new(),
                                    amount: 0.0,
                                },
                                sub_instruction: instruction_out.into(),
                            });
                        }
                    }
                    None => sub_instructions.push(Instruction {
                        ingredient: Ingredient {
                            name: ingr.name.clone(),
                            amount: ingr.amount,
                        },
                        sub_instruction: None,
                    }),
                }
            }
            None => {
                panic!(
                    "ERROR: trying to found nonexisting \"{}\" in {hash_map:?}",
                    ingr.name
                );
            }
        }

        let sub_instruction: Option<Vec<Instruction>> = {
            let mut tmp_inst: Vec<Instruction> = Vec::new();
            for instruction in sub_instructions {
                if instruction.sub_instruction.is_none() {
                } else {
                    tmp_inst.append(&mut instruction.sub_instruction.unwrap());
                }
            }
            tmp_inst.into()
        };
        got_instruction.push(Instruction {
            ingredient: Ingredient {
                name: ingr.name.clone(),
                amount: ingr.amount,
            },
            sub_instruction,
        });
    }

    got_instruction
}

/*
fn get_ingr(hash_map: &HashMap<String, Recipe>, ingr_orders: Vec<Ingredient>) -> Vec<Ingredient> {
    let mut got_ingr: Vec<Ingredient> = Vec::new();

    for ingr in &ingr_orders {
        match hash_map.get(&ingr.name) {
            Some(ingr_recepe) => {
                let ingr_production_per_second =
                    ingr.amount / ingr_recepe.craft_amount / ingr_recepe.craft_time;
                match &ingr_recepe.ingredients {
                    Some(ingr_recepe_ingrs) => {
                        for ingr_recepe_ingr_to_order in ingr_recepe_ingrs {
                            let neded_amount =
                                ingr_recepe_ingr_to_order.amount * ingr_production_per_second;
                            let mut ingr_out = get_ingr(
                                hash_map,
                                vec![Ingredient {
                                    name: ingr_recepe_ingr_to_order.name.clone(),
                                    amount: neded_amount,
                                }],
                            );
                            println!(
                                "LOG get_ingr: next is \"{}\"",
                                ingr_recepe_ingr_to_order.name
                            );

                            got_ingr.append(&mut ingr_out);
                        }
                    }
                    None => got_ingr.push(Ingredient {
                        name: ingr.name.clone(),
                        amount: ingr.amount,
                    }),
                }
            }
            None => {
                panic!(
                    "ERROR: trying to found nonexisting \"{}\" in {hash_map:?}",
                    ingr.name
                );
            }
        }
    }

    got_ingr
}
*/

/*
#[allow(dead_code)]
fn finde_base_ingredients(hash_map: &HashMap<String, Recipe>) -> Vec<Ingredient> {
    for (name, recepe) in hash_map.iter() {
        println!("{}", name);
        println!("{}", recepe.craft_amount);
        for item in recepe.ingredients.as_deref().unwrap_or_default().iter() {
            // i used this shit because of the Option<> mf
            dbg!(&item);
        }
    }
    vec![Ingredient {
        name: "oil".to_string(),
        amount: 2.0,
    }]
}
*/
fn recipe_add(hash_map: &mut HashMap<String, Recipe>, path: &Path) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json_recep: Vec<Recipe> = serde_json::from_reader(reader)?;
    for rec in json_recep {
        hash_map.insert(rec.name.clone(), rec);
    }
    Ok(())
}

fn get_order(path: &Path) -> std::io::Result<Vec<Ingredient>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json_orders: Vec<Ingredient> = serde_json::from_reader(reader)?;
    Ok(json_orders)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let construction_robots: f64 = 5.0 / 27.0;
    let logistic_robots: f64 = 5.0 / 43.0;
    let land_mine: f64 = 1.0 / 5.0;
    let rockets: f64 = 2.0 / 4.0;
    let rocket_fuel: f64 = 1.0 / 15.0;
    let accumulator: f64 = 5.0 / 10.0;

    let robot_frame: f64 = (construction_robots + logistic_robots) / 20.0;
    let blue_circuits: f64 = 12.0 / 10.0;

    let eletric_engines: f64 = robot_frame / 10.0;
    let low_density_structures: f64 = 1.0 / 15.0;
    let explosives: f64 = (rockets + land_mine * 2.0) / 2.0 / 4.0;
    let batterys: f64 = (accumulator * 5.0 + robot_frame * 2.0) / 4.0;

    let sulfuric_acid: f64 = (blue_circuits * 5.0 + batterys * 20.0) / 50.0;

    let sulfur: f64 = (explosives + sulfuric_acid * 5.0) / 2.0;
    let lubricant: f64 = (eletric_engines * 15.0) / 10.0;
    let plastic: f64 = (low_density_structures * 5.0) / 2.0;
    let solid_fuil: f64 = rocket_fuel * 10.0;

    let heavy_oil: f64 = (solid_fuil * 19.0) / 5.0;
    let light_oil: f64 = (lubricant * 10.0 + rocket_fuel * 10.0) / 5.0;
    let petroleum_gas: f64 = (sulfur * 30.0 + plastic * 20.0) / 5.0;

    let h_consumed: f64 = heavy_oil * HEAVY_OIL;
    let l_consumed: f64 = light_oil * LIGHT_OIL;
    let p_consumed: f64 = petroleum_gas * PETROLEUM_GAS;

    dbg!(&h_consumed);
    dbg!(&l_consumed);
    dbg!(&p_consumed);

    let mut recipes: HashMap<String, Recipe> = HashMap::new();

    let _ = recipe_add(&mut recipes, Path::new("./recipes.json"));

    let instruction = get_instruction(
        &recipes,
        get_order(Path::new("./request.json")).expect("normal json"),
    );
    dbg!(instruction);

    dbg!(&recipes);

    /*
    recipe_add(&mut recepes, None, "petroleum_gas".to_string(), 5.0, 25.0);

    recipe_add(
        &mut recepes,
        vec![Ingredient {
            name: "petroleum_gas".to_string(),
            amount: 30.0,
        }]
        .into(),
        "sulfur".to_string(),
        2.0,
        30.0,
    );

    dbg!(&recepes);

    let data = r#"
    [
    {
        "name": "oil",
        "ingredients": null,
        "craft_time": 0.0,
        "craft_amount": 0.0

    },
    {
        "name": "pee",
        "ingredients": [
        {
            "name": "oil",
            "amount": 2.0
        },
        {
            "name": "oils",
            "amount": 2.0
        }
        ],
        "craft_time": 0.1,
        "craft_amount": 0.2
    }
    ]
    "#;

    let test_rec: Vec<Recipe> = match serde_json::from_str(data) {
        Ok(json) => json,
        Err(error) => panic!("ERROR: {error}"),
    };

    dbg!(&recepes);
    */

    /*
    let ingr_output = get_ingr(
        &recepes,
        vec![Ingredient {
            name: String::from("fads"),
            amount: 0.0,
        }],
    );
    */

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
}
