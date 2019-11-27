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

    make_request(input_name);
}

fn make_request(pokemon: &str) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("{}{}","https://pokeapi.co/api/v2/pokemon/", pokemon);
    let resp: serde_json::Value = reqwest::get(&uri)?.json()?;
    println!("{:#?}", resp);
    Ok(())
}
