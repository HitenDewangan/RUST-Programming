use std::collections::HashMap;

fn main() {
    // Create a HashMap
    let mut student_grades: HashMap<String, i32> = HashMap::new();
    student_grades.insert(String::from("Alice"), 85);

    // Move ownership to new_student_grades
    let new_student_grades: HashMap<String, i32> = student_grades;

    println!("{:?}", new_student_grades);

    // -------------------------------------------------

    // Create a HashMap
    let mut student_grades: HashMap<String, i32> = HashMap::new();
    student_grades.insert(String::from("Alice"), 85);

    // Borrow an immutable reference to the HashMap
    let borrowed_grades: &HashMap<String, i32> = &student_grades;

    // student_grades.insert(String::from("Bob"), 90); // Error: student_grades is borrowed

    println!("Borrowed grades: {:?}", borrowed_grades.get("Alice"));
    println!("Original grades: {:?}", student_grades.get("Alice"));
    

    // -------------------------------------------------

    // Borrow a mutable reference to modify the HashMap
    let borrowed_grades: &mut HashMap<String, i32> = &mut student_grades;

    // Insert a new entry using the mutable reference
    borrowed_grades.insert(String::from("Bob"), 90);

    // The original HashMap is also updated because borrowed_grades is a reference
    println!("Modified grades: {:?}", borrowed_grades);

    println!("Original grades: {:?}", student_grades); // Same as borrowed_grades
}