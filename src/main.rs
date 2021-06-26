//#![feature(allocator_api)]
//extern crate reqwest;
//extern crate clap;
//extern crate console;
//extern crate serde;
//use std::include_str;
use clap::{Arg, App};
use console::style;
use serde::{Deserialize};
use serde_json;
#[macro_use]
extern crate lazy_static;

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

fn main() -> Result<(), serde_json::error::Error> {
    println!("{}", style("Pokedex CLI").bold().magenta());

    let matches = App::new("Pokedex")
        .version("0.2.0")
        .author("Ari Vaniderstine <ari.vaniderstine@embark-studios.com>; David Golembiowski <david[āŧ]dgolembiowski[doŧ]com>")
        .about("Pokedex CLI built with Rust")
        .arg(Arg::with_name("Pokemon Name")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Name of pokemon"))
        .get_matches();
    println!("{:?}", &matches);
    let input_name: String = matches.value_of("Pokemon Name").unwrap().into();
    let dex_json = include_str!("pokemon-dex.json");
    let pokedex: Vec<PokedexEntry> = serde_json::from_str(&dex_json)?;
    //println!("{:#?}", &pokedex);
    let entry: PokedexEntry = pokedex.into_iter().filter(|entry| &entry.name == &input_name).next().unwrap();
    print_pokemon(entry);
    Ok(())
}



#[derive(Debug, Deserialize)]
struct PokedexEntry {
    id: u32,
    name: String,
    stage: u32,
    galar_dex: Option<String>,
    base_stats: [u32; 6],
    ev_yield: [u32; 6],
    abilities: Vec<String>,
    types:  Vec<String>,//items":[["None",50],["Silver Powder",5],["None",1]],
    items: serde_json::Value, //Vec<Option<[Vec<String, u32>;1]>>, //String>,
    exp_group: String,
    egg_groups: Vec<String>,
    hatch_cycles: Option<u32>,
    height: f32,
    weight: f32,
    color: String, 
    level_up_moves: Vec<(u32, String)>,
    egg_moves: Vec<String>,
    tms: Vec<u32>,
    trs: Vec<u32>,
    evolutions: Vec<serde_json::Map<String, serde_json::Value>>,
    description: Option<String>,
    catch_rate: Option<u32>
}
/*
fn get_pokemon(name: &str) {
    match make_request(name) {
        Err(e) => handle_error(e),
        Ok(pokemon)  => {
            print_pokemon(pokemon);
        }
    }
}
*/
fn make_request(pokemon: &str) -> Result<Pokemon, reqwest::Error> {
    let uri = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    reqwest::get(&uri)?.json()
}

//fn print_pokemon(p: Pokemon) {
fn print_pokemon(p: PokedexEntry) {
    println!("ID: {:?}", style(&p.id).cyan());
    println!("Name: {:?}", style(&p.name).magenta());
    println!("Height: {:?}m", p.height as f32/10.0);
    println!("Weight: {:?}kg", p.weight as f32/10.0);
    println!("Stage: {:?}", &p.stage);
    println!("Galar dex: {:?}", &p.galar_dex);
    println!("Base stats: {:?}", &p.base_stats);
    println!("Ev yield: {:?}", &p.ev_yield);
    println!("Abilities: {:?}", &p.abilities);
    println!("Types: {:?}", &p.types);
    println!("Items: {:?}", &p.items);
    println!("Exp group: {:?}", &p.exp_group);
    println!("Egg groups: {:?}", &p.egg_groups);
    println!("Hatch cycles: {:?}", &p.hatch_cycles);
    println!("Color: {:?}", &p.color);
    println!("Level up moves: {:?}", &p.level_up_moves);
    println!("Egg moves: {:?}", &p.egg_moves);
    println!("Tms: {:?}", &p.tms);
    println!("Trs: {:?}", &p.trs);
    println!("Evolutions: {:?}", &p.evolutions);
    println!("Description: {:?}", &p.description);
    println!("Catch rate: {:?}", &p.catch_rate);
    //let ptypes = get_pokemon_types(p.types);
    //print_types(ptypes);
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
