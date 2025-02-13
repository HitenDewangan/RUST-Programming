struct Rectange {
    width: u32, 
    height: u32
}

fn main() {
    let r1 = Rectange { 
        width: 30, 
        height: 50 
    };

    print_dimensions(&r1); // Borrow r1

    // r1 can still be used here
    println!("r1 Width: {}", r1.width);
}

fn print_dimensions(rectangle: &Rectange) {
    println!("The rectangle is {} x {} pixels.", rectangle.width, rectangle.height);
}