use std::collections::HashMap;

fn main() {
    // Creating a new empty HashMap
    let mut student_grades = HashMap::new();

    // Inserting data: keys are student names, values are grades
    student_grades.insert("Alice", 85);
    student_grades.insert("Bob", 78);

    // Printing the HashMap
    println!("{:?}", student_grades);
}