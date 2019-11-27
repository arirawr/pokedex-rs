extern crate reqwest;
extern crate clap;
extern crate console;

use clap::{Arg, App};
use console::style;

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
        Err(e) => { 
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
        },
        Ok(response)  => {
            let mut data = response;
            print_info(&mut data);
        }
    }
}

fn make_request(pokemon: &str) -> Result<serde_json::Value, reqwest::Error> {
    let uri = format!("{}{}","https://pokeapi.co/api/v2/pokemon/", pokemon);
    let mut resp: serde_json::Value = reqwest::get(&uri)?.json()?;
    Ok(resp)
}

fn print_info(json: &mut serde_json::Value) {
    println!("ID: {}", style(json["id"].as_u64().unwrap()).cyan());
    println!("Name: {}", style(json["name"].as_str().unwrap()).magenta());
    println!("Types: {}", style(get_types(&mut json["types"])).magenta());
}

fn get_types(type_array: &mut serde_json::Value) -> String {
    if type_array[1].is_object() {
        return format!("{}, {}", type_array[0]["type"]["name"].as_str().unwrap(), type_array[1]["type"]["name"].as_str().unwrap());
    }
    else {
        return type_array[0]["type"]["name"].as_str().unwrap().to_string();
    }
}
