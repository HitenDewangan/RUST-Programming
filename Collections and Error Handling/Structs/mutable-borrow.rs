struct Rectange {
    width: u32, 
    height: u32
}

fn main() {
    let mut r1 = Rectange { 
        width: 30, 
        height: 50 
    };

    modify_dimensions(&mut r1); // Mutable borrow

    // r1 can still be used here
    println!("Modified Width: {}", r1.width);
    println!("Modified Height: {}", r1.height);
}

fn modify_dimensions(rect: &mut Rectange) {
    rect.width = 40;
    rect.height = 60;
}