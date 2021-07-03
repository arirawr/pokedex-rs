use serde::{Deserialize};
use serde_json;
use std::convert::TryFrom;
use std::fmt::Display;
use serde_repr::*;

#[derive(Deserialize, Debug)]
pub struct Move { 
    name:      String,
    move_id:   MoveId,
    available: bool,
    effects:   String,

    #[serde(rename = "type")]
    ty: Ty,
    tr: Tr,
    tm: Tm,

    category: usize,
    power:    u32,
    pp:       u32,
    priority: i32,
    target:   MoveTargets
}

#[derive(Deserialize, Debug)]
pub struct MoveId(usize);
/*
#[derive(Deserialize, Debug)]
pub struct Tr(Option<usize>);

#[derive(Deserialize, Debug)]
pub struct TrNo(usize);

#[derive(Deserialize, Debug)]
pub struct Tm(Option<usize>);

#[derive(Deserialize, Debug)]
pub struct TmNo(usize);
*/
#[derive(Deserialize, Debug)]
pub struct Tr(Option<TrNo>);

#[derive(Deserialize, Debug)]
pub struct TrNo(usize);

#[derive(Deserialize, Debug)]
pub struct Tm(Option<TmNo>);

#[derive(Deserialize, Debug)]
pub struct TmNo(usize);



impl Move {
    pub fn load_all() -> Result<Vec<Self>, serde_json::error::Error> {
        static mv_data: &'static str = include_str!("../new-moves.json");
        let moves: Vec<Self> = serde_json::from_str(&mv_data)?;
        Ok(moves)
    }
}
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(usize)]
pub enum Ty {
    Normal        = 0  as usize,
    Fighting      = 1  as usize,
    Flying        = 2  as usize,
    Poison        = 3  as usize,
    Ground        = 4  as usize,
    Rock          = 5  as usize,
    Bug           = 6  as usize,
    Ghost         = 7  as usize,
    Psychic       = 8  as usize,
    Steel         = 9  as usize,
    Fire          = 10 as usize,
    Water         = 11 as usize,
    Grass         = 12 as usize,
    Electric      = 13 as usize,
    Ice           = 14 as usize,
    Dragon        = 15 as usize,
    Dark          = 16 as usize,
    Fairy         = 17 as usize
} 

impl Display for Ty {

    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            &Ty::Normal   => { write!(f, "Normal") },
            &Ty::Fighting => { write!(f, "Fighting") },
            &Ty::Flying   => { write!(f, "Flying") },
            &Ty::Poison   => { write!(f, "Poison") },
            &Ty::Ground   => { write!(f, "Ground") },
            &Ty::Rock     => { write!(f, "Rock") },
            &Ty::Bug      => { write!(f, "Bug") },
            &Ty::Ghost    => { write!(f, "Ghost") },
            &Ty::Psychic  => { write!(f, "Psychic") },
            &Ty::Steel    => { write!(f, "Steel") },
            &Ty::Fire     => { write!(f, "Fire") },
            &Ty::Water    => { write!(f, "Water") },
            &Ty::Grass    => { write!(f, "Grass") },
            &Ty::Electric => { write!(f, "Electric") },
            &Ty::Ice      => { write!(f, "Ice") },
            &Ty::Dragon   => { write!(f, "Dragon") },
            &Ty::Dark     => { write!(f, "Dark") },
            &Ty::Fairy    => { write!(f, "Fairy") }
        }
    }
}

impl TryFrom<usize> for Ty {
    type Error = &'static str;

    fn try_from(ty_id: usize) -> Result<Ty, Self::Error> {
        match ty_id {
            0   => { Ok(Ty::Normal) },
            1   => { Ok(Ty::Fighting) },
            2   => { Ok(Ty::Flying) },
            3   => { Ok(Ty::Poison) },
            4   => { Ok(Ty::Ground) },
            5   => { Ok(Ty::Rock) },
            6   => { Ok(Ty::Bug) },
            7   => { Ok(Ty::Ghost) },
            8   => { Ok(Ty::Psychic) },
            9   => { Ok(Ty::Steel) },
            10  => { Ok(Ty::Fire) },
            11  => { Ok(Ty::Water) },
            12  => { Ok(Ty::Grass) },
            13  => { Ok(Ty::Electric) },
            14  => { Ok(Ty::Ice) },
            15  => { Ok(Ty::Dragon) },
            16  => { Ok(Ty::Dark) },
            17  => { Ok(Ty::Fairy) },
            _ => { Err("A type mapping does not exist for the supplied value.") }
        }
    }
}
#[derive(Deserialize, Debug)]
pub enum MoveTargets {
    All                 , // = "All",
    AllAdjacent         , // = "AllAdjacent",
    AllAdjacentOpponents, // = "AllAdjacentOpponents",
    AllAllies           , // = "AllAllies",
    Ally                , // = "Ally",
    AllyOrSelf          , // = "AllyOrSelf",
    AnyExceptSelf       , // = "AnyExceptSelf",
    Counter             , // = "Counter",
    Opponent            , // = "Opponent",
    RandomOpponent      , // = "RandomOpponent",
    
    #[serde(rename = "Self")]
    Self_       , // = "Self",
    SideAll     , // = "SideAll",
    SideOpponent, // = "SideOpponent",
    SideSelf    , // = "SideSelf"
}
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
    level_up_moves: Vec<(u32, MoveId)>,
    //egg_moves: Vec<usize>,
    egg_moves: Vec<MoveId>,
    //tms: Vec<usize>,
    //trs: Vec<usize>,
    tms: Vec<Tm>,
    trs: Vec<Tr>,
    evolutions: Vec<serde_json::Map<String, serde_json::Value>>,
    description: Option<String>,
    catch_rate: Option<u32>
}
//#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
//#[repr(String)]
/*
pub enum MoveTargets {
    All                  = "All",
    AllAdjacent          = "AllAdjacent",
    AllAdjacentOpponents = "AllAdjacentOpponents",
    AllAllies            = "AllAllies",
    Ally                 = "Ally",
    AllyOrSelf           = "AllyOrSelf",
    AnyExceptSelf        = "AnyExceptSelf",
    Counter              = "Counter",
    Opponent             = "Opponent",
    RandomOpponent       = "RandomOpponent",
    
    #[serde(rename = "Self")]
    Self_        = "Self",
    SideAll      = "SideAll",
    SideOpponent = "SideOpponent",
    SideSelf     = "SideSelf"
}
*/
