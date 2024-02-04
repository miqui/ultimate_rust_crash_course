// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
// #[derive(Debug)] // include this line right before your struct definition
// struct Grapes...


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
// impl Bite for...

/* 
fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    //let mut grapes = Grapes { amount_left: 100 };
    //grapes.bite();
    //println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    //bunny_nibbles(&mut carrot);
    //println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}
*/
/* 
impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
*/

#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    status: BookStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum BookStatus {
    Available,
    Borrowed(String), // Borrower's name
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    // method, cuz it has self in the signature
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn borrow_book(&mut self, book_id: u32, borrower: &str) -> Result<(), String> {
        let book = self.books.iter_mut().find(|b| b.id == book_id);

        match book {
            Some(b) => {
                if b.status == BookStatus::Available {
                    b.status = BookStatus::Borrowed(borrower.to_string());
                    Ok(())
                } else {
                    Err(format!("Book {} is already borrowed.", b.title))
                }
            },
            None => Err("Book not found in the library.".to_string()),
        }
    }

    fn return_book(&mut self, book_id: u32) -> Result<(), String> {
        let book = self.books.iter_mut().find(|b| b.id == book_id);

        match book {
            Some(b) => {
                if b.status != BookStatus::Available {
                    b.status = BookStatus::Available;
                    Ok(())
                } else {
                    Err(format!("Book {} was not borrowed.", b.title))
                }
            },
            None => Err("Book not found in the library.".to_string()),
        }
    }

    fn show_books(&self) {
        for book in &self.books {
            println!("{:?}", book);
        }
    }
}

fn main() {
    let mut library = Library::new();

    library.add_book(Book {
        id: 1,
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        status: BookStatus::Available,
    });

    library.add_book(Book {
        id: 2,
        title: "Programming Rust".to_string(),
        author: "Jim Blandy and Jason Orendorff".to_string(),
        status: BookStatus::Available,
    });

    // Attempt to borrow a book
    match library.borrow_book(1, "Alice") {
        Ok(_) => println!("Book borrowed successfully."),
        Err(e) => println!("Error: {}", e),
    }

    // Show current library status
    library.show_books();

    // Attempt to return a book
    match library.return_book(1) {
        Ok(_) => println!("Book returned successfully."),
        Err(e) => println!("Error: {}", e),
    }

    // Final library status
    library.show_books();
}

