

Enums are often referred to as algebraic data types and list all variations of some data.


Example:

enum CardinalDirections{
    North,
    South,
    East,
    West
}

fn main{
    let north = CardinalDirections:: North;
    let south = CardinalDirections:: South;
    let east = CardinalDirections:: East;
    let west = CardinalDirections:: West;
}