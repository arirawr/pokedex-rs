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
    url: String,
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

    match make_request(input_name) {
        Err(e) => handle_error(e),
        Ok(response)  => {
            let mut data = response;
            print_info(&mut data);
        }
    }
}

fn make_request(pokemon: &str) -> Result<Pokemon, reqwest::Error> {
    let uri = format!("{}{}","https://pokeapi.co/api/v2/pokemon/", pokemon);
    let resp: Pokemon = reqwest::get(&uri)?.json()?;
    Ok(resp)
}

fn print_info(p: &mut Pokemon) {
    println!("ID: {}", style(&p.id).cyan());
    println!("Name: {}", style(&p.name).magenta());
    println!("Types: {}", style(get_types(&mut p.types)).magenta());
    println!("Height: {}m", p.height as f32/10.0);
    println!("Weight: {}kg", p.weight as f32/10.0);
}

fn get_types(type_array: &mut Vec<TypeSlot>) -> String {
    let mut type_string = String::new();
    type_array.sort_by(|a, b| a.slot.cmp(&b.slot));
    for t in type_array {
        if t.slot > 1 { type_string.push_str(", ") }
        type_string.push_str(&t.type_object.name);
    }
    return type_string;
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
