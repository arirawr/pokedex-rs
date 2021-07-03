/*use serde::{Deserialize};
use serde_json;
use std::convert::TryFrom;
use std::fmt::Display;
use serde_repr::*;
mod moves;
//mod super::moves;
#[derive(Debug, Deserialize)]
pub struct PokedexEntry {
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
    //level_up_moves: Vec<(u32, usize)>,
    level_up_moves: Vec<(u32, moves::MoveId)>,
    //egg_moves: Vec<usize>,
    egg_moves: Vec<moves::MoveId>,
    //tms: Vec<usize>,
    //trs: Vec<usize>,
    tms: Vec<moves::Tm>,
    trs: Vec<moves::Tr>,
    evolutions: Vec<serde_json::Map<String, serde_json::Value>>,
    description: Option<String>,
    catch_rate: Option<u32>
}*/
