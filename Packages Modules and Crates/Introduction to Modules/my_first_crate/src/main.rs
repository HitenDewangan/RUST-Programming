// INLINE MODULE

mod something {
    fn private_function() {
        println!("This function is private to the math module.");
    }

    pub fn public_function() {
        println!("This function is public and can be accessed from outside.");
    }
}

fn main() {
    // something::private_function(); // Error: private function, not accessible here
    something::public_function(); // This works because the function is public
}

// -----------------------------------------------------

// mod math;  // importing the module

// fn main() {
//     let sum: i32 = math::add(5, 3);
//     let difference: i32 = math::subtract(5, 3);

//     println!("Sum: {}, Difference: {}", sum, difference);
// }