struct Book {
    title: String,
    pages: u32,
}

impl Book {
    fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }
}

fn main() {
    let mut book1 = Book {
        title: String::from("Old Title"),
        pages: 250,
    };

    book1.set_title(String::from("New Title"));
    println!("The new title of the book is {}.", book1.title);
}