extern crate reqwest;
extern crate clap;

use clap::{Arg, App};

fn main() {
    println!("Hello, world!");

    let matches = App::new("Pokedex")
        .version("0.1.0")
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
        Ok(_)  => return,
    }
}

fn make_request(pokemon: &str) -> Result<(), reqwest::Error> {
    let uri = format!("{}{}","https://pokeapi.co/api/v2/pokemon/", pokemon);
    let resp: serde_json::Value = reqwest::get(&uri)?.json()?;
    Ok(())
}
