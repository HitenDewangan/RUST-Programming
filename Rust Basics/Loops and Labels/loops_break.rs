fn main() {
    let mut outer_count = 0;

    'outer: loop {
        println!("Outer count: {}", outer_count);
        let mut inner_count = 0;

        loop {
            println!("Inner count: {}", inner_count);
            inner_count += 1;

            if inner_count == 2 {
                break;
            }

            if outer_count == 3 {
                break 'outer;
            }
        }
        println!("------------------------");
        outer_count += 1;
    }

    println!("Outer loop ended at count: {}", outer_count);
}