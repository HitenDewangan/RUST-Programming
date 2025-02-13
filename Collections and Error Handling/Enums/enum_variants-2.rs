// Enums Variants with data

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

fn sound(animal: Animal) -> &'static str {
    match animal {
        Animal::Dog(_) => "Bark",
        Animal::Cat(_) => "Meow",
        Animal::Bird(_) => "Tweet",
    }
}

fn main() {
    let pet1 = Animal::Dog(String::from("Buddy"));
    let pet2 = Animal::Cat(String::from("Whiskers"));
    let pet3 = Animal::Bird(String::from("Tweety"));

    println!("Pet1 makes a sound: {}", sound(pet1));
    println!("Pet2 makes a sound: {}", sound(pet2));
    println!("Pet3 makes a sound: {}", sound(pet3));
}