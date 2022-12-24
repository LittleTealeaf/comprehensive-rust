use std::fmt::{Display, Formatter, Result};

struct Library {
    books: Vec<Book>
}

struct Book {
    title: String,
    year: u16
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}


impl Library {
    fn new() -> Library {
        Library {
            books: vec![]
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    fn oldest_book<'a>(self: &'a Library)  -> Option<&'a Book> {
        let mut oldest: Option<&Book> = None;

       for book in &self.books {
           if let Some(old) = oldest {
               if old.year < book.year {
                   continue;
               }
           }
           oldest = Some(book);
       } 

        oldest
    }


}

fn main() {
    let mut library = Library::new();

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    library.print_books();

    if let Some(book) = library.oldest_book() {
        println!("My oldest book is {book}");
    }
    
}
