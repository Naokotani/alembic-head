use tracing::{event, Level};

pub struct Details {
    id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub author: String,
}

impl Details {
    pub fn new(id: i32, title: String, thumb: String, summary: String, author: String) -> Self {
        event!(
            Level::INFO,
            "Creating new content id: {} title: {}, author: {}",
            id,
            title,
            author,
        );
        Details {
            id,
            title,
            thumb,
            summary,
            author,
        }
    }
}

pub struct Product {}

pub struct Summary {
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub author: String,
}

pub trait Content {
    fn summarize(&self) -> Summary;
    fn read() {}
    fn update() {}
    fn destroy() {}
    fn ownwership() -> Ownership {
        Ownership::Unowned
    }
    fn purchase() -> Product{
        Product {}
    }
}

#[derive(PartialEq, Debug)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq, Debug)]
enum Ownership {
    Owned,
    Free,
    Unowned,
}

impl Ownership {
    pub fn retrieve(str: &str) -> Self {
        match str {
            "owned" => Ownership::Owned,
            "unowned" => Ownership::Unowned,
            "free" => Ownership::Free,
            _ => panic!("invalid ownership"),
        }
    }

    pub fn store(&self) -> &str {
        match self {
            Ownership::Owned => "owned",
            Ownership::Free => "free",
            Ownership::Unowned => "unowned",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log;

    #[test_log::test]
    pub fn new_content() {
        let details = Details::new(
            1,
            String::from("For Whom the Bell Tolls"),
            String::from("thumb.jpg"),
            String::from("A great read"),
            String::from("Earnest Hemingway"),
        );
        assert_eq!(details.id, 1);
        assert_eq!(details.title, "For Whom the Bell Tolls");
        assert_eq!(details.thumb, "thumb.jpg");
        assert_eq!(details.summary, "A great read");
        assert_eq!(details.author, "Earnest Hemingway");
    }

    #[test_log::test]
    pub fn ownership() {
        let owned = Ownership::retrieve("owned");
        let free = Ownership::retrieve("free");
        let unowned = Ownership::retrieve("unowned");

        assert_eq!(owned, Ownership::Owned);
        assert_eq!(unowned, Ownership::Unowned);
        assert_eq!(free, Ownership::Free);

        let owned = owned.store();
        let unowned = unowned.store();
        let free = free.store();

        assert_eq!(owned, "owned");
        assert_eq!(unowned, "unowned");
        assert_eq!(free, "free");
    }

}
