// super keyword

mod parent {
    pub mod child {
        pub fn child_function() {
            println!("This is the child function.");
        }

        pub fn call_parent_function() {
            super::parent_function();
        }
    }

    pub fn parent_function() {
        println!("This is the parent function.");
    }
}

fn main() {
    parent::child::call_parent_function();
}