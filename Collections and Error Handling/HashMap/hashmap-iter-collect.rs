use std::collections::HashMap;

fn main() {
    // Vectors of names and grades
    let names = vec!["Alice", "Bob", "Charlie"];
    let grades = vec![85, 78, 92];

    // Creating a HashMap using the collect() method
    let mut student_grades: HashMap<&str, i32> = 
        names.into_iter().zip(grades.into_iter()).collect();

    println!("{:?}", student_grades);

    student_grades.insert("Alice", 95);
    println!("{:?}", student_grades);

    // Accessing the value using `get()`
    let grade = student_grades.get("Alice");

    match grade {
        Some(g) => println!("Alice's grade is {}", g),
        None => println!("No grade found for Alice"),
    }

    // Accessing the value using indexing
    let grade = student_grades["Alice"]; // but this will panic if the key is not found
    println!("Alice's grade is {}", grade);

    // Updating Values in HAshMap -> insert()/entry()
    student_grades.insert("Alice", 95);

    let entry = student_grades.entry("Alice").or_insert(0);
    *entry += 10;
    println!("Alice's new grade is {}", student_grades["Alice"]);

    // Remove Elements
    student_grades.remove("Alice");
    println!("{:?}", student_grades);

    // Iterating through HashMap
    for (name, grade) in &student_grades {
        println!("{}: {}", name, grade);
    }

    // Common HashMap Methods
    /*
        len() : Returns the number of key-value pairs in the HashMap.
        is_empty() : Returns true if the HashMap is empty.
        remove() : Removes the key-value pair associated with the given key.
        contains_key() : Returns true if the HashMap contains the given key.
        
        get() : Returns a reference to the value associated with the given key.
        insert() : Inserts a key-value pair into the HashMap.
        entry() : Returns a mutable reference to the value associated with the given key.
        
        keys() : Returns an iterator over the keys of the HashMap.
        values() : Returns an iterator over the values of the HashMap.
        
        clear() : Removes all key-value pairs from the HashMap.
        get_or_insert() : Returns a mutable reference to the value associated with the given key, or inserts the default value if the key is not present in the HashMap.
        get_or_insert_with() : Returns a mutable reference to the value associated with the given key, or inserts the result of the closure if the key is not present in the HashMap.

     */
}