/*
An enum is like a list of possible options that a value can be. 
Each option is called a "variant."
*/

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    // more variants ...
}

fn main() {
    let dir1 = Direction::North;
    let dir2 = Direction::South;

    println!("{:?}", dir1);
    println!("{:?}", dir2);
}