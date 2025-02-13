struct Owner {
    name: String,
}

struct House {
    owner: Owner,
    rooms: u32,
}

fn main() {
    let owner1 = Owner{
        name: String::from("John Doe"),
    };

    let house1 = House {
        owner: owner1, // Move ownership from owner1 to house1
        rooms: 3,
    };

    // println!("Owner: {}", owner1.name); // Error: owner1 is no longer valid

    println!("House owner: {}", house1.owner.name);
    println!("House rooms: {}", house1.rooms);
}