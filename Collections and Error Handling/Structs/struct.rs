struct Rectange {
    width: u32,
    height: u32,
}

fn main() {
    let r1 = Rectange { 
        width: 30, 
        height: 50 
    };

    println!("r1 Width: {}", r1.width);
    println!("r1 Height: {}", r1.height);

    println!("-----------------");

    let width = 10;
    let height = 20;
    let r2 = Rectange { 
        width, 
        height 
    };

    println!("r2 Width: {}", r2.width);
    println!("r2 Height: {}", r2.height);

    println!("-----------------");


    let r3 = Rectange{
        height: 70,
        ..r1  // take the remaining field from r1
    };

    println!("r3 Width: {}", r3.width);
    println!("r3 Height: {}", r3.height);
}