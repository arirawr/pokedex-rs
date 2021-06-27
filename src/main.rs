#![allow(dead_code)]
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
//extern crate levenshtein;
use levenshtein::levenshtein;
use std::convert::TryInto;
use std::collections::HashSet;

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
struct Type(String);

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

    let mut input_name: String = matches.value_of("Pokemon Name").unwrap().into();
    static dex_json: &'static str = include_str!("pokemon-dex.json");
    
    let pokedex: Vec<PokedexEntry> = serde_json::from_str(&dex_json)?; 
    unsafe {
        let mut max_dist: usize = 4 as usize;
        let mut stack_name = &mut input_name;
        for entry in &pokedex {
            let mut dist = levenshtein(&entry.name, &stack_name);
            //println!("Lev Dist: &entry.name, &stack_name = {:?}", &dist);
            //if dist as u32 == 1 {
            //    println!("Matched: {}", &entry.name);
            //}
            if &dist < &max_dist {
                *stack_name = entry.name.clone();
                max_dist = dist;
            }
        }
        //println!("{:?}", &stack_name);
        let entry: Option<PokedexEntry> = pokedex
            .into_iter()
            .filter(|entry| &entry.name == stack_name)
            .next();

        print_pokemon(entry.unwrap());
    }

    /*let entry: Option<PokedexEntry> = pokedex.into_iter().filter(|entry| &entry.name == &input_name).next();
    match entry {
        Some(pokemon) => {
            print_pokemon(pokemon);
        },
        None => {
            let dirty_dup: Vec<PokedexEntry> = serde_json::from_str(&dex_json)?;
            let names: Vec<String> = dirty_dup.into_iter().map(|entry| entry.name).collect();
           
            eprintln!("Sorry, couldn't find '{}'", &input_name);
        }
    }*/
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
    tms: Vec<usize>,
    trs: Vec<usize>,
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
    println!("TRs:");
    pretty_print_trs(&p.trs);
    println!("Evolutions: {:?}", &p.evolutions);
    println!("Description: {:?}", &p.description);
    println!("Catch rate: {:?}", &p.catch_rate);
    //let ptypes = get_pokemon_types(p.types);
    //print_types(ptypes);
}
//"tr_no": 0, "tr_name": "Swords Dance", "tr_type": "Normal", "tr_effects": "Raises Attack by 2 stages.", "tr_damage": null
#[derive(Debug, Deserialize)]
struct Tr {
    no: usize,
    name: String,
    #[serde(rename = "type")]
    type_: String,
    effects: Option<String>,
    damage: Option<f32>
}

impl Tr {
    pub fn load() -> Result<Vec<Self>, serde_json::error::Error> {
        static tr_data: &'static str = include_str!("tr_data.json");
        let trs: Vec<Self> = serde_json::from_str(&tr_data)?;
        Ok(trs)
    }
}

fn pretty_print_trs(entry_trs: &Vec<usize>) -> Result<(), serde_json::error::Error> {
    println!("Called prtty");
    //let tr_data: &str = include_str!("tr_data.json");
    //println!("{:?}", &tr_data);
    //let mut redux: HashSet<usize> = HashSet::new();
    //for entry_tr in entry_trs {
    //    redux.insert(*entry_tr);
    //}
    //let trs: Vec<Tr> = serde_json::from_str(&tr_data)?;
    //println!("&trs = {:?}", &trs);
    /*let trs: Vec<Tr> = trs
        .into_iter()
        .filter(|idx| { *&redux.contains(&idx.no) })
        .collect();
    println!("&trs = {:?}", &trs);
    for tr in trs {
        println!("{:?}", &tr);
    }
    */
    match Tr::load() {
        Ok(vec_trs) => {
            for tr_no in entry_trs {
                println!("{:?}", vec_trs.index(*tr_no));
            }     
        }
        Err(e) => { println!("{:?}" , &e); }
    }
    Ok(())
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
        let type_name = t.type_object.0.to_string();
        types.push(type_name);
    }
    types
}

fn types_to_string(type_array: Vec<Type>) -> String {
    let mut array = Vec::new();
    for t in type_array {
        let type_name = t.0.to_string();
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

pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;

    fn index(&self, index: Idx) -> &Self::Output;
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &(**self)[index]
    }
}
