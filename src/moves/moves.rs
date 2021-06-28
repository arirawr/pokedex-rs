
pub enum MoveTypes {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Psychic,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Dragon,
    Dark,
    Fairy
} 

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
    Self_,
    SideAll,
    SideOpponent,
    SideSelf
}
