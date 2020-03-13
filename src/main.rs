extern crate reqwest;
extern crate clap;
extern crate console;
extern crate serde;

use clap::{Arg, App};
use console::style;
use serde::{Deserialize};

#[derive(Deserialize)]
struct Pokemon {
    id: u64,
    name: String,
    types: Vec<TypeSlot>,
    height: u64,
    weight: u64,
}

#[derive(Deserialize)]
struct TypeSlot {
    slot: u64, 
    #[serde(rename = "type")]
    type_object: Type,
}

#[derive(Deserialize, Debug)]
struct Type {
    name: String,
}

#[derive(Deserialize, Default, Debug)]
struct TypeEffectiveness {
    #[serde(default)]
    no_damage_to: Vec<Type>,
    #[serde(default)]
    no_damage_from: Vec<Type>,
    #[serde(default)]
    double_damage_to: Vec<Type>,
    #[serde(default)]
    double_damage_from: Vec<Type>,
    #[serde(default)]
    half_damage_to: Vec<Type>,
    #[serde(default)]
    half_damage_from: Vec<Type>,
}

#[derive(Deserialize, Debug)]
struct TypeFull {
    damage_relations: TypeEffectiveness,
}


fn main() {
    println!("{}", style("Pokedex CLI").bold().magenta());

    let matches = App::new("Pokedex")
        .version("0.1.1")
        .author("Ari Vaniderstine <ari.vaniderstine@embark-studios.com>")
        .about("Ari's first CLI")
        .arg(Arg::with_name("Pokemon Name")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Name of pokemon"))
        .get_matches();
    let input_name = matches.value_of("Pokemon Name").unwrap();

    get_pokemon(input_name);
}

fn get_pokemon(name: &str) {
    match make_request(name) {
        Err(e) => handle_error(e),
        Ok(pokemon)  => {
            print_pokemon(pokemon);
        }
    }
}

fn make_request(pokemon: &str) -> Result<Pokemon, reqwest::Error> {
    let uri = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    reqwest::get(&uri)?.json()
}

fn print_pokemon(p: Pokemon) {
    println!("ID: {}", style(&p.id).cyan());
    println!("Name: {}", style(&p.name).magenta());
    println!("Height: {}m", p.height as f32/10.0);
    println!("Weight: {}kg", p.weight as f32/10.0);

    let ptypes = get_pokemon_types(p.types);
    print_types(ptypes);
}

fn print_types(ptypes: Vec<String>) {
    let msg = format!("Types: {}", style(ptypes.join(", ")).yellow());
    println!("{}", msg);
    match get_type_effectiveness(ptypes) {
        Err(e) => handle_error(e),
        Ok(res)  => {
            // TODO: pretty print for type effectiveness
            println!("{}", style("\nType Effectiveness").bold());
            print!("2x damage from: ");
            let msg = format!("{}", style(types_to_string(res.double_damage_from)).yellow());
            println!("{}", msg);
            print!("2x damage to: ");
            let msg = format!("{}", style(types_to_string(res.double_damage_to)).yellow());
            println!("{}", msg);
            print!("1/2 damage from: ");
            let msg = format!("{}", style(types_to_string(res.half_damage_from)).yellow());
            println!("{}", msg);
            print!("1/2 damage to: ");
            let msg = format!("{}", style(types_to_string(res.half_damage_to)).yellow());
            println!("{}", msg);
            print!("0 damage from: ");
            let msg = format!("{}", style(types_to_string(res.no_damage_from)).yellow());
            println!("{}", msg);
            print!("0 damage to: ");
            let msg = format!("{}", style(types_to_string(res.no_damage_to)).yellow());
            println!("{}", msg);
        }
    }

}

fn get_pokemon_types(type_array: Vec<TypeSlot>) -> Vec<String> {
    let mut types = Vec::new();
    let mut temp = type_array;
    temp.sort_by(|a, b| a.slot.cmp(&b.slot));
    for t in temp {
        let type_name = t.type_object.name.to_string();
        types.push(type_name);
    }
    types
}

fn types_to_string(type_array: Vec<Type>) -> String {
    let mut array = Vec::new();
    for t in type_array {
        let type_name = t.name.to_string();
        array.push(type_name);
    }
    array.join(", ")
}

fn get_type_effectiveness(types: Vec<String>) -> Result<TypeEffectiveness, reqwest::Error> {
    let mut resp: TypeEffectiveness = TypeEffectiveness::default();
    for t in types {
        let uri = format!("https://pokeapi.co/api/v2/type/{}", t);
        //TODO: Calculate type effectiveness for Pokemon with multiple types
        let res: TypeFull = reqwest::get(&uri)?.json()?;
        resp = res.damage_relations;
        //println!("{:#?}", resp);
    }
    Ok(resp)
}

fn handle_error(e: reqwest::Error) {
    println!("{}", e);
}
