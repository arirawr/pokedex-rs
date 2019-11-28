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

#[derive(Deserialize)]
struct Type {
    name: String,
}

#[derive(Deserialize, Default)]
struct TypeEffectiveness {
    no_damage_to: Option<Vec<Type>>,
    no_damage_from: Option<Vec<Type>>,
    double_damage_to: Option<Vec<Type>>,
    double_damage_from: Option<Vec<Type>>,
    half_damage_to: Option<Vec<Type>>,
    half_damage_from: Option<Vec<Type>>,
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
            let mut data = pokemon;
            print_pokemon(&mut data);
        }
    }
}

fn make_request(pokemon: &str) -> Result<Pokemon, reqwest::Error> {
    let uri = format!("{}{}","https://pokeapi.co/api/v2/pokemon/", pokemon);
    let resp: Pokemon = reqwest::get(&uri)?.json()?;
    Ok(resp)
}

fn print_pokemon(p: &mut Pokemon) {
    println!("ID: {}", style(&p.id).cyan());
    println!("Name: {}", style(&p.name).magenta());
    println!("Height: {}m", p.height as f32/10.0);
    println!("Weight: {}kg", p.weight as f32/10.0);

    let ptypes = get_pokemon_types(&mut p.types);
    print_types(ptypes);
}

fn print_types(ptypes: Vec<String>) {
    let msg = format!("Types: {}", style(ptypes.join(", ")).yellow());
    println!("{}", msg);
    match get_type_effectiveness(ptypes) {
        Err(e) => handle_error(e),
        Ok(res)  => {
            if res.double_damage_from.is_some() {
                print!("Double damage from: ");
                for t in res.double_damage_from {
                    //print!("{}", t["name"]);
                }
            }
        }
    }

}

fn get_pokemon_types(type_array: &mut Vec<TypeSlot>) -> Vec<String> {
    let mut types = Vec::new();
    let temp = type_array;
    temp.sort_by(|a, b| a.slot.cmp(&b.slot));
    for t in temp {
        let type_name = t.type_object.name.to_string();
        types.push(type_name);
    }
    return types;
}

fn get_type_effectiveness(types: Vec<String>) -> Result<TypeEffectiveness, reqwest::Error> {
    let mut resp: TypeEffectiveness = TypeEffectiveness::default();
    for t in types {
        let uri = format!("https://pokeapi.co/api/v2/type/{}", t);
        resp = reqwest::get(&uri)?.json()?;
    }
    Ok(resp)
}

fn handle_error(e: reqwest::Error) {
    if e.is_http() {
        match e.url() {
            None => println!("No Url given"),
            Some(url) => println!("Problem making request to: {}", url),
        }
    }
    // Inspect the internal error and output it
    if e.is_serialization() {
    let serde_error = match e.get_ref() {
            None => return,
            Some(err) => err,
        };
        println!("problem parsing information {}", serde_error);
    }
    if e.is_redirect() {
        println!("server redirecting too many times or making loop");
    }
}
