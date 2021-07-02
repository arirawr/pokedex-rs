use serde::{Deserialize};
use serde_json;
use std::convert::TryFrom;
use std::fmt::Display;
use serde_repr::*;

#[derive(Deserialize, Debug)]
pub struct Move { 
    pub name:      String,
    pub move_id:   MoveId,
    pub available: bool,
    pub effects:   String,

    #[serde(rename = "type")]
    pub ty: Ty,
    pub tr: Tr,
    pub tm: Tm,

    pub category: usize,
    pub power:    u32,
    pub pp:       u32,
    pub priority: i32,
    pub target:   MoveTargets
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
    pub id: u32,
    pub name: String,
    pub stage: u32,
    pub galar_dex: Option<String>,
    pub base_stats: [u32; 6],
    pub ev_yield: [u32; 6],
    pub abilities: Vec<String>,
    pub types:  Vec<String>,//items":[["None",50],["Silver Powder",5],["None",1]],
    pub items: serde_json::Value, //Vec<Option<[Vec<String, u32>;1]>>, //String>,
    pub exp_group: String,
    pub egg_groups: Vec<String>,
    pub hatch_cycles: Option<u32>,
    pub height: f32,
    pub weight: f32,
    pub color: String, 
    //level_up_moves: Vec<(u32, usize)>,
    pub level_up_moves: Vec<(u32, MoveId)>,
    //egg_moves: Vec<usize>,
    pub egg_moves: Vec<MoveId>,
    //tms: Vec<usize>,
    //trs: Vec<usize>,
    pub tms: Vec<Tm>,
    pub trs: Vec<Tr>,
    pub evolutions: Vec<serde_json::Map<String, serde_json::Value>>,
    pub description: Option<String>,
    pub catch_rate: Option<u32>
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
