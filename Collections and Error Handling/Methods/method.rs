/*
Methods are functions associated with structs, enums, or traits, allowing us 
to define behavior that's specific to the data types you create.
*/

struct Book {
    title: String,
    pages: u32,
}

impl Book {
    // Method to calculate reading time assuming 1 page per minute
    fn reading_time(&self) -> u32 {
        self.pages
    }

    // Method to check if one book has more pages than another book
    fn has_more_pages_than(&self, other: &Book) -> bool {
        self.pages > other.pages
    }

    // Associated function (not a method) to create a book with a default title and given pages
    fn default_book(pages: u32) -> Book {
        Book {
            title: String::from("Untitled"),
            pages,
        }
    }
}

fn main() {
    let book1 = Book {
        title: String::from("The Rust Programming Language"),
        pages: 500,
    };
    let book2 = Book {
        title: String::from("Learning Rust"),
        pages: 300,
    };
    let book3 = Book {
        title: String::from("Rust in Action"),
        pages: 600,
    };

    println!(
        "The reading time for {} is {} minutes.",
        book1.title,
        book1.reading_time()
    );
    println!(
        "Does {} have more pages than '{}'? {}",
        book1.title,
        book2.title,
        book1.has_more_pages_than(&book2)
    );
    println!(
        "Does {} have more pages than '{}'? {}",
        book1.title,
        book3.title,
        book1.has_more_pages_than(&book3)
    );

    let default_book = Book::default_book(1000);
    println!(
        "Default Book: {} with {} pages",
        default_book.title, default_book.pages
    );
}