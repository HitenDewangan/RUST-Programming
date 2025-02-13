fn main() {
    let mut v: Vec<i32> = Vec::new(); // creates an empty vector

    v.push(1);
    v.push(2);
    v.push(3);
    
    v.pop();
    v.insert(2, 3);

    println!("{:?}", v); // Output: [1, 2, 3]

    let mut v = vec![1, 2, 3];
    println!("{:?}", v); // Output: [1, 2, 3]

    
    let third: &i32 = &v[2]; // reference to the third element in the vector
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }


    for i in &mut v { 
        println!("{}", i); // Output: 1, 2, 3
    }
    for i in v.iter() { 
        println!("{}", i); // Output: 1, 2, 3
    }
    for i in &mut v {
        *i += 1;
    }

    println!("{:?}", v); // Output: [2, 3, 4]

    // common methods on vectors
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Length: {}", v.len()); // Output: 5
    println!("Is empty?: {}", v.is_empty()); // Output: false
    println!("Does the vector contain 2?: {}", v.contains(&2)); // Output: false
    
    let removed = v.remove(2);
    println!("Removed value: {}", removed);
    println!("Vector after removal: {:?}", v);

    v.extend(vec![6, 7, 8]);
    println!("Vector after extension: {:?}", v);

}