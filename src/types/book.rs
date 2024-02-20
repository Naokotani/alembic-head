use crate::types::content::{Details, Content, Summary};

pub struct Book {
    pub details: Details,
    pub pages: i32,
}

impl Book {
    pub fn new(details: Details, pages: i32) -> Self {
        Book {
            details,
            pages,
        }
        
    }
}

impl Content for Book {
    fn summarize(&self) -> Summary {
        Summary {
            title: self.details.title,
            thumb: self.details.thumb,
            summary: self.details.summary,
            author: self.details.author,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_log;
    #[test_log::test]

    pub fn book() {
        let content = Details::new(
            1,
            String::from("Cool Book"),
            String::from("thumb.jpg"),
            String::from("so great book"),
            String::from("Frank Hayes"),
        );
        let book = Book::new(content, 300);

        assert_eq!(book.details.title, "Cool Book");
        assert_eq!(book.details.thumb, "thumb.jpg");
        assert_eq!(book.details.author, "Frank Hayes");
        assert_eq!(book.details.summary, "so great book");
        assert_eq!(book.pages, 300);
    }
}
