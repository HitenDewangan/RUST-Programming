fn main(){
    let  mut number = 3;

    while number != 0 {
        println!("{}!", number);
        
        number -= 1;
    }
    println!("Liftoff!");

    println!("------------------");

    let mut number = 4;
    let limit = 10;
    
    while number < limit && number %2 == 0 {
        println!("The number is: {}!", number);
        
        number += 2;
    }

    println!("Loop terminated with number: {}", number);
}