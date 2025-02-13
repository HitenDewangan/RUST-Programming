struct Rectange {
    width: u32,
    height: u32,
}

fn main() {
    let r1 = Rectange { 
        width: 30, 
        height: 50 
    };

    let r2 = r1; // Move ownership from r1 to r2

    // println!("r1 Width: {}", r1.width); // Error: r1 is no longer valid
    println!("r2 Width: {}", r2.width);
    println!("r2 Height: {}", r2.height);
}