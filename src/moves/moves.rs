use serde::{Deserialize};
use serde_json;
use std::convert::TryFrom;
use std::fmt::Display;
use serde_repr::*;

#[derive(Deserialize)]
pub struct Move { 
    name:      String,
    move_id:   usize,
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

#[derive(Deserialize)]
pub struct Tr(Option<usize>);

#[derive(Deserialize)]
pub struct Tm(Option<usize>);

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

#[derive(Deserialize)]
pub enum MoveTargets {
    All,
    AllAdjacent,
    AllAdjacentOpponents,
    AllAllies,
    Ally,
    AllyOrSelf,
    AnyExceptSelf,
    Counter,
    Opponent,
    RandomOpponent,
    #[serde(rename = "Self")]
    Self_,
    SideAll,
    SideOpponent,
    SideSelf
}
